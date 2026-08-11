// spec: queue-kit/SPEC.md §check-queue-hygiene — the queue holds only tasks, tags, and
// structure: no HTML comments, no duplicate lines, no column-0 prose
use crate::queue;
use std::collections::HashMap;

fn is_rule(line: &str) -> bool {
    match line.strip_prefix("---") {
        Some(rest) => rest.bytes().all(|b| b == b' ' || b == b'\t'),
        None => false,
    }
}

fn is_blank(line: &str) -> bool {
    line.bytes().all(|b| b == b' ' || b == b'\t')
}

pub fn run(args: &[String]) -> i32 {
    let leads = match queue::knob_array("QUEUE_KIT_PROSE_LEADS") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-hygiene: {}", e);
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-queue-hygiene: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-queue-hygiene: file not found: {}", file);
            return 2;
        }
    };

    let mut html: Vec<String> = Vec::new();
    let mut prose: Vec<String> = Vec::new();
    let mut dup: Vec<String> = Vec::new();
    let mut seen: HashMap<&str, usize> = HashMap::new();

    for (i, line) in text.lines().enumerate() {
        let fnr = i + 1;
        if line.contains("<!--") || line.contains("-->") {
            html.push(format!("{}:{}: {}", file, fnr, line));
        }

        // spec: queue-kit/SPEC.md §check-queue-hygiene — every column-0 line must be a
        // heading, a '- ' bullet, '---', or a configured QUEUE_KIT_PROSE_LEADS lead
        let col0 = match line.bytes().next() {
            Some(b) => b != b' ' && b != b'\t',
            None => false,
        };
        if col0 {
            let ok = line.starts_with('#')
                || matches!(line.as_bytes().get(1), Some(&c) if line.starts_with('-') && (c == b' ' || c == b'\t'))
                || is_rule(line)
                || leads.iter().any(|l| !l.is_empty() && line.starts_with(l.as_str()));
            if !ok {
                prose.push(format!("{}:{}: {}", file, fnr, line));
            }
        }

        if !is_blank(line) && !is_rule(line) {
            match seen.get(line) {
                Some(first) => dup.push(format!(
                    "{}:{}: {} (first seen at line {})",
                    file, fnr, line, first
                )),
                None => {
                    seen.insert(line, fnr);
                }
            }
        }
    }

    if !html.is_empty() || !prose.is_empty() || !dup.is_empty() {
        if !html.is_empty() {
            println!("check-queue-hygiene: HTML comment(s) in the queue (provenance belongs in git history):");
            for x in &html {
                println!("  {}", x);
            }
            println!("  help: delete the comment; record the why in the commit message, not the queue.");
        }
        if !dup.is_empty() {
            if !html.is_empty() {
                println!();
            }
            println!("check-queue-hygiene: exact-duplicate line(s) (copy-paste artifact):");
            for x in &dup {
                println!("  {}", x);
            }
            println!("  help: remove the duplicate; if two tasks genuinely share wording, differentiate them.");
        }
        if !prose.is_empty() {
            if !html.is_empty() || !dup.is_empty() {
                println!();
            }
            println!("check-queue-hygiene: column-0 prose (every column-0 line must be a heading,");
            println!("a '- ' bullet, '---', or a configured QUEUE_KIT_PROSE_LEADS lead):");
            for x in &prose {
                println!("  {}", x);
            }
            println!("  help: indent the prose to a continuation line under its bullet, or (for a");
            println!("        recurring protocol lead) add its token to QUEUE_KIT_PROSE_LEADS.");
        }
        return 1;
    }

    println!(
        "QUEUE-HYGIENE: clean (no HTML comments, no duplicate lines, no column-0 prose in {})",
        file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_horizontal_rule_is_exempt_from_both_prose_and_duplicate_arms() {
        assert!(is_rule("---"));
        assert!(is_rule("---   "));
        assert!(!is_rule("--- x"));
        assert!(!is_rule("--"));
    }

    #[test]
    fn blankness_is_whitespace_only_not_merely_empty() {
        assert!(is_blank(""));
        assert!(is_blank("  \t "));
        assert!(!is_blank(" x"));
    }
}
