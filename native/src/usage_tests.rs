// spec: delegation-kit/SPEC.md §Testing — the crate-side decision-table runner for usage-verdict
// and assertion runner for usage-trend, driven over the kit's committed fixtures on disk. These
// replace the two shell runners.
use crate::knobenv;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const KIT_PREFIX: &str = "DELEGATION_KIT_";

fn kit(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(rel)
}

fn text(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

// spec: delegation-kit/SPEC.md §Testing — a throwaway sandbox per case set, so no consumer config
// sits on the subject's lookup path
fn sandbox(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("checkwright-{}.{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the throwaway sandbox must be creatable");
    dir
}

struct Ran {
    out: String,
    code: i32,
}

// spec: delegation-kit/SPEC.md §Testing — the subject is spawned with its cwd inside the sandbox
// and its two streams merged, the `2>&1` capture the shell form took: every assertion reads the
// merged stream, and capturing them apart would pass the no-leak conjunct vacuously.
fn subject(cwd: &Path, script: &Path, args: &[String], extra: &[(&str, String)]) -> Ran {
    let capture = cwd.join(".merged-capture");
    let out = std::fs::File::create(&capture).expect("the merge capture must be creatable");
    let err = out.try_clone().expect("the merge capture must be duplicable");
    let mut cmd = Command::new("bash");
    cmd.arg(script)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err));
    // spec: delegation-kit/SPEC.md §Testing — the strip is the whole namespace, derived from the
    // process's own variables at run time; a hardcoded list re-creates the failure the poison
    // export exists to catch.
    for (name, _) in std::env::vars() {
        if name.starts_with(KIT_PREFIX) {
            cmd.env_remove(&name);
        }
    }
    for (name, value) in extra {
        cmd.env(name, value);
    }
    let status = cmd.status().expect("the subject must be spawnable");
    let merged = std::fs::read_to_string(&capture).expect("the merged capture must be readable");
    let _ = std::fs::remove_file(&capture);
    Ran {
        out: merged,
        code: status.code().unwrap_or(-1),
    }
}

// spec: delegation-kit/SPEC.md §Testing — `grep -c ''`'s count, under which a final unterminated
// line still counts; a line iterator's count is silently one short of it
fn counted_lines(body: &str) -> usize {
    body.split_inclusive('\n').count()
}

// spec: delegation-kit/SPEC.md §Testing — the credentials mtime is the whole login-window input,
// and it stays a spawn of the same floor program the shell form used; §usage-verdict's own cut is
// where the crate-floor question comes due.
fn set_mtime(path: &Path, epoch: i64) {
    let status = Command::new("touch")
        .arg("-d")
        .arg(format!("@{}", epoch))
        .arg(path)
        .status()
        .expect("touch must be spawnable");
    assert!(
        status.success(),
        "touch could not set the credentials mtime on {}",
        path.display()
    );
}

fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the clock must be at or after the epoch")
        .as_secs() as i64
}

struct Verdict {
    sandbox: PathBuf,
    gate: PathBuf,
    usage: PathBuf,
    cred: PathBuf,
    hist: PathBuf,
    now: i64,
}

impl Verdict {
    // spec: delegation-kit/SPEC.md §Testing — every snapshot timestamp is relative to one reading
    // of the clock: a per-case read introduces cross-case skew at the at-or-over boundary rows.
    fn write_snapshot(&self, pct: &str, age_off: i64, reset_off: i64, weekly: Option<(&str, i64)>) {
        let mut body = format!(
            "five_hour_used_pct={}\nfive_hour_resets_at={}\nupdated_at={}\n",
            pct,
            self.now + reset_off,
            self.now - age_off
        );
        if let Some((pct_7d, reset7d_off)) = weekly {
            body.push_str(&format!(
                "seven_day_used_pct={}\nseven_day_resets_at={}\n",
                pct_7d,
                self.now + reset7d_off
            ));
        }
        std::fs::write(&self.usage, body).expect("the snapshot must be writable");
    }

    fn set_credentials(&self, age: Option<i64>) {
        let _ = std::fs::remove_file(&self.cred);
        if let Some(seconds) = age {
            std::fs::write(&self.cred, "").expect("the credentials file must be writable");
            set_mtime(&self.cred, self.now - seconds);
        }
    }

    fn history(&self) -> String {
        std::fs::read_to_string(&self.hist).unwrap_or_default()
    }

    fn run(&self, extra: &[(&str, String)]) -> Ran {
        let args = vec![text(&self.usage), text(&self.cred)];
        subject(&self.sandbox, &self.gate, &args, extra)
    }

