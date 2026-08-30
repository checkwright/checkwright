// spec: evidence-kit/SPEC.md §check-battery-roster — the runner doc's battery-roster block holds
// name-set parity with EVIDENCE_KIT_SUITES, both directions
use crate::fresh;
use crate::walk;
use std::path::Path;

const BEGIN: &str = "<!-- battery-roster:begin -->";
const END: &str = "<!-- battery-roster:end -->";

// spec: evidence-kit/SPEC.md §check-battery-roster — the awk line normalization: drop a trailing
// `#` annotation, trim spaces/tabs (and a trailing CR), then squeeze runs of them to one space
fn normalize_line(raw: &str) -> String {
    let cut = raw.split('#').next().unwrap_or("");
    let trimmed = cut
        .trim_start_matches([' ', '\t'])
        .trim_end_matches([' ', '\t', '\r']);
    let mut out = String::new();
    let mut gap = false;
    for c in trimmed.chars() {
        if c == ' ' || c == '\t' {
            gap = true;
            continue;
        }
        if gap && !out.is_empty() {
            out.push(' ');
        }
        gap = false;
        out.push(c);
    }
    out
}

// spec: evidence-kit/SPEC.md §check-battery-roster — a roster line is a lowercase-led command
// word followed by at least one more word, which is what `^[a-z][a-z0-9_-]* ` selects
fn is_roster_line(line: &str) -> bool {
    let Some(rest) = line.strip_prefix(|c: char| c.is_ascii_lowercase()) else {
        return false;
    };
    let tail = rest.trim_start_matches(|c: char| {
        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-'
    });
    tail.starts_with(' ')
}

// spec: evidence-kit/SPEC.md §check-battery-roster — a suite's documented invocation is its run
// command minus a leading `env` and its VAR=value assignments, the validate harness's environment
// being no part of what a contributor types
fn normalize_invocation(cmd: &str) -> String {
    let mut head = true;
    let mut out: Vec<&str> = Vec::new();
    for tok in cmd.split_whitespace() {
        if head {
            if tok == "env" || assignment(tok) {
                continue;
            }
            head = false;
        }
        out.push(tok);
    }
    out.join(" ")
}

