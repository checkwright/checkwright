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
    "GATE_SDK_POWERSHELL",
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
    let here = walk::cwd()?;
    let hooks_dir = hook_launcher::hooks_dir(Path::new(&here))
        .ok_or_else(|| "not inside a git repository — there is no clone to place the hooks in".to_string())?;
    let replaced = match prior(&here, &hooks_dir)? {
        Prior::Live { dir, names } => return Err(live_refusal(&dir, &names, &hooks_dir, &bin)),
        Prior::Own | Prior::Quiet { set: None } => None,
        Prior::Quiet { set: Some(value) } => Some(value),
    };

    place(Path::new(&bin), &hooks_dir)?;
    if let Some(value) = replaced {
        println!(
            "install-hooks: core.hooksPath was {}, which holds no hook git runs; this clone now runs {}",
            value,
            hooks_dir.display()
        );
    }
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

// spec: gate-sdk/SPEC.md §install-hooks — the hooks directory git runs before this arm repoints it
#[derive(Debug, PartialEq)]
enum Prior {
    Own,
    Quiet { set: Option<String> },
    Live { dir: PathBuf, names: Vec<String> },
}

fn prior(here: &str, hooks_dir: &Path) -> Result<Prior, String> {
    let top = walk::toplevel_in_opt(here)?.unwrap_or_else(|| here.to_string());
    let got = proc::run(&programs::GIT, &["-C", &top, "config", "--path", "--get", "core.hooksPath"])?;
    let set = match (got.stdout(), got.code()) {
        (Some(out), _) => Some(String::from_utf8_lossy(out).trim_end_matches(['\r', '\n']).to_string()).filter(|v| !v.is_empty()),
        (None, Some(1)) => None,
        (None, _) => return Err(format!("cannot read core.hooksPath ({})", got.failure_report().unwrap_or_default())),
    };
    let dir = match &set {
        Some(value) => PathBuf::from(walk::abs_against(&top, value)),
        None => {
            let out = proc::run(&programs::GIT, &["-C", &top, "rev-parse", "--git-path", "hooks"])?;
            let path = out.stdout().ok_or_else(|| {
                format!("cannot resolve git's hooks directory ({})", out.failure_report().unwrap_or_default())
            })?;
            PathBuf::from(walk::abs_against(&top, String::from_utf8_lossy(path).trim()))
        }
    };
    if hook_launcher::same_dir(&dir, hooks_dir) {
        return Ok(Prior::Own);
    }
    let names = live_hooks(&dir)?;
    if names.is_empty() {
        Ok(Prior::Quiet { set })
    } else {
        Ok(Prior::Live { dir, names })
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — a live hook is a file git would run: a hook's name, a
// symlink followed, and the executable bit where the host has one
fn live_hooks(dir: &Path) -> Result<Vec<String>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = walk::list_dir(dir)?
        .into_iter()
        .map(|(name, _)| name)
        .filter(|name| hook_launcher::git_hook_named(name) && proc::is_executable(&dir.join(name)))
        .collect();
    names.sort();
    Ok(names)
}

fn live_refusal(dir: &Path, names: &[String], hooks_dir: &Path, bin: &str) -> String {
    format!(
        "this clone runs hooks from {} ({}), and pointing core.hooksPath at {} would stop them running — nothing placed\n  help: call the gates from your hook manager — '{} --git-hook pre-commit' from its pre-commit and '{} --git-hook commit-msg <message-file>' from its commit-msg — or move those hooks aside or unset core.hooksPath, then re-run --install-hooks.",
        dir.display(),
        names.join(", "),
        hooks_dir.display(),
        bin,
        bin
    )
}

// spec: gate-sdk/SPEC.md §install-hooks — each served hook is a hard link to the binary under the
// hook's name, a copy where the link cannot be made. The replacement is written under a sibling
// name and renamed over the hook, so a placement that fails leaves the hook already there in place
fn place(bin: &Path, dir: &Path) -> Result<(), String> {
    place_as(bin, dir, &run_tag())
}

fn place_as(bin: &Path, dir: &Path, tag: &str) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    for hook in hook_launcher::SERVED {
        let dst = dir.join(hook_launcher::file_name(hook));
        let staged = dir.join(staged_name(hook, tag));
        stage(bin, &staged)?;
        let renamed =
            std::fs::rename(&staged, &dst).map_err(|e| format!("cannot replace {}: {}", dst.display(), e));
        // spec: gate-sdk/SPEC.md §install-hooks — a rename between two links to one file succeeds
        // and moves nothing, so the sibling is removed on every path; a removal that fails is
        // reported, and moves the status only where the rename already had
        match (renamed, remove_own(&staged)) {
            (Ok(()), None) => {}
            (Ok(()), Some(left)) => eprintln!("install-hooks: {}", left),
            (Err(e), None) => return Err(e),
            (Err(e), Some(left)) => return Err(format!("{}; {}", e, left)),
        }
    }
    Ok(())
}

