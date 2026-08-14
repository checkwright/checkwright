// spec: canon-kit/SPEC.md §check-spec-derivable-section — a banned-heading section may not
// be a fenced code dump above the density budget; the pointer exemption is a consumer ERE,
// so the engine applies it rather than a hand-compiled kit literal
use crate::spec::{self, compile_pattern as compile};
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-spec-derivable-section: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-spec-derivable-section — one section's accumulator, flushed
// at every heading, at each file's first line and at end of input, exactly where the shell's
// `flush()` runs
#[derive(Default)]
struct Section {
    heading: String,
    file: String,
    line: usize,
    banned: bool,
    has_pointer: bool,
    body_nonblank: usize,
    fenced_nonblank: usize,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let spec_name = spec::knob_pub("CANON_KIT_SPEC_NAME")?;
    let specs = spec::canonical_specs_sorted(root)?;
    if specs.is_empty() {
        println!("SPEC-DERIVABLE-SECTION: clean (0 {} found)", spec_name);
        return Ok(0);
    }

    let banned_headings = spec::knob_array_pub("CANON_KIT_BANNED_HEADINGS")?;
    let banned: Vec<String> = banned_headings
        .iter()
        .map(|h| h.to_ascii_lowercase())
        .collect();
    let density: u64 = spec::knob_pub("CANON_KIT_DERIVABLE_DENSITY")?
        .parse()
        .map_err(|_| "CANON_KIT_DERIVABLE_DENSITY is not a number".to_string())?;
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the consumer pattern compiles before the
    // first corpus line is read, so a construct the substrate refuses exits 2 naming the knob
    // rather than scanning clean past the sections it meant to exempt
    let pointer_src = spec::knob_pub("CANON_KIT_DERIVABLE_POINTER_REGEX")?;
    let pointer = compile(&pointer_src, "CANON_KIT_DERIVABLE_POINTER_REGEX")?;

    let mut findings: Vec<String> = Vec::new();
    let mut cur = Section::default();
    for f in &specs {
        flush(&mut cur, &mut findings, density);
        let mut in_fence = false;
        for (idx, raw) in spec::read_text(Path::new(f))?.lines().enumerate() {
            let fnr = idx + 1;
            if raw.starts_with("```") {
                in_fence = !in_fence;
                if !cur.heading.is_empty() {
                    cur.body_nonblank += 1;
                    cur.fenced_nonblank += 1;
                }
                continue;
            }
            if !in_fence {
                if let Some(h) = heading_text(raw) {
                    flush(&mut cur, &mut findings, density);
                    cur.banned = banned.contains(&h.to_ascii_lowercase());
                    cur.heading = h;
                    cur.file = f.clone();
                    cur.line = fnr;
                    continue;
                }
            }
            if !cur.heading.is_empty() && raw.bytes().any(|c| !is_space(c)) {
                cur.body_nonblank += 1;
                if in_fence {
                    cur.fenced_nonblank += 1;
                }
                if pointer.is_match(raw) {
                    cur.has_pointer = true;
                }
            }
        }
    }
    flush(&mut cur, &mut findings, density);

