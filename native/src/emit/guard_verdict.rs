// spec: guard-kit/SPEC.md §guard-verdict — the decision the shell guard returns for one typed
// command, asked of the running binary or the one `--bin` names, in the tree or in the decision
// table's sandbox, through `--run-guard-tests`' own builder, spawn and ladder.
use super::run_guard_tests::{self as table, Sandbox};
use crate::hook::binding::{TOOL_BASH, TOOL_POWERSHELL};
use crate::{proc, walk};
use std::path::Path;

const TMP_KNOB: &str = "GATE_SDK_TMP_DIR";

// spec: guard-kit/SPEC.md §guard-verdict — the arm's own read and the guard's, which the child makes
pub const KNOBS: &[&str] = &{
    let mut out = [TMP_KNOB; 1 + crate::guard::host::KNOBS.len()];
    let mut i = 0;
    while i < crate::guard::host::KNOBS.len() {
        out[i + 1] = crate::guard::host::KNOBS[i];
        i += 1;
    }
    out
};

const USAGE: &str = "usage: checkwright-gates --guard-verdict [--sandbox] [--powershell] [--background] [--bin <binary>] (--stdin | [--] <command>)";

#[derive(Debug, Default, PartialEq)]
struct Ask {
    sandbox: bool,
    powershell: bool,
    background: bool,
    bin: Option<String>,
    stdin: bool,
    command: Option<String>,
}

#[derive(Debug, PartialEq)]
enum Parsed {
    Help,
    Ask(Ask),
}

pub fn run(args: &[String]) -> i32 {
    let ask = match parse(args) {
        Ok(Parsed::Help) => {
            println!("{}", USAGE);
            return 0;
        }
        Ok(Parsed::Ask(a)) => a,
        Err(e) => {
            eprintln!("checkwright-gates: --guard-verdict: {}", e);
            eprintln!("{}", USAGE);
            return 2;
        }
    };
    match verdict(ask) {
        Ok((out, err)) => {
            print!("{}", out);
            eprint!("{}", err);
            0
        }
        Err(e) => {
            eprintln!("checkwright-gates: --guard-verdict: {}", e);
            2
        }
    }
}

// spec: guard-kit/SPEC.md §guard-verdict — the grammar: `-h`/`--help` first, the flags, then one
// command operand, or `--stdin`; a leading `-` before `--` is an unknown option
fn parse(args: &[String]) -> Result<Parsed, String> {
    if matches!(args.first().map(String::as_str), Some("-h" | "--help")) {
        return Ok(Parsed::Help);
    }
    let mut ask = Ask::default();
    let mut operands: Vec<&str> = Vec::new();
    let mut escaped = false;
    let mut rest = args.iter();
    while let Some(a) = rest.next() {
        match a.as_str() {
            a if escaped => operands.push(a),
            "--" => escaped = true,
            "--sandbox" => ask.sandbox = true,
            "--powershell" => ask.powershell = true,
            "--background" => ask.background = true,
            "--stdin" => ask.stdin = true,
            "--bin" => match rest.next() {
                Some(b) if !b.is_empty() => ask.bin = Some(b.clone()),
                _ => return Err("--bin needs the path of the binary to ask".to_string()),
            },
            a if a.starts_with('-') => return Err(format!("unknown option {} (a command leading with '-' follows --)", a)),
            a => operands.push(a),
        }
    }
    match (operands.as_slice(), ask.stdin) {
        ([_, surplus, ..], _) => Err(format!("surplus operand {} (the command is one operand; quote it)", surplus)),
        ([_], true) => Err("a command operand beside --stdin (give one or the other)".to_string()),
        ([], false) => Err("no command (give it as the operand, or on stdin with --stdin)".to_string()),
        ([c], false) => {
            ask.command = Some(c.to_string());
            Ok(Parsed::Ask(ask))
        }
        ([], true) => Ok(Parsed::Ask(ask)),
    }
}

// spec: guard-kit/SPEC.md §guard-verdict — standard input whole, one trailing newline dropped
fn from_stdin(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    text.strip_suffix('\n').unwrap_or(&text).to_string()
}

// spec: guard-kit/SPEC.md §guard-verdict — the class line and the child's stdout on stdout, the
// child's stderr on stderr, each stream verbatim
fn render(code: i32, out: &str, err: &str) -> (String, String) {
    let class = table::classify(code, out.trim_end_matches('\n'));
    (format!("{}\n{}", class, out), err.to_string())
}

