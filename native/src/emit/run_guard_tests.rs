// spec: guard-kit/SPEC.md §Testing — the decision-table runner, an `Arm::Run` member:
// its contract is a three-valued exit — 0 clean, 1 a verdict mismatch, 2 a harness precondition —
// which `Arm::Emit` collapses to 0-or-2.
// spec: gate-sdk/SPEC.md §The non-gate arm — an arm table member rather than a hardcoded
// top-level flag, because the member is configured: it needs the vendored guard-kit root.
use super::guard_ps_variants;
use super::guard_variants::{self, Admit, Held, Ledger, Variant};
use crate::hook::binding::{TOOL_BASH, TOOL_POWERSHELL};
use crate::proc;
use crate::programs;
use crate::walk;
use std::path::Path;

// spec: guard-kit/SPEC.md §Testing — the declared roster and its omissions are ruled there.
pub const KNOBS: &[&str] =
    &["GATE_SDK_KIT_DIRS", "GUARD_KIT_CONSUMER_RULES_CMD", "GUARD_KIT_CONSUMER_CASES", LIMITS_KNOB];

const CASES_KNOB: &str = "GUARD_KIT_CONSUMER_CASES";
const LIMITS_KNOB: &str = "GUARD_KIT_CONSUMER_VARIANT_LIMITS";

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

// spec: guard-kit/SPEC.md §Testing — the PowerShell re-spellings a PowerShell host parses on the
// x64 Windows leg: every derived row of the two PowerShell tables as `<transform>\t<base>\t<variant>`.
pub fn emit_ps_variants(_: &[String]) -> Result<String, String> {
    ps_variant_lines(&kit_root()?)
}

fn ps_variant_lines(kit: &str) -> Result<String, String> {
    let mut out = String::new();
    for (table, width) in [("powershell-cases.tsv", 2), ("powershell-background-cases.tsv", 3)] {
        let path = format!("{}/guard-tests/{}", kit, table);
        for row in rows(&path, width)? {
            let base = &row[width - 1];
            for v in guard_ps_variants::derive(base) {
                out.push_str(&format!("{}\t{}\t{}\n", v.transform, base, v.cell));
            }
        }
    }
    Ok(out)
}

fn execute(args: &[String]) -> Result<i32, String> {
    let kit = kit_root()?;
    let cases = positional(args, 0, &format!("{}/guard-tests/cases.tsv", kit));
    let bg_cases = positional(args, 1, &format!("{}/guard-tests/background-cases.tsv", kit));
    let ps_cases = positional(args, 2, &format!("{}/guard-tests/powershell-cases.tsv", kit));
    let ps_bg_cases = positional(args, 3, &format!("{}/guard-tests/powershell-background-cases.tsv", kit));
    let limits = positional(args, 4, &format!("{}/guard-tests/variant-limits.tsv", kit));
    for f in [&cases, &bg_cases, &ps_cases, &ps_bg_cases, &limits] {
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
        let got = decide(&sandbox.root, &log, &cmd, None, TOOL_BASH)?;
        tally.check(&want, &got, &format!("[--hook shell-guard] {}", cmd));
    }
    for row in rows(&bg_cases, 3)? {
        let (want, rib, cmd) = (row[0].clone(), row[1].clone(), row[2].clone());
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &log, &cmd, Some(&rib), TOOL_BASH)?;
        tally.check(&want, &got, &format!("[--hook shell-guard] [run_in_background={}] {}", rib, cmd));
    }
    for (want, cmd) in rows(&ps_cases, 2)?
        .into_iter()
        .map(|r| (r[0].clone(), r[1].clone()))
    {
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &log, &cmd, None, TOOL_POWERSHELL)?;
        tally.check(&want, &got, &format!("[--hook shell-guard] [PowerShell] {}", cmd));
    }
    for row in rows(&ps_bg_cases, 3)? {
        let (want, rib, cmd) = (row[0].clone(), row[1].clone(), row[2].clone());
        let cmd = substitute(&cmd, &sandbox.root);
        let got = decide(&sandbox.root, &log, &cmd, Some(&rib), TOOL_POWERSHELL)?;
        tally.check(&want, &got, &format!("[--hook shell-guard] [PowerShell] [run_in_background={}] {}", rib, cmd));
    }

    if tally.ran == 0 {
        return Err(format!(
            "{}: no cases parsed from {} / {} / {} / {}",
            NAME, cases, bg_cases, ps_cases, ps_bg_cases
        ));
    }
    let kit_ran = tally.ran;
    let tables = [(cases.as_str(), false, TOOL_BASH), (bg_cases.as_str(), true, TOOL_BASH), (ps_cases.as_str(), false, TOOL_POWERSHELL), (ps_bg_cases.as_str(), true, TOOL_POWERSHELL)];
    let (derived, limit_rows) = variant_lane(&mut tally, &sandbox.root, &log, &tables, &limits)?;
    let consumer = consumer_lane(&mut tally)?;
    if tally.fails > 0 {
        println!(
            "{}: {}/{} case(s) failed",
            NAME, tally.fails, tally.ran
        );
        return Ok(1);
    }
    let consumer_note = consumer.map_or(String::new(), |c| {
        let derived_note = c.derived.map_or(String::new(), |(n, limit_rows, limits)| {
            format!(" with {} derived re-spelling(s) under {} limit row(s) from {}", n, limit_rows, limits)
        });
        format!(", and {} consumer case(s) from {}{}", c.rows, c.table, derived_note)
    });
    println!(
        "{}: ok ({} cases across the generic ruleset, the backgrounding arm, the PowerShell reader and its backgrounding arm, and {} derived row(s) of the four tables under {} limit row(s){})",
        NAME, kit_ran, derived, limit_rows, consumer_note
    );
    Ok(0)
}

