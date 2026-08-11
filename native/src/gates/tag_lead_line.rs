// spec: queue-kit/SPEC.md §check-tag-lead-line — every governed tag sits on its bullet's lead
// line, the only line the tag readers scan
use crate::queue;

// spec: queue-kit/SPEC.md §check-tag-lead-line — one class table, each entry the tag name plus
// its bracket terminator; the match literal and the arr[] key both come off it, and
// scripts/enum-sets.sh reads this same table
const CLASSES: &[&str] = &[
    "blocked-by:",
    "spec:",
    "design-pending]",
    "attend]",
    "drain-exempt:",
    "roadmap:",
];

fn classes_on(line: &str, lesson_tags: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for c in CLASSES {
        let term = &c[c.len() - 1..];
        let name = &c[..c.len() - 1];
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
    let mut inscan = false;
    let mut leadfnr = 0usize;
    let mut leadcls: Vec<String> = Vec::new();
    let mut fence = false;

    for (i, line) in text.lines().enumerate() {
        let fnr = i + 1;
        if sec.is_task(line) || queue::is_lessons_line(line) {
            inscan = true;
            leadfnr = 0;
            leadcls.clear();
            continue;
        }
        if queue::is_section_line(line) {
            inscan = false;
            leadfnr = 0;
            leadcls.clear();
            continue;
        }
        if !inscan {
            continue;
        }
        let t = line.trim_start_matches([' ', '\t']);
        if t.starts_with("```") {
            fence = !fence;
            continue;
        }
        if fence || t.starts_with('|') {
            continue;
        }
        if queue::is_bullet(line) {
            leadcls = classes_on(line, &lesson_tags);
            leadfnr = fnr;
            continue;
        }
        if leadfnr == 0 {
            continue;
        }
        for k in classes_on(line, &lesson_tags) {
            if !leadcls.contains(&k) {
                findings.push((fnr, k, leadfnr));
            }
        }
    }

    if !findings.is_empty() {
        println!("check-tag-lead-line: tag(s) pushed off the bullet lead line (a tag reader");
        println!("scans only the lead; a tag on a continuation line silently stops counting):");
        for (ln, cls, lead) in &findings {
            println!(
                "  {}:{}: [{}] on a continuation line; lead line {} carries no [{}]",
                file, ln, cls, lead, cls
            );
        }
        println!("  help: move the tag back onto the bullet's lead line (the '- ...' line). If a");
        println!("        reflow pushed it there, re-wrap so the lead line carries the tag.");
        return 1;
    }

    println!(
        "TAG-LEAD-LINE: clean (every governed tag in the task + Lessons sections is on its bullet lead line in {})",
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
        assert_eq!(classes_on("- x [design-pending]", &none), vec!["design-pending"]);
        assert!(classes_on("- x [blocked-by]", &none).is_empty());
        assert!(classes_on("- x design-pending]", &none).is_empty());
    }

    #[test]
    fn a_configured_lesson_tag_joins_the_class_set() {
        let tags = vec!["essay".to_string()];
        assert_eq!(classes_on("- x [essay]", &tags), vec!["essay"]);
        assert!(classes_on("- x [essay]", &[]).is_empty());
    }
}
