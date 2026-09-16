// spec: queue-kit/SPEC.md §The queue-counts arm — the size of each set, one `<key><TAB><count>`
// line in configured order. One output grammar: `--by` changes the partition key and nothing else,
// and the argv tail otherwise stays the optional queue file the rule itself consumes.
use crate::queue::{self, Sections};

// spec: queue-kit/SPEC.md §The queue-counts arm — an absent input appears rather than vanishing, so
// a partition value is always present: a field tag's first lead-line value, a bare tag's own name,
// or `(none)`.
const NONE: &str = "(none)";

struct Args {
    file: String,
    by: Option<String>,
}

// spec: queue-kit/SPEC.md §The queue-counts arm — the optional `[queue-file]`, an argument the
// rule consumes and falls back to a knob for, so it ports unchanged (gate-sdk/SPEC.md §The
// non-gate arm's distinguishing test). The last positional wins, as the shell loop's did.
fn parse(args: &[String]) -> Result<Args, String> {
    let mut a = Args {
        file: String::new(),
        by: None,
    };
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--by" => {
                let v = args.get(i + 1).cloned().unwrap_or_default();
                if v.is_empty() || v.starts_with('-') {
                    return Err("--by needs a <tag>".to_string());
                }
                a.by = Some(v);
                i += 2;
            }
            other if other.starts_with('-') => return Err(format!("unknown option: {}", other)),
            other => {
                a.file = other.to_string();
                i += 1;
            }
        }
    }
    Ok(a)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let a = parse(args)?;
    let file = if a.file.is_empty() {
        queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?
    } else {
        a.file.clone()
    };
    let text =
        std::fs::read_to_string(&file).map_err(|e| format!("file not found: {}: {}", file, e))?;
    let sec = Sections::active_and_deferred()?;
    Ok(match &a.by {
        Some(tag) => render_by(&text, &sec, tag),
        None => render(&text, &sec),
    })
}

// spec: queue-kit/SPEC.md §The queue-counts arm — the counted unit is the top-level entry bullet,
// and a heading outside the task set closes the section it ends, which is what keeps Done out
// with no section name enumerated here
fn render(text: &str, sec: &Sections) -> String {
    let sections = sec.task_sections();
    let mut counts = vec![0usize; sections.len()];
    let mut cur: Option<usize> = None;
    for line in text.lines() {
        if queue::is_section_line(line) {
            cur = queue::heading_name(line).and_then(|h| sections.iter().position(|s| *s == h));
            continue;
        }
        if let Some(i) = cur {
            if queue::is_top_level_bullet(line) && queue::first_bold_slug(line).is_some() {
                counts[i] += 1;
            }
        }
    }
    let mut out = String::new();
    for (i, name) in sections.iter().enumerate() {
        out.push_str(&format!("{}\t{}\n", name, counts[i]));
    }
    out
}

// spec: queue-kit/SPEC.md §The queue-counts arm — the arm enumerates no tag name, so an unknown tag
// honestly reports every entry under `(none)` rather than refusing
fn value_of(line: &str, tag: &str) -> String {
    if let Some(v) = queue::field_tags(line, tag).first().and_then(|t| t.value) {
        return v.to_string();
    }
    if line.contains(&format!("[{}]", tag)) {
        return tag.to_string();
    }
    NONE.to_string()
}

