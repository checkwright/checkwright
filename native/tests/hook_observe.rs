// spec: gate-sdk/SPEC.md §The harness-integration arm — the observed run driven end to end: the member
// re-run as a child, its would-be decision logged, the call allowed silently, and a log that cannot
// be written declining

mod common;

use std::path::{Path, PathBuf};
use std::process::Stdio;

const BIN: &str = env!("CARGO_BIN_EXE_checkwright-gates");

const LOG_KNOB: &str = "GATE_SDK_HOOK_OBSERVE_LOG";

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

impl Scratch {
    fn new(case: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("checkwright-observe-{}-{}", std::process::id(), case));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("repo")).expect("cannot make the scratch dir");
        std::fs::create_dir_all(dir.join("free")).expect("cannot make the scratch dir");
        let init = common::command("git")
            .arg("-C")
            .arg(dir.join("repo"))
            .args(["init", "-q"])
            .output()
            .expect("git did not spawn");
        assert!(init.status.success(), "git init: {}", String::from_utf8_lossy(&init.stderr));
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
    std::io::Write::write_all(&mut child.stdin.take().expect("piped stdin"), payload.as_bytes()).expect("payload");
    let out = child.wait_with_output().expect("cannot await the binary under test");
    Fired {
        code: out.status.code(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

const RUNNING: &str = r#"{"session_id":"s-1","hook_event_name":"SubagentStop","background_tasks":[{"id":"t1","type":"shell","status":"running"}]}"#;
const QUIET: &str = r#"{"session_id":"s-1","hook_event_name":"SubagentStop"}"#;
const FORK: &str = r#"{"session_id":"s-2","hook_event_name":"PreToolUse","tool_name":"Agent","tool_input":{"subagent_type":"fork","prompt":"typed text"}}"#;

const LIVENESS: &str = "subagent-stop-liveness";

fn log_env(s: &Scratch) -> Vec<(&'static str, String)> {
    vec![(LOG_KNOB, s.abs("observe.log"))]
}

fn silent(f: &Fired, case: &str) {
    assert_eq!(f.code, Some(0), "case {}: an observed call exits 0: {}", case, f.stderr);
    assert!(f.stdout.is_empty() && f.stderr.is_empty(), "case {}: an observed call spoke: {} {}", case, f.stdout, f.stderr);
}

fn lines(s: &Scratch) -> Vec<String> {
    std::fs::read_to_string(s.at("observe.log")).expect("the observe log was not written").lines().map(str::to_string).collect()
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a running own shell task logs the turn-end
// refusal it would have made, a quiet one logs none, and the call passes silently either way
#[test]
fn an_observed_turn_end_logs_its_would_be_decision_and_allows() {
    let s = Scratch::new("liveness");
    let repo = s.at("repo");
    silent(&fire(&s, &repo, &[LIVENESS, "--observe"], &log_env(&s), RUNNING), "running");
    silent(&fire(&s, &repo, &[LIVENESS, "--observe"], &log_env(&s), QUIET), "quiet");
    let l = lines(&s);
    assert_eq!(l.len(), 2, "{:?}", l);
    assert!(l[0].contains("member=subagent-stop-liveness event=SubagentStop session=s-1 would=block status=2 reason=withheld"), "{}", l[0]);
    assert!(!l[0].contains("background shell task"), "the refusal's text reached the log: {}", l[0]);
    assert!(l[1].contains("would=none status=0 reason=-"), "{}", l[1]);
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — the child carries the observe mark, so an
// observed session-context leaves a stale scratch path the enforcing run then takes
#[cfg(unix)]
#[test]
fn an_observed_session_context_sweeps_no_scratch() {
    let s = Scratch::new("sweep");
    let repo = s.at("repo");
    std::fs::create_dir_all(repo.join(".tmp")).expect("scratch root");
    let aged = repo.join(".tmp/aged.txt");
    std::fs::write(&aged, "x").expect("scratch file");
    let touched = common::command("touch").args(["-t", "200001010000"]).arg(&aged).status().expect("touch must be spawnable");
    assert!(touched.success(), "touch could not age {}", aged.display());
    let mut env = log_env(&s);
    env.push(("CONTEXT_KIT_SWEEP_HORIZON_HOURS", "1".to_string()));
    env.push(("GATE_SDK_TMP_DIR", ".tmp".to_string()));
    let start = r#"{"session_id":"s-3","hook_event_name":"SessionStart"}"#;
    silent(&fire(&s, &repo, &["session-context", "--observe"], &env, start), "observed");
    assert!(aged.is_file(), "an observed session-context swept the scratch dir");
    assert!(lines(&s)[0].contains("member=session-context event=SessionStart session=s-3"), "{:?}", lines(&s));
    let f = fire(&s, &repo, &["session-context"], &env, start);
    assert!(f.stdout.contains("Tidied 1 stale scratch path(s)"), "{}", f.stdout);
    assert!(!aged.exists(), "the enforcing run kept the stale path, so the trial proved nothing");
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a fork dispatch the guard blocks outright is
// logged as a block, and the line carries no tool input
#[test]
fn an_observed_dispatch_guard_logs_the_block_without_the_tool_input() {
    let s = Scratch::new("dispatch");
    silent(&fire(&s, &s.at("repo"), &["agent-dispatch-guard", "--observe"], &log_env(&s), FORK), "fork");
    let l = lines(&s);
    assert_eq!(l.len(), 1, "{:?}", l);
    assert!(l[0].contains("member=agent-dispatch-guard event=PreToolUse session=s-2 would=block status=2"), "{}", l[0]);
    assert!(!l[0].contains("typed text"), "the tool input reached the log: {}", l[0]);
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a log that cannot be written declines naming
// the knob, and the call still proceeds
#[test]
fn an_unwritable_log_declines_naming_the_knob() {
    let s = Scratch::new("unwritable");
    let f = fire(&s, &s.at("repo"), &[LIVENESS, "--observe"], &[(LOG_KNOB, s.abs("free"))], RUNNING);
    assert_eq!(f.code, Some(0), "a decline exits 0: {}", f.stderr);
    assert!(f.stdout.contains("systemMessage") && f.stdout.contains(LOG_KNOB), "{}", f.stdout);
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — a token outside the grammar, a repeated flag
// and a flag out of order each refuse at exit 2 naming the token, and nothing runs
#[test]
fn a_token_outside_the_grammar_refuses() {
    let s = Scratch::new("grammar");
    for (args, token) in [
        (&[LIVENESS, "--obsrve"][..], "--obsrve"),
        (&[LIVENESS, "--observe", "--observe"][..], "--observe"),
        (&[LIVENESS, "--binding", "antigravity", "--observe"][..], "--observe"),
    ] {
        let f = fire(&s, &s.at("repo"), args, &log_env(&s), RUNNING);
        assert_eq!(f.code, Some(2), "{:?}: {} {}", args, f.stdout, f.stderr);
        assert!(f.stderr.contains(&format!("outside the --hook grammar: {}", token)), "{:?}: {}", args, f.stderr);
        assert!(f.stderr.contains("[--observe] [--binding <name>]"), "{:?}: the grammar is not printed: {}", args, f.stderr);
    }
    assert!(!s.at("observe.log").exists(), "a refused registration logged");
}

// spec: gate-sdk/SPEC.md §The harness-integration arm — with no repository, an absolute log is
// written and the default, relative, log declines naming its knob
#[test]
fn a_tree_free_observed_run_logs_only_to_an_absolute_log() {
    let s = Scratch::new("tree-free");
    let mut env = log_env(&s);
    env.push(("DELEGATION_KIT_STOP_LOG", s.abs("stop.log")));
    env.push(("GATE_SDK_TMP_DIR", s.abs("free")));
    silent(&fire(&s, &s.at("free"), &[LIVENESS, "--observe"], &env, RUNNING), "absolute");
    let l = lines(&s);
    assert!(l.len() == 1 && l[0].contains("would=block status=2"), "{:?}", l);
    let env = vec![("DELEGATION_KIT_STOP_LOG", s.abs("stop.log"))];
    let f = fire(&s, &s.at("free"), &[LIVENESS, "--observe"], &env, RUNNING);
    assert_eq!(f.code, Some(0), "{}", f.stderr);
    assert!(f.stdout.contains("systemMessage") && f.stdout.contains(LOG_KNOB), "{}", f.stdout);
    assert!(!s.at("free/.tmp").exists(), "the relative default log was written");
}
