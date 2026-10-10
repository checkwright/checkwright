// spec: gate-sdk/SPEC.md §install-hooks — the per-clone hook opt-in: place both hooks in the
// clone's git directory, point core.hooksPath at them, verify the git identity once, and report
// what was placed.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the contract is three-state and
// every code is load-bearing: `check-identity`'s 1 propagates through, and an emitting arm cannot
// carry it.
use super::hook_launcher;
use crate::{proc, programs};
use crate::registry;
use crate::walk;
use std::path::{Path, PathBuf};

// spec: gate-sdk/SPEC.md §install-hooks — this arm's own names, then those its registry-resolved
// callee declares
pub const KNOBS: &[&str] = &[
    "GATE_SDK_GATES_DIR",
    "GATE_SDK_KIT_DIRS",
    "GATE_SDK_NATIVE_BIN",
    "GATE_SDK_IDENTITY_FILE",
    "GATE_SDK_GIT_EMAIL_FILE",
    "GATE_SDK_GIT_REMOTES_FILE",
    "GATE_SDK_GH_HOSTS_FILE",
    "GATE_SDK_GH_HOST",
];

const IDENTITY: &str = "check-identity";

const REFRESH: &str = "--refresh";

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the member takes one option and no positional,
// so any other token is a refusal before either knob resolves; usage itself lives on this arm's own
// front-end `case` arm, which the class gives every member holding one.
const USAGE: &str = "usage: --install-hooks [--refresh]
  --refresh re-places the hooks of a clone already opted in and changes nothing else.
  Takes no other argument: the rest of the input is the GATE_SDK_* configuration.";

pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("install-hooks: {}", e);
            2
        }
    }
}

fn absent(bin: &str) -> String {
    format!("no gate binary at {} — place or build the binary GATE_SDK_NATIVE_BIN names first", bin)
}

// spec: gate-sdk/SPEC.md §install-hooks — the hooks directory `--refresh` re-places into: `None`
// off the work-tree top, where the binary's path and the directory would resolve against another
// tree, and in a clone not opted in, which a refresh never opts in
fn refresh_target(here: &str) -> Result<Option<PathBuf>, String> {
    let Some(top) = walk::toplevel_in_opt(here)? else {
        return Ok(None);
    };
    if !hook_launcher::same_dir(Path::new(&top), Path::new(here)) {
        return Ok(None);
    }
    Ok(hook_launcher::hooks_dir(Path::new(here)).filter(|dir| hook_launcher::opted_in(Path::new(here), dir)))
}

// spec: gate-sdk/SPEC.md §install-hooks — the files re-placed and nothing else: no config write, no
// identity rung, no receipt
fn refresh() -> Result<i32, String> {
    let Some(hooks_dir) = refresh_target(&walk::cwd()?)? else {
        return Ok(0);
    };
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")?;
    if !Path::new(&bin).is_file() {
        return Err(absent(&bin));
    }
    place(Path::new(&bin), &hooks_dir)?;
    println!("install-hooks: refreshed {}", hooks_dir.display());
    Ok(0)
}

