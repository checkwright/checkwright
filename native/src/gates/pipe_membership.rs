// spec: gate-sdk/SPEC.md §check-pipe-membership — no set producer feeds a short-circuiting reader
// in a shell file under `pipefail`
use crate::exit::Fault;
use crate::shell_lines::{self, Options, ParseError};
use crate::walk;
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
            e.status()
        }
    }
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

// spec: gate-sdk/SPEC.md §check-pipe-membership — the file is in scope when any code line sets
// `pipefail`, and then every pipeline in it is judged, the reader's scope not narrowing it
fn scan(path: &str, text: &str, t: &mut Tally, findings: &mut Vec<String>) -> Result<(), ParseError> {
    let script = shell_lines::read(text, &Options { body: &[], pipefail: false })?;
    if !script.sets_pipefail {
        return Ok(());
    }
    t.in_scope += 1;
    for p in &script.pipelines {
        let (first, last) = (&p.stages[0].text, &p.stages[p.stages.len() - 1].text);
        if !short_circuits(last) {
            continue;
        }
        t.readers += 1;
        if set_producer(first) {
            findings.push(format!(
                "{}:{} — a set producer (`{}`) feeds a short-circuiting reader (`{}`) under `pipefail`, so a present member can read as absent",
                path,
                p.line,
                words(first).first().copied().unwrap_or(""),
                words(last).first().copied().unwrap_or("")
            ));
        }
    }
    Ok(())
}

fn rule(_args: &[String]) -> Result<i32, Fault> {
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the tracked corpus degrades to empty outside a
    // work tree, so the repository is probed first and an absent one refuses
    if walk::toplevel_opt().map_err(Fault::tool)?.is_none() {
        return Err(Fault::refused("not a git repository — the tracked shell corpus cannot be resolved"));
    }
    let shell = walk::tracked_shell_tree().map_err(Fault::tool)?;
    let suites = walk::tracked_suites().map_err(Fault::tool)?;
    let mut t = Tally { in_scope: 0, readers: 0 };
    let mut findings: Vec<String> = Vec::new();
    let mut unparsed: Vec<String> = Vec::new();
    for f in shell.iter().chain(&suites) {
        let text = std::fs::read(Path::new(f))
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| Fault::input(format!("cannot read {}: {}", f, e)))?;
        if let Err(e) = scan(f, &text, &mut t, &mut findings) {
            unparsed.push(format!("{}:{}: [LINT-PARSE] unparseable span", f, e.line));
        }
    }
    // spec: gate-sdk/SPEC.md §The shell reader — a span the reader cannot parse is unparseable
    // input; the parsed files' findings still print beside it
    if !unparsed.is_empty() {
        for f in findings.iter().chain(&unparsed) {
            println!("{}", f);
        }
        return Ok(crate::exit::INPUT);
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
        "PIPE-MEMBERSHIP: clean ({} shell file(s) and {} suite(s) scanned, {} under `pipefail`; {} pipeline(s) into a short-circuiting reader, none fed by a set)",
        shell.len(),
        suites.len(),
        t.in_scope,
        t.readers
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scanned(text: &str) -> (usize, Vec<String>) {
        let (mut t, mut found) = (Tally { in_scope: 0, readers: 0 }, Vec::new());
        scan("f.sh", text, &mut t, &mut found).expect("parses");
        (t.in_scope, found)
    }

    // spec: gate-sdk/SPEC.md §check-pipe-membership — the three producer shapes red, and a nested
    // substitution or a quoted `|` neither hides nor invents a pipeline
    #[test]
    fn a_set_producer_into_a_short_circuiting_reader_is_found() {
        let red = |l: &str| !scanned(&format!("set -o pipefail\n{}\n", l)).1.is_empty();
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

    // spec: gate-sdk/SPEC.md §check-pipe-membership — a pipe continued onto the next line is one
    // pipeline, named at its first line, and a file setting no `pipefail` is out of scope
    #[test]
    fn a_trailing_pipe_joins_the_next_line() {
        let (scope, found) = scanned("set -o pipefail\nprintf '%s\\n' \"${a[@]}\" |\n  grep -q x\n");
        assert_eq!(scope, 1);
        assert_eq!(found.len(), 1);
        assert!(found[0].starts_with("f.sh:2 "), "{}", found[0]);
        assert_eq!(scanned("printf '%s\\n' \"${a[@]}\" | grep -q x\n"), (0, Vec::new()));
    }
}
