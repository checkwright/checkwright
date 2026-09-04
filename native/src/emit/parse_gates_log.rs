// spec: evidence-kit/SPEC.md §Layout and configuration — the EVIDENCE_KIT_PARSER_gates adapter:
// the verbose run-gates log to one scenario per registered gate
// spec: gate-sdk/SPEC.md §The non-gate arm — an empty roster of the *happens to read nothing*
// kind: the log path arrives on argv and the arm resolves no kit knob
pub const KNOBS: &[&str] = &[];

// spec: gate-sdk/SPEC.md §run-gates — the tail grammar this reads is gate-sdk's, and the tails
// print only under GATE_SDK_VERBOSE, which is why EVIDENCE_KIT_RUN_gates sets it
fn tail(line: &str) -> Option<String> {
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

// spec: evidence-kit/SPEC.md §lib/evidence.sh — the log is the arm's only positional
pub fn emit(args: &[String]) -> Result<String, String> {
    let log = args.first().map(String::as_str).unwrap_or("");
    if log.is_empty() || !std::path::Path::new(log).is_file() {
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

    // spec: evidence-kit/SPEC.md §Layout and configuration — per-gate lines off a verbose log,
    // both FAIL tails read, a differently-shaped line contributing nothing
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

    // spec: evidence-kit/SPEC.md §Layout and configuration — a log with no tails yields no
    // output, which --run-validate's produced-no-result guard reads as the run failure it is
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

    // spec: evidence-kit/SPEC.md §Layout and configuration — a missing log fails closed
    #[test]
    fn a_missing_log_fails_closed() {
        assert!(emit(&[]).is_err(), "no operand at all was accepted");
        assert!(
            emit(&["/nonexistent/run.log".to_string()]).is_err(),
            "an unresolvable log path was accepted"
        );
    }
}
