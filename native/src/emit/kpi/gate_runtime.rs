// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-gate-runtime: full-battery runtime from the runner's timings file
use super::{na, now_epoch, read, Ctx};

const LABEL: &str = "gate runtime";

// spec: drift-kit/SPEC.md §Bundled KPIs — the reading-age caveat's own scale: a past measurement
// is stamped with how stale it is, because the file is state some other mechanism owns.
pub fn age_label(secs: i64) -> String {
    if secs < 90 {
        format!("{}s ago", secs)
    } else if secs < 5400 {
        format!("{}m ago", secs / 60)
    } else if secs < 172800 {
        format!("{}h ago", secs / 3600)
    } else {
        format!("{}d ago", secs / 86400)
    }
}

fn field(line: &str, n: usize) -> Option<&str> {
    line.split_whitespace().nth(n)
}

// spec: drift-kit/SPEC.md §Bundled KPIs — `sort -rn | head -3`: descending by the millisecond
// field, ties broken by GNU sort's reversed last-resort whole-line compare, so the roll is stable
// across runs rather than dependent on the file's order.
pub fn slowest(text: &str) -> String {
    let mut rows: Vec<(i64, &str)> = text
        .lines()
        .filter(|l| field(l, 0).is_some_and(|f| f != "TOTAL"))
        .filter_map(|l| {
            let ms = field(l, 1)?;
            if ms.is_empty() || !ms.bytes().all(|c| c.is_ascii_digit()) {
                return None;
            }
            Some((ms.parse::<i64>().ok()?, field(l, 0)?))
        })
        .collect();
    rows.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(a.1)));
    rows.iter()
        .take(3)
        .enumerate()
        .map(|(i, (ms, name))| {
            format!("{}{} {}ms", if i > 0 { ", " } else { "" }, name, ms)
        })
        .collect()
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.timings_file) {
        Some(t) if !t.is_empty() => t,
        _ => return na("lead", LABEL, "no timings file — run the battery", trend),
    };
    let total = text
        .lines()
        .find(|l| field(l, 0) == Some("TOTAL"))
        .and_then(|l| field(l, 1))
        .filter(|f| !f.is_empty() && f.bytes().all(|c| c.is_ascii_digit()))
        .and_then(|f| f.parse::<i64>().ok());
    let total = match total {
        Some(t) => t,
        None => return na("lead", LABEL, "no TOTAL line", trend),
    };
    if trend {
        return Some(format!("gates {}ms\n", total));
    }

    let now = now_epoch();
    let mtime = std::fs::metadata(&ctx.timings_file)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(now);
    let secs = (now - mtime).max(0);
    Some(format!(
        "lead\t{}\ttotal {}ms; slowest {} (read {})\n",
        LABEL,
        total,
        slowest(&text),
        age_label(secs)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the four age bands and their boundaries, which is
    // what makes a stale reading legible rather than a raw second count
    #[test]
    fn the_reading_age_takes_the_coarsest_band_its_value_reaches() {
        assert_eq!(age_label(0), "0s ago");
        assert_eq!(age_label(89), "89s ago");
        assert_eq!(age_label(90), "1m ago");
        assert_eq!(age_label(5399), "89m ago");
        assert_eq!(age_label(5400), "1h ago");
        assert_eq!(age_label(172799), "47h ago");
        assert_eq!(age_label(172800), "2d ago");
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — TOTAL is excluded from the roll and a non-numeric
    // row is not a timing, so neither can take one of the three slots
    #[test]
    fn the_roll_is_the_three_slowest_named_gates_and_never_the_total() {
        let t = "TOTAL 9999\ncheck-a 100\ncheck-b 300\ncheck-c 200\ncheck-d x\n";
        assert_eq!(slowest(t), "check-b 300ms, check-c 200ms, check-a 100ms");
        assert_eq!(slowest("TOTAL 5\n"), "");
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — equal timings resolve by the reversed whole-line
    // compare GNU `sort -rn` applies, so the roll does not reorder between two identical runs
    #[test]
    fn equal_timings_order_deterministically() {
        let t = "check-a 100\ncheck-b 100\ncheck-c 100\n";
        assert_eq!(slowest(t), "check-c 100ms, check-b 100ms, check-a 100ms");
    }
}
