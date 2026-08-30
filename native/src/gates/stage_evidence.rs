// spec: lifecycle-kit/SPEC.md §check-stage-evidence — stamp grammar + name-axis agreement
// (staleness) between the header and every stamp; cross-stage session-id distinctness; the
// stamp-provenance and stamp-commit-purity assertions over the fifth <head> field
use crate::proc;
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

// spec: lifecycle-kit/SPEC.md §The state machine — the <head> field's grammar: the `none`
// sentinel, or a lowercase hex abbreviation long enough for the prefix comparison the
// provenance assertion makes (git's own floor is seven)
fn is_head(s: &str) -> bool {
    s == "none"
        || ((7..=40).contains(&s.len())
            && s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)))
}

// spec: lifecycle-kit/SPEC.md §check-stage-evidence — the repo-root-relative spelling the
// purity assertion compares in, the frame `git diff --cached --name-only` prints
// spec: gate-sdk/SPEC.md §The crate's crosser — one normalizer in the crate, so this is a frame
// shift over its owner rather than a second implementation
fn norm(p: &str) -> String {
    walk::normalize_abs(p).trim_start_matches('/').to_string()
}

fn git_line(args: &[&str]) -> Option<String> {
    let c = proc::run("git", args).ok()?;
    let b = c.stdout()?;
    Some(String::from_utf8_lossy(b).trim().to_string())
}

fn git_blob(args: &[&str]) -> Option<String> {
    let c = proc::run("git", args).ok()?;
    let b = c.stdout()?;
    Some(String::from_utf8_lossy(b).into_owned())
}

