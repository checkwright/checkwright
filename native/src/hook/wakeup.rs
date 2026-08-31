// spec: guard-kit/SPEC.md §wakeup-guard — the PreToolUse(ScheduleWakeup|CronCreate) member: it
// denies fail-closed whatever the payload said, and logs the attempt for close's triage. It emits
// no envelope and speaks the protocol through its exit status alone.
use crate::hook;
use crate::proc;
use crate::walk;
use serde_json::Value;

const NAME: &str = "wakeup-guard";

pub fn run(payload: Option<&Value>) -> i32 {
    // spec: guard-kit/SPEC.md §wakeup-guard — the deny stands even where the log path does not
    // resolve: the block is unconditional, so a knob fault costs the record and not the rule
    if let Ok(log) = walk::knob_scalar("GUARD_KIT_WAKEUP_LOG") {
        append_attempt(&log, payload);
    }
    hook::block(NAME, DENIAL)
}

// spec: guard-kit/SPEC.md §wakeup-guard — the attempt record: a local-zone stamp, then the three
// payload fields that identify the attempt. Best-effort throughout, the shell form's `|| true`.
fn append_attempt(log: &str, payload: Option<&Value>) {
    use std::io::Write;
    let line = format!("{} {}\n", local_stamp(), summary(payload));
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(log) {
        let _ = f.write_all(line.as_bytes());
    }
}

// spec: drift-kit/SPEC.md §Bundled KPIs — `date` stays a subprocess wherever the *operator's zone*
// is the answer, which `date -Is` asks and `date -u` does not; an absent `date` costs the stamp
// alone, exactly as the shell form's discarded stderr did.
fn local_stamp() -> String {
    proc::run("date", &["-Is"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
        .unwrap_or_default()
}

// spec: guard-kit/SPEC.md §wakeup-guard — `jq -c '{session_id, tool_name, tool_input}'`, whose
// object literal fixes the key order and emits `null` for a field the payload does not carry; a
// payload that did not parse is logged whole, which here is the empty body the reader received.
fn summary(payload: Option<&Value>) -> String {
    let Some(doc) = payload else {
        return String::new();
    };
    let at = |k: &str| doc.get(k).cloned().unwrap_or(Value::Null).to_string();
    format!(
        r#"{{"session_id":{},"tool_name":{},"tool_input":{}}}"#,
        at("session_id"),
        at("tool_name"),
        at("tool_input")
    )
}

// spec: guard-kit/SPEC.md §wakeup-guard — the denial text, carried verbatim off the shell member:
// it names why a self-scheduled wakeup is refused and where the attempt is triaged.
const DENIAL: &str = "ScheduleWakeup/CronCreate is blocked in this repo — a self-scheduled wakeup re-fires its stored prompt in a later session as if the user typed it, long after its premises are stale, and the scheduling call is invisible at the moment it matters. Surface the intent to the user and let them re-invoke instead. Attempt logged (triaged at close alongside the friction log).";

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §wakeup-guard — the record's field order and its null-for-absent are
    // jq's object literal, so a reader of the existing log meets the shape it already parses
    #[test]
    fn the_attempt_record_keeps_jqs_field_order_and_nulls() {
        let doc: Value = serde_json::from_str(r#"{"tool_name":"ScheduleWakeup","extra":1}"#)
            .expect("the fixture must parse");
        assert_eq!(
            summary(Some(&doc)),
            r#"{"session_id":null,"tool_name":"ScheduleWakeup","tool_input":null}"#
        );
        assert_eq!(summary(None), "");
    }
}
