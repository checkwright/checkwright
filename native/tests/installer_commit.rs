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

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — a setup or assertion read aims at the scratch
// repository alone, never at the one an exported `GIT_DIR` or `GIT_WORK_TREE` names
fn unset_git_env(c: &mut Command) {
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            c.env_remove(&name);
        }
    }
}

fn git(s: &Scratch, args: &[&str]) -> String {
    let mut c = Command::new("git");
    unset_git_env(&mut c);
    let out = c.arg("-C").arg(&s.repo).args(args).output().expect("git did not spawn");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the run reads no knob, git setting or identity of
// the invoking environment
fn verb(s: &Scratch, bin: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    verb_with(s, bin, args, &[])
}

// spec: installer/SPEC.md §init — `verb` with variables the case itself exports, set after the scrub
fn verb_with(s: &Scratch, bin: &Path, args: &[&str], env: &[(&str, &str)]) -> (Option<i32>, String, String) {
    let mut c = Command::new(bin);
    c.args(args).current_dir(&s.repo).stdin(Stdio::null());
    unset_git_env(&mut c);
    for (name, _) in std::env::vars_os() {
        let n = name.to_string_lossy();
        if n.starts_with("GATE_") || n.contains("_KIT_") || n == "EMAIL" {
            c.env_remove(&name);
        }
    }
    c.env("HOME", s.dir.join("home"))
        .env("XDG_CONFIG_HOME", s.dir.join("home"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GATE_SDK_UPDATE_CHECK", "off")
        .env("GIT_CEILING_DIRECTORIES", &s.dir);
    for (k, v) in env {
        c.env(k, v);
    }
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

// spec: installer/SPEC.md §uninstall — the clone opted in to the placed hooks, `core.hooksPath` naming
// the directory `--install-hooks` made; a value set beforehand is recorded for the restore line
fn opted_in(s: &Scratch) {
    let placed = s.repo.join("scripts").join(s.bin.file_name().expect("a binary name"));
    let (code, _, err) = verb(s, &placed, &["--install-hooks"]);
    assert_eq!(code, Some(0), "the hooks were not placed: {}", err);
}

// spec: installer/SPEC.md §uninstall — a signing program that refuses fails the removal's commit
// after the files and the hooks directory are gone
fn refuse_signing(s: &Scratch) {
    git(s, &["config", "commit.gpgSign", "true"]);
    git(s, &["config", "gpg.program", "false"]);
}

const LEFT: &str = "this clone's core.hooksPath still names the hooks directory that was just removed.";

// spec: installer/SPEC.md §uninstall — a commit failing after the hooks directory's removal still
// reports the line restoring `core.hooksPath`, read before the chain record went
#[test]
fn a_failed_uninstall_commit_still_prints_the_restore_line() {
    let s = scratch("restore");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    opted_in(&s);
    refuse_signing(&s);
    let (code, out, err) = verb(&s, &s.bin, &["--uninstall"]);
    assert_eq!(code, Some(1), "uninstall did not exit 1: {}", err);
    assert!(err.contains("checkwright uninstall: the commit failed"), "{}", err);
    assert!(
        out.contains("Undo that yourself with:\n  git config --unset core.hooksPath\n"),
        "the restore line was lost: {}",
        out
    );
}

// spec: installer/SPEC.md §uninstall — the line restores the value the chain record kept where one
// was set before the opt-in, and a commit failing after the removal prints it
#[test]
fn a_failed_uninstall_commit_restores_the_value_the_record_kept() {
    let s = scratch("restore-prior");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    std::fs::create_dir_all(s.repo.join(".husky")).expect("the prior hooks directory");
    std::fs::write(s.repo.join(".husky").join("README"), "theirs\n").expect("a file in it");
    git(&s, &["add", ".husky"]);
    git(&s, &["commit", "-q", "-m", "theirs"]);
    git(&s, &["config", "core.hooksPath", ".husky"]);
    opted_in(&s);
    refuse_signing(&s);
    let (code, out, err) = verb(&s, &s.bin, &["--uninstall"]);
    assert_eq!(code, Some(1), "uninstall did not exit 1: {}", err);
    assert!(
        out.contains("Undo that yourself with:\n  git config core.hooksPath '.husky'\n"),
        "the prior value was lost: {}",
        out
    );
}

// spec: installer/SPEC.md §uninstall — a chain record that cannot be read is reported with its own
// sentence, no command advised, and a commit failing after the removal still prints it
#[test]
fn a_failed_uninstall_commit_reports_a_record_it_could_not_read() {
    let s = scratch("restore-unread");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    opted_in(&s);
    let record = s.repo.join(".git").join("gate-hooks").join("chain");
    std::fs::write(&record, "not a record\n").expect("cannot spoil the record");
    refuse_signing(&s);
    let (code, out, err) = verb(&s, &s.bin, &["--uninstall"]);
    assert_eq!(code, Some(1), "uninstall did not exit 1: {}", err);
    assert!(
        out.contains(LEFT) && out.contains("cannot read the chain record") && out.contains("no command is advised"),
        "the unread record was not reported: {}",
        out
    );
    assert!(!out.contains("Undo that yourself with"), "a command was advised over an unknown value: {}", out);
}

// spec: installer/SPEC.md §uninstall — a refusal to stage the removal is an exit after the hooks
// directory's removal, so it prints the restore line too
#[test]
fn a_removal_that_cannot_be_staged_still_prints_the_restore_line() {
    let s = scratch("restore-stage");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    opted_in(&s);
    std::fs::write(s.repo.join(".git").join("index.lock"), "").expect("cannot hold the index");
    let (code, out, err) = verb(&s, &s.bin, &["--uninstall"]);
    assert_eq!(code, Some(2), "uninstall did not exit 2: {}", err);
    assert!(err.contains("could not stage the removal"), "{}", err);
    assert!(out.contains(LEFT) && out.contains("git config --unset core.hooksPath"), "the restore line was lost: {}", out);
}

// spec: installer/SPEC.md §init — the commit needs the author and the committer both, so a read
// that fails for either half alone refuses, though the other half is exported
const AUTHOR: [(&str, &str); 2] = [("GIT_AUTHOR_NAME", "Author"), ("GIT_AUTHOR_EMAIL", "author@example.com")];
const COMMITTER: [(&str, &str); 2] = [("GIT_COMMITTER_NAME", "Committer"), ("GIT_COMMITTER_EMAIL", "committer@example.com")];

fn refused_with_only(case: &str, exported: &[(&str, &str)]) {
    let s = scratch(case);
    git(&s, &["config", "user.name", ""]);
    let (code, _, err) = verb_with(&s, &s.bin, &["--init"], exported);
    assert_eq!(code, Some(1), "init did not exit 1: {}", err);
    assert!(err.contains("checkwright init: git has no identity to make the commit with\n"), "{}", err);
    assert!(!s.repo.join("checkwright.lock").exists(), "the manifest was written");
}

// spec: installer/SPEC.md §init — an author alone is no identity to commit with
#[test]
fn an_author_identity_alone_refuses() {
    refused_with_only("author-alone", &AUTHOR);
}

// spec: installer/SPEC.md §init — a committer alone is no identity to commit with
#[test]
fn a_committer_identity_alone_refuses() {
    refused_with_only("committer-alone", &COMMITTER);
}

// spec: installer/SPEC.md §init — both halves exported pass with no configured name, so the two
// refusals above are of the half missing and not of the empty config
#[test]
fn both_exported_halves_pass_the_precondition() {
    let s = scratch("both-halves");
    git(&s, &["config", "user.name", ""]);
    let both: Vec<(&str, &str)> = AUTHOR.iter().chain(COMMITTER.iter()).copied().collect();
    let (code, _, err) = verb_with(&s, &s.bin, &["--init"], &both);
    assert_eq!(code, Some(0), "init did not exit 0: {}", err);
}

// spec: installer/SPEC.md §init — the identity precondition is waived where the clean-worktree check
// is, by `--no-commit`, a personal install and `--dry-run`, since it matters only where a commit is
// made
fn waived(case: &str, flag: &str) -> (Scratch, String, String) {
    let s = scratch(case);
    git(&s, &["config", "user.name", ""]);
    let (code, out, err) = verb(&s, &s.bin, &["--init", flag]);
    assert_eq!(code, Some(0), "init {} did not exit 0: {}", flag, err);
    assert!(!err.contains("no identity"), "{} refused for want of an identity: {}", flag, err);
    (s, out, err)
}

#[test]
fn no_commit_runs_with_no_identity_and_stages_the_vendoring() {
    let (s, _, _) = waived("waive-no-commit", "--no-commit");
    assert!(staged(&s).iter().any(|p| p == "checkwright.lock"), "the vendoring was not staged: {:?}", staged(&s));
}

#[test]
fn a_personal_install_runs_with_no_identity_and_commits_nothing() {
    let (s, _, _) = waived("waive-personal", "--personal");
    assert!(s.repo.join("checkwright.lock").is_file(), "the install was not made");
    assert!(staged(&s).is_empty(), "a personal install staged files: {:?}", staged(&s));
}

#[test]
fn a_dry_run_runs_with_no_identity_and_writes_nothing() {
    let (s, _, _) = waived("waive-dry-run", "--dry-run");
    assert!(!s.repo.join("checkwright.lock").exists(), "a dry run wrote the manifest");
}

// spec: installer/SPEC.md §init — the precondition runs before the run knows whether anything will
// change, so a re-run with nothing to change refuses too
#[test]
fn an_identity_less_rerun_with_nothing_to_change_refuses() {
    let s = scratch("noident-nothing");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    git(&s, &["config", "user.name", ""]);
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(1), "the re-run did not exit 1: {}", err);
    assert!(err.contains("checkwright init: git has no identity to make the commit with\n"), "{}", err);
}

// spec: installer/SPEC.md §update — update is init with one added precondition, so the identity
// refusal reaches it under init's own prefix
#[test]
fn an_identity_less_update_refuses_under_inits_prefix() {
    let s = scratch("noident-update");
    let (code, _, err) = verb(&s, &s.bin, &["--init"]);
    assert_eq!(code, Some(0), "the install failed: {}", err);
    git(&s, &["config", "user.name", ""]);
    let (code, _, err) = verb(&s, &s.bin, &["--update"]);
    assert_eq!(code, Some(1), "update did not exit 1: {}", err);
    assert!(err.contains("checkwright init: git has no identity to make the commit with\n"), "{}", err);
    assert!(err.contains("--no-commit to stage the vendoring"), "{}", err);
}