fn assignment(tok: &str) -> bool {
    let Some(eq) = tok.find('=') else { return false };
    let name = &tok[..eq];
    let mut cs = name.chars();
    match cs.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    cs.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

fn inner(args: &[String]) -> Result<i32, String> {
    let doc = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => {
            let top = walk::toplevel_opt().map_err(|e| format!("check-battery-roster: {}", e))?;
            let Some(top) = top else {
                return Err(
                    "check-battery-roster: not a git repository and no runner-doc argument".into(),
                );
            };
            let name = walk::knob_scalar("EVIDENCE_KIT_RUNNER_DOC")
                .map_err(|e| format!("check-battery-roster: {}", e))?;
            format!("{}/{}", top, name)
        }
    };
    if !Path::new(&doc).is_file() {
        return Err(format!("check-battery-roster: runner doc not found: {}", doc));
    }

    let suites = walk::knob_array("EVIDENCE_KIT_SUITES")
        .map_err(|e| format!("check-battery-roster: {}", e))?;
    if suites.is_empty() {
        return Err(format!(
            "check-battery-roster: EVIDENCE_KIT_SUITES is empty — no suite roster to hold {} against",
            doc
        ));
    }

    let text = std::fs::read(&doc)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("check-battery-roster: cannot read {}: {}", doc, e))?;
    if !text.contains(BEGIN) {
        return Err(format!(
            "check-battery-roster: no '{}' marker block in {}",
            BEGIN, doc
        ));
    }

    let mut roster: Vec<(String, usize)> = Vec::new();
    let mut inside = false;
    for (idx, raw) in fresh::file_lines(&text).iter().enumerate() {
        let line = normalize_line(raw);
        if line == BEGIN {
            inside = true;
            continue;
        }
        if line == END {
            inside = false;
            continue;
        }
        if inside && is_roster_line(&line) {
            // spec: evidence-kit/SPEC.md §check-battery-roster — a repeated line keeps the last
            // number, the associative-array assignment the shell form makes
            match roster.iter_mut().find(|(c, _)| *c == line) {
                Some(slot) => slot.1 = idx + 1,
                None => roster.push((line, idx + 1)),
            }
        }
    }

    // spec: evidence-kit/SPEC.md §check-battery-roster — a suite with no EVIDENCE_KIT_RUN_<suite>
    // has no documented invocation to compare; run-validate already refuses it, so reporting it
    // here would send the reader to the doc to fix a config bug
    let family = walk::knob_prefix("EVIDENCE_KIT_RUN_");
    let mut suite_of: Vec<(String, String)> = Vec::new();
    for s in &suites {
        let cmd = normalize_invocation(&walk::knob_in_family(&family, s).unwrap_or_default());
        if cmd.is_empty() {
            continue;
        }
        match suite_of.iter_mut().find(|(c, _)| *c == cmd) {
            Some(slot) => slot.1 = s.clone(),
            None => suite_of.push((cmd, s.clone())),
        }
    }

    let mut findings: Vec<String> = Vec::new();
    for (cmd, suite) in &suite_of {
        if !roster.iter().any(|(c, _)| c == cmd) {
            findings.push(format!(
                "{}: suite '{}' is absent from the battery-roster block — no line reads '{}'",
                doc, suite, cmd
            ));
        }
    }
    for (cmd, lineno) in &roster {
        if !suite_of.iter().any(|(c, _)| c == cmd) {
            findings.push(format!(
                "{}:{}: roster line runs no configured suite: '{}'",
                doc, lineno, cmd
            ));
        }
    }

    if !findings.is_empty() {
        println!("check-battery-roster: the battery-roster block is out of parity with EVIDENCE_KIT_SUITES:");
        let mut lines: Vec<String> = findings.iter().map(|f| format!("  {}", f)).collect();
        lines.sort();
        for l in &lines {
            println!("{}", l);
        }
        println!("  help: keep the block in name-set parity with the configured suites — add the");
        println!("        missing suite's documented invocation, or drop the line whose command");
        println!("        runs no configured suite (evidence-kit/SPEC.md §check-battery-roster).");
        return Ok(1);
    }

    println!(
        "BATTERY-ROSTER: clean ({} configured suite(s) in name-set parity with the battery-roster block in {})",
        suites.len(),
        doc
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_loses_its_annotation_and_its_whitespace_run() {
        assert_eq!(
            normalize_line("   bash bin/run-beta.sh     # beta — normalized\r"),
            "bash bin/run-beta.sh"
        );
        assert_eq!(normalize_line("\t<!-- battery-roster:begin -->  "), BEGIN);
        assert_eq!(normalize_line("# whole-line comment"), "");
    }

    // spec: evidence-kit/SPEC.md §check-battery-roster — the roster selector wants a
    // lowercase-led command word and at least one more word after it
    #[test]
    fn only_a_lowercase_led_multiword_line_is_a_roster_line() {
        assert!(is_roster_line("bash bin/run-alpha.sh"));
        assert!(is_roster_line("make-2_x check"));
        assert!(!is_roster_line("bash"));
        assert!(!is_roster_line("```bash"));
        assert!(!is_roster_line("Run the suites"));
        assert!(!is_roster_line(""));
    }

    // spec: evidence-kit/SPEC.md §check-battery-roster — both prefix spellings normalize away,
    // and a later assignment-shaped token is an argument rather than a prefix
    #[test]
    fn the_head_env_prefix_normalizes_away_in_either_spelling() {
        assert_eq!(
            normalize_invocation("env FIXTURE_VERBOSE=1 bash bin/run-beta.sh"),
            "bash bin/run-beta.sh"
        );
        assert_eq!(
            normalize_invocation("VERBOSE=1 DEEP=2 bash bin/run-alpha.sh"),
            "bash bin/run-alpha.sh"
        );
        assert_eq!(
            normalize_invocation("bash  bin/run-gamma.sh   --deep=1"),
            "bash bin/run-gamma.sh --deep=1"
        );
        assert_eq!(normalize_invocation("env VERBOSE=1"), "");
        assert_eq!(normalize_invocation(""), "");
    }

    #[test]
    fn an_assignment_needs_an_identifier_left_of_its_first_equals() {
        assert!(assignment("A=1"));
        assert!(assignment("_a9="));
        assert!(!assignment("--deep=1"));
        assert!(!assignment("9a=1"));
        assert!(!assignment("bash"));
    }
}