// spec: lifecycle-kit/SPEC.md §check-stage-evidence — the stamp-provenance and
// stamp-commit-purity assertions. `Ok(empty)` is clean AND every inertness condition;
// `Err` is a spawn failure the caller reports as exit 2 rather than as a clean run.
fn provenance(
    state: &str,
    stext: &str,
    first_stage: &str,
    queue: &str,
) -> Result<Vec<String>, String> {
    let inert = Ok(Vec::new());
    let dir = match Path::new(state).parent() {
        Some(d) if !d.as_os_str().is_empty() => d.to_string_lossy().into_owned(),
        _ => ".".to_string(),
    };
    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — inertness (a): no work tree holding
    // the state file, or a different one from the work tree the configured surfaces resolve
    // against — a vendored tree under test, a sandbox fixture
    let (Some(root_s), Some(root_c)) = (
        git_line(&["-C", &dir, "rev-parse", "--show-toplevel"]),
        git_line(&["rev-parse", "--show-toplevel"]),
    ) else {
        return inert;
    };
    if root_s != root_c {
        return inert;
    }
    let (Some(prefix), Some(full_head)) = (
        git_line(&["rev-parse", "--show-prefix"]),
        git_line(&["rev-parse", "HEAD"]),
    ) else {
        return inert;
    };
    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — an absolute knob value is rebased
    // onto the repo root rather than compared in a frame nothing else uses
    let rel = |p: &str| match p.strip_prefix(&format!("{}/", root_c)) {
        Some(inside) => norm(inside),
        None => norm(&format!("{}/{}", prefix, p)),
    };
    let rel_state = rel(state);
    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — inertness (b): the file handed to
    // the gate is not this work tree's own configured state file. The knob is deliberately NOT
    // put through `rel` — the question is asked from the repo root, where a case dir differs
    // comment-tier-exempt: `rel` vs bare `norm` on the two sides is invisible in the call
    if rel_state != norm(&walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE")?) {
        return inert;
    }
    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — inertness (c): the state file is not
    // tracked in HEAD, so there is no prior version to diff against and "newly introduced" is
    // unanswerable rather than false
    let Some(blob) = git_blob(&["show", &format!("HEAD:{}", rel_state)]) else {
        return inert;
    };

    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — identity is the `<session-id> <head>`
    // pair (--rename's column-1 rewrite must not re-introduce every line); the migration clause
    // is the second arm, a HEAD-version line still carrying four fields
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut migrating: Vec<String> = Vec::new();
    for l in stages::data_lines(&blob) {
        let f: Vec<&str> = l.split_whitespace().collect();
        match (f.get(2), f.get(4), f.len()) {
            (Some(sid), Some(h), _) => pairs.push((sid.to_string(), h.to_string())),
            (Some(sid), None, 4) => migrating.push(sid.to_string()),
            _ => {}
        }
    }

    let mut new_stamps: Vec<(String, String, String)> = Vec::new();
    for line in stages::data_lines(stext) {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (Some(&stg), Some(&sid), Some(&h)) = (f.get(1), f.get(2), f.get(4)) else {
            continue;
        };
        if pairs.iter().any(|(s, p)| s == sid && p == h) || migrating.iter().any(|s| s == sid) {
            continue;
        }
        new_stamps.push((stg.to_string(), h.to_string(), line.to_string()));
    }
    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — inertness (d): no newly introduced
    // stamp, which is every battery run that stamps nothing
    if new_stamps.is_empty() {
        return inert;
    }

    let mut out: Vec<String> = Vec::new();
    for (_, h, line) in &new_stamps {
        if h == "none" {
            out.push(format!("stamp records head 'none' inside a git work tree — the sentinel is for a tree with no commit to name, never a stamp taken in one: {}", line));
        } else if !full_head.starts_with(h.as_str()) {
            out.push(format!("stamp records head '{}' but HEAD is now '{}' — HEAD moved between the stamp's write and its commit, so commits landed under a mark that had not been made yet: {}", h, &full_head[..h.len().min(full_head.len())], line));
        }
    }

    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — the purity assertion is gated on the
    // state file being STAGED: a stamp nothing is committing is not a stamp this commit
    // introduces
    let staged_raw = proc::run("git", &["diff", "--cached", "--name-only"])?;
    let Some(bytes) = staged_raw.stdout() else {
        return Err("git diff --cached failed — the staged path set could not be read; treating as failure (not clean)".to_string());
    };
    let staged: Vec<String> = String::from_utf8_lossy(bytes)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect();
    if !staged.iter().any(|p| p == &rel_state) {
        return Ok(out);
    }

    let mut permitted: Vec<String> = vec![rel_state.clone()];
    // spec: lifecycle-kit/SPEC.md §check-stage-evidence — the exemption's predicate is the paths
    // bin/enter-stage.sh itself writes at this entry, so the valve ledger rides at ANY stage:
    // its scoping is membership, a non-admitting entry leaving the ledger unstaged
    let valve = walk::knob_scalar("LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE")?;
    if !valve.is_empty() {
        permitted.push(rel(&valve));
    }
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the boundary reset's own members stay the
    // first stage's alone, that reset being the only entry that writes them
    if new_stamps.iter().any(|(stg, _, _)| stg == first_stage) {
        permitted.push(rel(queue));
        for p in stages::supersede_set()?.iter().chain(stages::union_set()?.iter()) {
            permitted.push(rel(p));
        }
    }
    for p in &staged {
        if !permitted.contains(p) {
            out.push(format!("the commit introducing a stamp also stages '{}' — a stamp commit carries only the paths bin/enter-stage.sh writes at that entry, which here are {}, so the mark cannot be back-dated into a work commit: {}", p, permitted.join(" "), state));
        }
    }
    Ok(out)
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
        println!("  help: create it — prose header, a '---' separator, then one '<iter> <stage> <session-id> <YYYY-MM-DD> <head>' stamp per stage-skill invocation");
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
        println!("  help: run the stage skill (it stamps as its first step), or append the '<iter> <stage> <session-id> <YYYY-MM-DD> <head>' stamp below the '---' separator");
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
        let f5 = f.get(4).copied().unwrap_or("");
        // spec: lifecycle-kit/SPEC.md §check-stage-evidence — exactly five fields: the <head>
        // field is required and `none` is a value, because a permanently optional field is a
        // permanent disarm switch for the provenance assertion below
        if f.len() != 5 {
            errors.push(format!(
                "malformed stamp (want '<iter> <stage> <session-id> <YYYY-MM-DD> <head>'): {}",
                line
            ));
            continue;
        }
        if !is_head(f5) {
            errors.push(format!(
                "bad head '{}' (want 'none' or a 7-40 character lowercase hex abbreviation): {}",
                f5, line
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
        println!("  help: run /$stage in this session (it stamps {} as its first step), or append the '<iter> <stage> <session> <date> <head>' stamp", state);
        return 1;
    }

    match provenance(&state, &stext, &first_stage, &queue) {
        Err(e) => {
            eprintln!("check-stage-evidence: {}", e);
            return 2;
        }
        Ok(v) if !v.is_empty() => {
            println!("STAGE-EVIDENCE: {} stamp-provenance issue(s) in {}:", v.len(), state);
            for e in &v {
                println!("  {}", e);
            }
            println!("  help: re-run the stage skill's first step (bash lifecycle-kit/bin/enter-stage.sh <stage>) — it appends a fresh stamp at the current HEAD, which is a same-stage re-entry and in contract; then commit that stamp on its own (lifecycle-kit/SPEC.md §check-stage-evidence)");
            return 1;
        }
        Ok(_) => {}
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

    // spec: gate-sdk/SPEC.md §The crate's crosser — the retirement's two halves: the compared
    // frame stays relative, so `rel_state` is still spliceable into `HEAD:<path>`, and the
    // backslash run the retired local normalizer could not see is now repaired
    #[test]
    fn the_compared_frame_stays_relative_while_a_backslash_run_is_now_repaired() {
        assert_eq!(norm(".workflow/STATE.txt"), ".workflow/STATE.txt");
        assert_eq!(norm("sub/../.workflow/./x"), ".workflow/x");
        assert_eq!(norm(""), "");
        assert_eq!(norm(".workflow\\STATE.txt"), ".workflow/STATE.txt");
    }
}
