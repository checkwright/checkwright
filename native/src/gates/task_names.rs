// spec: queue-kit/SPEC.md §check-task-names — task entries lead with a unique kebab slug, done
// entries are bare slugs, every blocked-by resolves to a live task
use crate::queue;
use std::collections::HashMap;

// spec: queue-kit/SPEC.md §check-task-names — `\*\*[^*]*\*\*`: the bold lead-in as written,
// so an invalid slug is reported with the text the author actually typed
fn bold_run(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i + 3 < b.len() {
        if b[i] == b'*' && b[i + 1] == b'*' {
            let start = i + 2;
            let mut j = start;
            while j < b.len() && b[j] != b'*' {
                j += 1;
            }
            if j + 1 < b.len() && b[j] == b'*' && b[j + 1] == b'*' {
                return Some(&line[start..j]);
            }
        }
        i += 1;
    }
    None
}

fn is_bold_lead(line: &str) -> bool {
    queue::strip_bullet_lead(line)
        .map(|r| r.starts_with("**"))
        .unwrap_or(false)
}

// spec: queue-kit/SPEC.md §check-task-names — every `[blocked-by: <slug>]` on the line, in
// order; a marker followed by no valid slug is not a reference
fn blocked_refs(line: &str) -> Vec<String> {
    const M: &str = "[blocked-by:";
    let mut out = Vec::new();
    let b = line.as_bytes();
    let mut from = 0usize;
    while let Some(p) = line[from..].find(M) {
        let start = from + p;
        let mut j = start + M.len();
        while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
            j += 1;
        }
        let head = j;
        if j < b.len() && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit()) {
            j += 1;
            while j < b.len()
                && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-')
            {
                j += 1;
            }
            out.push(line[head..j].to_string());
            from = j;
            continue;
        }
        from = start + 1;
    }
    out
}

// spec: queue-kit/SPEC.md §check-task-names — a done entry is the bare slug only
fn bare_done_slug(line: &str) -> Option<&str> {
    let rest = queue::strip_bullet_lead(line)?;
    let trimmed = rest.trim_end_matches([' ', '\t']);
    let b = trimmed.as_bytes();
    if b.is_empty() || !(b[0].is_ascii_lowercase() || b[0].is_ascii_digit()) {
        return None;
    }
    if b[1..]
        .iter()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
    {
        Some(trimmed)
    } else {
        None
    }
}

