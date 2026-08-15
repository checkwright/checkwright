// spec: drift-kit/SPEC.md §The published-evidence extractor — docs/evidence-data.md is the
// byte-fresh projection of trajectory.sh --emit
use crate::fresh;
use std::path::Path;

const DEFAULT_PROJECTION: &str = "docs/evidence-data.md";
const EXTRACTOR: &str = "drift-kit/bin/trajectory.sh";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-trajectory-fresh: {}", e);
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
        // spec: drift-kit/SPEC.md §The published-evidence extractor — the extractor stays a
        // spawn: this cohort ports the comparator, not the emitter (§The generated-projection
        // freshness family records what that leaves owed)
        let traj = fresh::emitter_path(EXTRACTOR)?;
        if !fresh::executable(&traj) {
            return Err(format!("extractor not found: {}", traj));
        }
        fresh::emit(&traj, &["--emit"], "trajectory")?
    };

    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — both sides of the
    // shell form's equality are `$(…)`, which strips every trailing newline; the diff's right
    // side is the file itself, so the raw text is kept beside the stripped one
    let projection_raw = fresh::read_captured(projection)?;
    if emitted.trim_end_matches('\n') != projection_raw.trim_end_matches('\n') {
        println!(
            "check-trajectory-fresh: {} is stale vs the trajectory extractor:",
            projection
        );
        let left = format!("{}\n", emitted.trim_end_matches('\n'));
        fresh::print_capped_diff(&left, &projection_raw);
        println!(
            "  help: regenerate — bash drift-kit/bin/trajectory.sh --emit > docs/evidence-data.md"
        );
        return Ok(1);
    }
    println!(
        "TRAJECTORY-FRESH: clean ({} byte-matches the trajectory extractor)",
        projection
    );
    Ok(0)
}
