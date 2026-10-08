// spec: delegation-kit/SPEC.md §usage-verdict — the trustworthy budget verdict: one decision table
// over a usage snapshot, a three-state exit contract (0 OK/RESET-OK, 1 PAUSE, 2 STALE) and one
// verdict line. Two callers read it — the `--usage-verdict` arm and `agent-budget-guard` in process.
use crate::proc;
use crate::walk;

// spec: delegation-kit/SPEC.md §Layout and configuration — the arm's declared reads, every one a
// row of delegation-kit's table.
pub const KNOBS: &[&str] = &[
    "DELEGATION_KIT_USAGE_FILE",
    "DELEGATION_KIT_CRED_FILE",
    "DELEGATION_KIT_ACCOUNT_CONFIG",
    "DELEGATION_KIT_PAUSE_PCT",
    "DELEGATION_KIT_PAUSE_PCT_7D",
    "DELEGATION_KIT_STALE_AGE",
    "DELEGATION_KIT_LOGIN_WINDOW",
    "DELEGATION_KIT_LOGIN_SETTLE",
    "DELEGATION_KIT_REFRESH_CMD",
    "DELEGATION_KIT_REFRESH_MIN_AGE",
    "DELEGATION_KIT_USAGE_HISTORY",
    "DELEGATION_KIT_FAN_WIDTH",
];

const USAGE: &str = "usage: --usage-verdict [--] [usage-file [credentials-file]]\n  the two positionals override DELEGATION_KIT_USAGE_FILE and DELEGATION_KIT_CRED_FILE (test injection); \"--\" takes a path beginning with \"-\"";

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

// spec: delegation-kit/SPEC.md §usage-verdict — a declared knob that does not resolve is read
// as budget-unknown rather than as a decline: this member's return shape carries no decline, and 2
// is the code the contract already rules never-blocking. The front-end owns the real decline.
struct Config {
    usage_file: String,
    cred_file: String,
    account_config: String,
    pause_pct: String,
    pause_pct_7d: String,
    stale_age: String,
    login_window: i64,
    login_settle: i64,
    refresh_cmd: Vec<String>,
    refresh_min_age: i64,
    history: String,
    width: String,
}

