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
    #[cfg(windows)]
    let pathext = Some(std::env::var("PATHEXT").unwrap_or_default());
    #[cfg(not(windows))]
    let pathext: Option<String> = None;
    resolve_on_path(
        program,
        std::env::var_os("PATH").as_deref(),
        pathext.as_deref(),
        is_executable,
    )
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the fallback the section names, spelled once so
// the candidate set has a value when the host offers none
const PATHEXT_DEFAULT: &str = ".COM;.EXE;.BAT;.CMD";

// spec: gate-sdk/SPEC.md §Fail-closed contract — the crate's single owner of what an *installed*
// program may be named; `None` is a platform with no such question, so no caller appends an
// extension of its own and the two substrates stop disagreeing
fn exe_candidates(program: &str, pathext: Option<&str>) -> Vec<String> {
    let mut out = vec![program.to_string()];
    let Some(raw) = pathext else {
        return out;
    };
    let raw = if raw.trim().is_empty() {
        PATHEXT_DEFAULT
    } else {
        raw
    };
    out.extend(
        raw.split(';')
            .map(str::trim)
            .filter(|e| !e.is_empty())
            .map(|e| format!("{}{}", program, e)),
    );
    out
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the resolution as a pure function of its three
// inputs, because the arms that matter cannot execute on the host that develops them: the caller
// reads the environment, this decides, and a test supplies both halves
fn resolve_on_path<F: Fn(&std::path::Path) -> bool>(
    program: &str,
    path: Option<&std::ffi::OsStr>,
    pathext: Option<&str>,
    exists: F,
) -> bool {
    let candidates = exe_candidates(program, pathext);
    if program.contains('/') {
        return candidates
            .iter()
            .any(|c| exists(std::path::Path::new(c)));
    }
    let Some(path) = path else {
        return false;
    };
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the separator is std's, never a literal, so a
    // drive letter's colon does not shear every entry past the first into a fragment
    std::env::split_paths(path).any(|dir| {
        let dir = if dir.as_os_str().is_empty() {
            std::path::PathBuf::from(".")
        } else {
            dir
        };
        candidates.iter().any(|c| exists(&dir.join(c)))
    })
}

// spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the crate's one executability predicate, in
// the two forms the platform admits: an execute bit on unix, mere file-ness where the filesystem
// carries none
#[cfg(unix)]
pub fn is_executable(p: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(p)
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
pub fn is_executable(p: &std::path::Path) -> bool {
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

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the same code for a wrapper that *prints* it
    // rather than branching on it, and that section owns why the two accessors are distinct rather
    // than one.
    pub fn reported_code(&self) -> i32 {
        exit_code(&self.status)
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

// spec: drift-kit/SPEC.md §The KPI plugin contract — `run` with additions to the *child's*
// environment, the one shape `run` cannot carry. Writing the child's rather than the process's is
// what leaves knobenv's guard the only writer of the process-global one.
pub fn run_with_env(
    program: &str,
    args: &[&str],
    env: &[(String, String)],
) -> Result<Completed, String> {
    #[cfg(test)]
    recorder::note(program);
    let mut cmd = Command::new(program);
    cmd.args(args);
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().map_err(|e| {
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

// spec: gate-sdk/SPEC.md §Fail-closed contract — a child fed a body on stdin whose stdout is
// captured apart from its stderr; `code` is the pipeline element's own `$?`
pub struct Streamed {
    code: i32,
    stdout: Vec<u8>,
}

impl Streamed {
    pub fn code(&self) -> i32 {
        self.code
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — read whatever the child wrote, `Merged`'s
    // rule and not `Completed`'s: a filter's caller grades the *stream* it framed, so withholding
    // it on a non-zero status would hide the truncation the framing check exists to catch
    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — `run_with_stdin`'s file-backed counterpart: two
// capture files, so a body too large for a pipe cannot deadlock, and an unmerged stderr
pub fn run_streamed(
    program: &str,
    args: &[&str],
    input: &[u8],
    stderr: Stderr,
) -> Result<Streamed, String> {
    #[cfg(test)]
    recorder::note(program);
    let io_err = |e: std::io::Error| {
        format!(
            "cannot run {}: {} — the check could not run; treating as failure (not clean)",
            program, e
        )
    };
    let seq = MERGE_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir();
    let inp = dir.join(format!("checkwright-in.{}.{}", std::process::id(), seq));
    let outp = dir.join(format!("checkwright-out.{}.{}", std::process::id(), seq));
    let cleanup = || {
        let _ = std::fs::remove_file(&inp);
        let _ = std::fs::remove_file(&outp);
    };
    std::fs::write(&inp, input).map_err(io_err)?;
    let feed = match std::fs::File::open(&inp) {
        Ok(f) => f,
        Err(e) => {
            cleanup();
            return Err(io_err(e));
        }
    };
    let sink = match std::fs::File::create(&outp) {
        Ok(f) => f,
        Err(e) => {
            cleanup();
            return Err(io_err(e));
        }
    };
    let mut cmd = Command::new(program);
    cmd.args(args)
        .stdin(std::process::Stdio::from(feed))
        .stdout(std::process::Stdio::from(sink));
    if matches!(stderr, Stderr::Discard) {
        cmd.stderr(std::process::Stdio::null());
    }
    let status = match cmd.status() {
        Ok(s) => s,
        // spec: gate-sdk/SPEC.md §Fail-closed contract — bash's own verdict on a pipeline element
        // it could not start, which the member prints inside its own refusal
        Err(e) => {
            cleanup();
            let code = if e.kind() == std::io::ErrorKind::PermissionDenied {
                126
            } else {
                127
            };
            return Ok(Streamed {
                code,
                stdout: Vec::new(),
            });
        }
    };
    let stdout = match std::fs::read(&outp) {
        Ok(b) => b,
        Err(e) => {
            cleanup();
            return Err(io_err(e));
        }
    };
    cleanup();
    Ok(Streamed {
        code: exit_code(&status),
        stdout,
    })
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — a filter's stderr policy, named rather than
// passed as a bare flag: a probe run discards it the way the shell form's `2>/dev/null` does, and
// a scanning run leaves it alone
pub enum Stderr {
    Inherit,
    Discard,
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

    // spec: gate-sdk/SPEC.md §Fail-closed contract — a host with no suffix question asks for the
    // bare name and nothing else, so the Windows arm cannot leak a `.EXE` probe onto a Unix PATH
    #[test]
    fn a_platform_without_the_suffix_question_probes_the_bare_name_only() {
        assert_eq!(exe_candidates("cargo", None), vec!["cargo".to_string()]);
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — unset and empty are the same input to this
    // question and both take the fallback, which is the half a host that *has* PATHEXT never shows
    #[test]
    fn an_absent_or_blank_pathext_falls_back_to_the_default_set() {
        for raw in ["", "   "] {
            assert_eq!(
                exe_candidates("cargo", Some(raw)),
                vec![
                    "cargo".to_string(),
                    "cargo.COM".to_string(),
                    "cargo.EXE".to_string(),
                    "cargo.BAT".to_string(),
                    "cargo.CMD".to_string(),
                ],
                "a blank PATHEXT ({:?}) did not take the fallback set",
                raw
            );
        }
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the environment outranks the fallback, and a
    // host that lists an extension this crate never spelled is exactly why it is read at all
    #[test]
    fn a_populated_pathext_is_read_rather_than_the_fallback() {
        assert_eq!(
            exe_candidates("cargo", Some(".EXE; .PS1 ;;.CMD")),
            vec![
                "cargo".to_string(),
                "cargo.EXE".to_string(),
                "cargo.PS1".to_string(),
                "cargo.CMD".to_string(),
            ]
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the defect in full: a program installed only
    // as a shim answers `false` without the suffix set, which is a refusal on a present program
    #[test]
    fn a_program_installed_only_as_a_shim_is_found_by_the_suffix_set() {
        let path = std::env::join_paths(["/nowhere", "/opt/bin"]).expect("cannot join a PATH");
        let only_the_shim = |p: &Path| p == Path::new("/opt/bin/cargo.CMD");
        assert!(
            resolve_on_path("cargo", Some(&path), Some(""), only_the_shim),
            "the PATHEXT candidate set did not reach a .CMD shim"
        );
        assert!(
            !resolve_on_path("cargo", Some(&path), None, only_the_shim),
            "the bare name matched a .CMD shim, so this test proves nothing about the suffix set"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — an empty PATH entry is the cwd, bash's own
    // reading, kept across the rewrite
    #[test]
    fn an_empty_path_entry_still_means_the_working_directory() {
        let path = std::env::join_paths(["", "/opt/bin"]).expect("cannot join a PATH");
        assert!(resolve_on_path(
            "cargo",
            Some(&path),
            None,
            |p: &Path| p == Path::new("./cargo")
        ));
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the separator has no Linux-runnable oracle,
    // since `split_paths` compiles to the host's rule; this pins the API against a literal
    // returning, which is the only half of the defect a host here can still observe
    #[test]
    fn the_path_separator_is_never_spelled_as_a_literal() {
        let src = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/proc.rs"),
        )
        .expect("cannot read proc.rs");
        let literal = format!("split('{}')", ':');
        assert!(
            !src.contains(&literal),
            "a literal PATH separator is back in proc.rs — a Windows PATH separates on ';' and \
             its drive letters carry colons, so std::env::split_paths is the portable API"
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
