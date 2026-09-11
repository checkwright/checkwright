// spec: guard-kit/SPEC.md §Testing — the decision-table runner, bridged as an `Arm::Run` member:
// its contract is a three-valued exit — 0 clean, 1 a verdict mismatch, 2 a harness precondition —
// which `Arm::Emit` collapses to 0-or-2.
// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged-arm table member rather than a hardcoded
// top-level flag, because the member is configured: it needs the vendored guard-kit root.
use crate::proc::{self, Stderr};
use crate::walk;
use std::path::Path;

// spec: guard-kit/SPEC.md §Testing — one declared knob; the two omissions are ruled there,
// `GUARD_KIT_LOG` because the arm overrides it and the guard's own knobs because the spawned
// child's `lib/guard.sh` is their sole resolver.
pub const KNOBS: &[&str] = &["GATE_KIT_ROOTS_HERE"];

const NAME: &str = "run-guard-tests";

// spec: guard-kit/SPEC.md §Testing — `Drop` is the shell form's `trap 'rm -rf' EXIT`, armed the
// moment the directory exists so no later refusal can leak it.
struct Sandbox {
    root: String,
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

pub fn run(args: &[String]) -> i32 {
    match execute(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", e);
            2
        }
    }
}

fn execute(args: &[String]) -> Result<i32, String> {
    let kit = kit_root()?;
    let guard = format!("{}/templates/bash-guard.sh", kit);
    let lib = format!("{}/lib/guard.sh", kit);
    let cases = positional(args, 0, &format!("{}/guard-tests/cases.tsv", kit));
    let bg_cases = positional(args, 1, &format!("{}/guard-tests/background-cases.tsv", kit));
    for f in [&guard, &lib, &cases, &bg_cases] {
        if !Path::new(f).is_file() {
            return Err(format!("{}: missing {}", NAME, f));
        }
    }
    // spec: guard-kit/SPEC.md §Testing — the `jq` precondition survives the port because the
    // *subject* still spawns it; the section states what an absent `jq` would red instead.
    if !proc::on_path("jq") {
        return Err(format!("{}: jq not found on PATH", NAME));
    }

    let sandbox = build_sandbox()?;
    let log = format!("{}/friction.log", sandbox.root);

    let mut tally = Tally::default();
    for (want, cmd) in rows(&cases, 2)?
        .into_iter()
        .map(|r| (r[0].clone(), r[1].clone()))
    {
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &lib, &log, &guard, &cmd, None)?;
        tally.check(&want, &got, &cmd);
    }
    for row in rows(&bg_cases, 3)? {
        let (want, rib, cmd) = (row[0].clone(), row[1].clone(), row[2].clone());
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &lib, &log, &guard, &cmd, Some(&rib))?;
        tally.check(&want, &got, &format!("[run_in_background={}] {}", rib, cmd));
    }

    if tally.ran == 0 {
        return Err(format!(
            "{}: no cases parsed from {} / {}",
            NAME, cases, bg_cases
        ));
    }
    if tally.fails > 0 {
        println!(
            "{}: {}/{} case(s) failed",
            NAME, tally.fails, tally.ran
        );
        return Ok(1);
    }
    println!(
        "{}: ok ({} cases across the generic ruleset and the backgrounding arm)",
        NAME, tally.ran
    );
    Ok(0)
}

fn positional(args: &[String], at: usize, default: &str) -> String {
    match args.get(at) {
        Some(a) if !a.is_empty() => a.clone(),
        _ => default.to_string(),
    }
}

// spec: guard-kit/SPEC.md §Testing — the arm reaches guard-kit through the transported kit roots
// rather than a path relative to itself: a compiled member has no `BASH_SOURCE` anchor.
fn kit_root() -> Result<String, String> {
    walk::kit_roots_abs()?
        .into_iter()
        .find(|r| r.rsplit('/').next() == Some("guard-kit"))
        .ok_or_else(|| {
            format!(
                "{}: GATE_KIT_ROOTS_HERE names no guard-kit root, so the guard this table drives \
                 and the tables themselves cannot be found",
                NAME
            )
        })
}

#[derive(Default)]
struct Tally {
    ran: usize,
    fails: usize,
}

impl Tally {
    fn check(&mut self, want: &str, got: &str, label: &str) {
        self.ran += 1;
        if got != want {
            println!("  FAIL: want '{}', got '{}' -- {}", want, got, label);
            self.fails += 1;
        }
    }
}