fn config() -> Result<Config, String> {
    let k = walk::knob_scalar;
    let paths = crate::hook::usage::paths()?;
    Ok(Config {
        usage_file: paths.usage_file,
        cred_file: paths.cred_file,
        account_config: paths.account_config,
        pause_pct: k("DELEGATION_KIT_PAUSE_PCT")?,
        pause_pct_7d: k("DELEGATION_KIT_PAUSE_PCT_7D")?,
        stale_age: k("DELEGATION_KIT_STALE_AGE")?,
        login_window: int(&k("DELEGATION_KIT_LOGIN_WINDOW")?),
        login_settle: int(&k("DELEGATION_KIT_LOGIN_SETTLE")?),
        refresh_cmd: walk::knob_array("DELEGATION_KIT_REFRESH_CMD")?,
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

// spec: delegation-kit/SPEC.md §usage-verdict — the fail-closed readings: an unreadable snapshot, a
// missing mandatory key and a non-numeric percentage, each a budget-unknown its caller words.
enum Loaded {
    Read(Snapshot),
    Unreadable,
    MissingKeys(Snapshot),
    NonNumeric(Snapshot),
}

fn load(path: &str) -> Loaded {
    let Ok(body) = std::fs::read_to_string(path) else {
        return Loaded::Unreadable;
    };
    let snap = read_snapshot(&body);
    if snap.pct.is_empty() || snap.resets_at.is_empty() || snap.updated_at.is_empty() {
        return Loaded::MissingKeys(snap);
    }
    if !is_percentage(&snap.pct) {
        return Loaded::NonNumeric(snap);
    }
    Loaded::Read(snap)
}

struct Limits<'a> {
    pause: &'a str,
    pause_long: &'a str,
    stale_age: i64,
    pause_first: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Judged {
    ResetOk,
    AgeStale,
    Pause { long: bool },
    Clear,
}

// spec: delegation-kit/SPEC.md §usage-verdict — the weekly axis arms only when both seven_day keys
// are present and well-formed.
fn long_armed(snap: &Snapshot) -> bool {
    !snap.pct_7d.is_empty() && !snap.resets_7d.is_empty() && is_percentage(&snap.pct_7d) && is_epoch(&snap.resets_7d)
}

// spec: delegation-kit/SPEC.md §usage-verdict — the rule's checks over a parsed snapshot, in either
// order: RESET-OK, then age-STALE and the pause axes; under `pause_first` the axes lead, the long
// one ahead of RESET-OK (§The keyed verdict). `Clear` is an OK the reroute may still suppress.
fn judge(snap: &Snapshot, now: i64, limits: &Limits) -> Judged {
    // spec: delegation-kit/SPEC.md §usage-verdict — two pause axes judged independently; the weekly
    // axis pauses only while its window is live.
    let long = long_armed(snap) && int(&snap.resets_7d) - now > 0 && at_or_over(&snap.pct_7d, limits.pause_long);
    if long && limits.pause_first {
        return Judged::Pause { long };
    }
    if int(&snap.resets_at) - now <= 0 {
        return Judged::ResetOk;
    }
    let stale = now - int(&snap.updated_at) > limits.stale_age;
    if stale && !limits.pause_first {
        return Judged::AgeStale;
    }
    let short = at_or_over(&snap.pct, limits.pause);
    if short || long {
        return Judged::Pause { long };
    }
    if stale {
        return Judged::AgeStale;
    }
    Judged::Clear
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
    let history = walk::capture_path(&cfg.history);
    let path = std::path::Path::new(&history);
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

// spec: delegation-kit/SPEC.md §usage-verdict — the roll witnesses: the boundary of the newest sample
// for the snapshot's own account (any account where it carries none), read before this run's append,
// so the witness is a previous reading of the same account's window; every unanswerable log falls open.
fn previous_boundary(history: &str, account: &str) -> Option<i64> {
    if history.is_empty() {
        return None;
    }
    let body = std::fs::read_to_string(history).ok()?;
    let last = body.lines().rev().find(|line| {
        account.is_empty()
            || line
                .split_ascii_whitespace()
                .any(|f| f.strip_prefix("account=") == Some(account))
    })?;
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
// REFRESH_MIN_AGE save for a snapshot naming another account than the live one, and fail-soft;
// DELEGATION_KIT_REFRESH_CMD is an argv spawned with no shell.
fn refresh(cfg: &Config, live: &str) {
    let Some((program, rest)) = cfg.refresh_cmd.split_first() else {
        return;
    };
    if let Ok(body) = std::fs::read_to_string(&cfg.usage_file) {
        // spec: delegation-kit/SPEC.md §usage-verdict — the short-circuit probe is `awk -F=`'s
        // read, which takes a final unterminated record where the `read` loop above drops it, so
        // the two spellings stay the two the shell form had rather than collapsing into one.
        let field = |key: &str| {
            body.lines()
                .find_map(|l| l.split_once('=').filter(|(k, _)| *k == key))
                .map(|(_, v)| v.split('=').next().unwrap_or(v).to_string())
                .unwrap_or_default()
        };
        let stamp = field("updated_at");
        let account = field("account");
        let switched = !account.is_empty() && !live.is_empty() && account != live;
        if !switched && is_unsigned_epoch(&stamp) && now_epoch() - int(&stamp) < cfg.refresh_min_age {
            return;
        }
    }
    let args: Vec<&str> = rest.iter().map(String::as_str).collect();
    let program = crate::programs::Program::consumer("DELEGATION_KIT_REFRESH_CMD", program.as_str());
    let _ = proc::run(&program, &args);
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

    // spec: delegation-kit/SPEC.md §usage-verdict — the live identity, read before the refresh
    // because its short-circuit is keyed on it as well as the reroute after the read.
    let live = crate::hook::usage::json_field(&cfg.account_config, &["oauthAccount", "accountUuid"]);
    refresh(&cfg, &live);

    let stale = |body: String| (format!("{} -> STALE ({})", body, NEVER_BLOCKS), 2);

    let snap = match load(&cfg.usage_file) {
        Loaded::Read(s) => s,
        Loaded::Unreadable => {
            return stale(format!(
                "usage-verdict: cannot read {} width={}",
                cfg.usage_file, cfg.width
            ))
        }
        Loaded::MissingKeys(snap) => {
            return stale(format!(
                "usage-verdict: missing key(s) in {} (pct='{}' resets_at='{}' updated_at='{}') width={}",
                cfg.usage_file, snap.pct, snap.resets_at, snap.updated_at, cfg.width
            ))
        }
        Loaded::NonNumeric(snap) => {
            return stale(format!(
                "usage-verdict: non-numeric five_hour_used_pct='{}' in {} width={}",
                snap.pct, cfg.usage_file, cfg.width
            ))
        }
    };

    let now = now_epoch();
    let age = now - int(&snap.updated_at);
    let resets_in = int(&snap.resets_at) - now;
    let login_at = if std::fs::metadata(&cfg.cred_file).is_ok() {
        credentials_mtime(&cfg.cred_file)
    } else {
        0
    };
    let rolled = match previous_boundary(&walk::capture_path(&cfg.history), &snap.account) {
        Some(prev) => int(&snap.resets_at) != prev && int(&snap.updated_at) > prev,
        None => false,
    };
    let reading = format!(
        "used={}% age={}s resets_in={}s width={}",
        snap.pct, age, resets_in, cfg.width
    );
    let limits = Limits {
        pause: &cfg.pause_pct,
        pause_long: &cfg.pause_pct_7d,
        stale_age: int(&cfg.stale_age),
        pause_first: false,
    };

    match judge(&snap, now, &limits) {
        Judged::ResetOk => {
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
        Judged::AgeStale => {
            append_sample(&cfg, &snap, login_at, "STALE");
            return (
                format!(
                    "{} -> STALE (reading older than {}s; pct may lag reality; {})",
                    reading, cfg.stale_age, NEVER_BLOCKS
                ),
                2,
            );
        }
        Judged::Pause { long: true } => {
            append_sample(&cfg, &snap, login_at, "PAUSE");
            return (
                format!(
                    "used={}% (7d {}%) age={}s resets_in={}s width={} -> PAUSE (7-day window; at or over {}% of the live weekly window — remediation is days, not hours)",
                    snap.pct, snap.pct_7d, age, resets_in, cfg.width, cfg.pause_pct_7d
                ),
                1,
            );
        }
        Judged::Pause { long: false } => {
            append_sample(&cfg, &snap, login_at, "PAUSE");
            return (
                format!(
                    "{} -> PAUSE (5h window; at or over {}% of the live 5h window)",
                    reading, cfg.pause_pct
                ),
                1,
            );
        }
        Judged::Clear => {}
    }

    // spec: delegation-kit/SPEC.md §usage-verdict — the reroute follows the axis compares and may
    // suppress only the non-blocking outcome; a roll refutes the two lag arms' premise, never the
    // account-switch arm's, so the witnesses disarm the lag arms alone.
    let keyed = !snap.account.is_empty() && !live.is_empty();
    if keyed && snap.account != live {
        append_sample(&cfg, &snap, login_at, "STALE");
        return (
            format!(
                "{} -> STALE (snapshot predates an account switch; {})",
                reading, NEVER_BLOCKS
            ),
            2,
        );
    }
    let cred_age = now - login_at;
    let lagging = if keyed {
        int(&snap.updated_at) - login_at < cfg.login_settle
    } else {
        cred_age < cfg.login_window
    };
    if login_at > 0 && cred_age >= 0 && lagging && !rolled {
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

// spec: delegation-kit/SPEC.md §The keyed verdict — what the keyed read is handed: the two tables,
// the two thresholds, the stale age, the producer's bound and the toplevel it is spawned from.
pub struct KeyedConfig {
    pub usage: Vec<String>,
    pub producers: Vec<String>,
    pub pause: String,
    pub pause_long: String,
    pub stale_age: String,
    pub timeout: u64,
    pub root: String,
}

pub struct Keyed {
    pub line: String,
    pub word: &'static str,
    pub code: i32,
}

pub const USAGE_TOKEN: &str = "@USAGE_FILE@";

const KEYED_NEVER_BLOCKS: &str =
    "never blocks a foreign run — refresh the snapshot before trusting the number";

// spec: delegation-kit/SPEC.md §The keyed verdict — the adapter's own producer, run before every
// keyed read from the toplevel, bounded and fail-soft: its failure leaves the snapshot as it was.
fn produce(cfg: &KeyedConfig, adapter: &str, path: &str) {
    let argv: Vec<String> = crate::emit::foreign_run::argv_of(&cfg.producers, adapter)
        .iter()
        .map(|w| w.replace(USAGE_TOKEN, path))
        .collect();
    let Some((program, rest)) = argv.split_first() else {
        return;
    };
    let args: Vec<&str> = rest.iter().map(String::as_str).collect();
    let program = crate::programs::Program::consumer("DELEGATION_KIT_FOREIGN_USAGE_CMD", program.as_str());
    let _ = proc::run_bounded_in(&program, &args, Some(std::path::Path::new(&cfg.root)), cfg.timeout);
}

// spec: delegation-kit/SPEC.md §The keyed verdict — the account-keyed rule over one adapter's
// snapshot, pause axes ahead of age-STALE, with no identity arm, no sample and no `width=` field;
// `OFF` is the adapter with no snapshot.
pub fn keyed(cfg: &KeyedConfig, adapter: &str) -> Keyed {
    let head = format!("foreign-budget: adapter={}", adapter);
    let Some(path) = crate::emit::foreign_run::argv_of(&cfg.usage, adapter).into_iter().next() else {
        return Keyed {
            line: format!(
                "{} -> OFF (no DELEGATION_KIT_FOREIGN_USAGE snapshot is configured for this adapter; nothing budgets it, and it never blocks a foreign run)",
                head
            ),
            word: "OFF",
            code: 2,
        };
    };
    let path = walk::abs_against(&cfg.root, &path);
    produce(cfg, adapter, &path);
    let stale = |body: String| Keyed {
        line: format!("{} {} -> STALE (budget unknown; {})", head, body, KEYED_NEVER_BLOCKS),
        word: "STALE",
        code: 2,
    };
    let snap = match load(&path) {
        Loaded::Read(s) => s,
        Loaded::Unreadable => return stale(format!("cannot read {}", path)),
        Loaded::MissingKeys(snap) => {
            return stale(format!(
                "missing key(s) in {} (pct='{}' resets_at='{}' updated_at='{}')",
                path, snap.pct, snap.resets_at, snap.updated_at
            ))
        }
        Loaded::NonNumeric(snap) => {
            return stale(format!("non-numeric five_hour_used_pct='{}' in {}", snap.pct, path))
        }
    };
    let now = now_epoch();
    let age = now - int(&snap.updated_at);
    let resets_in = int(&snap.resets_at) - now;
    let long = if long_armed(&snap) {
        format!(" (long {}%)", snap.pct_7d)
    } else {
        String::new()
    };
    let reading = format!("{} used={}%{} age={}s resets_in={}s", head, snap.pct, long, age, resets_in);
    let limits = Limits {
        pause: &cfg.pause,
        pause_long: &cfg.pause_long,
        stale_age: int(&cfg.stale_age),
        pause_first: true,
    };
    let (word, code, clause) = match judge(&snap, now, &limits) {
        Judged::ResetOk => (
            "RESET-OK",
            0,
            format!(
                "window rolled over {}s ago; pct is from the dead window, so nothing pauses",
                resets_in.abs()
            ),
        ),
        Judged::Pause { long: true } => (
            "PAUSE",
            1,
            format!(
                "long window; at or over {}% of a window that resets in {}s{} — no adapter is spawned until it does",
                cfg.pause_long,
                int(&snap.resets_7d) - now,
                if resets_in <= 0 {
                    format!("; the short window rolled over {}s ago and its pct is from the dead window", resets_in.abs())
                } else {
                    String::new()
                }
            ),
        ),
        Judged::Pause { long: false } => (
            "PAUSE",
            1,
            format!(
                "short window; at or over {}% of a window that resets in {}s — no adapter is spawned until it does",
                cfg.pause, resets_in
            ),
        ),
        Judged::AgeStale => (
            "STALE",
            2,
            format!(
                "reading older than {}s and under its pause thresholds; pct may lag reality; {}",
                cfg.stale_age, KEYED_NEVER_BLOCKS
            ),
        ),
        Judged::Clear => (
            "OK",
            0,
            format!(
                "reading within {}s and under its pause thresholds; headroom now, a floor and not a recommendation",
                cfg.stale_age
            ),
        ),
    };
    Keyed { line: format!("{} -> {} ({})", reading, word, clause), word, code }
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

    // spec: delegation-kit/SPEC.md §The keyed verdict — one rule in two orders: an old at-or-over
    // reading of a live window is STALE on the account-keyed order and a PAUSE on the keyed one,
    // and a dead short window beside a live at-or-over long one pauses on the keyed order alone
    #[test]
    fn the_rule_is_one_function_in_two_orders() {
        let snap = |pct: &str, resets: i64, updated: i64, long: Option<(&str, i64)>| {
            let mut body = format!("five_hour_used_pct={}\nfive_hour_resets_at={}\nupdated_at={}\n", pct, resets, updated);
            if let Some((p, r)) = long {
                body.push_str(&format!("seven_day_used_pct={}\nseven_day_resets_at={}\n", p, r));
            }
            read_snapshot(&body)
        };
        let limits = |pause_first| Limits { pause: "80", pause_long: "95", stale_age: 600, pause_first };
        let now = 10_000;
        let old_high = snap("90", now + 100, now - 5000, None);
        assert_eq!(judge(&old_high, now, &limits(false)), Judged::AgeStale);
        assert_eq!(judge(&old_high, now, &limits(true)), Judged::Pause { long: false });
        let old_low = snap("10", now + 100, now - 5000, None);
        assert_eq!(judge(&old_low, now, &limits(false)), Judged::AgeStale);
        assert_eq!(judge(&old_low, now, &limits(true)), Judged::AgeStale);
        let old_long = snap("10", now + 100, now - 5000, Some(("96", now + 9000)));
        assert_eq!(judge(&old_long, now, &limits(true)), Judged::Pause { long: true });
        let dead_long = snap("10", now + 100, now - 5000, Some(("96", now - 1)));
        assert_eq!(judge(&dead_long, now, &limits(true)), Judged::AgeStale);
        let dead_short = |long| snap("90", now - 1, now - 5, long);
        assert_eq!(judge(&dead_short(Some(("95", now + 9000))), now, &limits(true)), Judged::Pause { long: true });
        assert_eq!(judge(&dead_short(Some(("95", now + 9000))), now, &limits(false)), Judged::ResetOk);
        for order in [false, true] {
            assert_eq!(judge(&dead_short(None), now, &limits(order)), Judged::ResetOk);
            assert_eq!(judge(&dead_short(Some(("94.9", now + 9000))), now, &limits(order)), Judged::ResetOk);
            assert_eq!(judge(&dead_short(Some(("96", now - 1))), now, &limits(order)), Judged::ResetOk);
            assert_eq!(judge(&snap("79.9", now + 100, now - 5, None), now, &limits(order)), Judged::Clear);
            assert_eq!(judge(&snap("80", now + 100, now - 5, None), now, &limits(order)), Judged::Pause { long: false });
            assert_eq!(
                judge(&snap("90", now + 100, now - 5, Some(("95", now + 9000))), now, &limits(order)),
                Judged::Pause { long: true },
                "the long axis is named when both fire"
            );
        }
    }

    // spec: delegation-kit/SPEC.md §usage-verdict — the witness reads the newest sample's boundary
    // for the snapshot's account and falls open on every way the log cannot answer
    #[test]
    fn the_roll_witness_falls_open_on_every_unanswerable_log() {
        let dir = std::env::temp_dir().join(format!("checkwright-verdict.{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let log = dir.join("history.log");
        let path = log.to_string_lossy().into_owned();
        assert_eq!(previous_boundary("", ""), None, "an unset knob falls open");
        assert_eq!(previous_boundary(&path, ""), None, "an absent file falls open");
        std::fs::write(&log, "updated_at=1 pct=3 resets_at=notanepoch verdict=OK\n")
            .expect("the tail must be writable");
        assert_eq!(previous_boundary(&path, ""), None, "a non-numeric tail falls open");
        std::fs::write(&log, "updated_at=1 resets_at=10 verdict=OK\nupdated_at=2 resets_at=20 verdict=OK\n")
            .expect("the tail must be writable");
        assert_eq!(previous_boundary(&path, ""), Some(20), "the newest sample is the witness");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: delegation-kit/SPEC.md §usage-verdict — a swap leaves another account's sample at the
    // tail, and the witness passes over it to the snapshot's own account's newest sample
    #[test]
    fn the_roll_witness_reads_the_snapshots_own_account() {
        let dir = std::env::temp_dir().join(format!("checkwright-verdict-acct.{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let log = dir.join("history.log");
        let path = log.to_string_lossy().into_owned();
        std::fs::write(
            &log,
            "updated_at=1 resets_at=10 verdict=OK account=a\nupdated_at=2 resets_at=20 verdict=OK account=b\n",
        )
        .expect("the tail must be writable");
        assert_eq!(previous_boundary(&path, "a"), Some(10), "the snapshot's own account is the witness");
        assert_eq!(previous_boundary(&path, "b"), Some(20));
        assert_eq!(previous_boundary(&path, ""), Some(20), "no account reads the newest sample of all");
        assert_eq!(previous_boundary(&path, "c"), None, "no sample for the account falls open");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
