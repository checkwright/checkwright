// spec: gate-sdk/SPEC.md §The harness-integration arm — the observed run: the member re-run as a
// child of this binary, the decision it would have made read off the child and logged, and the call
// allowed with nothing said, unless the log cannot be written
use super::binding;
use super::Payload;
use crate::emit::kpi;
use crate::exit;
use crate::{proc, programs, walk};
use serde_json::Value;

pub const LOG_KNOB: &str = "GATE_SDK_HOOK_OBSERVE_LOG";

pub const BLOCK: &str = "block";
pub const NONE: &str = "none";
pub const ADVISE: &str = "advise";
pub const GRANT: &str = "grant";
pub const REWRITE: &str = "rewrite";
pub const NOTICE: &str = "notice";
pub const ERROR: &str = "error";

// spec: gate-sdk/SPEC.md §The harness-integration arm — `-` stands for an absent value in every field
const ABSENT: &str = "-";

// spec: gate-sdk/SPEC.md §The harness-integration arm — a block's and an advise's reason, the one
// token whatever the member, since a member's refusal or context can quote the input it read
pub const WITHHELD: &str = "withheld";

#[derive(Debug, PartialEq)]
pub struct Would {
    pub decision: &'static str,
    pub reason: String,
}

pub fn run(member: &str, payload: &Payload) -> i32 {
    let value = payload.value();
    let log = match log_path() {
        Ok(p) => p,
        Err(e) => return super::decline(member, &e, value),
    };
    let knobs = super::knob_file();
    let mut argv = vec!["--hook", member];
    if let Some(file) = knobs.as_deref() {
        argv.extend([super::KNOBS_FLAG, file]);
    }
    let mark = [(super::OBSERVED_VAR.to_string(), "1".to_string())];
    let child = proc::self_image().map_err(|e| e.to_string()).and_then(|exe| {
        proc::run_with_stdin_in(
            &programs::CHECKWRIGHT_GATES.at(exe.display().to_string()),
            &argv,
            payload.bytes(),
            &proc::ChildEnv { set: &mark, ..proc::ChildEnv::default() },
        )
    });
    let (status, would) = match &child {
        Ok(done) => {
            let (out, err) = done.streams();
            let event = super::field(value, &[binding::EVENT_NAME]);
            (done.code(), classify(done.code(), &event, out, err))
        }
        Err(e) => (None, Would { decision: ERROR, reason: first_line(e) }),
    };
    let line = record(
        &super::utc_stamp(kpi::now_epoch()),
        member,
        &super::field(value, &[binding::EVENT_NAME]),
        &super::field(value, &[binding::SESSION_ID]),
        &would,
        status,
    );
    match append(&log, &line) {
        Ok(()) => 0,
        Err(e) => super::decline(
            member,
            &format!("{} is '{}', and the observe log could not be written: {}", LOG_KNOB, log, e),
            value,
        ),
    }
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the log's path: absolute under the
// tree-free mark, and in the main checkout from a linked worktree
fn log_path() -> Result<String, String> {
    let value = walk::knob_scalar(LOG_KNOB)?;
    super::tree_free_path(LOG_KNOB, &value)?;
    Ok(walk::capture_path(&value))
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the would-be decision table, each envelope
// read through the binding table's spellings; plain text on the session-start event is the context
// the harness adds from it
pub fn classify(code: Option<i32>, event: &str, stdout: &[u8], stderr: &[u8]) -> Would {
    let error = || Would { decision: ERROR, reason: first_line(&String::from_utf8_lossy(stderr)) };
    match code {
        Some(exit::HOOK_BLOCK) => Would { decision: BLOCK, reason: WITHHELD.to_string() },
        Some(0) if stdout.iter().all(u8::is_ascii_whitespace) => Would { decision: NONE, reason: ABSENT.to_string() },
        Some(0) => match serde_json::from_slice::<Value>(stdout) {
            Ok(v) => envelope(&v).unwrap_or_else(error),
            Err(_) if event == binding::EVENT_SESSION_START => Would { decision: ADVISE, reason: WITHHELD.to_string() },
            Err(_) => error(),
        },
        _ => error(),
    }
}

fn envelope(v: &Value) -> Option<Would> {
    let text = |x: Option<&Value>| x.and_then(Value::as_str).map_or(ABSENT.to_string(), first_line);
    let would = |decision, reason| Some(Would { decision, reason });
    if let Some(inner) = v.get(binding::HOOK_SPECIFIC_OUTPUT) {
        if inner.get(binding::PERMISSION_DECISION).and_then(Value::as_str) == Some(binding::DECISION_ALLOW) {
            let decision = if inner.get(binding::UPDATED_INPUT).is_some() { REWRITE } else { GRANT };
            return would(decision, text(inner.get(binding::PERMISSION_DECISION_REASON)));
        }
        return inner.get(binding::ADDITIONAL_CONTEXT)?.as_str().and_then(|_| would(ADVISE, WITHHELD.to_string()));
    }
    if v.get(binding::TURN_END_DECISION).and_then(Value::as_str) == Some(binding::DECISION_BLOCK) {
        return would(BLOCK, WITHHELD.to_string());
    }
    let alone = v.as_object().is_some_and(|o| o.len() == 1);
    match v.get(binding::SYSTEM_MESSAGE).and_then(Value::as_str) {
        Some(n) if alone => would(NOTICE, first_line(n)),
        _ => None,
    }
}

fn first_line(s: &str) -> String {
    match s.lines().next().map(str::trim_end) {
        Some(l) if !l.is_empty() => l.to_string(),
        _ => ABSENT.to_string(),
    }
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the line's fields in order, `reason=` last
// so a reason carrying spaces needs no quoting
pub fn record(stamp: &str, member: &str, event: &str, session: &str, would: &Would, status: Option<i32>) -> String {
    let status = status.map_or(ABSENT.to_string(), |c| c.to_string());
    format!(
        "ts={} member={} event={} session={} would={} status={} reason={}\n",
        stamp,
        member,
        super::stop_liveness::sanitize(event),
        super::stop_liveness::sanitize(session),
        would.decision,
        status,
        would.reason
    )
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the whole line in one write on an append
// handle, so concurrent hooks interleave whole lines
fn append(log: &str, line: &str) -> std::io::Result<()> {
    use std::io::Write;
    if let Some(dir) = std::path::Path::new(log).parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::OpenOptions::new().create(true).append(true).open(log)?.write_all(line.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(code: Option<i32>, out: &str, err: &str) -> (&'static str, String) {
        read_on(binding::EVENT_BEFORE_TOOL_CALL, code, out, err)
    }

    fn read_on(event: &str, code: Option<i32>, out: &str, err: &str) -> (&'static str, String) {
        let w = classify(code, event, out.as_bytes(), err.as_bytes());
        (w.decision, w.reason)
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — each row of the would-be decision
    // table, through a stub child's status and streams
    #[test]
    fn each_child_outcome_reads_as_its_tables_row() {
        let s = |x: &str| x.to_string();
        assert_eq!(read(Some(2), "", "m: refused\nmore"), ("block", s("withheld")));
        assert_eq!(read(Some(0), "\n", ""), ("none", s("-")));
        assert_eq!(read(Some(0), &binding::context_envelope("advice\nrest"), ""), ("advise", s("withheld")));
        assert_eq!(read(Some(0), &binding::allow_envelope("granted", None, Some("c")), ""), ("grant", s("granted")));
        assert_eq!(read(Some(0), &binding::allow_envelope("r", Some("ls"), None), ""), ("rewrite", s("r")));
        assert_eq!(read(Some(0), r#"{"decision":"block","reason":"still running"}"#, ""), ("block", s("withheld")));
        assert_eq!(read(Some(0), &binding::notice_envelope("declined"), ""), ("notice", s("declined")));
        assert_eq!(read(Some(1), "", "boom"), ("error", s("boom")));
        assert_eq!(read(None, "", ""), ("error", s("-")));
        assert_eq!(read(Some(0), "not json", "why"), ("error", s("why")));
        assert_eq!(read(Some(0), r#"{"systemMessage":"n","x":1}"#, ""), ("error", s("-")));
        let start = binding::EVENT_SESSION_START;
        assert_eq!(read_on(start, Some(0), "## Brief\nplain text", "why"), ("advise", s("withheld")));
        assert_eq!(read_on(start, Some(0), r#"{"systemMessage":"n","x":1}"#, ""), ("error", s("-")));
        assert_eq!(read_on(binding::EVENT_CHILD_TURN_END, Some(0), "plain text", "why"), ("error", s("why")));
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — the line's field order, `-` for absent
    // values, and payload strings that can never split it
    #[test]
    fn the_line_carries_its_fields_in_order() {
        let w = Would { decision: BLOCK, reason: "a reason with spaces".to_string() };
        assert_eq!(
            record("2026-11-14T00:00:00Z", "m", "SubagentStop", "", &w, Some(2)),
            "ts=2026-11-14T00:00:00Z member=m event=SubagentStop session=- would=block status=2 reason=a reason with spaces\n"
        );
        let n = Would { decision: ERROR, reason: "-".to_string() };
        assert!(record("t", "m", "a b", "s", &n, None).contains("event=a_b session=s would=error status=- reason=-"));
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a log that cannot be opened is an error
    // the caller declines on, never a silent drop
    #[test]
    fn an_unopenable_log_is_an_error() {
        let dir = std::env::temp_dir().join(format!("observe-unit-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ok = dir.join("sub/log");
        assert!(append(&ok.display().to_string(), "x\n").is_ok());
        assert!(append(&dir.display().to_string(), "x\n").is_err(), "a directory is no log");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
