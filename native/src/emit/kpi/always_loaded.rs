// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-always-loaded: standing per-session surface via
// context-kit, read as figures from an in-crate call rather than parsed off a spawn's stdout, and
// measured from the report's own iteration start so the row and the header name one start.
use super::{na, sibling_tool, Ctx};
use crate::emit::always_loaded;

const LABEL: &str = "always-loaded";

// spec: drift-kit/SPEC.md §Bundled KPIs — the presence witness is the kit's config template, never
// the surface the measurement reads: a knob file is no port candidate, so no later cut can delete
// it out from under this row.
const WITNESS: &str = "templates/context-config.knobs";

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    if sibling_tool(&ctx.kit_roots, WITNESS).is_none() {
        return na("lead", LABEL, "context-kit absent", trend);
    }
    let m = match always_loaded::measure(&ctx.iteration_start) {
        Ok(m) => m,
        Err(_) => return na("lead", LABEL, "meter failed", trend),
    };
    if trend {
        let mark = if m.stale() { " stale" } else { "" };
        return Some(match m.base_total {
            Some(base) => format!(
                "loaded {}l {:+}{}\n",
                m.total,
                m.total as i64 - base as i64,
                mark
            ),
            None => format!("loaded {}l{}\n", m.total, mark),
        });
    }
    Some(format!(
        "lead\t{}\t{}\n",
        LABEL,
        always_loaded::line(&m)
    ))
}
