// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-stage-economics-lag: closed iterations stamped after the newest one the stage-economics log prices
use super::{na, read, Ctx};

const LABEL: &str = "stage-economics lag";

// spec: drift-kit/SPEC.md §Bundled KPIs — keyed on iterations in first-terminal-stamp order, never on
// dates, since several closes can share a day; `None` is "nothing closed", `Some(None)` "none priced".
pub fn lag(
    stamps: &[String],
    terminal: &str,
    priced: &std::collections::HashSet<String>,
) -> Option<Option<(usize, String)>> {
    let mut closed: Vec<String> = Vec::new();
    for line in stamps {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 2 || f[0].starts_with('#') || f[1] != terminal {
            continue;
        }
        if !closed.iter().any(|c| c == f[0]) {
            closed.push(f[0].to_string());
        }
    }
    if closed.is_empty() {
        return None;
    }
    let at = closed.iter().rposition(|c| priced.contains(c));
    Some(at.map(|i| (closed.len() - 1 - i, closed[i].clone())))
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let Some(log) = read(&ctx.stage_economics_log) else {
        return na("lead", LABEL, "no stage-economics log", trend);
    };
    let Some(terminal) = ctx.stages.last() else {
        return na("lead", LABEL, "empty stage roster", trend);
    };
    let priced: std::collections::HashSet<String> = log
        .lines()
        .filter_map(|l| l.split(' ').nth(1).map(str::to_string))
        .collect();
    let stamps = crate::history::stamp_lines(&ctx.state_file);
    match lag(&stamps, terminal, &priced) {
        None => na("lead", LABEL, "no closed iteration", trend),
        Some(None) => na("lead", LABEL, "no closed iteration priced", trend),
        Some(Some((n, iter))) => Some(if trend {
            format!("econ {}\n", n)
        } else if n == 0 {
            format!("lead\t{}\t0 — newest close priced ({})\n", LABEL, iter)
        } else {
            format!(
                "lead\t{}\t{} close(s) unpriced since {} (run --emit stage-economics)\n",
                LABEL, n, iter
            )
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the lag counts closes after the last priced one in
    // first-terminal-stamp order; a re-stamped close keeps its first position, and nothing priced is
    // its own answer rather than a count of every close in history
    #[test]
    fn the_lag_counts_closes_after_the_newest_priced_one() {
        let stamps: Vec<String> = [
            "# header",
            "a scope s1 2026-01-01 h",
            "a close s2 2026-01-01 h",
            "b close s3 2026-01-01 h",
            "c close s4 2026-01-01 h",
            "b close s5 2026-01-02 h",
            "d scope s6 2026-01-02 h",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let set = |xs: &[&str]| xs.iter().map(|s| s.to_string()).collect();
        assert_eq!(lag(&stamps, "close", &set(&["a"])), Some(Some((2, "a".to_string()))));
        assert_eq!(lag(&stamps, "close", &set(&["a", "b"])), Some(Some((1, "b".to_string()))));
        assert_eq!(lag(&stamps, "close", &set(&["c", "d"])), Some(Some((0, "c".to_string()))));
        assert_eq!(lag(&stamps, "close", &set(&["zzz"])), Some(None));
        assert_eq!(lag(&stamps, "validate", &set(&["a"])), None);
    }
}
