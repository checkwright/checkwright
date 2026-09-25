// spec: guard-kit/SPEC.md §scratch-run — the echo-then-exec runner for scratch scripts: the body
// is printed before it runs, so an allowlisted execution documents itself in the transcript.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and `Arm::Run`, both forced rather
// than chosen: the runner reads `GATE_SDK_TMP_DIR`, and it passes the child's exit code through
// verbatim while its stdout must reach the terminal as the child produces it.
use crate::programs::{self, Program};

pub const KNOBS: &[&str] = &["GATE_SDK_TMP_DIR", "GUARD_KIT_SCRATCH_POWERSHELL"];

const NAME: &str = "scratch-run";
const USAGE: &str = "usage: --scratch-run <script> [args…]";
const HOST_KNOB: &str = "GUARD_KIT_SCRATCH_POWERSHELL";

// spec: guard-kit/SPEC.md §scratch-run — each flag carries its reason there: no profile, no prompt,
// the process-scope policy, no banner.
const HOST_FLAGS: &[&str] = &["-NoLogo", "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"];

// spec: guard-kit/SPEC.md §scratch-run — the runner-side half of the interpreter set, read off the
// file's own shebang rather than a roster: the runner has the *file*, so it reads what the file
// states. The `/usr/bin/env <interp>` spelling resolves to the same answer as the direct one.
pub fn shebang_interpreter(first_line: &str) -> Option<String> {
    let rest = first_line.strip_prefix("#!")?;
    let mut words = rest.split_whitespace();
    let first = words.next().unwrap_or_default();
    let base = |w: &str| w.rsplit('/').next().unwrap_or(w).to_string();
    if base(first) == "env" {
        return Some(base(words.next().unwrap_or_default()));
    }
    Some(base(first))
}

// spec: guard-kit/SPEC.md §scratch-run — a bash body's interpreter set; the empty interpreter is
// admitted for the same reason a shebang-less target is: nothing states an interpreter, so nothing
// contradicts the rule.
pub fn bash_family(interpreter: &str) -> bool {
    matches!(interpreter, "bash" | "sh" | "")
}

// spec: guard-kit/SPEC.md §scratch-run — a PowerShell body's interpreter set, empty admitted on
// `bash_family`'s ground.
pub fn powershell_family(interpreter: &str) -> bool {
    matches!(interpreter, "pwsh" | "powershell" | "")
}

// spec: guard-kit/SPEC.md §scratch-run — the extension decides the body's shell, `.ps1` matched
// without regard to ASCII case.
pub fn is_powershell_body(target: &str) -> bool {
    let b = target.as_bytes();
    b.len() >= 4 && b[b.len() - 4..].eq_ignore_ascii_case(b".ps1")
}

// spec: guard-kit/SPEC.md §scratch-run — the containment test reads the *resolved* path, never the
// spelling, so a symlink out of the scratch dir is refused where a lexical `..`-normalizing compare
// would pass it. Both sides come from `walk::canonicalize`, so the compare is within one dialect.
pub fn is_inside(root: &str, dir: &str) -> bool {
    crate::walk::at_or_under(root, dir)
}

// spec: guard-kit/SPEC.md §scratch-run — the snapshot's name: `.`-led so a listing does not show it
// as the target, the target's stem, the run's token, and the target's extension kept.
pub fn snapshot_name(target_file: &str, token: &str) -> String {
    let p = std::path::Path::new(target_file);
    let stem = p.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    match p.extension() {
        Some(ext) => format!(".{}.{}.{}", stem, token, ext.to_string_lossy()),
        None => format!(".{}.{}", stem, token),
    }
}

fn refuse(message: &str) -> i32 {
    eprintln!("{}: {}", NAME, message);
    2
}

// spec: guard-kit/SPEC.md §scratch-run — created exclusively and never opened over an existing
// path, owner read and write only on unix; a name another file holds takes the next token.
fn write_snapshot(dir: &std::path::Path, target_file: &str, body: &[u8]) -> Result<std::path::PathBuf, String> {
    use std::io::Write;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let mut last = String::new();
    for attempt in 0..16u32 {
        let token = format!("run-{}-{}-{}", std::process::id(), nanos, attempt);
        let path = crate::walk::child(dir, &snapshot_name(target_file, &token));
        let mut open = std::fs::OpenOptions::new();
        open.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            open.mode(0o600);
        }
        match open.open(&path) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(body).and_then(|_| f.flush()) {
                    let _ = std::fs::remove_file(&path);
                    return Err(format!("cannot write a snapshot in {}: {}", dir.display(), e));
                }
                return Ok(path);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => last = e.to_string(),
            Err(e) => return Err(format!("cannot create a snapshot in {}: {}", dir.display(), e)),
        }
    }
    Err(format!("cannot create a snapshot in {}: {}", dir.display(), last))
}

