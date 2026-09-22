// spec: gate-sdk/SPEC.md §check-action-job-ref — a backticked span matching a consumer-declared
// job pattern names a job key some walked workflow defines
use crate::actions::{self, Ev};
use crate::ere::Ere;
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const KNOB: &str = "GATE_SDK_JOB_REF_PATTERNS";

// spec: gate-sdk/SPEC.md §check-action-job-ref — the gate mints no valve, so the shared walk is
// handed a marker spelling no line can carry
const NO_VALVE: &str = "\u{0}";

// spec: gate-sdk/SPEC.md §check-action-job-ref — a single-backtick span on one line; a longer
// backtick run opens no span this gate reads
fn code_spans(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let mut open: Option<usize> = None;
    while i < b.len() {
        if b[i] != b'`' {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < b.len() && b[j] == b'`' {
            j += 1;
        }
        if j - i == 1 {
            match open {
                None => open = Some(i + 1),
                Some(s) => {
                    out.push(&line[s..i]);
                    open = None;
                }
            }
        }
        i = j;
    }
    out
}

fn is_yaml(p: &Path) -> bool {
    matches!(p.extension().and_then(|e| e.to_str()), Some("yml") | Some("yaml"))
}

// spec: gate-sdk/SPEC.md §check-action-job-ref — a workflow file's references are read off its
// full-line comments, a markdown file's off every line
fn reference_lines(text: &str, yaml: bool) -> Vec<(usize, &str)> {
    text.lines()
        .enumerate()
        .filter(|(_, l)| !yaml || l.trim_start().starts_with('#'))
        .map(|(i, l)| (i + 1, l))
        .collect()
}

fn job_keys(text: &str) -> Vec<String> {
    if !text.lines().any(|l| l.starts_with("jobs:")) {
        return Vec::new();
    }
    actions::walk_file(text, NO_VALVE)
        .into_iter()
        .filter_map(|e| match e {
            Ev::Job(name, _) => Some(name),
            _ => None,
        })
        .collect()
}

fn findings(files: &[(String, String)], pats: &[Ere]) -> (Vec<String>, usize, usize) {
    let jobs: BTreeSet<String> = files
        .iter()
        .filter(|(f, _)| is_yaml(Path::new(f)))
        .flat_map(|(_, t)| job_keys(t))
        .collect();
    let mut out = Vec::new();
    let mut refs = 0usize;
    for (f, text) in files {
        for (ln, line) in reference_lines(text, is_yaml(Path::new(f))) {
            for span in code_spans(line) {
                if !pats.iter().any(|p| p.is_match(span)) {
                    continue;
                }
                refs += 1;
                if !jobs.contains(span) {
                    out.push(format!("{}:{}: {} names no job in any walked workflow", f, ln, span));
                }
            }
        }
    }
    (out, refs, jobs.len())
}

pub fn run(args: &[String]) -> i32 {
    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    let root = Path::new(scanroot);
    if !root.is_dir() {
        eprintln!("check-action-job-ref: scan root not found: {}", scanroot);
        return 2;
    }
    let raw = match walk::knob_array(KNOB) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-action-job-ref: {} — the check could not run; treating as failure (not clean)", e);
            return 2;
        }
    };
    if raw.is_empty() {
        println!("ACTION-JOB-REF: clean ({} is empty — no span is a job reference, 0 checked)", KNOB);
        return 0;
    }
    let mut pats = Vec::new();
    for p in &raw {
        match Ere::compile(&format!("^({})$", p)) {
            Ok(e) => pats.push(e),
            Err(e) => {
                eprintln!(
                    "check-action-job-ref: {} element '{}' does not compile ({}) — the check could not run; treating as failure (not clean)",
                    KNOB, p, e
                );
                return 2;
            }
        }
    }
    let paths = match walk::find_files(root, &["md", "yml", "yaml"]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("check-action-job-ref: {} — the check could not run; treating as failure (not clean)", e);
            return 2;
        }
    };
    let mut files = Vec::new();
    for p in &paths {
        match std::fs::read_to_string(p) {
            Ok(t) => files.push((p.display().to_string(), t)),
            Err(e) => {
                eprintln!(
                    "check-action-job-ref: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                    p.display(),
                    e
                );
                return 2;
            }
        }
    }
    let (found, refs, jobs) = findings(&files, &pats);
    if !found.is_empty() {
        println!("check-action-job-ref: a backticked job reference names a job no walked workflow defines:");
        for f in &found {
            println!("  {}", f);
        }
        println!("  help: rename the reference to the job's current key, or unbacktick a sentence that");
        println!("        names a job family rather than one job. The patterns are {}.", KNOB);
        return 1;
    }
    println!(
        "ACTION-JOB-REF: clean ({} job reference(s) across {} file(s), each naming one of {} job key(s))",
        refs,
        files.len(),
        jobs
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-action-job-ref — only a single-backtick span is a reference
    #[test]
    fn spans_are_single_backtick_runs_on_one_line() {
        assert_eq!(code_spans("a `x` b `y-z`"), vec!["x", "y-z"]);
        assert_eq!(code_spans("``x`` and `y`"), vec!["y"]);
        assert!(code_spans("an `unclosed span").is_empty());
    }

    // spec: gate-sdk/SPEC.md §check-action-job-ref — a span outside every pattern is not a
    // reference, a matching one naming no job is a finding, and a workflow is read for its
    // job keys and its full-line comments
    #[test]
    fn a_matching_span_must_name_a_walked_job() {
        let wf = "jobs:\n  leg-a:\n    runs-on: x\n    steps:\n      - run: true\n  # see `leg-c`\n".to_string();
        let doc = "cites `leg-a`, `leg-b` and the slug `leg-a-notes`\n".to_string();
        let files = vec![("w.yml".to_string(), wf), ("d.md".to_string(), doc)];
        let pats = vec![Ere::compile("^(leg-[a-z])$").unwrap()];
        let (found, refs, jobs) = findings(&files, &pats);
        assert_eq!(jobs, 1);
        assert_eq!(refs, 3);
        assert_eq!(
            found,
            vec![
                "w.yml:6: leg-c names no job in any walked workflow".to_string(),
                "d.md:1: leg-b names no job in any walked workflow".to_string(),
            ]
        );
    }
}
