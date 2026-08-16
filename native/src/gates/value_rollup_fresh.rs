// spec: docs/site-architecture.md §Generated projections and their freshness gates — the
// value-rollup block in docs/value.md is the byte-fresh projection of gen-value-rollup.sh --emit
use crate::fresh;
use std::path::Path;

const DEFAULT_PROJECTION: &str = "docs/value.md";
const BEGIN: &str = "<!-- value-rollup:begin -->";
const END: &str = "<!-- value-rollup:end -->";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-value-rollup-fresh: {}", e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — the block reader moved to the shared marker module when
// its writer landed, so one implementation serves the comparator and the generator
fn marker_block(text: &str) -> String {
    crate::marker::read_block(text, BEGIN, END)
}

fn rule(args: &[String]) -> Result<i32, String> {
    let projection = fresh::positional(args, 0, DEFAULT_PROJECTION);
    let emit_src = args.get(1).map(String::as_str).unwrap_or("");

    if !Path::new(projection).is_file() {
        return Err(format!("projection not found: {}", projection));
    }

    let block = marker_block(&fresh::read_captured(projection)?);
    if block.is_empty() {
        return Err(format!(
            "no value-rollup marker block in {}",
            projection
        ));
    }

    let emitted = if !emit_src.is_empty() {
        if !Path::new(emit_src).is_file() {
            return Err(format!("emit source not found: {}", emit_src));
        }
        fresh::read_captured(emit_src)?
    } else {
        // spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — the join
        // is a function call, not a spawn: it ported in the same unit, so there is no shell left
        // to reach and fresh::emit's bash hop is retired for this member too.
        crate::emit::value_rollup::emit(&[])?
    };
    let emitted = emitted.trim_end_matches('\n');

    if block != emitted {
        println!(
            "check-value-rollup-fresh: the value-rollup block in {} is stale vs gen-value-rollup.sh:",
            projection
        );
        // spec: gate-sdk/SPEC.md §The consumer remainder cohort — both sides are
        // `printf '%s\n'` process substitutions here, so both carry one terminating newline
        fresh::print_capped_diff(&format!("{}\n", emitted), &format!("{}\n", block));
        println!("  help: regenerate — bash gate-sdk/bin/run-gates.sh --emit value-rollup --write");
        return Ok(1);
    }
    println!(
        "VALUE-ROLLUP-FRESH: clean (the value-rollup block in {} byte-matches gen-value-rollup.sh)",
        projection
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the
    // markers match on the whole line, so an indented or embedded spelling opens nothing
    #[test]
    fn the_block_is_the_lines_strictly_between_whole_line_markers() {
        let text = format!("head\n{}\na\nb\n{}\ntail\n", BEGIN, END);
        assert_eq!(marker_block(&text), "a\nb");
        assert_eq!(marker_block("no markers here\n"), "");
        assert_eq!(marker_block(&format!("  {}\na\n", BEGIN)), "");
    }
}