// spec: queue-kit/SPEC.md §The queue-counts arm — the same counted unit under a compound
// `<section>/<value>` key: sections in configured order, values by first appearance in queue order,
// and a section with no entries has no partition and emits no line.
fn render_by(text: &str, sec: &Sections, tag: &str) -> String {
    let sections = sec.task_sections();
    let mut buckets: Vec<Vec<(String, usize)>> = vec![Vec::new(); sections.len()];
    let mut cur: Option<usize> = None;
    for line in text.lines() {
        if queue::is_section_line(line) {
            cur = queue::heading_name(line).and_then(|h| sections.iter().position(|s| *s == h));
            continue;
        }
        if let Some(i) = cur {
            if queue::is_top_level_bullet(line) && queue::first_bold_slug(line).is_some() {
                let value = value_of(line, tag);
                match buckets[i].iter_mut().find(|(k, _)| *k == value) {
                    Some((_, n)) => *n += 1,
                    None => buckets[i].push((value, 1)),
                }
            }
        }
    }
    let mut out = String::new();
    for (i, name) in sections.iter().enumerate() {
        for (value, n) in &buckets[i] {
            out.push_str(&format!("{}/{}\t{}\n", name, value, n));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const Q: &str = "\
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** [cost: event/low] — do a thing.
  - **not-an-entry** — an indented bullet is body, not a second entry.
- **feat-b** [design-pending] — do another.

## Technical Debt

## Deferred

- **defer-a** [cost: once/high] [design-pending] — later.

## Chill

- **chill-a** — much later.

## Done

- done-a
- **done-b** — a Done entry shaped like an active one, to prove Done is out.

## Lessons Learned

- **l1** [attend] — a lesson is not a task section.
";

    fn sections(icebox: &str) -> Sections {
        Sections {
            active: vec!["New Features".into(), "Technical Debt".into()],
            deferred: "Deferred".into(),
            icebox: icebox.into(),
            done: String::new(),
        }
    }

    // spec: queue-kit/SPEC.md §The queue-counts arm — the section set is derived, so a configured
    // icebox is in as a live task section and Done is out as not one
    #[test]
    fn the_emitted_set_is_the_configured_task_sections_in_order() {
        assert_eq!(
            render(Q, &sections("Chill")),
            "New Features\t2\nTechnical Debt\t0\nDeferred\t1\nChill\t1\n"
        );
        assert_eq!(
            render(Q, &sections("")),
            "New Features\t2\nTechnical Debt\t0\nDeferred\t1\n"
        );
    }

    // spec: queue-kit/SPEC.md §The queue-counts arm — nothing here enumerates a section name, so a
    // consumer who renamed their sections gets their own names back
    #[test]
    fn renamed_sections_come_back_renamed() {
        let sec = Sections {
            active: vec!["Work".into()],
            deferred: "Someday".into(),
            icebox: String::new(),
            done: String::new(),
        };
        let q = "## Work\n\n- **w1** — one.\n\n## Someday\n\n- **s1** — two.\n- **s2** — three.\n\n## Done\n\n- **d1** — not counted.\n";
        assert_eq!(render(q, &sec), "Work\t1\nSomeday\t2\n");
    }

    // spec: queue-kit/SPEC.md §The queue-counts arm — an unknown option is a refusal rather than a
    // positional, so a mistyped flag cannot become a queue-file path
    #[test]
    fn an_unknown_option_is_refused_and_the_last_positional_wins() {
        let a = parse(&[]).expect("parse failed");
        assert_eq!((a.file.as_str(), a.by), ("", None));
        let a = parse(&["a.md".to_string()]).expect("parse failed");
        assert_eq!(a.file, "a.md");
        let a = parse(&["a.md".to_string(), "b.md".to_string()]).expect("parse failed");
        assert_eq!(a.file, "b.md");
        assert!(parse(&["--nope".to_string()]).is_err());
        let a = parse(&["--by".to_string(), "cost".to_string(), "a.md".to_string()])
            .expect("parse failed");
        assert_eq!((a.file.as_str(), a.by.as_deref()), ("a.md", Some("cost")));
        assert!(parse(&["--by".to_string()]).is_err());
        assert!(parse(&["--by".to_string(), "--cost".to_string()]).is_err());
    }

    // spec: queue-kit/SPEC.md §The queue-counts arm — the value rule: a field tag yields its first
    // lead-line value, a bare tag yields its own name, and an entry carrying neither yields
    // `(none)`, ordered by first appearance with an empty section emitting nothing.
    #[test]
    fn the_by_key_compounds_the_section_with_the_tags_value() {
        assert_eq!(
            render_by(Q, &sections("Chill"), "cost"),
            "New Features/event/low\t1\nNew Features/(none)\t1\nDeferred/once/high\t1\nChill/(none)\t1\n"
        );
        assert_eq!(
            render_by(Q, &sections("Chill"), "design-pending"),
            "New Features/(none)\t1\nNew Features/design-pending\t1\nDeferred/design-pending\t1\nChill/(none)\t1\n"
        );
    }

    // spec: queue-kit/SPEC.md §The queue-counts arm — the arm enumerates no tag name, so an unknown
    // tag reports every entry under `(none)` rather than refusing
    #[test]
    fn an_unknown_tag_reports_every_entry_under_none() {
        assert_eq!(
            render_by(Q, &sections(""), "no-such-tag"),
            "New Features/(none)\t2\nDeferred/(none)\t1\n"
        );
    }
}
