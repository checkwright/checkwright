// spec: delegation-kit/SPEC.md §usage-verdict — the trustworthy budget verdict: one decision table
// over a usage snapshot, a three-state exit contract (0 OK/RESET-OK, 1 PAUSE, 2 STALE) and one
// verdict line. Two callers read it — the `--usage-verdict` arm and `agent-budget-guard` in process.
use crate::proc;
use crate::walk;

// spec: delegation-kit/SPEC.md §Layout and configuration — the arm's declared reads. Every one is
// defined and defaulted in `delegation-kit/lib/delegation.sh`, which the config bridge sources to
// resolve them; the crate holds no default for a bridged knob.
pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_USAGE_FILE",
    "DELEGATION_KIT_CRED_FILE",
    "DELEGATION_KIT_PAUSE_PCT",
    "DELEGATION_KIT_PAUSE_PCT_7D",
    "DELEGATION_KIT_STALE_AGE",
    "DELEGATION_KIT_LOGIN_WINDOW",
    "DELEGATION_KIT_REFRESH_CMD",
    "DELEGATION_KIT_REFRESH_MIN_AGE",
    "DELEGATION_KIT_USAGE_HISTORY",
    "DELEGATION_KIT_FAN_WIDTH",
];

const USAGE: &str = "usage: run-gates.sh --usage-verdict [--] [usage-file [credentials-file]]\n  the two positionals override DELEGATION_KIT_USAGE_FILE and DELEGATION_KIT_CRED_FILE (test injection); \"--\" takes a path beginning with \"-\"";

// spec: delegation-kit/SPEC.md §usage-verdict — the consequence half of every STALE line: the status
// half is site-specific, the consequence half is uniform, so it is one constant rather than five.
const NEVER_BLOCKS: &str =
    "never blocks delegation — re-read or refresh before trusting the number";

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape half of that contract outlives the
// member's port: a positional beginning with `-` that names no option is a refusal, and `--` ends
// option processing. The `-h`/`--help` arm does not cross — it retires to the front-end's own help.
fn parse(args: &[String]) -> Result<(Option<&str>, Option<&str>), String> {
    let rest = if args.first().map(String::as_str) == Some("--") {
        &args[1..]
    } else {
        match args.iter().find(|a| a.starts_with('-')) {
            Some(bad) => {
                return Err(format!(
                    "usage-verdict: unrecognized option: {} — a path beginning with \"-\" is passed after a \"--\" separator",
                    bad
                ))
            }
            None => args,
        }
    };
    Ok((
        rest.first().map(String::as_str),
        rest.get(1).map(String::as_str),
    ))
}

// spec: delegation-kit/SPEC.md §usage-verdict — a declared knob the bridge did not resolve is read
// as budget-unknown rather than as a decline: this member's return shape carries no decline, and 2
// is the code the contract already rules never-blocking. The front-end owns the real decline.
struct Config {
    usage_file: String,
    cred_file: String,
    pause_pct: String,
    pause_pct_7d: String,
    stale_age: String,
    login_window: i64,
    refresh_cmd: String,
    refresh_min_age: i64,
    history: String,
    width: String,
}

fn config() -> Result<Config, String> {
    let k = walk::knob_scalar;
    Ok(Config {
        usage_file: k("DELEGATION_KIT_USAGE_FILE")?,
        cred_file: k("DELEGATION_KIT_CRED_FILE")?,
        pause_pct: k("DELEGATION_KIT_PAUSE_PCT")?,
        pause_pct_7d: k("DELEGATION_KIT_PAUSE_PCT_7D")?,
        stale_age: k("DELEGATION_KIT_STALE_AGE")?,
        login_window: int(&k("DELEGATION_KIT_LOGIN_WINDOW")?),
        refresh_cmd: k("DELEGATION_KIT_REFRESH_CMD")?,
        refresh_min_age: int(&k("DELEGATION_KIT_REFRESH_MIN_AGE")?),
        history: k("DELEGATION_KIT_USAGE_HISTORY")?,
        width: k("DELEGATION_KIT_FAN_WIDTH")?,
    })
}

