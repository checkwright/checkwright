// spec: guard-kit/SPEC.md §Testing — the decision-table runner, an `Arm::Run` member:
// its contract is a three-valued exit — 0 clean, 1 a verdict mismatch, 2 a harness precondition —
// which `Arm::Emit` collapses to 0-or-2.
// spec: gate-sdk/SPEC.md §The non-gate arm — an arm table member rather than a hardcoded
// top-level flag, because the member is configured: it needs the vendored guard-kit root.
use crate::proc;
use crate::programs;
use crate::walk;
use std::path::Path;

// spec: guard-kit/SPEC.md §Testing — the declared roster and its omissions are ruled there.
pub const KNOBS: &[&str] = &["GATE_SDK_KIT_DIRS", "GUARD_KIT_CONSUMER_RULES_CMD", "GUARD_KIT_CONSUMER_CASES"];

const CASES_KNOB: &str = "GUARD_KIT_CONSUMER_CASES";

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
    let cases = positional(args, 0, &format!("{}/guard-tests/cases.tsv", kit));
    let bg_cases = positional(args, 1, &format!("{}/guard-tests/background-cases.tsv", kit));
    let ps_cases = positional(args, 2, &format!("{}/guard-tests/powershell-cases.tsv", kit));
    let ps_bg_cases = positional(args, 3, &format!("{}/guard-tests/powershell-background-cases.tsv", kit));
    for f in [&cases, &bg_cases, &ps_cases, &ps_bg_cases] {
        if !Path::new(f).is_file() {
            return Err(format!("{}: missing {}", NAME, f));
        }
    }
    let sandbox = build_sandbox()?;
    let log = format!("{}/friction.log", sandbox.root);

    let mut tally = Tally::default();
    for (want, cmd) in rows(&cases, 2)?
        .into_iter()
        .map(|r| (r[0].clone(), r[1].clone()))
    {
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &log, &cmd, None, "Bash")?;
        tally.check(&want, &got, &format!("[--hook shell-guard] {}", cmd));
    }
    for row in rows(&bg_cases, 3)? {
        let (want, rib, cmd) = (row[0].clone(), row[1].clone(), row[2].clone());
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &log, &cmd, Some(&rib), "Bash")?;
        tally.check(&want, &got, &format!("[--hook shell-guard] [run_in_background={}] {}", rib, cmd));
    }
    for (want, cmd) in rows(&ps_cases, 2)?
        .into_iter()
        .map(|r| (r[0].clone(), r[1].clone()))
    {
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &log, &cmd, None, "PowerShell")?;
        tally.check(&want, &got, &format!("[--hook shell-guard] [PowerShell] {}", cmd));
    }
    for row in rows(&ps_bg_cases, 3)? {
        let (want, rib, cmd) = (row[0].clone(), row[1].clone(), row[2].clone());
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &log, &cmd, Some(&rib), "PowerShell")?;
        tally.check(&want, &got, &format!("[--hook shell-guard] [PowerShell] [run_in_background={}] {}", rib, cmd));
    }

    if tally.ran == 0 {
        return Err(format!(
            "{}: no cases parsed from {} / {} / {} / {}",
            NAME, cases, bg_cases, ps_cases, ps_bg_cases
        ));
    }
    let kit_ran = tally.ran;
    let consumer = consumer_lane(&mut tally)?;
    if tally.fails > 0 {
        println!(
            "{}: {}/{} case(s) failed",
            NAME, tally.fails, tally.ran
        );
        return Ok(1);
    }
    let consumer_note = consumer.map_or(String::new(), |(n, table)| format!(", and {} consumer case(s) from {}", n, table));
    println!(
        "{}: ok ({} cases across the generic ruleset, the backgrounding arm, the PowerShell reader and its backgrounding arm{})",
        NAME, kit_ran, consumer_note
    );
    Ok(0)
}

// spec: guard-kit/SPEC.md §Testing — the consumer lane: the consumer's rows against its own
// command, spawned directly from the arm's working directory; `None` where no table is named.
fn consumer_lane(tally: &mut Tally) -> Result<Option<(usize, String)>, String> {
    let table = crate::walk::knob_scalar(CASES_KNOB)?;
    if table.is_empty() {
        return Ok(None);
    }
    let argv = crate::guard::host::consumer_cmd()?;
    let Some((head, rest)) = argv.split_first() else {
        return Err(format!(
            "{}: {} names {} but GUARD_KIT_CONSUMER_RULES_CMD is empty, so there is no command for its rows to drive",
            NAME, CASES_KNOB, table
        ));
    };
    if !Path::new(&table).is_file() {
        return Err(format!("{}: {} names {}, which is not a readable file", NAME, CASES_KNOB, table));
    }
    let root = walk::cwd().map_err(|e| format!("{}: {}", NAME, e))?;
    let bin = running_binary()?;
    let program = programs::Program::consumer("GUARD_KIT_CONSUMER_RULES_CMD", head.as_str());
    let args: Vec<&str> = rest.iter().map(String::as_str).collect();
    let set = [("GATE_SDK_NATIVE_BIN".to_string(), bin)];
    let env = proc::ChildEnv { set: &set, unset: &[], cwd: None };
    let before = tally.ran;
    for row in rows(&table, 2)? {
        let cmd = substitute(&row[1], &root);
        let done = proc::run_with_stdin_in(&program, &args, payload(&cmd, None, "Bash").as_bytes(), &env)?;
        let out = String::from_utf8_lossy(done.streams().0).into_owned();
        let got = classify(done.reported_code(), out.trim_end_matches('\n'));
        tally.check(&row[0], &got, &format!("[consumer {}] {}", argv.join(" "), cmd));
    }
    Ok(Some((tally.ran - before, table)))
}

