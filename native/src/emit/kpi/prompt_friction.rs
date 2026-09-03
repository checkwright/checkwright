// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-prompt-friction: distinct/total prompting calls via guard-kit
use super::{na, sibling_tool, Ctx};
use crate::proc;

const LABEL: &str = "prompt friction";

// spec: drift-kit/SPEC.md §Bundled KPIs — `<distinct>/<total>`, an undeclared cross-kit output
// contract this member parses and guard-kit's scanner produces; the shape check is what keeps a
// changed spelling a visible `n/a` rather than a wrong number.
pub fn parse_count(s: &str) -> Option<(u64, u64)> {
    let (a, b) = s.split_once('/')?;
    if a.is_empty() || b.is_empty() || !a.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !b.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some((a.parse().ok()?, b.parse().ok()?))
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the presence witness is the library, never the surface
// the measurement reads: `lib/guard.sh` is permanently shell, so no later cut can delete it out
// from under this row.
pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    if sibling_tool(&ctx.kit_roots, crate::guard::LIB).is_none() {
        return na("lead", LABEL, "guard-kit absent", trend);
    }
    let scanner = match sibling_tool(&ctx.kit_roots, "bin/scan-prompts.sh") {
        Some(p) => p,
        None => return na("lead", LABEL, "scanner failed", trend),
    };
    let out = proc::run("bash", &[&scanner, "--count"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()));
    let count = match out {
        Some(o) => o.trim_end_matches('\n').to_string(),
        None => return na("lead", LABEL, "scanner failed", trend),
    };
    let (_, total) = match parse_count(&count) {
        Some(p) => p,
        None => return na("lead", LABEL, "unreadable count", trend),
    };
    if total == 0 {
        return na("lead", LABEL, "no friction logged", trend);
    }
    if trend {
        return Some(format!("prompt {}\n", count));
    }
    Some(format!(
        "lead\t{}\t{} distinct patterns / prompting calls\n",
        LABEL, count
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the count is anchored whole: a decorated or partial
    // spelling degrades to `n/a` rather than parsing to a number the scanner did not mean
    #[test]
    fn the_cross_kit_count_is_read_only_in_its_declared_shape() {
        assert_eq!(parse_count("29/100"), Some((29, 100)));
        assert_eq!(parse_count("0/0"), Some((0, 0)));
        assert_eq!(parse_count("29 / 100"), None);
        assert_eq!(parse_count("29"), None);
        assert_eq!(parse_count("/100"), None);
        assert_eq!(parse_count("29/100 calls"), None);
    }
}
