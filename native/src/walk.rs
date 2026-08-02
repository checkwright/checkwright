// spec: gate-sdk/SPEC.md §lib/gate.sh — the Rust counterpart of gate_find's pruned
// walk, reading the same GATE_SDK_PRUNE_DIRS knob so the two substrates cannot
// scan different trees
use std::fs;
use std::path::{Path, PathBuf};

pub const PRUNE_DIRS_DEFAULT: &str = "target .git node_modules .tmp gate-tests";

pub fn prune_dirs() -> Vec<String> {
    match std::env::var("GATE_SDK_PRUNE_DIRS") {
        Ok(v) if !v.trim().is_empty() => v.split_whitespace().map(String::from).collect(),
        _ => PRUNE_DIRS_DEFAULT
            .split_whitespace()
            .map(String::from)
            .collect(),
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — a directory that cannot be read is
// an error the caller must surface, never a silently smaller corpus: an unreadable
// tree reported as clean is the vacuity the substrate port exists not to introduce.
pub fn find_files(root: &Path, exts: &[&str]) -> Result<Vec<PathBuf>, String> {
    let prune = prune_dirs();
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let rd = fs::read_dir(&dir)
            .map_err(|e| format!("cannot read directory {}: {}", dir.display(), e))?;
        let mut entries: Vec<PathBuf> = Vec::new();
        for ent in rd {
            let ent = ent.map_err(|e| format!("cannot read entry in {}: {}", dir.display(), e))?;
            entries.push(ent.path());
        }
        entries.sort();
        for p in entries {
            let name = match p.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            let meta = fs::symlink_metadata(&p)
                .map_err(|e| format!("cannot stat {}: {}", p.display(), e))?;
            if meta.is_dir() {
                if prune.contains(&name) {
                    continue;
                }
                stack.push(p);
            } else if meta.is_file() {
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    if exts.contains(&ext) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — the
    // executed coupling for the one knob default this crate duplicates, which
    // check-knob-default-coupling cannot reach (shell idioms, kit roots only)
    #[test]
    fn prune_default_equals_the_shell_libraries() {
        let lib = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../gate-sdk/lib/gate.sh");
        let text = fs::read_to_string(&lib)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", lib.display(), e));
        let line = text
            .lines()
            .find(|l| l.trim_start().starts_with("GATE_PRUNE_DIRS=("))
            .expect("no GATE_PRUNE_DIRS=(…) default in gate-sdk/lib/gate.sh");
        let inner = line
            .split_once('(')
            .and_then(|(_, r)| r.split_once(')'))
            .map(|(v, _)| v)
            .expect("malformed GATE_PRUNE_DIRS array literal");
        let shell: Vec<&str> = inner.split_whitespace().collect();
        let rust: Vec<&str> = PRUNE_DIRS_DEFAULT.split_whitespace().collect();
        assert_eq!(
            rust, shell,
            "the native prune-dir default has drifted from the shell library's; \
             the two substrates would scan different trees"
        );
    }
}