// spec: delegation-kit/SPEC.md §usage-verdict — bash arithmetic reads an unset or non-numeric
// operand as 0, so the ported integer read carries the same floor rather than a refusal the shell
// form never had.
fn int(s: &str) -> i64 {
    s.trim().parse::<i64>().unwrap_or(0)
}

fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

// spec: delegation-kit/SPEC.md §usage-verdict — the loader's own numeric shape, `^[0-9]+(\.[0-9]+)?$`
// for a percentage and `^-?[0-9]+$` for an epoch: a hand-written matcher because the crate carries
// no regex engine and these two shapes are kit literals rather than consumer patterns.
fn is_percentage(s: &str) -> bool {
    let mut parts = s.splitn(2, '.');
    let whole = parts.next().unwrap_or("");
    if whole.is_empty() || !whole.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    match parts.next() {
        None => true,
        Some(frac) => !frac.is_empty() && frac.bytes().all(|b| b.is_ascii_digit()),
    }
}

fn is_unsigned_epoch(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

fn is_epoch(s: &str) -> bool {
    is_unsigned_epoch(s.strip_prefix('-').unwrap_or(s))
}

// spec: delegation-kit/SPEC.md §usage-verdict — each threshold compare is a float compare, never
// integer-only arithmetic, so a fractional percentage cannot silently skip PAUSE; and both compares
// are at-or-over, so a reading exactly at the threshold pauses.
fn at_or_over(pct: &str, threshold: &str) -> bool {
    match (pct.trim().parse::<f64>(), threshold.trim().parse::<f64>()) {
        (Ok(p), Ok(t)) => p >= t,
        _ => false,
    }
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — `key=value` lines split on the *first*
// `=`, and a final unterminated line is dropped: `while IFS='=' read` ends on the short read, so a
// port that iterated every line would accept a truncated snapshot the shell form refused.
fn snapshot_lines(body: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = body.split('\n').collect();
    lines.pop();
    lines
}

#[derive(Default)]
struct Snapshot {
    pct: String,
    resets_at: String,
    updated_at: String,
    pct_7d: String,
    resets_7d: String,
    account: String,
    tier: String,
    tokens_in: String,
    tokens_out: String,
}

fn read_snapshot(body: &str) -> Snapshot {
    let mut s = Snapshot::default();
    for line in snapshot_lines(body) {
        let (key, val) = match line.split_once('=') {
            Some(kv) => kv,
            None => (line, ""),
        };
        let slot = match key {
            "five_hour_used_pct" => &mut s.pct,
            "five_hour_resets_at" => &mut s.resets_at,
            "updated_at" => &mut s.updated_at,
            "seven_day_used_pct" => &mut s.pct_7d,
            "seven_day_resets_at" => &mut s.resets_7d,
            "account" => &mut s.account,
            "tier" => &mut s.tier,
            "tokens_in" => &mut s.tokens_in,
            "tokens_out" => &mut s.tokens_out,
            _ => continue,
        };
        *slot = val.to_string();
    }
    s
}

// spec: delegation-kit/SPEC.md §The usage.txt contract — the sample line's wire shape: raw values
// verbatim, optional keys omitted (never empty) when their source is absent.
fn append_sample(cfg: &Config, snap: &Snapshot, login_at: i64, verdict: &str) {
    if cfg.history.is_empty() {
        return;
    }
    let mut line = format!(
        "updated_at={} pct={} resets_at={} verdict={} login_at={}",
        snap.updated_at, snap.pct, snap.resets_at, verdict, login_at
    );
    if !snap.account.is_empty() {
        line.push_str(&format!(" account={}", snap.account));
    }
    if !snap.tier.is_empty() {
        line.push_str(&format!(" tier={}", snap.tier));
    }
    if !snap.pct_7d.is_empty() && !snap.resets_7d.is_empty() {
        line.push_str(&format!(" pct_7d={} resets_7d={}", snap.pct_7d, snap.resets_7d));
    }
    if !snap.tokens_in.is_empty() && !snap.tokens_out.is_empty() {
        line.push_str(&format!(
            " tokens_in={} tokens_out={}",
            snap.tokens_in, snap.tokens_out
        ));
    }
    let path = std::path::Path::new(&cfg.history);
    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(dir);
        }
    }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(f, "{}", line);
    }
}

