// spec: queue-kit/SPEC.md §check-task-conservation — every live slug present at HEAD is still
// present (live or done) in the working tree; the absence class diff-review misses
use crate::{proc, programs};
use crate::queue;
use std::collections::HashSet;

// spec: gate-sdk/SPEC.md §Fail-closed contract — the two failures stay apart: a git that
// could not be spawned is `Err` and reaches the caller's exit 2, while `Ok(None)` is a git
// that ran and said no. Folding them is how "no repository" comes to mean "no git".
fn git_capture(args: &[&str]) -> Result<Option<Vec<u8>>, String> {
    Ok(proc::run(&programs::GIT, args)?.stdout().map(<[u8]>::to_vec))
}

// spec: queue-kit/SPEC.md §check-task-conservation — the rule itself, taken apart from git so
// the conservation logic is unit-testable without a repository to stand it up in
fn diff(head: &str, work: &str, sec: &queue::Sections) -> (usize, Vec<String>) {
    let mut present: HashSet<String> = queue::live_slugs(work, sec).into_iter().collect();
    present.extend(queue::done_slugs(work, sec));

    let mut lost = Vec::new();
    let mut conserved = 0usize;
    for s in queue::live_slugs(head, sec) {
        if present.contains(&s) {
            conserved += 1;
        } else {
            lost.push(s);
        }
    }
    (conserved, lost)
}

// spec: queue-kit/SPEC.md §check-task-conservation — assertion B: a slug entering the live set (absent
// from HEAD's live and done sets) is at most `max` code points; a slug live at HEAD is grandfathered
fn over_ceiling(head: &str, work: &str, sec: &queue::Sections, max: usize) -> Vec<(String, usize)> {
    let mut known: HashSet<String> = queue::live_slugs(head, sec).into_iter().collect();
    known.extend(queue::done_slugs(head, sec));
    queue::live_slugs(work, sec)
        .into_iter()
        .filter(|s| !known.contains(s))
        .map(|s| {
            let n = s.chars().count();
            (s, n)
        })
        .filter(|(_, n)| *n > max)
        .collect()
}

