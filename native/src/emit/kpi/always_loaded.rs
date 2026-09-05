// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-always-loaded: standing per-session surface via
// context-kit, read as figures from an in-crate call rather than parsed off a spawn's stdout.
use super::{na, sibling_tool, Ctx};
use crate::emit::always_loaded;

const LABEL: &str = "always-loaded";

// spec: drift-kit/SPEC.md §Bundled KPIs — the presence witness is the library, never the surface
// the measurement reads: `lib/context.sh` carries `# no-port:` on the sole-resolver ground, so no
// later cut can delete it out from under this row.
const WITNESS: &str = "lib/context.sh";

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    if sibling_tool(&ctx.kit_roots, WITNESS).is_none() {
        return na("lead", LABEL, "context-kit absent", trend);
    }
    let m = match always_loaded::measure() {
        Ok(m) => m,
        Err(_) => return na("lead", LABEL, "meter failed", trend),
    };
    if trend {
        return Some(match m.base_total {
            Some(base) => format!(
                "loaded {}l {:+}\n",
                m.total,
                m.total as i64 - base as i64
            ),
            None => format!("loaded {}l\n", m.total),
        });
    }
    Some(format!(
        "lead\t{}\t{}\n",
        LABEL,
        always_loaded::line(&m)
    ))
}
