// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-queue-net-delta: the design-pending pool at the iteration-start commit against the worktree, two rows because one number would be gameable
use super::{read, Ctx};
use crate::{proc, programs};

// spec: drift-kit/SPEC.md §Bundled KPIs — both rows carry the same degrade text, because a pool
// this member cannot read leaves the entry axis and the weight axis equally unmeasured.
fn na(reason: &str, trend: bool) -> Option<String> {
    if trend {
        return Some(String::new());
    }
    Some(format!(
        "lead\tqueue net delta\t{}\nlead\tqueue carry weight\t{}\n",
        reason, reason
    ))
}

pub struct Pool {
    pub entries: Vec<(String, String)>,
    pub lines: i64,
}

fn is_slug_byte(c: u8) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-'
}

fn slug_shaped(s: &str) -> bool {
    let b = s.as_bytes();
    !b.is_empty() && (b[0].is_ascii_lowercase() || b[0].is_ascii_digit()) && b.iter().all(|c| is_slug_byte(*c))
}

// spec: queue-kit/SPEC.md §The queue format — the entry grammar re-implemented: `### <slug>`, or
// `#### <slug>` for a sub-task. A bare Done-style bullet is not an entry, which is what keeps the
// pool the design-pending one.
// spec: drift-kit/SPEC.md §Bundled KPIs — the baseline is a past revision, and a revision from before
// the heading grammar spells an entry as a bold-slug bullet, so that retired lead is read too
pub fn entry_slug(line: &str) -> Option<&str> {
    if let Some(rest) = line.strip_prefix("#### ").or_else(|| line.strip_prefix("### ")) {
        let slug = rest.trim_end_matches([' ', '\t']);
        return slug_shaped(slug).then_some(slug);
    }
    let after = line.trim_start_matches([' ', '\t']).strip_prefix('-')?;
    let inner = after.trim_start_matches([' ', '\t']);
    if inner.len() == after.len() {
        return None;
    }
    let inner = inner.strip_prefix("**")?;
    let end = inner.find("**")?;
    slug_shaped(&inner[..end]).then_some(&inner[..end])
}

// spec: drift-kit/SPEC.md §Bundled KPIs — one walk emitting the section-tagged slug of every entry
// heading plus a trailing line count, so the entry axis and the weight axis are read off the same
// parse. The unknown-heading reset is what scopes both to the pool's own sections.
pub fn pool(text: &str, deferred: &str, icebox: &str) -> Pool {
    let (dh, ih) = (format!("## {}", deferred), format!("## {}", icebox));
    let mut sec = "";
    let mut entries: Vec<(String, String)> = Vec::new();
    let mut lines = 0i64;
    for line in text.lines() {
        if line.starts_with("## ") {
            let t = line.trim_end();
            sec = if t == dh {
                "deferred"
            } else if !icebox.is_empty() && t == ih {
                "icebox"
            } else {
                ""
            };
            continue;
        }
        if sec.is_empty() {
            continue;
        }
        lines += 1;
        if let Some(slug) = entry_slug(line) {
            // spec: drift-kit/SPEC.md §Bundled KPIs — the pool is keyed by slug, the shape the
            // shell form's associative array had: a slug spelled twice is one entry and the last
            // spelling names its section.
            match entries.iter_mut().find(|(_, s)| s == slug) {
                Some(e) => e.0 = sec.to_string(),
                None => entries.push((sec.to_string(), slug.to_string())),
            }
        }
    }
    Pool { entries, lines }
}

fn find(p: &Pool, slug: &str) -> Option<String> {
    p.entries
        .iter()
        .find(|(_, s)| s == slug)
        .map(|(sec, _)| sec.clone())
}

// spec: drift-kit/SPEC.md §Bundled KPIs — *filed* and *drained* between two pools, one definition
// for both readers (§The queue-flow arm); an icebox move counts as neither, being compression.
pub fn flow(base: &Pool, now: &Pool) -> (i64, i64) {
    let filed = now
        .entries
        .iter()
        .filter(|(sec, s)| sec == "deferred" && find(base, s).is_none())
        .count() as i64;
    let drained = base
        .entries
        .iter()
        .filter(|(_, s)| find(now, s).is_none())
        .count() as i64;
    (filed, drained)
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.queue_file) {
        Some(t) => t,
        None => return na("n/a (no queue file)", trend),
    };
    if ctx.iteration_start.is_empty() {
        return na("n/a (no iteration baseline)", trend);
    }

    let spec = format!("{}:{}", ctx.iteration_start, ctx.queue_file);
    let base_text = proc::run(&programs::GIT, &["show", &spec])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()));
    let base_text = match base_text {
        Some(t) => t,
        None => {
            return na(
                &format!("n/a (queue absent at {})", ctx.iteration_start),
                trend,
            )
        }
    };

    let base = pool(&base_text, &ctx.deferred_section, &ctx.icebox_section);
    let now = pool(&text, &ctx.deferred_section, &ctx.icebox_section);
    let (filed, drained) = flow(&base, &now);

    let delta = filed - drained;
    let weight = now.lines - base.lines;
    if trend {
        return Some(format!("qnet {:+}\n", delta));
    }
    Some(format!(
        "lead\tqueue net delta\t{:+} ({} filed, {} drained since {})\nlead\tqueue carry weight\t{:+} lines ({} now, {} at {})\n",
        delta, filed, drained, ctx.iteration_start, weight, now.lines, base.lines, ctx.iteration_start
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the pool is the entries of the two configured
    // sections, and the line count is every line under them whatever its shape
    #[test]
    fn the_pool_reads_both_configured_sections_and_counts_all_their_lines() {
        let t = "## Active\n### live-one\n## Deferred\n### alpha\n\ncost line\n## Icebox\n### beta\n";
        let p = pool(t, "Deferred", "Icebox");
        assert_eq!(
            p.entries,
            vec![
                ("deferred".to_string(), "alpha".to_string()),
                ("icebox".to_string(), "beta".to_string())
            ]
        );
        assert_eq!(p.lines, 4);
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — an unset icebox knob means the pool is the deferred
    // section alone, so an icebox heading is an unknown one and its lines drop out
    #[test]
    fn an_unset_icebox_knob_leaves_that_section_out_of_the_pool() {
        let t = "## Deferred\n### alpha\n## Icebox\n### beta\n";
        let p = pool(t, "Deferred", "");
        assert_eq!(p.entries, vec![("deferred".to_string(), "alpha".to_string())]);
        assert_eq!(p.lines, 1);
    }

    // spec: drift-kit/SPEC.md §Bundled KPIs — an entry heading is an entry, and so is a past
    // revision's bold-slug bullet; a Done-style bare bullet and a non-slug heading are prose
    #[test]
    fn only_an_entry_heading_or_a_retired_bullet_lead_is_an_entry() {
        assert_eq!(entry_slug("### some-slug"), Some("some-slug"));
        assert_eq!(entry_slug("#### s2  "), Some("s2"));
        assert_eq!(entry_slug("- **some-slug** — prose"), Some("some-slug"));
        assert_eq!(entry_slug("- bare-slug"), None);
        assert_eq!(entry_slug("### Two Words"), None);
        assert_eq!(entry_slug("- **Two Words** — prose"), None);
        assert_eq!(entry_slug("-**no-space**"), None);
    }
}
