// spec: queue-kit/SPEC.md §check-queue-entry-budget — the entry-history arm: the commits at which
// one entry's counted extent fell, reported and never judged, with no exit-1 path at all
use crate::emit::queue_migrate;
use crate::gates::queue_entry_budget;
use crate::queue;

pub const USAGE: &str = "\
usage: --emit entry-history <slug> [queue-file]
  one row per commit in which the entry's COUNTED extent fell, newest first:
  \"<commit>  <before> -> <after>  <subject>\". Advisory — it reports commits and
  issues no verdict; whether a fall was an answer, a relocation or a discard is
  read off the commit by the reader who opens it.
  <slug> may be live or retired; a departed entry's header names the commit it
  departed at, and a slug in neither set is refused.
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
            other if other.starts_with('-') => {
                return Err(format!("unknown option: {}\n{}", other, USAGE))
            }
            other if slug.is_empty() => slug = other.to_string(),
            other => file = other.to_string(),
        }
    }
    if slug.is_empty() {
        return Err(format!("entry-history needs a <slug>\n{}", USAGE));
    }
    let sec_cfg = queue::Sections::active_and_deferred()?;
    let unit = queue_entry_budget::display_unit(queue_entry_budget::cap()?);
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

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — the addressable domain is the live slugs
    // plus the retired ones, and that membership is the bound the back-search would otherwise lack:
    // it is settled from one history pass before a blob is read.
    let text = std::fs::read_to_string(&file).unwrap_or_default();
    let live = queue::live_slugs(&text, &sec_cfg);
    if !live.contains(&slug) && !queue::retired_set(&file, &live).contains(&slug) {
        return Err(format!("not a live or retired slug: {}", slug));
    }

    let mut blobs = crate::history::Blobs::open(&top)?;

    let mut rows: Vec<Row> = Vec::new();
    let mut prev: Option<Row> = None;
    let mut walked = 0usize;
    let mut filing = String::new();
    let mut departure: Option<(String, String)> = None;
    for line in log.lines() {
        let (commit, subject) = match line.split_once(FIELD) {
            Some(p) => p,
            None => continue,
        };
        let count = match blobs.at(commit, &file)? {
            Some(text) => count_of(&queue_migrate::heading_form(&text, &sec_cfg), &sec_cfg, &slug, unit),
            None => None,
        };
        // spec: queue-kit/SPEC.md §check-queue-entry-budget — the bound, and the departure commit:
        // the post-disposition run precedes the walk, and its oldest member is the commit one step
        // newer than the last live one.
        let count = match count {
            Some(k) => k,
            None => {
                if walked > 0 {
                    break;
                }
                departure = Some((commit.to_string(), subject.to_string()));
                continue;
            }
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
    // spec: queue-kit/SPEC.md §check-queue-entry-budget — the retired set admits a slug on
    // entry shape alone, so a heading or bullet that never stood in a task section is addressable and
    // has no counted history; that is the one case the membership bound does not shorten
    if walked == 0 {
        return Err(format!(
            "no commit touching {} carries a counted entry for {}",
            file, slug
        ));
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — the departure is a header fact and never
    // a fall row: absence is not a count of zero, so reporting it as a decrease would invent a
    // measurement
    let mut out = match &departure {
        Some((commit, subject)) => format!(
            "ENTRY-HISTORY: {} — departed at {} ({}); {} commit(s) walked back to its filing commit {}\n",
            slug,
            short(commit),
            subject,
            walked,
            short(&filing)
        ),
        None => format!(
            "ENTRY-HISTORY: {} — {} commit(s) walked back to its filing commit {}\n",
            slug,
            walked,
            short(&filing)
        ),
    };
    out.push_str("advisory: it reports commits and issues no verdict\n");
    out.push('\n');
    if rows.is_empty() {
        out.push_str("  no commit in range reduced this entry's counted extent\n");
        return Ok(out);
    }
    for r in &rows {
        out.push_str(&format!(
            "  {}  {}{} -> {}{}  {}\n",
            short(&r.commit),
            r.before,
            unit.suffix(),
            r.after,
            unit.suffix(),
            r.subject
        ));
    }
    Ok(out)
}

// spec: queue-kit/SPEC.md §check-queue-entry-budget — the count is assertion A's own, read off the
// gate's walk: a change to what counts moves the cap and this report together
fn count_of(text: &str, sec_cfg: &queue::Sections, slug: &str, unit: queue::Unit) -> Option<usize> {
    queue_entry_budget::walk(text, sec_cfg)
        .entries
        .into_iter()
        .find(|e| e.slug == slug)
        .map(|e| e.size(unit))
}

fn short(sha: &str) -> String {
    sha.chars().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::{count_of, emit, short, USAGE};
    use crate::queue::Unit::{self, Cp, Lines};

    // spec: gate-sdk/SPEC.md §The non-gate arm — `--help` is no per-arm help flag: it is the
    // member's shape refusal, which carries the usage
    #[test]
    fn help_is_a_refusal_carrying_the_usage() {
        for help in ["--help", "-h"] {
            let err = emit(&[help.to_string()]).expect_err("help must refuse");
            assert!(err.contains(help) && err.contains(USAGE), "{}", err);
        }
    }

    fn count(q: &str, slug: &str, unit: Unit) -> Option<usize> {
        count_of(q, &sections(), slug, unit)
    }
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

### live-one

a promoted entry

## Deferred

### def-one

[cost: event/low] [recurrence: 2026-01-01]

a deferred entry

a second paragraph

## Done

- def-gone
";

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — the arm reads the cap's own count: the
    // whole extent in lines, and in code points the four content lines and their three breaks.
    #[test]
    fn the_count_is_the_caps_count() {
        assert_eq!(count(Q, "def-one", Lines), Some(8));
        let cp = "### def-one".len() + 1 + "[cost: event/low] [recurrence: 2026-01-01]".len() + 1 + 16 + 1 + 18;
        assert_eq!(count(Q, "def-one", Cp), Some(cp));
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — an entry promoted out of the deferred
    // pool is still measured, so a reader following one entry's history crosses the promotion
    #[test]
    fn a_promoted_entry_is_measured_where_it_now_stands() {
        assert_eq!(count(Q, "live-one", Lines), Some(4));
    }

    // spec: queue-kit/SPEC.md §check-queue-entry-budget — a bare slug under Done is not an entry
    // the cap measures, so it reads as absence and ends the walk
    #[test]
    fn a_done_slug_is_absence() {
        assert_eq!(count(Q, "def-gone", Lines), None);
        assert_eq!(count(Q, "no-such-slug", Cp), None);
    }

    #[test]
    fn the_commit_is_reported_short() {
        assert_eq!(short("0123456789abcdef"), "01234567");
    }
}
