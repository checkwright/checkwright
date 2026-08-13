// spec: lifecycle-kit/SPEC.md §check-stage-evidence — stamp grammar + name-axis agreement
// (staleness) between the header and every stamp; cross-stage session-id distinctness
use crate::stages;
use crate::walk;
use std::path::Path;

fn knob_or(args: &[String], at: usize, knob: &str) -> Result<String, String> {
    match args.get(at).filter(|a| !a.is_empty()) {
        Some(v) => Ok(v.clone()),
        None => walk::knob_scalar(knob),
    }
}

fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && [0, 1, 2, 3, 5, 6, 8, 9]
            .iter()
            .all(|&i| b[i].is_ascii_digit())
}

pub fn run(args: &[String]) -> i32 {
    let (queue, state, stage_set, first_stage, waiver, boundary) = match (
        knob_or(args, 0, "LIFECYCLE_KIT_QUEUE_FILE"),
        knob_or(args, 1, "LIFECYCLE_KIT_STATE_FILE"),
        stages::stages(),
        walk::knob_scalar("LIFECYCLE_KIT_FIRST_STAGE"),
        walk::knob_scalar("LIFECYCLE_KIT_WAIVER_TOKEN"),
        walk::knob_scalar("LIFECYCLE_KIT_SESSION_BOUNDARY"),
    ) {
        (Ok(a), Ok(b), Ok(c), Ok(d), Ok(e), Ok(f)) => (a, b, c, d, e, f),
        (a, b, c, d, e, f) => {
            let err = [
                a.err(),
                b.err(),
                c.err(),
                d.err(),
                e.err(),
                f.err(),
            ]
            .into_iter()
            .flatten()
            .next()
            .unwrap_or_default();
            eprintln!("check-stage-evidence: {}", err);
            return 2;
        }
    };

    // spec: lifecycle-kit/SPEC.md §lib/stages.sh — an unreadable queue yields no header, the
    // shell form's `grep … 2>/dev/null || true`, and the missing-header branch below reports it
    let qtext = std::fs::read(&queue)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    let Some(hdr) = stages::header(&qtext) else {
        println!("STAGE-EVIDENCE: no '## Iteration:' header in {}", queue);
        println!("  help: add '## Iteration: <name>' to {}", queue);
        return 1;
    };
    let iter = stages::header_iter(hdr);
    if iter.is_empty() {
        println!("STAGE-EVIDENCE: could not parse the iteration from: {}", hdr);
        println!("  help: header must read '## Iteration: <name>'");
        return 1;
    }

    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — the cursor is read before the
    // unnamed-iteration guard because that guard consumes both axes; a state file present but
    // carrying no stamp is this gate's no-cursor fallback and reds rather than going vacuous
    if !Path::new(&state).is_file() {
        println!("STAGE-EVIDENCE: {} is missing", state);
        println!("  help: create it — prose header, a '---' separator, then one '<iter> <stage> <session-id> <YYYY-MM-DD>' stamp per stage-skill invocation");
        return 1;
    }
    let stext = match std::fs::read(&state) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            eprintln!("check-stage-evidence: cannot read {}: {}", state, e);
            return 2;
        }
    };
    let stage = stages::current_stage(&stext);
    if stage.is_empty() {
        println!(
            "STAGE-EVIDENCE: {} carries no stamp — there is no current stage to attest",
            state
        );
        println!("  help: run the stage skill (it stamps as its first step), or append the '<iter> <stage> <session-id> <YYYY-MM-DD>' stamp below the '---' separator");
        return 1;
    }

    if iter == "—" && stage != first_stage {
        println!("STAGE-EVIDENCE: iteration is still unnamed ('—') at stage '{}' — /{} must name the iteration (header + stamp) before advancing past {}", stage, first_stage, first_stage);
        println!(
            "  help: set '## Iteration: <name>' and rewrite the matching {} stamp's '—' to <name>",
            state
        );
        return 1;
    }

    let mut errors: Vec<String> = Vec::new();
    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — cross-stage session-id distinctness
    // (a stage flip is a context boundary)
    let mut stage_of_sid: Vec<(String, String)> = Vec::new();
    for line in stages::data_lines(&stext) {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (f1, f2, f3) = (
            f.first().copied().unwrap_or(""),
            f.get(1).copied().unwrap_or(""),
            f.get(2).copied().unwrap_or(""),
        );
        let f4 = f.get(3).copied().unwrap_or("");
        if f4.is_empty() || f.len() > 4 {
            errors.push(format!(
                "malformed stamp (want '<iter> <stage> <session-id> <YYYY-MM-DD>'): {}",
                line
            ));
            continue;
        }
        let known = stages::stage_known(&stage_set, f2);
        if !known && (waiver.is_empty() || f2 != waiver) {
            errors.push(format!(
                "bad stage '{}' (not a lifecycle stage: {}{}): {}",
                f2,
                stage_set.join(" "),
                if waiver.is_empty() {
                    String::new()
                } else {
                    format!(", or the waiver token {}", waiver)
                },
                line
            ));
            continue;
        }
        if !is_date(f4) {
            errors.push(format!("bad date '{}': {}", f4, line));
            continue;
        }
        if !(f1 == iter || (f1 == "—" && iter == "—")) {
            errors.push(format!("stamp iteration '{}' is neither current ('{}') nor a legal '—' bootstrap (allowed only while the header is unnamed) — stale; /{} truncates at the iteration boundary and renames its bootstrap stamp on naming: {}", f1, iter, first_stage, line));
        }
        // spec: lifecycle-kit/SPEC.md §check-stage-evidence — the distinctness map runs only
        // at the 'stage' posture; 'iteration' skips this check alone, attribution still rides
        // the stamps
        if boundary == "stage" && f1 == iter && known {
            match stage_of_sid.iter().find(|(s, _)| s == f3) {
                Some((_, seen)) if seen != f2 => {
                    errors.push(format!("session id '{}' is shared by stages '{}' and '{}' of '{}' — a stage flip is a context boundary and needs a fresh session (same-stage re-entries may share or rotate freely; waiver stamps are exempt; the 'iteration' posture of LIFECYCLE_KIT_SESSION_BOUNDARY relaxes this check): {}", f3, seen, f2, iter, line));
                }
                Some(_) => {}
                None => stage_of_sid.push((f3.to_string(), f2.to_string())),
            }
        }
    }

    if !errors.is_empty() {
        println!("STAGE-EVIDENCE: {} issue(s) in {}:", errors.len(), state);
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: run /$stage in this session (it stamps {} as its first step), or append the '<iter> <stage> <session> <date>' stamp", state);
        return 1;
    }
    if boundary == "stage" {
        println!("STAGE-EVIDENCE: clean ('{}' / '{}' stamped; all stamps well-formed, current, and stage-distinct in session id)", iter, stage);
    } else {
        println!("STAGE-EVIDENCE: clean ('{}' / '{}' stamped; all stamps well-formed and current; cross-stage distinctness relaxed by the 'iteration' session boundary)", iter, stage);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stamp_date_is_shape_checked_not_calendar_checked() {
        assert!(is_date("2026-08-13"));
        assert!(is_date("9999-99-99"));
        assert!(!is_date("2026-8-13"));
        assert!(!is_date("2026-08-13 "));
        assert!(!is_date(""));
    }
}
