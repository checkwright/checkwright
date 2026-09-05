// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the SubagentStop member: it logs one
// open key=value record per firing, emits no hook JSON at all, and speaks the protocol through its
// exit status alone — 2 with a stderr reason on a red, corrupt or unresolved reading, 0 otherwise.
use crate::emit::kpi;
use crate::hook;
use crate::proc;
use crate::walk;
use serde_json::Value;

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the bounded call keeps a hung READER
// from being read as a live PRODUCER; the shell form spent `timeout 10` where `timeout(1)` was
// installed, and the compiled form waits unconditionally, removing one optional program.
const READER_BOUND_SECS: u64 = 10;

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record's field set is OPEN: a
// reader parses by key and never by position or arity, and the writer's set is this one table, so
// per-session attribution can be added later without moving any existing reader.
// spec: delegation-kit/SPEC.md §The turn-end liveness hook — `runs` sits between `records` and
// `decision`: beside the count whose set it names, and before `keys`, which stays last as the one
// free-ish value a space-delimited parse must never step over.
const FIELDS: &[&str] = &[
    "event",
    "session",
    "live",
    "verdict",
    "records",
    "runs",
    "decision",
    "keys",
];

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — one firing's whole observable: the
// exit status the harness reads, and the refusal reason, which IS the blocking message. Returned
// rather than printed so the member's cases can assert both without a process.
pub struct Firing {
    pub code: i32,
    pub stderr: String,
}

