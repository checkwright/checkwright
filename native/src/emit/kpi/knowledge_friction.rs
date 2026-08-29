// spec: drift-kit/SPEC.md §The knowledge-friction loop — three log states, three lines: absent is
// no capture loop, empty is not evidence of zero friction, non-empty is a lower bound. --trend
// keeps one grammar across all three, so a series spanning this change stays one series.
use super::{na, read, Ctx};

const LABEL: &str = "knowledge friction";

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.knowledge_log) {
        Some(t) => t,
        None => return na("lag", LABEL, "no knowledge-friction log", trend),
    };
    let count = text
        .lines()
        .filter(|l| l.chars().any(|c| !c.is_whitespace()))
        .count();
    if trend {
        return Some(format!("kfric {}\n", count));
    }
    if count == 0 {
        return Some(format!(
            "lag\t{}\t0 logged — not evidence of zero friction; no capture floor exists\n",
            LABEL
        ));
    }
    Some(format!(
        "lag\t{}\t{} re-derivation(s) logged this iteration (lower bound)\n",
        LABEL, count
    ))
}
