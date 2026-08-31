// spec: lifecycle-kit/SPEC.md §check-stage-entry — the PreToolUse(Write|Edit) member: the stage
// stamp is bin/enter-stage.sh's to write, and every gate that would catch a hand-stamp fires only
// at commit, so an uncommitted hand-stamp is otherwise never seen at all.
use crate::hook;
use crate::walk;
use serde_json::Value;
use std::path::{Path, PathBuf};

const NAME: &str = "workflow-state-guard";

pub fn run(payload: Option<&Value>) -> i32 {
    let workflow_dir = match walk::knob_scalar("GATE_SDK_WORKFLOW_DIR") {
        Ok(v) => v,
        // spec: guard-kit/SPEC.md §The guard framework — fail-open-but-loud: the rule turns on a
        // payload field and on path resolution, so a call it cannot judge is allowed with an
        // advisory naming the unenforced rule
        Err(_) => return degraded(),
    };
    if payload.is_none() {
        return degraded();
    }
    let path = hook::field(payload, &["tool_input", "file_path"]);
    if path.is_empty() {
        return 0;
    }
    let state_file = format!("{}/WORKFLOW-STATE.txt", workflow_dir);
    if resolve(&path) != resolve(&state_file) {
        return 0;
    }
    hook::block(NAME, &blocked(&state_file))
}

fn degraded() -> i32 {
    hook::advise(&format!(
        "{}: the hook payload or the state-file path could not be read, so the direct-edit rule for the lifecycle state file could not be enforced on this call. That file has one sanctioned writer, lifecycle-kit bin/enter-stage.sh.",
        NAME
    ))
}

// spec: lifecycle-kit/SPEC.md §check-stage-entry — resolved comparison, never textual: an absolute
// path, a `./` prefix and a path through a symlinked directory all name one file, and a textual
// match catches only the spelling it was written against.
// spec: gate-sdk/SPEC.md §The crate's crosser — the resolution goes through `walk::canonicalize`,
// the crate's only one; `readlink -f` resolves an existing parent with a missing leaf, so the
// parent carries the crossing and the leaf is rejoined after it.
fn resolve(p: &str) -> PathBuf {
    let path = Path::new(p);
    if let Some(c) = walk::canonicalize(path) {
        return PathBuf::from(c);
    }
    if let (Some(parent), Some(leaf)) = (path.parent(), path.file_name()) {
        let parent = if parent.as_os_str().is_empty() {
            Path::new(".")
        } else {
            parent
        };
        if let Some(c) = walk::canonicalize(parent) {
            return PathBuf::from(c).join(leaf);
        }
    }
    PathBuf::from(p.strip_prefix("./").unwrap_or(p))
}

fn blocked(state_file: &str) -> String {
    format!(
        "{} is written by lifecycle-kit's bin/enter-stage.sh, never by hand — run 'bash lifecycle-kit/bin/enter-stage.sh <stage>' to stamp, or 'bash lifecycle-kit/bin/enter-stage.sh --rename <name>' to rename the iteration (it rewrites the queue header and column 1 of every stamp in one motion, proving columns 2 through NF unchanged). The stamp *is* the stage transition, so a hand-written line moves the cursor for every reader for the rest of the session, and every gate that would catch it fires only at commit: an uncommitted hand-stamp is never seen at all. If enter-stage refuses, that refusal is a gate verdict to resolve at its source, not to write around.",
        state_file
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §check-stage-entry — the comparison is resolved rather than
    // textual, so three spellings of one file agree and a different file does not
    #[test]
    fn three_spellings_of_the_state_file_resolve_alike() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let abs = root.join(".workflow/WORKFLOW-STATE.txt");
        let plain = abs.display().to_string();
        assert!(abs.is_file(), "the state file must exist for this comparison");
        let dotted = root.join("./.workflow/./WORKFLOW-STATE.txt").display().to_string();
        let through_dir = root
            .join(".workflow/../.workflow/WORKFLOW-STATE.txt")
            .display()
            .to_string();
        assert_eq!(resolve(&plain), resolve(&dotted));
        assert_eq!(resolve(&plain), resolve(&through_dir));
        assert_ne!(
            resolve(&plain),
            resolve(&root.join(".workflow/gap-inbox.md").display().to_string())
        );
    }

    // spec: lifecycle-kit/SPEC.md §check-stage-entry — a path whose leaf does not yet exist still
    // resolves, which is `readlink -f`'s behaviour and the case a Write creating the file takes
    #[test]
    fn a_path_with_no_leaf_on_disk_still_resolves() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let missing = root.join(".workflow/no-such-file-here.txt");
        assert!(!missing.exists(), "the fixture path must not exist");
        let dir = walk::canonicalize(root.join(".workflow")).expect("the workflow dir must resolve");
        assert_eq!(
            resolve(&missing.display().to_string()),
            PathBuf::from(dir).join("no-such-file-here.txt")
        );
    }
}
