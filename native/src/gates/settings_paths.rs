// spec: context-kit/SPEC.md §check-settings-paths — every path the committed settings file tells
// the harness to run resolves in the working tree, and every `--hook` operand is a member
use crate::hook::{self, Registration};
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

fn command_token<'a>(tok: &[&'a str]) -> Option<&'a str> {
    hook::command_index(tok).map(|i| tok[i])
}

// spec: context-kit/SPEC.md §check-settings-paths — the extraction predicate's one holder; its
// second reader is guard-kit's compare-settings-allow, which calls it rather than copying it
pub fn literal_script_path(entry: &str) -> Option<&str> {
    let inner = entry.strip_prefix("Bash(")?.strip_suffix(')')?;
    let cand = command_token(&tokens(inner))?;
    // spec: context-kit/SPEC.md §check-settings-paths — a `*` in the command token makes it a
    // pattern, intentionally polymorphic over files that need not exist today; the `*` twin of
    // a literal grant is a separate token and stays in scope
    (cand.ends_with(".sh") && !cand.contains('*')).then_some(cand)
}

#[derive(Debug, PartialEq)]
enum HookPath {
    Literal(String),
    Placeholder,
    NoPath,
}

// spec: context-kit/SPEC.md §check-settings-paths — a hook's candidate carries a `/` whatever its
// extension; a leading project-root placeholder is stripped, and any other `$` roots it elsewhere
fn hook_path(cand: &str) -> HookPath {
    if !cand.contains('/') {
        return HookPath::NoPath;
    }
    let rest = strip_project_root(cand).unwrap_or_else(|| cand.to_string());
    if rest.contains('$') {
        return HookPath::Placeholder;
    }
    HookPath::Literal(rest)
}

fn strip_project_root(cand: &str) -> Option<String> {
    let (quote, body) = match cand.chars().next() {
        Some(q @ ('"' | '\'')) => (Some(q), &cand[1..]),
        _ => (None, cand),
    };
    let after = body
        .strip_prefix("${CLAUDE_PROJECT_DIR}")
        .or_else(|| body.strip_prefix("$CLAUDE_PROJECT_DIR"))?;
    let (after, quote) = match quote {
        Some(q) if after.starts_with(q) => (&after[1..], None),
        other => (after, other),
    };
    let rest = after.strip_prefix('/')?;
    let rest = match quote {
        Some(q) => rest.strip_suffix(q)?,
        None => rest,
    };
    Some(rest.to_string())
}

struct HookEntry<'a> {
    at: String,
    shown: String,
    tokens: Vec<&'a str>,
}

