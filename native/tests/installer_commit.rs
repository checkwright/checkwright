// spec: installer/SPEC.md §init — the commit `init` and `uninstall` make, driven end to end: a scratch
// package carrying the binary built for this test run, and a scratch repository whose identity and
// every other setting sit in its own config, never the process environment
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_checkwright-gates");

struct Scratch {
    dir: PathBuf,
    bin: PathBuf,
    repo: PathBuf,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

// spec: installer/SPEC.md §Layout — a package is its `package.json`, its profile roster and a
// payload holding each kit beside the artifact directory the running binary sits in
fn scratch(case: &str) -> Scratch {
    let dir = std::env::temp_dir().join(format!("checkwright-installer-commit-{}-{}", std::process::id(), case));
    let _ = std::fs::remove_dir_all(&dir);
    let pkg = dir.join("pkg");
    let art = pkg.join("payload").join("artifact").join("t");
    let repo = dir.join("repo");
    for d in [&art, &pkg.join("payload").join("a-kit"), &repo, &dir.join("home")] {
        std::fs::create_dir_all(d).expect("cannot make the scratch tree");
    }
    std::fs::write(pkg.join("package.json"), "{\"name\":\"scratch\",\"version\":\"1.2.3\"}\n").expect("package.json");
    std::fs::write(pkg.join("profiles.list"), "starter\ta-kit\n").expect("profiles.list");
    std::fs::write(pkg.join("payload").join("a-kit").join("x.txt"), "x\n").expect("the kit's file");
    let bin = art.join(Path::new(BIN).file_name().expect("a binary name"));
    std::fs::copy(BIN, &bin).expect("cannot place the binary under test");
    let s = Scratch { dir, bin, repo };
    git(&s, &["init", "-q"]);
    git(&s, &["config", "user.name", "Scratch"]);
    git(&s, &["config", "user.email", "scratch@example.com"]);
    s
}

fn git(s: &Scratch, args: &[&str]) -> String {
    let out = Command::new("git").arg("-C").arg(&s.repo).args(args).output().expect("git did not spawn");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the run reads no knob, git setting or identity of
// the invoking environment
fn verb(s: &Scratch, bin: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let mut c = Command::new(bin);
    c.args(args).current_dir(&s.repo).stdin(Stdio::null());
    for (name, _) in std::env::vars_os() {
        let n = name.to_string_lossy();
        if n.starts_with("GATE_") || n.contains("_KIT_") || n.starts_with("GIT_") || n == "EMAIL" {
            c.env_remove(&name);
        }
    }
    c.env("HOME", s.dir.join("home"))
        .env("XDG_CONFIG_HOME", s.dir.join("home"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GATE_SDK_UPDATE_CHECK", "off")
        .env("GIT_CEILING_DIRECTORIES", &s.dir);
    let out = c.output().expect("cannot run the binary under test");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn hook(s: &Scratch, body: &str) {
    use std::os::unix::fs::PermissionsExt;
    let hooks = s.repo.join(".git").join("hooks");
    std::fs::create_dir_all(&hooks).expect("cannot make the hooks directory");
    let h = hooks.join("pre-commit");
    std::fs::write(&h, format!("#!/bin/sh\n{}\n", body)).expect("cannot write the hook");
    std::fs::set_permissions(&h, std::fs::Permissions::from_mode(0o755)).expect("cannot mark the hook executable");
}

fn staged(s: &Scratch) -> Vec<String> {
    git(s, &["diff", "--cached", "--name-only"]).lines().map(str::to_string).collect()
}

// spec: installer/SPEC.md §init — a refusing adopter pre-commit fails the `init` verb at exit 1 with
// the files staged, the hook named with its directory, and the follow-up block still printed
#[test]
fn a_refusing_pre_commit_fails_init_with_the_files_staged_and_the_hook_named() {
    let s = scratch("refused");
    hook(&s, "echo 'the hook speaks' >&2\nexit 1");
    let (code, out, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(1), "init did not exit 1: {}", err);
    let staged = staged(&s);
    assert!(staged.iter().any(|p| p == "a-kit/x.txt") && staged.iter().any(|p| p == "checkwright.lock"), "the install is not staged: {:?}", staged);
    assert!(err.contains("the hook speaks"), "git's account was not relayed: {}", err);
    let hooks = Path::new(".git").join("hooks").display().to_string();
    let named = err.lines().any(|l| {
        l.starts_with("checkwright init: the commit failed, and your pre-commit hook in ")
            && l.contains(&hooks)
            && l.ends_with("judged it — the files are staged")
    });
    assert!(named, "the refusal did not name the hook and its directory: {}", err);
    assert!(err.contains("The kit directories (a-kit) are a vendored dependency") && err.contains("--no-commit"), "{}", err);
    assert!(out.contains("\nnext:\n") && out.contains("\nthen: "), "the follow-up block did not print: {}", out);
}

// spec: installer/SPEC.md §init — with no identity git's strict read accepts, `init` refuses at exit 1
// before any write, after relaying git's account
#[test]
fn an_identity_less_init_refuses_before_any_write() {
    let s = scratch("noident-init");
    git(&s, &["config", "user.name", ""]);
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(1), "init did not exit 1: {}", err);
    assert!(!s.repo.join("checkwright.lock").exists(), "the manifest was written");
    assert!(staged(&s).is_empty(), "files were staged");
    assert_eq!(git(&s, &["status", "--porcelain", "--untracked-files=all"]), "", "the worktree was written");
    assert!(err.contains("fatal: "), "git's account was not relayed: {}", err);
    assert!(err.contains("checkwright init: git has no identity to make the commit with\n"), "{}", err);
    assert!(err.contains("help: set user.name and user.email") && err.contains("--no-commit to stage the vendoring"), "{}", err);
}

// spec: installer/SPEC.md §uninstall — the identity precondition refuses before anything is removed
#[test]
fn an_identity_less_uninstall_refuses_before_any_removal() {
    let s = scratch("noident-uninstall");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    git(&s, &["config", "user.name", ""]);
    let (code, _, err) = verb(&s, &s.bin, &["--uninstall"]);
    assert_eq!(code, Some(1), "uninstall did not exit 1: {}", err);
    assert!(s.repo.join("checkwright.lock").is_file() && s.repo.join("a-kit").join("x.txt").is_file(), "files were removed");
    assert_eq!(git(&s, &["status", "--porcelain", "--untracked-files=all"]), "", "the worktree was written");
    assert!(err.contains("checkwright uninstall: git has no identity to make the commit with\n"), "{}", err);
    assert!(err.contains("--no-commit to stage the removal"), "{}", err);
}

// spec: installer/SPEC.md §uninstall — a commit failing after the hooks directory's removal still
// reports the line restoring `core.hooksPath`, read before the chain record went
#[test]
fn a_failed_uninstall_commit_still_prints_the_restore_line() {
    let s = scratch("restore");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    let placed = s.repo.join("scripts").join(s.bin.file_name().expect("a binary name"));
    let (code, _, err) = verb(&s, &placed, &["--install-hooks"]);
    assert_eq!(code, Some(0), "the hooks were not placed: {}", err);
    git(&s, &["config", "commit.gpgSign", "true"]);
    git(&s, &["config", "gpg.program", "false"]);
    let (code, out, err) = verb(&s, &s.bin, &["--uninstall"]);
    assert_eq!(code, Some(1), "uninstall did not exit 1: {}", err);
    assert!(err.contains("checkwright uninstall: the commit failed"), "{}", err);
    assert!(
        out.contains("Undo that yourself with:\n  git config --unset core.hooksPath\n"),
        "the restore line was lost: {}",
        out
    );
}
