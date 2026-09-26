// spec: guard-kit/SPEC.md §The shell guard — the `PreToolUse` shell guard: the consumer's rule
// command runs first, then the payload's tool selects a reader, the rule table decides, and a call
// nothing decided is logged.
use crate::guard::engine::{self, Cmd, Verdict};
use crate::guard::host::{self, Host};
use crate::guard::reader;
use crate::hook::{self, Payload};
use crate::{proc, programs};
use serde_json::Value;
use std::io::Write;

const NAME: &str = "shell-guard";

const KNOB: &str = "GUARD_KIT_CONSUMER_RULES_CMD";

// spec: guard-kit/SPEC.md §Consumer rules — what the consumer stage returned: nothing to say, a
// decision relayed as its bytes, or a fault the generic ruleset's answer carries.
enum Stage {
    Pass,
    Block(Vec<u8>),
    Decide(Vec<u8>),
    Fault(String),
}

pub fn run(payload: &Payload) -> i32 {
    let Some(p) = payload.value() else { return 0 };
    let argv = match host::consumer_cmd() {
        Ok(a) => a,
        Err(e) => return refused(&e),
    };
    let fault = match consumer_stage(&argv, payload.bytes()) {
        Stage::Pass => None,
        Stage::Block(err) => {
            let _ = std::io::stderr().write_all(&err);
            return 2;
        }
        Stage::Decide(out) => {
            let _ = std::io::stdout().write_all(&out);
            return 0;
        }
        Stage::Fault(f) => Some(f),
    };
    let fault = fault.as_deref();
    let selected = p.get("tool_name").and_then(Value::as_str).and_then(crate::guard::reader_for);
    let (Some((reader, shell)), Some(cmd)) = (selected, crate::guard::command_of(p)) else {
        return fault.map_or(0, hook::advise);
    };
    if let Some(c) = reader::control_byte(cmd) {
        return control_byte_block(c, fault);
    }
    let background = matches!(
        p.pointer("/tool_input/run_in_background"),
        Some(Value::Bool(true))
    ) || p.pointer("/tool_input/run_in_background").and_then(Value::as_str) == Some("true");
    let host = match Host::load(background) {
        Ok(h) => h,
        Err(e) => return refused(&e),
    };
    let verdict = engine::decide(reader, shell, &host, &Cmd::new(cmd)).map(|v| match v {
        Verdict::Block(m) => Verdict::Block(reader::unmark(&m)),
        Verdict::Advise(m) => Verdict::Advise(reader::unmark(&m)),
        Verdict::Allow(r) => Verdict::Allow(reader::unmark(&r)),
        Verdict::Rewrite(c, r) => Verdict::Rewrite(reader::unmark(&c), reader::unmark(&r)),
    });
    match verdict {
        Some(Verdict::Block(m)) => {
            let code = hook::block(NAME, &m);
            if let Some(f) = fault {
                eprintln!("{}: {}", NAME, f);
            }
            code
        }
        Some(Verdict::Advise(m)) => hook::advise(&fault.map_or(m.clone(), |f| format!("{}\n{}", m, f))),
        Some(Verdict::Allow(r)) => {
            println!("{}", crate::guard::allow_envelope_with(&r, fault));
            0
        }
        Some(Verdict::Rewrite(c, r)) => {
            println!("{}", crate::guard::rewrite_envelope_with(&c, &r, fault));
            0
        }
        None => {
            host.log_fallthrough(cmd, shell);
            fault.map_or(0, hook::advise)
        }
    }
}

// spec: guard-kit/SPEC.md §The shell guard — a refused knob read keeps the block, with the
// refusal's own text: the refused file is the guard's own and its repair tool is never guarded.
fn refused(e: &str) -> i32 {
    hook::block(
        NAME,
        &format!("guard-kit could not read its knobs, so no command runs until the config is repaired — {}. Repair the file with the Edit tool, which this guard does not intercept.", e),
    )
}

// spec: guard-kit/SPEC.md §The shell guard — a raw control byte could forge one of the reader's own
// marks, so the command is refused before any rule reads it.
fn control_byte_block(c: u8, fault: Option<&str>) -> i32 {
    let code = hook::block(
        NAME,
        &format!("this command carries the raw control byte 0x{:02x}, which no shell needs and which could forge the guard's own marks, so no rule reads it — write the byte as an escape the shell expands (printf '\\x{:02x}'), or remove it.", c, c),
    );
    if let Some(f) = fault {
        eprintln!("{}: {}", NAME, f);
    }
    code
}

