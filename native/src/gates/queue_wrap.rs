// spec: queue-kit/SPEC.md §check-queue-wrap — no queue line exceeds the wrap budget (Unicode
// code points), so a runaway never reflows a tag off its tag line
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

// spec: queue-kit/SPEC.md §check-queue-wrap — the discount removes one well-formed cost tag and one
// surface tag naming an existing root entry, each with one adjacent space, and nothing else, so a
// discounted tag line stays inside the bound that section states
fn discount_board_tags(line: &str, root_has: &dyn Fn(&str) -> bool) -> String {
    let mut s = line.to_string();
    for name in ["cost", "surface"] {
        let span = queue::field_tags(&s, name).iter().find_map(|t| {
            let discounted = match t.value {
                Some(v) if name == "cost" => queue::cost_class_valid(v),
                Some(v) => queue::surface_value_shaped(v) && root_has(v),
                None => false,
            };
            if discounted {
                Some((t.start, t.end))
            } else {
                None
            }
        });
        if let Some((a, b)) = span {
            let bytes = s.as_bytes();
            let (a, b) = if a > 0 && bytes[a - 1] == b' ' {
                (a - 1, b)
            } else if bytes.get(b) == Some(&b' ') {
                (a, b + 1)
            } else {
                (a, b)
            };
            s.replace_range(a..b, "");
        }
    }
    s
}

fn scan(
    text: &str,
    budget: usize,
    deferred: &str,
    root_has: &dyn Fn(&str) -> bool,
) -> Vec<(usize, usize, String)> {
    let mut over: Vec<(usize, usize, String)> = Vec::new();
    let mut fence = false;
    let mut in_deferred = false;
    // spec: queue-kit/SPEC.md §check-queue-wrap — the discounted line is a deferred entry's tag
    // line: the first non-blank line under its `###` heading, when it holds only tags
    let mut after_heading = false;
    for (i, line) in text.lines().enumerate() {
        if queue::is_section_line(line) {
            in_deferred = queue::heading_name(line) == Some(deferred);
        }
        if is_fence(line) {
            fence = !fence;
            continue;
        }
        if fence || is_table_row(line) {
            continue;
        }
        let tag_line = after_heading && queue::is_tag_line(line);
        if !line.trim().is_empty() {
            after_heading = in_deferred && queue::entry_heading(line).is_some_and(|(l, _)| l == 3);
        }
        let measured = if tag_line {
            discount_board_tags(line, root_has)
        } else {
            line.to_string()
        };
        let w = cplen(&measured);
        if w <= budget {
            continue;
        }
        // spec: queue-kit/SPEC.md §check-queue-wrap — a line over budget solely from one
        // unbreakable token (a URL or a path) is exempt: no wrap helps it
        let maxtok = measured.split_whitespace().map(cplen).max().unwrap_or(0);
        if maxtok > budget {
            continue;
        }
        over.push((i + 1, w, line.to_string()));
    }
    over
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
    let deferred = match queue::knob_scalar("QUEUE_KIT_DEFERRED_SECTION") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-queue-wrap: {}", e);
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

    let root_has = |v: &str| std::path::Path::new(v).symlink_metadata().is_ok();
    let over = scan(&text, budget, &deferred, &root_has);

    if !over.is_empty() {
        println!(
            "check-queue-wrap: line(s) over the {}-column budget (a runaway",
            budget
        );
        println!("that an editor reflows can push a tag off the tag line the tools key on):");
        for (ln, w, text) in &over {
            println!("  {}:{}: {} cols — {}", file, ln, w, text);
        }
        println!("  help: hard-wrap the line under the budget. Exempt already: table rows,");
        println!("        fenced code, and a line over budget solely from one unbreakable token;");
        println!("        a deferred tag line's one [cost:] and one [surface:] tag go uncounted.");
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

    fn root(v: &str) -> bool {
        v == "queue-kit"
    }

    fn tagged_lead() -> String {
        format!(
            "[cost: iteration/high] [surface: queue-kit] [spec: {}]",
            "x".repeat(70)
        )
    }

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

    #[test]
    fn a_tagged_deferred_tag_line_is_measured_without_its_board_tags() {
        let lead = tagged_lead();
        assert!(cplen(&lead) > 100);
        let text = format!("## Deferred\n\n### slug\n\n{}\n", lead);
        assert!(scan(&text, 100, "Deferred", &root).is_empty());
    }

    #[test]
    fn a_body_line_over_budget_still_reds() {
        let text = format!(
            "## Deferred\n\n### slug\n\n[cost: once/low] [surface: queue-kit]\n\n{}\n",
            "word ".repeat(21)
        );
        assert_eq!(scan(&text, 100, "Deferred", &root).len(), 1);
    }

    #[test]
    fn an_active_tag_line_carrying_the_tags_is_not_discounted() {
        let text = format!("## New Features\n\n### slug\n\n{}\n", tagged_lead());
        assert_eq!(scan(&text, 100, "Deferred", &root).len(), 1);
    }

    #[test]
    fn only_one_well_formed_tag_naming_an_existing_entry_is_discounted() {
        let doubled = "- **s** [cost: once/low] [cost: once/low] — x";
        assert_eq!(
            discount_board_tags(doubled, &root),
            "- **s** [cost: once/low] — x"
        );
        let elsewhere = "- **s** [surface: elsewhere] [cost: weekly/low] — x";
        assert_eq!(discount_board_tags(elsewhere, &root), elsewhere);
    }
}
