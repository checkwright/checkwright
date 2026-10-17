// spec: installer/SPEC.md §prove — demo's done claim made on the adopter's own `HEAD`, in a clone of
// its own: what the install configures, enforces and leaves inactive, the battery, the claim, and
// the verdict as the exit status and the last line of stdout.
use super::{demo, doctor, lock, workflow, GATES_DIR, QUEUE_FILE, STATE_FILE};
use crate::walkthrough::{self as show, Scratch};
use crate::{proc, programs};
use std::fmt::Write as _;
use std::path::Path;

const VERDICT: &str = "PROVE";
pub(super) const USAGE: &[&str] = &[
    "usage: checkwright prove",
    "",
    "Shows, on this repository, whether a task claimed done with no evidence is",
    "caught: clones HEAD into a scratch repository, reports what the install",
    "configures, enforces and leaves inactive, runs your battery, commits the claim",
    "there and runs it again, then removes the scratch. It writes nothing here and",
    "asks nothing. Exit status is the verdict: 0 held, 1 not held; the last line",
    "of output states it.",
];

// spec: installer/SPEC.md §prove — the three exit classes as a type, so a verdict about the tree
// cannot be raised on a precondition's spelling or the reverse
enum Outcome {
    Held { head: String, minutes: i64 },
    NotHeld { cause: String, help: String },
    Refuse { cause: String, help: String },
}

macro_rules! step {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(o) => return o,
        }
    };
}

fn refuse(cause: impl Into<String>) -> Outcome {
    Outcome::Refuse { cause: cause.into(), help: String::new() }
}

fn refuse_with(cause: impl Into<String>, help: impl Into<String>) -> Outcome {
    Outcome::Refuse { cause: cause.into(), help: help.into() }
}

pub fn run(args: &[String]) -> i32 {
    if let Some(a) = args.first() {
        if a == "-h" || a == "--help" {
            for line in USAGE {
                println!("{}", line);
            }
            return 0;
        }
        eprintln!("checkwright prove: unknown argument: {}", a);
        eprintln!("  {}", USAGE[0]);
        return 2;
    }
    let mut scratch = Scratch { dir: None };
    let outcome = prove(&mut scratch);
    drop(scratch);
    match outcome {
        Outcome::Held { head, minutes } => {
            println!("\n{}: held — caught at {}; time to proof {} min", VERDICT, head, minutes);
            0
        }
        Outcome::NotHeld { cause, help } => {
            if !help.is_empty() {
                eprintln!("  help: {}", help);
            }
            println!("\n{}: not held — {}", VERDICT, cause);
            1
        }
        Outcome::Refuse { cause, help } => {
            eprintln!("\n{}: FAIL(env) — {}", VERDICT, cause);
            if !help.is_empty() {
                eprintln!("  help: {}", help);
            }
            2
        }
    }
}

// spec: installer/SPEC.md §prove — every child runs with stdin closed and no terminal prompt, and
// meets none of the repository locators a hook exports, so a call aimed at the clone reaches it
fn git(dir: &Path, args: &[&str]) -> Result<String, String> {
    let root = dir.to_string_lossy().into_owned();
    let mut argv: Vec<&str> = vec!["-C", &root];
    argv.extend_from_slice(args);
    let set = [("GIT_TERMINAL_PROMPT".to_string(), "0".to_string())];
    let unset: Vec<String> = proc::GIT_REPO_LOCATORS.iter().map(|s| s.to_string()).collect();
    let env = proc::ChildEnv { set: &set, unset: &unset, cwd: None };
    let done = proc::run_with_stdin_in(&programs::GIT, &argv, b"", &env)?;
    match done.stdout() {
        Some(out) => Ok(String::from_utf8_lossy(out).into_owned()),
        None => Err(format!("git {} failed — {}", args.join(" "), done.failure_report().unwrap_or_default())),
    }
}

fn git_step(dir: &Path, args: &[&str]) -> Result<String, Outcome> {
    git(dir, args).map_err(refuse)
}

