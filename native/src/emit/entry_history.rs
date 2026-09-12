// spec: queue-kit/SPEC.md §check-queue-entry-budget — the entry-history arm: the commits at which
// one entry's counted extent fell, reported and never judged, with no exit-1 path at all
use crate::gates::queue_entry_budget::{self, Sec};
use crate::queue;

const USAGE: &str = "\
usage: --emit entry-history <slug> [queue-file]
  one row per commit in which the entry's COUNTED extent fell, newest first:
  \"<commit>  <before> -> <after>  <subject>\". Advisory — it reports commits and
  issues no verdict; whether a fall was an answer, a relocation or a discard is
  read off the commit by the reader who opens it.
";

// spec: queue-kit/SPEC.md §check-queue-entry-budget — git's own record separator between the two
// `--format` fields: a subject may carry anything a shell quotes, and a unit separator cannot
const FIELD: char = '\u{1f}';

struct Row {
    commit: String,
    before: usize,
    after: usize,
    subject: String,
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let mut slug = String::new();
    let mut file = String::new();
    for a in args {
        match a.as_str() {
            "-h" | "--help" => return Ok(USAGE.to_string()),
            other if other.starts_with('-') => return Err(format!("unknown option: {}", other)),
            other if slug.is_empty() => slug = other.to_string(),
            other => file = other.to_string(),
        }
    }
    if slug.is_empty() {
        return Err("entry-history needs a <slug>".to_string());
    }
    let sec_cfg = queue::Sections::active_and_deferred()?;
    let file = if file.is_empty() {
        queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?
    } else {
        file
    };
    let top = crate::walk::toplevel()?;
    let git = crate::history::Git { top: top.clone() };
    let log = git
        .read(&["log", &format!("--format=%H{}%s", FIELD), "--", &file])
        .ok_or_else(|| format!("no committed history for {}", file))?;
    let mut blobs = crate::history::Blobs::open(&top)?;

    let mut rows: Vec<Row> = Vec::new();
    let mut prev: Option<Row> = None;
    let mut walked = 0usize;
    let mut filing = String::new();
    for line in log.lines() {
        let (commit, subject) = match line.split_once(FIELD) {
            Some(p) => p,
            None => continue,
        };
        let count = match blobs.at(commit, &file)? {
            Some(text) => count_of(&text, &sec_cfg, &slug),
            None => None,
        };
        // spec: queue-kit/SPEC.md §check-queue-entry-budget — the bound: the first commit carrying
        // no entry for the slug ends the walk, and a slug absent at the newest commit ends it at
        // once, which the usage error below reports rather than an empty report
        let count = match count {
            Some(k) => k,
            None => break,
        };
        walked += 1;
        filing = commit.to_string();
        if let Some(newer) = prev.take() {
            if newer.before < count {
                rows.push(Row {
                    commit: newer.commit.clone(),
                    before: count,
                    after: newer.before,
                    subject: newer.subject.clone(),
                });
            }
        }
        prev = Some(Row {
            commit: commit.to_string(),
            before: count,
            after: count,
            subject: subject.to_string(),
        });
    }
    if walked == 0 {
        return Err(format!(
            "no entry for {} in the newest commit touching {} — the arm reads a live entry",
            slug, file
        ));
    }

    let mut out = format!(
        "ENTRY-HISTORY: {} — {} commit(s) walked back to its filing commit {}\n",
        slug,
        walked,
        short(&filing)
    );
    out.push_str("advisory: it reports commits and issues no verdict\n");
    out.push('\n');
    if rows.is_empty() {
        out.push_str("  no commit in range reduced this entry's counted extent\n");
        return Ok(out);
    }
    for r in &rows {
        out.push_str(&format!(
            "  {}  {} -> {}  {}\n",
            short(&r.commit),
            r.before,
            r.after,
            r.subject
        ));
    }
    Ok(out)
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the count is assertion A's own, read off the
// gate's walk: a change to what counts moves the cap and this report together
fn count_of(text: &str, sec_cfg: &queue::Sections, slug: &str) -> Option<usize> {
    queue_entry_budget::walk(text, sec_cfg)
        .entries
        .into_iter()
        .find(|e| e.slug == slug && e.sec != Sec::Other)
        .map(|e| e.count)
}

fn short(sha: &str) -> String {
    sha.chars().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::{count_of, short};
    use crate::queue::Sections;

    fn sections() -> Sections {
        Sections {
            active: vec!["New Features".to_string()],
            deferred: "Deferred".to_string(),
            icebox: "Icebox".to_string(),
            done: String::new(),
        }
    }

    const Q: &str = "\
## New Features

- **live-one** — a promoted entry
  a second line

## Deferred

- **def-one** [cost: event/low] — a deferred entry
  a second line
  a third line
  recurrence: def-one 2026-01-01

## Done

- def-gone
";

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — the arm reads the cap's own count, which
    // is the extent less the declaration discount: five lines of extent, one `recurrence:` line
    // discounted, four counted.
    #[test]
    fn the_count_is_the_caps_count_and_not_the_extent() {
        assert_eq!(count_of(Q, &sections(), "def-one"), Some(4));
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — an entry promoted out of the deferred
    // pool is still measured, so a reader following one entry's history crosses the promotion
    #[test]
    fn a_promoted_entry_is_measured_where_it_now_stands() {
        assert_eq!(count_of(Q, &sections(), "live-one"), Some(3));
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — a bare slug under Done is not an entry
    // the cap measures, so it reads as absence and ends the walk
    #[test]
    fn a_done_slug_is_absence() {
        assert_eq!(count_of(Q, &sections(), "def-gone"), None);
        assert_eq!(count_of(Q, &sections(), "no-such-slug"), None);
    }

    #[test]
    fn the_commit_is_reported_short() {
        assert_eq!(short("0123456789abcdef"), "01234567");
    }
}
