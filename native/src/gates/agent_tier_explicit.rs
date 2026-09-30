// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — every agent definition under the
// scanned directory declares a `model:` field in its frontmatter (assertion A: an explicit
// `inherit` passes, only omission reds), and where the tier binding is set its `model:` is the
// bound value of the class its `tier:` declares (assertion B)
use crate::emit::agent_tiers;
use crate::walk;
use std::path::Path;

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — the frontmatter is the first
// `---`-delimited block; a file that does not open one is unscannable and reds by
// construction, and the field is read only inside that first block, never past its close
pub(crate) fn has_explicit_model(text: &str) -> bool {
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

    let binding = match walk::knob_array("DELEGATION_KIT_TIER_MODEL") {
        Ok(b) => b,
        Err(e) => {
            eprintln!("check-agent-tier-explicit: {}", e);
            return 2;
        }
    };

    let mut bare: Vec<String> = Vec::new();
    let mut unbound: Vec<String> = Vec::new();
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
        if !binding.is_empty() {
            if let Some(finding) = generated(&text, &binding) {
                unbound.push(format!("{}: {}", f.display(), finding));
            }
        }
    }

    if !bare.is_empty() || !unbound.is_empty() {
        if !bare.is_empty() {
            println!("check-agent-tier-explicit: agent definition(s) whose frontmatter omits the model: field:");
            for f in &bare {
                println!("  {}: no 'model:' field in frontmatter", f);
            }
            println!("  help: state the tier in the definition's frontmatter — an omitted model: is not a neutral");
            println!("        default but the literal 'inherit' (the dispatcher's tier), so declare it even when");
            println!("        the answer is to inherit; 'model: inherit' passes.");
        }
        if !unbound.is_empty() {
            let Some(door) = super::door_or_report("check-agent-tier-explicit", "--emit agent-tiers --write") else {
                return 2;
            };
            println!("check-agent-tier-explicit: agent definition(s) whose model: is not their tier class's bound model:");
            for f in &unbound {
                println!("  {}", f);
            }
            println!("  help: {} — it regenerates each model:", door);
            println!("        from the definition's tier: and DELEGATION_KIT_TIER_MODEL; a definition that should ride");
            println!("        its dispatcher's tier states 'model: inherit' and no tier:.");
        }
        return 1;
    }

    let state = if binding.is_empty() {
        "binding off".to_string()
    } else {
        let tiered = files
            .iter()
            .filter_map(|f| std::fs::read_to_string(f).ok())
            .filter(|t| agent_tiers::field(t, "tier").is_some())
            .count();
        format!("{} holding their tier class's bound model", tiered)
    };
    println!(
        "AGENT-TIER-EXPLICIT: clean ({} agent definition(s) under {}, each declaring an explicit model:; {})",
        files.len(),
        dir,
        state
    );
    0
}

// spec: delegation-kit/SPEC.md §check-agent-tier-explicit — assertion B over one definition: an
// untiered one states `inherit`, and a tiered one names a bound class and holds, byte for byte,
// what `--emit agent-tiers --write` would write
fn generated(text: &str, binding: &[String]) -> Option<String> {
    let model = agent_tiers::field(text, "model");
    let Some(class) = agent_tiers::field(text, "tier") else {
        return match model.as_deref() {
            Some(m) if m != "inherit" => Some(format!("model: {} with no tier: field", m)),
            _ => None,
        };
    };
    let Some(expected) = crate::tier::bound(binding, &class) else {
        return Some(format!("tier '{}' names no bound class", class));
    };
    if agent_tiers::rewrite(text, expected) != text {
        return Some(format!(
            "stale model: {} (tier '{}' is bound to {})",
            model.as_deref().unwrap_or("-"),
            class,
            expected
        ));
    }
    None
}

// spec: delegation-kit/SPEC.md §The delegation model — D5's lookup: the type a definition declares
// is its first frontmatter block's `name:`, else the file's stem, as the harness names a type
pub(crate) fn defined_type(text: &str, stem: &str) -> String {
    let mut lines = text.lines();
    if lines.next() == Some("---") {
        for line in lines {
            if line == "---" {
                break;
            }
            if let Some(rest) = line.strip_prefix("name:") {
                let name = rest.trim().trim_matches(['"', '\'']);
                if !name.is_empty() {
                    return name.to_string();
                }
            }
        }
    }
    stem.to_string()
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

    // spec: delegation-kit/SPEC.md §check-agent-tier-explicit — assertion B's three findings and its
    // two passing shapes
    #[test]
    fn assertion_b_holds_the_generated_model_line() {
        let binding = vec!["judgment=big".to_string(), "mechanical=small".to_string()];
        assert_eq!(generated("---\ntier: judgment\nmodel: big\n---\n", &binding), None);
        assert_eq!(generated("---\nmodel: inherit\n---\n", &binding), None);
        let stale = generated("---\ntier: judgment\nmodel: small\n---\n", &binding).expect("stale");
        assert!(stale.starts_with("stale model: small"), "{}", stale);
        let unbound = generated("---\ntier: routing\nmodel: big\n---\n", &binding).expect("unbound");
        assert!(unbound.contains("names no bound class"), "{}", unbound);
        let untiered = generated("---\nmodel: big\n---\n", &binding).expect("untiered");
        assert!(untiered.contains("with no tier: field"), "{}", untiered);
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
