// spec: gate-sdk/SPEC.md §check-enforcement-fresh — docs/enforcement.md is the byte-fresh
// projection of the enforcement-map emitter
use crate::emit;
use crate::fresh;
use std::path::Path;

const DEFAULT_PROJECTION: &str = "docs/enforcement.md";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-enforcement-fresh: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let projection = fresh::positional(args, 0, DEFAULT_PROJECTION);
    let emit_src = args.get(1).map(String::as_str).unwrap_or("");

    if !Path::new(projection).is_file() {
        return Err(format!("projection not found: {}", projection));
    }

    let emitted = if !emit_src.is_empty() {
        if !Path::new(emit_src).is_file() {
            return Err(format!("emit source not found: {}", emit_src));
        }
        fresh::read_captured(emit_src)?
    } else {
        // spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — the
        // emitter is a function call, not a spawn: it ported in the same unit, so there is no
        // shell left to reach and fresh::emit's bash hop is retired for this member.
        emit::enforcement_map::emit(&[])?
    };

    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — both sides of the shell form's
    // equality were `$(…)`, which strips every trailing newline; the diff's right side is the file
    // itself, so the raw text is kept beside the stripped one
    let projection_raw = fresh::read_captured(projection)?;
    if emitted.trim_end_matches('\n') != projection_raw.trim_end_matches('\n') {
        println!(
            "check-enforcement-fresh: {} is stale vs the enforcement-map emitter:",
            projection
        );
        let left = format!("{}\n", emitted.trim_end_matches('\n'));
        fresh::print_capped_diff(&left, &projection_raw);
        println!("  help: regenerate — bash gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md");
        return Ok(1);
    }
    println!(
        "ENFORCEMENT-FRESH: clean ({} byte-matches the enforcement-map emitter)",
        projection
    );
    Ok(0)
}
