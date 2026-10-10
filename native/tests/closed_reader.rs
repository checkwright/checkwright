// spec: gate-sdk/SPEC.md §The non-gate arm — the departed-reader rule, run on the binary cargo
// builds for this test run with a stdout that is a pipe whose reader has already gone, and its
// bound on a child handed that stdout

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_checkwright-gates");

const QUEUE_HEAD: &str = "# TASK-QUEUE.md\n\n## Iteration: seeded\n\n## New Features\n\n## Technical Debt\n\n\
## Deferred\n\n### def-a\n\n[cost: event/low] [surface: gates]\n\nthe first entry.\n\n## Done\n";

// spec: queue-kit/SPEC.md §check-queue-sections — the heading whose absence is the planted red
const QUEUE_TAIL: &str = "\n## Lessons Learned\n";

struct Scratch(PathBuf);

impl Scratch {
    fn new(case: &str, queue: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("checkwright-closed-reader-{}-{}", std::process::id(), case));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("gates")).expect("cannot create the scratch gates dir");
        std::fs::write(dir.join("gates").join("gates.list"), "check-queue-sections\n")
            .expect("cannot write the scratch registry");
        std::fs::write(dir.join("TASK-QUEUE.md"), queue).expect("cannot write the scratch queue");
        Scratch(dir)
    }

    fn queue(&self) -> String {
        std::fs::read_to_string(self.0.join("TASK-QUEUE.md")).expect("cannot read the scratch queue")
    }

    // spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the case reads no knob of the invoking
    // environment: every kit's and the harness's names are dropped, then the scratch registry set
    fn command(&self, args: &[&str]) -> Command {
        let mut c = Command::new(BIN);
        c.args(args).current_dir(&self.0).stdin(Stdio::null()).stderr(Stdio::piped());
        for (name, _) in std::env::vars_os() {
            let n = name.to_string_lossy();
            if n.starts_with("GATE_") || n.contains("_KIT_") || n.starts_with("GIT_") {
                c.env_remove(&name);
            }
        }
        let queue_kit = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the crate sits under the repository root")
            .join("queue-kit");
        c.env("GATE_SDK_GATES_DIR", self.0.join("gates"))
            .env("GATE_SDK_KIT_DIRS", queue_kit)
            .env("GATE_SDK_TMP_DIR", self.0.join(".tmp"))
            .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir());
        c
    }

    fn read(&self, args: &[&str]) -> Output {
        self.command(args).stdout(Stdio::piped()).output().expect("cannot run the binary under test")
    }

    fn unread(&self, args: &[&str]) -> Output {
        self.command(args).stdout(departed()).output().expect("cannot run the binary under test")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — a pipe whose reader has gone, never a closed
// descriptor: the write end of a child's stdin, kept past that child's exit
fn departed() -> Stdio {
    let mut reader = Command::new(BIN)
        .arg("--source-stamp")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("cannot spawn the reader that departs");
    let writer = reader.stdin.take().expect("the reader's stdin is piped");
    reader.wait().expect("cannot await the reader that departs");
    Stdio::from(writer)
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the status with the reader gone is the status with it
// present, on an empty stderr
fn same_status_unread(case: &str, queue: &str, args: &[&str], want: i32) -> Scratch {
    let with_reader = Scratch::new(&format!("{}-read", case), queue).read(args);
    assert_eq!(with_reader.status.code(), Some(want), "{}: with a reader: {}", case, stderr(&with_reader));
    assert!(!with_reader.stdout.is_empty(), "{}: the arm prints nothing, so no write could fail", case);
    let scratch = Scratch::new(case, queue);
    let unread = scratch.unread(args);
    assert_eq!(unread.status.code(), Some(want), "{}: reader gone: {}", case, stderr(&unread));
    assert_eq!(stderr(&unread), "", "{}: reader gone: stderr is not empty", case);
    scratch
}

#[test]
fn a_member_keeps_its_verdict() {
    let whole = format!("{}{}", QUEUE_HEAD, QUEUE_TAIL);
    same_status_unread("member-clean", &whole, &["check-queue-sections"], 0);
    same_status_unread("member-finding", QUEUE_HEAD, &["check-queue-sections"], 1);
}

#[test]
fn an_emit_arm_that_prints_a_document_exits_clean() {
    same_status_unread("emit", QUEUE_HEAD, &["--emit-knob-roster"], 0);
}

#[test]
fn a_queue_move_lands_and_keeps_its_status() {
    let whole = format!("{}{}", QUEUE_HEAD, QUEUE_TAIL);
    let recur = ["--queue", "recur", "def-a", "2026-05-01"];
    for (case, queue, want) in [("queue-green", whole.as_str(), 0), ("queue-red", QUEUE_HEAD, 1)] {
        let scratch = same_status_unread(case, queue, &recur, want);
        assert!(scratch.queue().contains("[recurrence: 2026-05-01]"), "{}: the move was not written", case);
    }
}

#[test]
fn a_refused_queue_move_still_exits_two() {
    let scratch = Scratch::new("queue-refused", QUEUE_HEAD);
    let out = scratch.unread(&["--queue", "recur", "no-such-slug", "2026-05-01"]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(!stderr(&out).contains("panicked"), "a refusal printed a panic trace: {}", stderr(&out));
    assert_eq!(scratch.queue(), QUEUE_HEAD, "a refused move changed the queue");
}

// spec: gate-sdk/SPEC.md §The non-gate arm — a child handed the inherited stdout answers for its
// own writes: the second assertion is the rule's bound, so a change making the rule total reds here
#[cfg(unix)]
#[test]
fn a_child_on_the_inherited_stdout_answers_for_its_own_writes() {
    let run = |case: &str, body: &str, unread: bool| {
        let scratch = Scratch::new(case, QUEUE_HEAD);
        std::fs::create_dir_all(scratch.0.join(".tmp")).expect("cannot create the scratch .tmp");
        std::fs::write(scratch.0.join(".tmp").join("child.sh"), body).expect("cannot write the scratch script");
        let args = ["--scratch-run", ".tmp/child.sh"];
        if unread { scratch.unread(&args) } else { scratch.read(&args) }
    };
    for (case, body) in [("child-silent", "exit 7\n"), ("child-prints", "echo printed\nexit 7\n")] {
        let with_reader = run(&format!("{}-read", case), body, false);
        assert_eq!(with_reader.status.code(), Some(7), "{}: with a reader: {}", case, stderr(&with_reader));
    }
    let silent = run("child-silent", "exit 7\n", true);
    assert_eq!(silent.status.code(), Some(7), "a child that wrote nothing: {}", stderr(&silent));
    assert_eq!(stderr(&silent), "", "a child that wrote nothing: stderr is not empty");
    let prints = run("child-prints", "echo printed\nexit 7\n", true);
    assert_eq!(
        prints.status.code(),
        Some(128 + libc::SIGPIPE),
        "a child that wrote after its reader had gone: {}",
        stderr(&prints)
    );
}
