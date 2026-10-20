// spec: guard-kit/SPEC.md §The rule roster — the link test the scratch grants take, rule
// `bounded_write`'s bounds clause (e), held where the host makes a link: a write through one takes
// no grant, and the same write to a plain scratch target is granted

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_checkwright-gates");

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn git(root: &Path, args: &[&str]) {
    let done = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir())
        .output()
        .expect("cannot run git");
    assert!(done.status.success(), "git {:?}: {}", args, String::from_utf8_lossy(&done.stderr));
}

// spec: guard-kit/SPEC.md §Testing — a repository holding a gitignored `.tmp/`, a tracked file and a
// tracked directory, and the committed allow entry a recorded launch needs
fn scratch() -> Scratch {
    let dir = std::env::temp_dir().join(format!("checkwright-grant-link-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".tmp/real")).expect("cannot create the scratch dir");
    std::fs::create_dir_all(dir.join("sub")).expect("cannot create the tracked dir");
    std::fs::create_dir_all(dir.join(".claude")).expect("cannot create the settings dir");
    std::fs::write(dir.join(".gitignore"), ".tmp/\nfriction.log\n").expect("ignore file");
    std::fs::write(dir.join("tracked.md"), "tracked\n").expect("tracked file");
    std::fs::write(dir.join("sub/kept.md"), "tracked\n").expect("tracked dir");
    std::fs::write(dir.join(".claude/settings.json"), "{ \"permissions\": { \"allow\": [\"Bash(git status)\"] } }\n")
        .expect("settings");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", ".gitignore", "tracked.md", "sub/kept.md"]);
    Scratch(dir)
}

#[cfg(unix)]
fn link(target: &Path, at: &Path, _dir: bool) -> bool {
    std::os::unix::fs::symlink(target, at).is_ok()
}

#[cfg(windows)]
fn link(target: &Path, at: &Path, dir: bool) -> bool {
    if dir {
        std::os::windows::fs::symlink_dir(target, at).is_ok()
    } else {
        std::os::windows::fs::symlink_file(target, at).is_ok()
    }
}

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the case reads no knob of the invoking environment
fn granted(root: &Path, command: &str) -> bool {
    let payload = serde_json::json!({"tool_name": "Bash", "tool_input": {"command": command}}).to_string();
    let mut c = Command::new(BIN);
    c.args(["--hook", "shell-guard"]).current_dir(root).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    for (name, _) in std::env::vars_os() {
        let n = name.to_string_lossy();
        if n.starts_with("GATE_") || n.contains("_KIT_") || n.starts_with("GIT_") {
            c.env_remove(&name);
        }
    }
    c.env("GATE_SDK_NATIVE_BIN", BIN)
        .env("GUARD_KIT_LOG", root.join("friction.log"))
        .env("PWD", root)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir());
    let mut child = c.spawn().expect("cannot run the binary under test");
    std::io::Write::write_all(&mut child.stdin.take().expect("piped stdin"), payload.as_bytes()).expect("payload");
    let out = child.wait_with_output().expect("cannot await the binary under test");
    out.status.code() == Some(0) && String::from_utf8_lossy(&out.stdout).contains("\"permissionDecision\":\"allow\"")
}

#[test]
fn a_scratch_write_through_a_link_takes_no_grant() {
    let s = scratch();
    let root = s.0.as_path();
    let pairs = [
        (": > .tmp/l", ": > .tmp/plain"),
        ("echo x >> .tmp/l", "echo x >> .tmp/plain"),
        ("echo x > .tmp/l", "echo x > .tmp/plain"),
        ("touch .tmp/dl/f", "touch .tmp/real/f"),
        ("cp tracked.md .tmp/dl", "cp tracked.md .tmp/copy.md"),
        (
            "git status > .tmp/l 2>&1 & echo \"pid=$! run=k\" > .tmp/k.run; wait",
            "git status > .tmp/k.log 2>&1 & echo \"pid=$! run=p\" > .tmp/p.run; wait",
        ),
        (
            "git status > .tmp/k.log 2>&1 & echo \"pid=$! run=lk\" > .tmp/lk.run; wait",
            "git status > .tmp/k.log 2>&1 & echo \"pid=$! run=p\" > .tmp/p.run; wait",
        ),
    ];
    for (_, plain) in pairs {
        assert!(granted(root, plain), "{:?} is a plain scratch write and takes its grant", plain);
    }
    let linked = link(&root.join("tracked.md"), &root.join(".tmp/l"), false)
        && link(&root.join("tracked.md"), &root.join(".tmp/lk.run"), false)
        && link(&root.join("sub"), &root.join(".tmp/dl"), true);
    if !linked {
        return;
    }
    for (through, _) in pairs {
        assert!(!granted(root, through), "{:?} reaches through a link and takes no grant", through);
    }
}