// spec: guard-kit/SPEC.md §scratch-run — the interpreter the body's file states, checked against
// the extension and the knob; every answer here lands before the echo.
fn interpreter_for(target: &str, text: &str, host: &str) -> Result<Program, String> {
    let stated = text.lines().next().and_then(shebang_interpreter);
    if !is_powershell_body(target) {
        return match stated {
            Some(i) if powershell_family(&i) && !i.is_empty() => Err(format!(
                "refusing {} — its shebang names '{}', and a PowerShell body is named .ps1 \
                 (guard-kit/SPEC.md §scratch-run). Rename it with the .ps1 extension.",
                target, i
            )),
            Some(i) if !bash_family(&i) => Err(format!(
                "refusing {} — scratch execution runs in bash, or in PowerShell where the project \
                 names a host (guard-kit/SPEC.md §scratch-run), and its shebang names '{}'. Rewrite \
                 the body as a shell script, or do the work in a language the control covers.",
                target, i
            )),
            _ => Ok(programs::BASH),
        };
    }
    if let Some(i) = stated.filter(|i| !powershell_family(i)) {
        return Err(format!(
            "refusing {} — a .ps1 is a PowerShell body and its shebang names '{}' \
             (guard-kit/SPEC.md §scratch-run). Drop the shebang, or name the body for its shell.",
            target, i
        ));
    }
    if host.is_empty() {
        return Err(format!(
            "refusing {} — the PowerShell scratch path is off in this project ({} is empty, \
             guard-kit/SPEC.md §scratch-run). Write the body as a bash script under the scratch dir \
             and run it through this runner.",
            target, HOST_KNOB
        ));
    }
    let program = Program::consumer("GUARD_KIT_SCRATCH_POWERSHELL", host);
    let found = if host.chars().any(std::path::is_separator) {
        std::path::Path::new(host).is_file()
    } else {
        crate::proc::on_path(&program)
    };
    if !found {
        return Err(format!(
            "refusing {} — {} names '{}', which does not resolve to a PowerShell host here",
            target, HOST_KNOB, host
        ));
    }
    Ok(program)
}

