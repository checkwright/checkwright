// spec: gate-sdk/SPEC.md §gen-pre-commit — the two generated git hooks, each a two-line handoff to the
// binary's `--git-hook` arm; the arm and `check-graph` assertion D both read this one emission
use crate::{registry, walk};
use std::path::Path;

pub const KNOBS: &[&str] = &["GATE_SDK_HOOKS_DIR", "GATE_SDK_NATIVE_BIN", "GATE_SDK_KIT_DIRS"];

pub const USAGE: &str = "usage: --emit git-hooks pre-commit|commit-msg|--write";

fn pre_commit_text(bin: &str) -> String {
    format!(
        r#"#!/bin/sh
# pre-commit - GENERATED, DO NOT EDIT.
#
# Emitted by:
#     {bin} --emit git-hooks --write
# It hands off to the gate binary, which reads the per-gate `# graph:`
# manifests at commit time and runs the triggered subset of the gates.list
# battery. Every check also runs whole-tree via:
#     {bin} --run
# check-graph asserts this file equals `--emit git-hooks pre-commit`.
#
# Install (opt-in, per clone):   {bin} --install-hooks
# Bypass once (use sparingly):   git commit --no-verify
exec {bin} --git-hook pre-commit
"#
    )
}

fn commit_msg_text(bin: &str) -> String {
    format!(
        r#"#!/bin/sh
# commit-msg - GENERATED, DO NOT EDIT.
#
# Emitted by:
#     {bin} --emit git-hooks --write
# It hands off to the gate binary, which runs every tier=commit-msg gate on
# the prospective message file git passes as $1.
# check-graph asserts this file equals `--emit git-hooks commit-msg`.
#
# Install (opt-in, per clone):   {bin} --install-hooks
# Bypass once (use sparingly):   git commit --no-verify
exec {bin} --git-hook commit-msg "$1"
"#
    )
}

fn resolved_manifests(text: &str, anchored: &[String]) -> Vec<Vec<(String, String)>> {
    let mut names: Vec<String> = Vec::new();
    for m in registry::members(text) {
        if !names.contains(&m) {
            names.push(m);
        }
    }
    names
        .iter()
        .map(|n| match registry::resolve(n, anchored) {
            Some(src) => {
                let body = std::fs::read(&src)
                    .map(|b| String::from_utf8_lossy(&b).into_owned())
                    .unwrap_or_default();
                registry::manifest_line(&body)
                    .map(registry::manifest_fields)
                    .unwrap_or_default()
            }
            None => Vec::new(),
        })
        .collect()
}

// spec: installer/SPEC.md §init — the conditional `commit_msg` emits on, asked of a registry text
// and absolute resolve dirs, so a caller planning a tree that does not exist yet reads this
// predicate rather than restating it.
pub fn owes_commit_msg(registry: &str, anchored: &[String]) -> bool {
    resolved_manifests(registry, anchored)
        .iter()
        .any(|f| registry::field(f, "tier") == "commit-msg")
}

