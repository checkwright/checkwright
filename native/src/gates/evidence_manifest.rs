// spec: evidence-kit/SPEC.md §check-evidence-manifest — (B) manifest grammar + current-iteration
// scoping; with lifecycle configured also (A) close-entry green block and (C) validate-stamp↔
// evidence coupling
use crate::evidence;
use crate::exit::Fault;
use crate::walk;
use std::path::Path;

fn knob_or(args: &[String], at: usize, knob: &str) -> Result<String, String> {
    match args.get(at).filter(|a| !a.is_empty()) {
        Some(v) => Ok(v.clone()),
        None => walk::knob_scalar(knob),
    }
}

// spec: evidence-kit/SPEC.md §check-evidence-manifest — (B)'s n-field shape: a short line leaves the
// last field empty and a long one leaves a remainder
fn fields(line: &str, n: usize) -> (Vec<String>, String) {
    let f: Vec<&str> = line.split_whitespace().collect();
    let at = |i: usize| f.get(i).copied().unwrap_or("").to_string();
    ((0..n).map(at).collect(), f[n.min(f.len())..].join(" "))
}

const V1_SHAPE: &str = "<iteration> <suite> sha256=… pass=… fail=… ignore=… verdict=… <date>";
const V2_SHAPE: &str =
    "<run-key> <suite> sha256=… pass=… fail=… ignore=… verdict=… <date> commit=… tree=… cmd=… origin=…";

// spec: evidence-kit/SPEC.md §Evidence manifest — a suite line under its major's grammar, each bad
// field its own finding; `None` when the line is not one
fn suite_line(line: &str, v2: bool, grammar: &mut Vec<String>) -> Option<Vec<String>> {
    let (width, shape) = if v2 { (12, V2_SHAPE) } else { (8, V1_SHAPE) };
    let (f, rest) = fields(line, width);
    if f[width - 1].is_empty() || !rest.is_empty() {
        grammar.push(format!("malformed line (want '{}'): {}", shape, line));
        return None;
    }
    let before = grammar.len();
    if !f[2].strip_prefix("sha256=").is_some_and(evidence::is_hex64) {
        grammar.push(format!("bad sha256 field '{}': {}", f[2], line));
    }
    for (i, name) in [(3usize, "pass"), (4, "fail"), (5, "ignore")] {
        if !f[i].strip_prefix(&format!("{}=", name)).is_some_and(evidence::is_digits) {
            grammar.push(format!("bad {} field '{}': {}", name, f[i], line));
        }
    }
    if f[6] != "verdict=clean" && f[6] != "verdict=new-failures" {
        grammar.push(format!("bad verdict field '{}' (want verdict=clean|new-failures): {}", f[6], line));
    }
    if !evidence::is_date(&f[7]) {
        grammar.push(format!("bad date '{}': {}", f[7], line));
    }
    if v2 {
        let commit = f[8].strip_prefix("commit=");
        if !commit.is_some_and(|c| c == "-" || crate::provenance::object_id(c)) {
            grammar.push(format!("bad commit field '{}' (want commit=<40|64 lowercase hex>|-): {}", f[8], line));
        }
        let tree = f[9].strip_prefix("tree=");
        let unborn = commit == Some("-");
        if !tree.is_some_and(|t| if unborn { t == "-" } else { t == "clean" || t == "modified" }) {
            grammar.push(format!(
                "bad tree field '{}' (want tree=clean|modified, and tree=- exactly when commit=-): {}",
                f[9], line
            ));
        }
        if !f[10].strip_prefix("cmd=").is_some_and(evidence::is_hex64) {
            grammar.push(format!("bad cmd field '{}': {}", f[10], line));
        }
        if f[11] != "origin=local" && f[11] != "origin=imported" {
            grammar.push(format!("bad origin field '{}' (want origin=local|imported): {}", f[11], line));
        }
    }
    (grammar.len() == before).then_some(f)
}

const CONTROL_SHAPE: &str =
    "<run-key> <suite> control=… scenario=… witnessed=red|green|absent <date> commit=… cmd=… assign=…";

