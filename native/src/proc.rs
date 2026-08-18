// spec: gate-sdk/SPEC.md §Fail-closed contract — the crate's one spawn site, so the
// captured-emptiness false-green has no spelling in a gate module: a failed spawn is the
// `Err` arm and stdout is reachable only through an accessor that read the exit status
use std::process::Command;

// spec: gate-sdk/SPEC.md §Fail-closed contract — a child that ran. Constructing one is the
// proof the spawn succeeded, which is the half `Command::output()`'s `Ok` actually carries.
pub struct Completed {
    status: std::process::ExitStatus,
    stdout: Vec<u8>,
}

impl Completed {
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the only path to stdout, and it reads
    // the status rather than sitting beside it, so a caller cannot reach a crashed child's
    // empty capture and branch on its emptiness
    pub fn stdout(&self) -> Option<&[u8]> {
        if self.status.success() {
            Some(&self.stdout)
        } else {
            None
        }
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the exit code for a caller whose child
    // grades its own outcome by it: `git grep` says 1 for no-match and ≥2 for an error, and
    // folding both into `None` makes an unreadable corpus read as a clean one
    pub fn code(&self) -> Option<i32> {
        self.status.code()
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — `Err` is a *spawn* failure and nothing
// else, so folding it into a benign branch is something a caller has to write down rather
// than inherit from one `Result` that meant two things at once
pub fn run(program: &str, args: &[&str]) -> Result<Completed, String> {
    let out = Command::new(program).args(args).output().map_err(|e| {
        format!(
            "cannot run {}: {} — the check could not run; treating as failure (not clean)",
            program, e
        )
    })?;
    Ok(Completed {
        status: out.status,
        stdout: out.stdout,
    })
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — `run` with a body written to the child's stdin,
// the one shape `run` cannot carry: a shell caller's `printf … | git hash-object --stdin` has no
// argv spelling, and routing it here keeps the spawn site single
pub fn run_with_stdin(program: &str, args: &[&str], input: &[u8]) -> Result<Completed, String> {
    use std::io::Write;
    use std::process::Stdio;
    let spawn_err = |e: std::io::Error| {
        format!(
            "cannot run {}: {} — the check could not run; treating as failure (not clean)",
            program, e
        )
    };
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(spawn_err)?;
    let mut pipe = child
        .stdin
        .take()
        .ok_or_else(|| format!("cannot run {}: no stdin pipe — treating as failure (not clean)", program))?;
    pipe.write_all(input).map_err(spawn_err)?;
    drop(pipe);
    let out = child.wait_with_output().map_err(spawn_err)?;
    Ok(Completed {
        status: out.status,
        stdout: out.stdout,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walk;
    use std::path::Path;

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the wrapper is exercised directly,
    // because a gate's own fixture pair cannot prove it: no static input crashes a child.
    #[test]
    fn a_child_that_exited_non_zero_yields_no_stdout() {
        let c = run("git", &["--no-such-flag-whatsoever"])
            .expect("git is absent — the crate's own test suite already requires it");
        assert!(
            c.stdout().is_none(),
            "stdout was readable from a child that exited non-zero — the captured-emptiness \
             false-green is representable again"
        );
    }

    #[test]
    fn a_spawn_that_never_happened_is_an_error_not_an_empty_capture() {
        let e = run("checkwright-no-such-program-exists", &[])
            .err()
            .expect("a missing program reported success");
        assert!(
            e.contains("not clean"),
            "the spawn-failure line dropped the fail-closed wording, so a caller printing it \
             would not say the check did not run: {}",
            e
        );
    }

    #[test]
    fn a_child_that_succeeded_yields_its_stdout() {
        let c = run("git", &["--version"]).expect("cannot run git --version");
        assert!(
            !c.stdout()
                .expect("no stdout from a child that exited zero")
                .is_empty(),
            "git --version printed nothing"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the routing half, in the roster shape
    // §check-reads-couples' unit test B uses for filesystem walks; that section owns the
    // corpus this scans and why the bridge helpers sit outside it
    #[test]
    fn no_gate_module_constructs_a_subprocess_itself() {
        walk::bridge_declared_knobs(&crate::knobenv::lock());
        let gates = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/gates");
        let files = walk::find_files(&gates, &["rs"]).expect("cannot enumerate the gate modules");
        assert!(!files.is_empty(), "no gate module found to scan");
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            let text = std::fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            if text.contains("Command") {
                offenders.push(f.display().to_string());
            }
        }
        assert!(
            offenders.is_empty(),
            "a gate module builds its own subprocess ({:?}) — `Ok` from `Command::output()` \
             means the spawn succeeded, never that the child did, so reading stdout there \
             reproduces the captured-emptiness false-green. Route it through proc::run, and \
             widen proc.rs if the call needs something proc::run does not carry",
            offenders
        );
    }
}
