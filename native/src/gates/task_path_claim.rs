// spec: canon-kit/SPEC.md §check-task-path-claim — every ticked task in a configured task list
// names only paths that exist
use crate::spec;
use crate::walk;
use std::path::Path;

const LISTS_KNOB: &str = "CANON_KIT_TASK_LIST_GLOBS";
const VALVE: &str = "task-path-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-task-path-claim: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let globs = spec::knob_array_pub(LISTS_KNOB)?;
    if globs.is_empty() {
        println!(
            "TASK-PATH-CLAIM: clean (no task list is configured: {} is empty)",
            LISTS_KNOB
        );
        return Ok(0);
    }
    let lists = task_lists(root, &globs)?;

    let mut findings: Vec<(String, Finding)> = Vec::new();
    let mut tally = Tally::default();
    for list in &lists {
        let text = spec::read_text(Path::new(list)).map_err(|e| format!("cannot read task list {}: {}", list, e))?;
        let scan = scan(&text, &|tok: &str| walk::child(Path::new(root), tok).exists());
        tally.ticked += scan.tally.ticked;
        tally.paths += scan.tally.paths;
        tally.valved += scan.tally.valved;
        findings.extend(scan.findings.into_iter().map(|f| (list.clone(), f)));
    }

    if !findings.is_empty() {
        println!(
            "check-task-path-claim: {} path(s) a ticked task names that do not exist:",
            findings.len()
        );
        for (list, f) in &findings {
            let valve = if f.empty_valve {
                " (its task-path-exempt valve carries no reason)"
            } else {
                ""
            };
            println!("  {}:{}: `{}` does not exist, in: {}{}", list, f.line, f.token, f.task, valve);
        }
        println!("  help: restore the path, correct the task to name the path it produced, or untick a task that is not done.");
        println!(
            "  help: a path absent by design takes '<!-- {} <reason> -->' on the task's line or the one above.",
            VALVE
        );
        return Ok(1);
    }
    println!(
        "TASK-PATH-CLAIM: clean ({} task list(s), {} ticked task(s), {} path(s), {} valved; every ticked task names only paths that exist)",
        lists.len(),
        tally.ticked,
        tally.paths,
        tally.valved
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-task-path-claim — a task list is a markdown file the knob's globs
// match, through the pruning walk
pub(crate) fn task_lists(root: &str, globs: &[String]) -> Result<Vec<String>, String> {
    let mut lists: Vec<String> = walk::glob_corpus(Path::new(root), globs)?
        .into_iter()
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "md"))
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    lists.sort();
    lists.dedup();
    Ok(lists)
}

#[derive(Debug)]
struct Finding {
    line: usize,
    token: String,
    task: String,
    empty_valve: bool,
}

#[derive(Default)]
struct Tally {
    ticked: usize,
    paths: usize,
    valved: usize,
}

struct Scan {
    findings: Vec<Finding>,
    tally: Tally,
}

// spec: canon-kit/SPEC.md §check-task-path-claim — the valve's reason, on the task's line or the
// one above
pub(crate) fn valve_reason(line: &str, valve: &str) -> Option<String> {
    let at = line.find(valve)?;
    let rest = &line[at + valve.len()..];
    let rest = rest.split("-->").next().unwrap_or(rest);
    Some(rest.trim().to_string())
}

// spec: canon-kit/SPEC.md §check-task-path-claim — a ticked task's marker and the column its
// continuation lines indent past
fn ticked(line: &str) -> Option<usize> {
    let indent = line.len() - line.trim_start_matches([' ', '\t']).len();
    let rest = &line[indent..];
    let mut b = rest.bytes();
    let marker = b.next()?;
    if !matches!(marker, b'-' | b'*' | b'+') || b.next()? != b' ' {
        return None;
    }
    let box_ = rest[2..].trim_start_matches(' ');
    (box_.starts_with("[x]") || box_.starts_with("[X]")).then_some(indent)
}

