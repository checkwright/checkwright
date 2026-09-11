// spec: gate-sdk/SPEC.md §Fail-closed contract — the crate's one shipped spawn site outside the
// one declared `spawn-funnel-exempt:` shape, so the captured-emptiness false-green has no
// spelling elsewhere and no spawn goes around the Windows resolution below
use std::process::Command;

// spec: gate-sdk/SPEC.md §Fail-closed contract — a child that ran. Constructing one is the
// proof the spawn succeeded, which is the half `Command::output()`'s `Ok` actually carries.
pub struct Completed {
    status: std::process::ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
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

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the same widening `Merged` carries, for a
    // caller that *prints* a failed child's status rather than branching on it: a signal-killed
    // child reports `128 + n`, the spelling bash's own `$?` gave the shell forms this replaced
    pub fn reported_code(&self) -> i32 {
        exit_code(&self.status)
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the sanctioned widening: the failed child's
    // whole account of itself, reachable only where `stdout()` already said `None`, composed into
    // a `String` so no caller can parse it back into a verdict
    pub fn failure_report(&self) -> Option<String> {
        if self.status.success() {
            return None;
        }
        let part = |label: &str, raw: &[u8]| {
            let s = String::from_utf8_lossy(raw).trim().to_string();
            if s.is_empty() {
                format!("{}: <empty>", label)
            } else {
                format!("{}: {}", label, s)
            }
        };
        Some(format!(
            "exit {}; {}; {}",
            exit_code(&self.status),
            part("stdout", &self.stdout),
            part("stderr", &self.stderr)
        ))
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — `Err` is a *spawn* failure and nothing
// else, so folding it into a benign branch is something a caller has to write down rather
// than inherit from one `Result` that meant two things at once
pub fn run(program: &str, args: &[&str]) -> Result<Completed, String> {
    #[cfg(test)]
    recorder::note(program);
    let target = spawn_target(program)?;
    let out = Command::new(target.as_ref()).args(args).output().map_err(|e| {
        format!(
            "cannot run {}: {} — the check could not run; treating as failure (not clean)",
            program, e
        )
    })?;
    Ok(Completed {
        status: out.status,
        stdout: out.stdout,
        stderr: out.stderr,
    })
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the wrapper contract's presence probe, bash's
// `command -v <prog>`: it exists so a wrapper's refusal is its own message at the shell form's
// own point in the order, with `run`'s `Err` arm left as the backstop
pub fn on_path(program: &str) -> bool {
    which(program).is_some()
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the same probe reporting *where* it resolved,
// `command -v`'s stdout rather than its exit status: a caller that renders the resolved path must
// not walk PATH a second time to learn it.
pub fn which(program: &str) -> Option<String> {
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
// spec: gate-sdk/SPEC.md §Fail-closed contract — the bare name is the LAST candidate: Windows
// will not execute an extensionless file, so a bare-first order was the POSIX rule applied on
// the one platform that does not use it
fn exe_candidates(program: &str, pathext: Option<&str>) -> Vec<String> {
    let Some(raw) = pathext else {
        return vec![program.to_string()];
    };
    let raw = if raw.trim().is_empty() {
        PATHEXT_DEFAULT
    } else {
        raw
    };
    let mut out: Vec<String> = raw
        .split(';')
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(|e| format!("{}{}", program, e))
        .collect();
    out.push(program.to_string());
    out
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the spawn's own resolution, the same `which` the
// probe answers from, so `on_path(P)` true means the spawn of `P` reaches the file `which(P)`
// named; it runs AFTER `recorder::note`, which leaves every registry declaration matching
#[cfg(windows)]
fn spawn_target(program: &str) -> Result<std::borrow::Cow<'_, str>, String> {
    let dirs: Vec<std::path::PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    let pathext = std::env::var("PATHEXT").unwrap_or_default();
    let system_root = std::env::var("SystemRoot").ok();
    spawn_resolution(
        program,
        &dirs,
        Some(pathext.as_str()),
        system_root.as_deref(),
        is_executable,
    )
    .map(std::borrow::Cow::Owned)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the pass-through arm, `resolve_floor_tool`'s own
// ground: resolving here would swap the spawned literal for an absolute path on every host the
// battery runs on
#[cfg(not(windows))]
fn spawn_target(program: &str) -> Result<std::borrow::Cow<'_, str>, String> {
    Ok(std::borrow::Cow::Borrowed(program))
}

// spec: gate-sdk/SPEC.md §check-graph — the funnel's decision as a pure function of its inputs,
// `resolve_on_path`'s own shape: the roster picks which face resolves, and a host that cannot
// execute the Windows arm still asserts it by injecting the three
#[cfg_attr(not(windows), allow(dead_code))]
fn spawn_resolution<F: Fn(&std::path::Path) -> bool>(
    program: &str,
    dirs: &[std::path::PathBuf],
    pathext: Option<&str>,
    system_root: Option<&str>,
    exists: F,
) -> Result<String, String> {
    // spec: gate-sdk/SPEC.md §check-graph — an argv[0] carrying a separator is a path the caller
    // already resolved, never a name for the platform to search, so it passes through untouched
    if program.contains('/') || program.contains('\\') {
        return Ok(program.to_string());
    }
    let disposition = homonym_disposition(program);
    // spec: gate-sdk/SPEC.md §check-graph — the system-directory rejection is program-class-
    // specific: a name the roster does not carry has no homonym there and earns no rejection
    let root = match disposition {
        Some(_) => system_root,
        None => None,
    };
    match resolve_outside_system_dir(program, dirs, pathext, root, exists) {
        Ok(p) => Ok(p),
        Err(e) if disposition == Some(NoResolution::Refuse) => Err(e),
        Err(_) => Ok(program.to_string()),
    }
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the resolution as a pure function of its three
// inputs, because the arms that matter cannot execute on the host that develops them: the caller
// reads the environment, this decides, and a test supplies both halves
fn resolve_on_path<F: Fn(&std::path::Path) -> bool>(
    program: &str,
    path: Option<&std::ffi::OsStr>,
    pathext: Option<&str>,
    exists: F,
) -> Option<String> {
    let candidates = exe_candidates(program, pathext);
    if program.contains('/') {
        return candidates
            .iter()
            .find(|c| exists(std::path::Path::new(c)))
            .cloned();
    }
    let path = path?;
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the separator is std's, never a literal, so a
    // drive letter's colon does not shear every entry past the first into a fragment
    std::env::split_paths(path).find_map(|dir| {
        let dir = if dir.as_os_str().is_empty() {
            std::path::PathBuf::from(".")
        } else {
            dir
        };
        candidates
            .iter()
            .map(|c| dir.join(c))
            .find(|p| exists(p))
            .map(|p| p.display().to_string())
    })
}

// spec: gate-sdk/SPEC.md §check-graph — the three views Windows shows one system directory
// through; all three reach the same WSL launcher, so rejecting `System32` alone leaves a PATH
// spelled with either of its siblings walking straight around the rejection
const WINDOWS_SYSTEM_DIR_VIEWS: [&str; 3] = ["System32", "SysWOW64", "Sysnative"];

// spec: gate-sdk/SPEC.md §check-graph — what a host offering only the system directory's homonym
// earns. `Refuse` is `resolve_interpreter`'s face and `FallBack` is `resolve_floor_tool`'s, so the
// roster below picks the face rather than a call site picking it by which function it calls
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(not(windows), allow(dead_code))]
enum NoResolution {
    Refuse,
    FallBack,
}

// spec: gate-sdk/SPEC.md §check-graph — the governed roster: a name belongs here when the Windows
// system directory ships a program of that name that is NOT the program the payload wants. It is a
// fact about the platform, never about a consumer, so no knob widens it
#[cfg_attr(not(windows), allow(dead_code))]
const SYSTEM_DIR_HOMONYMS: &[(&str, NoResolution)] = &[
    ("bash", NoResolution::Refuse),
    ("sort", NoResolution::FallBack),
];

// spec: gate-sdk/SPEC.md §check-graph — the roster's one lookup, by exact match against the
// unresolved program name the caller passed
#[cfg_attr(not(windows), allow(dead_code))]
fn homonym_disposition(program: &str) -> Option<NoResolution> {
    SYSTEM_DIR_HOMONYMS
        .iter()
        .find(|(name, _)| *name == program)
        .map(|(_, d)| *d)
}

// spec: gate-sdk/SPEC.md §check-graph — a directory folded the way Windows compares one, case and
// separator both: `c:/windows/system32` and `C:\Windows\System32` are one directory, and a
// comparison that missed that would pass on every developer host and fail on the one host at issue
fn windows_dir_key(p: &std::path::Path) -> String {
    p.to_string_lossy()
        .replace('/', "\\")
        .trim_end_matches('\\')
        .to_lowercase()
}

// spec: gate-sdk/SPEC.md §check-graph — the rejection as a pure function of its inputs, the shape
// `resolve_on_path`'s tests already drive: the caller reads the environment and this decides, so
// the Windows arm is asserted from a host that can never execute it
// spec: gate-sdk/SPEC.md §check-graph — the directories arrive already split because a Windows
// PATH cannot survive `split_paths` on a POSIX host, which is what makes the arm injectable
fn resolve_outside_system_dir<F: Fn(&std::path::Path) -> bool>(
    program: &str,
    dirs: &[std::path::PathBuf],
    pathext: Option<&str>,
    system_root: Option<&str>,
    exists: F,
) -> Result<String, String> {
    let candidates = exe_candidates(program, pathext);
    let rejected: Vec<String> = system_root
        .into_iter()
        .flat_map(|root| {
            WINDOWS_SYSTEM_DIR_VIEWS
                .iter()
                .map(move |view| windows_dir_key(&std::path::Path::new(root).join(view)))
        })
        .collect();
    dirs.iter()
        .filter(|dir| !rejected.contains(&windows_dir_key(dir.as_path())))
        .find_map(|dir| {
            let dir: &std::path::Path = if dir.as_os_str().is_empty() {
                std::path::Path::new(".")
            } else {
                dir.as_path()
            };
            candidates
                .iter()
                .map(|c| dir.join(c))
                .find(|p| exists(p))
                .map(|p| p.display().to_string())
        })
        .ok_or_else(|| {
            format!(
                "cannot resolve {0} to an executable on PATH — a Windows system directory is \
                 skipped, its {0} being the WSL launcher rather than a shell; the check could not \
                 run, treating as failure (not clean)",
                program
            )
        })
}

// spec: gate-sdk/SPEC.md §check-graph — the interpreter a spawn runs, resolved to a path rather
// than named: `GATE_SDK_PROGRAM_FLOOR` guarantees a `bash` exists on the host, never that a bare
// name reaches it, and on Windows the bare name reaches System32's WSL launcher instead.
// spec: gate-sdk/SPEC.md §check-graph — the `Refuse` face of `SYSTEM_DIR_HOMONYMS`, reached
// through the funnel rather than pointed at a call site: the roster owns which names take it, so
// no site chooses a disposition by choosing which resolver it calls
#[cfg_attr(not(windows), allow(dead_code))]
pub fn resolve_interpreter(program: &str) -> Result<String, String> {
    #[cfg(windows)]
    let (pathext, system_root) = (
        Some(std::env::var("PATHEXT").unwrap_or_default()),
        std::env::var("SystemRoot").ok(),
    );
    #[cfg(not(windows))]
    let (pathext, system_root): (Option<String>, Option<String>) = (None, None);
    let dirs: Vec<std::path::PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    resolve_outside_system_dir(
        program,
        &dirs,
        pathext.as_deref(),
        system_root.as_deref(),
        is_executable,
    )
}

// spec: context-kit/SPEC.md §bin/env-probe — the `FallBack` face of `SYSTEM_DIR_HOMONYMS`, and a
// REPORTING resolver besides: its value is rendered in doctor's banner and the env-probe emitter,
// which is the identity the funnel cannot absorb and why it keeps its own callers
// spec: context-kit/SPEC.md §bin/env-probe — a host offering the tool nowhere outside the system
// directory falls back to the bare name rather than refusing: the verdict is then the roster's own
// absent or wrong-impl, which is the true reading of such a host and the fail-closed direction.
#[cfg(windows)]
pub fn resolve_floor_tool(program: &str) -> String {
    let dirs: Vec<std::path::PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    let pathext = std::env::var("PATHEXT").unwrap_or_default();
    let system_root = std::env::var("SystemRoot").ok();
    resolve_outside_system_dir(
        program,
        &dirs,
        Some(pathext.as_str()),
        system_root.as_deref(),
        is_executable,
    )
    .unwrap_or_else(|_| program.to_string())
}

// spec: context-kit/SPEC.md §bin/env-probe — the name passes through where the platform has no
// system-directory homonym: a POSIX spawn already searches `PATH` and nothing else, and resolving
// here would swap the spawned literal for an absolute path on every host the battery runs on.
#[cfg(not(windows))]
pub fn resolve_floor_tool(program: &str) -> String {
    program.to_string()
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
    run_merged_in(program, args, &[], None)
}

// spec: gate-sdk/SPEC.md §run-gate-tests — `run_merged` with the child's own working directory and
// environment additions, the one shape `run_merged` cannot carry: a fixture case runs *inside* its
// own case dir, and setting the caller's cwd instead would be process-global.
pub fn run_merged_in(
    program: &str,
    args: &[&str],
    env: &[(String, String)],
    cwd: Option<&std::path::Path>,
) -> Result<Merged, String> {
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
    let mut cmd = Command::new(spawn_target(program)?.as_ref());
    cmd.args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::from(out))
        .stderr(std::process::Stdio::from(err));
    for (k, v) in env {
        cmd.env(k, v);
    }
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let status = cmd.status();
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

// spec: delegation-kit/SPEC.md §The turn-end liveness hook — `run` under a wall-clock bound, the
// one shape `run` cannot carry: a hook member calling a consumer-named reader must not hang a turn
// on it. `Ok(None)` is the bound expiring, `timeout(1)`'s 124 without the optional program.
pub fn run_bounded(program: &str, args: &[&str], secs: u64) -> Result<Option<i32>, String> {
    #[cfg(test)]
    recorder::note(program);
    let mut child = Command::new(spawn_target(program)?.as_ref())
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| {
            format!(
                "cannot run {}: {} — the check could not run; treating as failure (not clean)",
                program, e
            )
        })?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(Some(exit_code(&status))),
            Ok(None) => {}
            Err(e) => return Err(format!("cannot wait for {}: {}", program, e)),
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — `run_bounded` for a caller that needs
// the child's *output* and not only its code. Capture goes to a file, not a pipe: a poll loop and a
// filled pipe buffer deadlock each other. `Ok(None)` is the bound expiring.
pub fn run_bounded_capture(
    program: &str,
    args: &[&str],
    secs: u64,
) -> Result<Option<(i32, Vec<u8>)>, String> {
    #[cfg(test)]
    recorder::note(program);
    let spawn_err = |e: std::io::Error| {
        format!(
            "cannot run {}: {} — the check could not run; treating as failure (not clean)",
            program, e
        )
    };
    let capture = std::env::temp_dir().join(format!(
        "checkwright-bounded.{}.{}",
        std::process::id(),
        MERGE_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let out = std::fs::File::create(&capture).map_err(spawn_err)?;
    let mut child = Command::new(spawn_target(program)?.as_ref())
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::from(out))
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| {
            let _ = std::fs::remove_file(&capture);
            spawn_err(e)
        })?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
    let outcome = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(exit_code(&status)),
            Ok(None) => {}
            Err(e) => {
                let _ = std::fs::remove_file(&capture);
                return Err(format!("cannot wait for {}: {}", program, e));
            }
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    };
    let bytes = std::fs::read(&capture).map_err(spawn_err)?;
    let _ = std::fs::remove_file(&capture);
    Ok(outcome.map(|code| (code, bytes)))
}

// spec: drift-kit/SPEC.md §The KPI plugin contract — `run` with additions to the *child's*
// environment, the one shape `run` cannot carry. Writing the child's rather than the process's is
// what leaves knobenv's guard the only writer of the process-global one.
pub fn run_with_env(
    program: &str,
    args: &[&str],
    env: &[(String, String)],
) -> Result<Completed, String> {
    run_with_env_in(program, args, env, None)
}

// spec: context-kit/SPEC.md §Testing — `run_with_env` with the child's own working directory, the
// one shape it cannot carry: an arm invoking a front-end by absolute path must still place the
// child inside the tree, that front-end refusing outside a git repository.
pub fn run_with_env_in(
    program: &str,
    args: &[&str],
    env: &[(String, String)],
    cwd: Option<&std::path::Path>,
) -> Result<Completed, String> {
    #[cfg(test)]
    recorder::note(program);
    let mut cmd = Command::new(spawn_target(program)?.as_ref());
    cmd.args(args);
    for (k, v) in env {
        cmd.env(k, v);
    }
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
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
        stderr: out.stderr,
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
    let mut child = Command::new(spawn_target(program)?.as_ref())
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
    // comment-tier-exempt: the body is written while the output drains, so a child that answers as
    // it reads cannot fill its stdout pipe and stall against a writer still blocked on its stdin
    let (written, out) = std::thread::scope(|s| {
        let writer = s.spawn(move || pipe.write_all(input));
        let out = child.wait_with_output();
        (writer.join(), out)
    });
    match written {
        Ok(Ok(())) => {}
        Ok(Err(e)) => return Err(spawn_err(e)),
        Err(_) => {
            return Err(format!(
                "cannot run {}: the stdin writer panicked — treating as failure (not clean)",
                program
            ))
        }
    }
    let out = out.map_err(spawn_err)?;
    Ok(Completed {
        status: out.status,
        stdout: out.stdout,
        stderr: out.stderr,
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
    let mut cmd = Command::new(spawn_target(program)?.as_ref());
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

// spec: gate-sdk/SPEC.md §Fail-closed contract — where an *uncaptured* child's two streams go, the
// one shape the capturing spawns above cannot carry: `Inherit` is a consumer hook running in the
// caller's own terminal and `File` is `>"$log" 2>&1`, one description both handles dup onto.
pub enum Sink {
    Inherit,
    File(std::path::PathBuf),
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the spine's own spawn: a configured command
// whose output is a captured *artifact* rather than a value this process reads, so it is written
// through as the child produces it. `Err` stays a spawn failure alone, `run`'s rule.
pub fn run_to(program: &str, args: &[&str], sink: &Sink) -> Result<i32, String> {
    run_to_env(program, args, &[], sink)
}

// spec: gate-sdk/SPEC.md §run-gates — `run_to` with the child's own declared knob environment
// added, the shape a member dispatching another member owes: the callee is a child rather than an
// in-process call precisely so it reads the knobs its own registry entry declares.
pub fn run_to_env(
    program: &str,
    args: &[&str],
    env: &[(String, String)],
    sink: &Sink,
) -> Result<i32, String> {
    #[cfg(test)]
    recorder::note(program);
    let spawn_err = |e: std::io::Error| {
        format!(
            "cannot run {}: {} — the check could not run; treating as failure (not clean)",
            program, e
        )
    };
    let mut cmd = Command::new(spawn_target(program)?.as_ref());
    cmd.args(args);
    for (k, v) in env {
        cmd.env(k, v);
    }
    if let Sink::File(path) = sink {
        let out = std::fs::File::create(path).map_err(spawn_err)?;
        let err = out.try_clone().map_err(spawn_err)?;
        cmd.stdout(std::process::Stdio::from(out))
            .stderr(std::process::Stdio::from(err));
    }
    cmd.status().map(|s| exit_code(&s)).map_err(spawn_err)
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
// keeps one grammar for `--emit-parse-gates-log` and no fourth tail shape is minted
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

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the branch the narrowing itself made
    // unreadable: a child that diagnoses itself on stdout, which a shell script's own `echo`
    // does by default, and which `stdout()` withholds on a non-zero exit
    #[test]
    fn a_child_that_failed_on_stdout_alone_still_reports_what_it_said() {
        let c = run("bash", &["-c", "echo the-generator-said-this; exit 3"])
            .expect("bash is absent — it is on the program floor the gate modules spawn against");
        let report = c
            .failure_report()
            .expect("no report from a child that exited non-zero");
        assert!(
            report.contains("the-generator-said-this"),
            "the report dropped a failed child's stdout, so a generator that diagnoses itself \
             there is unreadable exactly as it was before the widening: {}",
            report
        );
        assert!(
            report.contains("exit 3"),
            "the report did not name the exit code: {}",
            report
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the 2026-08-31 Windows round's own shape: a
    // child that exits non-zero and says nothing on either stream still names the status it died
    // with, because the exit code is the datum that always exists
    #[test]
    fn a_child_that_failed_silently_still_reports_its_exit_code() {
        let c = run("bash", &["-c", "exit 4"])
            .expect("bash is absent — it is on the program floor the gate modules spawn against");
        let report = c
            .failure_report()
            .expect("no report from a child that exited non-zero");
        assert!(
            report.contains("exit 4"),
            "a silent child yielded a report that does not name its exit code — the bare refusal \
             that cost a whole CI round is representable again: {:?}",
            report
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
                    "cargo.COM".to_string(),
                    "cargo.EXE".to_string(),
                    "cargo.BAT".to_string(),
                    "cargo.CMD".to_string(),
                    "cargo".to_string(),
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
                "cargo.EXE".to_string(),
                "cargo.PS1".to_string(),
                "cargo.CMD".to_string(),
                "cargo".to_string(),
            ]
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the ordering the defect turned on, injected
    // on a host that cannot execute it: Node's Windows install leaves an extensionless `npm` sh
    // script beside its `npm.cmd` shim, and `CreateProcessW` can run only the second
    #[test]
    fn a_pathext_shim_beats_the_extensionless_file_beside_it() {
        let path = std::env::join_paths(["/opt/nodejs"]).expect("cannot join a PATH");
        let both_exist =
            |p: &Path| p == Path::new("/opt/nodejs/npm") || p == Path::new("/opt/nodejs/npm.CMD");
        assert_eq!(
            resolve_on_path("npm", Some(&path), Some(""), both_exist),
            Some("/opt/nodejs/npm.CMD".to_string()),
            "the extensionless sh script beat the .CMD shim beside it, which is the resolution \
             CreateProcessW cannot run"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the bare name survives as a candidate and is
    // reached where no PATHEXT variant exists, which is the half the reordering must not break
    #[test]
    fn the_bare_name_is_still_reached_where_no_variant_exists() {
        let path = std::env::join_paths(["/opt/bin"]).expect("cannot join a PATH");
        assert_eq!(
            resolve_on_path("cargo", Some(&path), Some(""), |p: &Path| p
                == Path::new("/opt/bin/cargo")),
            Some("/opt/bin/cargo".to_string()),
            "the bare name stopped being a candidate, so a caller naming cargo.exe and a Unix \
             host no longer resolve through the same loop"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the funnel's pass-through arm, on
    // `a_posix_floor_tool_is_spawned_under_its_bare_name`'s shape: resolving on POSIX would swap
    // the spawned literal for an absolute path on every host the battery runs on
    #[cfg(not(windows))]
    #[test]
    fn a_posix_spawn_reaches_the_bare_name_the_caller_passed() {
        assert_eq!(spawn_target("bash").as_deref(), Ok("bash"));
        assert_eq!(
            spawn_target("checkwright-no-such-program").as_deref(),
            Ok("checkwright-no-such-program")
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the defect in full: a program installed only
    // as a shim answers `false` without the suffix set, which is a refusal on a present program
    #[test]
    fn a_program_installed_only_as_a_shim_is_found_by_the_suffix_set() {
        let path = std::env::join_paths(["/nowhere", "/opt/bin"]).expect("cannot join a PATH");
        let only_the_shim = |p: &Path| p == Path::new("/opt/bin/cargo.CMD");
        assert!(
            resolve_on_path("cargo", Some(&path), Some(""), only_the_shim).is_some(),
            "the PATHEXT candidate set did not reach a .CMD shim"
        );
        assert!(
            resolve_on_path("cargo", Some(&path), None, only_the_shim).is_none(),
            "the bare name matched a .CMD shim, so this test proves nothing about the suffix set"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — an empty PATH entry is the cwd, bash's own
    // reading, kept across the rewrite
    #[test]
    fn an_empty_path_entry_still_means_the_working_directory() {
        let path = std::env::join_paths(["", "/opt/bin"]).expect("cannot join a PATH");
        assert_eq!(
            resolve_on_path("cargo", Some(&path), None, |p: &Path| p == Path::new("./cargo")),
            Some("./cargo".to_string())
        );
    }

    // spec: gate-sdk/SPEC.md §check-graph — the host's own folding, applied to both sides of every
    // assertion below, so a test written on a POSIX developer machine still reads a joined path
    // the way the Windows this repair exists for would
    fn folded(s: &str) -> String {
        s.replace('\\', "/").to_lowercase()
    }

    fn a_bash_in_every_directory(p: &Path) -> bool {
        folded(&p.to_string_lossy()).ends_with("/bash.exe")
    }

    // spec: gate-sdk/SPEC.md §check-graph — round 7's measured Windows host as an assertion a
    // Linux developer machine runs: System32 precedes Git's `usr/bin`, both hold a `bash`, and the
    // resolution must reach the second because the first one is the WSL launcher
    #[test]
    fn the_attested_windows_path_resolves_past_the_wsl_launcher() {
        let dirs = vec![
            std::path::PathBuf::from(r"C:\Windows\System32"),
            std::path::PathBuf::from(r"C:\Program Files\Git\usr\bin"),
        ];
        let got = resolve_outside_system_dir(
            "bash",
            &dirs,
            Some(""),
            Some(r"C:\Windows"),
            a_bash_in_every_directory,
        )
        .expect("nothing resolved on a PATH whose second entry holds a bash");
        assert_eq!(
            folded(&got),
            "c:/program files/git/usr/bin/bash.exe",
            "the resolution took System32's bash, which is the WSL launcher and not a shell"
        );
    }

    // spec: gate-sdk/SPEC.md §check-graph — the case-sensitive comparison passes on every host
    // except the one this exists for, so the folding is asserted with the two sides spelled apart
    #[test]
    fn the_system_directory_is_rejected_in_its_lowercase_spelling_too() {
        let dirs = vec![
            std::path::PathBuf::from(r"c:\windows\system32"),
            std::path::PathBuf::from(r"C:\Program Files\Git\usr\bin"),
        ];
        let got = resolve_outside_system_dir(
            "bash",
            &dirs,
            Some(""),
            Some(r"C:\WINDOWS"),
            a_bash_in_every_directory,
        )
        .expect("nothing resolved on a PATH whose second entry holds a bash");
        assert_eq!(
            folded(&got),
            "c:/program files/git/usr/bin/bash.exe",
            "a lowercase system directory escaped the rejection"
        );
    }

    // spec: gate-sdk/SPEC.md §check-graph — the refusal is named rather than a fall-through to the
    // bare name, and it is asserted over every view in the roster rather than the one spelling
    #[test]
    fn a_path_offering_only_the_system_directory_is_a_named_refusal() {
        for view in WINDOWS_SYSTEM_DIR_VIEWS {
            let dirs = vec![Path::new(r"C:\Windows").join(view)];
            let err = resolve_outside_system_dir(
                "bash",
                &dirs,
                Some(""),
                Some(r"C:\Windows"),
                a_bash_in_every_directory,
            )
            .expect_err("the WSL launcher was accepted as a bash");
            assert!(
                err.contains("WSL launcher") && err.contains("not clean"),
                "the {} refusal did not name its cause fail-closed: {}",
                view,
                err
            );
        }
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the floor probe's case, which the interpreter's
    // does not cover: `sort` is on `PATH` in both directories and the resolution must reach the
    // coreutils one, System32's being the line sorter that has no `-V` for the floor to compare with
    #[test]
    fn the_floor_probe_resolves_sort_past_the_system_directorys_line_sorter() {
        let dirs = vec![
            std::path::PathBuf::from(r"C:\Windows\System32"),
            std::path::PathBuf::from(r"C:\Program Files\Git\usr\bin"),
        ];
        let got = resolve_outside_system_dir(
            "sort",
            &dirs,
            Some(""),
            Some(r"C:\Windows"),
            |p: &Path| folded(&p.to_string_lossy()).ends_with("/sort.exe"),
        )
        .expect("nothing resolved on a PATH whose second entry holds a sort");
        assert_eq!(
            folded(&got),
            "c:/program files/git/usr/bin/sort.exe",
            "the resolution took System32's sort, which cannot version-compare"
        );
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the passthrough arm, asserted on the host that
    // compiles it: resolving on POSIX would change the spawned literal every registry declaration
    // is compared against, so the name must come back unaltered
    #[cfg(not(windows))]
    #[test]
    fn a_posix_floor_tool_is_spawned_under_its_bare_name() {
        assert_eq!(resolve_floor_tool("sort"), "sort");
    }

    // spec: gate-sdk/SPEC.md §check-graph — the half no resolver case reaches: that a BARE-NAME
    // spawn now takes the refusing resolution, which is what deltas 1 and 2 change and what
    // leaves no trace in the twenty-three call sites that spawn `"bash"` before and after
    #[test]
    fn a_funnelled_refuse_member_refuses_on_a_system_directory_only_path() {
        let dirs = vec![std::path::PathBuf::from(r"C:\Windows\System32")];
        let err = spawn_resolution(
            "bash",
            &dirs,
            Some(""),
            Some(r"C:\Windows"),
            a_bash_in_every_directory,
        )
        .expect_err("a funnelled bare `bash` accepted the WSL launcher");
        assert!(
            err.contains("WSL launcher") && err.contains("not clean"),
            "the funnel's refusal did not name its cause fail-closed: {}",
            err
        );
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the roster's other disposition through the same
    // funnel: a `FallBack` member on the same PATH yields the bare name, so the verdict is the
    // floor roster's own absent-or-wrong-impl rather than a refusal the probe cannot report
    #[test]
    fn a_funnelled_fallback_member_yields_the_bare_name_on_the_same_path() {
        let dirs = vec![std::path::PathBuf::from(r"C:\Windows\System32")];
        assert_eq!(
            spawn_resolution("sort", &dirs, Some(""), Some(r"C:\Windows"), |p: &Path| {
                folded(&p.to_string_lossy()).ends_with("/sort.exe")
            }),
            Ok("sort".to_string()),
            "a FallBack member refused instead of falling back to the bare name"
        );
    }

    // spec: gate-sdk/SPEC.md §check-graph — a name the roster does not carry earns no
    // system-directory rejection: the homonym question is program-class-specific, so a
    // System32-only PATH still resolves `npm` there
    #[test]
    fn a_name_off_the_roster_earns_no_system_directory_rejection() {
        let dirs = vec![std::path::PathBuf::from(r"C:\Windows\System32")];
        let got = spawn_resolution("npm", &dirs, Some(""), Some(r"C:\Windows"), |p: &Path| {
            folded(&p.to_string_lossy()).ends_with("/npm.cmd")
        })
        .expect("an off-roster name refused, which only a roster member may do");
        assert_eq!(
            folded(&got),
            "c:/windows/system32/npm.cmd",
            "an off-roster name was denied the system directory, which is a rejection it never \
             earned and a resolution it needs"
        );
    }

    // spec: gate-sdk/SPEC.md §check-graph — an argv[0] the caller already resolved passes
    // through: `dispatch`'s and the floor probe's callers hand paths, not names
    #[test]
    fn a_resolved_path_passes_through_the_funnel_untouched() {
        let none: Vec<std::path::PathBuf> = Vec::new();
        assert_eq!(
            spawn_resolution("/opt/bin/bash", &none, Some(""), Some(r"C:\Windows"), |_: &Path| {
                false
            }),
            Ok("/opt/bin/bash".to_string())
        );
    }

    // spec: gate-sdk/SPEC.md §check-graph — the arm every host the battery runs on takes: no
    // system root, so nothing is rejected and the first match wins exactly as it did before
    #[test]
    fn a_posix_path_resolves_its_first_match_unchanged() {
        let dirs = vec![
            std::path::PathBuf::from("/nowhere"),
            std::path::PathBuf::from("/opt/bin"),
        ];
        assert_eq!(
            resolve_outside_system_dir("bash", &dirs, None, None, |p: &Path| p
                == Path::new("/opt/bin/bash")),
            Ok("/opt/bin/bash".to_string()),
            "the repair changed the resolution on a platform that has no system directory"
        );
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

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the shipped half of a module: a
    // `#[cfg(test)]`-gated item is dropped, so a bridge helper's own spawn is not a shipped one.
    // rustfmt's shape is the boundary — the gated item ends at a `}` in the attribute's column
    fn shipped_scope(text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut keep = vec![true; lines.len()];
        let mut i = 0usize;
        while i < lines.len() {
            if lines[i].trim() != "#[cfg(test)]" {
                i += 1;
                continue;
            }
            let indent = lines[i].len() - lines[i].trim_start().len();
            let closer = format!("{}}}", " ".repeat(indent));
            keep[i] = false;
            let mut j = i + 1;
            // spec: gate-sdk/SPEC.md §Fail-closed contract — the item's head line decides its
            // extent: one that closes on `;` with its braces balanced is the whole item, and any
            // other opens a body ending at the closer
            let single = lines.get(j).is_some_and(|h| {
                h.trim_end().ends_with(';')
                    && h.matches('{').count() == h.matches('}').count()
            });
            while j < lines.len() {
                keep[j] = false;
                if single || lines[j] == closer {
                    break;
                }
                j += 1;
            }
            i = j + 1;
        }
        lines
            .iter()
            .zip(keep)
            .map(|(l, k)| if k { *l } else { "" })
            .collect::<Vec<&str>>()
            .join("\n")
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the tracker is asserted rather than trusted:
    // a gated `mod` with no brace, a gated braced item, and shipped code on the far side of both
    #[test]
    fn the_shipped_scope_drops_a_cfg_test_item_and_keeps_what_follows() {
        let src = "#[cfg(test)]\nmod usage_tests;\nfn shipped() {}\n#[cfg(test)]\nmod tests {\n    fn hidden() {}\n}\nfn also_shipped() {}\n";
        let out = shipped_scope(src);
        assert!(!out.contains("usage_tests"), "a gated `mod x;` survived: {}", out);
        assert!(!out.contains("hidden"), "a gated `mod tests` body survived: {}", out);
        assert!(out.contains("fn shipped()"), "shipped code between two gates was dropped: {}", out);
        assert!(out.contains("also_shipped"), "shipped code after a gated body was dropped: {}", out);
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the routing half, in the roster shape
    // §check-reads-couples' unit test B uses for filesystem walks; that section owns the
    // corpus this scans and the one exception class it declares
    #[test]
    fn no_module_outside_proc_constructs_a_subprocess_itself() {
        walk::bridge_declared_knobs(&crate::knobenv::lock());
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let files = walk::find_files(&src, &["rs"]).expect("cannot enumerate the crate modules");
        assert!(!files.is_empty(), "no module found to scan");
        let root = std::fs::read_to_string(src.join("main.rs")).expect("cannot read main.rs");
        // spec: gate-sdk/SPEC.md §Fail-closed contract — a module `main.rs` declares under
        // `#[cfg(test)]` is test-only in whole and carries no file-level marker saying so, so
        // the declaration is the roster rather than a second list here
        let test_only: Vec<String> = root
            .lines()
            .collect::<Vec<&str>>()
            .windows(2)
            .filter(|w| w[0].trim() == "#[cfg(test)]")
            .filter_map(|w| {
                w[1].trim()
                    .strip_prefix("mod ")
                    .and_then(|m| m.strip_suffix(';'))
                    .map(|m| format!("{}.rs", m))
            })
            .collect();
        let mut offenders: Vec<String> = Vec::new();
        for f in &files {
            let leaf = f.file_name().unwrap_or_default().to_string_lossy().into_owned();
            if leaf == "proc.rs" || test_only.contains(&leaf) {
                continue;
            }
            let text = std::fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", f.display(), e));
            // spec: gate-sdk/SPEC.md §Fail-closed contract — the roster is the *code*
            // spellings a construction needs, never the bare word: a gate's own remedy text
            // may name `core.sshCommand`, and a detector that fires on prose gets muted
            let shipped = shipped_scope(&text);
            let lines: Vec<&str> = shipped.lines().collect();
            for (n, line) in lines.iter().enumerate() {
                // spec: gate-sdk/SPEC.md §Fail-closed contract — an import names the type and
                // constructs nothing, so `use …::CommandExt` is not a spawn site
                if line.trim_start().starts_with("use ") {
                    continue;
                }
                if !["Command::", "process::Command"].iter().any(|sp| line.contains(sp)) {
                    continue;
                }
                // spec: gate-sdk/SPEC.md §Fail-closed contract — the declared exception class,
                // on the tree's own valve convention: a named cause above the site, so the next
                // reader meets a decision rather than an oversight
                let window = &lines[n.saturating_sub(4)..n];
                if window.iter().any(|l| l.contains("spawn-funnel-exempt:")) {
                    continue;
                }
                offenders.push(format!("{}:{}", f.display(), n + 1));
            }
        }
        assert!(
            offenders.is_empty(),
            "a shipped module builds its own subprocess ({:?}) — `Ok` from `Command::output()` \
             means the spawn succeeded, never that the child did, so reading stdout there \
             reproduces the captured-emptiness false-green, and it goes around the Windows \
             resolution `proc::run*` applies. Route it through proc::run, widen proc.rs if the \
             call needs something proc::run does not carry, or declare the shape it cannot with \
             a `spawn-funnel-exempt:` cause above the site",
            offenders
        );
    }
}
