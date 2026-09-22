// spec: queue-kit/SPEC.md §check-tag-lead-line — every governed tag sits on its entry's tag
// line or its lesson's bullet, the only lines the tag readers scan
use crate::queue;

// spec: queue-kit/SPEC.md §check-tag-lead-line — one class table, each entry the tag name plus
// its bracket terminator; the match literal and the arr[] key both come off it, and
// `--emit-enum-sets` references this same table rather than parsing this file as text
pub const CLASSES: &[&str] = &[
    "blocked-by:",
    "spec:",
    "attend]",
    "drain-exempt:",
    "roadmap:",
    "observed-by:",
    "cost:",
    "surface:",
    "cap-credit:",
    "recurrence:",
    "roadmap-summary:",
    "not-icebox-eligible:",
];

// spec: queue-kit/SPEC.md §check-tag-lead-line — the terminator strip, held once in the module
// that owns the table: a tag's name is its entry without the bracket terminator
pub fn tag_name(class: &str) -> &str {
    &class[..class.len() - 1]
}

fn classes_on(line: &str, lesson_tags: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for c in CLASSES {
        let term = &c[c.len() - 1..];
        let name = tag_name(c);
        if line.contains(&format!("[{}{}", name, term)) {
            out.push(name.to_string());
        }
    }
    for t in lesson_tags {
        if line.contains(&format!("[{}]", t)) {
            out.push(t.clone());
        }
    }
    out
}

pub fn run(args: &[String]) -> i32 {
    let sec = match queue::Sections::active_and_deferred() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-tag-lead-line: {}", e);
            return 2;
        }
    };
    let lesson_tags = match queue::knob_array("QUEUE_KIT_LESSON_TAGS") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-tag-lead-line: {}", e);
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-tag-lead-line: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-tag-lead-line: file not found: {}", file);
            return 2;
        }
    };

    let mut findings: Vec<(usize, String, usize)> = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    // spec: queue-kit/SPEC.md §check-tag-lead-line — each line's lead: in a task section the owning
    // entry's heading (its tag line carrying the classes), in Lessons the lesson bullet itself
    let mut lead: Vec<Option<(usize, Vec<String>)>> = vec![None; lines.len()];
    for e in queue::entries(&lines, &sec) {
        let cls = classes_on(e.tags(&lines), &lesson_tags);
        for (i, slot) in lead.iter_mut().enumerate().take(e.end).skip(e.start) {
            *slot = if i == e.start || Some(i) == e.tag_line {
                None
            } else {
                Some((e.tag_line.unwrap_or(e.start) + 1, cls.clone()))
            };
        }
    }
    let mut in_lessons = false;
    let mut lesson: Option<(usize, Vec<String>)> = None;
    for (i, line) in lines.iter().enumerate() {
        if queue::is_section_line(line) {
            in_lessons = queue::is_lessons_line(line);
            lesson = None;
            continue;
        }
        if !in_lessons {
            continue;
        }
        if queue::is_bullet(line) {
            lesson = Some((i + 1, classes_on(line, &lesson_tags)));
            continue;
        }
        if lesson.is_some() {
            lead[i] = lesson.clone();
        }
    }

    let mut fence = false;
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim_start_matches([' ', '\t']);
        if t.starts_with("```") {
            fence = !fence;
            continue;
        }
        if fence || t.starts_with('|') {
            continue;
        }
        let Some((leadfnr, leadcls)) = &lead[i] else { continue };
        for k in classes_on(line, &lesson_tags) {
            if !leadcls.contains(&k) {
                findings.push((i + 1, k, *leadfnr));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-tag-lead-line: tag(s) off the entry's tag line (a tag reader scans only");
        println!("the tag line under the heading, or a lesson's bullet; a tag elsewhere silently");
        println!("stops counting):");
        for (ln, cls, lead) in &findings {
            println!(
                "  {}:{}: [{}] in the body; lead line {} carries no [{}]",
                file, ln, cls, lead, cls
            );
        }
        println!("  help: move the tag onto the tag line, the first line under the entry's heading");
        println!("        (or onto the lesson's bullet line). Prose about a tag names it without");
        println!("        its brackets.");
        return 1;
    }

    println!(
        "TAG-LEAD-LINE: clean (every governed tag in the task + Lessons sections is on its entry's tag line or lesson bullet in {})",
        file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_class_is_keyed_by_its_name_and_matched_with_its_terminator() {
        let none: Vec<String> = Vec::new();
        assert_eq!(classes_on("- x [blocked-by: a]", &none), vec!["blocked-by"]);
        assert_eq!(classes_on("- x [attend]", &none), vec!["attend"]);
        assert!(classes_on("- x [blocked-by]", &none).is_empty());
        assert!(classes_on("- x attend]", &none).is_empty());
    }

    // spec: queue-kit/SPEC.md §check-tag-lead-line — membership tracks reader semantics, so
    // [observed-by:] is governed from the moment its reader scans lead lines, with or without a
    // live instance in any one consumer's queue
    #[test]
    fn the_observation_producer_tag_is_a_governed_class() {
        let none: Vec<String> = Vec::new();
        assert_eq!(
            classes_on("- x [observed-by: gates]", &none),
            vec!["observed-by"]
        );
        assert!(classes_on("- x [observed-by]", &none).is_empty());
    }

    #[test]
    fn the_deferred_board_tags_are_governed_classes() {
        let none: Vec<String> = Vec::new();
        assert_eq!(
            classes_on("- x [cost: once/low] [surface: queue-kit]", &none),
            vec!["cost", "surface"]
        );
        assert!(classes_on("- x cost: once/low", &none).is_empty());
    }

    // spec: queue-kit/SPEC.md §check-tag-lead-line — the three former body declarations are field
    // tags read off the tag line, so they are governed classes like the rest
    #[test]
    fn the_former_declarations_are_governed_classes() {
        let none: Vec<String> = Vec::new();
        assert_eq!(
            classes_on("[recurrence: 2026-01-01] [roadmap-summary: S.] [not-icebox-eligible: 2026-01-01 g]", &none),
            vec!["recurrence", "roadmap-summary", "not-icebox-eligible"]
        );
        assert!(classes_on("[roadmap-summary: S.]", &none).iter().all(|c| c != "roadmap"));
    }

    #[test]
    fn a_configured_lesson_tag_joins_the_class_set() {
        let tags = vec!["essay".to_string()];
        assert_eq!(classes_on("- x [essay]", &tags), vec!["essay"]);
        assert!(classes_on("- x [essay]", &[]).is_empty());
    }
}
