// spec: gate-sdk/SPEC.md §The harness-integration arm — the `--knobs` token driven end to end: the
// named file is the only file layer inside a repository and outside one, a file the arm cannot use
// declines naming it, and the token on a member that reads a repository is refused

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_checkwright-gates");

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

impl Scratch {
    fn new(case: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("checkwright-hook-knobs-{}-{}", std::process::id(), case));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("free")).expect("cannot make the scratch dir");
        std::fs::create_dir_all(dir.join("state").join("tmp")).expect("cannot make the state dir");
        Scratch(dir)
    }

    fn at(&self, rel: &str) -> PathBuf {
        self.0.join(rel)
    }

    fn abs(&self, rel: &str) -> String {
        self.at(rel).display().to_string()
    }

    // spec: installer/SPEC.md §The user install — the knob file `init --user` writes: two absolute
    // scalars under the state dir
    fn user_knobs(&self) -> String {
        let file = self.abs("member.knobs");
        let text = format!(
            "GATE_SDK_TMP_DIR = {}\nDELEGATION_KIT_STOP_LOG = {}\n",
            self.abs("state/tmp"),
            self.abs("state/stop.log")
        );
        std::fs::write(&file, text).expect("knob file");
        file
    }
}

struct Fired {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

// spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the run reads no knob or git setting of the invoking
// environment, and git's discovery stops at the scratch dir
fn fire(s: &Scratch, from: &Path, args: &[&str], env: &[(&str, String)], payload: &str) -> Fired {
    let mut c = Command::new(BIN);
    c.arg("--hook").args(args).current_dir(from).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    for (name, _) in std::env::vars_os() {
        let n = name.to_string_lossy();
        if n.starts_with("GATE_") || n.contains("_KIT_") || n.starts_with("GIT_") {
            c.env_remove(&name);
        }
    }
    c.env("HOME", s.at("home")).env("GIT_CONFIG_NOSYSTEM", "1").env("GIT_CEILING_DIRECTORIES", &s.0).env("PWD", from);
    for (k, v) in env {
        c.env(k, v);
    }
    let mut child = c.spawn().expect("cannot run the binary under test");
    if let Err(e) = std::io::Write::write_all(&mut child.stdin.take().expect("piped stdin"), payload.as_bytes()) {
        assert_eq!(e.kind(), std::io::ErrorKind::BrokenPipe, "payload: {}", e);
    }
    let out = child.wait_with_output().expect("cannot await the binary under test");
    Fired {
        code: out.status.code(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

const RUNNING: &str = r#"{"session_id":"s-1","hook_event_name":"SubagentStop","background_tasks":[{"id":"t1","type":"shell","status":"running"}]}"#;
const QUIET: &str = r#"{"session_id":"s-1","hook_event_name":"SubagentStop"}"#;
const MEMBER: &str = "subagent-stop-liveness";
const NOTICE: &str = "every hook guard in this tree is off";

fn refused_turn_end(f: &Fired, case: &str) {
    assert_eq!(f.code, Some(2), "case {}: the running shell task did not refuse: {} {}", case, f.stdout, f.stderr);
    assert!(f.stderr.contains("background shell task"), "case {}: {}", case, f.stderr);
    assert!(!f.stdout.contains(NOTICE), "case {}: {}", case, f.stdout);
}

fn declined(f: &Fired, names: &[&str]) {
    assert_eq!(f.code, Some(0), "a decline exits 0: {}", f.stderr);
    assert!(f.stdout.contains("systemMessage") && f.stdout.contains("--knobs"), "{}", f.stdout);
    for n in names {
        assert!(f.stdout.contains(n), "the decline did not name {}: {}", n, f.stdout);
    }
    assert!(!f.stdout.contains(NOTICE), "{}", f.stdout);
}

fn git_init(dir: &Path) {
    let ok = Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .status()
        .is_ok_and(|s| s.success());
    assert!(ok, "git init failed in {}", dir.display());
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — outside any repository the file's absolute
// values are the member's, and the turn end it guards is refused on a running task
#[test]
fn the_knob_file_runs_the_member_with_no_repository() {
    let s = Scratch::new("outside");
    let file = s.user_knobs();
    refused_turn_end(&fire(&s, &s.at("free"), &[MEMBER, "--knobs", &file], &[], RUNNING), "outside");
    let log = std::fs::read_to_string(s.at("state/stop.log")).expect("the file's log was not written");
    assert!(log.contains("decision=refuse"), "{}", log);
    let f = fire(&s, &s.at("free"), &[MEMBER, "--knobs", &file], &[], QUIET);
    assert_eq!((f.code, f.stdout.as_str()), (Some(0), ""), "{}", f.stderr);
}

// spec: gate-sdk/SPEC.md §The knob file — inside a repository no toplevel is entered and neither the
// project's knob file nor `<KIT>_KNOB_FILE` is read: each names a log the run must not write
#[test]
fn the_knob_file_is_the_only_layer_inside_a_repository() {
    let s = Scratch::new("inside");
    let repo = s.at("free");
    git_init(&repo);
    std::fs::create_dir_all(repo.join("scripts")).expect("scripts dir");
    std::fs::write(repo.join("scripts/delegation-kit-config.knobs"), format!("DELEGATION_KIT_STOP_LOG = {}\n", s.abs("project.log")))
        .expect("project knob file");
    std::fs::write(s.at("env.knobs"), "this line is not in the grammar\n").expect("env knob file");
    let file = s.user_knobs();
    let env = vec![("DELEGATION_KIT_KNOB_FILE", s.abs("env.knobs"))];
    refused_turn_end(&fire(&s, &repo.join("scripts"), &[MEMBER, "--knobs", &file], &env, RUNNING), "inside");
    assert!(s.at("state/stop.log").is_file(), "the file's log was not written");
    assert!(!s.at("project.log").exists(), "the project knob file was read");
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a file the arm cannot use declines at exit 0
// naming `--knobs` and the file, and the line where one is at fault
#[test]
fn an_unusable_knob_file_declines_naming_it() {
    let s = Scratch::new("declines");
    let at = s.at("free");
    declined(&fire(&s, &at, &[MEMBER, "--knobs", "rel.knobs"], &[], RUNNING), &["relative path: rel.knobs"]);
    let missing = s.abs("missing.knobs");
    declined(&fire(&s, &at, &[MEMBER, "--knobs", &missing], &[], RUNNING), &[&missing]);
    // comment-tier-exempt: the undeclared name is assembled from parts rather than spelled, so
    // check-kit-ref-liveness does not read a knob no kit declares as a dangling reference
    let undeclared = format!("GATE_SDK_TMP_DIR = /x\n{}_NO_SUCH_KNOB = 1\n", "DELEGATION_KIT");
    for (case, text, line) in [
        ("malformed", "GATE_SDK_TMP_DIR = /x\nno equals sign here\n", ":2:"),
        ("unowned", "NO_SUCH_THING = 1\n", ":1:"),
        ("undeclared", undeclared.as_str(), ":2:"),
    ] {
        let file = s.abs(&format!("{}.knobs", case));
        std::fs::write(&file, text).expect("knob file");
        let f = fire(&s, &at, &[MEMBER, "--knobs", &file], &[], RUNNING);
        declined(&f, &[&format!("{}{}", file, line)]);
    }
    assert!(!s.at("state/stop.log").exists(), "a declined firing logged");
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — an unreadable file declines too; a reader
// whose privilege reads it anyway leaves the case unprovable on this host
#[cfg(unix)]
#[test]
fn an_unreadable_knob_file_declines() {
    use std::os::unix::fs::PermissionsExt;
    let s = Scratch::new("unreadable");
    let file = s.user_knobs();
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    if std::fs::read(&file).is_ok() {
        return;
    }
    declined(&fire(&s, &s.at("free"), &[MEMBER, "--knobs", &file], &[], RUNNING), &[&file]);
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — `--knobs` on a member that reads a
// repository is refused at exit 2 naming the tree-free members, wherever the arm stands, and the
// token order is the grammar's
#[test]
fn the_token_is_refused_off_a_tree_free_member_and_out_of_order() {
    let s = Scratch::new("grammar");
    let file = s.user_knobs();
    let f = fire(&s, &s.at("free"), &["shell-guard", "--knobs", &file], &[], "{}");
    assert_eq!(f.code, Some(2), "{} {}", f.stdout, f.stderr);
    assert!(f.stderr.contains("--knobs") && f.stderr.contains(MEMBER), "{}", f.stderr);
    git_init(&s.at("free"));
    let f = fire(&s, &s.at("free"), &[MEMBER, "--knobs", &file, "--observe"], &[], RUNNING);
    assert_eq!(f.code, Some(2), "{} {}", f.stdout, f.stderr);
    assert!(f.stderr.contains("token outside the --hook grammar: --observe"), "{}", f.stderr);
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — an observed run's child reads the parent's
// layer, so its log lands under the file's scratch dir carrying the refusal, and a bound call takes
// the bound-call decline
#[test]
fn an_observed_run_hands_the_file_to_its_child_and_a_bound_call_declines() {
    let s = Scratch::new("observe");
    let file = s.user_knobs();
    let f = fire(&s, &s.at("free"), &[MEMBER, "--observe", "--knobs", &file], &[], RUNNING);
    assert_eq!((f.code, f.stdout.as_str()), (Some(0), ""), "{}", f.stderr);
    let log = std::fs::read_to_string(s.at("state/tmp/hook-observe.log")).expect("the observe log was not written");
    assert!(log.contains("would=block"), "{}", log);
    let f = fire(&s, &s.at("free"), &[MEMBER, "--binding", "antigravity", "--knobs", &file], &[], RUNNING);
    assert_eq!(f.code, Some(0), "{}", f.stderr);
    assert!(f.stdout.contains("bound call"), "{}", f.stdout);
}