    if !findings.is_empty() {
        println!("SPEC-DERIVABLE-SECTION: {} violation(s):", findings.len());
        for l in &findings {
            println!("  {}", l);
        }
        println!("  help: a banned-heading section ({}) that is mostly a code dump drifts — shed the body to a one-line index pointer (cite the code), keeping the prose that owns semantics", banned_headings.join(" "));
        return Ok(1);
    }
    println!(
        "SPEC-DERIVABLE-SECTION: clean ({} {}, no banned-heading section exceeds the {}% fenced budget)",
        specs.len(),
        spec_name,
        density
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-spec-derivable-section — the density test is the shell's
// cross-multiplied one, so no division rounds the comparison; the reported percentage is the
// separately truncated `int(fenced * 100 / body)`
fn flush(cur: &mut Section, findings: &mut Vec<String>, density: u64) {
    if cur.heading.is_empty() {
        return;
    }
    if cur.banned && !cur.has_pointer && cur.body_nonblank > 0 {
        let fenced = cur.fenced_nonblank as u64;
        let body = cur.body_nonblank as u64;
        if fenced * 100 > density * body {
            findings.push(format!(
                "{}: section \"{}\" (line {}) is {}% fenced (banned heading, budget {}%) — shed to a one-line index pointer",
                cur.file,
                cur.heading,
                cur.line,
                fenced * 100 / body,
                density
            ));
        }
    }
    *cur = Section::default();
}

// spec: canon-kit/SPEC.md §check-spec-derivable-section — `^#{1,6}[[:space:]]`, then
// `sub(/^#{1,6}[[:space:]]+/, "", h)` and the trailing-space strip. A run of seven or more
// `#` is not a heading, because no prefix of 1..6 is followed by whitespace.
fn heading_text(line: &str) -> Option<String> {
    let b = line.as_bytes();
    let mut n = 0usize;
    while n < b.len() && b[n] == b'#' {
        n += 1;
    }
    if !(1..=6).contains(&n) {
        return None;
    }
    if !matches!(b.get(n), Some(c) if is_space(*c)) {
        return None;
    }
    let mut i = n;
    while i < b.len() && is_space(b[i]) {
        i += 1;
    }
    let mut e = b.len();
    while e > i && is_space(b[e - 1]) {
        e -= 1;
    }
    Some(line[i..e].to_string())
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_heading_is_one_to_six_hashes_and_the_text_is_trimmed_both_ends() {
        assert_eq!(heading_text("# Public API  ").as_deref(), Some("Public API"));
        assert_eq!(heading_text("###### Public API").as_deref(), Some("Public API"));
        assert_eq!(heading_text("####### Public API"), None);
        assert_eq!(heading_text("#Public API"), None);
        assert_eq!(heading_text("no heading"), None);
    }

    // spec: canon-kit/SPEC.md §check-spec-derivable-section — the budget is a strict
    // greater-than on the cross-multiplied comparison, so a section exactly at the budget
    // passes; reproduced as a test because the boundary is the one a rewrite would move
    #[test]
    fn a_section_exactly_at_the_budget_passes_and_one_line_over_reds() {
        let mut findings = Vec::new();
        let at = Section {
            heading: "Public API".into(),
            file: "SPEC.md".into(),
            line: 1,
            banned: true,
            body_nonblank: 10,
            fenced_nonblank: 6,
            ..Default::default()
        };
        let mut cur = at;
        flush(&mut cur, &mut findings, 60);
        assert!(findings.is_empty(), "60% against a 60% budget is not over");

        let mut cur = Section {
            heading: "Public API".into(),
            file: "SPEC.md".into(),
            line: 1,
            banned: true,
            body_nonblank: 10,
            fenced_nonblank: 7,
            ..Default::default()
        };
        flush(&mut cur, &mut findings, 60);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].contains("is 70% fenced"), "{}", findings[0]);
    }

    // spec: canon-kit/SPEC.md §check-spec-derivable-section — the pointer exemption and the
    // banned-heading test are both required, so neither alone suppresses or raises a finding
    #[test]
    fn an_unbanned_heading_and_a_pointer_bearing_section_both_stay_silent() {
        let mut findings = Vec::new();
        let mut cur = Section {
            heading: "Prose".into(),
            banned: false,
            body_nonblank: 10,
            fenced_nonblank: 10,
            ..Default::default()
        };
        flush(&mut cur, &mut findings, 60);
        let mut cur = Section {
            heading: "Public API".into(),
            banned: true,
            has_pointer: true,
            body_nonblank: 10,
            fenced_nonblank: 10,
            ..Default::default()
        };
        flush(&mut cur, &mut findings, 60);
        assert!(findings.is_empty());
    }
}