// spec: guard-kit/SPEC.md §Testing — the variant lane: each row's re-spellings fed as the row is
// and held to the one-sided relation, each bash row's added writes held to never being granted, a
// failure green only under a limit row held from both sides.
fn variant_lane(
    tally: &mut Tally,
    root: &str,
    log: &str,
    tables: &[(&str, bool, &str)],
    limits: &str,
) -> Result<(usize, usize), String> {
    let mut ledger =
        Ledger::parse(&rows(limits, 3)?, limits).map_err(|e| format!("{}: {}", NAME, e))?;
    let mut derived = 0;
    for &(table, flagged, tool) in tables {
        let base = sectioned(table, if flagged { 3 } else { 2 })?;
        let authored: std::collections::HashSet<(&str, &str)> =
            base.iter().map(|(_, r)| if flagged { (r[1].as_str(), r[2].as_str()) } else { ("", r[1].as_str()) }).collect();
        for (section, row) in &base {
            let (want, flag, cell) = if flagged { (&row[0], row[1].as_str(), &row[2]) } else { (&row[0], "", &row[1]) };
            let (respelled, added) = if tool == TOOL_BASH {
                (guard_variants::derive(cell), guard_variants::added_writes(cell))
            } else {
                (guard_ps_variants::derive(cell), Vec::new())
            };
            let all = respelled.iter().map(|v| (v, false)).chain(added.iter().map(|v| (v, true)));
            for (variant, is_added) in all {
                if authored.contains(&(flag, variant.cell.as_str())) {
                    continue;
                }
                let cmd = substitute(&variant.cell, root);
                let got = decide(root, log, &cmd, flagged.then_some(flag), tool)?;
                derived += 1;
                tally.ran += 1;
                let held = if is_added { guard_variants::added_relation(&got) } else { guard_variants::relation(want, &got) };
                let Some(why) = unadmitted(&mut ledger, held, section.as_deref(), variant, &got) else {
                    continue;
                };
                let tool_note = if tool == TOOL_BASH { "" } else { " [PowerShell]" };
                let flag_note = if flagged { format!(" [run_in_background={}]", flag) } else { String::new() };
                println!(
                    "  FAIL: want '{}', got '{}' -- [--hook shell-guard]{} [variant {} of {}, {}]{} {}",
                    want, got, tool_note, variant.transform, rule_label(section.as_deref()), why, flag_note, cmd
                );
                tally.fails += 1;
            }
        }
    }
    for finding in ledger.stale() {
        println!("  FAIL: {} -- {}", finding, limits);
        tally.fails += 1;
    }
    tally.ran += ledger.len();
    Ok((derived, ledger.len()))
}

fn unadmitted(ledger: &mut Ledger, held: Held, section: Option<&str>, variant: &Variant, got: &str) -> Option<&'static str> {
    match held {
        Held::Holds => None,
        Held::Never => Some("no limit admits this class"),
        Held::Limitable => match ledger.admit(section, &variant.transform, got) {
            Admit::Admitted => None,
            Admit::NoLimit => Some("no limit row"),
            Admit::BelowFloor => Some("below its limit row's floor"),
        },
    }
}

fn rule_label(section: Option<&str>) -> String {
    section.map_or("no rule section".to_string(), |s| format!("rule {}", s))
}

struct Consumer {
    rows: usize,
    table: String,
    derived: Option<(usize, usize, String)>,
}

