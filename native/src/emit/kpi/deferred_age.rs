// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-deferred-age: age of the oldest defer date in the deferred section, the Surfaced mark where one exists and the Filed provenance date otherwise (queue-kit/SPEC.md §The queue format owns the definition; the second implementation is accepted residual — drift-kit cannot source queue-kit's lib without a cross-kit cycle)
use super::{date_epoch, is_iso_day, na, now_epoch, read, section_lines, Ctx};

const LABEL: &str = "deferred age";

fn marked_date(line: &str, mark: &str) -> Option<String> {
    let mut from = 0usize;
    while let Some(hit) = line[from..].find(mark) {
        let at = from + hit + mark.len();
        if let Some(d) = line.get(at..at + 10) {
            if is_iso_day(d) {
                return Some(d.to_string());
            }
        }
        from = at;
    }
    None
}

fn is_entry_lead(line: &str) -> bool {
    line.trim_start()
        .strip_prefix("- **")
        .and_then(|r| r.bytes().next())
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}

// spec: queue-kit/SPEC.md §The queue format — one defer date per entry, read off its body lines:
// the first Surfaced mark, else the first Filed mark; a line outside any entry dates nothing
pub fn defer_dates(lines: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut surfaced: Option<String> = None;
    let mut filed: Option<String> = None;
    let mut open = false;
    for line in lines {
        if is_entry_lead(line) {
            if let Some(d) = surfaced.take().or(filed.take()) {
                out.push(d);
            }
            open = true;
            continue;
        }
        if !open {
            continue;
        }
        if surfaced.is_none() {
            surfaced = marked_date(line, "Surfaced ");
        }
        if filed.is_none() {
            filed = marked_date(line, "Filed ");
        }
    }
    if let Some(d) = surfaced.or(filed) {
        out.push(d);
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

    // spec: queue-kit/SPEC.md §The queue format — Surfaced outranks an older Filed on the same entry,
    // an entry with Filed alone dates by it, and a preamble line or a later repeat dates nothing
    #[test]
    fn each_entry_contributes_its_surfaced_else_filed_date() {
        let lines = vec![
            "  Preamble quoting Filed 2026-01-01.",
            "- **resurfaced** [cost: event/low] — lead",
            "  Filed 2026-07-07 by close.",
            "  Surfaced 2026-08-01 at build.",
            "- **filed-only** — lead",
            "  Filed 2026-07-20 by scope.",
            "  Filed 2026-06-01 quoted later.",
            "  Filed not-a-date",
            "- **undated** — lead",
            "  no mark here",
        ];
        assert_eq!(defer_dates(&lines), vec!["2026-07-20", "2026-08-01"]);
    }
}
