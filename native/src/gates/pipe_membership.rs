// spec: gate-sdk/SPEC.md §check-pipe-membership — no set producer feeds a short-circuiting reader
// in a shell file under `pipefail`
use crate::gates::path_dialect::shell_split;
use crate::{proc, programs, walk};
use std::path::Path;

const NAME: &str = "check-pipe-membership";

// spec: gate-sdk/SPEC.md §check-pipe-membership — the leading words that precede a pipeline's first
// command without being it
const LEADERS: &[&str] = &["if", "then", "else", "elif", "while", "until", "do", "!", "{", "time"];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-pipe-membership — in scope when a code line sets `pipefail`, as
// `-o pipefail` or a combined flag word ending in `o`
fn sets_pipefail(code: &str) -> bool {
    let words: Vec<&str> = code.split_whitespace().collect();
    words.contains(&"set")
        && words.windows(2).any(|w| {
            w[1] == "pipefail"
                && w[0].len() >= 2
                && w[0].starts_with('-')
                && !w[0].starts_with("--")
                && w[0].ends_with('o')
                && w[0][1..].chars().all(|c| c.is_ascii_alphabetic())
        })
}

// spec: gate-sdk/SPEC.md §check-pipe-membership — logical lines: a code half ending in `\` or `|`
// continues onto the next; each logical line keeps its first physical line's number
fn logical_lines(text: &str) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut open: Option<(usize, String)> = None;
    for (idx, raw) in text.lines().enumerate() {
        let code = shell_split(raw).code;
        let trimmed = code.trim_end();
        let (body, cont) = if let Some(b) = trimmed.strip_suffix('\\') {
            (b.to_string(), true)
        } else {
            (trimmed.to_string(), trimmed.ends_with('|') && !trimmed.ends_with("||"))
        };
        let entry = open.get_or_insert((idx, String::new()));
        entry.1.push(' ');
        entry.1.push_str(&body);
        if !cont {
            out.extend(open.take());
        }
    }
    out.extend(open);
    out
}

// spec: gate-sdk/SPEC.md §check-pipe-membership — every pipeline on a logical line as its stages,
// nested substitutions and subshells read as pipelines of their own and quoted text as content
fn pipelines(line: &str) -> Vec<Vec<String>> {
    struct Frame {
        dq: bool,
        stages: Vec<String>,
        cur: String,
    }
    fn close(f: &mut Frame, out: &mut Vec<Vec<String>>) {
        let mut stages = std::mem::take(&mut f.stages);
        stages.push(std::mem::take(&mut f.cur));
        if stages.len() > 1 {
            out.push(stages);
        }
    }
    let b = line.as_bytes();
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut stack = vec![Frame { dq: false, stages: Vec::new(), cur: String::new() }];
    let (mut i, mut sq) = (0usize, false);
    while i < b.len() {
        let c = b[i];
        let top = stack.len() - 1;
        let dq = stack[top].dq;
        if sq {
            if c == b'\'' {
                sq = false;
            }
            stack[top].cur.push(c as char);
            i += 1;
            continue;
        }
        if c == b'\\' && i + 1 < b.len() {
            stack[top].cur.push(c as char);
            stack[top].cur.push(b[i + 1] as char);
            i += 2;
            continue;
        }
        let next = b.get(i + 1).copied();
        if c == b'$' && next == Some(b'(') {
            stack[top].cur.push_str("$(…)");
            stack.push(Frame { dq: false, stages: Vec::new(), cur: String::new() });
            i += 2;
            continue;
        }
        if dq {
            if c == b'"' {
                stack[top].dq = false;
            }
            stack[top].cur.push(c as char);
            i += 1;
            continue;
        }
        match c {
            b'\'' => {
                sq = true;
                stack[top].cur.push('\'');
            }
            b'"' => {
                stack[top].dq = true;
                stack[top].cur.push('"');
            }
            b'(' => {
                stack.push(Frame { dq: false, stages: Vec::new(), cur: String::new() });
            }
            b')' if stack.len() > 1 => {
                let mut f = stack.pop().unwrap_or(Frame { dq: false, stages: Vec::new(), cur: String::new() });
                close(&mut f, &mut out);
            }
            b'|' if next == Some(b'|') => {
                close(&mut stack[top], &mut out);
                i += 1;
            }
            b'|' => {
                let s = std::mem::take(&mut stack[top].cur);
                stack[top].stages.push(s);
            }
            b'&' if next == Some(b'&') => {
                close(&mut stack[top], &mut out);
                i += 1;
            }
            b';' | b'&' => close(&mut stack[top], &mut out),
            _ => stack[top].cur.push(c as char),
        }
        i += 1;
    }
    while let Some(mut f) = stack.pop() {
        close(&mut f, &mut out);
    }
    out
}

fn words(stage: &str) -> Vec<&str> {
    let mut w: Vec<&str> = stage.split_whitespace().collect();
    while w.first().is_some_and(|x| LEADERS.contains(x)) {
        w.remove(0);
    }
    w
}

// spec: gate-sdk/SPEC.md §check-pipe-membership — a set producer: `printf` or `echo` with an array
// expansion among its arguments, or a loop's `done`
fn set_producer(stage: &str) -> bool {
    let w = words(stage);
    match w.first() {
        Some(&"done") => true,
        Some(&"printf") | Some(&"echo") => ["[@]", "[*]"].iter().any(|m| {
            stage.match_indices(m).any(|(at, _)| {
                stage[..at].rfind("${").map_or(true, |open| !stage[open + 2..].starts_with('#'))
            })
        }),
        _ => false,
    }
}

