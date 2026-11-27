// spec: gate-sdk/SPEC.md §check-crate-arms — every child an integration test starts is built here,
// and reads no host git configuration, as `proc`'s test build pins it; a caller's own value wins
#![allow(dead_code)]

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

static GLOBAL: OnceLock<PathBuf> = OnceLock::new();

pub fn command(program: impl AsRef<OsStr>) -> Command {
    let mut c = Command::new(program);
    pin(&mut c);
    c
}

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — no knob or git setting of the invoking environment
// reaches the child
pub fn scrubbed(program: impl AsRef<OsStr>) -> Command {
    stripped(program, |n| n.starts_with("GATE_") || n.contains("_KIT_") || n.starts_with("GIT_"))
}

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — a setup or assertion read aims at the scratch
// repository alone, never at the one an exported `GIT_DIR` or `GIT_WORK_TREE` names
pub fn without_git_env(program: impl AsRef<OsStr>) -> Command {
    stripped(program, |n| n.starts_with("GIT_"))
}

// spec: gate-sdk/SPEC.md §check-crate-arms — the pin lands after the strip, so an inherited
// `GIT_CONFIG_GLOBAL` cannot remove it
fn stripped(program: impl AsRef<OsStr>, drop: impl Fn(&str) -> bool) -> Command {
    let mut c = Command::new(program);
    for (name, _) in std::env::vars_os() {
        if drop(&name.to_string_lossy()) {
            c.env_remove(&name);
        }
    }
    pin(&mut c);
    c
}

// spec: gate-sdk/SPEC.md §run-gates — a scratch a `--run` reaches is a git repository, since the
// arm refuses outside one
pub fn git_init(dir: &std::path::Path) {
    let ok = without_git_env("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .status()
        .is_ok_and(|s| s.success());
    assert!(ok, "git init failed in {}", dir.display());
}

// spec: gate-sdk/SPEC.md §run-gates — the tests' own private temp dir, made mode 700 and
// owner-checked before use as the binary's is; one that is not private stops the test run
fn private_dir() -> std::path::PathBuf {
    #[cfg(unix)]
    let uid = unsafe { libc::geteuid() };
    #[cfg(not(unix))]
    let uid = 0u32;
    let dir = std::env::temp_dir().join(format!("checkwright-test-{}", uid));
    #[cfg(unix)]
    let _ = std::os::unix::fs::DirBuilderExt::mode(&mut std::fs::DirBuilder::new(), 0o700).create(&dir);
    #[cfg(not(unix))]
    let _ = std::fs::DirBuilder::new().create(&dir);
    let meta = std::fs::symlink_metadata(&dir).unwrap_or_else(|e| panic!("cannot read {}: {}", dir.display(), e));
    assert!(meta.is_dir(), "{} is not a directory, so the tests write nothing there", dir.display());
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        assert!(
            meta.uid() == uid && meta.mode() & 0o077 == 0,
            "{} is not private to this user, so the tests write nothing there",
            dir.display()
        );
    }
    dir
}

// spec: gate-sdk/SPEC.md §check-crate-arms — the empty global file, and no system file off native
// Windows
fn pin(c: &mut Command) {
    let global = GLOBAL.get_or_init(|| {
        let p = private_dir().join("empty.gitconfig");
        let _ = std::fs::File::create(&p);
        p
    });
    c.env("GIT_CONFIG_GLOBAL", global);
    if !cfg!(windows) {
        c.env("GIT_CONFIG_NOSYSTEM", "1");
    }
}
