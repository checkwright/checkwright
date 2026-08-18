// spec: gate-sdk/SPEC.md §check-commit-msg — the commit message matches no banned pattern, the
// leak guard for the message surface the pre-commit hook never sees
use crate::ere::Ere;
use crate::fresh;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-commit-msg — `grep -hEv '^[[:space:]]*(#|$)'`: a pattern line is
// one that is neither all-whitespace nor whitespace-then-`#`
fn is_pattern(line: &str) -> bool {
    let rest = line.trim_start_matches([' ', '\t']);
    !rest.is_empty() && !rest.starts_with('#')
}

// spec: gate-sdk/SPEC.md §check-commit-msg — `gate_msg_pattern_files` with no positional: every
// required file must exist and be readable, and each local one joins where it does
fn resolve_files() -> Result<Vec<String>, String> {
    let required = walk::knob_array("GATE_MSG_PATTERN_FILES")?;
    let local = walk::knob_array("GATE_MSG_PATTERN_FILES_LOCAL")?;
    let mut out: Vec<String> = Vec::new();
    for f in required {
        if !Path::new(&f).is_file() {
            return Err(format!(
                "gate_msg_pattern_files: required tracked pattern file missing: {}",
                f
            ));
        }
        if std::fs::File::open(&f).is_err() {
            return Err(format!(
                "gate_msg_pattern_files: pattern file not readable: {}",
                f
            ));
        }
        out.push(f);
    }
    for f in local {
        if Path::new(&f).is_file() && std::fs::File::open(&f).is_ok() {
            out.push(f);
        }
    }
    Ok(out)
}

pub fn run(args: &[String]) -> i32 {
    // spec: gate-sdk/SPEC.md §check-commit-msg — no-arg is a clean skip: the message is not a
    // whole-tree surface and a history scan is deferred
    let Some(msg) = args.first() else {
        println!("COMMIT-MSG: clean (no message file argument — the commit-msg hook surface is not a whole-tree target; skipped)");
        return 0;
    };
    if !Path::new(msg).is_file() {
        eprintln!("check-commit-msg: message file not found: {}", msg);
        return 2;
    }

    let files: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        match resolve_files() {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("COMMIT-MSG: {}", fresh::fail_closed("pattern-files", Some(2)));
                return 2;
            }
        }
    };

    let mut patterns: Vec<String> = Vec::new();
    for f in &files {
        let Ok(bytes) = std::fs::read(f) else {
            eprintln!(
                "COMMIT-MSG: {}",
                fresh::fail_closed("grep-patterns", Some(2))
            );
            return 2;
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        patterns.extend(
            fresh::file_lines(&text)
                .into_iter()
                .filter(|l| is_pattern(l))
                .map(String::from),
        );
    }

    if patterns.is_empty() {
        println!("COMMIT-MSG: clean (0 banned pattern(s) configured; message unchecked)");
        return 0;
    }

    let mut compiled: Vec<Ere> = Vec::new();
    for p in &patterns {
        match Ere::compile(p) {
            Ok(e) => compiled.push(e),
            Err(e) => {
                eprintln!("check-commit-msg: {}: {}", p, e);
                eprintln!("COMMIT-MSG: {}", fresh::fail_closed("grep", Some(2)));
                return 2;
            }
        }
    }

    let Ok(bytes) = std::fs::read(msg) else {
        eprintln!("COMMIT-MSG: {}", fresh::fail_closed("grep", Some(2)));
        return 2;
    };
    let text = String::from_utf8_lossy(&bytes).into_owned();
    // spec: gate-sdk/SPEC.md §check-commit-msg — `grep -EnHf`: one record per *matching line*,
    // filename-prefixed and line-numbered, however many patterns that line matches
    let hits: Vec<String> = fresh::file_lines(&text)
        .iter()
        .enumerate()
        .filter(|(_, line)| compiled.iter().any(|re| re.is_match(line)))
        .map(|(i, line)| format!("{}:{}:{}", msg, i + 1, line))
        .collect();

    if !hits.is_empty() {
        println!("check-commit-msg: commit message matches a banned pattern (leaked local/private term):");
        for h in &hits {
            println!("{}", h);
        }
        println!("  help: rewrite the message to remove the leaked term; the pattern set is");
        println!("        GATE_SDK_MSG_PATTERN_FILES (+ the local list). The Co-Authored-By");
        println!("        trailer is a footer convention, not a leak — do not ban it.");
        return 1;
    }

    println!(
        "COMMIT-MSG: clean (message matches none of {} banned pattern(s))",
        patterns.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-commit-msg — the pattern-file filter drops comments and
    // blanks with leading whitespace allowed before either
    #[test]
    fn a_pattern_line_is_neither_blank_nor_a_comment() {
        assert!(is_pattern("claude\\.ai/"));
        assert!(is_pattern("  ^Key: .*"));
        assert!(!is_pattern(""));
        assert!(!is_pattern("   \t "));
        assert!(!is_pattern("# a comment"));
        assert!(!is_pattern("\t  # an indented comment"));
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the live pattern shapes this gate ships
    // with, compiled and matched through the crate's own engine rather than through grep
    #[test]
    fn the_shipped_pattern_shapes_compile_and_select_the_right_lines() {
        let pats = [
            "/(home|Users)/[A-Za-z0-9._-]+",
            "^[A-Za-z][A-Za-z-]*: .*[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",
        ];
        let res: Vec<Ere> = pats
            .iter()
            .map(|p| Ere::compile(p).expect("a shipped pattern failed to compile"))
            .collect();
        let hit = |s: &str| res.iter().any(|r| r.is_match(s));
        // spec: gate-sdk/SPEC.md §check-tree-terms — the leaking shape is composed rather than
        // spelled: this module is tracked, and the same pattern set scans the tracked tree
        assert!(hit(&format!("see /{}/someone/x for the log", "home")));
        assert!(hit("Session-Id: 3f2504e0-4f89-41d3-9a0c-0305e82c3301"));
        assert!(!hit("Co-Authored-By: A Contributor <noreply@example.com>"));
        assert!(!hit("tighten the resize path"));
        assert!(!hit("  3f2504e0-4f89-41d3-9a0c-0305e82c3301"));
    }
}
