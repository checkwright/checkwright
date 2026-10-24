// spec: installer/SPEC.md §The consumer smoke — the personal arm: `init --personal` at the lattice
// minimum on a consumer with a tracked file in the gates directory and an uncommitted edit, its
// hooks, a re-run, prove, the three refusals and the reversal
use super::consumer::{
    commits, consumer, failed, find_line, git_out, head, out, path_set, porcelain, run_line, write, Lock, GATES_DIR,
    PROFILE_DERIVED,
};
use super::masked::{refusing_gate, PIPELINE_PLANT};
use super::profiles::followups;
use super::{fail, in_consumer_fed, merged_in, refuse, say, text, Outcome, Run, Step};
use crate::programs;

const MARK: &str = "# checkwright:personal:begin";
const README_EDIT: &str = "an uncommitted line\n";
const PROJECT_SCRIPT: &str = "build.sh";

fn exclude_file(c: &str) -> Result<String, Outcome> {
    let shown = git_out(c, &["rev-parse", "--git-path", "info/exclude"])?.trim().to_string();
    Ok(if crate::walk::path_root(&shown).is_some() { shown } else { format!("{}/{}", c, shown) })
}

fn read(p: &str) -> String {
    std::fs::read_to_string(p).unwrap_or_default()
}

// spec: installer/SPEC.md §The consumer smoke — the consumer every step runs on: a tracked README
// and a tracked project file in the gates directory under a name init never writes
fn project(state: &Run, label: &str) -> Result<String, Outcome> {
    let c = consumer(state, label)?;
    write(&format!("{}/README.md", c), "# a project\n")?;
    write(&format!("{}/{}/{}", c, GATES_DIR, PROJECT_SCRIPT), "#!/bin/sh\necho build\n")?;
    git_out(&c, &["add", "README.md", &format!("{}/{}", GATES_DIR, PROJECT_SCRIPT)])?;
    git_out(&c, &["commit", "-q", "-m", "a project of its own"])?;
    Ok(c)
}