pub fn run(args: &[String]) -> i32 {
    let sec = match queue::Sections::with_done() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-task-conservation: {}", e);
            return 2;
        }
    };
    let max = match queue::knob_scalar("QUEUE_KIT_SLUG_MAX") {
        Ok(v) if v == "off" => None,
        Ok(v) => match v.parse::<usize>() {
            Ok(n) => Some(n),
            Err(_) => {
                eprintln!("check-task-conservation: QUEUE_KIT_SLUG_MAX is not a positive integer or off: {}", v);
                return 2;
            }
        },
        Err(e) => {
            eprintln!("check-task-conservation: {}", e);
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-task-conservation: {}", e);
                return 2;
            }
        },
    };

    match git_capture(&["rev-parse", "--git-dir"]) {
        Err(e) => {
            eprintln!("check-task-conservation: {}", e);
            return 2;
        }
        Ok(None) => {
            println!("TASK-CONSERVATION: clean (no git repository — no HEAD baseline to compare)");
            return 0;
        }
        Ok(Some(_)) => {}
    }

    let head = match git_capture(&["show", &format!("HEAD:{}", file)]) {
        Err(e) => {
            eprintln!("check-task-conservation: {}", e);
            return 2;
        }
        // spec: queue-kit/SPEC.md §check-task-conservation — a HEAD written in the retired bullet
        // grammar is read through the migration, so the converting commit is conserved against its
        // own pre-image and a slug live before it stays grandfathered
        Ok(Some(b)) => crate::emit::queue_migrate::heading_form(&String::from_utf8_lossy(&b), &sec).into_owned(),
        Ok(None) => {
            println!(
                "TASK-CONSERVATION: clean ({} not at HEAD — no prior live slugs to conserve)",
                file
            );
            return 0;
        }
    };

    if !std::path::Path::new(&file).is_file() {
        eprintln!("check-task-conservation: worktree file not found: {}", file);
        return 2;
    }
    let work = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("check-task-conservation: cannot read {}: {}", file, e);
            return 2;
        }
    };

    let (conserved, lost) = diff(&head, &work, &sec);
    let long = max.map(|m| over_ceiling(&head, &work, &sec, m)).unwrap_or_default();
    let mut red = false;

    if !lost.is_empty() {
        red = true;
        println!("check-task-conservation: live slug(s) present at HEAD but gone from the working");
        println!("tree — neither live nor done (a lost task; the absence class diff-review misses):");
        for s in &lost {
            println!("  {}", s);
        }
        println!("  help: restore the entry, or move its slug to the done section if it completed.");
        println!("        A rename must move the old slug to done and sweep every [blocked-by:] ref.");
    }
    if let (false, Some(m)) = (long.is_empty(), max) {
        red = true;
        println!("check-task-conservation: new or renamed slug(s) over the {}-code-point ceiling", m);
        println!("(QUEUE_KIT_SLUG_MAX; a slug live at HEAD is grandfathered, a new one is not):");
        for (s, n) in &long {
            println!("  {} ({} code points, ceiling {})", s, n, m);
        }
        println!("  help: shorten the slug before it lands — it becomes the entry's heading and anchor.");
    }
    if red {
        return 1;
    }

    println!(
        "TASK-CONSERVATION: clean ({} HEAD live slug(s) all still present in {})",
        conserved, file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sections() -> queue::Sections {
        queue::Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: "Icebox".into(),
            done: "Done".into(),
        }
    }

    #[test]
    fn a_slug_still_live_or_moved_to_done_is_conserved() {
        let head = "## New Features\n### a\nx\n### b\ny\n";
        let work = "## New Features\n### a\nx\n## Done\n- b\n";
        assert_eq!(diff(head, work, &sections()), (2, vec![]));
    }

    #[test]
    fn a_slug_in_neither_set_is_lost_and_reported_in_head_order() {
        let head = "## New Features\n### a\nx\n### b\ny\n### c\nz\n";
        let work = "## New Features\n### b\ny\n";
        let (conserved, lost) = diff(head, work, &sections());
        assert_eq!(conserved, 1);
        assert_eq!(lost, vec!["a".to_string(), "c".to_string()]);
    }

    // spec: queue-kit/SPEC.md §check-task-conservation — eviction to the icebox is conserved by
    // construction, and an entry carried into done with its live shape intact is not
    #[test]
    fn eviction_conserves_but_a_relocated_live_shape_does_not() {
        let head = "## New Features\n### a\nx\n";
        assert_eq!(diff(head, "## Icebox\n### a\nx\n", &sections()), (1, vec![]));
        let (_, lost) = diff(head, "## Done\n### a\nx\n", &sections());
        assert_eq!(lost, vec!["a".to_string()]);
    }

    // spec: queue-kit/SPEC.md §check-task-conservation — the commit converting a bullet-grammar
    // queue is conserved against its own pre-image, read through the migration
    #[test]
    fn a_bullet_grammar_head_is_read_through_the_migration() {
        let head = "## New Features\n\n- **a** — x\n- **b** — y\n\n## Done\n";
        let work = "## New Features\n\n### a\n\nx\n\n## Done\n\n- b\n";
        let head = crate::emit::queue_migrate::heading_form(head, &sections());
        assert_eq!(diff(&head, work, &sections()), (2, vec![]));
    }

    // spec: queue-kit/SPEC.md §check-task-conservation — assertion B binds a new slug and a rename,
    // and grandfathers a long slug already live (or done) at HEAD
    #[test]
    fn only_a_slug_new_to_head_is_held_to_the_ceiling() {
        let head = "## New Features\n### grandfathered-long-slug\nx\n## Done\n- done-long-slug\n";
        let work = "## New Features\n### grandfathered-long-slug\nx\n### brand-new-long-slug\ny\n\
                    ### short\nz\n### done-long-slug\nw\n## Done\n";
        assert_eq!(
            over_ceiling(head, work, &sections(), 10),
            vec![("brand-new-long-slug".to_string(), 19)]
        );
        assert!(over_ceiling(head, work, &sections(), 19).is_empty());
        assert!(over_ceiling("", "## New Features\n### abc\nx\n", &sections(), 3).is_empty());
    }
}
