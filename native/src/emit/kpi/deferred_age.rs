// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-deferred-age: age of the oldest defer date in the deferred section, the Surfaced mark where one exists and the Filed provenance date otherwise (queue-kit/SPEC.md §The queue format owns the definition; the second implementation is accepted residual — drift-kit cannot source queue-kit's lib without a cross-kit cycle)
use super::{date_epoch, is_iso_day, na, now_epoch, read, section_lines, Ctx};

const LABEL: &str = "deferred age";

// spec: drift-kit/SPEC.md §Bundled KPIs — `grep -o '(Surfaced|Filed) <iso-day>'` then the date
// field: both marks read, anywhere on a line and more than once on one.
pub fn defer_dates(lines: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in lines {
        for mark in ["Surfaced ", "Filed "] {
            let mut from = 0usize;
            while let Some(hit) = line[from..].find(mark) {
                let at = from + hit + mark.len();
                if line.len() >= at + 10 && is_iso_day(&line[at..at + 10]) {
                    out.push(line[at..at + 10].to_string());
                }
                from = at;
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.queue_file) {
        Some(t) => t,
        None => return na("lead", LABEL, "no queue file", trend),
    };
    let dates = defer_dates(&section_lines(&text, &ctx.deferred_section));
    if dates.is_empty() {
        return na("lead", LABEL, "no defer date", trend);
    }

    let now = now_epoch();
    let mut oldest_ts = now;
    let mut oldest_date = String::new();
    for d in &dates {
        if let Some(ts) = date_epoch(d) {
            if ts < oldest_ts {
                oldest_ts = ts;
                oldest_date = d.clone();
            }
        }
    }
    if oldest_date.is_empty() {
        return na("lead", LABEL, "no parseable defer date", trend);
    }

    let days = (now - oldest_ts) / 86400;
    if trend {
        return Some(format!("defer {}d\n", days));
    }
    Some(format!(
        "lead\t{}\toldest {}d (deferred {})\n",
        LABEL, days, oldest_date
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — both marks count and the set is deduplicated, so an
    // entry carrying Filed and Surfaced contributes two dates and a repeated one contributes once
    #[test]
    fn both_defer_marks_are_read_and_the_set_is_deduplicated() {
        let lines = vec![
            "  Filed 2026-07-07, Surfaced 2026-08-01.",
            "  Filed 2026-07-07 again",
            "  Filed not-a-date",
        ];
        assert_eq!(defer_dates(&lines), vec!["2026-07-07", "2026-08-01"]);
    }
}