pub fn run(payload: Option<&Value>) -> i32 {
    let log = match walk::knob_scalar("DELEGATION_KIT_STOP_LOG") {
        Ok(v) => v,
        Err(e) => return hook::decline("subagent-stop-liveness", &e),
    };
    let liveness_cmd = match walk::knob_scalar("DELEGATION_KIT_LIVENESS_CMD") {
        Ok(v) => v,
        Err(e) => return hook::decline("subagent-stop-liveness", &e),
    };
    let run_dir = match walk::knob_scalar("GATE_SDK_TMP_DIR") {
        Ok(v) => v,
        Err(e) => return hook::decline("subagent-stop-liveness", &e),
    };
    let reader = reader_argv(&liveness_cmd, &run_dir);
    let firing = fire(payload, &log, reader.as_deref(), &run_dir);
    if !firing.stderr.is_empty() {
        eprint!("{}", firing.stderr);
    }
    firing.code
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the firing itself, the reader argv
// already resolved and `None` where nothing resolved. Nothing here writes stdout: this member emits
// no hook JSON on any path, and a byte there would be a decision it never makes.
pub fn fire(
    payload: Option<&Value>,
    log: &str,
    reader: Option<&[String]>,
    run_dir: &str,
) -> Firing {
    let event = sanitize(&hook::field(payload, &["hook_event_name"]));
    let session = sanitize(&hook::field(payload, &["session_id"]));
    let keys = sanitize(&top_level_keys(payload));
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the contracted loop guard, read
    // only to bound the one arm whose condition never resolves
    let continuing = payload
        .and_then(|d| d.get("stop_hook_active"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let reader_status = reader.and_then(read_liveness);

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record set is read AFTER the
    // reader ran, so a record created during the reading is counted rather than missed and an
    // in-flight record errs toward refusing
    let run_keys = read_run_keys(run_dir);
    let records = run_keys.len();
    let runs = sanitize(&run_keys.join(","));

    let verdict = match reader_status {
        None => "unavailable",
        Some(0) => "green",
        Some(1) => "red",
        // spec: delegation-kit/SPEC.md §The turn-end liveness hook — reader exit 2 splits by record
        // count, and the split names the DIAGNOSIS without deciding the refusal: over a non-empty
        // set it is `corrupt`, over an empty one `unresolved`, and both refuse
        Some(2) if records > 0 => "corrupt",
        Some(2) => "unresolved",
        Some(_) => "error",
    };
    let live = if verdict == "red" { "yes" } else { "no" };

    let mut decision = if matches!(verdict, "red" | "corrupt" | "unresolved") {
        "refuse"
    } else {
        "allow"
    };
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — `unresolved` refuses ONCE: its
    // condition is a reader that cannot run, which no turn content changes, so a second refusal is
    // a loop that spends the session's whole budget; `red` and `corrupt` stay unconditional
    if verdict == "unresolved" && continuing {
        decision = "allow";
    }

    let values = [
        event.as_str(),
        session.as_str(),
        live,
        verdict,
        &records.to_string(),
        runs.as_str(),
        decision,
        keys.as_str(),
    ];
    append_record(log, &hook::utc_stamp(kpi::now_epoch()), &values);

    if decision != "refuse" {
        return Firing { code: 0, stderr: String::new() };
    }
    Firing { code: 2, stderr: refusal(verdict, reader, run_dir, &runs) }
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record's line form: the stamp,
// then one `key=value` per FIELDS entry, two spaces between. Composed from the table rather than
// spelled per field, so adding an attribution key is a one-table edit.
fn append_record(log: &str, stamp: &str, values: &[&str]) {
    let mut line = stamp.to_string();
    for (k, v) in FIELDS.iter().zip(values) {
        line.push_str("  ");
        line.push_str(k);
        line.push('=');
        line.push_str(v);
    }
    line.push('\n');
    if let Some(dir) = std::path::Path::new(log).parent() {
        if !dir.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(dir);
        }
    }
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the log write is best-effort: an
    // unwritable log is not a reason to refuse a turn, which is the shell form's `2>/dev/null`
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(log) {
        let _ = f.write_all(line.as_bytes());
    }
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — every value is one whitespace-free
// token, so a payload string can never split the space-delimited line
fn sanitize(v: &str) -> String {
    let out: String = v
        .chars()
        .map(|c| if c.is_whitespace() { '_' } else { c })
        .collect();
    if out.is_empty() {
        "-".to_string()
    } else {
        out
    }
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the payload's top-level key set,
// comma-joined; sorted where the shell form's `keys_unsorted` was document order, because
// `preserve_order` would add a dependency and the record is parsed by key, never by order.
fn top_level_keys(payload: Option<&Value>) -> String {
    payload
        .and_then(Value::as_object)
        .map(|m| m.keys().cloned().collect::<Vec<_>>().join(","))
        .unwrap_or_default()
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the reader argv, resolved once for
// both the spawn and the refusal text: an unset knob is the DEFAULT and an override is spawned
// directly, no interpreter word.
fn reader_argv(cmd: &str, run_dir: &str) -> Option<Vec<String>> {
    if cmd.is_empty() {
        let exe = std::env::current_exe().ok()?;
        return Some(vec![
            exe.display().to_string(),
            "check-producer-liveness".to_string(),
            run_dir.to_string(),
        ]);
    }
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — executability rather than mere
    // file-ness is the override's resolution predicate now that no interpreter word is prepended:
    // an override without the bit cannot be spawned, so it resolves to no reader at all.
    if !proc::is_executable(std::path::Path::new(cmd)) {
        return None;
    }
    Some(vec![cmd.to_string(), run_dir.to_string()])
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the reading is a CHILD's exit class,
// for the default exactly as for an override: one code path with two values of one argv, so every
// row of the verdict table stays true of both.
fn read_liveness(argv: &[String]) -> Option<i32> {
    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    match proc::run_bounded(&argv[0], &args, READER_BOUND_SECS) {
        // spec: delegation-kit/SPEC.md §The turn-end liveness hook — a timeout is an error and
        // allows, so a refusal is only ever the reader's own verdict
        Ok(None) => Some(124),
        Ok(Some(code)) => Some(code),
        // spec: delegation-kit/SPEC.md §The turn-end liveness hook — a spawn that never started is
        // `unavailable`, not `error`: `error` reports a reader that RAN and returned an unmapped
        // code, so reporting it here would name a reading that was never taken.
        Err(_) => None,
    }
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record set is the run dir's own
// `*.run` membership, a single-level listing rather than a walk beneath it, which is the shell
// form's `shopt -s nullglob; records=("$RUN_DIR"/*.run)`. An unreadable dir is an empty set.
// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the keys are kept rather than counted
// away: `records` is this set's length and `runs` is its members, so the count and the names are
// one derivation over one listing and never two readings that could disagree.
fn read_run_keys(run_dir: &str) -> Vec<String> {
    let mut keys: Vec<String> = walk::list_dir(std::path::Path::new(run_dir))
        .map(|entries| {
            entries
                .iter()
                .filter(|(name, is_dir)| !is_dir && name.ends_with(".run"))
                .map(|(name, _)| name[..name.len() - ".run".len()].to_string())
                .collect()
        })
        .unwrap_or_default();
    keys.sort();
    keys
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the message branch is three-way
// because the three refusing verdicts have three findings and three remedies; folding `unresolved`
// onto `corrupt`'s arm would print "does not parse" over a case holding no record to parse
// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the `red` and `corrupt` arms name the
// record set the decision was taken over, which at `records=1` is the matched record exactly and
// above it the candidate set; `unresolved` takes no such field, its set being empty by construction
fn refusal(verdict: &str, reader: Option<&[String]>, run_dir: &str, runs: &str) -> String {
    let ways = "Two ways forward: wait for the producer on its own artifact, in a loop that ends when the condition goes true; or delete the record once the producer has exited.";
    let (finding, ways, look) = match verdict {
        "red" => (
            format!("a launch record under {} names a live producer, so this turn may not end on it. The record set read was runs={}", run_dir, runs),
            ways.to_string(),
            "to see the record set for yourself",
        ),
        "corrupt" => (
            format!("a launch record under {} does not parse, so no reading says whether a producer is live and this turn may not end on it. The record set read was runs={}", run_dir, runs),
            ways.to_string(),
            "to see which record is malformed",
        ),
        _ => (
            format!("the liveness reader produced no reading at all, and there is no launch record under {} for it to have been about, so nothing says whether a producer is live and this turn may not end on it", run_dir),
            "The reader is what to fix here, not a record: it failed over an empty record set, so this is a reader that could not run at all rather than a malformed record. Under a worktree-isolated dispatch, binary-dispatched gates do not resolve — the lawful response there is to report the gate as unavailable and return, never to build one.".to_string(),
            "to see the reader's own reason",
        ),
    };
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the reader argv is printed as the
    // hook spawned it, interpreter word and all where there is one, so re-running it by hand is the
    // same invocation rather than a transcription of it
    format!(
        "turn-end refused: {}.\n{}\nRun `{}` {}.\n",
        finding,
        ways,
        reader.map(|a| a.join(" ")).unwrap_or_default(),
        look
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the payload the recovered case
    // table fires with, carried verbatim so the `keys` expectation below is over its real key set
    const PAYLOAD: &str = r#"{"session_id":"s-1","transcript_path":"/x/y.jsonl","hook_event_name":"SubagentStop","stop_hook_active":false}"#;
    const CONTINUING: &str = r#"{"session_id":"s-1","hook_event_name":"SubagentStop","stop_hook_active":true}"#;

    // spec: gate-sdk/SPEC.md §check-test-hermetic — one scratch root per case, named for the case,
    // so parallel tests never share a run dir or a log
    struct Scratch(std::path::PathBuf);

    impl Scratch {
        fn new(case: &str) -> Scratch {
            let d = std::env::temp_dir().join(format!("cw-stop-{}-{}", std::process::id(), case));
            let _ = std::fs::remove_dir_all(&d);
            std::fs::create_dir_all(d.join("runs")).expect("scratch must be creatable");
            Scratch(d)
        }
        fn at(&self, rel: &str) -> String {
            self.0.join(rel).display().to_string()
        }
        // spec: delegation-kit/SPEC.md §The turn-end liveness hook — a reader stub is written the
        // way the contract requires a shell override to be written: its own shebang and its own
        // executable bit, because no interpreter word is prepended to it
        fn stub(&self, name: &str, body: &str) -> String {
            let p = self.at(name);
            std::fs::write(&p, format!("#!/usr/bin/env bash\n{}\n", body))
                .expect("stub must be writable");
            crate::install::make_executable(std::path::Path::new(&p)).expect("stub must be runnable");
            p
        }
        // comment-tier-exempt: a reader stub's resolved argv, one per exit class — the case table's own shape
        fn reader(&self, name: &str, code: i32) -> Vec<String> {
            let p = self.stub(name, &format!("exit {}", code));
            reader_argv(&p, &self.at("runs")).expect("an executable stub must resolve")
        }
        fn record(&self, name: &str) {
            std::fs::write(self.0.join("runs").join(name), "pid=1 run=k\n").expect("record");
        }
        fn log(&self, name: &str) -> String {
            std::fs::read_to_string(self.at(name)).unwrap_or_default()
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn payload(src: &str) -> Option<Value> {
        if src.is_empty() {
            return None;
        }
        serde_json::from_str(src).ok()
    }

    // spec: gate-sdk/SPEC.md §The port-candidate criteria — criterion 2: a port proves the two
    // substrates agree at port time and nothing keeps them agreeing after. These expectations are
    // the deleted case table's, carried forward as the coverage that replaces a spent oracle.
    fn want(line: &str, case: &str, parts: &[&str]) {
        for p in parts {
            assert!(line.contains(p), "case {}: line lacks '{}': {}", case, p, line);
        }
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case A: no reader at all. The
    // hook holds no reading, so it says so rather than reporting a clean tree it never asked
    // about, and allows.
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — `keys` is the SORTED set, the
    // ruled replacement for `jq keys_unsorted`'s document order, so the ruling is asserted here
    // rather than the deleted table's literal.
    #[test]
    fn no_reader_at_all_is_unavailable_and_allows() {
        let s = Scratch::new("no-reader");
        let f = fire(payload(PAYLOAD).as_ref(), &s.at("a.log"), None, &s.at("runs"));
        assert_eq!(f.code, 0);
        assert!(f.stderr.is_empty(), "an allowing firing must write no reason");
        want(&s.log("a.log"), "no-reader", &[
            "event=SubagentStop", "session=s-1", "live=no", "verdict=unavailable",
            "records=0", "runs=-", "decision=allow",
            "keys=hook_event_name,session_id,stop_hook_active,transcript_path",
        ]);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the firing/non-firing pair `runs`
    // owes: a populated run dir names its records' keys, an empty one renders the absent token
    #[test]
    fn the_record_set_is_named_by_key_and_an_empty_set_renders_absent() {
        let s = Scratch::new("runs-pair");
        // comment-tier-exempt: one row parsed back by key, never by position — the record's own contract
        fn field(line: &str, key: &str) -> String {
            line.split_whitespace()
                .find_map(|t| t.strip_prefix(&format!("{}=", key)))
                .unwrap_or_default()
                .to_string()
        }

        fire(payload(PAYLOAD).as_ref(), &s.at("empty.log"), None, &s.at("runs"));
        let empty = s.log("empty.log");
        assert_eq!(field(&empty, "runs"), "-", "an empty record set renders absent: {}", empty);
        assert_eq!(field(&empty, "records"), "0");

        s.record("beta.run");
        s.record("alpha.run");
        fire(payload(PAYLOAD).as_ref(), &s.at("full.log"), None, &s.at("runs"));
        let full = s.log("full.log");
        assert_eq!(
            field(&full, "runs"), "alpha,beta",
            "the set is the `.run` basenames, suffix stripped, sorted and comma-joined: {}", full
        );
        assert_eq!(
            field(&full, "records"), "2",
            "the count and the names are one derivation over one listing"
        );
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record set reaches the refused
    // session too, on the two record-bearing arms and never on `unresolved`, whose set is empty by
    // construction
    #[test]
    fn the_two_record_bearing_arms_name_the_set_and_unresolved_does_not() {
        let s = Scratch::new("runs-message");
        s.record("sibling.run");
        let red = fire(payload(PAYLOAD).as_ref(), &s.at("mr.log"), Some(&s.reader("mrr", 1)), &s.at("runs"));
        want(&red.stderr, "red-runs", &["runs=sibling"]);
        let corrupt = fire(payload(PAYLOAD).as_ref(), &s.at("mc.log"), Some(&s.reader("mcc", 2)), &s.at("runs"));
        want(&corrupt.stderr, "corrupt-runs", &["runs=sibling"]);

        let u = Scratch::new("runs-message-unresolved");
        let unresolved = fire(payload(PAYLOAD).as_ref(), &u.at("mu.log"), Some(&u.reader("muu", 2)), &u.at("runs"));
        assert_eq!(unresolved.code, 2);
        assert!(
            !unresolved.stderr.contains("runs="),
            "the unresolved arm named a record set that is empty by construction: {}",
            unresolved.stderr
        );
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — cases B-E: one firing per reader
    // exit class over a non-empty record set. The exit code is the predicate; `corrupt` carries
    // `live=no decision=refuse`, which is why decision cannot be derived from live.
    #[test]
    fn each_reader_exit_class_takes_its_own_verdict_arm() {
        for (verdict, code, live, decision, rc) in [
            ("green", 0, "live=no", "allow", 0),
            ("red", 1, "live=yes", "refuse", 2),
            ("corrupt", 2, "live=no", "refuse", 2),
            ("error", 77, "live=no", "allow", 0),
        ] {
            let s = Scratch::new(&format!("class-{}", verdict));
            s.record("k.run");
            let stub = s.reader(&format!("reader-{}", verdict), code);
            let log = s.at(&format!("{}.log", verdict));
            let f = fire(payload(PAYLOAD).as_ref(), &log, Some(&stub), &s.at("runs"));
            assert_eq!(f.code, rc, "reader exit {} must map to exit {}", code, rc);
            assert_eq!(
                !f.stderr.is_empty(), rc == 2,
                "a refusal carries its reason on stderr and an allow carries none: {}", verdict
            );
            want(&s.log(&format!("{}.log", verdict)), verdict, &[
                &format!("verdict={}", verdict), live, "records=1",
                &format!("decision={}", decision),
            ]);
        }
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case F: each refusing arm names
    // its own finding and its own remedy, and the red arm names the reader command so the session
    // can see the record set for itself
    #[test]
    fn each_refusing_arm_names_its_own_finding_and_remedy() {
        let s = Scratch::new("messages");
        s.record("k.run");
        let red = fire(payload(PAYLOAD).as_ref(), &s.at("r.log"), Some(&s.reader("rr", 1)), &s.at("runs"));
        want(&red.stderr, "red-message", &[
            "turn-end refused", "wait for the producer on its own artifact",
            "delete the record once the producer has exited", &s.at("runs"),
        ]);
        let corrupt = fire(payload(PAYLOAD).as_ref(), &s.at("c.log"), Some(&s.reader("cc", 2)), &s.at("runs"));
        want(&corrupt.stderr, "corrupt-message", &["does not parse", "which record is malformed"]);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case F2: the SAME reader exit
    // over an EMPTY record set is a different reading. It still refuses — the count names the
    // diagnosis and decides nothing.
    #[test]
    fn reader_exit_two_over_an_empty_set_is_unresolved_not_corrupt() {
        let s = Scratch::new("unresolved");
        let f = fire(payload(PAYLOAD).as_ref(), &s.at("u.log"), Some(&s.reader("ru", 2)), &s.at("runs"));
        assert_eq!(f.code, 2);
        want(&s.log("u.log"), "unresolved", &[
            "verdict=unresolved", "live=no", "records=0", "decision=refuse",
        ]);
        want(&f.stderr, "unresolved-message", &[
            "turn-end refused", "produced no reading at all", "the reader's own reason",
        ]);
        assert!(
            !f.stderr.contains("does not parse"),
            "the unresolved arm reused the corrupt arm's wording over a case holding no record"
        );
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case F4: `unresolved` refuses
    // ONCE, its condition being invariant under turn content; `red` and `corrupt` on the same
    // payload still refuse.
    #[test]
    fn unresolved_allows_once_the_harness_is_already_continuing() {
        let s = Scratch::new("continuing");
        let f = fire(payload(CONTINUING).as_ref(), &s.at("uc.log"), Some(&s.reader("ruc", 2)), &s.at("runs"));
        assert_eq!(f.code, 0);
        want(&s.log("uc.log"), "unresolved-continuing", &[
            "verdict=unresolved", "records=0", "decision=allow",
        ]);
        s.record("k.run");
        for (verdict, code) in [("red", 1), ("corrupt", 2)] {
            let log = s.at(&format!("{}c.log", verdict));
            let g = fire(payload(CONTINUING).as_ref(), &log, Some(&s.reader(&format!("c{}", verdict), code)), &s.at("runs"));
            assert_eq!(g.code, 2, "{} still refuses while continuing", verdict);
            want(&s.log(&format!("{}c.log", verdict)), verdict, &[
                &format!("verdict={}", verdict), "decision=refuse",
            ]);
        }
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case F3: the record set is
    // counted AFTER the reader ran: a stub that writes a record and then exits 2 reads as
    // `corrupt` over records=1, where counting first would miss the in-flight record.
    #[test]
    fn the_record_set_is_counted_after_the_reader_ran() {
        let s = Scratch::new("race");
        let path = s.stub(
            "reader-race",
            &format!("printf 'pid=1 run=r\\n' > {}/r.run\nexit 2", s.at("runs")),
        );
        let stub = reader_argv(&path, &s.at("runs")).expect("an executable stub must resolve");
        let f = fire(payload(PAYLOAD).as_ref(), &s.at("race.log"), Some(&stub), &s.at("runs"));
        assert_eq!(f.code, 2);
        want(&s.log("race.log"), "race", &["verdict=corrupt", "records=1", "decision=refuse"]);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case G: an OVERRIDE that resolves
    // to nothing is `unavailable`'s remaining producer once the unset knob means the default, so
    // the hook degrades and allows rather than refusing every turn end behind a mis-set knob.
    #[test]
    fn an_unresolvable_override_is_unavailable_and_allows() {
        let s = Scratch::new("absent");
        s.record("k.run");
        let reader = reader_argv(&s.at("nowhere/check.sh"), &s.at("runs"));
        assert!(reader.is_none(), "a path that is not there resolves to no reader");
        let f = fire(payload(PAYLOAD).as_ref(), &s.at("g.log"), reader.as_deref(), &s.at("runs"));
        assert_eq!(f.code, 0);
        assert!(f.stderr.is_empty());
        want(&s.log("g.log"), "absent-reader", &["verdict=unavailable", "live=no", "decision=allow"]);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the unset knob is the DEFAULT and
    // not an absence: it resolves to the running executable, which carries the gate as a member, so
    // the argv is the binary, the gate name and the run dir.
    #[test]
    fn an_unset_knob_resolves_the_running_executable_and_the_gate_name() {
        let argv = reader_argv("", "/run/dir").expect("the default must resolve");
        let exe = std::env::current_exe().expect("a running test has an executable");
        assert_eq!(
            argv,
            vec![exe.display().to_string(), "check-producer-liveness".to_string(), "/run/dir".to_string()],
            "the default is the running executable spawned with the gate name and the run dir"
        );
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — an override is spawned DIRECTLY
    // with the scratch dir as its only argument, so one lacking the executable bit resolves to no
    // reader rather than being run under a borrowed interpreter.
    #[test]
    fn an_override_is_spawned_with_no_interpreter_word_and_owes_its_own_bit() {
        let s = Scratch::new("override-argv");
        let exec = s.stub("with-bit", "exit 0");
        assert_eq!(
            reader_argv(&exec, "/run/dir").expect("an executable override must resolve"),
            vec![exec.clone(), "/run/dir".to_string()],
            "argv[0] is the override itself, and the run dir is its only argument"
        );

        let plain = s.at("no-bit");
        std::fs::write(&plain, "#!/usr/bin/env bash\nexit 0\n").expect("stub must be writable");
        assert!(
            reader_argv(&plain, "/run/dir").is_none(),
            "an override without the executable bit cannot be spawned, so it resolves to no reader"
        );
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case H: an unwritable log drops
    // the line silently. The reading still decides, so the firing allows on its verdict rather
    // than on the log.
    #[test]
    fn an_unwritable_log_drops_the_line_and_decides_anyway() {
        let s = Scratch::new("unwritable");
        std::fs::write(s.at("blocker"), "not a directory\n").expect("blocker");
        let f = fire(payload(PAYLOAD).as_ref(), &s.at("blocker/probe.log"), None, &s.at("runs"));
        assert_eq!(f.code, 0);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case I: whitespace in a payload
    // value must not split the space-delimited line. The FIELD COUNT is the assertion, because a
    // split is invisible to a substring match.
    #[test]
    fn whitespace_in_a_payload_cannot_split_the_line() {
        let s = Scratch::new("spacey");
        let f = fire(
            payload(r#"{"session_id":"a b\tc","hook_event_name":"Subagent Stop"}"#).as_ref(),
            &s.at("i.log"),
            None,
            &s.at("runs"),
        );
        assert_eq!(f.code, 0);
        let line = s.log("i.log");
        assert_eq!(
            line.split_whitespace().count(), 9,
            "want 9 whitespace-separated fields (stamp + 8 keys), got: {}", line
        );
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case J: an empty payload degrades
    // every payload-derived field to `-`, and the refusal is exact, because the decision reads the
    // liveness reader and no payload field.
    #[test]
    fn an_empty_payload_degrades_its_fields_and_still_decides() {
        let s = Scratch::new("empty-payload");
        s.record("k.run");
        let f = fire(None, &s.at("j.log"), Some(&s.reader("rj", 1)), &s.at("runs"));
        assert_eq!(f.code, 2);
        want(&s.log("j.log"), "empty-payload", &[
            "event=-", "session=-", "keys=-", "live=yes", "decision=refuse",
        ]);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — case K: one firing appends
    // exactly one line, so the log is a firing count as well as a record.
    #[test]
    fn one_firing_appends_exactly_one_line() {
        let s = Scratch::new("append");
        let log = s.at("k.log");
        fire(payload(PAYLOAD).as_ref(), &log, None, &s.at("runs"));
        fire(payload(PAYLOAD).as_ref(), &log, None, &s.at("runs"));
        assert_eq!(s.log("k.log").lines().filter(|l| !l.is_empty()).count(), 2);
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — every value is one whitespace-free
    // token and an absent one is `-`, so the space-delimited line can never be split by content
    #[test]
    fn every_logged_value_is_one_token() {
        assert_eq!(sanitize("a b\tc\nd"), "a_b_c_d");
        assert_eq!(sanitize(""), "-");
        assert_eq!(sanitize("plain"), "plain");
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record is open: a reader
    // parses by key, so the writer's field set is the table and the line is composed from it
    #[test]
    fn the_record_is_key_addressed_over_the_field_table() {
        let values = ["SubagentStop", "s1", "no", "green", "0", "-", "allow", "a,b"];
        assert_eq!(values.len(), FIELDS.len(), "the table and the row must agree");
        let dir = std::env::temp_dir().join(format!("cw-stop-{}", std::process::id()));
        let log = dir.join("subagent-stop-liveness.log");
        let path = log.display().to_string();
        append_record(&path, "1970-01-01T00:00:00Z", &values);
        let text = std::fs::read_to_string(&path).expect("the record must be written");
        let _ = std::fs::remove_dir_all(&dir);
        for (k, v) in FIELDS.iter().zip(values) {
            assert!(
                text.contains(&format!("{}={}", k, v)),
                "the record must carry {} by key",
                k
            );
        }
        assert!(text.starts_with("1970-01-01T00:00:00Z  event="));
    }

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the keys value is derived from the
    // payload rather than transcribed, and an unreadable payload yields the absent token
    #[test]
    fn the_key_set_comes_from_the_payload() {
        let doc: Value =
            serde_json::from_str(r#"{"session_id":"s","hook_event_name":"SubagentStop"}"#)
                .expect("the fixture must parse");
        assert_eq!(top_level_keys(Some(&doc)), "hook_event_name,session_id");
        assert_eq!(sanitize(&top_level_keys(None)), "-");
    }
}
