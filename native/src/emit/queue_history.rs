// spec: queue-kit/SPEC.md §The queue-history arm — one entry's section transitions, oldest first, a
// report with no verdict and no exit-1 path
use crate::queue;

pub const USAGE: &str = "\
usage: --emit queue-history <slug> [queue-file]
  the slug on its own line, then one row per commit at which the entry's place
  changed, oldest first: \"<YYYY-MM-DD> <short-sha> <from> -> <to>  <subject>\".
  A place is the task section heading the entry, the done section, or (absent).
  <slug> may be live, done or retired; a slug in none of those is refused.
";

pub const KNOBS: &[&str] = &[
    "QUEUE_KIT_QUEUE_FILE",
    "QUEUE_KIT_ACTIVE_SECTIONS",
    "QUEUE_KIT_DEFERRED_SECTION",
    "QUEUE_KIT_ICEBOX_SECTION",
    "QUEUE_KIT_DONE_SECTION",
];

pub fn emit(args: &[String]) -> Result<String, String> {
    let mut slug = String::new();
    let mut file = String::new();
    for a in args {
        match a.as_str() {
            other if other.starts_with('-') => return Err(format!("unknown option: {}\n{}", other, USAGE)),
            other if slug.is_empty() => slug = other.to_string(),
            other if file.is_empty() => file = other.to_string(),
            other => return Err(format!("unexpected argument: {}\n{}", other, USAGE)),
        }
    }
    if slug.is_empty() {
        return Err(format!("queue-history needs a <slug>\n{}", USAGE));
    }
    let sec = queue::Sections::with_done()?;
    let file = if file.is_empty() { queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")? } else { file };
    Ok(render(&slug, &queue::transitions(&file, &slug, &sec)?))
}

fn render(slug: &str, rows: &[queue::Transition]) -> String {
    let mut out = format!("{}\n", slug);
    for t in rows {
        let short: String = t.commit.chars().take(8).collect();
        out.push_str(&format!("  {} {} {} -> {}  {}\n", t.date, short, t.from, t.to, t.subject));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The non-gate arm — `--help` is the member's shape refusal carrying the usage
    #[test]
    fn help_and_a_missing_slug_are_refusals_carrying_the_usage() {
        for argv in [vec!["--help".to_string()], vec![], vec!["a".into(), "b".into(), "c".into()]] {
            let err = emit(&argv).expect_err("a usage error must refuse");
            assert!(err.contains(USAGE), "{}", err);
        }
    }

    // spec: queue-kit/SPEC.md §The queue-history arm — the slug alone, then one indented row per
    // transition, oldest first, the commit shortened
    #[test]
    fn the_report_is_the_slug_then_one_row_per_transition() {
        let row = |c: &str, d: &str, from: &str, to: &str, s: &str| queue::Transition {
            commit: c.into(),
            date: d.into(),
            subject: s.into(),
            from: from.into(),
            to: to.into(),
            prior: None,
        };
        let rows = vec![
            row("0123456789ab", "2026-01-01", "(absent)", "Deferred", "file it"),
            row("fedcba987654", "2026-02-01", "Deferred", "New Features", "promote it"),
        ];
        assert_eq!(
            render("a-slug", &rows),
            "a-slug\n  2026-01-01 01234567 (absent) -> Deferred  file it\n  \
             2026-02-01 fedcba98 Deferred -> New Features  promote it\n"
        );
        assert_eq!(render("lone", &[]), "lone\n");
    }
}
