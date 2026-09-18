// spec: drift-kit/SPEC.md §The install-evidence projection — docs/install-evidence.md is the
// byte-fresh projection of the install-evidence arm, and the gate is inert on a counted zero where
// the gitignored record does not resolve to a file.
use crate::fresh;
use std::path::Path;

const DEFAULT_PROJECTION: &str = "docs/install-evidence.md";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-install-evidence-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let projection = fresh::positional(args, 0, DEFAULT_PROJECTION);
    let emit_src = args.get(1).map(String::as_str).unwrap_or("");

    // spec: drift-kit/SPEC.md §The install-evidence projection — counted inertness, the shape
    // check-action-pinning carries for a tree holding none of its subject: the record is
    // gitignored, so CI, a fresh clone and every adopter's tree have nothing to re-emit from.
    if emit_src.is_empty() {
        let (record, spelled) = crate::emit::install_evidence::record_path()?;
        if record.is_empty() || !Path::new(&record).is_file() {
            println!(
                "INSTALL-EVIDENCE-FRESH: clean (0 install-observation records — {} names no file, so there is nothing to re-emit from)",
                spelled
            );
            return Ok(0);
        }
    }

    if !Path::new(projection).is_file() {
        return Err(format!("projection not found: {}", projection));
    }

    let emitted = if !emit_src.is_empty() {
        if !Path::new(emit_src).is_file() {
            return Err(format!("emit source not found: {}", emit_src));
        }
        fresh::read_captured(emit_src)?
    } else {
        crate::emit::install_evidence::emit(&[])?
    };

    let projection_raw = fresh::read_captured(projection)?;
    if emitted.trim_end_matches('\n') != projection_raw.trim_end_matches('\n') {
        println!(
            "check-install-evidence-fresh: {} is stale vs the install-evidence projection:",
            projection
        );
        let left = format!("{}\n", emitted.trim_end_matches('\n'));
        fresh::print_capped_diff(&left, &projection_raw);
        println!(
            "  help: regenerate — bash gate-sdk/bin/run-gates.sh --emit install-evidence > docs/install-evidence.md"
        );
        return Ok(1);
    }
    println!(
        "INSTALL-EVIDENCE-FRESH: clean ({} byte-matches the install-evidence projection)",
        projection
    );
    Ok(0)
}
