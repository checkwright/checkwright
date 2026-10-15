// spec: evidence-kit/SPEC.md §Layout and configuration — `parse-gates-log`: the verbose run-gates
// log to one scenario per registered gate
// spec: gate-sdk/SPEC.md §The non-gate arm — an empty roster of the *happens to read nothing*
// kind: the log path arrives on argv and the arm resolves no kit knob
pub const KNOBS: &[&str] = &[];
pub const USAGE: &str = "usage: --emit parse-gates-log <log>";

// spec: gate-sdk/SPEC.md §run-gates — the tail grammar this reads is gate-sdk's, and the tails
// print only under GATE_SDK_VERBOSE
pub(crate) fn tail(line: &str) -> Option<String> {
    let status = if line.starts_with("  PASS: ") {
        "pass"
    } else if line.starts_with("  FAIL: ") {
        "fail"
    } else {
        return None;
    };
    let name = line.split_whitespace().nth(1).unwrap_or("");
    Some(format!("{} {}", name, status))
}

// spec: gate-sdk/SPEC.md §run-config-variant — one member's section of a verbose log: its verdict,
// the tail its own name carries, and the output above that tail
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Section {
    pub name: String,
    pub tail: Option<String>,
    pub output: String,
}

// spec: gate-sdk/SPEC.md §run-gates — a section opens on its `===== <name> =====` banner and the
// summary banner ends the last; the member's tail is the last tail line naming it
pub(crate) fn sections(log: &str) -> Vec<Section> {
    let mut out: Vec<(String, Vec<&str>)> = Vec::new();
    for line in log.lines() {
        if let Some(name) = line.strip_prefix("===== ").and_then(|l| l.strip_suffix(" =====")) {
            if name == "gates summary" {
                break;
            }
            out.push((name.to_string(), Vec::new()));
        } else if let Some((_, body)) = out.last_mut() {
            body.push(line);
        }
    }
    out.into_iter()
        .map(|(name, body)| {
            let at = body.iter().rposition(|l| {
                tail(l).is_some_and(|t| t.rsplit_once(' ').is_some_and(|(n, _)| n == name))
            });
            let above = &body[..at.unwrap_or(body.len())];
            let end = above.iter().rposition(|l| !l.is_empty()).map_or(0, |i| i + 1);
            Section {
                tail: at.and_then(|i| tail(body[i])),
                output: above[..end].join("\n"),
                name,
            }
        })
        .collect()
}

// spec: evidence-kit/SPEC.md §The evidence adapters — the log is the arm's only positional
pub fn emit(args: &[String]) -> Result<String, String> {
    let log = args.first().map(String::as_str).unwrap_or("");
    if log.is_empty() {
        return Err(format!("needs a <log>\n{}", USAGE));
    }
    if !std::path::Path::new(log).is_file() {
        return Err(format!("log not found: {}", log));
    }
    let text = super::read_text(log)?;
    let mut out = String::new();
    for line in text.lines() {
        if let Some(row) = tail(line) {
            out.push_str(&row);
            out.push('\n');
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: evidence-kit/SPEC.md §Layout and configuration — the two-space-indented `PASS:` /
    // `FAIL:` tail and its gate name, and nothing else
    #[test]
    fn both_tails_map_to_a_scenario_and_nothing_else_does() {
        assert_eq!(tail("  PASS: check-foo").as_deref(), Some("check-foo pass"));
        assert_eq!(
            tail("  FAIL: check-bar (exit 1)").as_deref(),
            Some("check-bar fail")
        );
        assert_eq!(
            tail("  FAIL: check-baz (dispatch harness error, exit 2)").as_deref(),
            Some("check-baz fail")
        );
        assert_eq!(
            tail("  FAIL: check-qux (unresolved)").as_deref(),
            Some("check-qux fail")
        );
        assert_eq!(tail("PASS: check-foo"), None);
        assert_eq!(tail("    PASS: check-foo"), None);
        assert_eq!(tail("  pass: check-foo"), None);
        assert_eq!(tail("===== gates summary ====="), None);
    }

    // spec: gate-sdk/SPEC.md §run-config-variant — each banner opens a section, the tail naming its
    // member closes the output, a nested tail naming another member stays output, and the summary
    // banner ends the split
    #[test]
    fn a_verbose_log_splits_into_one_section_per_member() {
        let log = "\n===== check-a =====\nA: clean (1 file)\n  PASS: check-a\n\n===== check-b =====\nB: 2 finding(s)\n  PASS: check-inner\n\n  FAIL: check-b (exit 1)\n    spec: https://example.invalid/b\n\n===== check-c =====\n  PASS: check-c\n\n===== gates summary =====\n===== check-late =====\n";
        let got = sections(log);
        assert_eq!(got.len(), 3, "{:?}", got);
        assert_eq!(got[0].tail.as_deref(), Some("check-a pass"));
        assert_eq!(got[0].output, "A: clean (1 file)");
        assert_eq!(got[1].tail.as_deref(), Some("check-b fail"));
        assert_eq!(got[1].output, "B: 2 finding(s)\n  PASS: check-inner");
        assert_eq!(got[2].output, "");
        assert!(sections("no banner\n  PASS: x\n").is_empty());
    }

    // spec: evidence-kit/SPEC.md §bin/run-validate.sh — a log with no tails yields no output, the
    // no-parseable-result run failure
    #[test]
    fn a_log_with_no_tails_yields_no_output() {
        let dir = std::env::temp_dir().join("cw-parse-gates-log-notail");
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let log = dir.join("run.log");
        std::fs::write(&log, "running gates\nAll 3 gates passed.\n").expect("write log");
        let args = vec![log.display().to_string()];
        assert_eq!(emit(&args).expect("the arm refused a readable log"), "");
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — a log that does not resolve is exit 2
    #[test]
    fn a_missing_log_fails_closed() {
        assert!(emit(&[]).is_err(), "no operand at all was accepted");
        assert!(
            emit(&["/nonexistent/run.log".to_string()]).is_err(),
            "an unresolvable log path was accepted"
        );
    }
}
