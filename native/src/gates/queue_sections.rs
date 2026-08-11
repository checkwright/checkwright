// spec: queue-kit/SPEC.md §check-queue-sections — the queue carries each required ## section
// heading exactly once, the fail-closed floor under every section-scoped scanner
use crate::queue;

// spec: queue-kit/SPEC.md §check-queue-sections — a required name ending in `:` is matched as
// a prefix, because a titled heading carries its value after the colon; every other name must
// be the whole heading
fn matches(line: &str, sec: &str) -> bool {
    if sec.ends_with(':') {
        return line.starts_with("## ") && line[3..].starts_with(sec);
    }
    queue::heading_name(line).map(|n| n == sec).unwrap_or(false)
}

pub fn run(args: &[String]) -> i32 {
    let required = match queue::knob_array("QUEUE_KIT_REQUIRED_SECTIONS") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-sections: {}", e);
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-queue-sections: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-queue-sections: file not found: {}", file);
            return 2;
        }
    };

    let mut missing: Vec<String> = Vec::new();
    let mut dup: Vec<String> = Vec::new();
    for sec in &required {
        let n = text.lines().filter(|l| matches(l, sec)).count();
        if n == 0 {
            missing.push(sec.clone());
        } else if n > 1 {
            dup.push(format!("{} ({} occurrences)", sec, n));
        }
    }

    if !missing.is_empty() || !dup.is_empty() {
        println!(
            "check-queue-sections: required '##' section(s) not present exactly once in {}",
            file
        );
        println!("(every section-scoped scanner — amendment-queue, task-names, conservation, the");
        println!("session-context index — locates work by these headings and finds nothing when one drops):");
        for s in &missing {
            println!("  missing:   ## {}", s);
        }
        for s in &dup {
            println!("  duplicate: ## {}", s);
        }
        println!("  help: restore the heading (spelled exactly), or remove the duplicate. The");
        println!("        required set is QUEUE_KIT_REQUIRED_SECTIONS (queue-config.sh).");
        return 1;
    }

    println!(
        "QUEUE-SECTIONS: clean ({} required section(s) each present once in {})",
        required.len(),
        file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_colon_terminated_name_matches_as_a_prefix() {
        assert!(matches("## Iteration: demo", "Iteration:"));
        assert!(matches("## Iteration:", "Iteration:"));
        assert!(!matches("## Iterations: x", "Iteration:"));
        assert!(matches("## Done", "Done"));
        assert!(matches("## Done  ", "Done"));
        assert!(!matches("## Done extra", "Done"));
    }
}