fn prove(scratch: &mut Scratch) -> Outcome {
    // spec: installer/SPEC.md §prove — the preconditions, each refusing at exit 2 before the clone
    let root = step!(super::required_root("prove reads the repository it is run in; run it inside the repository `init` installed into.")
        .map_err(|r| refuse_with(r.message, r.help)));
    let Ok(sha) = git(&root, &["rev-parse", "--verify", "-q", "HEAD^{commit}"]) else {
        return refuse_with("HEAD names no commit", "prove reads HEAD; run init and commit before proving.");
    };
    let sha = sha.trim().to_string();
    let init_help = format!(
        "run checkwright init here and commit what it writes; prove reads the {} HEAD carries.",
        lock::FILE
    );
    let Ok(lock_text) = git(&root, &["show", &format!("HEAD:{}", lock::FILE)]) else {
        return refuse_with(format!("HEAD carries no {}", lock::FILE), init_help);
    };
    let Some(manifest) = lock::Manifest::parse(&lock_text).filter(lock::Manifest::schema_ok) else {
        return refuse_with(format!("the {} at HEAD carries a schema this build does not know", lock::FILE), init_help);
    };
    if manifest.field("version").is_empty() {
        return refuse_with(format!("the {} at HEAD records no install, only files a removal kept", lock::FILE), init_help);
    }

    // spec: installer/SPEC.md §prove — the clone, under demo's base and with demo's local settings,
    // checked out detached at HEAD's commit
    let clone = step!(demo::make_scratch(scratch, "checkwright-prove").map_err(refuse));
    let (source, target) = (root.to_string_lossy().into_owned(), clone.to_string_lossy().into_owned());
    step!(git_step(&root, &["clone", "-q", "--no-checkout", "--", &source, &target]));
    for (k, v) in demo::SCRATCH_CONFIG {
        step!(git_step(&clone, &["config", k, v]));
    }
    step!(git_step(&clone, &["checkout", "-q", "--detach", &sha]));
    let head = step!(git_step(&clone, &["rev-parse", "--short", "HEAD"])).trim().to_string();

    let seam = manifest.own_file(&format!("{}/gate-sdk-config.knobs", GATES_DIR));
    let Some(bin) = super::seam_binary(&clone, &seam) else {
        return refuse_with("HEAD carries no gate binary at the path GATE_SDK_NATIVE_BIN names", init_help);
    };
    let gates = doctor::tree_decl_dirs(&clone)
        .map(|(g, _)| g)
        .unwrap_or_else(|| clone.join(crate::knobs::gates_dir()).display().to_string());
    if let Ok(tmp) = crate::knobs::wire_in(&gates, "GATE_SDK_TMP_DIR") {
        if let Some(rel) = in_clone(&root, &tmp) {
            if !crate::walk::names_cwd(Path::new(&rel)) {
                let _ = crate::walk::make_scratch(clone.join(rel));
            }
        }
    }

    // spec: installer/SPEC.md §prove — the report: configured, enforced, inactive, then the note
    let door = super::init::command_token(&bin);
    print!("{}", report(&root, &clone, &manifest, &door));

    // spec: installer/SPEC.md §prove — the battery at HEAD; a red one does not stop the run
    println!("\nbattery at HEAD");
    let (_, out) = step!(battery(&clone, &bin));
    show::say(&summary(&out));

    // spec: installer/SPEC.md §prove — the claim, over the files a battery at the clone's root reads
    println!("\nthe claim");
    let queue_rel = resolved(&root, &gates, "LIFECYCLE_KIT_QUEUE_FILE", QUEUE_FILE);
    let state_rel = resolved(&root, &gates, "LIFECYCLE_KIT_STATE_FILE", STATE_FILE);
    let help = format!(
        "re-run init with --profile {}, the profile demo installs, to keep a task queue and stage state; init --help lists the profiles and the --with-kit selection.",
        super::profile::DERIVED
    );
    let mut texts = Vec::new();
    for (label, rel) in [(QUEUE_FILE, &queue_rel), (STATE_FILE, &state_rel)] {
        match rel.as_ref().and_then(|r| std::fs::read_to_string(clone.join(r)).ok()) {
            Some(t) => texts.push(t),
            None => {
                let named = rel.as_deref().unwrap_or(label);
                return Outcome::NotHeld {
                    cause: format!("this tree keeps no {} to claim a task done in", named),
                    help,
                };
            }
        }
    }
    let (queue_rel, state_rel) = (queue_rel.unwrap_or_default(), state_rel.unwrap_or_default());
    let today = crate::emit::kpi::iso_day(crate::emit::kpi::now_epoch().div_euclid(86_400));
    let (claimed_queue, claimed_state) = match demo::claim(&texts[0], &texts[1], &head, &today) {
        Ok(c) => c,
        Err(m) => {
            return Outcome::NotHeld {
                cause: format!("{} keeps no {} to claim a task done in", queue_rel, m.part()),
                help: String::new(),
            }
        }
    };
    for (rel, body) in [(&queue_rel, &claimed_queue), (&state_rel, &claimed_state)] {
        if let Err(e) = std::fs::write(clone.join(rel), body) {
            return refuse(format!("cannot write {} in the clone: {}", rel, e));
        }
    }
    step!(git_step(&clone, &["add", "--", &queue_rel, &state_rel]));
    step!(git_step(&clone, &["commit", "-q", "--no-verify", "-m", &format!("done: {}", demo::TASK)]));
    show::say(&format!(
        "a task moved to Done and validate and close stamped for iteration {}, with no evidence behind them",
        demo::ITERATION
    ));
    let (rc, out) = step!(battery(&clone, &bin));
    let quoted = if rc == 0 { None } else { demo::caught(&out) };
    let Some(quoted) = quoted else {
        show::say(&summary(&out));
        return Outcome::NotHeld { cause: "the claim was not caught".to_string(), help: String::new() };
    };
    for block in &quoted {
        println!();
        for line in block {
            println!("{}", line);
        }
    }

    // spec: installer/SPEC.md §prove — time to proof, from the install commit to this run's clock
    let added = step!(git_step(
        &clone,
        &["log", "--full-history", "--no-renames", "--diff-filter=A", "--format=%ct", "HEAD", "--", lock::FILE]
    ));
    let since = added.lines().filter_map(|l| l.trim().parse::<i64>().ok()).min();
    let minutes = since.map(|t| minutes_since(t, crate::emit::kpi::now_epoch())).unwrap_or(0);
    Outcome::Held { head, minutes }
}

