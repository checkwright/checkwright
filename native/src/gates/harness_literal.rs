// spec: gate-sdk/SPEC.md §check-harness-literal — no file on the configured corpus carries a
// configured harness literal outside a site declaring the binding
use crate::fresh;
use crate::gates::portability_floor::{binary, declared, token_reason};
use crate::listing;
use crate::walk;

const VALVE: &str = "harness-binding:";

// spec: gate-sdk/SPEC.md §check-harness-literal — the reason ends at the comment closer wherever
// it sits on the line, so text after the closer justifies nothing
fn valve_reason(line: &str) -> Option<&str> {
    token_reason(VALVE, line).map(|r| r.split_once("-->").map_or(r, |(reason, _)| reason).trim())
}

pub fn run(_args: &[String]) -> i32 {
    match inner() {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

fn inner() -> Result<i32, String> {
    let literals = walk::knob_words("GATE_SDK_HARNESS_LITERALS")?;
    let paths = walk::knob_words("GATE_SDK_HARNESS_LITERAL_PATHS")?;

    // spec: gate-sdk/SPEC.md §check-harness-literal — an empty knob disables the gate, in a
    // sentence a nothing-found clean never prints
    if literals.is_empty() || paths.is_empty() {
        let absent = match (literals.is_empty(), paths.is_empty()) {
            (true, true) => "no harness literal and no corpus",
            (true, false) => "no harness literal",
            _ => "no corpus",
        };
        println!("HARNESS-LITERAL: clean ({} configured; the gate is disabled and nothing was scanned)", absent);
        return Ok(0);
    }

    let specs: Vec<&str> = paths.iter().map(String::as_str).collect();
    let listing =
        listing::tracked(None, &specs).map_err(|e| format!("check-harness-literal: {}", e.text()))?;

    let mut hits: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    let mut skipped_binary = 0usize;
    for path in listing.iter().map(String::as_str) {
        let Ok(bytes) = std::fs::read(path) else {
            return Err(format!(
                "check-harness-literal: corpus member not readable: {}\nHARNESS-LITERAL: {}",
                path,
                fresh::fail_closed("corpus-member", Some(2))
            ));
        };
        if binary(&bytes) {
            skipped_binary += 1;
            continue;
        }
        scanned += 1;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let lines = fresh::file_lines(&text);
        for (i, line) in lines.iter().enumerate() {
            let Some(lit) = literals.iter().find(|l| line.contains(l.as_str())) else {
                continue;
            };
            if declared(&lines, i, valve_reason) {
                continue;
            }
            hits.push(format!("{}:{}:{}\n    literal: {}", path, i + 1, line, lit));
        }
    }

    if !hits.is_empty() {
        println!("check-harness-literal: file(s) name a harness-bound literal outside a declared site:");
        for h in &hits {
            println!("{}", h);
        }
        println!("  help: name the harness surface generically, or — where the site reads the");
        println!("        literal or declares the binding — mark it 'harness-binding: <reason>'");
        println!("        on that line or the one above (an HTML comment in markdown). An empty");
        println!("        reason is a violation: its only reader is whoever reviews the diff.");
        return Ok(1);
    }
    println!(
        "HARNESS-LITERAL: clean ({} file(s) scanned under {} configured pathspec(s), {} binary \
         member(s) skipped; none names one of the {} harness literal(s) outside a declared site)",
        scanned,
        paths.len(),
        skipped_binary,
        literals.len()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_comment_closer_is_not_a_reason() {
        assert!(valve_reason("the hook sets DEMO_DIR").is_none());
        assert_eq!(valve_reason("<!-- harness-binding: -->"), Some(""));
        assert_eq!(valve_reason("<!-- harness-binding:-->"), Some(""));
        assert_eq!(valve_reason("# harness-binding:  "), Some(""));
        assert_eq!(
            valve_reason("<!-- harness-binding: the sentence declaring the binding -->"),
            Some("the sentence declaring the binding")
        );
    }

    #[test]
    fn text_after_the_closer_is_not_a_reason() {
        assert_eq!(valve_reason("<!-- harness-binding: --> DEMO_DIR"), Some(""));
        assert_eq!(valve_reason("<!-- harness-binding:--> reads DEMO_DIR -->"), Some(""));
        assert_eq!(valve_reason("<!-- harness-binding: declared --> DEMO_DIR"), Some("declared"));
        let same_line = ["<!-- harness-binding: --> DEMO_DIR"];
        assert!(!declared(&same_line, 0, valve_reason));
    }

    #[test]
    fn the_window_is_the_line_or_the_one_above() {
        let lines = ["<!-- harness-binding: declared -->", "reads DEMO_DIR", "", "DEMO_DIR again"];
        assert!(declared(&lines, 1, valve_reason));
        assert!(!declared(&lines, 3, valve_reason));
        let empty = ["<!-- harness-binding: -->", "reads DEMO_DIR"];
        assert!(!declared(&empty, 1, valve_reason));
    }
}
