// spec: context-kit/SPEC.md §check-settings-paths — every committed allow-list entry whose
// command token is a literal repo-relative .sh path resolves in the working tree
use crate::walk;
use serde_json::Value;

// spec: context-kit/SPEC.md §check-settings-paths — the kit-owned allow-array read, string-typed
// entries only; hand-compiled rather than routed through a path expression
fn allow_entries(doc: &Value) -> Vec<String> {
    doc.get("permissions")
        .and_then(|p| p.get("allow"))
        .and_then(|a| a.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

// spec: context-kit/SPEC.md §check-settings-paths — the shell splits with `read -ra` and no
// pathname expansion, so the grant is never globbed against the tree; splitting on ASCII
// whitespace is that property in a language which has no expansion to suppress
fn tokens(inner: &str) -> Vec<&str> {
    inner.split_ascii_whitespace().collect()
}

// spec: context-kit/SPEC.md §check-settings-paths — the command token is not always argv[0]: a
// grant may lead with `env NAME=VALUE ...` before the interpreter, and one on this tree does;
// a `bash`/`sh` interpreter word is then skipped too
fn command_token<'a>(tok: &[&'a str]) -> Option<&'a str> {
    let mut i = 0usize;
    if tok.first() == Some(&"env") {
        i = 1;
        while i < tok.len() && is_assignment(tok[i]) {
            i += 1;
        }
    }
    if matches!(tok.get(i), Some(&"bash") | Some(&"sh")) {
        i += 1;
    }
    tok.get(i).copied()
}

fn is_assignment(t: &str) -> bool {
    let Some(at) = t.find('=') else { return false };
    if at == 0 {
        return false;
    }
    let b = t.as_bytes();
    (b[0].is_ascii_alphabetic() || b[0] == b'_') && at > 0
}

pub fn run(args: &[String]) -> i32 {
    let mut fixture: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--fixture" => {
                fixture = args.get(i + 1).cloned();
                if fixture.is_none() {
                    eprintln!("check-settings-paths: --fixture needs a directory");
                    return 2;
                }
                i += 2;
            }
            a => {
                eprintln!("check-settings-paths: unexpected argument: {}", a);
                return 2;
            }
        }
    }

    let (settings_file, root) = match &fixture {
        Some(d) => {
            if !std::path::Path::new(d).is_dir() {
                eprintln!("check-settings-paths: fixture dir not found: {}", d);
                return 2;
            }
            (format!("{}/settings.json", d), d.clone())
        }
        None => {
            let sf = match walk::knob_scalar("CONTEXT_KIT_SETTINGS_FILE") {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-settings-paths: {}", e);
                    return 2;
                }
            };
            (sf, ".".to_string())
        }
    };

    // spec: context-kit/SPEC.md §check-settings-paths — the settings file is the sole subject and
    // the sibling gate reads it on the same terms, so an absent or unparseable one is fail-closed
    // here too, never a clean skip
    let text = match std::fs::read_to_string(&settings_file) {
        Ok(t) => t,
        Err(_) => {
            eprintln!(
                "check-settings-paths: settings file not readable: {}",
                settings_file
            );
            return 2;
        }
    };
    let doc: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("check-settings-paths: {} is not valid JSON", settings_file);
            return 2;
        }
    };

    let mut dead: Vec<String> = Vec::new();
    let mut checked = 0usize;
    for entry in allow_entries(&doc) {
        let Some(inner) = entry.strip_prefix("Bash(").and_then(|r| r.strip_suffix(')')) else {
            continue;
        };
        let tok = tokens(inner);
        let Some(cand) = command_token(&tok) else {
            continue;
        };
        if !cand.ends_with(".sh") {
            continue;
        }
        // spec: context-kit/SPEC.md §check-settings-paths — a `*` in the command token makes it a
        // pattern, intentionally polymorphic over files that need not exist today; the `*` twin of
        // a literal grant is a separate token and stays in scope
        if cand.contains('*') {
            continue;
        }
        checked += 1;
        if !std::path::Path::new(&format!("{}/{}", root, cand)).is_file() {
            dead.push(format!("{} — no such file: {}", entry, cand));
        }
    }

    if !dead.is_empty() {
        println!(
            "check-settings-paths: {} grants a path that does not resolve in the tree:",
            settings_file
        );
        for d in &dead {
            println!("  {}", d);
        }
        println!("  help: repoint each entry at the path that replaced it, or drop the entry if the");
        println!("        grant is spent — a port that replaces checks/<gate>.sh with <gate>.gate");
        println!("        strands both the bare form and its '*' twin.");
        return 1;
    }

    println!(
        "SETTINGS-PATHS: clean ({} literal .sh grant(s) in {} resolve)",
        checked, settings_file
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §check-settings-paths — the command token walk, which is what a
    // vacuously-matching predicate would silently skip; the checked count is what the fixture
    // pair pins and this is its unit-level twin
    #[test]
    fn the_command_token_skips_an_env_prefix_and_an_interpreter_word() {
        assert_eq!(command_token(&tokens("bash scripts/x.sh")), Some("scripts/x.sh"));
        assert_eq!(command_token(&tokens("scripts/x.sh --flag")), Some("scripts/x.sh"));
        assert_eq!(
            command_token(&tokens("env FOO=1 BAR=2 bash scripts/x.sh")),
            Some("scripts/x.sh")
        );
        assert_eq!(command_token(&tokens("env FOO=1 scripts/x.sh")), Some("scripts/x.sh"));
        assert_eq!(command_token(&tokens("sh scripts/x.sh")), Some("scripts/x.sh"));
        assert_eq!(command_token(&tokens("git status")), Some("git"));
        assert_eq!(command_token(&tokens("")), None);
    }

    #[test]
    fn only_string_typed_allow_entries_are_read() {
        let doc: Value = serde_json::from_str(
            r#"{"permissions":{"allow":["Bash(bash a.sh)", 7, {"x":1}, "Bash(bash b.sh)"]}}"#,
        )
        .unwrap();
        assert_eq!(
            allow_entries(&doc),
            vec!["Bash(bash a.sh)".to_string(), "Bash(bash b.sh)".to_string()]
        );
        let none: Value = serde_json::from_str("{}").unwrap();
        assert!(allow_entries(&none).is_empty());
    }
}