// spec: delegation-kit/SPEC.md §usage-verdict — the roll witnesses: read the newest sample's
// boundary before any append of this run's own, so the reroute is judged against the previous
// reading, never against itself. Absent, unreadable or non-numeric falls open to the reroute.
fn previous_boundary(history: &str) -> Option<i64> {
    if history.is_empty() {
        return None;
    }
    let body = std::fs::read_to_string(history).ok()?;
    let last = body.lines().next_back()?;
    let field = last
        .split_ascii_whitespace()
        .find_map(|f| f.strip_prefix("resets_at="))?;
    if is_unsigned_epoch(field) {
        field.parse::<i64>().ok()
    } else {
        None
    }
}

// spec: delegation-kit/SPEC.md §usage-verdict — the credentials-file mtime dates the last auth
// event and is the whole login-window input; read in process rather than through `stat -c %Y`,
// which is the tree's last off-floor spawn and leaves with this cut.
fn credentials_mtime(path: &str) -> i64 {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

// spec: delegation-kit/SPEC.md §usage-verdict — demand-driven refresh, short-circuited under
// REFRESH_MIN_AGE and fail-soft. `bash -c` survives the port because
// DELEGATION_KIT_REFRESH_CMD *is* a command seam rather than an implementation detail.
fn refresh(cfg: &Config) {
    if cfg.refresh_cmd.is_empty() {
        return;
    }
    if let Ok(body) = std::fs::read_to_string(&cfg.usage_file) {
        // spec: delegation-kit/SPEC.md §usage-verdict — the short-circuit probe is `awk -F=`'s
        // read, which takes a final unterminated record where the `read` loop above drops it, so
        // the two spellings stay the two the shell form had rather than collapsing into one.
        let stamp = body
            .lines()
            .find_map(|l| l.split_once('=').filter(|(k, _)| *k == "updated_at"))
            .map(|(_, v)| v.split('=').next().unwrap_or(v).to_string())
            .unwrap_or_default();
        if is_unsigned_epoch(&stamp) && now_epoch() - int(&stamp) < cfg.refresh_min_age {
            return;
        }
    }
    let _ = proc::run("bash", &["-c", &cfg.refresh_cmd]);
}

// spec: delegation-kit/SPEC.md §usage-verdict — the rule itself, returning the verdict line and the
// exit status: one function with two callers, the `--usage-verdict` arm and `agent-budget-guard`,
// which grades the `i32` on its `code == 1` branch and relays the `String` verbatim.
// spec: gate-sdk/SPEC.md §The bin/-tool contract — an empty line is the shape refusal's return: the
// usage has already gone to stderr, so the caller prints nothing on stdout and the status is whole.
pub fn verdict(args: &[String]) -> (String, i32) {
    let (usage_arg, cred_arg) = match parse(args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("{}", USAGE);
            return (String::new(), 2);
        }
    };
    let mut cfg = match config() {
        Ok(c) => c,
        Err(e) => return (format!("usage-verdict: {} -> STALE ({})", e, NEVER_BLOCKS), 2),
    };
    if let Some(p) = usage_arg {
        cfg.usage_file = p.to_string();
    }
    if let Some(p) = cred_arg {
        cfg.cred_file = p.to_string();
    }

    refresh(&cfg);

    let stale = |body: String| (format!("{} -> STALE ({})", body, NEVER_BLOCKS), 2);

    let body = match std::fs::read_to_string(&cfg.usage_file) {
        Ok(b) => b,
        Err(_) => {
            return stale(format!(
                "usage-verdict: cannot read {} width={}",
                cfg.usage_file, cfg.width
            ))
        }
    };
    let snap = read_snapshot(&body);
    if snap.pct.is_empty() || snap.resets_at.is_empty() || snap.updated_at.is_empty() {
        return stale(format!(
            "usage-verdict: missing key(s) in {} (pct='{}' resets_at='{}' updated_at='{}') width={}",
            cfg.usage_file, snap.pct, snap.resets_at, snap.updated_at, cfg.width
        ));
    }
    if !is_percentage(&snap.pct) {
        return stale(format!(
            "usage-verdict: non-numeric five_hour_used_pct='{}' in {} width={}",
            snap.pct, cfg.usage_file, cfg.width
        ));
    }

    let now = now_epoch();
    let age = now - int(&snap.updated_at);
    let resets_in = int(&snap.resets_at) - now;
    let login_at = if std::fs::metadata(&cfg.cred_file).is_ok() {
        credentials_mtime(&cfg.cred_file)
    } else {
        0
    };
    let rolled = match previous_boundary(&cfg.history) {
        Some(prev) => int(&snap.resets_at) != prev && int(&snap.updated_at) > prev,
        None => false,
    };
    let reading = format!(
        "used={}% age={}s resets_in={}s width={}",
        snap.pct, age, resets_in, cfg.width
    );

    // spec: delegation-kit/SPEC.md §usage-verdict — check order: parse -> RESET-OK -> age-STALE ->
    // pause axes -> login-STALE -> OK.
    if resets_in <= 0 {
        append_sample(&cfg, &snap, login_at, "RESET-OK");
        return (
            format!(
                "{} -> RESET-OK (window rolled over {}s ago; pct is from the dead window, re-read for the live value)",
                reading,
                resets_in.abs()
            ),
            0,
        );
    }

    if age > int(&cfg.stale_age) {
        append_sample(&cfg, &snap, login_at, "STALE");
        return (
            format!(
                "{} -> STALE (reading older than {}s; pct may lag reality; {})",
                reading, cfg.stale_age, NEVER_BLOCKS
            ),
            2,
        );
    }

    // spec: delegation-kit/SPEC.md §usage-verdict — two pause axes judged independently; the weekly
    // axis arms only when both seven_day keys are present and its window is live.
    let pause_5h = at_or_over(&snap.pct, &cfg.pause_pct);
    let pause_7d = !snap.pct_7d.is_empty()
        && !snap.resets_7d.is_empty()
        && is_percentage(&snap.pct_7d)
        && is_epoch(&snap.resets_7d)
        && int(&snap.resets_7d) - now > 0
        && at_or_over(&snap.pct_7d, &cfg.pause_pct_7d);

    if pause_5h || pause_7d {
        append_sample(&cfg, &snap, login_at, "PAUSE");
        if pause_7d {
            return (
                format!(
                    "used={}% (7d {}%) age={}s resets_in={}s width={} -> PAUSE (7-day window; at or over {}% of the live weekly window — remediation is days, not hours)",
                    snap.pct, snap.pct_7d, age, resets_in, cfg.width, cfg.pause_pct_7d
                ),
                1,
            );
        }
        return (
            format!(
                "{} -> PAUSE (5h window; at or over {}% of the live 5h window)",
                reading, cfg.pause_pct
            ),
            1,
        );
    }

    // spec: delegation-kit/SPEC.md §usage-verdict — the reroute follows the axis compares and may
    // suppress only the non-blocking outcome; a demonstrated roll refutes its lagging-reading
    // premise, so the witnesses disarm it.
    let cred_age = now - login_at;
    if login_at > 0 && cred_age >= 0 && cred_age < cfg.login_window && !rolled {
        append_sample(&cfg, &snap, login_at, "STALE");
        return (
            format!(
                "{} -> STALE (auth changed {}s ago; a /login starts fresh windows the server-fed pct lags; {})",
                reading, cred_age, NEVER_BLOCKS
            ),
            2,
        );
    }

    append_sample(&cfg, &snap, login_at, "OK");
    (format!("{} -> OK", reading), 0)
}

