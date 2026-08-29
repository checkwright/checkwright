// spec: drift-kit/SPEC.md §Bundled KPIs — kpi-task-split: feature↔debt split of the queue's Done slugs
use super::{na, read, section_lines, Ctx};
use crate::proc;

const LABEL: &str = "task split (feat/debt)";

// spec: drift-kit/SPEC.md §Bundled KPIs — the Done section's bare-bullet grammar: a lone slug on
// its own line, which is what a completed entry is compressed to; a prose bullet is not one.
fn bare_slug(line: &str) -> Option<&str> {
    let rest = line.strip_prefix('-')?;
    let slug = rest.trim_start_matches([' ', '\t']);
    if slug.len() == rest.len() {
        return None;
    }
    let slug = slug.trim_end_matches([' ', '\t']);
    let mut cs = slug.chars();
    let first = cs.next()?;
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return None;
    }
    if !cs.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return None;
    }
    Some(slug)
}

pub fn run(ctx: &Ctx, trend: bool) -> Option<String> {
    let text = match read(&ctx.queue_file) {
        Some(t) => t,
        None => return na("lead", LABEL, "no queue file", trend),
    };

    let mut slugs: Vec<&str> = section_lines(&text, &ctx.done_section)
        .into_iter()
        .filter_map(bare_slug)
        .collect();
    slugs.sort_unstable();
    slugs.dedup();

    let total = slugs.len();
    if total == 0 {
        return na("lead", LABEL, "nothing Done this iteration", trend);
    }

    let (mut feat, mut debt) = (0usize, 0usize);
    for s in &slugs {
        let subj = proc::run("git", &["log", "-1", "--format=%s", &format!("--grep={}", s)])
            .ok()
            .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
            .unwrap_or_default();
        let subj = subj.trim_end_matches('\n');
        if subj.starts_with("feat") {
            feat += 1;
        } else if subj.starts_with("fix") || subj.starts_with("refactor") {
            debt += 1;
        }
    }

    if trend {
        return Some(format!("split {}f/{}d\n", feat, debt));
    }
    let unclassified = total - feat - debt;
    let mut value = format!("{}f / {}d of {} done", feat, debt, total);
    if unclassified > 0 {
        value.push_str(&format!(" ({} unclassified)", unclassified));
    }
    Some(format!("lead\t{}\t{}\n", LABEL, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: drift-kit/SPEC.md §Bundled KPIs — the Done grammar is the bare-slug line alone, so an
    // active-section entry and a nested continuation do not enter the split
    #[test]
    fn only_a_bare_slug_bullet_is_a_done_entry() {
        assert_eq!(bare_slug("- some-slug"), Some("some-slug"));
        assert_eq!(bare_slug("- some-slug   "), Some("some-slug"));
        assert_eq!(bare_slug("-\tslug2"), Some("slug2"));
        assert_eq!(bare_slug("- **bold-slug** — prose"), None);
        assert_eq!(bare_slug("-no-space"), None);
        assert_eq!(bare_slug("  - indented"), None);
        assert_eq!(bare_slug("- Two words"), None);
    }
}
