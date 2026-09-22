// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-incident-recurrence: re-filings recorded by the queue's `[recurrence:]` tags, and the highest-count slug (queue-kit/SPEC.md §The tag algebra owns the grammar; the second implementation is accepted residual — drift-kit cannot source queue-kit's lib without a cross-kit cycle)
use super::{is_iso_day, na, read, Ctx};

const LABEL: &str = "incident recurrence";

pub struct Tally {
    pub total: usize,
    pub top_slug: String,
    pub top_count: usize,
}

// spec: queue-kit/SPEC.md §The tag algebra — the pairing re-implemented: an entry heading
// (`### <slug>` or `#### <slug>`) names the entry, and its tag line is the first non-blank line
// under it when that line opens with a bracket
fn heading_slug(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("#### ").or_else(|| line.strip_prefix("### "))?;
    Some(rest.trim_end())
}

// spec: drift-kit/SPEC.md §Bundled KPIs — the `[recurrence:]` array on an entry's tag line: one
// comma-separated date per re-filing, and a tag carrying no date contributes nothing.
fn array_dates(tag_line: &str) -> usize {
    let Some(open) = tag_line.find("[recurrence:") else { return 0 };
    let body = &tag_line[open + "[recurrence:".len()..];
    let body = &body[..body.find(']').unwrap_or(body.len())];
    body.split(',').filter(|t| is_iso_day(t.trim())).count()
}

pub fn tally(text: &str) -> Tally {
    let (mut total, mut top, mut slug) = (0usize, 0usize, String::new());
    let mut owner: Option<&str> = None;
    for line in text.lines() {
        if let Some(s) = heading_slug(line) {
            owner = Some(s);
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let Some(s) = owner.take() else { continue };
        if !line.starts_with('[') {
            continue;
        }
        let n = array_dates(line);
        if n == 0 {
            continue;
        }
        total += n;
        if n > top {
            top = n;
            slug = s.to_string();
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
        return na("lag", LABEL, "no recurrence tag in the queue", trend);
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

    // spec: drift-kit/SPEC.md §Bundled KPIs — the count is the dates in the array, so a tag whose
    // elements are not dates contributes nothing and cannot become the top slug, and a body line
    // is never a tag line
    #[test]
    fn only_dated_arrays_count_and_the_top_is_the_widest_one() {
        let t = "### alpha\n\n[cost: once/low] [recurrence: 2026-01-01, 2026-02-02]\n\nbody\n\
                 ### beta\n\n[recurrence: 2026-03-03]\n\
                 ### gamma\n\n[recurrence: pending]\n\
                 ### delta\n\nprose mentioning [recurrence: 2026-04-04]\n";
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