// spec: gate-sdk/SPEC.md §check-pipe-membership — a short-circuiting reader: `head`, or `grep` with
// a quiet or max-count flag, spelled long or in a short cluster
fn short_circuits(stage: &str) -> bool {
    let w = words(stage);
    match w.first() {
        Some(&"head") => true,
        Some(&"grep") => w[1..].iter().any(|a| {
            matches!(*a, "--quiet" | "--silent" | "--max-count")
                || a.starts_with("--max-count=")
                || (a.len() >= 2
                    && a.starts_with('-')
                    && !a.starts_with("--")
                    && a[1..].chars().take_while(|c| c.is_ascii_alphabetic()).any(|c| c == 'q' || c == 'm'))
        }),
        _ => false,
    }
}

struct Tally {
    in_scope: usize,
    readers: usize,
}

fn scan(path: &str, text: &str, t: &mut Tally, findings: &mut Vec<String>) {
    if !text.lines().any(|l| sets_pipefail(&shell_split(l).code)) {
        return;
    }
    t.in_scope += 1;
    for (idx, line) in logical_lines(text) {
        for stages in pipelines(&line) {
            let last = stages.last().map(String::as_str).unwrap_or("");
            if !short_circuits(last) {
                continue;
            }
            t.readers += 1;
            if set_producer(&stages[0]) {
                findings.push(format!(
                    "{}:{} — a set producer (`{}`) feeds a short-circuiting reader (`{}`) under `pipefail`, so a present member can read as absent",
                    path,
                    idx + 1,
                    words(&stages[0]).first().copied().unwrap_or(""),
                    words(last).first().copied().unwrap_or("")
                ));
            }
        }
    }
}

fn rule(_args: &[String]) -> Result<i32, String> {
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the tracked corpus degrades to empty outside a
    // work tree, so the repository is probed first and an absent one refuses
    let inside = proc::run(&programs::GIT, &["rev-parse", "--is-inside-work-tree"])
        .map(|c| c.stdout().is_some())
        .unwrap_or(false);
    if !inside {
        return Err("not a git repository — the tracked shell corpus cannot be resolved".into());
    }
    let shell = walk::tracked_shell_tree()?;
    let mut t = Tally { in_scope: 0, readers: 0 };
    let mut findings: Vec<String> = Vec::new();
    for f in &shell {
        let text = std::fs::read(Path::new(f))
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read {}: {}", f, e))?;
        scan(f, &text, &mut t, &mut findings);
    }
    if !findings.is_empty() {
        println!("PIPE-MEMBERSHIP: {} violation(s):", findings.len());
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: test membership with a `for` loop over the set or a `[[ ]]` test, never a pipeline —");
        println!("        the reader exits on its first match, the writer takes SIGPIPE, and `pipefail` makes");
        println!("        the signal the pipeline's status (gate-sdk/SPEC.md §check-pipe-membership)");
        return Ok(1);
    }
    println!(
        "PIPE-MEMBERSHIP: clean ({} shell file(s) scanned, {} under `pipefail`; {} pipeline(s) into a short-circuiting reader, none fed by a set)",
        shell.len(),
        t.in_scope,
        t.readers
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-pipe-membership — the scope test reads the flag word, never the
    // bare word `pipefail`
    #[test]
    fn pipefail_is_set_by_its_flag_word() {
        assert!(sets_pipefail("set -o pipefail"));
        assert!(sets_pipefail("set -euo pipefail"));
        assert!(!sets_pipefail("echo pipefail"));
        assert!(!sets_pipefail("set +o pipefail"));
    }

    // spec: gate-sdk/SPEC.md §check-pipe-membership — the three producer shapes red, and a nested
    // substitution or a quoted `|` neither hides nor invents a pipeline
    #[test]
    fn a_set_producer_into_a_short_circuiting_reader_is_found() {
        let red = |l: &str| {
            pipelines(l)
                .iter()
                .any(|s| short_circuits(s.last().unwrap()) && set_producer(&s[0]))
        };
        assert!(red(r#"if printf '%s\n' "${a[@]}" | grep -qxF "$x"; then"#));
        assert!(red("done | head -1"));
        assert!(red(r#"n="$(echo "${a[*]}" | grep -m1 -E "a|b")""#));
        assert!(red(r#"printf '%s\n' "${a[@]}" | grep -Fxq -- "$x" && echo y"#));
        assert!(!red(r#"printf '%s\n' "${a[@]}" | sort"#));
        assert!(!red("find . -print -quit | grep -q ."));
        assert!(!red(r#"printf '%s\n' "${a[@]}" | grep -c x"#));
        assert!(!red(r#"echo "a|b" | grep -q a"#));
        assert!(!red(r#"echo "${#a[@]}" | grep -q 0"#), "a length is one value, not a set");
    }

    // spec: gate-sdk/SPEC.md §check-pipe-membership — a pipe continued onto the next line is one pipeline
    #[test]
    fn a_trailing_pipe_joins_the_next_line() {
        let l = logical_lines("printf '%s\\n' \"${a[@]}\" |\n  grep -q x\n");
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].0, 0);
    }
}