// spec: guard-kit/SPEC.md §Testing — the consumer lane: the consumer's rows against its own
// command, spawned directly from the arm's working directory; `None` where no table is named, and
// the rows' bash re-spellings derived only where the consumer names its limits table.
fn consumer_lane(tally: &mut Tally) -> Result<Option<Consumer>, String> {
    let table = crate::walk::knob_scalar(CASES_KNOB)?;
    let limits = crate::walk::knob_scalar(LIMITS_KNOB)?;
    if table.is_empty() && !limits.is_empty() {
        return Err(format!(
            "{}: {} names {} but {} is empty, so there are no rows to derive",
            NAME, LIMITS_KNOB, limits, CASES_KNOB
        ));
    }
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
    let mut ledger = None;
    if !limits.is_empty() {
        if !Path::new(&limits).is_file() {
            return Err(format!("{}: {} names {}, which is not a readable file", NAME, LIMITS_KNOB, limits));
        }
        let parsed = Ledger::parse(&rows(&limits, 3)?, &limits).map_err(|e| format!("{}: {}: {}", NAME, LIMITS_KNOB, e))?;
        ledger = Some(parsed);
    }
    let ask = |cmd: &str| -> Result<String, String> {
        let done = proc::run_with_stdin_in(&program, &args, payload(cmd, None, TOOL_BASH).as_bytes(), &env)?;
        let out = String::from_utf8_lossy(done.streams().0).into_owned();
        Ok(classify(done.reported_code(), out.trim_end_matches('\n')))
    };
    let label = format!("[consumer {}]", argv.join(" "));
    let base = sectioned(&table, 2)?;
    for (_, row) in &base {
        let cmd = substitute(&row[1], &root);
        tally.check(&row[0], &ask(&cmd)?, &format!("{} {}", label, cmd));
    }
    let Some(mut ledger) = ledger else {
        return Ok(Some(Consumer { rows: base.len(), table, derived: None }));
    };
    let authored: std::collections::HashSet<&str> = base.iter().map(|(_, r)| r[1].as_str()).collect();
    let mut derived = 0;
    for (section, row) in &base {
        for variant in guard_variants::derive(&row[1]) {
            if authored.contains(variant.cell.as_str()) {
                continue;
            }
            let cmd = substitute(&variant.cell, &root);
            let got = ask(&cmd)?;
            derived += 1;
            tally.ran += 1;
            let held = guard_variants::relation(&row[0], &got);
            let Some(why) = unadmitted(&mut ledger, held, section.as_deref(), &variant, &got) else {
                continue;
            };
            println!(
                "  FAIL: want '{}', got '{}' -- {} [variant {} of {}, {}] {}",
                row[0], got, label, variant.transform, rule_label(section.as_deref()), why, cmd
            );
            tally.fails += 1;
        }
    }
    for finding in ledger.stale() {
        println!("  FAIL: {} -- {}", finding, limits);
        tally.fails += 1;
    }
    tally.ran += ledger.len();
    Ok(Some(Consumer { rows: base.len(), table, derived: Some((derived, ledger.len(), limits)) }))
}

