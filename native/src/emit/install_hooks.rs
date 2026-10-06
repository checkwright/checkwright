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
use std::path::Path;

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

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the member takes no argument, so any token is
// a refusal before either knob resolves; usage itself lives on this arm's own front-end `case` arm,
// which the class gives every member holding one.
const USAGE: &str = "usage: --install-hooks
  Takes no argument: the whole input is the GATE_SDK_* configuration.";

pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("install-hooks: {}", e);
            2
        }
    }
}

fn dispatch(args: &[String]) -> Result<i32, String> {
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
        return Err(format!(
            "no gate binary at {} — place or build the binary GATE_SDK_NATIVE_BIN names first",
            bin
        ));
    }
    let hooks_dir = hook_launcher::hooks_dir(Path::new(&walk::cwd()?))
        .ok_or_else(|| "not inside a git repository — there is no clone to place the hooks in".to_string())?;

    place(Path::new(&bin), &hooks_dir)?;
    config("core.hooksPath", &hooks_dir.display().to_string());
    // spec: gate-sdk/SPEC.md §install-hooks — the blame guard stays a file-existence test, so a
    // consumer without the file gets the same one-line output it always got.
    if Path::new(".git-blame-ignore-revs").is_file() {
        config("blame.ignoreRevsFile", ".git-blame-ignore-revs");
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
// hook's name, a copy where the link cannot be made, replaced on a re-run
fn place(bin: &Path, dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
    for hook in hook_launcher::SERVED {
        let dst = dir.join(hook_launcher::file_name(hook));
        if dst.symlink_metadata().is_ok() {
            std::fs::remove_file(&dst).map_err(|e| format!("cannot replace {}: {}", dst.display(), e))?;
        }
        if std::fs::hard_link(bin, &dst).is_err() {
            std::fs::copy(bin, &dst)
                .map_err(|e| format!("cannot place {} from {}: {}", dst.display(), bin.display(), e))?;
        }
        crate::install::make_executable(&dst)?;
    }
    Ok(())
}

// spec: gate-sdk/SPEC.md §install-hooks — a per-clone git-config write that fails is reported,
// the shape the sibling per-clone installer rules for its own driver step, never a crash.
fn config(key: &str, value: &str) {
    match proc::run(&programs::GIT, &["config", key, value]) {
        Ok(c) if c.stdout().is_some() => println!("Installed: {} = {}", key, value),
        Ok(c) => eprintln!(
            "install-hooks: could not set {} ({})",
            key,
            c.failure_report().unwrap_or_default()
        ),
        Err(e) => eprintln!("install-hooks: {}", e),
    }
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
}