fn scan(text: &str, exists: &dyn Fn(&str) -> bool) -> Scan {
    let raw: Vec<&str> = text.lines().collect();
    let mut out = Scan {
        findings: Vec::new(),
        tally: Tally::default(),
    };
    let mut fence = spec::Fence::default();
    let mut i = 0usize;
    while i < raw.len() {
        let line = raw[i];
        if fence.delimits(line) {
            i += 1;
            continue;
        }
        let Some(indent) = (!fence.is_open()).then(|| ticked(line)).flatten() else {
            i += 1;
            continue;
        };
        out.tally.ticked += 1;
        let mut body: Vec<&str> = vec![line];
        let mut j = i + 1;
        while j < raw.len() {
            let next = raw[j];
            let lead = next.len() - next.trim_start_matches([' ', '\t']).len();
            if spec::is_blank(next) || lead <= indent || spec::is_list_item(next) || spec::fence_opening(next).is_some() {
                break;
            }
            body.push(next);
            j += 1;
        }
        let valve = [Some(i), i.checked_sub(1)]
            .iter()
            .flatten()
            .find_map(|&k| valve_reason(raw[k], VALVE));
        let exempt = valve.as_deref().is_some_and(|r| !r.is_empty());
        let empty_valve = valve.as_deref() == Some("");
        let task = line.trim().to_string();
        for (k, part) in body.iter().enumerate() {
            for tok in path_tokens(part) {
                out.tally.paths += 1;
                if exists(&tok) {
                    continue;
                }
                if exempt {
                    out.tally.valved += 1;
                    continue;
                }
                out.findings.push(Finding {
                    line: i + k + 1,
                    token: tok,
                    task: task.clone(),
                    empty_valve,
                });
            }
        }
        i = j;
    }
    out
}

fn path_byte(c: u8) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, b'.' | b'_' | b'/' | b'@' | b'+' | b'-')
}

// spec: canon-kit/SPEC.md §check-task-path-claim — the path-token grammar: a maximal run of the
// path alphabet, its trailing sentence punctuation dropped, holding a non-leading `/` and ending in
// `/` or an extension, and neither rooted, scheme-bearing, climbing nor a placeholder
fn path_tokens(line: &str) -> Vec<String> {
    let b = line.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if !path_byte(b[i]) {
            i += 1;
            continue;
        }
        let s = i;
        while i < b.len() && path_byte(b[i]) {
            i += 1;
        }
        let e = i;
        let before = s.checked_sub(1).map(|k| b[k]);
        let after = b.get(e).copied();
        let mut tok = &line[s..e];
        while let Some(t) = tok.strip_suffix(['.', ',', ';', ':']) {
            tok = t;
        }
        let placeholder = |c: Option<u8>| matches!(c, Some(b'[' | b'{' | b'<' | b'*'));
        if placeholder(before) || placeholder(after) || before == Some(b'~') {
            continue;
        }
        if is_path(tok) {
            out.push(tok.to_string());
        }
    }
    out
}

fn is_path(tok: &str) -> bool {
    if walk::path_root(tok).is_some() || tok.contains("//") || !tok[1.min(tok.len())..].contains('/') {
        return false;
    }
    if tok.split('/').any(|seg| seg == "..") {
        return false;
    }
    if tok.ends_with('/') {
        return true;
    }
    let last = tok.rsplit('/').next().unwrap_or("");
    match last.rsplit_once('.') {
        Some((_, ext)) => !ext.is_empty() && ext.bytes().all(|c| c.is_ascii_alphanumeric()),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_token_needs_an_inner_slash_and_an_extension_or_a_trailing_slash() {
        let got = path_tokens("- [x] T005 [P] [US1] Build it in src/models/user.py, and `lib/`.");
        assert_eq!(got, vec!["src/models/user.py", "lib/"]);
        for not in [
            "a/b",
            "/abs/path.py",
            "~/notes/x.py",
            "https://x.dev/a.py",
            "a//b.py",
            "../up/x.py",
            "a/../x.py",
            "<dir>/x.py",
            "src/{name}.py",
            "src/*.py",
            "[a]/x.py",
            "file.py",
            "1.2",
            "v1.2/x",
        ] {
            assert!(path_tokens(not).is_empty(), "{:?} read as a path", not);
        }
    }

    #[test]
    fn only_a_ticked_task_outside_a_fence_is_read_with_its_continuation() {
        let text = "- [ ] open src/a.py\n* [X] done src/b.py\n  and src/c.py\n\n+ [x] also\n  - [x] nested src/d.py\n```\n- [x] quoted src/e.py\n```\n";
        let s = scan(text, &|_| false);
        let got: Vec<(usize, &str)> = s.findings.iter().map(|f| (f.line, f.token.as_str())).collect();
        assert_eq!(got, vec![(2, "src/b.py"), (3, "src/c.py"), (6, "src/d.py")]);
        assert_eq!(s.tally.ticked, 3);
    }

    #[test]
    fn a_valve_needs_its_reason_and_an_existing_path_holds() {
        let text = "<!-- task-path-exempt: deleted it -->\n- [x] remove src/old.py\n\n<!-- task-path-exempt: -->\n- [x] remove src/gone.py\n- [x] keep src/here.py\n";
        let s = scan(text, &|t| t == "src/here.py");
        let got: Vec<(usize, bool)> = s.findings.iter().map(|f| (f.line, f.empty_valve)).collect();
        assert_eq!(got, vec![(5, true)]);
        assert_eq!(s.tally.valved, 1);
        assert_eq!(s.tally.paths, 3);
    }
}