pub fn run(args: &[String]) -> i32 {
    let (line, code) = verdict(args);
    if !line.is_empty() {
        println!("{}", line);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the firing and the non-firing case of the
    // shape refusal: a `-`-prefixed first positional refuses, the same token after `--` is a path
    #[test]
    fn a_dash_prefixed_positional_refuses_and_the_escape_takes_it() {
        let flag = vec!["--help".to_string()];
        assert!(parse(&flag).is_err());
        let escaped = vec!["--".to_string(), "--help".to_string()];
        assert_eq!(parse(&escaped).expect("the escape must take it").0, Some("--help"));
        let plain = vec!["usage.txt".to_string(), "creds.json".to_string()];
        assert_eq!(
            parse(&plain).expect("two paths must parse"),
            (Some("usage.txt"), Some("creds.json"))
        );
        assert_eq!(parse(&[]).expect("no argv must parse"), (None, None));
    }

    // spec: delegation-kit/SPEC.md §The usage.txt contract — a final unterminated line is dropped,
    // which is `while IFS='=' read`'s own behaviour and not an accident of the shell form
    #[test]
    fn an_unterminated_final_line_is_not_a_snapshot_key() {
        let whole = read_snapshot("five_hour_used_pct=42\nupdated_at=7\n");
        assert_eq!(whole.pct, "42");
        assert_eq!(whole.updated_at, "7");
        let truncated = read_snapshot("five_hour_used_pct=42\nupdated_at=7");
        assert_eq!(truncated.pct, "42");
        assert_eq!(truncated.updated_at, "");
    }

    // spec: delegation-kit/SPEC.md §usage-verdict — both pause compares are at-or-over and a
    // fractional percentage cannot skip PAUSE, which integer-only arithmetic would
    #[test]
    fn the_threshold_compare_is_at_or_over_and_fractional() {
        assert!(at_or_over("80", "80"));
        assert!(at_or_over("80.5", "80"));
        assert!(!at_or_over("79.9", "80"));
        assert!(!at_or_over("", "80"));
        assert!(is_percentage("0") && is_percentage("12.5") && is_percentage("100"));
        assert!(!is_percentage("") && !is_percentage("12.") && !is_percentage("-1") && !is_percentage("x"));
        assert!(is_epoch("-5") && is_epoch("5") && !is_epoch("notanepoch"));
    }

    // spec: delegation-kit/SPEC.md §usage-verdict — the witness reads the newest sample's boundary
    // and falls open on every way the log cannot answer
    #[test]
    fn the_roll_witness_falls_open_on_every_unanswerable_log() {
        let dir = std::env::temp_dir().join(format!("checkwright-verdict.{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let log = dir.join("history.log");
        let path = log.to_string_lossy().into_owned();
        assert_eq!(previous_boundary(""), None, "an unset knob falls open");
        assert_eq!(previous_boundary(&path), None, "an absent file falls open");
        std::fs::write(&log, "updated_at=1 pct=3 resets_at=notanepoch verdict=OK\n")
            .expect("the tail must be writable");
        assert_eq!(previous_boundary(&path), None, "a non-numeric tail falls open");
        std::fs::write(&log, "updated_at=1 resets_at=10 verdict=OK\nupdated_at=2 resets_at=20 verdict=OK\n")
            .expect("the tail must be writable");
        assert_eq!(previous_boundary(&path), Some(20), "the newest sample is the witness");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
