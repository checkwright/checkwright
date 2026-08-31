// spec: delegation-kit/SPEC.md §The delegation model — the PreToolUse(Agent) budget guard: block on
// a PAUSE verdict, advise on every other. The payload is unused; the verdict is the whole input.
use crate::hook;
use crate::proc;
use crate::walk;
use serde_json::Value;

const NAME: &str = "agent-budget-guard";

// spec: delegation-kit/SPEC.md §usage-verdict — the verdict binary stays external and stays spawned:
// it is a member of the kit-`bin/` owed cohort, not of this cut, so the arm reaches it through its
// knob exactly as the shell member did.
pub fn run(_payload: Option<&Value>) -> i32 {
    let bin = match walk::knob_scalar("DELEGATION_KIT_VERDICT_BIN") {
        Ok(v) => v,
        Err(e) => return hook::decline(NAME, &e),
    };
    // spec: delegation-kit/SPEC.md §The delegation model — the shell form captured `2>&1`, so the
    // verdict a caller quotes is whatever the binary said on either stream; a spawn that never ran
    // takes the advise arm, the same `*)` branch a non-1 exit status took.
    let (verdict, code) = match proc::run_merged("bash", &[&bin]) {
        Ok(m) => (
            String::from_utf8_lossy(m.output()).trim_end().to_string(),
            m.reported_code(),
        ),
        Err(e) => (e, 127),
    };
    if code == 1 {
        return hook::block(NAME, &format!("{}\n{}", verdict, CORRECTIVE));
    }
    hook::advise(&format!("budget verdict ({}): {}", NAME, verdict))
}

// spec: delegation-kit/SPEC.md §The delegation model — the block's corrective half, carried verbatim
// off the shell member: it names the axis-to-window mapping and the two knobs that raise it.
const CORRECTIVE: &str = "corrective: the verdict names the axis that fired — a 5h PAUSE clears when that window resets (hours); a 7-day PAUSE costs days, so pause delegation and let the supervisor carry the week. The full delegation protocol is /agent-execution. To override deliberately, raise the matching knob (DELEGATION_KIT_PAUSE_PCT for 5h, DELEGATION_KIT_PAUSE_PCT_7D for the weekly axis) via the .claude/settings.local.json env block, which the hook re-reads per fire (delegation-kit/SPEC.md §The delegation model).";