// spec: guard-kit/SPEC.md §scratch-run — the refusal order is the contract: every refusal fires
// *before* the echo, so a refused run prints no body and stays distinguishable from a child that
// exited 2 after its body was printed.
pub fn run(args: &[String]) -> i32 {
    let Some(target) = args.first() else {
        return refuse(USAGE);
    };
    let scratch = match crate::walk::knob_scalar("GATE_SDK_TMP_DIR") {
        Ok(s) => s,
        Err(e) => return refuse(&e),
    };
    let host = match crate::walk::knob_scalar(HOST_KNOB) {
        Ok(s) => s,
        Err(e) => return refuse(&e),
    };
    let Some(scratch_abs) = crate::walk::canonicalize(&scratch) else {
        return refuse(&format!(
            "no scratch dir at {} (GATE_SDK_TMP_DIR)",
            scratch
        ));
    };
    let path = std::path::Path::new(target);
    if !path.is_file() {
        return refuse(&format!("no such script: {}", target));
    }
    // spec: guard-kit/SPEC.md §scratch-run — the leaf is rejoined after the crossing rather than
    // canonicalized with it, which is the shell's `cd "$(dirname)" && pwd -P` exactly: a symlinked
    // *directory* resolves, and the target's own name is compared where it is spelled.
    let parent = match path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
        _ => std::path::PathBuf::from("."),
    };
    let Some(parent_abs) = crate::walk::canonicalize(&parent) else {
        return refuse(&format!("no such script: {}", target));
    };
    if !is_inside(&scratch_abs, &parent_abs) {
        return refuse(&format!(
            "refusing {} — outside the scratch dir {}",
            target, scratch_abs
        ));
    }
    let body = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => return refuse(&format!("cannot read {}: {}", target, e)),
    };
    let program = match interpreter_for(target, &String::from_utf8_lossy(&body), &host) {
        Ok(p) => p,
        Err(m) => return refuse(&m),
    };
    let file = path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
    // spec: guard-kit/SPEC.md §scratch-run — the snapshot is written before the echo, so a
    // snapshot the runner cannot make is a refusal that printed no body.
    let snapshot = match write_snapshot(&parent, &file, &body) {
        Ok(p) => p,
        Err(e) => return refuse(&e),
    };
    let snap = snapshot.display().to_string();
    // spec: guard-kit/SPEC.md §scratch-run — the echo is the compensating control, so it is flushed
    // before the child is spawned: an unflushed buffer would let the child's own output overtake
    // the body it is supposed to follow, which is the ordering the whole control rests on.
    use std::io::Write;
    let mut out = std::io::stdout();
    let _ = writeln!(out, "=== {}: {} ===", NAME, target);
    let _ = out.write_all(&body);
    let _ = writeln!(out, "=== {}: executing {} as {} ===", NAME, target, snap);
    let _ = out.flush();
    let mut argv: Vec<&str> = Vec::new();
    if is_powershell_body(target) {
        argv.extend_from_slice(HOST_FLAGS);
    }
    argv.push(snap.as_str());
    argv.extend(args[1..].iter().map(String::as_str));
    let code = match crate::proc::run_to(&program, &argv, &crate::proc::Sink::Inherit) {
        Ok(code) => code,
        Err(e) => refuse(&e),
    };
    // spec: guard-kit/SPEC.md §scratch-run — the child's code is the contract, so a failed removal
    // is reported and changes nothing.
    if let Err(e) = std::fs::remove_file(&snapshot) {
        eprintln!("{}: could not remove the snapshot {}: {}", NAME, snap, e);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §scratch-run — the shebang classifier across its cases, the
    // `/usr/bin/env` indirection resolving to the same answer as the direct spelling.
    #[test]
    fn the_shebang_classifier_resolves_the_interpreter_word() {
        assert_eq!(shebang_interpreter("#!/bin/bash").as_deref(), Some("bash"));
        assert_eq!(shebang_interpreter("#!/bin/sh").as_deref(), Some("sh"));
        assert_eq!(
            shebang_interpreter("#!/usr/bin/env bash").as_deref(),
            Some("bash")
        );
        assert_eq!(
            shebang_interpreter("#!/usr/bin/env python3").as_deref(),
            Some("python3")
        );
        assert_eq!(shebang_interpreter("#!  /usr/bin/perl -w").as_deref(), Some("perl"));
        assert_eq!(shebang_interpreter("#!/usr/bin/env pwsh").as_deref(), Some("pwsh"));
        assert_eq!(
            shebang_interpreter("echo no shebang here"),
            None,
            "a target with no shebang states no interpreter, so nothing contradicts the rule"
        );
    }

    // spec: guard-kit/SPEC.md §scratch-run — each shell's set, and the two spellings that are not a
    // widening: `sh` is the shell family and an empty interpreter states nothing.
    #[test]
    fn each_body_admits_its_own_shell_and_a_stated_nothing() {
        assert!(bash_family("bash") && bash_family("sh") && bash_family(""));
        assert!(powershell_family("pwsh") && powershell_family("powershell") && powershell_family(""));
        for other in ["python3", "perl", "node", "ruby", "env"] {
            assert!(!bash_family(other), "{} was admitted", other);
            assert!(!powershell_family(other), "{} was admitted", other);
        }
        assert!(!bash_family("pwsh") && !powershell_family("bash"));
    }

    // spec: guard-kit/SPEC.md §scratch-run — the extension decides, without regard to ASCII case.
    #[test]
    fn the_extension_decides_the_body_s_shell() {
        assert!(is_powershell_body(".tmp/x.ps1") && is_powershell_body("X.PS1") && is_powershell_body("a.Ps1"));
        assert!(!is_powershell_body(".tmp/x.sh") && !is_powershell_body("ps1") && !is_powershell_body("x.ps12"));
    }

    // spec: guard-kit/SPEC.md §scratch-run — the snapshot keeps the extension and hides behind a dot.
    #[test]
    fn the_snapshot_name_keeps_the_extension_behind_a_dot() {
        assert_eq!(snapshot_name("probe.sh", "t"), ".probe.t.sh");
        assert_eq!(snapshot_name("x.ps1", "t"), ".x.t.ps1");
        assert_eq!(snapshot_name("plain", "t"), ".plain.t");
    }

    // spec: guard-kit/SPEC.md §scratch-run — the refusals the interpreter read owns, each naming its
    // corrective; the host checks run only for a PowerShell body.
    #[test]
    fn the_interpreter_read_refuses_each_contradiction() {
        let err = |t: &str, b: &str, h: &str| interpreter_for(t, b, h).err().unwrap_or_default();
        assert!(err("x.sh", "#!/usr/bin/env pwsh\n", "").contains(".ps1"));
        assert!(err("x.py", "#!/usr/bin/env python3\n", "pwsh").contains("runs in bash"));
        assert!(err("x.ps1", "#!/bin/bash\n", "pwsh").contains("PowerShell body"));
        assert!(err("x.ps1", "Write-Output 1\n", "").contains(HOST_KNOB));
        assert!(err("x.ps1", "Write-Output 1\n", "/no/such/dir/pwsh").contains("does not resolve"));
        assert!(interpreter_for("x.sh", "echo\n", "").is_ok());
        assert!(interpreter_for("x.sh", "#!/bin/sh\necho\n", "/no/such/pwsh").is_ok());
    }

    // spec: guard-kit/SPEC.md §scratch-run — the containment predicate over two already-resolved
    // answers: a sibling sharing the root's *prefix* is outside, and so is a path above the root.
    #[test]
    fn containment_is_a_boundary_test_rather_than_a_string_prefix() {
        assert!(is_inside("/a/tmp", "/a/tmp"));
        assert!(is_inside("/a/tmp", "/a/tmp/sub"));
        assert!(!is_inside("/a/tmp", "/a/tmpx"), "a prefix-sharing sibling is outside");
        assert!(!is_inside("/a/tmp", "/a"), "the parent of the root is outside");
        assert!(!is_inside("/a/tmp", "/b/tmp"));
    }
}
