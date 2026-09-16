// spec: lifecycle-kit/SPEC.md §check-audit-roster — every audit-roster block carries the grammar
// §The audit roster states, under the line cap, with unique classes and a stage-checked last
use crate::stages;
use crate::walk;
use std::path::Path;

const HEAD: [&str; 4] = ["class", "scope", "due", "last"];
const SWEPT: [&str; 3] = ["corpus", "hits", "declined"];

fn key_line(line: &str) -> Option<(&str, &str)> {
    let (key, val) = line.split_once(':')?;
    if key.is_empty() || !key.bytes().all(|b| b.is_ascii_lowercase()) {
        return None;
    }
    Some((key, val.trim()))
}

enum Read {
    Absent,
    Text(String),
}

fn read(path: &str) -> Result<Read, String> {
    match std::fs::read(path) {
        Ok(b) => Ok(Read::Text(String::from_utf8_lossy(&b).into_owned())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Read::Absent),
        Err(e) => Err(format!("{}: {}", path, e)),
    }
}

struct Stamp {
    line: usize,
    iteration: String,
    stage: String,
}

// spec: lifecycle-kit/SPEC.md §check-audit-roster — the pure half: assertions A, B and C over
// the text, returning the findings and the current-iteration-checkable stamps for D
fn check(text: &str, cap: usize, stages: &[String]) -> (Vec<(usize, String)>, Vec<Stamp>, usize) {
    let mut findings: Vec<(usize, String)> = Vec::new();
    let mut stamps: Vec<Stamp> = Vec::new();
    let mut blocks: Vec<Vec<(usize, &str)>> = Vec::new();
    let mut cur: Vec<(usize, &str)> = Vec::new();
    let mut in_header = true;
    for (idx, line) in text.lines().enumerate() {
        let fnr = idx + 1;
        // assertion B: no line exceeds the configured byte cap
        if line.len() > cap {
            findings.push((
                fnr,
                format!("line is {} bytes, over the {}-byte cap", line.len(), cap),
            ));
        }
        if line.trim().is_empty() {
            if !cur.is_empty() {
                blocks.push(std::mem::take(&mut cur));
            }
            continue;
        }
        if in_header && line.starts_with('#') {
            continue;
        }
        in_header = false;
        cur.push((fnr, line));
    }
    if !cur.is_empty() {
        blocks.push(cur);
    }

    let mut classes: Vec<(String, usize)> = Vec::new();
    for b in &blocks {
        let first = b[0].0;
        // assertion A: the keys in order, one per line, no stray line, every value well-formed
        let mut want: Vec<&str> = HEAD.to_vec();
        let (mut i, mut at) = (0usize, 0usize);
        while i < want.len() {
            let Some(&(fnr, line)) = b.get(at) else {
                findings.push((first, format!("block is missing its '{}:' line", want[i])));
                i += 1;
                continue;
            };
            let Some((key, val)) = key_line(line) else {
                findings.push((
                    fnr,
                    format!(
                        "expected the '{}:' line here, found a line with no '<key>:' shape",
                        want[i]
                    ),
                ));
                i += 1;
                at += 1;
                continue;
            };
            if key != want[i] {
                if let Some(ahead) = want[i + 1..].iter().position(|w| *w == key) {
                    for w in &want[i..i + 1 + ahead] {
                        findings.push((
                            fnr,
                            format!("block is missing its '{}:' line before '{}:'", w, key),
                        ));
                    }
                    i += 1 + ahead;
                } else {
                    findings.push((
                        fnr,
                        format!("expected the '{}:' line here, found '{}:'", want[i], key),
                    ));
                    i += 1;
                    at += 1;
                }
                continue;
            }
            if val.is_empty() {
                let remedy = if key == "declined" {
                    " — write the literal 'none' when nothing was declined"
                } else {
                    ""
                };
                findings.push((fnr, format!("empty {}{}", key, remedy)));
            }
            match key {
                "class" if !val.is_empty() => classes.push((val.to_string(), fnr)),
                "last" if !val.is_empty() && val != "never" => {
                    want.extend_from_slice(&SWEPT);
                    let parts: Vec<&str> = val.split_whitespace().collect();
                    if parts.len() != 2 {
                        findings.push((
                            fnr,
                            format!(
                                "last is neither 'never' nor '<iteration> <stage>': '{}'",
                                val
                            ),
                        ));
                    } else if !stages::stage_known(stages, parts[1]) {
                        findings.push((fnr, format!("last names '{}', which is not a configured stage (LIFECYCLE_KIT_STAGES)", parts[1])));
                    } else {
                        stamps.push(Stamp {
                            line: fnr,
                            iteration: parts[0].to_string(),
                            stage: parts[1].to_string(),
                        });
                    }
                }
                "corpus" => {
                    if let Some(list) = val.strip_prefix("surfaces:") {
                        if list.trim().is_empty() {
                            findings.push((fnr, "corpus 'surfaces:' names no surface".to_string()));
                        }
                    }
                }
                "hits" if !val.is_empty() && !val.bytes().all(|c| c.is_ascii_digit()) => {
                    findings.push((fnr, format!("hits is not a decimal integer: '{}'", val)));
                }
                _ => {}
            }
            i += 1;
            at += 1;
        }
        for &(fnr, _) in b.iter().skip(at) {
            findings.push((fnr, "stray line after the block's last key".to_string()));
        }
    }
    // assertion C: class slugs are unique
    for (n, (c, fnr)) in classes.iter().enumerate() {
        if let Some((_, prior)) = classes[..n].iter().find(|(p, _)| p == c) {
            findings.push((
                *fnr,
                format!("duplicate class '{}' (first at line {})", c, prior),
            ));
        }
    }
    findings.sort_by_key(|f| f.0);
    (findings, stamps, blocks.len())
}

