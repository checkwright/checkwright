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
    #[cfg(test)]
    recorder::note(program);
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

// spec: gate-sdk/SPEC.md §Fail-closed contract — the wrapper contract's presence probe, bash's
// `command -v <prog>`: it exists so a wrapper's refusal is its own message at the shell form's
// own point in the order, with `run`'s `Err` arm left as the backstop
pub fn on_path(program: &str) -> bool {
    if program.contains('/') {
        return is_executable(std::path::Path::new(program));
    }
    let Ok(path) = std::env::var("PATH") else {
        return false;
    };
    path.split(':').any(|dir| {
        let base = if dir.is_empty() { "." } else { dir };
        is_executable(&std::path::Path::new(base).join(program))
    })
}

#[cfg(unix)]
fn is_executable(p: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(p)
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(p: &std::path::Path) -> bool {
    p.is_file()
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — a child whose two streams were merged, the
// `2>&1` capture a wrapper's shell form takes; the false green is closed on the `succeeded()`
// side rather than by withholding the report
pub struct Merged {
    status: std::process::ExitStatus,
    output: Vec<u8>,
}

impl Merged {
    pub fn succeeded(&self) -> bool {
        self.status.success()
    }

    pub fn output(&self) -> &[u8] {
        &self.output
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the exit code for a wrapper whose program
    // grades itself by it: ShellCheck says 1 for findings and ≥2 for an error it could not lint
    // past, and folding both into `succeeded()` makes an unlintable fragment read as findings
    pub fn code(&self) -> Option<i32> {
        self.status.code()
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — `run`'s merged-capture face: two handles on one
// file description (`try_clone` is `dup`), `dispatch`'s own technique, so the streams interleave
// as bash's `2>&1` did rather than concatenating in the wrong order
pub fn run_merged(program: &str, args: &[&str]) -> Result<Merged, String> {
    #[cfg(test)]
    recorder::note(program);
    let spawn_err = |e: std::io::Error| {
        format!(
            "cannot run {}: {} — the check could not run; treating as failure (not clean)",
            program, e
        )
    };
    let capture = std::env::temp_dir().join(format!(
        "checkwright-merged.{}.{}",
        std::process::id(),
        MERGE_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let out = std::fs::File::create(&capture).map_err(spawn_err)?;
    let err = out.try_clone().map_err(spawn_err)?;
    let status = Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::from(out))
        .stderr(std::process::Stdio::from(err))
        .status();
    let status = match status {
        Ok(s) => s,
        Err(e) => {
            let _ = std::fs::remove_file(&capture);
            return Err(spawn_err(e));
        }
    };
    let output = std::fs::read(&capture).map_err(spawn_err)?;
    let _ = std::fs::remove_file(&capture);
    Ok(Merged { status, output })
}

static MERGE_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

// spec: gate-sdk/SPEC.md §Fail-closed contract — `run` with a body written to the child's stdin,
// the one shape `run` cannot carry: a shell caller's `printf … | git hash-object --stdin` has no
// argv spelling, and routing it here keeps the spawn site single
pub fn run_with_stdin(program: &str, args: &[&str], input: &[u8]) -> Result<Completed, String> {
    #[cfg(test)]
    recorder::note(program);
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

// spec: gate-sdk/SPEC.md §Fail-closed contract — the spawn recorder unit test A observes
// through, on the shape walk.rs's read recorder already has. Test-scoped deliberately: a
// production recorder would be state with no reader, and it is unreachable from a gate module.
#[cfg(test)]
pub mod recorder {
    use std::cell::RefCell;

    thread_local! {
        static OBSERVED: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
    }

    pub fn start() {
        OBSERVED.with(|o| *o.borrow_mut() = Some(Vec::new()));
    }

    pub fn stop() -> Vec<String> {
        OBSERVED.with(|o| o.borrow_mut().take()).unwrap_or_default()
    }

    pub fn note(program: &str) {
        OBSERVED.with(|o| {
            if let Some(v) = o.borrow_mut().as_mut() {
                if !v.iter().any(|e| e == program) {
                    v.push(program.to_string());
                }
            }
        });
    }
}

// spec: gate-sdk/SPEC.md §run-gates — one battery member's child: its own argv, its declared knob
// environment and nothing else's, a private `TMPDIR`, and stdout+stderr merged into one capture
// file. `code` is the member's verdict, so output is carried whatever the child's outcome.
pub struct Dispatched {
    pub code: i32,
    pub output: Vec<u8>,
}

// spec: gate-sdk/SPEC.md §run-gates — a child killed by a signal reports `128 + n`, the spelling
// bash's own `$?` gave the shell dispatcher this replaced, so the `FAIL: <name> (exit N)` tail
// keeps one grammar for `scripts/parse-gates-log.sh` and no fourth tail shape is minted
fn exit_code(status: &std::process::ExitStatus) -> i32 {
    if let Some(c) = status.code() {
        return c;
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(sig) = status.signal() {
            return 128 + sig;
        }
    }
    2
}

// spec: gate-sdk/SPEC.md §run-gates — the merge is two handles on **one** file description
// (`try_clone` is `dup`), so the two streams share an offset and interleave exactly as the shell
// dispatcher's `2>&1` did; reading them as two pipes would reorder a gate's own report.
// spec: gate-sdk/SPEC.md §run-gates — stdin is `/dev/null`: under a worker pool an inherited
// terminal is a shared resource two concurrent members could both read from.
pub fn dispatch(
    argv: &[String],
    env: &[(String, String)],
    drop_env: &[String],
    tmpdir: &std::path::Path,
    capture: &std::path::Path,
) -> Result<Dispatched, String> {
    let program = argv
        .first()
        .ok_or_else(|| "dispatch: empty argv — treating as failure (not clean)".to_string())?;
    let io_err = |what: &str, e: std::io::Error| {
        format!(
            "cannot {} for {}: {} — the check could not run; treating as failure (not clean)",
            what, program, e
        )
    };
    let out = std::fs::File::create(capture).map_err(|e| io_err("create the capture file", e))?;
    let err = out.try_clone().map_err(|e| io_err("share the capture file", e))?;
    let mut cmd = Command::new(program);
    cmd.args(&argv[1..])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::from(out))
        .stderr(std::process::Stdio::from(err))
        .env("TMPDIR", tmpdir);
    for name in drop_env {
        cmd.env_remove(name);
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    let status = cmd.status().map_err(|e| io_err("spawn", e))?;
    let output = std::fs::read(capture).map_err(|e| io_err("read the capture file", e))?;
    Ok(Dispatched {
        code: exit_code(&status),
        output,
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
            // spec: gate-sdk/SPEC.md §Fail-closed contract — the roster is the *code*
            // spellings a construction needs, never the bare word: a gate's own remedy text
            // may name `core.sshCommand`, and a detector that fires on prose gets muted
            if ["Command::", "process::Command"].iter().any(|sp| text.contains(sp)) {
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
