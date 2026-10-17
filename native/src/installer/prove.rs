// spec: installer/SPEC.md §prove — demo's done claim made on the adopter's own `HEAD`, in a clone of
// its own: what the install configures, enforces and leaves inactive, the battery, the claim, and
// the verdict as the exit status and the last line of stdout.
use super::{demo, doctor, lock, workflow, GATES_DIR, QUEUE_FILE, STATE_FILE};
use crate::gates::task_path_claim::LISTS_KNOB;
use crate::walkthrough::{self as show, Scratch};
use crate::{proc, programs};
use std::fmt::Write as _;
use std::path::Path;

const VERDICT: &str = "PROVE";
const QUEUE_KNOB: &str = "LIFECYCLE_KIT_QUEUE_FILE";
const STATE_KNOB: &str = "LIFECYCLE_KIT_STATE_FILE";
// spec: installer/SPEC.md §prove — a clone-relative path no clone holds, git tracking nothing in its
// own directory
const ABSENT: &str = ".git/checkwright-prove-absent";
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
    step!(git_step(&root, &clone_argv(&source, &target)));
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
    if let Ok(tmp) = crate::knobs::wire_in(&clone, &gates, "GATE_SDK_TMP_DIR") {
        if let Some(rel) = in_clone(&root, &clone, &tmp) {
            let _ = crate::walk::make_scratch(clone.join(rel));
        }
    }

    // spec: installer/SPEC.md §prove — the report: configured, enforced, inactive, then the note
    let door = super::init::command_token(&bin);
    print!("{}", report(&root, &clone, &manifest, &door));

    // spec: installer/SPEC.md §prove — both battery runs read the claim's files at the paths resolved
    // here, so an absolute knob rebased onto the clone is the file the battery reads
    let queue = resolved(&root, &clone, &gates, QUEUE_KNOB, QUEUE_FILE);
    let state = resolved(&root, &clone, &gates, STATE_KNOB, STATE_FILE);
    let claim_env = claim_env(&[&queue, &state]);

    // spec: installer/SPEC.md §prove — the battery at HEAD; a red one does not stop the run
    println!("\nbattery at HEAD");
    let (_, out) = step!(battery(&clone, &bin, &claim_env));
    show::say(&summary(&out));

    // spec: installer/SPEC.md §prove — the claim, over the files a battery at the clone's root reads
    println!("\nthe claim");
    let claim = step!(stage(&clone, &gates, &manifest, &queue, &state, &head));
    for (rel, body) in &claim.writes {
        if let Err(e) = std::fs::write(clone.join(rel), body) {
            return refuse(format!("cannot write {} in the clone: {}", rel, e));
        }
    }
    let mut add = vec!["add", "--"];
    add.extend(claim.writes.iter().map(|(rel, _)| rel.as_str()));
    step!(git_step(&clone, &add));
    step!(git_step(&clone, &["commit", "-q", "--no-verify", "-m", &format!("done: {}", demo::TASK)]));
    show::say(&claim.said);
    let (rc, out) = step!(battery(&clone, &bin, &claim_env));
    let quoted = if rc == 0 { None } else { demo::caught(&out, &claim.named) };
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

// spec: installer/SPEC.md §prove — a path clone, never `--local`: git then dies on a scratch on
// another filesystem rather than copying the objects it cannot hardlink
fn clone_argv<'a>(source: &'a str, target: &'a str) -> [&'a str; 6] {
    ["clone", "-q", "--no-checkout", "--", source, target]
}

// spec: installer/SPEC.md §prove — whole minutes, and a committer time ahead of the clock reads as 0
fn minutes_since(then: i64, now: i64) -> i64 {
    (now - then).max(0) / 60
}

// spec: installer/SPEC.md §prove — a knob's path inside the repository, rebased onto the clone and
// resolved on its filesystem; a path outside the repository is none of the clone's
fn in_clone(root: &Path, clone: &Path, value: &str) -> Option<String> {
    let v = Path::new(value);
    let rel = if crate::walk::path_root(value).is_none() {
        v.to_path_buf()
    } else {
        match v.strip_prefix(root) {
            Ok(r) => r.to_path_buf(),
            Err(_) => v.strip_prefix(real_path(root)?).ok()?.to_path_buf(),
        }
    };
    confined(clone, &rel)
}

