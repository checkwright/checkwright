// spec: gate-sdk/SPEC.md §lib/gate.sh — the crate's one serialization point for the
// process-global config bridge: a case writes a knob only while holding this guard, so
// cargo's threads cannot interleave two writers of one variable
use std::path::Path;
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
    pub fn set(&self, name: &str, value: &str) {
        std::env::set_var(name, value);
    }

    pub fn remove(&self, name: &str) {
        std::env::remove_var(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the roster of process-global environment writes; a
    // spelling outside this module is a write nothing serializes
    const ENV_WRITE_APIS: &[&str] = &["set_var", "remove_var"];

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the machine side of the serialization contract, in
    // the roster shape §check-reads-couples' unit test B uses: a guard a later case bypasses by
    // calling the std API directly leaves the race the guard was landed to delete
    #[test]
    fn no_module_outside_this_one_writes_the_environment() {
        let knobs = lock();
        crate::walk::bridge_declared_knobs(&knobs);
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
}