// spec: lifecycle-kit/SPEC.md §check-audit-roster — assertion D's state read: a data line whose
// first two fields are the iteration and the stage
fn stamped(state: &str, iteration: &str, stage: &str) -> bool {
    stages::data_lines(state).iter().any(|l| {
        let mut f = l.split_whitespace();
        f.next() == Some(iteration) && f.next() == Some(stage)
    })
}

fn fail(msg: String) -> i32 {
    eprintln!("check-audit-roster: {}", msg);
    2
}

pub fn run(args: &[String]) -> i32 {
    let hermetic = args.first().filter(|a| !a.is_empty()).cloned();
    let roster = match &hermetic {
        Some(p) => {
            if !Path::new(p).is_file() {
                return fail(format!("roster file not found: {}", p));
            }
            p.clone()
        }
        None => match walk::knob_scalar("LIFECYCLE_KIT_AUDIT_ROSTER_FILE") {
            Ok(v) if v.is_empty() => {
                println!("AUDIT-ROSTER: clean (LIFECYCLE_KIT_AUDIT_ROSTER_FILE is empty — no roster configured)");
                return 0;
            }
            Ok(v) => v,
            Err(e) => return fail(e),
        },
    };
    let cap = match walk::knob_scalar("LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP") {
        Ok(v) => match v.parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => {
                return fail(format!(
                    "LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP is not a positive integer: {}",
                    v
                ))
            }
        },
        Err(e) => return fail(e),
    };
    let stage_roster = match stages::stages() {
        Ok(s) => s,
        Err(e) => return fail(e),
    };
    let text = match read(&roster) {
        Ok(Read::Text(t)) => t,
        Ok(Read::Absent) => {
            println!("AUDIT-ROSTER: clean (no roster at {} — inert)", roster);
            return 0;
        }
        Err(e) => return fail(format!("roster file not readable: {}", e)),
    };

    let (mut findings, stamps, blocks) = check(&text, cap, &stage_roster);

    let mut d_checked = 0usize;
    if hermetic.is_none() && !stamps.is_empty() {
        let queue_path = match walk::knob_scalar("LIFECYCLE_KIT_QUEUE_FILE") {
            Ok(v) => v,
            Err(e) => return fail(e),
        };
        let current = match read(&queue_path) {
            Ok(Read::Text(q)) => stages::header(&q).map(stages::header_iter),
            Ok(Read::Absent) => None,
            Err(e) => return fail(format!("queue file not readable: {}", e)),
        };
        if let Some(current) = current.filter(|c| !c.is_empty()) {
            let state_path = match walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE") {
                Ok(v) => v,
                Err(e) => return fail(e),
            };
            let state = match read(&state_path) {
                Ok(Read::Text(s)) => s,
                Ok(Read::Absent) => String::new(),
                Err(e) => return fail(format!("state file not readable: {}", e)),
            };
            // assertion D: a current-iteration last names a stage the state file stamped
            for s in stamps.iter().filter(|s| s.iteration == current) {
                d_checked += 1;
                if !stamped(&state, &s.iteration, &s.stage) {
                    findings.push((
                        s.line,
                        format!(
                            "last names '{} {}', but {} carries no '{} {}' stamp — the named stage never ran this iteration",
                            s.iteration, s.stage, state_path, s.iteration, s.stage
                        ),
                    ));
                }
            }
        }
    }

    if !findings.is_empty() {
        println!(
            "check-audit-roster: {} finding(s) in {}:",
            findings.len(),
            roster
        );
        for (l, w) in &findings {
            println!("  {}:{}: {}", roster, l, w);
        }
        println!("  help: each block is 'class:', 'scope:', 'due:', 'last:' and, unless last is 'never', 'corpus:', 'hits:', 'declined:' — one line each, in that order, blocks separated by a blank line; hits a decimal integer, declined the literal 'none' when nothing was set aside, last '<iteration> <stage>' naming a configured stage the state file stamped when it is the current iteration, every line within LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP bytes, and each class once. A sweep replaces its block's attestation lines and sends its narration to the commit message (lifecycle-kit/SPEC.md §The audit roster).");
        return 1;
    }
    if blocks == 0 {
        println!("AUDIT-ROSTER: clean (no class block in {} — inert)", roster);
    } else if hermetic.is_some() {
        println!(
            "AUDIT-ROSTER: clean ({} block(s) in {}; grammar, cap and class uniqueness hold — hermetic file argument, so no stamp check)",
            blocks, roster
        );
    } else {
        println!(
            "AUDIT-ROSTER: clean ({} block(s) in {}; grammar, cap and class uniqueness hold, {} current-iteration last stamp(s) checked against the state file)",
            blocks, roster, d_checked
        );
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stages() -> Vec<String> {
        ["scope", "build", "close"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn a_never_block_closes_after_four_keys_and_a_swept_one_after_seven() {
        let t = "# contract: x\n\nclass: a\nscope: s\ndue: d\nlast: never\n\nclass: b\nscope: s\ndue: d\nlast: it close\ncorpus: surfaces: x, y\nhits: 3\ndeclined: none\n";
        let (f, st, n) = check(t, 1500, &stages());
        assert!(f.is_empty(), "{:?}", f);
        assert_eq!((st.len(), n), (1, 2));
    }

    #[test]
    fn a_dropped_due_an_empty_declined_and_a_stray_line_each_red() {
        let t = "class: a\nscope: s\nlast: never\n\nclass: b\nscope: s\ndue: d\nlast: it close\ncorpus: c\nhits: x\ndeclined:\nmore\n";
        let (f, _, _) = check(t, 1500, &stages());
        let w: Vec<&str> = f.iter().map(|x| x.1.as_str()).collect();
        assert!(w
            .iter()
            .any(|m| m.contains("missing its 'due:' line before 'last:'")));
        assert!(w.iter().any(|m| m.starts_with("empty declined")));
        assert!(w.iter().any(|m| m.contains("not a decimal integer")));
        assert!(w.iter().any(|m| m.starts_with("stray line")));
    }

    #[test]
    fn the_cap_the_stage_roster_and_class_uniqueness_are_asserted() {
        let t = format!("class: a\nscope: {}\ndue: d\nlast: it validate\ncorpus: c\nhits: 1\ndeclined: none\n\nclass: a\nscope: s\ndue: d\nlast: never\n", "x".repeat(40));
        let (f, _, _) = check(&t, 30, &stages());
        let w: Vec<&str> = f.iter().map(|x| x.1.as_str()).collect();
        assert!(w.iter().any(|m| m.contains("over the 30-byte cap")));
        assert!(w.iter().any(|m| m.contains("not a configured stage")));
        assert!(w.iter().any(|m| m.starts_with("duplicate class 'a'")));
    }

    #[test]
    fn a_stamp_matches_on_the_first_two_fields_below_the_separator() {
        let s = "header\n---\nit build sid 2026-01-01 abcdef1\n";
        assert!(stamped(s, "it", "build"));
        assert!(!stamped(s, "it", "close"));
        assert!(!stamped("it build x\n", "it", "build"));
    }
}