    fn run_logged(&self) -> Ran {
        self.run(&[("DELEGATION_KIT_USAGE_HISTORY", text(&self.hist))])
    }
}

// spec: delegation-kit/SPEC.md §Testing — the roll witnesses need a pre-seeded history tail the
// table's columns cannot express, so they are asserted beside it over one fixed shape
fn roll_case(v: &Verdict, desc: &str, tail: &str, want_code: i32, want_verdict: &str) {
    v.write_snapshot("3", 0, 18000, None);
    v.set_credentials(Some(60));
    let _ = std::fs::remove_file(&v.hist);
    if !tail.is_empty() {
        std::fs::write(&v.hist, format!("{}\n", tail)).expect("the history tail must be writable");
    }
    let ran = v.run_logged();
    assert_eq!(
        ran.code, want_code,
        "[{}]: want exit {}, got {} -- {}",
        desc, want_code, ran.code, ran.out
    );
    assert!(
        ran.out.contains(&format!("-> {}", want_verdict)),
        "[{}]: output missing '-> {}': {}",
        desc,
        want_verdict,
        ran.out
    );
}

// spec: delegation-kit/SPEC.md §Testing — the decision table is the kit's own reviewable test data,
// read from disk rather than transcribed into Rust literals, and the beside-the-table assertions
// ride the same sandbox, the same strip and the same single clock reading.
#[test]
fn the_kits_verdict_decision_table_holds() {
    // spec: delegation-kit/SPEC.md §Testing — the poison is a real export in this process, so a
    // broken strip fails the table loudly instead of passing it vacuously.
    let knobs = knobenv::lock();
    knobs.set("DELEGATION_KIT_PAUSE_PCT", "0");

    let dir = sandbox("usage-tests");
    let v = Verdict {
        gate: kit("delegation-kit/bin/usage-verdict.sh"),
        usage: dir.join("usage.txt"),
        cred: dir.join(".credentials.json"),
        hist: dir.join("history.log"),
        sandbox: dir,
        now: now_epoch(),
    };

    let table = kit("delegation-kit/usage-tests/cases.tsv");
    let body = std::fs::read_to_string(&table).expect("the kit's decision table must be read");
    let mut ran = 0usize;
    for line in body.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        assert_eq!(cols.len(), 11, "malformed case row: {}", line);
        let (verdict, want, pct) = (cols[0], cols[1], cols[2]);
        let (pct_7d, append, axis, desc) = (cols[6], cols[8], cols[9], cols[10]);
        let num = |c: &str| c.parse::<i64>().unwrap_or_else(|_| panic!("case [{}]: {}", desc, line));
        let weekly = if pct_7d == "-" {
            None
        } else {
            Some((pct_7d, num(cols[7])))
        };
        v.write_snapshot(pct, num(cols[3]), num(cols[4]), weekly);
        v.set_credentials(if cols[5] == "-" { None } else { Some(num(cols[5])) });
        let _ = std::fs::remove_file(&v.hist);

        let got = v.run_logged();
        ran += 1;
        assert_eq!(
            got.code,
            num(want) as i32,
            "[{}]: want exit {}, got {} -- {}",
            desc,
            want,
            got.code,
            got.out
        );
        assert!(
            got.out.contains(&format!("-> {}", verdict)),
            "[{}]: output missing verdict '-> {}': {}",
            desc,
            verdict,
            got.out
        );
        // spec: delegation-kit/SPEC.md §Testing — the trailing space discriminates the default
        // width from a two-digit one
        assert!(
            got.out.contains("width=2 "),
            "[{}]: verdict line dropped the fan-width field: {}",
            desc,
            got.out
        );
        match axis {
            "5h" => assert!(
                got.out.contains("5h window"),
                "[{}]: PAUSE did not name the 5h axis: {}",
                desc,
                got.out
            ),
            "7d" => assert!(
                got.out.contains("7-day window"),
                "[{}]: PAUSE did not name the 7-day axis: {}",
                desc,
                got.out
            ),
            _ => {}
        }

        let log = v.history();
        assert_eq!(
            counted_lines(&log),
            num(append) as usize,
            "[{}]: appended sample count",
            desc
        );
        if append == "1" {
            assert_eq!(
                log.contains("pct_7d="),
                pct_7d != "-",
                "[{}]: the sample line's pct_7d does not track the snapshot's: {}",
                desc,
                log
            );
        }
    }
    // spec: delegation-kit/SPEC.md §Testing — the anti-vacuity guard the shell runner spelled as
    // its exit-2 `ran == 0` refusal
    assert!(ran > 0, "no cases parsed from {}", table.display());

    // spec: delegation-kit/SPEC.md §usage-verdict — the width field tracks the fan-width knob
    // rather than a literal
    v.write_snapshot("40", 0, 3600, None);
    v.set_credentials(None);
    let widened = v.run(&[("DELEGATION_KIT_FAN_WIDTH", "7".to_string())]);
    assert!(
        widened.out.contains("width=7 "),
        "the width field did not track DELEGATION_KIT_FAN_WIDTH: {}",
        widened.out
    );

    // spec: delegation-kit/SPEC.md §usage-verdict — demand-driven refresh, each arm proved through
    // a stub command because a real poll would need the network
    let stub = v.sandbox.join("refresh-stub.sh");
    let stamp = v.sandbox.join("refresh-ran");
    let refresh = |path: &Path| ("DELEGATION_KIT_REFRESH_CMD", format!("bash {}", text(path)));

    std::fs::write(
        &stub,
        format!(
            "#!/usr/bin/env bash\ntouch \"{}\"\n{{\n    printf 'five_hour_used_pct=95\\n'\n    printf 'five_hour_resets_at={}\\n'\n    printf 'updated_at={}\\n'\n}} > \"{}.tmp\" && mv \"{}.tmp\" \"{}\"\n",
            text(&stamp),
            v.now + 3600,
            v.now,
            text(&v.usage),
            text(&v.usage),
            text(&v.usage)
        ),
    )
    .expect("the refresh stub must be writable");
    v.write_snapshot("40", 1200, 3600, None);
    v.set_credentials(None);
    let _ = std::fs::remove_file(&stamp);
    let armed = v.run(&[refresh(&stub)]);
    assert!(
        stamp.exists(),
        "[refresh-armed-stale]: a stale snapshot did not invoke DELEGATION_KIT_REFRESH_CMD: {}",
        armed.out
    );
    assert!(
        armed.out.contains("used=95%"),
        "[refresh-armed-stale]: the verdict read the cached pct, not the refreshed one: {}",
        armed.out
    );

    std::fs::write(
        &stub,
        "#!/usr/bin/env bash\necho \"usage-poller: fetch failed\" >&2\nexit 1\n",
    )
    .expect("the refresh stub must be writable");
    v.write_snapshot("40", 1200, 3600, None);
    v.set_credentials(None);
    let before = std::fs::read_to_string(&v.usage).expect("the seeded snapshot must be readable");
    let soft = v.run(&[refresh(&stub)]);
    assert_eq!(
        std::fs::read_to_string(&v.usage).unwrap_or_default(),
        before,
        "[refresh-fail-soft]: a failed refresh mutated the snapshot"
    );
    assert_eq!(
        soft.code, 2,
        "[refresh-fail-soft]: want the cached snapshot judged STALE, got exit {} -- {}",
        soft.code, soft.out
    );
    assert!(
        soft.out.contains("-> STALE"),
        "[refresh-fail-soft]: want the cached snapshot judged STALE: {}",
        soft.out
    );
    // spec: delegation-kit/SPEC.md §usage-verdict — callers relay the verdict line verbatim, so
    // this conjunct is negative and rides the merged stream
    assert!(
        !soft.out.contains("fetch failed"),
        "[refresh-fail-soft]: a refresh diagnostic leaked into the verdict output: {}",
        soft.out
    );

    std::fs::write(
        &stub,
        format!("#!/usr/bin/env bash\ntouch \"{}\"\n", text(&stamp)),
    )
    .expect("the refresh stub must be writable");
    v.write_snapshot("40", 0, 3600, None);
    v.set_credentials(None);
    let _ = std::fs::remove_file(&stamp);
    let fresh = v.run(&[refresh(&stub)]);
    assert!(
        !stamp.exists(),
        "[refresh-skip-fresh]: a snapshot under REFRESH_MIN_AGE (default 60s) still invoked the \
         refresh — the render path would hammer the source: {}",
        fresh.out
    );

    let crossed = v.now - 3600;
    roll_case(
        &v,
        "roll-witnesses-disarm-reroute",
        &format!(
            "updated_at={} pct=86.0 resets_at={} verdict=PAUSE login_at=0",
            v.now - 7200,
            crossed
        ),
        0,
        "OK",
    );
    roll_case(
        &v,
        "roll-witness-boundary-unmoved",
        &format!(
            "updated_at={} pct=86.0 resets_at={} verdict=PAUSE login_at=0",
            v.now - 60,
            v.now + 18000
        ),
        2,
        "STALE",
    );
    roll_case(
        &v,
        "roll-witness-uncrossed-boundary",
        &format!(
            "updated_at={} pct=86.0 resets_at={} verdict=PAUSE login_at=0",
            v.now - 60,
            v.now + 900
        ),
        2,
        "STALE",
    );
    roll_case(
        &v,
        "roll-witness-malformed-tail",
        &format!(
            "updated_at={} pct=86.0 resets_at=notanepoch verdict=PAUSE login_at=0",
            v.now - 7200
        ),
        2,
        "STALE",
    );
    roll_case(&v, "roll-witness-absent-history", "", 2, "STALE");

    // spec: delegation-kit/SPEC.md §usage-verdict — the witness reads the previous sample, never
    // the one this run appends, so a first-ever sample cannot disarm its own reroute
    let _ = std::fs::remove_file(&v.hist);
    roll_case(&v, "roll-witness-not-self-witnessing", "", 2, "STALE");
    assert_eq!(
        counted_lines(&v.history()),
        1,
        "[roll-witness-not-self-witnessing]: the run did not leave exactly its own sample behind"
    );

    let _ = std::fs::remove_dir_all(&v.sandbox);
    knobs.remove("DELEGATION_KIT_PAUSE_PCT");
}