// spec: gate-sdk/SPEC.md §gen-pre-commit — shell-inert verbatim, anything else POSIX single-quoted
// by a fixed rule so the committed hook is byte-identical across clones
fn quote_elem(s: &str) -> String {
    let inert = |c: char| c.is_ascii_alphanumeric() || "_./:=+,@%-".contains(c);
    if !s.is_empty() && s.chars().all(inert) {
        return s.to_string();
    }
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn native_bin() -> Result<String, String> {
    Ok(quote_elem(&walk::knob_scalar("GATE_SDK_NATIVE_BIN")?))
}

pub fn pre_commit() -> Result<String, String> {
    Ok(pre_commit_text(&native_bin()?))
}

// spec: gate-sdk/SPEC.md §gen-pre-commit — `None` is the one statement of the conditional: no
// registered member is `tier=commit-msg`, so no commit-msg hook is owed. The check dirs are read
// anchored at the root the emission names, so the answer does not hang on the working directory.
pub fn commit_msg(root: &str, gates_dir: &str) -> Result<Option<String>, String> {
    let list = registry::list_path(gates_dir);
    if !Path::new(&list).is_file() {
        return Err(format!("no registry at {}", list));
    }
    let text = super::read_text(&list)?;
    let mut check_dirs = vec![gates_dir.to_string()];
    for k in walk::kit_roots_abs()? {
        check_dirs.push(format!("{}/checks", walk::relative_to(root, &k)));
    }
    let anchored: Vec<String> = check_dirs.iter().map(|d| walk::abs_against(root, d)).collect();
    if !owes_commit_msg(&text, &anchored) {
        return Ok(None);
    }
    Ok(Some(commit_msg_text(&native_bin()?)))
}

fn write(path: &str, text: &str) -> Result<String, String> {
    std::fs::write(path, text).map_err(|e| format!("cannot write {}: {}", path, e))?;
    crate::install::make_executable(Path::new(path))?;
    Ok(format!("git-hooks: wrote {}\n", path))
}

fn hook_path(name: &str) -> Result<String, String> {
    Ok(format!("{}/{}", walk::knob_scalar("GATE_SDK_HOOKS_DIR")?, name))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let op = match args {
        [op] if op == "pre-commit" || op == "commit-msg" || op == "--write" => op.as_str(),
        _ => return Err(USAGE.to_string()),
    };
    let root = walk::toplevel_opt()?.ok_or_else(|| "not inside a git repository".to_string())?;
    let gates_dir = crate::knobs::gates_dir();
    match op {
        "pre-commit" => pre_commit(),
        "commit-msg" => commit_msg(&root, &gates_dir)?.ok_or_else(|| {
            "no registered member is tier=commit-msg, so there is no commit-msg hook to print".to_string()
        }),
        _ => {
            let dir = walk::knob_scalar("GATE_SDK_HOOKS_DIR")?;
            let hook = pre_commit()?;
            let msg = commit_msg(&root, &gates_dir)?;
            std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {}", dir, e))?;
            let mut out = write(&hook_path("pre-commit")?, &hook)?;
            if let Some(m) = msg {
                out.push_str(&write(&hook_path("commit-msg")?, &m)?);
            }
            Ok(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_inert_element_is_verbatim_and_any_other_is_single_quoted() {
        assert_eq!(quote_elem("native/target/release/checkwright-gates"), "native/target/release/checkwright-gates");
        assert_eq!(quote_elem(""), "''");
        assert_eq!(quote_elem("a b"), "'a b'");
        assert_eq!(quote_elem("t\ta'\\\n"), "'t\ta'\\''\\\n'");
    }

    // spec: gate-sdk/SPEC.md §gen-pre-commit — each hook is the shebang and one `exec` of the arm,
    // below a comment header
    #[test]
    fn each_hook_is_a_two_line_handoff() {
        for (text, exec) in [
            (pre_commit_text("b"), "exec b --git-hook pre-commit"),
            (commit_msg_text("b"), "exec b --git-hook commit-msg \"$1\""),
        ] {
            let code: Vec<&str> = text.lines().filter(|l| !l.starts_with('#') || l.starts_with("#!")).collect();
            assert_eq!(code, vec!["#!/bin/sh", exec], "{}", text);
        }
    }

    // spec: gate-sdk/SPEC.md §check-crate-arms — the hook emits under every shipped install
    // profile's kit set, set through a knob file in a scratch gates dir; a tree without the
    // installer's profile roster has nothing to hold
    #[test]
    fn the_pre_commit_hook_emits_under_every_install_profile() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("repo root").to_path_buf();
        let Ok(roster) = std::fs::read_to_string(repo.join("installer/profiles.list")) else {
            return;
        };
        let mut profiles: Vec<(String, Vec<String>)> = Vec::new();
        for line in roster.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
            let (p, kit) = line.split_once('\t').expect("a `<profile><TAB><kit>` row");
            match profiles.iter_mut().find(|(n, _)| n == p) {
                Some((_, kits)) => kits.push(kit.to_string()),
                None => profiles.push((p.to_string(), vec![kit.to_string()])),
            }
        }
        assert!(!profiles.is_empty(), "no profile parsed from installer/profiles.list");
        let mut full: Vec<String> = walk::list_dir(&repo)
            .expect("repo root")
            .into_iter()
            .map(|(n, _)| n)
            .filter(|n| repo.join(n).join("checks").is_dir() || repo.join(n).join("smoke").is_dir())
            .collect();
        full.sort();
        profiles.push(("full".to_string(), full));

        let env = crate::knobenv::lock();
        let d = std::env::temp_dir().join(format!("checkwright-hook-profiles.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        std::fs::copy(repo.join("scripts/gates.list"), d.join("gates.list")).expect("registry");
        let saved: Vec<(&str, Option<String>)> = ["GATE_SDK_GATES_DIR", "GATE_SDK_KIT_DIRS", "GATE_SDK_ROOT"]
            .into_iter()
            .map(|k| (k, std::env::var(k).ok()))
            .collect();
        env.set("GATE_SDK_GATES_DIR", &d.display().to_string());
        env.remove("GATE_SDK_KIT_DIRS");
        env.set("GATE_SDK_ROOT", &repo.join("gate-sdk").display().to_string());
        let mut failed: Vec<String> = Vec::new();
        for (name, kits) in &profiles {
            let dirs: Vec<String> = kits.iter().map(|k| repo.join(k).display().to_string()).collect();
            std::fs::write(d.join("gate-sdk-config.knobs"), format!("GATE_SDK_KIT_DIRS = {}\n", dirs.join(" ")))
                .expect("knob file");
            crate::knobs::reset(&env);
            let emitted = pre_commit().and_then(|_| commit_msg(&repo.display().to_string(), &d.display().to_string()));
            if let Err(e) = emitted {
                failed.push(format!("{}: {}", name, e));
            }
        }
        for (k, v) in saved {
            match v {
                Some(v) => env.set(k, &v),
                None => env.remove(k),
            }
        }
        crate::knobs::reset(&env);
        let _ = std::fs::remove_dir_all(&d);
        assert!(failed.is_empty(), "the hooks do not emit under: {:?}", failed);
    }
}