// spec: context-kit/SPEC.md §check-settings-paths — every `type: command` hook under `hooks`,
// whatever its event; events iterate sorted, groups and hooks in registration order
fn hook_entries(doc: &Value) -> Vec<HookEntry<'_>> {
    let mut out = Vec::new();
    let Some(events) = doc.get("hooks").and_then(Value::as_object) else {
        return out;
    };
    for (event, groups) in events {
        for (i, g) in groups.as_array().into_iter().flatten().enumerate() {
            let hooks = g.get("hooks").and_then(Value::as_array);
            for (j, h) in hooks.into_iter().flatten().enumerate() {
                if h.get("type").and_then(Value::as_str) != Some("command") {
                    continue;
                }
                let tokens = hook::command_tokens(h);
                let shown = match h.get("args") {
                    Some(_) => tokens.join(" "),
                    None => h.get("command").and_then(Value::as_str).unwrap_or("").to_string(),
                };
                out.push(HookEntry {
                    at: format!("hooks.{}[{}].hooks[{}]", event, i, j),
                    shown,
                    tokens,
                });
            }
        }
    }
    out
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
        let Some(cand) = literal_script_path(&entry) else {
            continue;
        };
        checked += 1;
        if !std::path::Path::new(&format!("{}/{}", root, cand)).is_file() {
            dead.push(format!("{} — no such file: {}", entry, cand));
        }
    }

    let (mut hooks_checked, mut hooks_skipped, mut bad_member) = (0usize, 0usize, false);
    for h in hook_entries(&doc) {
        if let Some(cand) = command_token(&h.tokens) {
            match hook_path(cand) {
                HookPath::Literal(p) => {
                    hooks_checked += 1;
                    if !std::path::Path::new(&format!("{}/{}", root, p)).is_file() {
                        dead.push(format!("{}: {} — no such file: {}", h.at, h.shown, p));
                    }
                }
                HookPath::Placeholder => hooks_skipped += 1,
                HookPath::NoPath => {}
            }
        }
        match hook::registration(&h.tokens) {
            Registration::Member(m) if !hook::members().contains(&m) => {
                bad_member = true;
                dead.push(format!("{}: {} — no such hook member: {}", h.at, h.shown, m));
            }
            Registration::NoOperand => {
                bad_member = true;
                dead.push(format!("{}: {} — --hook names no member", h.at, h.shown));
            }
            _ => {}
        }
    }

    if !dead.is_empty() {
        println!(
            "check-settings-paths: {} names a path or hook member that does not resolve:",
            settings_file
        );
        for d in &dead {
            println!("  {}", d);
        }
        println!("  help: repoint each entry at the path that replaced it, or drop the entry if the");
        println!("        grant is spent — a port that replaces checks/<gate>.sh with <gate>.gate");
        println!("        strands both the bare form and its '*' twin; swap a hook's registration");
        println!("        before deleting the script it runs, or its guard fails open silently.");
        if bad_member {
            println!("  help: this binary carries: {}", hook::members().join(", "));
        }
        return 1;
    }

    println!(
        "SETTINGS-PATHS: clean ({} literal .sh grant(s) and {} hook path(s) in {} resolve; {} hook(s) skipped for a placeholder)",
        checked, hooks_checked, settings_file, hooks_skipped
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
    fn the_literal_script_path_is_scoped_to_a_literal_sh_command_token() {
        assert_eq!(literal_script_path("Read(scripts/x.sh)"), None);
        assert_eq!(literal_script_path("Bash(git status)"), None);
        assert_eq!(literal_script_path("Bash(bash scripts/*.sh)"), None);
        assert_eq!(literal_script_path("Bash(bash scripts/x.sh)"), Some("scripts/x.sh"));
        assert_eq!(
            literal_script_path("Bash(env FOO=1 bash scripts/x.sh)"),
            Some("scripts/x.sh")
        );
        assert_eq!(literal_script_path("Bash(scripts/x.sh --flag)"), Some("scripts/x.sh"));
    }

    #[test]
    fn a_hook_path_strips_only_the_project_root_placeholder() {
        let lit = |s: &str| HookPath::Literal(s.to_string());
        assert_eq!(hook_path("scripts/x.sh"), lit("scripts/x.sh"));
        assert_eq!(hook_path("scripts/x.py"), lit("scripts/x.py"));
        assert_eq!(hook_path("${CLAUDE_PROJECT_DIR}/scripts/x.sh"), lit("scripts/x.sh"));
        assert_eq!(hook_path("$CLAUDE_PROJECT_DIR/scripts/x.sh"), lit("scripts/x.sh"));
        assert_eq!(hook_path("\"$CLAUDE_PROJECT_DIR\"/scripts/x.sh"), lit("scripts/x.sh"));
        assert_eq!(hook_path("\"${CLAUDE_PROJECT_DIR}/scripts/x.sh\""), lit("scripts/x.sh"));
        assert_eq!(hook_path("${CLAUDE_PLUGIN_ROOT}/x.sh"), HookPath::Placeholder);
        assert_eq!(hook_path("$CLAUDE_PROJECT_DIRX/x.sh"), HookPath::Placeholder);
        assert_eq!(hook_path("node"), HookPath::NoPath);
    }

    #[test]
    fn every_hook_event_is_read_with_its_location() {
        let doc: Value = serde_json::from_str(
            r#"{"hooks":{"Stop":[{"hooks":[{"type":"prompt","prompt":"x"},{"type":"command","command":"bash a/b.sh"}]}],
               "PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"node","args":["c/d.js","--x"]}]}]}}"#,
        )
        .unwrap();
        let got: Vec<(String, String)> =
            hook_entries(&doc).into_iter().map(|h| (h.at, h.shown)).collect();
        assert_eq!(
            got,
            vec![
                ("hooks.PreToolUse[0].hooks[0]".to_string(), "node c/d.js --x".to_string()),
                ("hooks.Stop[0].hooks[1]".to_string(), "bash a/b.sh".to_string()),
            ]
        );
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
