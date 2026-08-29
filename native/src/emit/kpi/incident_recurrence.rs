// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-incident-recurrence: re-filings recorded by the queue's `recurrence:` declarations, and the highest-count slug (queue-kit/SPEC.md §The tag algebra owns the grammar; the second implementation is accepted residual — drift-kit cannot source queue-kit's lib without a cross-kit cycle)
use super::{is_iso_day, na, read, Ctx};

const LABEL: &str = "incident recurrence";

pub struct Tally {
    pub total: usize,
    pub top_slug: String,
    pub top_count: usize,
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the declaration is self-slug-bearing and lives on a line
// of its own, so one anchored scan reads it with no entry-boundary parsing: `recurrence: <slug>`
// then one date per re-filing, and a declaration carrying no date contributes nothing.
pub fn tally(text: &str) -> Tally {
    let (mut total, mut top, mut slug) = (0usize, 0usize, String::new());
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 3 || f[0] != "recurrence:" {
            continue;
        }
        let n = f[2..].iter().filter(|t| is_iso_day(t)).count();
        if n == 0 {
            continue;
        }
        total += n;
        if n > top {
            top = n;
            slug = f[1].to_string();
        }
    }
    Tally {
        total,
        top_slug: if slug.is_empty() { "-".to_string() } else { slug },
        top_count: top,
    }
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.queue_file) {
        Some(t) => t,
        None => return na("lag", LABEL, "no queue file", trend),
    };
    let t = tally(&text);
    if t.total == 0 {
        return na("lag", LABEL, "no recurrence declaration in the queue", trend);
    }
    if trend {
        return Some(format!("recur {}\n", t.total));
    }
    Some(format!(
        "lag\t{}\t{} re-filing(s) recorded; highest {} at {} (captured filings only — a lower bound)\n",
        LABEL, t.total, t.top_slug, t.top_count
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the count is the dates on the line, so a declaration
    // whose fields are not dates contributes nothing and cannot become the top slug
    #[test]
    fn only_dated_declarations_count_and_the_top_is_the_widest_one() {
        let t = "recurrence: alpha 2026-01-01 2026-02-02\n\
                 recurrence: beta 2026-03-03\n\
                 recurrence: gamma pending\n\
                 prose mentioning recurrence: delta 2026-04-04\n";
        let r = tally(t);
        assert_eq!(r.total, 3);
        assert_eq!(r.top_slug, "alpha");
        assert_eq!(r.top_count, 2);
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — an empty tally names no slug rather than an empty
    // one, which is what the `-` placeholder is for
    #[test]
    fn an_empty_tally_names_a_placeholder_slug() {
        let r = tally("nothing here\n");
        assert_eq!(r.total, 0);
        assert_eq!(r.top_slug, "-");
        assert_eq!(r.top_count, 0);
    }
}
