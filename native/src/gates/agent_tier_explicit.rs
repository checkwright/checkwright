// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — every agent definition under the
// scanned directory declares a `model:` field in its frontmatter (an explicit `inherit`
// passes; only omission reds)
use crate::walk;
use std::path::Path;

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — the frontmatter is the first
// `---`-delimited block; a file that does not open one is unscannable and reds by
// construction, and the field is read only inside that first block, never past its close
fn has_explicit_model(text: &str) -> bool {
    let mut lines = text.lines();
    match lines.next() {
        Some("---") => {}
        _ => return false,
    }
    for line in lines {
        if line == "---" {
            return false;
        }
        if is_model_line(line) {
            return true;
        }
    }
    false
}

fn is_model_line(line: &str) -> bool {
    let Some(rest) = line.strip_prefix("model:") else {
        return false;
    };
    let trimmed = rest.trim_start_matches([' ', '\t', '\x0b', '\x0c', '\r']);
    !trimmed.is_empty()
}

pub fn run(args: &[String]) -> i32 {
    let mut dir = match args.first() {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("DELEGATION_KIT_AGENT_DIR") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-agent-tier-explicit: {}", e);
                return 2;
            }
        },
    };
    if dir.ends_with('/') {
        dir = dir.trim_end_matches('/').to_string();
    }

    let root = Path::new(&dir);
    if !root.is_dir() {
        println!(
            "AGENT-TIER-EXPLICIT: clean (0 agent definition(s) under {}; no agent-definition directory)",
            dir
        );
        return 0;
    }

    let files = match walk::find_files(root, &["md"]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("check-agent-tier-explicit: {}", e);
            return 2;
        }
    };

    if files.is_empty() {
        println!(
            "AGENT-TIER-EXPLICIT: clean (0 agent definition(s) under {}; nothing to check)",
            dir
        );
        return 0;
    }

    let mut bare: Vec<String> = Vec::new();
    for f in &files {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("check-agent-tier-explicit: cannot read {}: {}", f.display(), e);
                return 2;
            }
        };
        if !has_explicit_model(&text) {
            bare.push(f.display().to_string());
        }
    }

    if !bare.is_empty() {
        println!("check-agent-tier-explicit: agent definition(s) whose frontmatter omits the model: field:");
        for f in &bare {
            println!("  {}: no 'model:' field in frontmatter", f);
        }
        println!("  help: state the tier in the definition's frontmatter — an omitted model: is not a neutral");
        println!("        default but the literal 'inherit' (the dispatcher's tier), so declare it even when");
        println!("        the answer is to inherit; 'model: inherit' passes.");
        return 1;
    }

    println!(
        "AGENT-TIER-EXPLICIT: clean ({} agent definition(s) under {}, each declaring an explicit model:)",
        files.len(),
        dir
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_must_be_inside_the_first_frontmatter_block() {
        assert!(has_explicit_model("---\nmodel: inherit\n---\nbody\n"));
        assert!(has_explicit_model("---\nmodel: sonnet\n---\n"));
        assert!(!has_explicit_model("---\n---\nmodel: sonnet\n"));
        assert!(!has_explicit_model("no frontmatter\nmodel: sonnet\n"));
        assert!(!has_explicit_model("---\nno model here\n---\n"));
    }

    #[test]
    fn a_model_line_needs_a_non_whitespace_value() {
        assert!(is_model_line("model: x"));
        assert!(is_model_line("model:x"));
        assert!(!is_model_line("model:"));
        assert!(!is_model_line("model:   "));
        assert!(!is_model_line("Model: x"));
    }
}