// spec: installer/SPEC.md §prove — a clone-relative path as the clone's filesystem resolves it, each
// symlink followed and each `..` taken where the filesystem takes it, as the clone-relative path of
// the file it lands on; a path landing outside the clone, or on a dangling symlink, is none
fn confined(clone: &Path, rel: &Path) -> Option<String> {
    use std::path::Component;
    if rel.components().any(|c| matches!(c, Component::Prefix(_) | Component::RootDir)) {
        return None;
    }
    let top = real_path(clone)?;
    let mut probe = top.join(rel);
    let mut absent = Vec::new();
    while std::fs::symlink_metadata(&probe).is_err() {
        absent.push(probe.file_name()?.to_os_string());
        probe = probe.parent()?.to_path_buf();
    }
    let mut real = real_path(&probe)?;
    real.extend(absent.iter().rev());
    let inside = real.strip_prefix(&top).ok()?;
    if inside.as_os_str().is_empty() {
        return None;
    }
    Some(inside.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/"))
}

// spec: gate-sdk/SPEC.md §The crate's crosser — the resolved path, its extended prefix stripped since
// `confined` composes onto it
fn real_path(p: &Path) -> Option<std::path::PathBuf> {
    crate::walk::canonicalize(p).map(|c| std::path::PathBuf::from(crate::walk::strip_extended_prefix(&c)))
}

// spec: installer/SPEC.md §prove — one claim file: the name the tree gives it and where it lands in
// the clone, `None` where it names no file of the clone's
struct ClaimFile {
    knob: &'static str,
    named: String,
    rel: Option<String>,
}

fn resolved(root: &Path, clone: &Path, gates: &str, knob: &'static str, seeded: &str) -> ClaimFile {
    match crate::knobs::wire_in(clone, gates, knob) {
        Ok(v) if !v.is_empty() => ClaimFile { knob, rel: in_clone(root, clone, &v), named: v },
        _ => ClaimFile { knob, named: seeded.to_string(), rel: confined(clone, Path::new(seeded)) },
    }
}

// spec: installer/SPEC.md §prove — every battery run sets both knobs, a file outside the clone to a
// path the clone does not hold, so no value inherited from the environment reaches past the clone
fn claim_env(files: &[&ClaimFile]) -> Vec<(String, String)> {
    files
        .iter()
        .map(|f| {
            let at = f.rel.clone().unwrap_or_else(|| format!("{}/{}", ABSENT, f.knob));
            (f.knob.to_string(), at)
        })
        .collect()
}

// spec: installer/SPEC.md §prove — one staged claim: each file it writes with its new body, the
// string a reddened block must name to catch it, and the line the run says of it
struct Claim {
    writes: Vec<(String, String)>,
    named: String,
    said: String,
}

// spec: installer/SPEC.md §prove — the stage claim where the clone holds both lifecycle files, else
// the task claim
fn stage(clone: &Path, gates: &str, manifest: &lock::Manifest, queue: &ClaimFile, state: &ClaimFile, head: &str) -> Result<Claim, Outcome> {
    let read = |f: &ClaimFile| f.rel.as_ref().and_then(|r| std::fs::read_to_string(clone.join(r)).ok());
    let (q, s) = match (read(queue), read(state)) {
        (Some(q), Some(s)) => (q, s),
        (q, _) => return task_claim(clone, gates, manifest, if q.is_none() { queue } else { state }),
    };
    let (queue_rel, state_rel) = (queue.rel.clone().unwrap_or_default(), state.rel.clone().unwrap_or_default());
    let today = crate::emit::kpi::iso_day(crate::emit::kpi::now_epoch().div_euclid(86_400));
    let (cq, cs) = demo::claim(&q, &s, head, &today).map_err(|m| Outcome::NotHeld {
        cause: format!("{} keeps no {} to claim a task done in", queue_rel, m.part()),
        help: String::new(),
    })?;
    Ok(Claim {
        writes: vec![(queue_rel, cq), (state_rel, cs)],
        named: demo::ITERATION.to_string(),
        said: format!(
            "a task moved to Done and validate and close stamped for iteration {}, with no evidence behind them",
            demo::ITERATION
        ),
    })
}

// spec: installer/SPEC.md §prove — the task claim: demo's task ticked done naming a path under a
// top-level directory named for the verb, appended to the first task list resolving in the clone
fn task_claim(clone: &Path, gates: &str, manifest: &lock::Manifest, missing: &ClaimFile) -> Result<Claim, Outcome> {
    let Some(list) = task_list(clone, gates).map_err(refuse)? else {
        return Err(Outcome::NotHeld {
            cause: format!(
                "this tree keeps no {} and no task list to claim a task done in",
                missing.rel.as_deref().unwrap_or(&missing.named)
            ),
            help: not_held_help(manifest),
        });
    };
    let path = format!("prove/{}.md", demo::TASK);
    if std::fs::symlink_metadata(clone.join(&path)).is_ok() {
        return Err(refuse(format!("the clone already holds {}, the path the task claim names", path)));
    }
    let mut body = std::fs::read_to_string(clone.join(&list))
        .map_err(|e| refuse(format!("cannot read the task list {} in the clone: {}", list, e)))?;
    if !body.is_empty() && !body.ends_with('\n') {
        body.push('\n');
    }
    let _ = writeln!(body, "- [x] {} in {}", demo::TASK, path);
    let said = format!("a task ticked done in {} naming {}, a path this tree does not hold", list, path);
    Ok(Claim { writes: vec![(list, body)], named: path, said })
}

// spec: installer/SPEC.md §prove — the first task list in path order resolving inside the clone,
// the globs and the prune set read from the clone as a battery at its root reads them
fn task_list(clone: &Path, gates: &str) -> Result<Option<String>, String> {
    let knob = |k: &str| crate::knobs::wire_in(clone, gates, k);
    let globs: Vec<String> = knob(LISTS_KNOB)?.split('\t').filter(|g| !g.is_empty()).map(String::from).collect();
    if globs.is_empty() {
        return Ok(None);
    }
    let mut prune: Vec<String> = Vec::new();
    for k in ["GATE_SDK_PRUNE_DIRS", "GATE_SDK_PRUNE_EXTRA_DIRS"] {
        prune.extend(knob(k)?.split_whitespace().map(String::from));
    }
    let top = clone.display().to_string();
    let lists = crate::gates::task_path_claim::task_lists_pruned(&top, &globs, &|n| prune.iter().any(|d| d == n))?;
    Ok(lists.iter().find_map(|l| Path::new(l).strip_prefix(clone).ok().and_then(|r| confined(clone, r))))
}

// spec: installer/SPEC.md §prove — the no-file help names the profile demo installs, and the task
// list knob where the lock carries the kit owning it
fn not_held_help(manifest: &lock::Manifest) -> String {
    let profile = format!(
        "re-run init with --profile {}, the profile demo installs, to keep a task queue and stage state",
        super::profile::DERIVED
    );
    let kits = manifest.field("kits");
    let owned = crate::knobs::static_owner(LISTS_KNOB).is_some_and(|k| kits.split_whitespace().any(|n| n == k.root));
    let knob = if owned { format!(", or name a task list in {}", LISTS_KNOB) } else { String::new() };
    format!("{}{}; init --help lists the profiles and the --with-kit selection.", profile, knob)
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
fn battery(clone: &Path, bin: &str, claim_env: &[(String, String)]) -> Result<(i32, String), Outcome> {
    let door = clone.join(bin);
    let mut set = vec![("GIT_TERMINAL_PROMPT".to_string(), "0".to_string())];
    set.extend_from_slice(claim_env);
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

    // spec: installer/SPEC.md §prove — the clone is by path and never `--local`
    #[test]
    fn the_clone_is_by_path_and_never_local() {
        let argv = super::clone_argv("/src", "/dst");
        assert!(!argv.contains(&"--local"), "{:?}", argv);
        assert_eq!(&argv[argv.len() - 2..], &["/src", "/dst"]);
    }

    // spec: installer/SPEC.md §prove — whole minutes, never negative
    #[test]
    fn time_to_proof_is_whole_minutes_and_never_negative() {
        assert_eq!(super::minutes_since(1_000, 1_000 + 59), 0);
        assert_eq!(super::minutes_since(1_000, 1_000 + 61), 1);
        assert_eq!(super::minutes_since(1_000, 1_000 + 3_600), 60);
        assert_eq!(super::minutes_since(2_000, 1_000), 0);
    }

    struct Tree(std::path::PathBuf);

    impl Tree {
        fn new(tag: &str) -> Tree {
            let d = std::env::temp_dir().join(format!("checkwright-prove-{}.{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&d);
            std::fs::create_dir_all(d.join("clone/docs")).expect("scratch");
            std::fs::create_dir_all(d.join("outside")).expect("scratch");
            std::fs::write(d.join("clone/TASK-QUEUE.md"), "q").expect("write");
            std::fs::write(d.join("clone/docs/q.md"), "q").expect("write");
            Tree(d)
        }
        fn clone(&self) -> std::path::PathBuf {
            self.0.join("clone")
        }
    }

    impl Drop for Tree {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    // spec: installer/SPEC.md §prove — a knob's relative path is the clone's, a rooted one inside the
    // repository is rebased onto it, and one outside it is none of the clone's
    #[test]
    fn a_knob_path_is_rebased_onto_the_clone_or_refused() {
        let t = Tree::new("rebase");
        let (root, clone) = (std::path::Path::new("/repo"), t.clone());
        let at = |v: &str| super::in_clone(root, &clone, v);
        assert_eq!(at("TASK-QUEUE.md").as_deref(), Some("TASK-QUEUE.md"));
        assert_eq!(at("/repo/docs/q.md").as_deref(), Some("docs/q.md"));
        assert_eq!(at("docs/absent/new.md").as_deref(), Some("docs/absent/new.md"));
        assert_eq!(at("/elsewhere/q.md"), None);
        assert_eq!(at("./docs/../TASK-QUEUE.md").as_deref(), Some("TASK-QUEUE.md"));
        assert_eq!(at("../../TASK-QUEUE.md"), None);
        assert_eq!(at("docs/../../q.md"), None);
        assert_eq!(at("/repo/../q.md"), None);
        assert_eq!(at("."), None);
    }

    // spec: installer/SPEC.md §prove — a committed symlink out of the clone names none of its files,
    // so the claim is never read or written through it
    #[cfg(unix)]
    #[test]
    fn a_symlink_out_of_the_clone_names_none_of_its_files() {
        let t = Tree::new("symlink-out");
        let clone = t.clone();
        std::fs::write(t.0.join("outside/TASK-QUEUE.md"), "theirs").expect("write");
        std::os::unix::fs::symlink("../outside/TASK-QUEUE.md", clone.join("queue.md")).expect("symlink");
        std::os::unix::fs::symlink("../outside", clone.join("away")).expect("symlink");
        std::os::unix::fs::symlink("../outside/gone.md", clone.join("dangling.md")).expect("symlink");
        std::os::unix::fs::symlink("docs/q.md", clone.join("near.md")).expect("symlink");
        let at = |v: &str| super::in_clone(std::path::Path::new("/repo"), &clone, v);
        assert_eq!(at("queue.md"), None);
        assert_eq!(at("away/TASK-QUEUE.md"), None);
        assert_eq!(at("away/new.md"), None);
        assert_eq!(at("dangling.md"), None);
        assert_eq!(at("near.md").as_deref(), Some("docs/q.md"));
    }

    // spec: installer/SPEC.md §prove — `..` after a directory symlink is taken where the filesystem
    // takes it, so the knob selects the file the battery reads, never the lexical one
    #[cfg(unix)]
    #[test]
    fn a_parent_step_after_a_directory_symlink_follows_the_filesystem() {
        let t = Tree::new("dotdot");
        let clone = t.clone();
        std::fs::create_dir_all(clone.join("real/sub")).expect("scratch");
        std::fs::write(clone.join("real/TASK-QUEUE.md"), "real").expect("write");
        std::os::unix::fs::symlink("real/sub", clone.join("alias")).expect("symlink");
        let at = super::in_clone(std::path::Path::new("/repo"), &clone, "alias/../TASK-QUEUE.md");
        assert_eq!(at.as_deref(), Some("real/TASK-QUEUE.md"));
    }

    fn lists_at(clone: &std::path::Path) -> String {
        std::fs::create_dir_all(clone.join("scripts")).expect("scratch");
        std::fs::write(clone.join("scripts/canon-config.knobs"), "CANON_KIT_TASK_LIST_GLOBS[] = specs/*/tasks.md\n")
            .expect("write");
        clone.join("scripts").display().to_string()
    }

    // spec: installer/SPEC.md §prove — the task claim lands at the end of the first task list in path
    // order, the clone's knob naming the lists, as a new ticked item naming the absent path
    #[test]
    fn the_task_claim_ticks_demos_task_at_the_end_of_the_first_list() {
        let _env = crate::knobenv::lock();
        let t = Tree::new("task-claim");
        let clone = t.clone();
        let gates = lists_at(&clone);
        for d in ["specs/b", "specs/a"] {
            std::fs::create_dir_all(clone.join(d)).expect("scratch");
        }
        std::fs::write(clone.join("specs/a/tasks.md"), "- [x] T001 one").expect("write");
        std::fs::write(clone.join("specs/b/tasks.md"), "- [ ] T002 two\n").expect("write");
        let missing = super::ClaimFile { knob: super::STATE_KNOB, named: "s".into(), rel: Some("s".into()) };
        let lock = super::lock::Manifest::parse(r#"{"kits":["gate-sdk","canon-kit"]}"#).expect("lock");
        let Ok(claim) = super::task_claim(&clone, &gates, &lock, &missing) else { panic!("no task claim staged") };
        assert_eq!(claim.named, "prove/add-login-page.md");
        assert_eq!(claim.writes.len(), 1);
        assert_eq!(claim.writes[0].0, "specs/a/tasks.md");
        assert_eq!(claim.writes[0].1, "- [x] T001 one\n- [x] add-login-page in prove/add-login-page.md\n");
        std::fs::create_dir_all(clone.join("prove")).expect("scratch");
        std::fs::write(clone.join("prove/add-login-page.md"), "").expect("write");
        assert!(matches!(super::task_claim(&clone, &gates, &lock, &missing), Err(super::Outcome::Refuse { .. })));
    }

    // spec: installer/SPEC.md §prove — a tree keeping no task list answers not held, its help naming
    // the task list knob only where the lock carries the kit owning it
    #[test]
    fn no_task_list_is_not_held_and_the_help_names_the_knob_by_the_lock() {
        let _env = crate::knobenv::lock();
        let t = Tree::new("no-list");
        let clone = t.clone();
        let gates = lists_at(&clone);
        let missing = super::ClaimFile { knob: super::QUEUE_KNOB, named: "TASK-QUEUE.md".into(), rel: None };
        for (kits, names) in [(r#"["canon-kit"]"#, true), (r#"["gate-sdk"]"#, false)] {
            let lock = super::lock::Manifest::parse(&format!(r#"{{"kits":{}}}"#, kits)).expect("lock");
            let Err(super::Outcome::NotHeld { cause, help }) = super::task_claim(&clone, &gates, &lock, &missing) else {
                panic!("a tree with no task list staged a claim")
            };
            assert_eq!(cause, "this tree keeps no TASK-QUEUE.md and no task list to claim a task done in");
            assert_eq!(help.contains(super::LISTS_KNOB), names, "{}", help);
            assert!(help.contains("--profile full"), "{}", help);
        }
    }

    // spec: installer/SPEC.md §prove — a task list that is a symlink out of the clone is no list of
    // the clone's, so the claim is never written through it and the walk takes the next
    #[cfg(unix)]
    #[test]
    fn a_task_list_symlinked_out_of_the_clone_is_never_claimed_in() {
        let _env = crate::knobenv::lock();
        let t = Tree::new("list-out");
        let clone = t.clone();
        let gates = lists_at(&clone);
        std::fs::write(t.0.join("outside/tasks.md"), "- [x] theirs\n").expect("write");
        std::fs::create_dir_all(clone.join("specs/a")).expect("scratch");
        std::os::unix::fs::symlink("../../../outside/tasks.md", clone.join("specs/a/tasks.md")).expect("symlink");
        assert_eq!(super::task_list(&clone, &gates), Ok(None));
        std::fs::create_dir_all(clone.join("specs/b")).expect("scratch");
        std::fs::write(clone.join("specs/b/tasks.md"), "- [ ] mine\n").expect("write");
        assert_eq!(super::task_list(&clone, &gates).as_ref().map(|o| o.as_deref()), Ok(Some("specs/b/tasks.md")));
    }

    // spec: installer/SPEC.md §prove — every battery run sets both knobs, one naming nothing of the
    // clone's to a path no clone holds, so an outside value in the environment is overridden
    #[test]
    fn every_battery_run_sets_both_knobs() {
        let inside = super::ClaimFile { knob: super::QUEUE_KNOB, named: "q.md".into(), rel: Some("q.md".into()) };
        let outside = super::ClaimFile { knob: super::STATE_KNOB, named: "/elsewhere/s".into(), rel: None };
        let env = super::claim_env(&[&inside, &outside]);
        assert_eq!(env.len(), 2, "{:?}", env);
        assert_eq!(env[0], (super::QUEUE_KNOB.to_string(), "q.md".to_string()));
        assert_eq!(env[1].0, super::STATE_KNOB);
        assert!(env[1].1.starts_with(super::ABSENT) && !env[1].1.contains("elsewhere"), "{:?}", env[1]);
    }

    // spec: installer/SPEC.md §prove — the summary is the battery's own line, else its last
    #[test]
    fn the_summary_is_the_batterys_own_line() {
        assert_eq!(super::summary("x\nAll 9 gates passed\n"), "All 9 gates passed");
        assert_eq!(super::summary("1 of 9 gates FAILED: check-x\ntrailer\n"), "1 of 9 gates FAILED: check-x");
        assert_eq!(super::summary("only this\n\n"), "only this");
    }
}
