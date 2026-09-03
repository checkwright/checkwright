// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-prompt-friction: distinct/total prompting calls via
// guard-kit, read as two integers from an in-crate call rather than parsed off a spawn's stdout.
use super::{na, sibling_tool, Ctx};
use crate::emit::scan_prompts;

const LABEL: &str = "prompt friction";

// spec: drift-kit/SPEC.md §Bundled KPIs — the presence witness is the library, never the surface
// the measurement reads: `lib/guard.sh` is permanently shell, so no later cut can delete it out
// from under this row.
pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    if sibling_tool(&ctx.kit_roots, crate::guard::LIB).is_none() {
        return na("lead", LABEL, "guard-kit absent", trend);
    }
    let (distinct, total) =
        scan_prompts::count(&ctx.guard_log, &ctx.settings, &ctx.settings_local);
    if total == 0 {
        return na("lead", LABEL, "no friction logged", trend);
    }
    let count = format!("{}/{}", distinct, total);
    if trend {
        return Some(format!("prompt {}\n", count));
    }
    Some(format!(
        "lead\t{}\t{} distinct patterns / prompting calls\n",
        LABEL, count
    ))
}
