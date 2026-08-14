// spec: canon-kit/SPEC.md §check-spec-dod-singleton — a canonical spec carries the
// Definition-of-Done heading the configured number of times
use crate::spec;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-spec-dod-singleton: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let spec_name = spec::knob_pub("CANON_KIT_SPEC_NAME")?;
    let specs = spec::canonical_specs_sorted(root)?;
    if specs.is_empty() {
        println!("SPEC-DOD-SINGLETON: clean (0 {} found)", spec_name);
        return Ok(0);
    }

    let heading = spec::knob_pub("CANON_KIT_DOD_HEADING")?;
    let want = heading.to_ascii_lowercase();
    let mode = spec::knob_pub("CANON_KIT_DOD_MODE")?;

    let mut errors: Vec<String> = Vec::new();
    for f in &specs {
        let n = dod_headings(&spec::read_text(Path::new(f))?, &want);
        // spec: canon-kit/SPEC.md §check-spec-dod-singleton — `at-most-one` exists because a
        // reference-spec corpus legitimately has none, so only the doubled case is a finding
        // there; an unrecognised mode is neither branch, exactly as the shell reads it
        let over = match mode.as_str() {
            "exactly-one" => n != 1,
            "at-most-one" => n > 1,
            _ => false,
        };
        if over {
            let bound = if mode == "exactly-one" {
                "need exactly 1"
            } else {
                "need at most 1"
            };
            errors.push(format!(
                "{} has {} \"{}\" heading(s) ({})",
                f, n, heading, bound
            ));
        }
    }

    if !errors.is_empty() {
        println!("SPEC-DOD-SINGLETON: {} violation(s) ({}):", errors.len(), mode);
        for e in &errors {
            println!("  {}", e);
        }
        // spec: canon-kit/SPEC.md §check-spec-dod-singleton — the shell appends the
        // add-one clause under `${CANON_KIT_DOD_MODE:+…}`, so an empty mode drops it
        let tail = if mode.is_empty() {
            ""
        } else {
            " (add one if missing under exactly-one)"
        };
        println!("  help: a duplicate Definition-of-Done checklist is two sources on the completion contract — fold the doubled/appended one into the canonical '## {}' heading{}", heading, tail);
        return Ok(1);
    }
    println!(
        "SPEC-DOD-SINGLETON: clean ({} {} scanned, {})",
        specs.len(),
        spec_name,
        mode
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-spec-dod-singleton — heading match is level-insensitive
// within `##`–`####` and the comparison is containment on the lowercased heading text, so a
// decorated Definition-of-Done heading still counts
fn dod_headings(text: &str, want: &str) -> usize {
    let mut c = 0usize;
    for line in text.lines() {
        if let Some(h) = heading_text_2_4(line) {
            if h.to_ascii_lowercase().contains(want) {
                c += 1;
            }
        }
    }
    c
}

// spec: canon-kit/SPEC.md §check-spec-dod-singleton — `/^#{2,4}[[:space:]]/` and the
// `sub(/^#{2,4}[[:space:]]+/, "", h)` that follows it: a run of five or more `#` matches
// neither, because no prefix of 2..4 is followed by whitespace
fn heading_text_2_4(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let mut n = 0usize;
    while n < b.len() && b[n] == b'#' {
        n += 1;
    }
    if !(2..=4).contains(&n) {
        return None;
    }
    if !matches!(b.get(n), Some(c) if is_space(*c)) {
        return None;
    }
    let mut i = n;
    while i < b.len() && is_space(b[i]) {
        i += 1;
    }
    Some(&line[i..])
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_two_to_four_hashes_are_headings_and_the_text_is_stripped() {
        assert_eq!(heading_text_2_4("## Definition of Done"), Some("Definition of Done"));
        assert_eq!(heading_text_2_4("####\tDefinition of Done"), Some("Definition of Done"));
        assert_eq!(heading_text_2_4("# Definition of Done"), None);
        assert_eq!(heading_text_2_4("##### Definition of Done"), None);
        assert_eq!(heading_text_2_4("##Definition of Done"), None);
    }

    #[test]
    fn the_match_is_containment_on_the_lowercased_heading() {
        let text = "## Definition of Done\n\n### the DEFINITION OF DONE, restated\n#### nope\n";
        assert_eq!(dod_headings(text, "definition of done"), 2);
    }
}
