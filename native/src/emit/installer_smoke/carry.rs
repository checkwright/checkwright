// spec: installer/SPEC.md §The consumer smoke — the close-surface arm: the kits' capture logs,
// declared in the tree under test, read clean by check-close-surfaces in a vendored tree, and an
// undeclared one red
use super::consumer::{append, consumer, exists, failed, first_starting, out, path_set, seam_of, write, Lock, PROFILE_DERIVED, WORKFLOW_DIR};
use super::profiles::commit_all;
use super::{fail, merged_in, refuse, say, Outcome, Run, Step};
use crate::emit::close_surfaces::{declaration_lines, split_declaration};
use crate::{programs, walk};

const GATE: &str = "check-close-surfaces";
const CONTROL: &str = "close-surface-smoke-undeclared.log";

// spec: installer/SPEC.md §The consumer smoke — the expected set is read from the tree under test,
// never the payload: each capture-tier declaration in the canonical spec of a kit the consumer holds
fn expected(state: &Run, kits: &[String]) -> Result<Vec<String>, Outcome> {
    let basename = walk::knob_scalar("LIFECYCLE_KIT_ROSTER_BASENAME").map_err(refuse)?;
    let mut paths: Vec<String> = Vec::new();
    for root in walk::kit_roots_at(&state.root).map_err(refuse)? {
        let root = root.trim_end_matches('/');
        let leaf = root.rsplit('/').next().unwrap_or(root);
        if root.is_empty() || !kits.iter().any(|k| k == leaf) {
            continue;
        }
        let spec = walk::abs_against(&state.root, &format!("{}/{}", root, basename));
        let Ok(text) = std::fs::read_to_string(&spec) else { continue };
        for line in declaration_lines(&text) {
            let (path, _, reclaim) = split_declaration(line);
            if reclaim != "-" && !paths.contains(&path) {
                paths.push(path);
            }
        }
    }
    Ok(paths)
}

fn gate(state: &Run, c: &str) -> Result<crate::proc::Merged, Outcome> {
    let bin = seam_of(c).unwrap_or_default();
    let bin_path = format!("{}/{}", c, bin);
    if bin.is_empty() || !crate::proc::is_executable(std::path::Path::new(&bin_path)) {
        return Err(fail(format!(
            "close-surface arm: the seam names no executable gate binary at '{}' to run {} with",
            bin, GATE
        )));
    }
    merged_in(&programs::CHECKWRIGHT_GATES.at(bin_path), &[GATE], &path_set(&state.run_path), c)
}

fn ignore(c: &str, rel: &str) -> Step {
    let file = format!("{}/.gitignore", c);
    if exists(&file) {
        append(&file, &format!("{}\n", rel))
    } else {
        write(&file, &format!("{}\n", rel))
    }
}

pub(super) fn close_surface(state: &mut Run) -> Step {
    let c = consumer(state, "close-surface")?;
    let m = state.verb(&c, &["init", "--profile", PROFILE_DERIVED])?;
    if !m.succeeded() {
        return Err(failed(&m, format!("close-surface arm: init --profile {} failed", PROFILE_DERIVED)));
    }
    let kits = Lock::of(&c)?.list("kits");
    let logs = expected(state, &kits)?;
    if logs.is_empty() {
        return Err(refuse(format!(
            "close-surface arm: no kit of the {} consumer declares a capture log in the tree under test, so the arm would pass by vacuity",
            PROFILE_DERIVED
        )));
    }
    for rel in &logs {
        write(&format!("{}/{}", c, rel), "a capture line\n")?;
        ignore(&c, rel)?;
    }
    commit_all(&c, "close-surface arm: the kits' capture logs", "close-surface arm: could not commit the gitignore")?;
    let m = gate(state, &c)?;
    if !m.succeeded() {
        return Err(failed(&m, format!(
            "close-surface arm: {} is not clean over {} gitignored capture log(s) the vendored kits declare",
            GATE,
            logs.len()
        )));
    }
    say(&format!("close-surface: {} capture log(s) declared in a vendored tree — {}", logs.len(), first_starting(&out(&m), "CLOSE-SURFACES:")));

    let control = format!("{}/{}", WORKFLOW_DIR, CONTROL);
    write(&format!("{}/{}", c, control), "a capture line\n")?;
    ignore(&c, &control)?;
    let m = gate(state, &c)?;
    let finding = format!("{}: capture-tier workflow member with no 'close-surface:' declaration", control);
    if m.code() != Some(1) || !out(&m).contains(&finding) {
        return Err(failed(&m, format!(
            "close-surface arm: {} exited {} over an undeclared gitignored {}, not red naming it undeclared",
            GATE,
            m.reported_code(),
            control
        )));
    }
    std::fs::remove_file(format!("{}/{}", c, control)).map_err(|e| fail(format!("could not remove {}: {}", control, e)))?;
    let restored = super::consumer::git(&c, &["checkout", "-q", "--", ".gitignore"])?;
    if !restored.succeeded() {
        return Err(failed(&restored, "close-surface arm: could not restore the consumer's .gitignore"));
    }
    say(&format!("close-surface control: an undeclared {} reads red", control));
    Ok(())
}
