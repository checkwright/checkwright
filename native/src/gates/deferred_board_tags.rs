// spec: queue-kit/SPEC.md §check-deferred-board-tags — every top-level deferred entry carries one
// cost tag and one surface tag on its lead line, and no active-section lead line carries either
use crate::queue;
use crate::walk;
use std::path::Path;

#[derive(PartialEq)]
enum Sec {
    Active,
    Deferred,
    Other,
}

// spec: queue-kit/SPEC.md §check-deferred-board-tags — assertion A over one deferred lead line
fn deferred_findings(line: &str, root: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    match queue::field_tags(line, "cost").as_slice() {
        [] => out.push("no [cost:] tag".to_string()),
        [t] => {
            if !t.value.is_some_and(queue::cost_class_valid) {
                out.push(format!(
                    "[cost:{}] is not <recurrence>/<magnitude> ({} / {})",
                    t.raw,
                    queue::COST_RECURRENCES.join("|"),
                    queue::COST_MAGNITUDES.join("|")
                ));
            }
        }
        many => out.push(format!(
            "{} [cost:] tags; an entry carries exactly one",
            many.len()
        )),
    }
    match queue::field_tags(line, "surface").as_slice() {
        [] => out.push("no [surface:] tag".to_string()),
        [t] => {
            let named = t
                .value
                .filter(|v| queue::surface_value_shaped(v))
                .is_some_and(|v| root.iter().any(|e| e == v));
            if !named {
                out.push(format!(
                    "[surface:{}] names no top-level entry of the repository root",
                    t.raw
                ));
            }
        }
        many => out.push(format!(
            "{} [surface:] tags; an entry carries exactly one",
            many.len()
        )),
    }
    out
}

// spec: queue-kit/SPEC.md §check-deferred-board-tags — assertion B: a promotion drops both tags
fn active_findings(line: &str) -> Vec<String> {
    ["cost", "surface"]
        .iter()
        .filter(|n| !queue::field_tags(line, n).is_empty())
        .map(|n| format!("[{}:] on an active entry; a promotion drops it", n))
        .collect()
}

pub fn run(args: &[String]) -> i32 {
    if args.len() > 2 {
        eprintln!("check-deferred-board-tags: unexpected argument: {}", args[2]);
        return 2;
    }
    let sec = match queue::Sections::active_and_deferred() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-deferred-board-tags: {}", e);
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-deferred-board-tags: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-deferred-board-tags: file not found: {}", file);
            return 2;
        }
    };
    // spec: queue-kit/SPEC.md §check-deferred-board-tags — the surface value set is the root's own
    // listing, so the consumer's tree is its configuration and no knob carries it
    let root = args.get(1).cloned().unwrap_or_else(|| ".".to_string());
    let entries: Vec<String> = match walk::list_dir(Path::new(&root)) {
        Ok(v) => v.into_iter().map(|(n, _)| n).collect(),
        Err(e) => {
            eprintln!("check-deferred-board-tags: repository root unreadable: {}", e);
            return 2;
        }
    };

    let mut findings: Vec<String> = Vec::new();
    let mut cur = Sec::Other;
    let mut fence = false;
    let mut seen = 0usize;
    for (i, line) in text.lines().enumerate() {
        if queue::is_section_line(line) {
            let name = queue::heading_name(line);
            cur = if sec.is_deferred(line) {
                Sec::Deferred
            } else if sec.active.iter().any(|a| Some(a.as_str()) == name) {
                Sec::Active
            } else {
                Sec::Other
            };
            continue;
        }
        if line.trim_start_matches([' ', '\t']).starts_with("```") {
            fence = !fence;
            continue;
        }
        if fence || cur == Sec::Other {
            continue;
        }
        let slug = match queue::bullet_slug(line) {
            Some(s) => s,
            None => continue,
        };
        let found = if cur == Sec::Deferred {
            if !queue::is_top_level_bullet(line) {
                continue;
            }
            seen += 1;
            deferred_findings(line, &entries)
        } else {
            active_findings(line)
        };
        for f in found {
            findings.push(format!("{}:{}: {} — {}", file, i + 1, slug, f));
        }
    }

    if !findings.is_empty() {
        println!("check-deferred-board-tags: deferred board tag violation(s) (an entry's cost class");
        println!("and primary surface ride its lead line, the only line the board's readers scan):");
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: give each deferred lead line exactly one [cost: <recurrence>/<magnitude>],");
        println!("        classed from the entry's own cost prose, and one [surface: <entry>] naming");
        println!("        the top-level root entry it mainly changes; a promotion drops both");
        println!("        (queue-kit/SPEC.md section check-deferred-board-tags).");
        return 1;
    }

    println!(
        "DEFERRED-BOARD-TAGS: clean ({} {} entr{} tagged with one cost and one surface, no active entry carrying either, in {})",
        seen,
        sec.deferred,
        if seen == 1 { "y" } else { "ies" },
        file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> Vec<String> {
        vec!["TASK-QUEUE.md".to_string(), "widget-kit".to_string()]
    }

    #[test]
    fn a_lead_line_with_one_valid_tag_of_each_is_clean() {
        let line = "- **a** [design-pending] [cost: iteration/high] [surface: widget-kit] — x";
        assert!(deferred_findings(line, &root()).is_empty());
    }

    #[test]
    fn an_absent_tag_is_named_per_tag() {
        let got = deferred_findings("- **a** [design-pending] — x", &root());
        assert_eq!(got, vec!["no [cost:] tag", "no [surface:] tag"]);
    }

    #[test]
    fn a_value_outside_the_closed_grammar_or_the_root_listing_reds() {
        let bad_cost = "- **a** [cost: weekly/high] [surface: widget-kit] — x";
        assert_eq!(deferred_findings(bad_cost, &root()).len(), 1);
        let spaced = "- **a** [cost:once/low] [surface: widget-kit] — x";
        assert_eq!(deferred_findings(spaced, &root()).len(), 1);
        let missing = "- **a** [cost: once/low] [surface: no-such-kit] — x";
        assert_eq!(deferred_findings(missing, &root()).len(), 1);
        let pathy = "- **a** [cost: once/low] [surface: widget-kit/README.md] — x";
        assert_eq!(deferred_findings(pathy, &root()).len(), 1);
    }

    #[test]
    fn a_repeated_tag_reds_even_when_each_instance_is_valid() {
        let line = "- **a** [cost: once/low] [cost: event/low] [surface: widget-kit] — x";
        assert_eq!(
            deferred_findings(line, &root()),
            vec!["2 [cost:] tags; an entry carries exactly one"]
        );
    }

    #[test]
    fn an_active_lead_line_carrying_either_tag_reds() {
        assert_eq!(active_findings("- **a** [surface: widget-kit] — x").len(), 1);
        assert!(active_findings("- **a** — the cost: prose is not a tag").is_empty());
    }
}