fn dispatch(args: &[String]) -> Result<i32, String> {
    if args.first().map(String::as_str) == Some(REFRESH) {
        return match args.get(1) {
            Some(a) => Err(format!("{} takes no arguments (got: {})\n{}", REFRESH, a, USAGE)),
            None => refresh(),
        };
    }
    let surplus = super::file_survey::positionals(args, "argument")
        .map_err(|e| format!("{}\n{}", e, USAGE))?;
    if let Some(a) = surplus.first() {
        return Err(format!("takes no arguments (got: {})\n{}", a, USAGE));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — both knobs and the hooks directory are resolved
    // before any write, so a configuration failure refuses without half-installing.
    let bin = walk::knob_scalar("GATE_SDK_NATIVE_BIN")?;
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    if !Path::new(&bin).is_file() {
        return Err(absent(&bin));
    }
    let hooks_dir = hook_launcher::hooks_dir(Path::new(&walk::cwd()?))
        .ok_or_else(|| "not inside a git repository — there is no clone to place the hooks in".to_string())?;

    place(Path::new(&bin), &hooks_dir)?;
    // spec: gate-sdk/SPEC.md §install-hooks — an unwired clone is no opt-in: the refusal comes
    // ahead of the receipt, which would read as one
    config("core.hooksPath", &hooks_dir.display().to_string()).map_err(|e| {
        format!(
            "{} — the hooks are placed in {} and git is not pointed at them, so no hook runs; clear the cause and re-run",
            e,
            hooks_dir.display()
        )
    })?;
    // spec: gate-sdk/SPEC.md §install-hooks — the blame guard stays a file-existence test, so a
    // consumer without the file gets the same one-line output it always got; its failed write is
    // reported and the status unmoved, since no hook depends on it.
    if Path::new(".git-blame-ignore-revs").is_file() {
        if let Err(e) = config("blame.ignoreRevsFile", ".git-blame-ignore-revs") {
            eprintln!("install-hooks: {}", e);
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — the receipt prints whatever the rung returned: a
    // failed verification still tells the session which hooks the wiring enabled, and the status
    // is what carries the finding.
    let identity_rc = identity_rung(&gates_dir);

    println!("Active hooks:");
    for (name, _) in walk::list_dir(&hooks_dir).unwrap_or_default() {
        println!("  {}", name);
    }
    println!();
    println!("Disable with:  git config --unset core.hooksPath");
    Ok(identity_rc)
}

// spec: gate-sdk/SPEC.md §install-hooks — each served hook is a hard link to the binary under the
// hook's name, a copy where the link cannot be made. The replacement is written under a sibling
// name and renamed over the hook, so a placement that fails leaves the hook already there in place
fn place(bin: &Path, dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    for hook in hook_launcher::SERVED {
        let dst = dir.join(hook_launcher::file_name(hook));
        let staged = dir.join(staged_name(hook));
        let placed = stage(bin, &staged).and_then(|_| {
            std::fs::rename(&staged, &dst).map_err(|e| format!("cannot replace {}: {}", dst.display(), e))
        });
        // spec: gate-sdk/SPEC.md §install-hooks — a rename between two links to one file succeeds
        // and moves nothing, so the sibling is removed on every path
        let _ = std::fs::remove_file(&staged);
        placed?;
    }
    Ok(())
}

const STAGED_SUFFIX: &str = ".new";

// spec: gate-sdk/SPEC.md §install-hooks — the sibling's name carries the placing process's id, so
// two placements at once never remove or rename each other's
fn staged_name(hook: &str) -> String {
    format!("{}{}.{}", hook_launcher::file_name(hook), STAGED_SUFFIX, std::process::id())
}

fn stage(bin: &Path, staged: &Path) -> Result<(), String> {
    let _ = std::fs::remove_file(staged);
    if std::fs::hard_link(bin, staged).is_err() {
        copy_new(bin, staged)
            .map_err(|e| format!("cannot place {} from {}: {}", staged.display(), bin.display(), e))?;
    }
    crate::install::make_executable(staged)
}

// spec: gate-sdk/SPEC.md §install-hooks — the copy creates its file and never opens a name already
// there: a sibling that survived its removal may be a link to the file the placed hooks serve,
// and a write through it would replace or truncate them
fn copy_new(bin: &Path, staged: &Path) -> std::io::Result<()> {
    let mut src = std::fs::File::open(bin)?;
    let mut dst = std::fs::OpenOptions::new().write(true).create_new(true).open(staged)?;
    std::io::copy(&mut src, &mut dst).map(|_| ())
}

fn config(key: &str, value: &str) -> Result<(), String> {
    let c = proc::run(&programs::GIT, &["config", key, value])?;
    if c.stdout().is_none() {
        return Err(format!("could not set {} ({})", key, c.failure_report().unwrap_or_default()));
    }
    println!("Installed: {} = {}", key, value);
    Ok(())
}

// spec: gate-sdk/SPEC.md §install-hooks — the apply-and-verify rung. The gate is resolved through
// the registry so a consumer shadow wins; an in-process call by name would resolve the *crate's*
// member and silently stop honouring it, which narrows a consumer-facing seam.
fn identity_rung(gates_dir: &str) -> i32 {
    let roots = match walk::kit_roots_abs() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("install-hooks: {}", e);
            return 2;
        }
    };
    let dirs = registry::resolve_dirs(gates_dir, &roots);
    // spec: gate-sdk/SPEC.md §install-hooks — resolving nowhere is a silent skip at the wiring's
    // own status, which a consumer shipping no such gate is entitled to; anything else that
    // cannot be dispatched fails the opt-in rather than being waved through.
    let Some(src) = registry::resolve(IDENTITY, &dirs) else {
        return 0;
    };
    println!();
    println!("Verifying git identity ({})…", IDENTITY);
    if src.ends_with(".gate") {
        // spec: gate-sdk/SPEC.md §run-gates — the descriptor branch re-execs this binary as a
        // child rather than calling the member in process, on that section's fault-isolation ground
        if crate::gates::declared(IDENTITY).is_none() {
            eprintln!(
                "install-hooks: {} declares a descriptor at {} but this binary carries no such subcommand — the gate could not run; treating as failure (not clean)",
                IDENTITY, src
            );
            return 2;
        }
        let exe = match std::env::current_exe() {
            Ok(p) => p.display().to_string(),
            Err(e) => {
                eprintln!("install-hooks: cannot resolve this binary's own path: {}", e);
                return 2;
            }
        };
        match proc::run_to(&programs::CHECKWRIGHT_GATES.at(exe), &[IDENTITY], &proc::Sink::Inherit) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("install-hooks: {}", e);
                2
            }
        }
    } else {
        // spec: gate-sdk/SPEC.md §install-hooks — a consumer `.sh` shadow is that consumer's rule
        // and this arm is not entitled to substitute its own, so it is spawned as the shell rung
        // spawned it, with its two streams in the caller's terminal and its status propagated.
        let shadow = programs::Program::consumer(programs::GATE_DECLARATION, src);
        match proc::run_to(&shadow, &[], &proc::Sink::Inherit) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("install-hooks: {}", e);
                2
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the option half refuses before either knob
    // resolves, so a stray flag wires nothing
    #[test]
    fn a_flag_refuses_with_the_usage_before_any_wiring() {
        let err = dispatch(&["--help".to_string()]).expect_err("--help must refuse");
        assert!(err.contains("--help") && err.contains(USAGE), "{}", err);
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — a surplus positional, bare or after `--`,
    // refuses before any wiring rather than reading as a successful opt-in
    #[test]
    fn a_bare_positional_refuses_with_the_usage_before_any_wiring() {
        for argv in [vec!["foo"], vec!["--", "foo"]] {
            let args: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
            let err = dispatch(&args).expect_err("a positional must refuse");
            assert!(err.contains("got: foo") && err.contains(USAGE), "{}", err);
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — `--refresh` is the arm's one option and takes no
    // operand, so a token after it refuses before anything is placed
    #[test]
    fn refresh_refuses_a_trailing_token_with_the_usage() {
        for extra in ["foo", "--force"] {
            let err = dispatch(&[REFRESH.to_string(), extra.to_string()]).expect_err("an operand must refuse");
            assert!(err.contains(&format!("got: {}", extra)) && err.contains(USAGE), "{}", err);
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a refresh has a target only at the work-tree top of a
    // clone whose `core.hooksPath` names the hooks directory, so its caller never opts a clone in
    #[test]
    fn refresh_targets_only_the_top_of_a_clone_already_opted_in() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-refresh.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("repo/sub")).expect("scratch");
        let top = walk::normalize_abs(&d.join("repo").display().to_string());
        let sub = format!("{}/sub", top);
        proc::run(&programs::GIT, &["-C", &top, "init", "-q"]).expect("git init");
        let unopted = refresh_target(&top);
        let hooks = hook_launcher::hooks_dir(Path::new(&top)).expect("a hooks directory");
        let elsewhere = d.join("elsewhere").display().to_string();
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &elsewhere]).expect("git config");
        let other_path = refresh_target(&top);
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &hooks.display().to_string()]).expect("git config");
        let opted = refresh_target(&top);
        let below = refresh_target(&sub);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(unopted, Ok(None));
        assert_eq!(other_path, Ok(None));
        assert_eq!(opted, Ok(Some(hooks)));
        assert_eq!(below, Ok(None));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — both served hooks are placed whatever the registry
    // holds, each the binary's bytes, and a re-run replaces a file already there
    #[test]
    fn both_hooks_are_placed_and_a_rerun_replaces_them() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        place(&bin, &hooks).expect("first placement");
        let names: Vec<String> = walk::list_dir(&hooks).expect("lists").into_iter().map(|(n, _)| n).collect();
        std::fs::remove_file(&bin).expect("the update's unlink");
        std::fs::write(&bin, "two").expect("the updated binary");
        let stale = std::fs::read_to_string(hooks.join(hook_launcher::file_name("pre-commit"))).expect("reads");
        place(&bin, &hooks).expect("second placement");
        let fresh: Vec<String> = hook_launcher::SERVED
            .iter()
            .map(|h| std::fs::read_to_string(hooks.join(hook_launcher::file_name(h))).expect("reads"))
            .collect();
        let executable = crate::proc::is_executable(&hooks.join(hook_launcher::file_name("commit-msg")));
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(names, vec![hook_launcher::file_name("commit-msg"), hook_launcher::file_name("pre-commit")]);
        assert_eq!(stale, "one", "a replaced binary must leave the placed hook as it was");
        assert_eq!(fresh, vec!["two".to_string(), "two".to_string()]);
        assert!(executable);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a replacement that cannot be written leaves both
    // hooks as they were and no sibling behind, and so does a re-run over an unchanged binary
    #[test]
    fn a_replacement_that_cannot_be_written_leaves_the_placed_hooks() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-kept.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        place(&bin, &hooks).expect("first placement");
        place(&bin, &hooks).expect("a re-run over the same file");
        let listed = |dir: &Path| -> Vec<String> { walk::list_dir(dir).expect("lists").into_iter().map(|(n, _)| n).collect() };
        let rerun = listed(&hooks);
        let refused = place(&d.join("no-such-binary"), &hooks);
        let after = listed(&hooks);
        let kept: Vec<String> = hook_launcher::SERVED
            .iter()
            .map(|h| std::fs::read_to_string(hooks.join(hook_launcher::file_name(h))).expect("the hook is still there"))
            .collect();
        let _ = std::fs::remove_dir_all(&d);
        let both = vec![hook_launcher::file_name("commit-msg"), hook_launcher::file_name("pre-commit")];
        assert_eq!(rerun, both, "a re-run left a sibling behind");
        assert!(refused.is_err_and(|e| e.contains("cannot place")), "an absent source must refuse");
        assert_eq!(after, both, "a refused placement left a sibling behind or removed a hook");
        assert_eq!(kept, vec!["one".to_string(), "one".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a sibling that survived under this run's name and
    // links to the file the placed hooks serve is never written through by the copy, and a
    // placement that can remove it replaces both hooks
    #[test]
    fn a_surviving_sibling_is_never_written_through() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-survivor.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        place(&bin, &hooks).expect("first placement");
        std::fs::remove_file(&bin).expect("the update's unlink");
        std::fs::write(&bin, "two").expect("the updated binary");
        let served = hooks.join(hook_launcher::file_name("pre-commit"));
        let survivor = hooks.join(staged_name("pre-commit"));
        std::fs::hard_link(&served, &survivor).expect("a killed run's sibling");
        let copied = copy_new(&bin, &survivor);
        let through = std::fs::read_to_string(&served).expect("reads");
        let replaced = place(&bin, &hooks);
        let fresh = std::fs::read_to_string(&served).expect("reads");
        let left = survivor.exists();
        let _ = std::fs::remove_dir_all(&d);
        assert!(copied.is_err_and(|e| e.kind() == std::io::ErrorKind::AlreadyExists), "the copy opened a name already there");
        assert_eq!(through, "one", "the copy wrote through the sibling into the placed hook");
        assert_eq!(replaced, Ok(()));
        assert_eq!(fresh, "two");
        assert!(!left, "the placement left its sibling behind");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — another run's sibling is neither removed nor renamed
    #[test]
    fn another_runs_sibling_is_left_alone() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-foreign.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("git/gate-hooks")).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        let foreign: Vec<PathBuf> = hook_launcher::SERVED
            .iter()
            .map(|h| hooks.join(format!("{}{}.0", hook_launcher::file_name(h), STAGED_SUFFIX)))
            .collect();
        for f in &foreign {
            std::fs::write(f, "theirs").expect("another run's sibling");
        }
        let placed = place(&bin, &hooks);
        let kept: Vec<String> = foreign.iter().map(|f| std::fs::read_to_string(f).unwrap_or_default()).collect();
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(placed, Ok(()));
        assert_eq!(kept, vec!["theirs".to_string(), "theirs".to_string()]);
    }
}