// spec: guard-kit/SPEC.md §Testing — the guard reads its knobs from the binary, and the
// repo-relative default names nothing from inside the sandbox, so the running binary is exported
fn running_binary() -> Result<String, String> {
    let bin = crate::proc::self_image()
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

// spec: guard-kit/SPEC.md §Testing — the three substitutions, in the fixed order.
fn substitute(cmd: &str, root: &str) -> String {
    cmd.replace("@ROOT@", root).replace("@NL@", "\n").replace("@CR@", "\r")
}

// spec: guard-kit/SPEC.md §Testing — the table grammar, and the split reproduces bash's
// `IFS=$'\t' read -r`: tab is an IFS *whitespace* character, so leading and trailing tabs are
// stripped and a run of them is one delimiter.
fn rows(path: &str, width: usize) -> Result<Vec<Vec<String>>, String> {
    Ok(sectioned(path, width)?.into_iter().map(|(_, cells)| cells).collect())
}

// spec: guard-kit/SPEC.md §Testing — each row beside the rule section it sits in, a section
// opening at a comment line beginning `# rule` and a backticked rule name.
fn sectioned(path: &str, width: usize) -> Result<Vec<Sectioned>, String> {
    let body = std::fs::read_to_string(path)
        .map_err(|e| format!("{}: cannot read {}: {}", NAME, path, e))?;
    let mut section: Option<String> = None;
    let mut out = Vec::new();
    for line in body.lines() {
        if let Some(name) = section_name(line) {
            section = Some(name.to_string());
        }
        let cells = fields(line, width);
        if cells[0].trim_matches(' ').is_empty() || cells[0].starts_with('#') {
            continue;
        }
        out.push((section.clone(), cells));
    }
    Ok(out)
}

type Sectioned = (Option<String>, Vec<String>);

fn section_name(line: &str) -> Option<&str> {
    let name = line.strip_prefix("# rule `")?.split('`').next()?;
    (!name.is_empty()).then_some(name)
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
    use crate::hook::binding::{BACKGROUND, COMMAND, TOOL_INPUT, TOOL_NAME};
    let mut input = serde_json::Map::new();
    input.insert(COMMAND.to_string(), serde_json::Value::String(cmd.to_string()));
    if background == Some("true") {
        input.insert(BACKGROUND.to_string(), serde_json::Value::Bool(true));
    }
    serde_json::Value::Object(
        [
            (TOOL_NAME.to_string(), serde_json::Value::String(tool.to_string())),
            (TOOL_INPUT.to_string(), serde_json::Value::Object(input)),
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
    use crate::hook::binding::{ADDITIONAL_CONTEXT, DECISION_ALLOW, PERMISSION_DECISION, UPDATED_INPUT};
    if out.contains(&format!("\"{}\"", UPDATED_INPUT)) {
        return "rewrite".to_string();
    }
    if out.contains(&format!("\"{}\"", ADDITIONAL_CONTEXT)) {
        return "advise".to_string();
    }
    if out.contains(&format!("\"{}\":\"{}\"", PERMISSION_DECISION, DECISION_ALLOW)) {
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
    write(&at("two words.md"), "tracked\n")?;
    write(&at("scratch.txt"), "scratch\n")?;
    git(&sandbox.root, &["add", "tracked.md", "two words.md"])?;

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

    // spec: guard-kit/SPEC.md §Testing — the three substitutions in their fixed order.
    #[test]
    fn every_substitution_applies_to_a_command_cell() {
        assert_eq!(
            substitute("cat @ROOT@/f <<EOF@NL@body@NL@EOF", "/tmp/s"),
            "cat /tmp/s/f <<EOF\nbody\nEOF"
        );
        assert_eq!(substitute("ls >&2@CR@x", "/tmp/s"), "ls >&2\rx");
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

    // spec: guard-kit/SPEC.md §Testing — a section opens at `# rule` and a backticked name.
    #[test]
    fn a_section_opens_at_a_rule_comment_and_nowhere_else() {
        assert_eq!(section_name("# rule `cd_compound` — cd in a compound command"), Some("cd_compound"));
        assert_eq!(section_name("# rule `git_c_root`'s arm (d) — an export"), Some("git_c_root"));
        assert_eq!(section_name("#          rule `append_scratch`'s create grant reaches it"), None);
        assert_eq!(section_name("# rules `a` and `b`"), None);
        assert_eq!(section_name("block\tcd deploy && ls"), None);
    }

    // spec: guard-kit/SPEC.md §Testing — the backgrounding flag rides beside the command.
    #[test]
    fn the_payload_carries_the_backgrounding_flag_only_when_the_row_sets_it() {
        assert_eq!(payload("ls", None, TOOL_BASH), r#"{"tool_input":{"command":"ls"},"tool_name":"Bash"}"#);
        assert_eq!(
            payload("ls", Some("false"), TOOL_BASH),
            r#"{"tool_input":{"command":"ls"},"tool_name":"Bash"}"#
        );
        assert_eq!(
            payload("ls", Some("true"), TOOL_BASH),
            r#"{"tool_input":{"command":"ls","run_in_background":true},"tool_name":"Bash"}"#
        );
        assert_eq!(payload("ls", None, TOOL_POWERSHELL), r#"{"tool_input":{"command":"ls"},"tool_name":"PowerShell"}"#);
    }

    // spec: guard-kit/SPEC.md §Testing — every line the Windows leg parses is a transform, an
    // eligible base and a variant the deriver made of it; an unreadable table refuses
    #[test]
    fn each_powershell_variant_line_names_its_transform_base_and_variant() {
        let kit = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../guard-kit");
        let lines = ps_variant_lines(&kit.display().to_string()).unwrap();
        let mut n = 0;
        for line in lines.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            assert_eq!(f.len(), 3, "{}", line);
            assert!(guard_ps_variants::TRANSFORMS.contains(&f[0]), "{}", line);
            assert!(guard_ps_variants::derive(f[1]).iter().any(|v| v.transform == f[0] && v.cell == f[2]), "{}", line);
            n += 1;
        }
        assert!(n > 0);
        assert!(lines.lines().any(|l| l.starts_with("ps-alias-lead\t")));
        assert!(ps_variant_lines("/nonexistent").is_err());
    }
}