pub fn run(args: &[String]) -> i32 {
    let sec = match queue::Sections::with_done() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-task-names: {}", e);
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-task-names: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-task-names: file not found: {}", file);
            return 2;
        }
    };

    let (mut missing, mut invalid, mut dup, mut baddone) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut live: HashMap<String, usize> = HashMap::new();
    let mut done: Vec<String> = Vec::new();
    let mut brefs: Vec<(String, usize)> = Vec::new();
    let mut cur = "";

    for (i, line) in text.lines().enumerate() {
        let fnr = i + 1;
        if sec.is_task(line) {
            cur = "task";
            continue;
        }
        if sec.is_done(line) {
            cur = "done";
            continue;
        }
        if queue::is_section_line(line) {
            cur = "other";
            continue;
        }

        if cur == "task" && queue::is_bullet(line) {
            let ind = queue::indent(line);
            let isbold = is_bold_lead(line);
            if ind == 0 || isbold {
                if let Some(slug) = queue::bullet_slug(line) {
                    match live.get(slug) {
                        Some(first) => dup.push(format!(
                            "{}:{}: {} (first seen at line {})",
                            file, fnr, slug, first
                        )),
                        None => {
                            live.insert(slug.to_string(), fnr);
                        }
                    }
                } else if isbold {
                    let what = bold_run(line).unwrap_or("(unparsable bold lead-in)");
                    invalid.push(format!("{}:{}: {}", file, fnr, what));
                } else {
                    missing.push(format!("{}:{}: {}", file, fnr, line));
                }
            }
            for r in blocked_refs(line) {
                brefs.push((r, fnr));
            }
            continue;
        }

        if cur == "done" && queue::is_bullet(line) {
            match bare_done_slug(line) {
                Some(d) => done.push(d.to_string()),
                None => baddone.push(format!("{}:{}: {}", file, fnr, line)),
            }
        }
    }

    let mut unresolved: Vec<String> = Vec::new();
    let mut stale: Vec<String> = Vec::new();
    for (r, ln) in &brefs {
        if live.contains_key(r) {
            continue;
        }
        let entry = format!("{}:{}: [blocked-by: {}]", file, ln, r);
        if done.contains(r) {
            stale.push(entry);
        } else {
            unresolved.push(entry);
        }
    }

    let total = missing.len()
        + invalid.len()
        + dup.len()
        + baddone.len()
        + unresolved.len()
        + stale.len();
    if total > 0 {
        let mut sep = false;
        let mut block = |head: &[&str], items: &Vec<String>, help: &[&str]| {
            if items.is_empty() {
                return;
            }
            if sep {
                println!();
            }
            sep = true;
            for h in head {
                println!("{}", h);
            }
            for x in items {
                println!("  {}", x);
            }
            for h in help {
                println!("{}", h);
            }
        };
        block(
            &["check-task-names: task entry without a bold kebab-case slug:"],
            &missing,
            &["  help: lead the entry with a slug — '- **the-slug** — <prose>'."],
        );
        block(
            &["check-task-names: task entry whose bold lead-in is not a valid slug:"],
            &invalid,
            &[
                "  help: a slug matches [a-z0-9][a-z0-9-]* (lowercase kebab-case); for a",
                "        non-task note, use a plain or italic indented bullet instead.",
            ],
        );
        block(
            &["check-task-names: duplicate slug (active + deferred + sub-tasks are one namespace):"],
            &dup,
            &["  help: rename one — a slug is a task's stable handle for its whole life."],
        );
        block(
            &["check-task-names: done entry that is not a bare slug:"],
            &baddone,
            &["  help: a done entry is the bare slug only — '- the-slug'; the story lives in git."],
        );
        block(
            &["check-task-names: blocked-by pointing at no live task:"],
            &unresolved,
            &["  help: name a live task (active or deferred); fix the slug or add the blocker."],
        );
        block(
            &["check-task-names: stale blocked-by pointing at a completed (done) task:"],
            &stale,
            &[
                "  help: the blocker is done — remove the now-stale [blocked-by:] tag (it alone",
                "        keeps the entry unpickable).",
            ],
        );
        return 1;
    }

    println!(
        "TASK-NAMES: clean ({} live slug(s) unique, {} done, all blockers resolve to live tasks in {})",
        live.len(),
        done.len(),
        file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_blocked_by_marker_needs_a_slug_to_be_a_reference() {
        assert_eq!(blocked_refs("- x [blocked-by: a-b] y"), vec!["a-b"]);
        assert_eq!(
            blocked_refs("[blocked-by:a] [blocked-by:   b]"),
            vec!["a", "b"]
        );
        assert!(blocked_refs("[blocked-by: ]").is_empty());
        assert!(blocked_refs("[blocked-by: Upper]").is_empty());
    }

    #[test]
    fn a_done_entry_is_the_bare_slug_only() {
        assert_eq!(bare_done_slug("- the-slug"), Some("the-slug"));
        assert_eq!(bare_done_slug("- the-slug   "), Some("the-slug"));
        assert_eq!(bare_done_slug("- the-slug and prose"), None);
        assert_eq!(bare_done_slug("- **bold**"), None);
    }

    #[test]
    fn an_invalid_bold_lead_in_reports_the_text_as_written() {
        assert_eq!(bold_run("- **Not A Slug** — x"), Some("Not A Slug"));
        assert_eq!(bold_run("- **"), None);
        assert!(is_bold_lead("  - **x**"));
        assert!(!is_bold_lead("  - x"));
    }
}
