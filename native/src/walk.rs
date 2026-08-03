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
    // spec: gate-sdk/SPEC.md §check-reads-couples — every walk passes here, so recording at
    // this one line is what makes unit test A's observation complete.
    #[cfg(test)]
    recorder::note(&root.display().to_string());
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

// spec: gate-sdk/SPEC.md §check-reads-couples — the recorder unit test A observes through.
// Test-scoped deliberately: a production recorder would be state with no reader.
#[cfg(test)]
pub mod recorder {
    use std::cell::RefCell;

    thread_local! {
        static OBSERVED: RefCell<Option<Vec<String>>> = RefCell::new(None);
    }

    pub fn start() {
        OBSERVED.with(|o| *o.borrow_mut() = Some(Vec::new()));
    }

    pub fn stop() -> Vec<String> {
        OBSERVED.with(|o| o.borrow_mut().take()).unwrap_or_default()
    }

    pub fn note(root: &str) {
        OBSERVED.with(|o| {
            if let Some(v) = o.borrow_mut().as_mut() {
                if !v.iter().any(|e| e == root) {
                    v.push(root.to_string());
                }
            }
        });
    }
}

// spec: gate-sdk/SPEC.md §check-reads-couples — unit test A's case-dir lookup lives here
// rather than beside the registry so that the directory scan it needs obeys test B instead
// of being excused from it.
#[cfg(test)]
pub fn fixture_case_dirs(gate: &str) -> Vec<PathBuf> {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let rd = fs::read_dir(&repo)
        .unwrap_or_else(|e| panic!("cannot read repo root {}: {}", repo.display(), e));
    let mut kits: Vec<PathBuf> = Vec::new();
    for ent in rd {
        kits.push(ent.expect("cannot read a repo-root entry").path());
    }
    kits.sort();
    let mut out = Vec::new();
    for kit in kits {
        for case in ["good", "bad"] {
            let d = kit.join("gate-tests").join(gate).join(case);
            if d.is_dir() {
                out.push(d);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test B: a walk outside this file
    // would be invisible to the recorder and would unverify test A, so the roster of
    // filesystem-walk spellings is asserted absent from every other module.
    const WALK_APIS: &[&str] = &["read_dir", "ReadDir"];

    #[test]
    fn no_module_outside_this_one_walks_the_filesystem() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = find_files(&src, &["rs"]).expect("cannot enumerate the crate's sources");
        assert!(!files.is_empty(), "no crate source found to scan");
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            if f.file_name().and_then(|n| n.to_str()) == Some("walk.rs") {
                continue;
            }
            let text = fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            for api in WALK_APIS {
                if text.contains(api) {
                    offenders.push(format!("{} names {}", f.display(), api));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "a filesystem walk outside walk.rs is invisible to the recorder, so unit test A \
             would assert over an incomplete observation: {:?} — route it through \
             walk::find_files",
            offenders
        );
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — the same assertion's other half: a
    // vendored walker cannot be caught by a spelling roster, so the empty dependency set
    // that rules one out is asserted instead of assumed.
    #[test]
    fn the_crate_vendors_no_walker_because_it_vendors_nothing() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let text = fs::read_to_string(&manifest)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", manifest.display(), e));
        let mut in_deps = false;
        let mut declared: Vec<String> = Vec::new();
        for line in text.lines() {
            let t = line.trim();
            if t.starts_with('[') {
                in_deps = t == "[dependencies]";
                continue;
            }
            if in_deps && !t.is_empty() && !t.starts_with('#') {
                declared.push(t.to_string());
            }
        }
        assert!(
            declared.is_empty(),
            "the crate now declares dependencies ({:?}). A vendored walker would bypass \
             walk.rs's recorder and unverify unit test A, and an empty dependency set is \
             also what gate-sdk/SPEC.md's vendoring model states the payload rests on — \
             confirm the new dependency performs no filesystem walk, then widen this test \
             deliberately rather than deleting it",
            declared
        );
    }

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
