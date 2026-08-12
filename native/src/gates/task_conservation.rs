// spec: queue-kit/SPEC.md §check-task-conservation — every live slug present at HEAD is still
// present (live or done) in the working tree; the absence class diff-review misses
use crate::proc;
use crate::queue;
use std::collections::HashSet;

// spec: gate-sdk/SPEC.md §Fail-closed contract — the two failures stay apart: a git that
// could not be spawned is `Err` and reaches the caller's exit 2, while `Ok(None)` is a git
// that ran and said no. Folding them is how "no repository" comes to mean "no git".
fn git_capture(args: &[&str]) -> Result<Option<Vec<u8>>, String> {
    Ok(proc::run("git", args)?.stdout().map(<[u8]>::to_vec))
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

pub fn run(args: &[String]) -> i32 {
    let sec = match queue::Sections::with_done() {
        Ok(s) => s,
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
        Ok(Some(b)) => String::from_utf8_lossy(&b).into_owned(),
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

    if !lost.is_empty() {
        println!("check-task-conservation: live slug(s) present at HEAD but gone from the working");
        println!("tree — neither live nor done (a lost task; the absence class diff-review misses):");
        for s in &lost {
            println!("  {}", s);
        }
        println!("  help: restore the entry, or move its slug to the done section if it completed.");
        println!("        A rename must move the old slug to done and sweep every [blocked-by:] ref.");
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
        let head = "## New Features\n- **a** x\n- **b** y\n";
        let work = "## New Features\n- **a** x\n## Done\n- b\n";
        assert_eq!(diff(head, work, &sections()), (2, vec![]));
    }

    #[test]
    fn a_slug_in_neither_set_is_lost_and_reported_in_head_order() {
        let head = "## New Features\n- **a** x\n- **b** y\n- **c** z\n";
        let work = "## New Features\n- **b** y\n";
        let (conserved, lost) = diff(head, work, &sections());
        assert_eq!(conserved, 1);
        assert_eq!(lost, vec!["a".to_string(), "c".to_string()]);
    }

    // spec: queue-kit/SPEC.md §check-task-conservation — eviction to the icebox is conserved by
    // construction, and an entry carried into done with its live shape intact is not
    #[test]
    fn eviction_conserves_but_a_relocated_live_shape_does_not() {
        let head = "## New Features\n- **a** x\n";
        assert_eq!(diff(head, "## Icebox\n- **a** x\n", &sections()), (1, vec![]));
        let (_, lost) = diff(head, "## Done\n- **a** [design-pending] — x\n", &sections());
        assert_eq!(lost, vec!["a".to_string()]);
    }
}