// spec: evidence-kit/SPEC.md §Evidence manifest — a v2 control line's nine fields, each bad field
// its own finding; `None` when the line is not one
fn control_line(line: &str, grammar: &mut Vec<String>) -> Option<Vec<String>> {
    let (f, rest) = fields(line, 9);
    if f[8].is_empty() || !rest.is_empty() {
        grammar.push(format!("malformed control line (want '{}'): {}", CONTROL_SHAPE, line));
        return None;
    }
    let before = grammar.len();
    if f[2].strip_prefix("control=").map_or(true, str::is_empty) {
        grammar.push(format!("bad control field '{}': {}", f[2], line));
    }
    if f[3].strip_prefix("scenario=").map_or(true, str::is_empty) {
        grammar.push(format!("bad scenario field '{}': {}", f[3], line));
    }
    if !["witnessed=red", "witnessed=green", "witnessed=absent"].contains(&f[4].as_str()) {
        grammar.push(format!("bad witnessed field '{}' (want witnessed=red|green|absent): {}", f[4], line));
    }
    if !evidence::is_date(&f[5]) {
        grammar.push(format!("bad date '{}': {}", f[5], line));
    }
    if !f[6].strip_prefix("commit=").is_some_and(|c| c == "-" || crate::provenance::object_id(c)) {
        grammar.push(format!("bad commit field '{}' (want commit=<40|64 lowercase hex>|-): {}", f[6], line));
    }
    if !f[7].strip_prefix("cmd=").is_some_and(evidence::is_hex64) {
        grammar.push(format!("bad cmd field '{}': {}", f[7], line));
    }
    if !f[8].strip_prefix("assign=").is_some_and(evidence::is_hex64) {
        grammar.push(format!("bad assign field '{}': {}", f[8], line));
    }
    (grammar.len() == before).then_some(f)
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-evidence-manifest: {}", e);
            e.status()
        }
    }
}

fn rule(args: &[String]) -> Result<i32, Fault> {
    let manifest = knob_or(args, 0, "EVIDENCE_KIT_MANIFEST_FILE").map_err(Fault::refused)?;
    let queue = knob_or(args, 1, "EVIDENCE_KIT_QUEUE_FILE").map_err(Fault::refused)?;
    let state = knob_or(args, 2, "EVIDENCE_KIT_STATE_FILE").map_err(Fault::refused)?;
    let suites = walk::knob_array("EVIDENCE_KIT_SUITES").map_err(Fault::refused)?;

    if !Path::new(&manifest).is_file() {
        println!("EVIDENCE-MANIFEST: manifest not found: {}", manifest);
        println!("  help: seed it with a '# contract: {}' header; --run-validate appends one line per suite", evidence::MANIFEST_CONTRACT);
        return Ok(1);
    }
    let mtext = read_or_empty(&manifest);
    let v2 = match evidence::manifest_header(&mtext) {
        evidence::Header::V2 => true,
        evidence::Header::V1 => false,
        evidence::Header::Unknown(found) => {
            println!(
                "EVIDENCE-MANIFEST: first line '{}' is not a versioned wire-format header ('# contract: {}', or the legacy '# contract: {}') in {}",
                found, evidence::MANIFEST_CONTRACT, evidence::MANIFEST_CONTRACT_V1, manifest
            );
            println!("  help: the manifest's first line names the major whose grammar its lines follow; --run-validate writes v2 and rewrites a v1 header at its next fold, refuses any other, and the iteration-boundary truncation preserves it");
            return Ok(1);
        }
    };
    let contract = if v2 { evidence::MANIFEST_CONTRACT } else { evidence::MANIFEST_CONTRACT_V1 };

    // spec: evidence-kit/SPEC.md §check-evidence-manifest — a queue naming no iteration and a state
    // file yielding no stage are the empty cursor
    let iter = evidence::queue_iteration(&read_or_empty(&queue)).unwrap_or_default();
    let stext = read_or_empty(&state);
    let stage = evidence::state_stage(&stext).unwrap_or_default();

    // assertion B: every manifest line follows its header's major — v1's eight fields, or v2's
    // line-kind rule, twelve-field suite line and nine-field control line placed under its suite
    // line — and carries the current iteration
    let mut grammar: Vec<String> = Vec::new();
    let mut clean_suite_date: Vec<(String, String)> = Vec::new();
    let mut have_line_for_iter = false;
    let mut suites_seen: Vec<(String, String)> = Vec::new();
    for line in evidence::data_lines(&mtext) {
        let third = line.split_whitespace().nth(2).unwrap_or("");
        if v2 && third.starts_with("control=") {
            let Some(f) = control_line(line, &mut grammar) else {
                continue;
            };
            if !suites_seen.iter().any(|(k, s)| *k == f[0] && *s == f[1]) {
                grammar.push(format!("control line with no '{} {}' suite line above it: {}", f[0], f[1], line));
            }
            continue;
        }
        if v2 && !third.starts_with("sha256=") {
            grammar.push(format!(
                "unknown line kind '{}' (a v2 line's third field is sha256=… or control=…): {}",
                third, line
            ));
            continue;
        }
        let Some(f) = suite_line(line, v2, &mut grammar) else {
            continue;
        };
        suites_seen.push((f[0].clone(), f[1].clone()));
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
        println!("  help: every line follows the '{}' grammar its header names and carries the current iteration; a foreign line means the iteration-boundary truncation (LIFECYCLE_KIT_BOUNDARY_TRUNCATE) did not clear the manifest", contract);
        return Ok(1);
    }

    // spec: evidence-kit/SPEC.md §check-evidence-manifest — an empty cursor disarms A and C
    // here, at the declared early-out, rather than letting an empty stage slip past two live
    // assertions
    if iter.is_empty() || !Path::new(&state).is_file() || stage.is_empty() {
        println!("EVIDENCE-MANIFEST: clean (grammar holds in {}; no lifecycle state — close-entry/stamp-coupling disarmed)", manifest);
        return Ok(0);
    }

    let validate_stamp = evidence::earliest_validate(&stext, &iter);
    let have_validate = validate_stamp.is_some();
    let earliest_validate = validate_stamp.unwrap_or_default();

    let mut errors: Vec<String> = Vec::new();

    // assertion C: a validate stamp demands ≥1 evidence line, re-armed only once the cursor has
    // advanced past validate (the entry stamp precedes the suites)
    if have_validate && stage != "validate" && !have_line_for_iter {
        errors.push(format!("iteration '{}' has a validate stamp but no evidence line — validate ran and recorded nothing (run --run-validate on the binary GATE_SDK_NATIVE_BIN names)", iter));
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
        println!("  help: record a --run-validate evidence line per suite before the close entry is stamped; the entry stamp proves invocation, the evidence line proves the green result; a line recorded before a baseline row moved keeps the old baseline's verdict, so after deferring a known red (file the task, move the row) re-run that suite for a fresh line (evidence-kit/SPEC.md §check-evidence-manifest)");
        return Ok(1);
    }
    println!("EVIDENCE-MANIFEST: clean (grammar + close-entry/stamp-coupling hold for '{}' at stage '{}' in {})", iter, stage, manifest);
    Ok(0)
}

