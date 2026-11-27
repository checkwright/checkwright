// spec: gate-sdk/SPEC.md §lib/gate.sh — the crate's one serialization point for the
// process-global environment: a case writes a variable only while holding this guard, so
// cargo's threads cannot interleave two writers of one variable
use std::sync::{Mutex, MutexGuard};

static LOCK: Mutex<()> = Mutex::new(());

// spec: gate-sdk/SPEC.md §lib/gate.sh — the guard IS the write API, so what a caller must
// hold to write at all is the lock itself; a free-standing mutex a case may forget to take
// would leave the discipline this replaces
pub struct KnobEnv {
    _guard: MutexGuard<'static, ()>,
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — a poisoned lock is a sibling case that panicked, which
// says nothing about the environment, so the guard is recovered rather than cascading one
// case's failure into every case that writes a knob
pub fn lock() -> KnobEnv {
    KnobEnv {
        _guard: LOCK.lock().unwrap_or_else(|e| e.into_inner()),
    }
}

impl KnobEnv {
    #[cfg(test)]
    pub fn set(&self, name: &str, value: &str) {
        std::env::set_var(name, value);
    }

    pub fn set_os(&self, name: &str, value: &std::ffi::OsStr) {
        std::env::set_var(name, value);
    }

    pub fn remove(&self, name: &str) {
        std::env::remove_var(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the roster of process-global environment writes; a
    // spelling outside this module is a write nothing serializes
    const ENV_WRITE_APIS: &[&str] = &["set_var", "remove_var"];

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the machine side of the serialization contract, in
    // the roster shape §check-reads-couples' unit test B uses: a guard a later case bypasses by
    // calling the std API directly leaves the race the guard was landed to delete
    #[test]
    fn no_module_outside_this_one_writes_the_environment() {
        let _knobs = lock();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = crate::walk::find_files(&src, &["rs"]).expect("cannot enumerate the sources");
        assert!(!files.is_empty(), "no crate source found to scan");
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            if f.file_name().and_then(|n| n.to_str()) == Some("knobenv.rs") {
                continue;
            }
            let text = std::fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            for api in ENV_WRITE_APIS {
                if text.contains(api) {
                    offenders.push(format!("{} names {}", f.display(), api));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "an environment write outside knobenv.rs is unserialized, so a sibling case on \
             another cargo thread can observe it: {:?} — take knobenv::lock() and write through \
             the guard it returns",
            offenders
        );
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the walk's own read of the knob set, from which every
    // reader in walk.rs is derived by closure, so a new reader joins the roster unlisted
    const KNOB_READ: &str = "prune_dirs";

    fn ident(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn calls(body: &str, name: &str) -> bool {
        let pat = format!("{}(", name);
        body.match_indices(&pat).any(|(i, _)| !body[..i].chars().next_back().is_some_and(ident))
    }

    fn fn_name(line: &str) -> Option<&str> {
        let rest = line.trim_start().strip_prefix("pub(crate) ").or_else(|| line.trim_start().strip_prefix("pub "));
        let rest = rest.unwrap_or(line.trim_start()).strip_prefix("fn ")?;
        let end = rest.find(|c: char| !ident(c))?;
        Some(&rest[..end])
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a fn runs from its `fn` line to the first `}` line at its
    // own indent; a fn with no such line is refused rather than read as ending at the file's end
    fn fns<'a>(text: &'a str, path: &Path, at: impl Fn(&[&str], usize) -> bool) -> Vec<(&'a str, String)> {
        let lines: Vec<&str> = text.lines().collect();
        let mut out = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let Some(name) = fn_name(l).filter(|_| at(&lines, i)) else { continue };
            let indent = &l[..l.len() - l.trim_start().len()];
            let close = format!("{}}}", indent);
            let end = (i..lines.len())
                .find(|&j| lines[j] == close || (j == i && l.trim_end().ends_with('}')))
                .unwrap_or_else(|| panic!("{}:{}: fn {} has no closing line at its indent", path.display(), i + 1, name));
            out.push((name, lines[i..=end].join("\n")));
        }
        out
    }

    fn walk_readers(src: &Path) -> Vec<String> {
        let path = src.join("walk.rs");
        let text = std::fs::read_to_string(&path).expect("cannot read walk.rs");
        let top = fns(&text, &path, |lines, i| !lines[i].starts_with(' '))
            .into_iter()
            .filter(|(n, _)| *n != KNOB_READ)
            .collect::<Vec<_>>();
        let mut readers = vec![KNOB_READ.to_string()];
        loop {
            let more: Vec<String> = top
                .iter()
                .filter(|(n, b)| !readers.iter().any(|r| r == n) && readers.iter().any(|r| calls(b, r)))
                .map(|(n, _)| n.to_string())
                .collect();
            if more.is_empty() {
                return readers;
            }
            readers.extend(more);
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a walk reads the knob set, so a case walking unlocked
    // reads it mid-write of a sibling's temporary knobs and refuses; every case calling a reader
    // holds the guard
    #[test]
    fn every_case_calling_a_prune_dirs_reader_holds_the_lock() {
        let _knobs = lock();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let readers = walk_readers(&src);
        assert!(readers.iter().any(|r| r == "find_files"), "the closure lost find_files: {:?}", readers);
        let files = crate::walk::find_files(&src, &["rs"]).expect("cannot enumerate the sources");
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            let case = |lines: &[&str], i: usize| {
                (0..i).rev().map(|j| lines[j].trim()).take_while(|l| l.starts_with("#[")).any(|l| l == "#[test]")
            };
            let home = f.file_name().and_then(|n| n.to_str()) == Some("knobenv.rs");
            for (name, body) in fns(&text, f, case) {
                let held = body.contains("knobenv::lock()") || (home && body.contains("= lock()"));
                if let Some(r) = readers.iter().find(|r| calls(&body, r)).filter(|_| !held) {
                    offenders.push(format!("{}: {} calls {}", f.display(), name, r));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "a case calling a prune_dirs reader without the knob lock races a sibling's knob write: \
             {:?} — take crate::knobenv::lock() for the case's length",
            offenders
        );
    }
}
