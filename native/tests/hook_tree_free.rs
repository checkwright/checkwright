// spec: gate-sdk/SPEC.md §The harness-integration arm — the tree-free dispatch driven end to end: a
// tree-free member started outside any repository runs in place on absolute knobs and takes a
// relative one through its own fault route, and every other no-tree call keeps the notice

mod common;

use std::path::{Path, PathBuf};
use std::process::Stdio;

const BIN: &str = env!("CARGO_BIN_EXE_checkwright-gates");

const NOTICE: &str = "every hook guard in this tree is off";

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

impl Scratch {
    fn new(case: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("checkwright-tree-free-{}-{}", std::process::id(), case));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("free").join("runs")).expect("cannot make the scratch dir");
        Scratch(dir)
    }

    fn at(&self, rel: &str) -> PathBuf {
        self.0.join(rel)
    }

    fn abs(&self, rel: &str) -> String {
        self.at(rel).display().to_string()
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
    let mut c = common::scrubbed(BIN);
    c.arg("--hook").args(args).current_dir(from).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    c.env("HOME", s.at("home")).env("GIT_CEILING_DIRECTORIES", &s.0).env("PWD", from);
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

const MEMBER: &[&str] = &["subagent-stop-liveness"];

fn absolute(s: &Scratch) -> Vec<(&'static str, String)> {
    vec![("GATE_SDK_TMP_DIR", s.abs("free/runs")), ("DELEGATION_KIT_STOP_LOG", s.abs("stop.log"))]
}

fn declined(f: &Fired, knob: &str) {
    assert_eq!(f.code, Some(0), "a decline exits 0: {}", f.stderr);
    assert!(f.stdout.contains("systemMessage") && f.stdout.contains(knob), "the envelope did not name {}: {}", knob, f.stdout);
    assert!(f.stdout.contains("absolute"), "the envelope did not name the remedy: {}", f.stdout);
    assert!(!f.stdout.contains(NOTICE), "a tree-free member wrote the no-tree notice: {}", f.stdout);
}

fn no_tree(f: &Fired, case: &str) {
    assert_eq!(f.code, Some(0), "case {}: {}", case, f.stderr);
    assert!(f.stdout.contains(NOTICE), "case {}: the no-tree notice is missing: {}", case, f.stdout);
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — absolute knobs run the member in place, and
// its own running shell task refuses the turn end
#[test]
fn a_tree_free_member_runs_with_no_repository_on_absolute_knobs() {
    let s = Scratch::new("runs");
    let f = fire(&s, &s.at("free"), MEMBER, &absolute(&s), RUNNING);
    assert_eq!(f.code, Some(2), "the running shell task did not refuse: {} {}", f.stdout, f.stderr);
    assert!(f.stderr.contains("background shell task"), "{}", f.stderr);
    assert!(!f.stdout.contains(NOTICE), "{}", f.stdout);
    let log = std::fs::read_to_string(s.at("stop.log")).expect("the absolute log was not written");
    assert!(log.contains("decision=refuse"), "{}", log);
    let f = fire(&s, &s.at("free"), MEMBER, &absolute(&s), QUIET);
    assert_eq!(f.code, Some(0), "{}", f.stderr);
    assert!(f.stdout.is_empty(), "an allow writes no stdout: {}", f.stdout);
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a relative path-valued knob, the defaults
// included, declines naming the knob, and nothing is written beside the firing directory
#[test]
fn a_relative_path_knob_declines_naming_itself() {
    let s = Scratch::new("relative");
    declined(&fire(&s, &s.at("free"), MEMBER, &[], RUNNING), "GATE_SDK_TMP_DIR");
    let env = vec![("GATE_SDK_TMP_DIR", s.abs("free/runs")), ("DELEGATION_KIT_STOP_LOG", "rel.log".to_string())];
    declined(&fire(&s, &s.at("free"), MEMBER, &env, RUNNING), "DELEGATION_KIT_STOP_LOG");
    assert!(!s.at("free/rel.log").exists(), "the relative log was written");
    std::fs::write(s.at("cmd.knobs"), "DELEGATION_KIT_LIVENESS_CMD[] = bin/reader\n").expect("knob file");
    let mut env = absolute(&s);
    env.push(("DELEGATION_KIT_KNOB_FILE", s.abs("cmd.knobs")));
    declined(&fire(&s, &s.at("free"), MEMBER, &env, RUNNING), "DELEGATION_KIT_LIVENESS_CMD");
    assert!(!s.at("stop.log").exists(), "a declined firing logged");
}

// spec: gate-sdk/SPEC.md §The knob file — under the mark a consumer-set knob file is honored when
// absolute and declines when relative, and a default locator is never opened
#[test]
fn knob_files_open_only_by_an_absolute_locator() {
    let s = Scratch::new("knobfile");
    let tmp = vec![("GATE_SDK_TMP_DIR", s.abs("free/runs"))];
    let mut env = tmp.clone();
    env.push(("DELEGATION_KIT_KNOB_FILE", "pinned.knobs".to_string()));
    declined(&fire(&s, &s.at("free"), MEMBER, &env, QUIET), "DELEGATION_KIT_KNOB_FILE");
    std::fs::write(s.at("pinned.knobs"), format!("DELEGATION_KIT_STOP_LOG = {}\n", s.abs("pinned.log"))).expect("knob file");
    let mut env = tmp.clone();
    env.push(("DELEGATION_KIT_KNOB_FILE", s.abs("pinned.knobs")));
    let f = fire(&s, &s.at("free"), MEMBER, &env, QUIET);
    assert_eq!(f.code, Some(0), "{} {}", f.stdout, f.stderr);
    assert!(s.at("pinned.log").is_file(), "the absolute knob file was not honored: {}", f.stdout);
    std::fs::create_dir_all(s.at("free/scripts")).expect("stray scripts dir");
    std::fs::write(s.at("free/scripts/delegation-config.knobs"), format!("DELEGATION_KIT_STOP_LOG = {}\n", s.abs("stray.log")))
        .expect("stray knob file");
    declined(&fire(&s, &s.at("free"), MEMBER, &tmp, QUIET), "DELEGATION_KIT_STOP_LOG");
    assert!(!s.at("stray.log").exists(), "the stray knob file was read");
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — every no-tree call but the tree-free member's
// shipped binding keeps the notice, a refused repository included
#[test]
fn every_other_no_tree_call_keeps_the_notice() {
    let s = Scratch::new("notice");
    let env = absolute(&s);
    for (case, args) in [
        ("not-tree-free", &["shell-guard"][..]),
        ("unknown", &["no-such-member"][..]),
        ("binding", &["subagent-stop-liveness", "--binding", "antigravity"][..]),
    ] {
        no_tree(&fire(&s, &s.at("free"), args, &env, RUNNING), case);
    }
    std::fs::create_dir_all(s.at("refused")).expect("refused dir");
    std::fs::write(s.at("refused/.git"), "gitdir: /nonexistent/checkwright-tree-free\n").expect("broken gitfile");
    let f = fire(&s, &s.at("refused"), MEMBER, &env, RUNNING);
    no_tree(&f, "refused");
    assert!(f.stderr.contains("marks a repository"), "{}", f.stderr);
    assert!(!s.at("stop.log").exists(), "a declined firing logged");
}

const WAKE: &str = r#"{"session_id":"s-1","hook_event_name":"PreToolUse","tool_name":"ScheduleWakeup","tool_input":{"delaySeconds":600}}"#;
const HEADERLESS: &str = r#"{"session_id":"s-1","hook_event_name":"PreToolUse","tool_name":"SendMessage","tool_input":{"to":"main","message":"done?"}}"#;
const DISPATCH: &str = r#"{"session_id":"s-1","hook_event_name":"PreToolUse","tool_name":"Agent","tool_input":{"subagent_type":"untyped","isolation":"worktree"}}"#;

// spec: guard-kit/SPEC.md §wakeup-guard — with no repository the deny stands on an absolute log,
// which records the attempt, and on a relative one, the default included, which names its fault
// and writes nothing
#[test]
fn with_no_repository_the_wakeup_guard_denies_logged_or_naming_its_fault() {
    let s = Scratch::new("wakeup");
    let env = vec![("GUARD_KIT_WAKEUP_LOG", s.abs("wake.log"))];
    let f = fire(&s, &s.at("free"), &["wakeup-guard"], &env, WAKE);
    assert_eq!(f.code, Some(2), "{} {}", f.stdout, f.stderr);
    assert!(!f.stderr.contains("went unlogged"), "{}", f.stderr);
    let log = std::fs::read_to_string(s.at("wake.log")).expect("the absolute log was not written");
    assert!(log.contains("ScheduleWakeup"), "{}", log);
    let f = fire(&s, &s.at("free"), &["wakeup-guard"], &[], WAKE);
    assert_eq!(f.code, Some(2), "a relative log let the call through: {}", f.stdout);
    assert!(f.stderr.contains("went unlogged") && f.stderr.contains("GUARD_KIT_WAKEUP_LOG"), "{}", f.stderr);
    assert!(!s.at("free/.workflow").exists(), "the relative log's dir was written");
    std::fs::write(s.at("bad.knobs"), "NO_SUCH_KNOB = 1\n").expect("knob file");
    let f = fire(&s, &s.at("free"), &["wakeup-guard", "--knobs", &s.abs("bad.knobs")], &[], WAKE);
    assert_eq!(f.code, Some(2), "a faulty knob file let the call through: {}", f.stdout);
    assert!(f.stderr.contains("went unlogged: --knobs"), "{}", f.stderr);
}

// spec: guard-kit/SPEC.md §escalation-guard — with no repository the advisory reads the payload
// alone, and a faulty knob file declines naming itself
#[test]
fn with_no_repository_the_escalation_guard_advises_and_a_faulty_file_declines() {
    let s = Scratch::new("escalation");
    let f = fire(&s, &s.at("free"), &["escalation-guard"], &[], HEADERLESS);
    assert_eq!(f.code, Some(0), "{}", f.stderr);
    assert!(f.stdout.contains("decision-shape header") && !f.stdout.contains(NOTICE), "{}", f.stdout);
    let f = fire(&s, &s.at("free"), &["escalation-guard", "--knobs", "rel.knobs"], &[], HEADERLESS);
    assert_eq!(f.code, Some(0), "{}", f.stderr);
    assert!(f.stdout.contains("--knobs") && f.stdout.contains("relative path: rel.knobs"), "{}", f.stdout);
    assert!(!f.stdout.contains("decision-shape header"), "a declined firing advised: {}", f.stdout);
}

// spec: delegation-kit/SPEC.md §Layout and configuration — with no repository a relative usage path
// is the `STALE` verdict naming the knob, advised and never blocking
#[test]
fn with_no_repository_a_relative_usage_path_reads_stale() {
    let s = Scratch::new("budget");
    let env = vec![("DELEGATION_KIT_USAGE_FILE", "usage.txt".to_string())];
    let f = fire(&s, &s.at("free"), &["agent-budget-guard"], &env, DISPATCH);
    assert_eq!(f.code, Some(0), "{}", f.stderr);
    assert!(f.stdout.contains("DELEGATION_KIT_USAGE_FILE is 'usage.txt'") && f.stdout.contains("STALE"), "{}", f.stdout);
}

// spec: delegation-kit/SPEC.md §Layout and configuration — with no repository and D5 on, the
// relative default agent dir leaves D5 unenforced with its note, and an absolute one enforces it
#[test]
fn with_no_repository_d5_on_a_relative_agent_dir_goes_unenforced() {
    let s = Scratch::new("dispatch");
    let on = vec![("DELEGATION_KIT_REQUIRE_TIER", "on".to_string())];
    let f = fire(&s, &s.at("free"), &["agent-dispatch-guard"], &on, DISPATCH);
    assert_eq!(f.code, Some(0), "{}", f.stderr);
    assert!(f.stdout.contains("went unenforced") && f.stdout.contains("DELEGATION_KIT_AGENT_DIR"), "{}", f.stdout);
    std::fs::create_dir_all(s.at("agents")).expect("agent dir");
    let mut abs = on.clone();
    abs.push(("DELEGATION_KIT_AGENT_DIR", s.abs("agents")));
    let f = fire(&s, &s.at("free"), &["agent-dispatch-guard"], &abs, DISPATCH);
    assert_eq!(f.code, Some(2), "D5 on an absolute dir let an unknown tier through: {}", f.stdout);
}

// spec: gate-sdk/SPEC.md §The crate's crosser — a repository path, the working directory or an
// argument with no UTF-8 spelling is refused by name, its bytes escaped, never decoded lossily
#[cfg(target_os = "linux")]
mod unspelled {
    use super::*;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    fn odd(s: &Scratch, leaf: &[u8]) -> PathBuf {
        s.0.join(OsStr::from_bytes(leaf))
    }

    fn git(dir: &Path, args: &[&str]) -> String {
        let out = common::command("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .expect("git did not spawn");
        assert!(out.status.success(), "git {:?}: {}", args, String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    #[test]
    fn a_working_directory_and_a_toplevel_with_no_utf8_spelling_are_refused() {
        let s = Scratch::new("cwd");
        let outside = odd(&s, b"out-\xff");
        std::fs::create_dir_all(&outside).expect("odd dir");
        let f = fire(&s, &outside, MEMBER, &absolute(&s), RUNNING);
        no_tree(&f, "cwd");
        assert!(f.stderr.contains("the current directory") && f.stderr.contains("out-\\xff"), "{}", f.stderr);
        let inside = odd(&s, b"repo-\xfe");
        std::fs::create_dir_all(&inside).expect("odd repo");
        git(&inside, &["init", "-q"]);
        let f = fire(&s, &inside, MEMBER, &absolute(&s), RUNNING);
        no_tree(&f, "toplevel");
        assert!(f.stderr.contains("the work tree git answers") && f.stderr.contains("repo-\\xfe"), "{}", f.stderr);
        assert!(!f.stderr.contains('\u{fffd}'), "a lossy spelling leaked: {}", f.stderr);
    }

    #[test]
    fn an_argument_with_no_utf8_spelling_exits_two_without_a_panic() {
        let s = Scratch::new("argv");
        let out = common::command(BIN).arg("--hook").arg(OsStr::from_bytes(b"x\xff")).output().expect("spawn");
        let err = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(2), "{}", err);
        assert!(err.contains("argument 2") && err.contains("x\\xff") && !err.contains("panicked"), "{}", err);
        assert!(!err.contains("--no-verify"), "a plain start named the hook remedies: {}", err);
        let hooks = s.at("hooks");
        std::fs::create_dir_all(&hooks).expect("hooks dir");
        std::os::unix::fs::symlink(BIN, hooks.join("pre-commit")).expect("served start name");
        let out = common::command(hooks.join("pre-commit")).arg(OsStr::from_bytes(b"\xfe")).output().expect("spawn");
        let err = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(2), "{}", err);
        assert!(err.contains("argument 1") && err.contains("--no-verify") && err.contains("core.hooksPath"), "{}", err);
        assert!(!err.contains("panicked"), "{}", err);
    }

    // spec: gate-sdk/SPEC.md §install-hooks — a UTF-8 linked worktree whose common directory has no
    // UTF-8 spelling places, points and removes nothing
    #[test]
    fn a_common_directory_with_no_utf8_spelling_is_refused_with_nothing_changed() {
        let s = Scratch::new("common");
        let main = s.at("main");
        std::fs::create_dir_all(&main).expect("main dir");
        git(&main, &["init", "-q"]);
        git(&main, &["config", "user.name", "Scratch"]);
        git(&main, &["config", "user.email", "scratch@example.com"]);
        let art = s.at("pkg/payload/artifact/t");
        std::fs::create_dir_all(&art).expect("artifact dir");
        std::fs::create_dir_all(s.at("pkg/payload").join("a-kit")).expect("kit dir");
        std::fs::write(s.at("pkg/package.json"), "{\"name\":\"scratch\",\"version\":\"1.2.3\"}\n").expect("package.json");
        std::fs::write(s.at("pkg/profiles.list"), "starter\ta-kit\n").expect("profiles.list");
        std::fs::write(s.at("pkg/payload").join("a-kit").join("x.txt"), "x\n").expect("the kit's file");
        let bin = art.join(Path::new(BIN).file_name().expect("a binary name"));
        // spec: delegation-kit/SPEC.md §The turn-end liveness hook — the binary is placed by a child
        // process, so no sibling test's fork holds its write descriptor into the spawn as ETXTBSY
        let placed = common::command("cp").arg(BIN).arg(&bin).status().expect("cp did not spawn");
        assert!(placed.success(), "cannot place the binary under test");
        let run = |verb: &str, at: &Path| {
            let mut c = common::scrubbed(&bin);
            c.arg(verb).current_dir(at).stdin(Stdio::null()).env_remove("EMAIL");
            c.env("HOME", s.at("home"))
                .env("XDG_CONFIG_HOME", s.at("home"))
                .env("GATE_SDK_UPDATE_CHECK", "off")
                .env("GIT_CEILING_DIRECTORIES", &s.0);
            c.output().expect("cannot run the binary under test")
        };
        let init = run("--init", &main);
        assert_eq!(init.status.code(), Some(0), "the install failed: {}", String::from_utf8_lossy(&init.stderr));
        git(&main, &["worktree", "add", "-q", &s.abs("wt")]);
        let renamed = odd(&s, b"main-\xff");
        std::fs::rename(&main, &renamed).expect("rename the main checkout");
        git(&renamed, &["worktree", "repair", &s.abs("wt")]);
        let common = renamed.join(".git");
        for verb in ["--install-hooks", "--uninstall"] {
            let out = run(verb, &s.at("wt"));
            let err = String::from_utf8_lossy(&out.stderr);
            assert_eq!(out.status.code(), Some(2), "{}: {}", verb, err);
            assert!(err.contains("main-\\xff") && !err.contains("panicked"), "{}: {}", verb, err);
            assert!(!common.join("gate-hooks").exists(), "{}: a hooks directory was placed", verb);
            let pointed = common::command("git")
                .arg("-C")
                .arg(s.at("wt"))
                .args(["config", "core.hooksPath"])
                .output()
                .expect("git");
            assert!(pointed.stdout.is_empty(), "{}: core.hooksPath was set", verb);
            assert!(s.at("wt/checkwright.lock").is_file() && s.at("wt").join("a-kit").join("x.txt").is_file(), "{}: files were removed", verb);
            assert_eq!(git(&s.at("wt"), &["status", "--porcelain"]), "", "{}: the worktree was written", verb);
        }
    }
}
