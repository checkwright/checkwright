// spec: gate-sdk/SPEC.md §check-pipefail-readers — `--emit pipefail-fix [<path>…]`, the gate's
// remedy: every fixable hit in the gate's corpus rewritten in place, any other left for a hand fix
use crate::exit::Fault;
use crate::gates::pipefail_readers::{self, Lint};
use crate::shell_lines::{self, Edit};

pub const KNOBS: &[&str] = &[
    "GATE_SDK_PIPEFAIL_PATHS",
    "GATE_SDK_PIPEFAIL_LIBRARIES",
    "GATE_SDK_PIPEFAIL_ALL",
    "GATE_SDK_PIPEFAIL_WAIVER_MIN",
    "GATE_SDK_PRUNE_DIRS",
    "GATE_SDK_PRUNE_EXTRA_DIRS",
];

pub const USAGE: &str = "usage: --emit pipefail-fix [<path>…]";

pub fn emit(args: &[String]) -> Result<String, Fault> {
    if args.iter().any(|a| a.starts_with('-')) {
        return Err(Fault::usage(USAGE));
    }
    let Some((files, _)) = pipefail_readers::corpus(args).map_err(|e| e.prefixed("pipefail-fix"))? else {
        return Ok("pipefail-fix: nothing to rewrite (no shell path configured)\n".to_string());
    };
    let cfg = pipefail_readers::config().map_err(|e| e.prefixed("pipefail-fix"))?;
    let mut out = String::new();
    let mut failed: Vec<String> = Vec::new();
    for f in &files {
        let text = match pipefail_readers::load(f) {
            Ok(t) => t,
            Err(e) => {
                failed.push(e.message);
                continue;
            }
        };
        let hits = match pipefail_readers::lint(f, &text, &cfg) {
            Lint::Off => continue,
            Lint::Unparsed(line) => {
                failed.push(format!("{}:{}: [LINT-PARSE] unparseable span — the file is left as it was", f, line));
                continue;
            }
            Lint::Hits(h) => h,
        };
        let (mut edits, mut lines): (Vec<Edit>, Vec<usize>) = (Vec::new(), Vec::new());
        for h in hits {
            if let Some(fix) = h.fix {
                edits.extend(fix);
                lines.push(h.line);
            }
        }
        if edits.is_empty() {
            continue;
        }
        // spec: gate-sdk/SPEC.md §check-pipefail-readers — written over the existing file, so its
        // mode is kept
        let fixed = shell_lines::apply(&text, &edits);
        if let Err(e) = std::fs::write(f, &fixed) {
            failed.push(format!("{}: cannot write: {}", f, e));
            continue;
        }
        for line in lines {
            out.push_str(&format!("{}:{}: fixed\n", f, line));
        }
    }
    if !failed.is_empty() {
        let fault = Fault::input(format!("pipefail-fix: could not rewrite:\n  {}", failed.join("\n  ")));
        return Err(if out.is_empty() { fault } else { fault.with_stdout(out.trim_end().to_string()) });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-pipefail-readers — a flag is no operand, so it refuses before
    // any knob or file is read
    #[test]
    fn a_flag_refuses_with_the_usage() {
        for args in [&["--write"][..], &["a.sh", "-n"]] {
            let argv: Vec<String> = args.iter().map(|a| a.to_string()).collect();
            let fault = emit(&argv).expect_err("a flag must refuse");
            assert_eq!(fault.message, USAGE, "{:?}", args);
            assert_eq!(fault.status(), crate::exit::USAGE, "{:?}", args);
        }
    }
}