// spec: evidence-kit/SPEC.md §check-evidence-manifest — an absent or unreadable queue or state file
// reads as the empty cursor, never a refusal
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
        let (f, rest) = fields("i s sha256=x pass=1 fail=0 ignore=0 verdict=clean 2026-01-01", 8);
        assert!(rest.is_empty());
        assert_eq!(f[0], "i");
        assert_eq!(f[7], "2026-01-01");
        assert!(fields("i s c", 8).0[7].is_empty());
        assert!(!fields("a b c d e f g h i", 8).1.is_empty());
    }

    #[test]
    fn the_field_predicates_match_the_shell_character_classes() {
        use evidence::{is_date, is_digits, is_hex64};
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

    // spec: evidence-kit/SPEC.md §Evidence manifest — a control line's nine fields, each bad field
    // its own finding
    #[test]
    fn a_control_line_holds_its_nine_fields() {
        let h = "a".repeat(64);
        let d = "d".repeat(64);
        let good = format!("k s control=c scenario=t witnessed=red 2026-01-01 commit=- cmd={} assign={}", h, d);
        let mut g = Vec::new();
        assert!(control_line(&good, &mut g).is_some(), "{:?}", g);
        for bad in [
            good.replace("witnessed=red", "witnessed=pink"),
            good.replace("control=c", "control="),
            good.replace("scenario=t", "scenario="),
            good.replace("commit=-", "commit=xyz"),
            good.replace(&format!("cmd={}", h), "cmd=short"),
            good.replace(&format!("assign={}", d), "assign=short"),
            good.replace(&format!(" assign={}", d), ""),
            format!("{} extra", good),
        ] {
            let mut g = Vec::new();
            assert!(control_line(&bad, &mut g).is_none(), "{}", bad);
            assert_eq!(g.len(), 1, "{}: {:?}", bad, g);
        }
    }

    // spec: evidence-kit/SPEC.md §Evidence manifest — a v2 suite line's four appended fields, each bad
    // spelling its own finding, and tree=- bound to commit=-
    #[test]
    fn a_v2_suite_line_holds_its_four_appended_fields() {
        let h = "a".repeat(64);
        let c = "b".repeat(40);
        let head = format!("k s sha256={} pass=1 fail=0 ignore=0 verdict=clean 2026-01-01", h);
        let ok = |tail: &str| {
            let mut g = Vec::new();
            suite_line(&format!("{} {}", head, tail), true, &mut g).is_some()
        };
        assert!(ok(&format!("commit={} tree=clean cmd={} origin=local", c, h)));
        assert!(ok(&format!("commit=- tree=- cmd={} origin=imported", h)));
        assert!(!ok(&format!("commit=- tree=clean cmd={} origin=local", h)));
        assert!(!ok(&format!("commit={} tree=- cmd={} origin=local", c, h)));
        assert!(!ok(&format!("commit=XYZ tree=clean cmd={} origin=local", h)));
        assert!(!ok(&format!("commit={} tree=clean cmd=short origin=local", c)));
        assert!(!ok(&format!("commit={} tree=clean cmd={} origin=remote", c, h)));
        let mut g = Vec::new();
        assert!(suite_line(&head, true, &mut g).is_none(), "an eight-field line passed under v2");
        let mut g = Vec::new();
        assert!(suite_line(&head, false, &mut g).is_some(), "a v1 line failed under v1");
    }
}