// spec: guard-kit/SPEC.md §Testing — the two substitutions, in the fixed order.
fn substitute(cmd: &str, root: &str) -> String {
    cmd.replace("@ROOT@", root).replace("@NL@", "\n")
}

// spec: guard-kit/SPEC.md §Testing — the table grammar, and the split reproduces bash's
// `IFS=$'\t' read -r`: tab is an IFS *whitespace* character, so leading and trailing tabs are
// stripped and a run of them is one delimiter.
fn rows(path: &str, width: usize) -> Result<Vec<Vec<String>>, String> {
    let body = std::fs::read_to_string(path)
        .map_err(|e| format!("{}: cannot read {}: {}", NAME, path, e))?;
    let mut out: Vec<Vec<String>> = Vec::new();
    for line in body.lines() {
        let cells = fields(line, width);
        if cells[0].trim_matches(' ').is_empty() || cells[0].starts_with('#') {
            continue;
        }
        out.push(cells);
    }
    Ok(out)
}

fn fields(line: &str, width: usize) -> Vec<String> {
    let mut rest = line.trim_matches('\t');
    let mut out: Vec<String> = Vec::new();
    while out.len() + 1 < width {
        match rest.find('\t') {
            Some(at) => {
                out.push(rest[..at].to_string());
                rest = rest[at..].trim_start_matches('\t');
            }
            None => break,
        }
    }
    out.push(rest.to_string());
    while out.len() < width {
        out.push(String::new());
    }
    out
}

// spec: guard-kit/SPEC.md §Testing — one case: the *subject* is still the unchanged
// `templates/bash-guard.sh`, spawned from inside the sandbox with the same four inputs the shell
// harness supplied.
fn decide(
    root: &str,
    lib: &str,
    log: &str,
    guard: &str,
    cmd: &str,
    background: Option<&str>,
) -> Result<String, String> {
    let script = r#"cd "$1" || exit 2; GUARD_KIT_LIB="$2" GUARD_KIT_LOG="$3" exec bash "$4""#;
    let done = proc::run_streamed(
        "bash",
        &["-c", script, "bash", root, lib, log, guard],
        payload(cmd, background).as_bytes(),
        Stderr::Discard,
    )?;
    // spec: guard-kit/SPEC.md §Testing — the shell form captured the guard in `$( … )`, which
    // strips every trailing newline before the ladder's emptiness arm reads it.
    let out = String::from_utf8_lossy(done.stdout())
        .trim_end_matches('\n')
        .to_string();
    Ok(classify(done.code(), &out))
}

// spec: guard-kit/SPEC.md §Testing — the hook payload, `jq -nc`'s construction moved in-crate;
// the section names both fields' readers and that no field is added.
fn payload(cmd: &str, background: Option<&str>) -> String {
    let mut input = serde_json::Map::new();
    input.insert("command".to_string(), serde_json::Value::String(cmd.to_string()));
    if background == Some("true") {
        input.insert("run_in_background".to_string(), serde_json::Value::Bool(true));
    }
    serde_json::Value::Object(
        [(
            "tool_input".to_string(),
            serde_json::Value::Object(input),
        )]
        .into_iter()
        .collect(),
    )
    .to_string()
}

// spec: guard-kit/SPEC.md §Testing — the five-way ladder in exit-code-then-substring order; the
// order is load-bearing and the section says why.
fn classify(rc: i32, out: &str) -> String {
    if rc == 2 {
        return "block".to_string();
    }
    if rc != 0 {
        return format!("exit{}", rc);
    }
    if out.contains("\"updatedInput\"") {
        return "rewrite".to_string();
    }
    if out.contains("\"additionalContext\"") {
        return "advise".to_string();
    }
    if out.contains("\"permissionDecision\":\"allow\"") {
        return "allow".to_string();
    }
    if out.is_empty() {
        return "fallthrough".to_string();
    }
    "unknown".to_string()
}

