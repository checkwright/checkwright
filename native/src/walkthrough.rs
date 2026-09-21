// spec: installer/SPEC.md §demo — the narration both walkthroughs share, `--run-demo` and the
// `demo` verb: one banner, one green token, one excerpt rule; each keeps its own acts
use crate::ere::Ere;
use std::path::PathBuf;

const RULE: &str = "════════════════════════════════════════════════════════════";

// spec: gate-sdk/SPEC.md §Consumer smoke — the shell form's `trap cleanup EXIT`, which removed the
// scratch on every exit path. The walkthrough narrates and tears down its own scratch as part of
// its arc, so there is no mode to keep and no `--keep` to suppress this.
pub struct Scratch {
    pub dir: Option<PathBuf>,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Some(dir) = &self.dir {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
}

pub fn banner(title: &str) {
    println!("\n{}", RULE);
    println!("  {}", title);
    println!("{}", RULE);
}

pub fn say(line: &str) {
    println!("  {}", line);
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the positive green token, matched on the summary line's
// own grammar rather than on a gate count, and echoed back as the act's one-line result.
pub fn green_line(out: &str) -> Result<String, String> {
    let re = Ere::compile("All [0-9]+ gates passed").map_err(|e| e.to_string())?;
    let hits: Vec<&str> = out.lines().filter(|l| re.is_match(l)).collect();
    Ok(hits.join("\n"))
}

pub fn ends_with_newline(out: &str) -> String {
    if out.ends_with('\n') {
        out.to_string()
    } else {
        format!("{}\n", out)
    }
}

// spec: gate-sdk/SPEC.md §Consumer smoke — every gate the battery reddened, read off its own
// `FAIL: <gate>` verdict lines in print order, for a walkthrough that names no gate in advance
pub fn failed_gates(out: &str) -> Vec<String> {
    let mut gates: Vec<String> = Vec::new();
    for line in out.lines() {
        let Some(rest) = line.trim_start().strip_prefix("FAIL: ") else {
            continue;
        };
        let name = rest.split_whitespace().next().unwrap_or("").to_string();
        if !name.is_empty() && !gates.contains(&name) {
            gates.push(name);
        }
    }
    gates
}

// spec: gate-sdk/SPEC.md §Consumer smoke — the reddened gate's own block quoted back, from its
// section header through its `FAIL:` line and the invariant line beneath it, so the reader sees the
// finding, the help line and the remedy rather than being told they exist
pub fn excerpt(out: &str, gate: &str) -> Vec<String> {
    let head = format!("===== {} =====", gate);
    let tail = format!("FAIL: {}", gate);
    let mut quoted = Vec::new();
    let mut on = false;
    let mut past_verdict = false;
    for line in out.lines() {
        if line.contains(&head) {
            on = true;
        }
        if !on {
            continue;
        }
        if past_verdict && !line.starts_with(crate::runner::SPEC_LINE_PREFIX) {
            break;
        }
        quoted.push(format!("  | {}", line));
        if line.contains(&tail) {
            past_verdict = true;
        }
    }
    quoted
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §Consumer smoke — the excerpt runs from the gate's section header
    // through its own `FAIL:` line AND the invariant line beneath it, and stops there, so the
    // remedy joins the quote and a later gate's block never does.
    #[test]
    fn the_excerpt_stops_after_its_own_fail_lines_invariant() {
        let spec = format!("{}gate-sdk/SPEC.md §Whatever — the rule", crate::runner::SPEC_LINE_PREFIX);
        let out = format!(
            "before\n===== check-x =====\nfinding\nFAIL: check-x (exit 1)\n{}\n\
             ===== check-y =====\nFAIL: check-y (exit 1)\n",
            spec
        );
        assert_eq!(
            excerpt(&out, "check-x"),
            vec![
                "  | ===== check-x =====".to_string(),
                "  | finding".to_string(),
                "  | FAIL: check-x (exit 1)".to_string(),
                format!("  | {}", spec),
            ]
        );
        // comment-tier-exempt: a descriptor carrying no `# spec:` line is already a red under the
        // self-lint contract, so this case pins a shape the battery cannot otherwise reach
        let bare = "===== check-x =====\nfinding\nFAIL: check-x (exit 1)\n\
                    ===== check-y =====\nFAIL: check-y (exit 1)\n";
        assert_eq!(
            excerpt(bare, "check-x"),
            vec![
                "  | ===== check-x =====".to_string(),
                "  | finding".to_string(),
                "  | FAIL: check-x (exit 1)".to_string(),
            ]
        );
        assert!(excerpt(&out, "check-absent").is_empty());
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the green token is the summary line's own grammar
    // and not a gate count, so a battery whose roster grew still matches and a red one does not.
    #[test]
    fn the_green_token_matches_the_summary_grammar_alone() {
        assert!(matches!(
            green_line("noise\nAll 108 gates passed\n"),
            Ok(ref l) if l == "All 108 gates passed"
        ));
        assert!(matches!(
            green_line("1 of 108 gates FAILED: check-x\n"),
            Ok(ref l) if l.is_empty()
        ));
    }

    // spec: gate-sdk/SPEC.md §Consumer smoke — the reddened set is every distinct `FAIL:` verdict in
    // print order, and the summary's `FAILED:` line names no gate a second time
    #[test]
    fn the_reddened_set_is_read_off_the_verdict_lines() {
        let out = "===== check-a =====\nFAIL: check-a (exit 1)\n===== check-b =====\n\
                   FAIL: check-b (exit 2)\nFAIL: check-a (exit 1)\n2 of 9 gates FAILED: check-a check-b\n";
        assert_eq!(failed_gates(out), vec!["check-a".to_string(), "check-b".to_string()]);
        assert!(failed_gates("All 9 gates passed\n").is_empty());
    }
}
