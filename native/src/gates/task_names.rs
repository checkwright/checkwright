// spec: queue-kit/SPEC.md §check-task-names — task entries are headings with a unique kebab slug,
// done entries are bare slugs, every blocked-by resolves to a live task, references are links
use crate::queue;
use std::collections::HashMap;

// spec: queue-kit/SPEC.md §check-task-names — assertion R's single-backtick token: a double-backtick
// span is a quoted literal, not a citation
fn single_backtick_slugs(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    queue::backtick_slugs(line)
        .into_iter()
        .filter(|&(s, e)| !(s >= 2 && b[s - 2] == b'`') && b.get(e + 1) != Some(&b'`'))
        .map(|(s, e)| &line[s..e])
        .collect()
}

// spec: queue-kit/SPEC.md §check-task-names — a fence line toggles the window no reference is read in
fn fenced_lines(lines: &[&str]) -> Vec<bool> {
    let mut inside = false;
    lines
        .iter()
        .map(|l| {
            if l.trim_start().starts_with("```") {
                inside = !inside;
                return true;
            }
            inside
        })
        .collect()
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
    let lines: Vec<&str> = text.lines().collect();
    let entries = queue::entries(&lines, &sec);
    // spec: queue-kit/SPEC.md §check-task-names — each line's owning entry and whether it is that
    // entry's tag line; a sub-task's lines are its own, not its parent's
    let mut owner: Vec<Option<(&str, bool)>> = vec![None; lines.len()];
    for e in &entries {
        match live.get(&e.slug) {
            Some(first) => dup.push(format!(
                "{}:{}: {} (first seen at line {})",
                file,
                e.start + 1,
                e.slug,
                first
            )),
            None => {
                live.insert(e.slug.clone(), e.start + 1);
            }
        }
        for (i, slot) in owner.iter_mut().enumerate().take(e.end).skip(e.start) {
            *slot = Some((e.slug.as_str(), Some(i) == e.tag_line));
        }
        if let Some(t) = e.tag_line {
            for r in blocked_refs(lines[t]) {
                brefs.push((r, t + 1));
            }
        }
    }
    let mut cur = "";
    for (i, line) in lines.iter().enumerate() {
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
        if cur == "task" {
            if queue::heading_level(line).is_some_and(|l| l >= 3) && queue::entry_heading(line).is_none() {
                invalid.push(format!("{}:{}: {}", file, fnr, line));
            } else if owner[i].is_none() && queue::is_top_level_bullet(line) {
                missing.push(format!("{}:{}: {}", file, fnr, line));
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

    // spec: queue-kit/SPEC.md §check-task-names — assertion R: a same-file link must name a live
    // entry, and a single-backticked live slug in an entry body must be that link
    let fenced = fenced_lines(&lines);
    let (mut dangling, mut unlinked) = (Vec::new(), Vec::new());
    for (i, line) in lines.iter().enumerate() {
        if fenced[i] {
            continue;
        }
        for (s, e) in queue::link_slugs(line) {
            if !live.contains_key(&line[s..e]) {
                dangling.push(format!("{}:{}: (#{})", file, i + 1, &line[s..e]));
            }
        }
        let Some((own, is_tags)) = owner[i] else { continue };
        if is_tags || queue::heading_level(line).is_some() {
            continue;
        }
        for tok in single_backtick_slugs(line) {
            if tok != own && live.contains_key(tok) {
                unlinked.push(format!("{}:{}: `{}`", file, i + 1, tok));
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
        + stale.len()
        + dangling.len()
        + unlinked.len();
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
            &["check-task-names: task-section bullet outside every entry (an entry is a heading):"],
            &missing,
            &[
                "  help: open the entry with its slug as a heading — '### the-slug' — or run",
                "        bash gate-sdk/bin/run-gates.sh --emit queue-migrate --write <queue-file>.",
            ],
        );
        block(
            &["check-task-names: task-section heading that is not a valid slug:"],
            &invalid,
            &[
                "  help: an entry heading is the bare slug, [a-z0-9][a-z0-9-]* (lowercase",
                "        kebab-case), at '###' or '####' for a sub-task; tags go on the line below.",
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
        block(
            &["check-task-names: same-file link naming no live entry (a dangling reference):"],
            &dangling,
            &["  help: a retired slug is cited in backticks — '`the-slug`', not a link."],
        );
        block(
            &["check-task-names: live entry cited in backticks rather than linked:"],
            &unlinked,
            &["  help: a live reference is a link — '[the-slug](#the-slug)'."],
        );
        return 1;
    }

    println!(
        "TASK-NAMES: clean ({} live slug(s) unique, {} done, all blockers resolve to live tasks and every reference is a live link or a retired citation in {})",
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

    // spec: queue-kit/SPEC.md §check-task-names — assertion R's token is single-backtick, and a
    // fence hides every reference inside it
    #[test]
    fn a_citation_token_is_single_backtick_and_a_fence_hides_it() {
        assert_eq!(single_backtick_slugs("see `a-b` and ``c-d``"), vec!["a-b"]);
        let lines = ["x", "```", "`a`", "```", "y"];
        assert_eq!(fenced_lines(&lines), vec![false, true, true, true, false]);
    }
}