pub(super) fn personal(state: &mut Run) -> Step {
    let min = state.profile_min.clone();
    let set = path_set(&state.run_path);
    let c = project(state, "personal")?;
    super::consumer::append(&format!("{}/README.md", c), README_EDIT)?;
    let exclude = exclude_file(&c)?;
    let pre_exclude = read(&exclude);
    let (head0, index0, porcelain0, commits0) = (
        head(&c)?,
        git_out(&c, &["ls-files", "-s", "-z"])?,
        porcelain(&c)?,
        commits(&c)?,
    );

    let m = state.verb(&c, &["init", "--personal", "--profile", &min])?;
    let said = out(&m);
    if !m.succeeded() || !find_line(&said, |l| l.starts_with("INIT: ") && l.contains(" as a personal install, excluded in ")).is_some() {
        return Err(failed(&m, format!("personal arm: init --personal --profile {} did not report a personal install", min)));
    }
    if head(&c)? != head0 || commits(&c)? != commits0 || git_out(&c, &["ls-files", "-s", "-z"])? != index0 || porcelain(&c)? != porcelain0 {
        return Err(failed(&m, "personal arm: init --personal moved HEAD, the index or the porcelain, whose one entry is the uncommitted README edit"));
    }
    if !read(&format!("{}/README.md", c)).ends_with(README_EDIT) {
        return Err(fail("personal arm: init --personal lost the uncommitted README edit"));
    }
    let blocks = read(&exclude).lines().filter(|l| *l == MARK).count();
    if blocks != 1 {
        return Err(fail(format!("personal arm: the exclude file carries {} personal block(s), not one", blocks)));
    }
    let mut roster: Vec<String> = Lock::of(&c)?.files().into_iter().map(|(p, _)| p).collect();
    roster.push("checkwright.lock".to_string());
    let feed = roster.iter().map(|p| format!("{}\n", p)).collect::<String>();
    let ignored = in_consumer_fed(&programs::GIT, &["check-ignore", "--stdin"], &[], &c, feed.as_bytes())?;
    let hidden: std::collections::BTreeSet<String> =
        text(ignored.stdout().unwrap_or_default()).lines().map(|l| l.trim_end_matches('\r').to_string()).collect();
    let shown: Vec<&String> = roster.iter().filter(|p| !hidden.contains(*p)).collect();
    if !shown.is_empty() {
        return Err(fail(format!("personal arm: {} recorded path(s) do not read ignored, first {}", shown.len(), shown[0])));
    }
    if !find_line(&said, |l| l.starts_with("then: this install is personal:")).is_some() {
        return Err(failed(&m, "personal arm: init --personal printed no personal then: line"));
    }
    say(&format!("install: {} personal, HEAD, index and the README edit unchanged, {} path(s) ignored in one block", min, roster.len()));

    for line in followups(&said) {
        let toks: Vec<&str> = line.split_whitespace().collect();
        let r = run_line(&c, &toks, &set)?;
        if !r.succeeded() {
            return Err(failed(&r, format!("personal arm: the follow-up command '{}' failed", line)));
        }
    }
    write(&format!("{}/personal-plant.sh", c), PIPELINE_PLANT)?;
    git_out(&c, &["add", "personal-plant.sh"])?;
    let r = merged_in(&programs::GIT, &["commit", "-m", "chore: add a pipeline that can lose its match"], &set, &c)?;
    let Some(gate) = refusing_gate(&out(&r), &c).filter(|_| !r.succeeded()) else {
        return Err(failed(&r, "personal arm: the planted pipeline was not refused by a registered gate the pre-commit hook named"));
    };
    git_out(&c, &["rm", "-q", "--cached", "personal-plant.sh"])?;
    let _ = std::fs::remove_file(format!("{}/personal-plant.sh", c));
    git_out(&c, &["add", "README.md"])?;
    let r = merged_in(&programs::GIT, &["commit", "-m", "docs: extend the readme"], &set, &c)?;
    let hooked = find_line(&out(&r), |l| l.starts_with("pre-commit: ") && l.ends_with(" gate(s) passed.")).is_some();
    if !r.succeeded() || !hooked {
        return Err(failed(&r, "personal arm: the README edit did not commit through the installed hooks"));
    }
    say(&format!("hooks: the plant refused by {}, the README edit committed through them", gate));

    let (commits1, block1) = (commits(&c)?, read(&exclude));
    let m = state.verb(&c, &["init"])?;
    if !m.succeeded() || commits(&c)? != commits1 || read(&exclude) != block1 {
        return Err(failed(&m, "personal arm: a bare init re-run failed, committed, or rewrote the exclude file"));
    }
    say("re-run: bare init exit 0, no commit, the exclude file byte-identical");

    let scratch = format!("{}/personal-prove", state.scratch);
    std::fs::create_dir_all(&scratch).map_err(|_| refuse("personal arm: could not make prove's scratch base"))?;
    let m = state.verb_with(&c, &["prove"], &[("DEMO_TMP_DIR".to_string(), scratch)])?;
    let help = find_line(&out(&m), |l| l.trim_start().starts_with("help: this clone's install is personal")).is_some();
    if m.code() != Some(2) || !help {
        return Err(failed(&m, format!("personal arm: prove exited {} without a help line naming the personal install", m.reported_code())));
    }
    say("prove: exit 2, its help naming the personal install");

    refused(state, "process-record", &["init", "--personal", "--profile", PROFILE_DERIVED], "commits a process record", false)?;
    refused(state, "tracked-path", &["init", "--personal", "--profile", &min], &format!("{}/gates.list", GATES_DIR), true)?;
    refused(state, "no-commit", &["init", "--personal", "--no-commit"], "--personal beside --no-commit", false)?;

    let before = commits(&c)?;
    let m = state.verb(&c, &["uninstall"])?;
    if !m.succeeded() || commits(&c)? != before {
        return Err(failed(&m, "personal arm: uninstall failed or made a commit"));
    }
    if read(&exclude) != pre_exclude || super::consumer::exists(&format!("{}/checkwright.lock", c)) || !porcelain(&c)?.trim().is_empty() {
        return Err(failed(&m, "personal arm: uninstall left the exclude file off its pre-install bytes, a checkwright.lock, or a dirty porcelain"));
    }
    say("reversal: uninstall exit 0, no commit, the exclude file at its pre-install bytes, no manifest, porcelain empty");
    Ok(())
}

// spec: installer/SPEC.md §The consumer smoke — a refusal in a fresh consumer exits 2, names what it
// refused, and leaves the tree object, the porcelain and the exclude file as they were
fn refused(state: &Run, label: &str, args: &[&str], names: &str, track_registry: bool) -> Step {
    let c = project(state, &format!("personal-{}", label))?;
    if track_registry {
        write(&format!("{}/{}/gates.list", c, GATES_DIR), "# the project's own\n")?;
        git_out(&c, &["add", &format!("{}/gates.list", GATES_DIR)])?;
        git_out(&c, &["commit", "-q", "-m", "a registry of its own"])?;
    }
    let exclude = exclude_file(&c)?;
    let (tree0, porcelain0, exclude0) = (super::consumer::tree(&c)?, porcelain(&c)?, read(&exclude));
    let m = state.verb(&c, args)?;
    if m.code() != Some(2) || !out(&m).contains(names) {
        return Err(failed(&m, format!("personal arm: {} exited {} without naming {}", args.join(" "), m.reported_code(), names)));
    }
    if super::consumer::tree(&c)? != tree0 || porcelain(&c)? != porcelain0 || read(&exclude) != exclude0 {
        return Err(failed(&m, format!("personal arm: the refused {} changed the tree, the porcelain or the exclude file", args.join(" "))));
    }
    say(&format!("refusal: {} exit 2, naming {}, nothing changed", args.join(" "), names));
    Ok(())
}
