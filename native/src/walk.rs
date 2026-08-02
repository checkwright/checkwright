// spec: gate-sdk/SPEC.md §lib/gate.sh — the Rust counterpart of gate_find's pruned
// walk: the same prune-dir set, read from the same GATE_SDK_PRUNE_DIRS knob so the
// two substrates cannot scan different trees. The default literal below is the one
// check-knob-default-coupling holds against the shell default.
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