// spec: delegation-kit/SPEC.md §Testing — the trend needles are exact golden strings; loosening one
// into a pattern weakens the assertion it ports
const TREND_NEEDLES: &[(&str, &str)] = &[
    ("per-account grouping: acctA heads its own block", "account acctA"),
    ("per-account grouping: acctB heads its own block", "account acctB"),
    (
        "spike-then-correction excluded, not averaged: the ends stay put",
        "reset@20000 tier=pro: 10.0%->25.0%",
    ),
    ("spike-then-correction: the pair is flagged suspect", "2 suspect"),
    (
        "weekly reunion across the switch-back: one acctA week spanning",
        "[7d] reset@600000 tier=pro: 30.0%->70.0%",
    ),
    (
        "weekly reunion: every acctA sample in one segment",
        "7 sample(s)",
    ),
    ("token delta on the weekly report", "tokens: +1200 in / +410 out"),
    ("weekly headroom on the report", "weekly headroom:"),
    (
        "PAUSE onset annotated where the pause first landed",
        "first PAUSE onset at epoch 33600",
    ),
];

// spec: delegation-kit/SPEC.md §Testing — the segment counts are anchored on the reporter's own
// two-space indentation, which is load-bearing
const TREND_SEGMENTS: &[(&str, &str, usize)] = &[
    ("5h segments: acctA's reset-boundary windows and acctB's login split", "  [5h]", 5),
    ("7d segments: acctA's reunited week and acctB's login split", "  [7d]", 3),
];