const STAGED_SUFFIX: &str = ".new";

// spec: gate-sdk/SPEC.md §install-hooks — the sibling's name carries the placing process's id and
// a value drawn at random per run, so a placement in another process-id namespace draws another
fn run_tag() -> String {
    use std::hash::{BuildHasher, Hasher};
    let drawn = std::collections::hash_map::RandomState::new().build_hasher().finish();
    format!("{}.{:016x}", std::process::id(), drawn)
}

fn staged_name(hook: &str, tag: &str) -> String {
    format!("{}{}.{}", hook_launcher::file_name(hook), STAGED_SUFFIX, tag)
}

// spec: gate-sdk/SPEC.md §install-hooks — the sibling is created, never opened or removed first: a
// name already there is another placement's, and the link and the copy both refuse it. A stage
// that fails leaves nothing of its own behind
fn stage(bin: &Path, staged: &Path) -> Result<(), String> {
    if std::fs::hard_link(bin, staged).is_err() {
        copy_new(bin, staged)
            .map_err(|e| format!("cannot place {} from {}: {}", staged.display(), bin.display(), e))?;
    }
    crate::install::make_executable(staged).map_err(|e| match remove_own(staged) {
        Some(left) => format!("{}; {}", e, left),
        None => e,
    })
}

fn remove_own(staged: &Path) -> Option<String> {
    match std::fs::remove_file(staged) {
        Ok(()) => None,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => Some(format!(
            "cannot remove {}: {} — it serves no hook; delete it by hand",
            staged.display(),
            e
        )),
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — the copy creates its file and never opens a name already
// there: a sibling there may be a link to the file the placed hooks serve, and a write through it
// would replace or truncate them. A copy cut short removes its file or names it in the refusal
fn copy_new(bin: &Path, staged: &Path) -> std::io::Result<()> {
    copy_from(&mut std::fs::File::open(bin)?, staged)
}

fn copy_from(src: &mut impl std::io::Read, staged: &Path) -> std::io::Result<()> {
    let mut dst = std::fs::OpenOptions::new().write(true).create_new(true).open(staged)?;
    let copied = std::io::copy(src, &mut dst).map(|_| ());
    drop(dst);
    copied.map_err(|e| match remove_own(staged) {
        Some(left) => std::io::Error::new(e.kind(), format!("{}; {}", e, left)),
        None => e,
    })
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
    // spec: gate-sdk/SPEC.md §run-gates — a descriptor re-execs this binary as a child rather than
    // calling the member in process, on that section's fault-isolation ground
    // spec: gate-sdk/SPEC.md §install-hooks — a consumer shadow is that consumer's rule and this arm
    // is not entitled to substitute its own, so it is started as the battery starts it, with its two
    // streams in the caller's terminal and its status propagated.
    let exe = match crate::proc::self_image() {
        Ok(p) => p.display().to_string(),
        Err(e) => {
            eprintln!("install-hooks: cannot resolve this binary's own path: {}", e);
            return 2;
        }
    };
    let inv = match registry::invocation(IDENTITY, &src, &exe) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("install-hooks: {} at {} — {}", IDENTITY, src, e);
            return 2;
        }
    };
    let args: Vec<&str> = inv.args.iter().map(String::as_str).collect();
    let env: Vec<(String, String)> = match inv.substrate {
        registry::Substrate::Compiled => Vec::new(),
        _ => vec![("GATE_SDK_NATIVE_BIN".to_string(), exe.clone())],
    };
    match proc::run_to_env(&inv.program, &args, &env, &proc::Sink::Inherit) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("install-hooks: {}", e);
            2
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

    fn scratch_repo(tag: &str) -> (PathBuf, String, PathBuf) {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-prior-{}.{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("repo")).expect("scratch");
        let top = walk::normalize_abs(&d.join("repo").display().to_string());
        proc::run(&programs::GIT, &["-C", &top, "init", "-q"]).expect("git init");
        let hooks = hook_launcher::hooks_dir(Path::new(&top)).expect("a hooks directory");
        (d, top, hooks)
    }

    fn plant(dir: &Path, name: &str, executable: bool) {
        std::fs::create_dir_all(dir).expect("hooks dir");
        let file = dir.join(name);
        std::fs::write(&file, "#!/bin/sh\nexit 0\n").expect("hook");
        if executable {
            crate::install::make_executable(&file).expect("chmod");
        }
    }

    fn hooks_path(top: &str) -> Option<String> {
        let out = proc::run(&programs::GIT, &["-C", top, "config", "--get", "core.hooksPath"]).expect("git config");
        out.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string())
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a live pre-commit in git's default directory refuses
    // and wires nothing, where a directory of samples alone proceeds
    #[test]
    fn a_live_hook_in_the_default_directory_refuses_and_samples_proceed() {
        let (d, top, hooks) = scratch_repo("default");
        let default = PathBuf::from(format!("{}/.git/hooks", top));
        let samples = prior(&top, &hooks);
        plant(&default, "pre-commit", true);
        let live = prior(&top, &hooks);
        let unset = hooks_path(&top);
        let placed = hooks.exists();
        let same_while_present = matches!(&live, Ok(Prior::Live { dir, .. }) if hook_launcher::same_dir(dir, &default));
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(samples, Ok(Prior::Quiet { set: None }), "git's template hooks are samples");
        match live {
            Ok(Prior::Live { dir, names }) => {
                assert!(same_while_present, "{} is not {}", dir.display(), default.display());
                assert_eq!(names, vec!["pre-commit".to_string()]);
                let said = live_refusal(&dir, &names, &hooks, "bin/gates");
                assert!(said.contains("(pre-commit)") && said.contains("nothing placed"), "{}", said);
                assert!(said.contains("'bin/gates --git-hook pre-commit'"), "{}", said);
                assert!(said.contains("\n  help: "), "{}", said);
            }
            other => panic!("a live pre-commit must refuse: {:?}", other),
        }
        assert_eq!(unset, None);
        assert!(!placed, "the refusal placed the hooks directory");
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a core.hooksPath naming a directory with a live hook
    // refuses with the value unchanged, and one holding no hook is replaced with a note
    #[test]
    fn a_hooks_path_with_a_live_hook_refuses_and_an_empty_one_is_replaced() {
        let (d, top, hooks) = scratch_repo("set");
        let theirs = d.join("husky").display().to_string();
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &theirs]).expect("git config");
        let quiet = prior(&top, &hooks);
        plant(Path::new(&theirs), "pre-push", true);
        let live = prior(&top, &hooks);
        let kept = hooks_path(&top);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(quiet, Ok(Prior::Quiet { set: Some(theirs.clone()) }));
        assert!(matches!(&live, Ok(Prior::Live { names, .. }) if names == &vec!["pre-push".to_string()]), "{:?}", live);
        assert_eq!(kept, Some(theirs));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — on a host with an executable bit, a hook without it is
    // one git skips, so it proceeds
    #[cfg(unix)]
    #[test]
    fn a_hook_without_the_executable_bit_proceeds() {
        let (d, top, hooks) = scratch_repo("bit");
        plant(&PathBuf::from(format!("{}/.git/hooks", top)), "pre-commit", false);
        let got = prior(&top, &hooks);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(got, Ok(Prior::Quiet { set: None }));
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a core.hooksPath already naming the hooks directory is
    // a re-run, whatever the default directory holds
    #[test]
    fn a_rerun_proceeds_past_a_live_hook_in_the_default_directory() {
        let (d, top, hooks) = scratch_repo("rerun");
        plant(&PathBuf::from(format!("{}/.git/hooks", top)), "pre-commit", true);
        proc::run(&programs::GIT, &["-C", &top, "config", "core.hooksPath", &hooks.display().to_string()]).expect("git config");
        let got = prior(&top, &hooks);
        let _ = std::fs::remove_dir_all(&d);
        assert_eq!(got, Ok(Prior::Own));
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

    // spec: gate-sdk/SPEC.md §install-hooks — two placements forced onto one sibling name: the
    // second refuses, writes nothing through the first's sibling and leaves it there, and a
    // placement under a name of its own replaces both hooks
    #[test]
    fn a_placement_drawing_a_name_already_there_refuses_and_leaves_it() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-survivor.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (bin, hooks) = (d.join("bin"), d.join("git/gate-hooks"));
        std::fs::write(&bin, "one").expect("binary");
        place(&bin, &hooks).expect("first placement");
        std::fs::remove_file(&bin).expect("the update's unlink");
        std::fs::write(&bin, "two").expect("the updated binary");
        let served = hooks.join(hook_launcher::file_name("pre-commit"));
        let theirs = hooks.join(staged_name("pre-commit", "7.drawn"));
        std::fs::hard_link(&served, &theirs).expect("another placement's sibling");
        let copied = copy_new(&bin, &theirs);
        let second = place_as(&bin, &hooks, "7.drawn");
        let through = std::fs::read_to_string(&served).expect("reads");
        let kept = std::fs::read_to_string(&theirs).unwrap_or_default();
        let replaced = place(&bin, &hooks);
        let fresh = std::fs::read_to_string(&served).expect("reads");
        let left: Vec<String> = walk::list_dir(&hooks).expect("lists").into_iter().map(|(n, _)| n).collect();
        let _ = std::fs::remove_dir_all(&d);
        assert!(copied.is_err_and(|e| e.kind() == std::io::ErrorKind::AlreadyExists), "the copy opened a name already there");
        assert!(second.is_err_and(|e| e.contains("cannot place")), "a second placement on one name must refuse");
        assert_eq!(through, "one", "the refused placement wrote through the sibling into the placed hook");
        assert_eq!(kept, "one", "the refused placement removed or replaced the other's sibling");
        assert_eq!(replaced, Ok(()));
        assert_eq!(fresh, "two");
        assert_eq!(
            left,
            vec![
                hook_launcher::file_name("commit-msg"),
                hook_launcher::file_name("pre-commit"),
                staged_name("pre-commit", "7.drawn")
            ],
            "a placement left its own sibling behind or removed another's"
        );
    }

    // spec: gate-sdk/SPEC.md §install-hooks — two runs of one process draw two names, and a
    // sibling that cannot be removed is reported with its path where an absent one is not
    #[test]
    fn a_run_draws_its_own_name_and_a_failed_removal_is_reported() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-removal.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("held")).expect("scratch");
        let (first, second) = (run_tag(), run_tag());
        let held = remove_own(&d.join("held"));
        let absent = remove_own(&d.join("absent"));
        let _ = std::fs::remove_dir_all(&d);
        assert_ne!(first, second, "two runs drew one sibling name");
        assert!(first.starts_with(&format!("{}.", std::process::id())), "{}", first);
        assert!(held.is_some_and(|r| r.contains("cannot remove") && r.contains("held")), "a failed removal went unreported");
        assert_eq!(absent, None);
    }

    struct CutShort<F: FnMut()>(F);

    impl<F: FnMut()> std::io::Read for CutShort<F> {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            (self.0)();
            Err(std::io::Error::other("the source went away"))
        }
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a copy cut short removes the file it created, and a
    // sibling it cannot remove is named in the refusal
    #[test]
    fn a_copy_cut_short_removes_its_file_or_names_it() {
        let d = std::env::temp_dir().join(format!("checkwright-install-hooks-cut.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("scratch");
        let (gone, held) = (d.join("gone"), d.join("held"));
        let removed = copy_from(&mut CutShort(|| {}), &gone);
        let left = gone.exists();
        let swap = || {
            if cfg!(unix) {
                std::fs::remove_file(&held).expect("unlinks the open sibling");
                std::fs::create_dir(&held).expect("a name the removal cannot take");
            }
        };
        let kept = copy_from(&mut CutShort(swap), &held);
        let _ = std::fs::remove_dir_all(&d);
        assert!(removed.is_err_and(|e| !e.to_string().contains("cannot remove")), "a removal that worked was reported");
        assert!(!left, "a copy cut short left its file behind");
        let named = kept.is_err_and(|e| {
            let said = e.to_string();
            said.contains("the source went away") && said.contains("cannot remove") && said.contains("held")
        });
        assert_eq!(named, cfg!(unix), "a failed removal must join the refusal");
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
