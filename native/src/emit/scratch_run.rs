// spec: guard-kit/SPEC.md §scratch-run — the echo-then-exec runner for scratch scripts: the body
// is printed before it runs, so an allowlisted execution documents itself in the transcript.
// spec: gate-sdk/SPEC.md §The non-gate arm — bridged and `Arm::Run`, both forced rather than
// chosen: the runner resolves `GATE_SDK_TMP_DIR`, and it passes the child's exit code through
// verbatim while its stdout must reach the terminal as the child produces it.

pub const KNOBS: &[&str] = &["GATE_SDK_TMP_DIR"];

const NAME: &str = "scratch-run";
const USAGE: &str = "usage: --scratch-run <script> [args…]";

// spec: guard-kit/SPEC.md §scratch-run — the bash-only rule's runner-side half, read off the file's
// own shebang rather than a roster: the runner has the *file*, so it reads what the file states.
// The `/usr/bin/env <interp>` spelling resolves to the same answer as the direct one.
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

// spec: guard-kit/SPEC.md §scratch-run — scratch execution is bash-only, and the empty interpreter
// is admitted for the same reason a shebang-less target is: nothing states an interpreter, so
// nothing contradicts the rule.
pub fn bash_family(interpreter: &str) -> bool {
    matches!(interpreter, "bash" | "sh" | "")
}

// spec: guard-kit/SPEC.md §scratch-run — the containment test reads the *resolved* path, never the
// spelling, so a symlink out of the scratch dir is refused where a lexical `..`-normalizing compare
// would pass it. Both sides come from `walk::canonicalize`, so the compare is within one dialect.
pub fn is_inside(root: &str, dir: &str) -> bool {
    dir == root || dir.starts_with(&format!("{}/", root))
}

fn refuse(message: &str) -> i32 {
    eprintln!("{}: {}", NAME, message);
    2
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
    let text = String::from_utf8_lossy(&body);
    if let Some(interp) = text.lines().next().and_then(shebang_interpreter) {
        if !bash_family(&interp) {
            return refuse(&format!(
                "refusing {} — scratch execution is bash-only (guard-kit/SPEC.md §scratch-run) and \
                 its shebang names '{}'. Rewrite the body as a shell script, or do the work in a \
                 language the control covers.",
                target, interp
            ));
        }
    }
    // spec: guard-kit/SPEC.md §scratch-run — the echo is the compensating control, so it is flushed
    // before the child is spawned: an unflushed buffer would let the child's own output overtake
    // the body it is supposed to follow, which is the ordering the whole control rests on.
    use std::io::Write;
    let mut out = std::io::stdout();
    let _ = writeln!(out, "=== {}: {} ===", NAME, target);
    let _ = out.write_all(&body);
    let _ = writeln!(out, "=== {}: executing {} ===", NAME, target);
    let _ = out.flush();
    // spec: guard-kit/SPEC.md §scratch-run — the hardcoded interpreter is the bash-only rule, not an
    // unexamined default: widening it would convert a grant for "run bash on a reviewed body" into
    // one for "run anything on a reviewed body" with no settings edit, which is refused outright.
    let mut argv: Vec<&str> = vec![target.as_str()];
    argv.extend(args[1..].iter().map(String::as_str));
    match crate::proc::run_to("bash", &argv, &crate::proc::Sink::Inherit) {
        Ok(code) => code,
        Err(e) => refuse(&e),
    }
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
        assert_eq!(
            shebang_interpreter("echo no shebang here"),
            None,
            "a target with no shebang states no interpreter, so nothing contradicts the rule"
        );
    }

    // spec: guard-kit/SPEC.md §scratch-run — bash-only, and the two spellings that are not a
    // widening: `sh` is the shell family and an empty interpreter states nothing.
    #[test]
    fn only_the_shell_family_and_a_stated_nothing_are_admitted() {
        assert!(bash_family("bash") && bash_family("sh") && bash_family(""));
        for other in ["python3", "perl", "node", "ruby", "env"] {
            assert!(!bash_family(other), "{} was admitted", other);
        }
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