// spec: delegation-kit/SPEC.md §Testing — the static history fixture stays on disk: its epochs are
// safe because the reporter measures within-segment deltas, never against *now*.
#[test]
fn the_kits_trend_fixture_reports_its_segments() {
    // spec: delegation-kit/SPEC.md §Testing — the poison is the same knob the reporter reads, which
    // is what makes the unset-knob arm below meaningful rather than vacuous.
    let knobs = knobenv::lock();
    let history = kit("delegation-kit/usage-tests/trend-history.log");
    knobs.set("DELEGATION_KIT_USAGE_HISTORY", &text(&history));

    let dir = sandbox("trend-tests");
    let trend = kit("delegation-kit/bin/usage-trend.sh");

    let ran = subject(&dir, &trend, &[text(&history)], &[]);
    assert_eq!(ran.code, 0, "want exit 0, got {} -- {}", ran.code, ran.out);
    for (label, needle) in TREND_NEEDLES {
        assert!(
            ran.out.contains(needle),
            "[{}]: output missing '{}'",
            label,
            needle
        );
    }
    for (label, prefix, want) in TREND_SEGMENTS {
        let got = ran.out.lines().filter(|l| l.starts_with(prefix)).count();
        assert_eq!(got, *want, "[{}]: segment lines under '{}'", label, prefix);
    }

    // spec: delegation-kit/SPEC.md §Trend reporter — the two fail-closed arms, each asserting the
    // child's own status rather than reproducing the shell idiom that read it
    let unset = subject(&dir, &trend, &[], &[]);
    assert_eq!(
        unset.code, 2,
        "[unset knob]: want exit 2, got {} -- {}",
        unset.code, unset.out
    );
    let missing = subject(&dir, &trend, &[text(&dir.join("nope.log"))], &[]);
    assert_eq!(
        missing.code, 2,
        "[missing history]: want exit 2, got {} -- {}",
        missing.code, missing.out
    );

    let _ = std::fs::remove_dir_all(&dir);
    knobs.remove("DELEGATION_KIT_USAGE_HISTORY");
}
