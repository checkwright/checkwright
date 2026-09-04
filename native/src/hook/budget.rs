// spec: delegation-kit/SPEC.md §The delegation model — the PreToolUse(Agent) budget guard: block on
// a PAUSE verdict, advise on every other. The payload is unused; the verdict is the whole input.
use crate::hook;
use serde_json::Value;

const NAME: &str = "agent-budget-guard";

// spec: delegation-kit/SPEC.md §usage-verdict — the verdict rule is in this binary, so the guard
// calls it directly: no path to resolve, no `bash` to spawn, one `String` and no stream question.
// The arm-unavailable path the spawn error stood for is the front-end's, which fails open.
pub fn run(_payload: Option<&Value>) -> i32 {
    let (verdict, code) = hook::verdict::verdict(&[]);
    if code == 1 {
        return hook::block(NAME, &format!("{}\n{}", verdict, CORRECTIVE));
    }
    hook::advise(&format!("budget verdict ({}): {}", NAME, verdict))
}

// spec: delegation-kit/SPEC.md §The delegation model — the block's corrective half, carried verbatim
// off the shell member: it names the axis-to-window mapping and the two knobs that raise it.
const CORRECTIVE: &str = "corrective: the verdict names the axis that fired — a 5h PAUSE clears when that window resets (hours); a 7-day PAUSE costs days, so pause delegation and let the supervisor carry the week. The full delegation protocol is /agent-execution. To override deliberately, raise the matching knob (DELEGATION_KIT_PAUSE_PCT for 5h, DELEGATION_KIT_PAUSE_PCT_7D for the weekly axis) via the .claude/settings.local.json env block, which the hook re-reads per fire (delegation-kit/SPEC.md §The delegation model).";
