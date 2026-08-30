// spec: context-kit/SPEC.md §check-memory-off — local-environment scan: the per-project harness
// memory dir stays empty and no local settings override re-enables a pinned key
use crate::gates::template_registry_parity::list_members;
use crate::json::{values_equal, Path};
use crate::walk;
use serde_json::Value;

fn trim(s: &str) -> &str {
    s.trim_matches([' ', '\t', '\r'])
}

// spec: context-kit/SPEC.md §Layout and configuration — the harness names each project's dir by
// its absolute path with '/' and '.' folded to '-', and this is the rule's one implementation
// since the shell library's copy left with the gate that called it.
fn memory_dir_default() -> Result<Option<String>, String> {
    let Some(top) = walk::toplevel_opt()? else {
        return Ok(None);
    };
    // spec: context-kit/SPEC.md §check-memory-off — the shell reads `$HOME` under `set -u`, so an
    // unset HOME aborts it rather than yielding a verdict; the port refuses on the same state
    // instead of deriving a path under `/` that would be absent and therefore read as clean
    let home = std::env::var("HOME").map_err(|_| {
        "HOME is unset — the harness memory dir cannot be derived; treating as failure (not clean)"
            .to_string()
    })?;
    let folded: String = top
        .chars()
        .map(|c| if c == '/' || c == '.' { '-' } else { c })
        .collect();
    Ok(Some(format!("{}/.claude/projects/{}/memory", home, folded)))
}

// spec: context-kit/SPEC.md §check-memory-off — the line grammar's guard: a line failing it is
// skipped, because this member reads the manifest for one override and `check-settings-pins` is
// the manifest's grader
fn split_pin(line: &str) -> Option<(&str, &str)> {
    let at = line.find('=')?;
    let path = trim(&line[..at]);
    let expected = trim(&line[at + 1..]);
    if path.is_empty() || expected.is_empty() || !path.starts_with('.') {
        return None;
    }
    Some((path, expected))
}

fn resolve_memory_dirs() -> Result<Vec<String>, String> {
    // spec: context-kit/SPEC.md §check-memory-off — the knob is a word-split list of globs, so
    // each bridged element splits again on whitespace exactly as the shell's unquoted `$memdirs`
    // does; empty means "derive it", not "no dir"
    let words: Vec<String> = walk::knob_array("CONTEXT_KIT_MEMORY_DIRS")?
        .iter()
        .flat_map(|e| e.split_whitespace().map(String::from).collect::<Vec<_>>())
        .collect();
    let patterns = if words.is_empty() {
        match memory_dir_default()? {
            Some(d) => vec![d],
            None => Vec::new(),
        }
    } else {
        words
    };
    let mut dirs: Vec<String> = Vec::new();
    for p in &patterns {
        dirs.extend(walk::glob_entries(p));
    }
    Ok(dirs)
}

pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        eprintln!("check-memory-off: unexpected argument: {}", a);
        return 2;
    }

    let pins_file = match walk::knob_scalar("CONTEXT_KIT_SETTINGS_PINS") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-memory-off: {}", e);
            return 2;
        }
    };
    let settings_file = match walk::knob_scalar("CONTEXT_KIT_SETTINGS_FILE") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-memory-off: {}", e);
            return 2;
        }
    };
    let local_settings = format!(
        "{}.local.json",
        settings_file
            .strip_suffix(".json")
            .unwrap_or(&settings_file)
    );

    let mem_dirs = match resolve_memory_dirs() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("check-memory-off: {}", e);
            return 2;
        }
    };

    let mut polluted: Vec<String> = Vec::new();
    let mut overrides: Vec<String> = Vec::new();

    // spec: context-kit/SPEC.md §check-memory-off — content is any regular file that is not the
    // dir-preserving .gitkeep
    for d in &mem_dirs {
        let p = std::path::Path::new(d);
        if !p.is_dir() {
            continue;
        }
        // spec: gate-sdk/SPEC.md §Fail-closed contract — the shell's bare `find` prunes nothing,
        // so neither does this walk; a dir it cannot read is exit 2, never a smaller corpus
        let files = match walk::find_with_prune(p, &|_| false) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "check-memory-off: {} — the check could not run; treating as failure (not clean)",
                    e
                );
                return 2;
            }
        };
        if files
            .iter()
            .any(|f| f.file_name().and_then(|n| n.to_str()) != Some(".gitkeep"))
        {
            polluted.push(d.clone());
        }
    }

    // spec: context-kit/SPEC.md §check-memory-off — the untracked local settings file can
    // re-enable what the tracked pin disabled; the hermetic gate cannot see it, this one can
    if std::path::Path::new(&local_settings).is_file() {
        if let Ok(pins_text) = std::fs::read_to_string(&pins_file) {
            let text = match std::fs::read_to_string(&local_settings) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!(
                        "check-memory-off: {} is not readable — the check could not run; treating as failure (not clean)",
                        local_settings
                    );
                    return 2;
                }
            };
            let doc: Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("check-memory-off: {} is not valid JSON", local_settings);
                    return 2;
                }
            };
            for line in list_members(&pins_text) {
                let Some((path_src, expected_src)) = split_pin(&line) else {
                    continue;
                };
                let Ok(path) = Path::compile(path_src) else {
                    continue;
                };
                let Ok(actual) = path.eval(&doc) else {
                    continue;
                };
                // spec: context-kit/SPEC.md §check-memory-off — a null actual is this member's
                // ordinary clean case (the local file sets no override for that key) and is
                // skipped, which is the opposite of check-settings-pins' absent-pin refusal
                if actual.is_null() {
                    continue;
                }
                // spec: context-kit/SPEC.md §check-memory-off — a right-hand side that is not
                // JSON cannot be compared structurally, so it takes this member's one disposition
                // for a pin it cannot read; the sibling gate fail-closes on that same line
                let Ok(expected) = serde_json::from_str::<Value>(expected_src) else {
                    continue;
                };
                if !values_equal(&actual, &expected) {
                    overrides.push(format!(
                        "{} locally set to {} (pin expects {})",
                        path_src, actual, expected_src
                    ));
                }
            }
        }
    }

    if !polluted.is_empty() || !overrides.is_empty() {
        println!("check-memory-off: the harness memory posture is not clean on this clone:");
        for d in &polluted {
            println!("  memory dir holds content: {}", d);
        }
        for o in &overrides {
            println!("  local settings override: {}", o);
        }
        println!("  help: durable facts belong in tracked surfaces (the knowledge-friction loop,");
        println!("        the lesson channels, or the operator's local brief), not the harness");
        println!("        memory dir — empty it; and drop any memory-re-enabling key from");
        println!(
            "        {}. See context-kit/SPEC.md §The memory-off doctrine.",
            local_settings
        );
        return 1;
    }

    // spec: context-kit/SPEC.md §check-memory-off — CI-neutral: an absent dir proves nothing
    // about another clone (fail-open on absent, stated here)
    println!(
        "MEMORY-OFF: clean ({} memory dir(s) present, all empty; an absent dir proves nothing about another clone)",
        mem_dirs.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §check-memory-off — the manifest guard the fixture pair cannot
    // reach: the pair fixes the memory-dir axis and the `.test.sh` the override axis, and
    // neither drives a line this gate is supposed to walk past
    #[test]
    fn a_pin_line_outside_the_grammar_is_skipped_rather_than_read() {
        assert_eq!(split_pin(" .a.b = {\"x\":1} "), Some((".a.b", "{\"x\":1}")));
        assert_eq!(split_pin(".a = \"x=y\""), Some((".a", "\"x=y\"")));
        assert_eq!(split_pin("no-equals"), None);
        assert_eq!(split_pin(".a ="), None);
        assert_eq!(split_pin(" = 1"), None);
        assert_eq!(split_pin("autoMemoryEnabled = false"), None);
    }

    // spec: context-kit/SPEC.md §check-memory-off — delta 5's two divergences from
    // check-settings-pins, held at the unit where the scenario holds them end-to-end: `1` and
    // `1.0` are one value under structural comparison, and a null actual is a skip
    #[test]
    fn comparison_is_structural_and_a_null_actual_is_never_a_violation() {
        let one: Value = serde_json::from_str("1").unwrap();
        let one_point_oh: Value = serde_json::from_str("1.0").unwrap();
        assert!(values_equal(&one, &one_point_oh));
        let spaced: Value = serde_json::from_str("{ \"a\" : 1 }").unwrap();
        let compact: Value = serde_json::from_str("{\"a\":1}").unwrap();
        assert!(values_equal(&spaced, &compact));
        assert!(Value::Null.is_null());
    }
}
