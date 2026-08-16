// spec: delegation-kit/SPEC.md §One template, a resident pointer — every `the template's
// **<name>** rule` citation in SPEC §The delegation model resolves to a template bullet's
// bold lead-in (forward direction only)
use std::collections::HashSet;
use std::path::Path;

// spec: delegation-kit/SPEC.md §One template, a resident pointer — a lead-in is a top-level
// `- **Name**` bullet; its bold span, trailing period stripped and any inline code deleted
fn template_leadins(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        if !line.starts_with("- **") {
            continue;
        }
        let after_open = &line[4..];
        if let Some(p) = after_open.find("**") {
            let mut name = after_open[..p].to_string();
            if name.ends_with('.') {
                name.pop();
            }
            out.push(strip_backticks(&name));
        }
    }
    out
}

fn strip_backticks(s: &str) -> String {
    let mut out = String::new();
    let mut in_code = false;
    for c in s.chars() {
        if c == '`' {
            in_code = !in_code;
            continue;
        }
        if !in_code {
            out.push(c);
        }
    }
    out
}

// spec: delegation-kit/SPEC.md §One template, a resident pointer — the section's body, joined
// with a leading space per line so a bold span cannot straddle two source lines without one
fn delegation_model_section(text: &str) -> String {
    let mut sec = String::new();
    let mut insec = false;
    for line in text.lines() {
        if is_delegation_model_heading(line) {
            insec = true;
            continue;
        }
        if line.starts_with("## ") {
            insec = false;
            continue;
        }
        if insec {
            sec.push(' ');
            sec.push_str(line);
        }
    }
    sec
}

fn is_delegation_model_heading(line: &str) -> bool {
    match line.strip_prefix("## The delegation model") {
        Some(rest) => rest
            .bytes()
            .all(|b| matches!(b, b' ' | b'\t' | b'\x0b' | b'\x0c' | b'\r')),
        None => false,
    }
}

// spec: delegation-kit/SPEC.md §One template, a resident pointer — a citation is a `**Name**`
// span immediately followed (whitespace only) by the word `rule` or `bullet`, word-bounded
fn extract_citations(sec: &str) -> Vec<String> {
    let mut out = Vec::new();
    let n = sec.len();
    let mut i = 0usize;
    while i < n {
        let op = match sec[i..].find("**") {
            Some(p) => i + p,
            None => break,
        };
        let after = op + 2;
        if after > n {
            break;
        }
        let cl = match sec[after..].find("**") {
            Some(p) => after + p,
            None => break,
        };
        let name = &sec[after..cl];
        let tail = &sec[cl + 2..];
        if tail_cites_rule_or_bullet(tail) {
            let mut c = name.to_string();
            if c.ends_with('.') {
                c.pop();
            }
            out.push(c);
        }
        i = cl + 2;
    }
    out
}

fn tail_cites_rule_or_bullet(tail: &str) -> bool {
    let b = tail.as_bytes();
    let mut i = 0usize;
    while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\x0b' | b'\x0c' | b'\r' | b'\n') {
        i += 1;
    }
    if i == 0 {
        return false;
    }
    for w in ["rule", "bullet"] {
        if b[i..].starts_with(w.as_bytes()) {
            let after = i + w.len();
            if after == b.len() || !b[after].is_ascii_alphanumeric() {
                return true;
            }
        }
    }
    false
}

pub fn run(args: &[String]) -> i32 {
    let spec_file = args
        .first()
        .cloned()
        .unwrap_or_else(|| "delegation-kit/SPEC.md".to_string());
    let template_file = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "delegation-kit/templates/agent-execution.md".to_string());

    for f in [&spec_file, &template_file] {
        if !Path::new(f).is_file() {
            eprintln!("check-rule-citation: not found: {}", f);
            return 2;
        }
    }

    let spec_text = match std::fs::read_to_string(&spec_file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("check-rule-citation: cannot read {}: {}", spec_file, e);
            return 2;
        }
    };
    let template_text = match std::fs::read_to_string(&template_file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("check-rule-citation: cannot read {}: {}", template_file, e);
            return 2;
        }
    };

    let leadins: HashSet<String> = template_leadins(&template_text).into_iter().collect();

    let mut sec = delegation_model_section(&spec_text);
    sec = strip_backticks(&sec);
    let cites = extract_citations(&sec);

    let mut unresolved: Vec<String> = Vec::new();
    for c in &cites {
        if !leadins.contains(c) {
            unresolved.push(c.clone());
        }
    }

    if !unresolved.is_empty() {
        println!(
            "check-rule-citation: SPEC §The delegation model cites a rule that does not resolve to a template lead-in in {}:",
            template_file
        );
        for u in &unresolved {
            println!("  **{}**", u);
        }
        println!("  help: a citation names a template bullet's bold lead-in verbatim (minus its trailing period); fix the name or the renamed lead-in — delegation-kit/SPEC.md §One template, a resident pointer");
        return 1;
    }

    println!(
        "RULE-CITATION: clean ({} citation(s) in SPEC §The delegation model each resolve to a template lead-in; {} lead-in(s) available)",
        cites.len(),
        leadins.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_leadin_is_the_bold_span_minus_trailing_period_and_code_spans() {
        assert_eq!(
            template_leadins("- **Resume journal.** Narration goes there.\n"),
            vec!["Resume journal".to_string()]
        );
        // spec: delegation-kit/SPEC.md §One template, a resident pointer — the shell's gsub
        // deletes a backtick span wherever it falls in the name, reproduced rather than tidied
        assert_eq!(
            template_leadins("- **`foo` bar.** text\n"),
            vec![" bar".to_string()]
        );
        assert!(template_leadins("  - **Nested.** not top-level\n").is_empty());
    }

    #[test]
    fn a_citation_needs_the_rule_or_bullet_word_immediately_after() {
        let sec = " see the **Resume journal** rule and **Something** rule and **Other** thing.";
        assert_eq!(extract_citations(sec), vec!["Resume journal", "Something"]);
    }

    #[test]
    fn the_section_closes_on_the_next_level_two_heading() {
        let spec = "## The delegation model\nbody **X** rule\n## Next\nbody **Y** rule\n";
        let sec = delegation_model_section(spec);
        assert!(sec.contains("**X**"));
        assert!(!sec.contains("**Y**"));
    }
}
