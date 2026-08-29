// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-always-loaded: standing per-session surface via context-kit
use super::{na, sibling_tool, Ctx};
use crate::proc;

const LABEL: &str = "always-loaded";

fn digits_from(b: &[u8], at: usize) -> usize {
    let mut n = at;
    while n < b.len() && b[n].is_ascii_digit() {
        n += 1;
    }
    n
}

// spec: drift-kit/SPEC.md §Bundled KPIs — `grep -oE '^[0-9]+l'`: the surface total, anchored at
// the value's own start, so a count appearing later in the parenthetical is not it.
pub fn leading_total(value: &str) -> Option<&str> {
    let b = value.as_bytes();
    let end = digits_from(b, 0);
    if end == 0 || b.get(end) != Some(&b'l') {
        return None;
    }
    Some(&value[..end + 1])
}

// spec: drift-kit/SPEC.md §Bundled KPIs — `grep -oE '[+-][0-9]+ since'` then the sign-and-digits
// half: the delta is read only where the meter marked it as one, never off a bare signed number.
pub fn since_delta(value: &str) -> Option<&str> {
    let b = value.as_bytes();
    for i in 0..b.len() {
        if b[i] != b'+' && b[i] != b'-' {
            continue;
        }
        let end = digits_from(b, i + 1);
        if end == i + 1 {
            continue;
        }
        if value[end..].starts_with(" since") {
            return Some(&value[i..end]);
        }
    }
    None
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let meter = match sibling_tool(&ctx.kit_roots, "bin/always-loaded.sh") {
        Some(p) => p,
        None => return na("lead", LABEL, "context-kit absent", trend),
    };
    let out = proc::run("bash", &[&meter])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()));
    let line = match out {
        Some(o) => o.trim_end_matches('\n').to_string(),
        None => return na("lead", LABEL, "meter failed", trend),
    };
    let value = line.strip_prefix("always-loaded: ").unwrap_or(&line);
    if value.is_empty() {
        return na("lead", LABEL, "empty meter output", trend);
    }

    if trend {
        let total = match leading_total(value) {
            Some(t) => t,
            None => return Some(String::new()),
        };
        return Some(match since_delta(value) {
            Some(d) => format!("loaded {} {}\n", total, d),
            None => format!("loaded {}\n", total),
        });
    }
    Some(format!("lead\t{}\t{}\n", LABEL, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the trend fragment reads the meter's own prose, an
    // undeclared cross-kit output contract; both reads are anchored so a reshaped line degrades
    #[test]
    fn the_trend_fragment_reads_the_anchored_total_and_the_marked_delta() {
        let v = "213l (surfaces 203 · hook 10)  +2 since de046195";
        assert_eq!(leading_total(v), Some("213l"));
        assert_eq!(since_delta(v), Some("+2"));
        assert_eq!(leading_total("surfaces 213l"), None);
        assert_eq!(since_delta("213l (hook 10)"), None);
        assert_eq!(since_delta("-14 since abc"), Some("-14"));
    }
}