// spec: guard-kit/SPEC.md §Testing — the guard reads its knobs from the binary, and the
// repo-relative default names nothing from inside the sandbox, so the running binary is exported
fn running_binary() -> Result<String, String> {
    let bin = std::env::current_exe()
        .map_err(|e| format!("{}: cannot name the running binary: {}", NAME, e))?;
    Ok(walk::normalize_abs(&bin.to_string_lossy()))
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
                "{}: the kit roots name no guard-kit root, so the guard this table drives \
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

// spec: guard-kit/SPEC.md §Testing — one case, spawned from inside the sandbox with the same inputs
// the harness supplied, driving the `--hook shell-guard` member directly with no front end.
fn decide(root: &str, log: &str, cmd: &str, background: Option<&str>, tool: &str) -> Result<String, String> {
    let bin = running_binary()?;
    let set = [
        ("GUARD_KIT_LOG".to_string(), log.to_string()),
        ("GATE_SDK_NATIVE_BIN".to_string(), bin.clone()),
        ("PWD".to_string(), root.to_string()),
    ];
    let env = proc::ChildEnv { set: &set, unset: &[], cwd: Some(Path::new(root)) };
    let done = proc::run_with_stdin_in(
        &programs::CHECKWRIGHT_GATES.at(bin.clone()),
        &["--hook", "shell-guard"],
        payload(cmd, background, tool).as_bytes(),
        &env,
    )?;
    let (code, out) = (
        done.reported_code(),
        String::from_utf8_lossy(done.streams().0).into_owned(),
    );
    // spec: guard-kit/SPEC.md §Testing — the shell form captured the guard in `$( … )`, which
    // strips every trailing newline before the ladder's emptiness arm reads it.
    Ok(classify(code, out.trim_end_matches('\n')))
}

// spec: guard-kit/SPEC.md §Testing — the hook payload, `jq -nc`'s construction moved in-crate,
// carrying the tool name the member selects its reader by.
fn payload(cmd: &str, background: Option<&str>, tool: &str) -> String {
    let mut input = serde_json::Map::new();
    input.insert("command".to_string(), serde_json::Value::String(cmd.to_string()));
    if background == Some("true") {
        input.insert("run_in_background".to_string(), serde_json::Value::Bool(true));
    }
    serde_json::Value::Object(
        [
            ("tool_name".to_string(), serde_json::Value::String(tool.to_string())),
            ("tool_input".to_string(), serde_json::Value::Object(input)),
        ]
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
    let made = std::env::temp_dir().join(format!("checkwright-guard-tests.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&made);
    mkdir(&made.display().to_string())?;
    // spec: gate-sdk/SPEC.md §The crate's crosser — `sandbox.root` is composed into paths by
    // string concatenation (`at`, below), so it must be stripped, unlike a caller that only
    // compares this producer's answer against its own.
    let root = walk::canonicalize(&made)
        .map(|c| walk::normalize_abs(walk::strip_extended_prefix(&c)))
        .ok_or_else(|| format!("{}: cannot resolve the sandbox {}", NAME, made.display()))?;
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
    let done = proc::run(&programs::GIT, &argv)?;
    if let Some(report) = done.failure_report() {
        return Err(format!(
            "{}: cannot build the sandbox — git {} failed ({})",
            NAME,
            args.join(" "),
            report
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
        assert_eq!(payload("ls", None, "Bash"), r#"{"tool_input":{"command":"ls"},"tool_name":"Bash"}"#);
        assert_eq!(
            payload("ls", Some("false"), "Bash"),
            r#"{"tool_input":{"command":"ls"},"tool_name":"Bash"}"#
        );
        assert_eq!(
            payload("ls", Some("true"), "Bash"),
            r#"{"tool_input":{"command":"ls","run_in_background":true},"tool_name":"Bash"}"#
        );
        assert_eq!(payload("ls", None, "PowerShell"), r#"{"tool_input":{"command":"ls"},"tool_name":"PowerShell"}"#);
    }
}