// spec: installer/SPEC.md §prove — whole minutes, and a committer time ahead of the clock reads as 0
fn minutes_since(then: i64, now: i64) -> i64 {
    (now - then).max(0) / 60
}

// spec: installer/SPEC.md §prove — a knob's path inside the repository, rebased onto the clone; a
// path outside the repository is none of the clone's
fn in_clone(root: &Path, value: &str) -> Option<String> {
    if crate::walk::path_root(value).is_none() {
        return Some(value.to_string());
    }
    let rel = Path::new(value).strip_prefix(root).ok()?;
    Some(rel.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/"))
}

fn resolved(root: &Path, gates: &str, knob: &str, seeded: &str) -> Option<String> {
    match crate::knobs::wire_in(gates, knob) {
        Ok(v) if !v.is_empty() => in_clone(root, &v),
        _ => Some(seeded.to_string()),
    }
}

fn report(root: &Path, clone: &Path, manifest: &lock::Manifest, door: &str) -> String {
    let mut out = String::new();
    let kits: Vec<String> = manifest.field("kits").split_whitespace().map(String::from).collect();
    let list = manifest.own_file(&format!("{}/gates.list", GATES_DIR));
    let list_text = if list.is_empty() { None } else { std::fs::read_to_string(clone.join(&list)).ok() };
    out.push_str("configured\n");
    let _ = writeln!(out, "  {:<12} {}", "profile", manifest.field("profile"));
    let _ = writeln!(out, "  {:<12} {} — {}", "kits", kits.len(), kits.join(" "));
    match &list_text {
        Some(t) => {
            let _ = writeln!(out, "  {:<12} {} member(s) in {}", "registry", crate::registry::members(t).len(), list);
        }
        None => {
            let _ = writeln!(out, "  {:<12} none recorded at HEAD — re-run init", "registry");
        }
    }

    let hooks = crate::emit::hook_launcher::hooks_dir(root);
    let opted = hooks.as_deref().is_some_and(|d| crate::emit::hook_launcher::opted_in(root, d));
    let gone = hooks.as_deref().map(crate::emit::hook_launcher::missing).unwrap_or_default();
    out.push_str("enforced\n");
    let _ = writeln!(out, "  {:<12} {}", "at commit", doctor::hooks_line(opted, &gone));
    let ci = if clone.join(workflow::PATH).is_file() { workflow::PATH } else { "none seeded" };
    let _ = writeln!(out, "  {:<12} {}", "in CI", ci);

    out.push_str("inactive\n");
    if let Some(t) = &list_text {
        doctor::tree_disarmed_block(&mut out, clone, t);
    }
    if !opted {
        let _ = writeln!(out, "  {:<12} not opted in — no hook runs the battery at commit until {} --install-hooks", "hooks", door);
    }

    if git(root, &["status", "--porcelain", "-z"]).is_ok_and(|s| !s.trim().is_empty()) {
        out.push_str("note: your worktree differs from HEAD; this run read HEAD alone.\n");
    }
    out
}

// spec: installer/SPEC.md §prove — the battery is the clone's own door, spawned in the clone
fn battery(clone: &Path, bin: &str) -> Result<(i32, String), Outcome> {
    let door = clone.join(bin);
    let set = [("GIT_TERMINAL_PROMPT".to_string(), "0".to_string())];
    let unset: Vec<String> = proc::GIT_REPO_LOCATORS.iter().map(|s| s.to_string()).collect();
    let m = proc::run_merged_scrubbed(
        &programs::CHECKWRIGHT_GATES.at(door.to_string_lossy().into_owned()),
        &["--run"],
        &set,
        clone,
        &unset,
    )
    .map_err(refuse)?;
    Ok((m.reported_code(), String::from_utf8_lossy(m.output()).into_owned()))
}

// spec: installer/SPEC.md §prove — the battery's own summary line, or its last line where it printed
// none
fn summary(out: &str) -> String {
    out.lines()
        .rev()
        .find(|l| l.contains("gates passed") || l.contains("gates FAILED"))
        .or_else(|| out.lines().rev().find(|l| !l.trim().is_empty()))
        .unwrap_or("the battery printed nothing")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    // spec: installer/SPEC.md §prove — `--help` answers before any precondition, and any other word
    // is a usage refusal
    #[test]
    fn help_answers_and_an_operand_is_refused() {
        assert_eq!(super::run(&["--help".to_string()]), 0);
        assert_eq!(super::run(&["extra".to_string()]), 2);
    }

    // spec: installer/SPEC.md §prove — whole minutes, never negative
    #[test]
    fn time_to_proof_is_whole_minutes_and_never_negative() {
        assert_eq!(super::minutes_since(1_000, 1_000 + 59), 0);
        assert_eq!(super::minutes_since(1_000, 1_000 + 61), 1);
        assert_eq!(super::minutes_since(1_000, 1_000 + 3_600), 60);
        assert_eq!(super::minutes_since(2_000, 1_000), 0);
    }

    // spec: installer/SPEC.md §prove — a knob's relative path is the clone's, a rooted one inside the
    // repository is rebased onto it, and one outside it is none of the clone's
    #[test]
    fn a_knob_path_is_rebased_onto_the_clone_or_refused() {
        let root = std::path::Path::new("/repo");
        assert_eq!(super::in_clone(root, "TASK-QUEUE.md").as_deref(), Some("TASK-QUEUE.md"));
        assert_eq!(super::in_clone(root, "/repo/docs/q.md").as_deref(), Some("docs/q.md"));
        assert_eq!(super::in_clone(root, "/elsewhere/q.md"), None);
    }

    // spec: installer/SPEC.md §prove — the summary is the battery's own line, else its last
    #[test]
    fn the_summary_is_the_batterys_own_line() {
        assert_eq!(super::summary("x\nAll 9 gates passed\n"), "All 9 gates passed");
        assert_eq!(super::summary("1 of 9 gates FAILED: check-x\ntrailer\n"), "1 of 9 gates FAILED: check-x");
        assert_eq!(super::summary("only this\n\n"), "only this");
    }
}
