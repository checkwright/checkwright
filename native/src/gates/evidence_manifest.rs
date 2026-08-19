// spec: evidence-kit/SPEC.md §check-evidence-manifest — (B) manifest grammar + current-iteration
// scoping; with lifecycle configured also (A) close-entry green block and (C) validate-stamp↔
// evidence coupling
use crate::evidence;
use crate::walk;
use std::path::Path;

fn knob_or(args: &[String], at: usize, knob: &str) -> Result<String, String> {
    match args.get(at).filter(|a| !a.is_empty()) {
        Some(v) => Ok(v.clone()),
        None => walk::knob_scalar(knob),
    }
}

// spec: evidence-kit/SPEC.md §Evidence manifest — bash `read -r f1 … f8 rest`: eight
// whitespace-separated fields with everything past the eighth landing in `rest`, so a short
// line leaves f8 empty and a long one leaves rest non-empty
fn fields(line: &str) -> (Vec<String>, String) {
    let f: Vec<&str> = line.split_whitespace().collect();
    let at = |i: usize| f.get(i).copied().unwrap_or("").to_string();
    ((0..8).map(at).collect(), f[8.min(f.len())..].join(" "))
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

// spec: evidence-kit/SPEC.md §Evidence manifest — `^[0-9]{4}-[0-9]{2}-[0-9]{2}$`, matched
// digit-wise as the shell's own character class does rather than parsed as a calendar date
fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 10 {
        return false;
    }
    b.iter().enumerate().all(|(i, c)| {
        if i == 4 || i == 7 {
            *c == b'-'
        } else {
            c.is_ascii_digit()
        }
    })
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-evidence-manifest: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let manifest = knob_or(args, 0, "EVIDENCE_KIT_MANIFEST_FILE")?;
    let queue = knob_or(args, 1, "EVIDENCE_KIT_QUEUE_FILE")?;
    let state = knob_or(args, 2, "EVIDENCE_KIT_STATE_FILE")?;
    let suites = walk::knob_array("EVIDENCE_KIT_SUITES")?;

    if !Path::new(&manifest).is_file() {
        println!("EVIDENCE-MANIFEST: manifest not found: {}", manifest);
        println!("  help: seed it with a '# contract: {}' header; run-validate appends one line per suite", evidence::MANIFEST_CONTRACT);
        return Ok(1);
    }
    let mtext = read_or_empty(&manifest);
    let want_header = format!("# contract: {}", evidence::MANIFEST_CONTRACT);
    if mtext.lines().next().unwrap_or("") != want_header {
        println!(
            "EVIDENCE-MANIFEST: first line is not the versioned wire-format header '{}' in {}",
            want_header, manifest
        );
        println!("  help: the manifest's first line declares the wire contract the attestation payload is versioned by; run-validate never rewrites it, and the iteration-boundary truncation preserves it");
        return Ok(1);
    }

    // spec: evidence-kit/SPEC.md §lib/evidence.sh — both readers swallow their own failure at
    // the call site (`|| true`), so an absent or headerless file is an empty cursor here
    let iter = evidence::queue_iteration(&read_or_empty(&queue)).unwrap_or_default();
    let stext = read_or_empty(&state);
    let stage = evidence::state_stage(&stext).unwrap_or_default();

    // assertion B: every manifest line carries the version header, the eight-field shape, and
    // the current iteration
    let mut grammar: Vec<String> = Vec::new();
    let mut clean_suite_date: Vec<(String, String)> = Vec::new();
    let mut have_line_for_iter = false;
    for line in evidence::data_lines(&mtext) {
        let (f, rest) = fields(line);
        if f[7].is_empty() || !rest.is_empty() {
            grammar.push(format!("malformed line (want '<iteration> <suite> sha256=… pass=… fail=… ignore=… verdict=… <date>'): {}", line));
            continue;
        }
        let mut ok = true;
        if !f[2].strip_prefix("sha256=").map(is_hex64).unwrap_or(false) {
            grammar.push(format!("bad sha256 field '{}': {}", f[2], line));
            ok = false;
        }
        for (i, name) in [(3usize, "pass"), (4, "fail"), (5, "ignore")] {
            if !f[i]
                .strip_prefix(&format!("{}=", name))
                .map(is_digits)
                .unwrap_or(false)
            {
                grammar.push(format!("bad {} field '{}': {}", name, f[i], line));
                ok = false;
            }
        }
        if f[6] != "verdict=clean" && f[6] != "verdict=new-failures" {
            grammar.push(format!(
                "bad verdict field '{}' (want verdict=clean|new-failures): {}",
                f[6], line
            ));
            ok = false;
        }
        if !is_date(&f[7]) {
            grammar.push(format!("bad date '{}': {}", f[7], line));
            ok = false;
        }
        if !ok {
            continue;
        }
        if !iter.is_empty() && f[0] != iter {
            grammar.push(format!("foreign iteration '{}' (current is '{}') — the iteration-boundary truncation was skipped: {}", f[0], iter, line));
            continue;
        }
        have_line_for_iter = true;
        if f[6] == "verdict=clean" {
            clean_suite_date.retain(|(s, _)| *s != f[1]);
            clean_suite_date.push((f[1].clone(), f[7].clone()));
        }
    }

    if !grammar.is_empty() {
        println!(
            "EVIDENCE-MANIFEST: {} grammar issue(s) in {}:",
            grammar.len(),
            manifest
        );
        for g in &grammar {
            println!("  {}", g);
        }
        println!("  help: every line is the eight-field '{}' shape and carries the current iteration; a foreign line means the iteration-boundary truncation (LIFECYCLE_KIT_BOUNDARY_TRUNCATE) did not clear the manifest", evidence::MANIFEST_CONTRACT);
        return Ok(1);
    }

    // spec: evidence-kit/SPEC.md §check-evidence-manifest — an empty cursor disarms A and C
    // here, at the declared early-out, rather than letting an empty stage slip past two live
    // assertions; the no-cursor window is reachable in normal operation
    if iter.is_empty() || !Path::new(&state).is_file() || stage.is_empty() {
        println!("EVIDENCE-MANIFEST: clean (grammar holds in {}; no lifecycle state — close-entry/stamp-coupling disarmed)", manifest);
        return Ok(0);
    }

    let mut earliest_validate = String::new();
    let mut have_validate = false;
    for line in evidence::state_lines(&stext) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.first().copied().unwrap_or("") != iter || f.get(1).copied().unwrap_or("") != "validate"
        {
            continue;
        }
        have_validate = true;
        let d = f.get(3).copied().unwrap_or("");
        if earliest_validate.is_empty() || d < earliest_validate.as_str() {
            earliest_validate = d.to_string();
        }
    }

    let mut errors: Vec<String> = Vec::new();

    // assertion C: a validate stamp demands ≥1 evidence line, re-armed only once the cursor has
    // advanced past validate (the entry stamp precedes the suites)
    if have_validate && stage != "validate" && !have_line_for_iter {
        errors.push(format!("iteration '{}' has a validate stamp but no evidence line — validate ran and recorded nothing (run evidence-kit/bin/run-validate.sh)", iter));
    }

    // assertion A: a close-entry cursor requires the full green block — every configured suite a
    // clean line dated on/after the earliest validate stamp
    if stage == "close" {
        for suite in &suites {
            match clean_suite_date.iter().find(|(s, _)| s == suite) {
                None => errors.push(format!(
                    "close entry: suite '{}' has no clean evidence line for '{}'",
                    suite, iter
                )),
                Some((_, d)) => {
                    if !earliest_validate.is_empty() && d.as_str() < earliest_validate.as_str() {
                        errors.push(format!("close entry: suite '{}' clean line is dated {}, before the earliest validate stamp {} — stale evidence", suite, d, earliest_validate));
                    }
                }
            }
        }
    }

    if !errors.is_empty() {
        println!(
            "EVIDENCE-MANIFEST: {} issue(s) coupling {} to {}:",
            errors.len(),
            manifest,
            state
        );
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: record a run-validate evidence line per suite before the close entry is stamped; the entry stamp proves invocation, the evidence line proves the green result");
        return Ok(1);
    }
    println!("EVIDENCE-MANIFEST: clean (grammar + close-entry/stamp-coupling hold for '{}' at stage '{}' in {})", iter, stage, manifest);
    Ok(0)
}

// spec: evidence-kit/SPEC.md §check-evidence-manifest — an unreadable queue or state file is
// the empty-cursor shape the shell's `2>/dev/null || true` already produces, never a refusal;
// the manifest itself is guarded by the not-found branch above before it is read
fn read_or_empty(path: &str) -> String {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_eight_field_shape_rejects_both_a_short_line_and_a_long_one() {
        let (f, rest) = fields("i s sha256=x pass=1 fail=0 ignore=0 verdict=clean 2026-01-01");
        assert!(rest.is_empty());
        assert_eq!(f[0], "i");
        assert_eq!(f[7], "2026-01-01");
        assert!(fields("i s c").0[7].is_empty());
        assert!(!fields("a b c d e f g h i").1.is_empty());
    }

    #[test]
    fn the_field_predicates_match_the_shell_character_classes() {
        assert!(is_hex64(&"a".repeat(64)));
        assert!(!is_hex64(&"A".repeat(64)));
        assert!(!is_hex64(&"a".repeat(63)));
        assert!(is_digits("0"));
        assert!(!is_digits(""));
        assert!(!is_digits("1x"));
        assert!(is_date("2026-01-01"));
        assert!(!is_date("2026-1-01"));
        assert!(!is_date("2026-01-01x"));
    }
}
