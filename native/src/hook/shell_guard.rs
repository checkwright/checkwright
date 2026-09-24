// spec: guard-kit/SPEC.md §The guard framework — the `PreToolUse` shell guard: the payload's tool
// selects a reader, the rule table decides, and a call nothing decided is logged.
use crate::guard::bash::Bash;
use crate::guard::engine::{self, Cmd, Shell, Verdict};
use crate::guard::host::Host;
use crate::hook;
use serde_json::Value;

const NAME: &str = "shell-guard";

pub fn run(payload: Option<&Value>) -> i32 {
    let Some(p) = payload else { return 0 };
    if p.get("tool_name").and_then(Value::as_str) != Some("Bash") {
        return 0;
    }
    let Some(cmd) = p.pointer("/tool_input/command").and_then(Value::as_str) else {
        return 0;
    };
    let cmd = cmd.trim_end_matches('\n');
    if cmd.is_empty() {
        return 0;
    }
    let background = matches!(
        p.pointer("/tool_input/run_in_background"),
        Some(Value::Bool(true))
    ) || p.pointer("/tool_input/run_in_background").and_then(Value::as_str) == Some("true");
    // spec: guard-kit/SPEC.md §The guard framework — a refused knob read keeps the block, with the
    // refusal's own text: the refused file is the guard's own and its repair tool is never guarded.
    let host = match Host::load(background) {
        Ok(h) => h,
        Err(e) => {
            return hook::block(
                NAME,
                &format!("guard-kit could not read its knobs, so no command runs until the config is repaired — {}. Repair the file with the Edit tool, which this guard does not intercept.", e),
            )
        }
    };
    match engine::decide(&Bash, Shell::Bash, &host, &Cmd::new(cmd)) {
        Some(Verdict::Block(m)) => hook::block(NAME, &m),
        Some(Verdict::Advise(m)) => hook::advise(&m),
        Some(Verdict::Allow(r)) => {
            println!("{}", crate::guard::allow_envelope(&r));
            0
        }
        Some(Verdict::Rewrite(c, r)) => {
            println!("{}", crate::guard::rewrite_envelope(&c, &r));
            0
        }
        None => {
            host.log_fallthrough(cmd);
            0
        }
    }
}
