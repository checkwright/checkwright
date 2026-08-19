// spec: evidence-kit/SPEC.md §lib/evidence.sh — the binary side of evidence-kit's own library:
// the readers its gates share, kept here rather than reached for in `crate::stages` so the
// compiled form inherits the shell library's deliberate independence from lifecycle-kit

// spec: evidence-kit/SPEC.md §Evidence manifest — the versioned wire format the header
// declares. The spec owns the value; the shell library and this const are its two
// implementations, and the unit test below is what holds them equal.
pub const MANIFEST_CONTRACT: &str = "evidence-manifest v1";

// spec: evidence-kit/SPEC.md §check-evidence-manifest — `ek_data_lines`: everything but a
// comment line and a blank one. Distinct from `crate::stages::data_lines` — same name,
// different primitive, and that section owns why binding to the other one is silent.
pub fn data_lines(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| {
            let t = l.trim_start_matches([' ', '\t']);
            !t.is_empty() && !t.starts_with('#')
        })
        .collect()
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_queue_iteration`: the first `## Iteration:`
// line with its lead and any residual `[stage:` field stripped. `None` is the helper's non-zero
// return — an absent file or no header at all — where an empty string is a header with no name.
pub fn queue_iteration(text: &str) -> Option<String> {
    let hdr = text.lines().find(|l| l.starts_with("## Iteration:"))?;
    let mut s = hdr.strip_prefix("## Iteration:").unwrap_or(hdr);
    s = s.trim_start_matches([' ', '\t']);
    Some(match s.find("[stage:") {
        Some(i) => s[..i].trim_end_matches([' ', '\t']).to_string(),
        None => s.to_string(),
    })
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_state_stage`'s corpus: every non-blank line
// below the `---` separator, which is also the set assertion C's validate-stamp scan reads
pub fn state_lines(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut seen = false;
    for line in text.lines() {
        if !seen {
            if line.starts_with("---") && line[3..].chars().all(|c| c == ' ' || c == '\t') {
                seen = true;
            }
            continue;
        }
        if line.split_whitespace().next().is_some() {
            out.push(line);
        }
    }
    out
}

// spec: evidence-kit/SPEC.md §lib/evidence.sh — `ek_state_stage`: the last data line's second
// field. `None` on all three non-zero shapes — absent file, no data line, no second field.
pub fn state_stage(text: &str) -> Option<String> {
    let last = state_lines(text).last().copied()?;
    last.split_whitespace().nth(1).map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: evidence-kit/SPEC.md §Evidence manifest — the wire-format version has two
    // implementations, so the crate's copy is held to the shell library's by executing it.
    // A static roster would be a third holder of the same value.
    #[test]
    fn the_wire_contract_matches_the_shell_library() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let completed = crate::proc::run(
            "bash",
            &[
                "-c",
                "cd \"$1\" || exit 2; . evidence-kit/lib/evidence.sh; \
                 printf '%s' \"$EVIDENCE_MANIFEST_CONTRACT\"",
                "bash",
                &repo.display().to_string(),
            ],
        )
        .expect("cannot run the shell library");
        let out = completed
            .stdout()
            .expect("evidence-kit/lib/evidence.sh could not report the wire contract");
        assert_eq!(String::from_utf8_lossy(out), MANIFEST_CONTRACT);
    }

    #[test]
    fn comments_and_blanks_are_not_data_lines() {
        assert_eq!(data_lines("# h\n\n  \nu a pass\n  # c\n"), vec!["u a pass"]);
    }

    // spec: evidence-kit/SPEC.md §lib/evidence.sh — the two readers part company on shape:
    // the iteration is a header field, the stage a positional on the last data line
    #[test]
    fn the_iteration_and_the_cursor_are_read_from_their_own_shapes() {
        assert_eq!(
            queue_iteration("# q\n## Iteration: alpha  [stage: build]\n").as_deref(),
            Some("alpha")
        );
        assert_eq!(queue_iteration("## Iteration:").as_deref(), Some(""));
        assert_eq!(queue_iteration("# q\n"), None);
        assert_eq!(
            state_stage("h\n---\nit scope s1 d\nit close s3 d\n").as_deref(),
            Some("close")
        );
        assert_eq!(state_stage("h\n---\n"), None);
        assert_eq!(state_stage("h\nit close s3 d\n"), None);
        assert_eq!(state_stage("h\n---\nlonely\n"), None);
    }
}
