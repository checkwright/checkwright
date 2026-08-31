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
const FIELDS: &[&str] = &[
    "event",
    "session",
    "live",
    "verdict",
    "records",
    "decision",
    "keys",
];

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

    let event = sanitize(&hook::field(payload, &["hook_event_name"]));
    let session = sanitize(&hook::field(payload, &["session_id"]));
    let keys = sanitize(&top_level_keys(payload));
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the contracted loop guard, read
    // only to bound the one arm whose condition never resolves
    let continuing = payload
        .and_then(|d| d.get("stop_hook_active"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let reader_status = read_liveness(&liveness_cmd, &run_dir);

    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record set is counted AFTER
    // the reader ran, so a record created during the reading is counted rather than missed and an
    // in-flight record errs toward refusing
    let records = count_records(&run_dir);

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
        decision,
        keys.as_str(),
    ];
    append_record(&log, &hook::utc_stamp(kpi::now_epoch()), &values);

    if decision != "refuse" {
        return 0;
    }
    refuse(verdict, &liveness_cmd, &run_dir)
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

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — no shipped default: the reader is a
// path the consumer names, so an empty or unreadable one means the reader never ran at all.
fn read_liveness(cmd: &str, run_dir: &str) -> Option<i32> {
    if cmd.is_empty() || !std::path::Path::new(cmd).is_file() {
        return None;
    }
    // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the liveness reader stays external
    // and stays spawned: it is a member of the `scripts/` owed cohort, not of this cut.
    match proc::run_bounded("bash", &[cmd, run_dir], READER_BOUND_SECS) {
        // spec: delegation-kit/SPEC.md §The turn-end liveness hook — a timeout is an error and
        // allows, so a refusal is only ever the reader's own verdict
        Ok(None) => Some(124),
        Ok(Some(code)) => Some(code),
        Err(_) => Some(127),
    }
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the record set is the run dir's own
// `*.run` membership, a single-level listing rather than a walk beneath it, which is the shell
// form's `shopt -s nullglob; records=("$RUN_DIR"/*.run)`. An unreadable dir is an empty set.
fn count_records(run_dir: &str) -> usize {
    walk::list_dir(std::path::Path::new(run_dir))
        .map(|entries| {
            entries
                .iter()
                .filter(|(name, is_dir)| !is_dir && name.ends_with(".run"))
                .count()
        })
        .unwrap_or(0)
}

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — the message branch is three-way
// because the three refusing verdicts have three findings and three remedies; folding `unresolved`
// onto `corrupt`'s arm would print "does not parse" over a case holding no record to parse
fn refuse(verdict: &str, liveness_cmd: &str, run_dir: &str) -> i32 {
    let ways = "Two ways forward: wait for the producer on its own artifact, in a loop that ends when the condition goes true; or delete the record once the producer has exited.";
    let (finding, ways, look) = match verdict {
        "red" => (
            format!("a launch record under {} names a live producer, so this turn may not end on it", run_dir),
            ways.to_string(),
            "to see the record set for yourself",
        ),
        "corrupt" => (
            format!("a launch record under {} does not parse, so no reading says whether a producer is live and this turn may not end on it", run_dir),
            ways.to_string(),
            "to see which record is malformed",
        ),
        _ => (
            format!("the liveness reader produced no reading at all, and there is no launch record under {} for it to have been about, so nothing says whether a producer is live and this turn may not end on it", run_dir),
            "The reader is what to fix here, not a record: it failed over an empty record set, so this is a reader that could not run at all rather than a malformed record. Under a worktree-isolated dispatch, binary-dispatched gates do not resolve — the lawful response there is to report the gate as unavailable and return, never to build one.".to_string(),
            "to see the reader's own reason",
        ),
    };
    eprintln!("turn-end refused: {}.", finding);
    eprintln!("{}", ways);
    eprintln!("Run `bash {} {}` {}.", liveness_cmd, run_dir, look);
    2
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let values = ["SubagentStop", "s1", "no", "green", "0", "allow", "a,b"];
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
