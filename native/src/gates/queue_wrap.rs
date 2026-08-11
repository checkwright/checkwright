// spec: queue-kit/SPEC.md §check-queue-wrap — no queue line exceeds the wrap budget (Unicode
// code points), so a runaway never reflows to column 0
use crate::queue;

// comment-tier-exempt: the shell side reaches code-point width by making awk bytewise under
// LC_ALL=C and subtracting UTF-8 continuation bytes; chars() counts the same quantity directly
fn cplen(s: &str) -> usize {
    s.chars().count()
}

fn is_fence(line: &str) -> bool {
    line.trim_start_matches([' ', '\t']).starts_with("```")
}

fn is_table_row(line: &str) -> bool {
    line.trim_start_matches([' ', '\t']).starts_with('|')
}

pub fn run(args: &[String]) -> i32 {
    let budget_raw = match queue::knob_scalar("QUEUE_KIT_WRAP_BUDGET") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-wrap: {}", e);
            return 2;
        }
    };
    let budget: usize = match budget_raw.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!(
                "check-queue-wrap: QUEUE_KIT_WRAP_BUDGET is not a positive integer: {}",
                budget_raw
            );
            return 2;
        }
    };
    let file = match args.first() {
        Some(a) => a.clone(),
        None => match queue::knob_scalar("QUEUE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-queue-wrap: {}", e);
                return 2;
            }
        },
    };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("check-queue-wrap: file not found: {}", file);
            return 2;
        }
    };

    let mut over: Vec<(usize, usize, String)> = Vec::new();
    let mut fence = false;
    for (i, line) in text.lines().enumerate() {
        if is_fence(line) {
            fence = !fence;
            continue;
        }
        if fence || is_table_row(line) {
            continue;
        }
        let w = cplen(line);
        if w <= budget {
            continue;
        }
        // spec: queue-kit/SPEC.md §check-queue-wrap — a line over budget solely from one
        // unbreakable token (a URL or a path) is exempt: no wrap helps it
        let maxtok = line.split_whitespace().map(cplen).max().unwrap_or(0);
        if maxtok > budget {
            continue;
        }
        over.push((i + 1, w, line.to_string()));
    }

    if !over.is_empty() {
        println!(
            "check-queue-wrap: line(s) over the {}-column budget (a runaway",
            budget
        );
        println!("that reflows to column 0 corrupts the '- ' lead the tools key on):");
        for (ln, w, text) in &over {
            println!("  {}:{}: {} cols — {}", file, ln, w, text);
        }
        println!("  help: hard-wrap the line at ~80 columns. Exempt already: table rows,");
        println!("        fenced code, and a line over budget solely from one unbreakable token.");
        return 1;
    }

    println!(
        "QUEUE-WRAP: clean (no line exceeds {} columns in {})",
        budget, file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_is_counted_in_code_points_not_bytes() {
        assert_eq!(cplen("abc"), 3);
        assert_eq!(cplen("—x"), 2);
        assert_eq!(cplen("é"), 1);
    }

    #[test]
    fn fences_and_table_rows_are_recognised_with_leading_space() {
        assert!(is_fence("```bash"));
        assert!(is_fence("   ```"));
        assert!(!is_fence("x ```"));
        assert!(is_table_row("  | a | b |"));
        assert!(!is_table_row("a | b"));
    }
}
