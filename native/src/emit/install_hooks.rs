// spec: gate-sdk/SPEC.md §install-hooks — the per-clone hook opt-in: wire core.hooksPath at this
// clone, verify the git identity once, and report what the wiring enabled.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the contract is three-state and
// every code is load-bearing: `check-identity`'s 1 propagates through, and an emitting arm cannot
// carry it.
use crate::proc;
use crate::registry;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §install-hooks — this arm's own three names, then the five its
// registry-resolved callee declares: a dispatching arm must itself be handed what it passes down,
// and the unit test below holds this tail against `check-identity`'s own entry so it cannot rot.
pub const KNOBS: &[&str] = &[
    "GATE_SDK_HOOKS_DIR",
    "GATE_SDK_GATES_DIR",
    "GATE_KIT_ROOTS_HERE",
    "GATE_SDK_IDENTITY_FILE",
    "GATE_SDK_GIT_EMAIL_FILE",
    "GATE_SDK_GIT_REMOTES_FILE",
    "GATE_SDK_GH_HOSTS_FILE",
    "GATE_SDK_GH_HOST",
];

const IDENTITY: &str = "check-identity";

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the member takes no positional, so the shape
// refusal has no free text to bind on and only the option half binds; usage itself lives on this
// arm's own front-end `case` arm, which the class gives every member holding one.
const USAGE: &str = "usage: run-gates.sh --install-hooks
  Takes no argument: the whole input is the bridged GATE_SDK_* environment.";

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
    super::file_survey::positionals(args, "argument").map_err(|e| format!("{}\n{}", e, USAGE))?;

    // spec: gate-sdk/SPEC.md §install-hooks — both knobs are resolved before any wiring, the order
    // the shell form read them in, so a bridge failure refuses without half-installing.
    let hooks_dir = walk::knob_scalar("GATE_SDK_HOOKS_DIR")?;
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    if !Path::new(&hooks_dir).is_dir() {
        return Err(format!(
            "no hooks dir at {} — generate the pre-commit hook first:\n  bash gate-sdk/bin/gen-pre-commit.sh --write",
            hooks_dir
        ));
    }

    chmod_hooks(&hooks_dir);
    config("core.hooksPath", &hooks_dir);
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
    for (name, _) in listing(&hooks_dir) {
        println!("  {}", name);
    }
    println!();
    println!("Disable with:  git config --unset core.hooksPath");
    Ok(identity_rc)
}

// spec: gate-sdk/SPEC.md §check-hook-exec-bit — a per-clone convenience rather than an assertion:
// that gate's subject is the *committed* mode, which this cannot repair, so one entry failing to
// chmod does not fail the opt-in.
fn chmod_hooks(dir: &str) {
    for (name, _) in listing(dir) {
        let _ = crate::install::make_executable(&Path::new(dir).join(&name));
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — the per-clone git-config writes degrade soft outside a
// repository, the shape the sibling per-clone installer already rules for its own driver step,
// rather than crashing the opt-in.
fn config(key: &str, value: &str) {
    match proc::run("git", &["config", key, value]) {
        Ok(c) if c.stdout().is_some() => println!("Installed: {} = {}", key, value),
        Ok(c) => eprintln!(
            "install-hooks: could not set {} ({})",
            key,
            c.failure_report().unwrap_or_default()
        ),
        Err(e) => eprintln!("install-hooks: {}", e),
    }
}

// spec: gate-sdk/SPEC.md §install-hooks — `ls -1 | sed 's/^/  /'`, the opt-in's receipt: the only
// place a session sees which hooks the wiring just enabled, so its shape is preserved and its
// order is `list_dir`'s sort, which is the order `ls` printed.
fn listing(dir: &str) -> Vec<(String, bool)> {
    walk::list_dir(Path::new(dir)).unwrap_or_default()
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
        // child rather than calling the member in process: an in-process call cannot hand the
        // callee its own declared knob set, which is the discipline that refusal exists for.
        let Some(declared) = crate::gates::knobs(IDENTITY) else {
            eprintln!(
                "install-hooks: {} declares a descriptor at {} but this binary carries no such subcommand — the gate could not run; treating as failure (not clean)",
                IDENTITY, src
            );
            return 2;
        };
        let exe = match std::env::current_exe() {
            Ok(p) => p.display().to_string(),
            Err(e) => {
                eprintln!("install-hooks: cannot resolve this binary's own path: {}", e);
                return 2;
            }
        };
        let env = super::child_knobs(declared);
        match proc::run_to_env(&exe, &[IDENTITY], &env, &proc::Sink::Inherit) {
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
        match proc::run_to(&src, &[], &proc::Sink::Inherit) {
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

    // spec: gate-sdk/SPEC.md §install-hooks — the declared-knob discipline reaches a dispatching
    // arm as a containment: a child is handed what this arm itself received, so a callee knob
    // absent from this roster arrives unset and the opt-in fails on a gate that was fine.
    #[test]
    fn the_arm_declares_every_knob_its_callee_declares() {
        let callee = crate::gates::knobs(IDENTITY).expect("check-identity must be registered");
        let missing: Vec<&str> = callee
            .iter()
            .copied()
            .filter(|k| !KNOBS.contains(k))
            .collect();
        assert!(
            missing.is_empty(),
            "{} declares {:?}, which --install-hooks does not: a dispatching arm is handed only \
             its own declared set, so the child would receive these unset",
            IDENTITY,
            missing
        );
    }
}
