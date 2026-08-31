// spec: queue-kit/SPEC.md §The queue-counts arm — the size of each task section, one
// `<section-name><TAB><count>` line in configured order. No flags and no modes: the shell tool's
// whole argv was an optional queue file, and the rule itself consumes it.
use crate::queue::{self, Sections};

// spec: queue-kit/SPEC.md §The queue-counts arm — the optional `[queue-file]`, an argument the
// rule consumes and falls back to a knob for, so it ports unchanged (gate-sdk/SPEC.md §The
// non-gate arm's distinguishing test). The last positional wins, as the shell loop's did.
fn parse(args: &[String]) -> Result<String, String> {
    let mut file = String::new();
    for a in args {
        if a.starts_with('-') {
            return Err(format!("unknown option: {}", a));
        }
        file = a.clone();
    }
    Ok(file)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let named = parse(args)?;
    let file = if named.is_empty() {
        queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?
    } else {
        named
    };
    let text =
        std::fs::read_to_string(&file).map_err(|e| format!("file not found: {}: {}", file, e))?;
    Ok(render(&text, &Sections::active_and_deferred()?))
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

#[cfg(test)]
mod tests {
    use super::*;

    const Q: &str = "\
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** — do a thing.
  - **not-an-entry** — an indented bullet is body, not a second entry.
- **feat-b** — do another.

## Technical Debt

## Deferred

- **defer-a** — later.

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
        assert_eq!(parse(&[]), Ok(String::new()));
        assert_eq!(parse(&["a.md".to_string()]), Ok("a.md".to_string()));
        assert_eq!(
            parse(&["a.md".to_string(), "b.md".to_string()]),
            Ok("b.md".to_string())
        );
        assert!(parse(&["--nope".to_string()]).is_err());
    }
}
