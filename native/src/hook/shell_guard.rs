// spec: guard-kit/SPEC.md §The shell guard — the `PreToolUse` shell guard: the consumer's rule
// command runs first, then the payload's tool selects a reader, the rule table decides, and a call
// nothing decided is logged.
use crate::guard::engine::{self, Cmd, Verdict};
use crate::guard::host::{self, Host};
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
    let background = matches!(
        p.pointer("/tool_input/run_in_background"),
        Some(Value::Bool(true))
    ) || p.pointer("/tool_input/run_in_background").and_then(Value::as_str) == Some("true");
    let host = match Host::load(background) {
        Ok(h) => h,
        Err(e) => return refused(&e),
    };
    match engine::decide(reader, shell, &host, &Cmd::new(cmd)) {
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
            host.log_fallthrough(cmd);
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
    match done.code() {
        Some(2) => Stage::Block(err.to_vec()),
        Some(0) if out.iter().all(u8::is_ascii_whitespace) => Stage::Pass,
        Some(0) => match serde_json::from_slice::<Value>(out) {
            Ok(Value::Object(_)) => Stage::Decide(out.to_vec()),
            _ => fault("exited 0 with stdout that is not a JSON object".to_string()),
        },
        Some(n) => fault(format!("exited {}", n)),
        None => fault(format!("was killed (status {})", done.reported_code())),
    }
}