fn verdict(ask: Ask) -> Result<(String, String), String> {
    let here = walk::cwd()?;
    let bin = match &ask.bin {
        Some(b) => {
            let abs = walk::abs_against(&here, b);
            if !proc::is_executable(Path::new(&abs)) {
                return Err(format!("--bin {} is not an executable file", b));
            }
            abs
        }
        None => table::running_binary()?,
    };
    let raw = match ask.command {
        Some(c) => c,
        None => {
            use std::io::Read;
            let mut bytes = Vec::new();
            std::io::stdin()
                .read_to_end(&mut bytes)
                .map_err(|e| format!("cannot read the command from stdin: {}", e))?;
            from_stdin(&bytes)
        }
    };
    let top = walk::toplevel_opt()?;
    let base = top.clone().unwrap_or_else(|| here.clone());
    let tmp = walk::abs_against(&base, &walk::knob_scalar(TMP_KNOB)?);
    std::fs::create_dir_all(&tmp).map_err(|e| format!("cannot create the scratch dir {}: {}", tmp, e))?;
    let log = format!("{}/guard-verdict.{}.log", tmp, std::process::id());
    let sandbox: Option<Sandbox> = if ask.sandbox { Some(table::build_sandbox()?) } else { None };
    let root = match (&sandbox, top) {
        (Some(s), _) => s.root.clone(),
        (None, Some(t)) => t,
        (None, None) => return Err("not inside a git repository; ask with --sandbox".to_string()),
    };
    let tool = if ask.powershell { TOOL_POWERSHELL } else { TOOL_BASH };
    let payload = table::payload(&table::substitute(&raw, &root), ask.background.then_some("true"), tool);
    let answer = table::ask(&bin, &root, &log, &payload);
    let _ = std::fs::remove_file(&log);
    drop(sandbox);
    let (code, out, err) = answer?;
    Ok(render(code, &out, &err))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    fn command(a: &[&str]) -> Option<String> {
        match parse(&args(a)) {
            Ok(Parsed::Ask(ask)) => ask.command,
            other => panic!("{:?} parsed as {:?}", a, other),
        }
    }

    // spec: guard-kit/SPEC.md §guard-verdict — help first, and each grammar refusal
    #[test]
    fn the_grammar_takes_one_command_and_refuses_the_rest() {
        assert_eq!(parse(&args(&["--help"])), Ok(Parsed::Help));
        assert_eq!(parse(&args(&["-h", "x"])), Ok(Parsed::Help));
        assert_eq!(command(&["--", "-x y"]), Some("-x y".to_string()));
        assert_eq!(command(&["git status"]), Some("git status".to_string()));
        assert_eq!(command(&["--sandbox", "--", "--help"]), Some("--help".to_string()));
        for refused in [
            &[][..],
            &["--sandbox"],
            &["a", "b"],
            &["--", "a", "b"],
            &["-x", "--", "a"],
            &["--stdin", "a"],
            &["--bin"],
            &["a", "--help"],
        ] {
            assert!(parse(&args(refused)).is_err(), "{:?} must refuse", refused);
        }
        let Ok(Parsed::Ask(ask)) = parse(&args(&["--stdin", "--powershell", "--background", "--bin", "b"])) else {
            panic!("the flags must parse");
        };
        assert!(ask.stdin && ask.powershell && ask.background && ask.command.is_none());
        assert_eq!(ask.bin.as_deref(), Some("b"));
    }

    // spec: guard-kit/SPEC.md §guard-verdict — exactly one trailing newline goes
    #[test]
    fn stdin_drops_one_trailing_newline() {
        assert_eq!(from_stdin(b"cd deploy && ls\n"), "cd deploy && ls");
        assert_eq!(from_stdin(b"a\n\n"), "a\n");
        assert_eq!(from_stdin(b"a"), "a");
    }

    // spec: guard-kit/SPEC.md §guard-verdict — `@ROOT@` names whichever root the mode asks from
    #[test]
    fn the_substitutions_name_the_modes_root() {
        for root in ["/tmp/checkwright-guard-tests.1", "/work/tree"] {
            assert_eq!(table::substitute("bash @ROOT@/x.sh@NL@y", root), format!("bash {}/x.sh\ny", root));
        }
    }

    // spec: guard-kit/SPEC.md §guard-verdict — the class line, then each stream as the hook wrote it
    #[test]
    fn the_class_line_precedes_both_streams() {
        assert_eq!(render(2, "", "shell-guard: no\n"), ("block\n".to_string(), "shell-guard: no\n".to_string()));
        assert_eq!(render(0, "", ""), ("fallthrough\n".to_string(), String::new()));
        let allow = r#"{"hookSpecificOutput":{"permissionDecision":"allow"}}"#;
        assert_eq!(render(0, &format!("{}\n", allow), "").0, format!("allow\n{}\n", allow));
        assert_eq!(render(7, "x", "y"), ("exit7\nx".to_string(), "y".to_string()));
    }

    // spec: guard-kit/SPEC.md §guard-verdict — the roster is the arm's read and the guard's
    #[test]
    fn the_roster_is_the_scratch_dir_and_the_guards_knobs() {
        assert_eq!(KNOBS[0], TMP_KNOB);
        assert_eq!(&KNOBS[1..], crate::guard::host::KNOBS);
    }
}