// spec: guard-kit/SPEC.md §Testing — the sandbox's five preconditions, each a case's precondition
// rather than scenery: the section enumerates them and says what turns green for the wrong reason
// when a harness builds four.
fn build_sandbox() -> Result<Sandbox, String> {
    let made = proc::run("mktemp", &["-d"])?;
    let root = made
        .stdout()
        .map(|o| String::from_utf8_lossy(o).trim().to_string())
        .filter(|r| !r.is_empty())
        .ok_or_else(|| format!("{}: cannot create a sandbox directory", NAME))?;
    let sandbox = Sandbox { root };
    let at = |rel: &str| format!("{}/{}", sandbox.root, rel);

    git(&sandbox.root, &["init", "-q"])?;
    write(&at(".gitignore"), "scratch.txt\n.tmp/\nfriction.log\n")?;
    write(&at("tracked.md"), "tracked\n")?;
    write(&at("scratch.txt"), "scratch\n")?;
    git(&sandbox.root, &["add", "tracked.md"])?;

    mkdir(&at(".tmp"))?;
    write(&at(".tmp/dead-producer.run"), "pid=2147483646 run=dead-producer\n")?;
    write(&at(".tmp/notes.txt"), "not a record\n")?;

    mkdir(&at(".claude"))?;
    write(
        &at(".claude/settings.json"),
        concat!(
            "{ \"permissions\": { \"allow\": [",
            "\"Bash(git status)\", \"Bash(ls)\", \"Bash(printf:*)\", ",
            "\"Bash(bash scripts/check-*.sh)\", \"Bash(scripts/check-*.sh *)\", ",
            "\"Bash(rm -rf .tmp/*)\", \"Bash(bash */checks/check-*.sh)\", ",
            "\"Bash(find .tmp/* -exec cat {} +)\", \"Bash(git rm -q *)\"",
            "] } }\n"
        ),
    )?;
    Ok(sandbox)
}

fn git(root: &str, args: &[&str]) -> Result<(), String> {
    let mut argv: Vec<&str> = vec!["-C", root];
    argv.extend_from_slice(args);
    let done = proc::run("git", &argv)?;
    if done.code() != Some(0) {
        return Err(format!(
            "{}: cannot build the sandbox — git {} failed",
            NAME,
            args.join(" ")
        ));
    }
    Ok(())
}

fn mkdir(path: &str) -> Result<(), String> {
    std::fs::create_dir_all(path)
        .map_err(|e| format!("{}: cannot create {}: {}", NAME, path, e))
}

fn write(path: &str, body: &str) -> Result<(), String> {
    std::fs::write(path, body).map_err(|e| format!("{}: cannot write {}: {}", NAME, path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §Testing — the ladder's order is the assertion, not its arms.
    #[test]
    fn the_classification_ladder_lets_the_first_match_win() {
        assert_eq!(classify(2, "anything"), "block");
        assert_eq!(classify(3, ""), "exit3");
        assert_eq!(
            classify(0, r#"{"updatedInput":{},"additionalContext":"x"}"#),
            "rewrite"
        );
        assert_eq!(classify(0, r#"{"additionalContext":"x"}"#), "advise");
        assert_eq!(classify(0, r#"{"permissionDecision":"allow"}"#), "allow");
        assert_eq!(classify(0, ""), "fallthrough");
        assert_eq!(classify(0, "{}"), "unknown");
    }

    // spec: guard-kit/SPEC.md §Testing — the two substitutions in their fixed order.
    #[test]
    fn both_substitutions_apply_to_a_command_cell() {
        assert_eq!(
            substitute("cat @ROOT@/f <<EOF@NL@body@NL@EOF", "/tmp/s"),
            "cat /tmp/s/f <<EOF\nbody\nEOF"
        );
    }

    // spec: guard-kit/SPEC.md §Testing — the row grammar reproduces `IFS=$'\t' read -r`.
    #[test]
    fn a_row_splits_into_exactly_its_declared_width() {
        assert_eq!(fields("block\tcd deploy && ls", 2), vec!["block", "cd deploy && ls"]);
        assert_eq!(
            fields("advise\ttrue\tbash x &", 3),
            vec!["advise", "true", "bash x &"]
        );
        assert_eq!(fields("block", 2), vec!["block", ""]);
        assert_eq!(
            fields("advise\ttrue\ta\tb", 3),
            vec!["advise", "true", "a\tb"],
            "a tab inside the command cell belongs to the last field, as bash's read leaves it"
        );
    }

    // spec: guard-kit/SPEC.md §Testing — the backgrounding flag rides beside the command.
    #[test]
    fn the_payload_carries_the_backgrounding_flag_only_when_the_row_sets_it() {
        assert_eq!(payload("ls", None), r#"{"tool_input":{"command":"ls"}}"#);
        assert_eq!(
            payload("ls", Some("false")),
            r#"{"tool_input":{"command":"ls"}}"#
        );
        assert_eq!(
            payload("ls", Some("true")),
            r#"{"tool_input":{"command":"ls","run_in_background":true}}"#
        );
    }
}