// spec: guard-kit/SPEC.md §Consumer rules — the harness's own hook protocol, spoken by the consumer's
// command: the payload bytes on stdin as received, the working directory and environment inherited.
fn consumer_stage(argv: &[String], input: &[u8]) -> Stage {
    let Some((head, rest)) = argv.split_first() else { return Stage::Pass };
    let args: Vec<&str> = rest.iter().map(String::as_str).collect();
    let program = programs::Program::consumer("GUARD_KIT_CONSUMER_RULES_CMD", head.as_str());
    let named = argv.join(" ");
    let fault = |what: String| {
        Stage::Fault(format!(
            "the consumer rule command `{}` {}, so its rules did not run on this call and only guard-kit's generic rules decided it. Fix the command {} names.",
            named, what, KNOB
        ))
    };
    let done = match proc::run_with_stdin_in(&program, &args, input, &proc::ChildEnv::default()) {
        Ok(d) => d,
        Err(e) => return fault(format!("could not run: {}", e)),
    };
    let (out, err) = done.streams();
    match classify(done.code(), out, err) {
        Ok(stage) => stage,
        Err(what) => fault(what.unwrap_or_else(|| format!("was killed (status {})", done.reported_code()))),
    }
}

// spec: guard-kit/SPEC.md §Consumer rules — the consumer's exit read as the harness reads a hook's;
// a fault is described here, or left `None` for a kill whose status only the caller holds.
fn classify(code: Option<i32>, out: &[u8], err: &[u8]) -> Result<Stage, Option<String>> {
    match code {
        Some(2) => Ok(Stage::Block(err.to_vec())),
        Some(0) if out.iter().all(u8::is_ascii_whitespace) => Ok(Stage::Pass),
        Some(0) => match serde_json::from_slice::<Value>(out) {
            Ok(Value::Object(_)) => Ok(Stage::Decide(out.to_vec())),
            _ => Err(Some("exited 0 with stdout that is not a JSON object".to_string())),
        },
        Some(n) => Err(Some(format!("exited {}", n))),
        None => Err(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §Consumer rules — each exit the consumer can give lands on one stage;
    // a block relays the consumer's stderr and a decision its stdout, byte for byte
    #[test]
    fn the_consumer_exit_selects_the_stage() {
        assert!(matches!(classify(Some(2), b"", b"no\n"), Ok(Stage::Block(e)) if e == b"no\n"));
        assert!(matches!(classify(Some(0), b" \n", b""), Ok(Stage::Pass)));
        let obj = br#"{"decision":"block"}"#;
        assert!(matches!(classify(Some(0), obj, b""), Ok(Stage::Decide(o)) if o == obj));
        for out in [&b"[1]"[..], b"allow", b"\"s\""] {
            assert!(matches!(classify(Some(0), out, b""), Err(Some(m)) if m.contains("not a JSON object")));
        }
        assert!(matches!(classify(Some(1), b"", b""), Err(Some(m)) if m == "exited 1"));
        assert!(matches!(classify(None, b"", b""), Err(None)));
    }

    // spec: guard-kit/SPEC.md §Consumer rules — no configured command is no consumer stage
    #[test]
    fn an_empty_consumer_command_passes() {
        assert!(matches!(consumer_stage(&[], b"{}"), Stage::Pass));
    }

    // spec: guard-kit/SPEC.md §The shell guard — a raw control byte and a refused knob read both
    // block, a consumer fault beside them notwithstanding
    #[test]
    fn a_control_byte_and_a_refused_knob_read_block() {
        assert_eq!(control_byte_block(0x01, None), 2);
        assert_eq!(control_byte_block(0x1b, Some("the consumer rule command exited 1")), 2);
        assert_eq!(refused("the knob file is unreadable"), 2);
    }

    // spec: guard-kit/SPEC.md §The shell guard — a call with no parseable payload exits 0 before
    // any knob or consumer command is read
    #[test]
    fn a_missing_payload_is_let_through() {
        assert_eq!(run(&Payload { bytes: Vec::new(), value: None }), 0);
    }
}
