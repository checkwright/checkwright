// spec: lifecycle-kit/SPEC.md §lib/stages.sh — the Rust counterpart of the stage machine's
// shared surface: the derived stage roster, the two boundary sets, the registration block.
// The shell library is not retired, so this module sits beside it rather than replacing it
use crate::walk;

pub fn stages() -> Result<Vec<String>, String> {
    walk::knob_array("LIFECYCLE_KIT_STAGES")
}

pub fn stage_known(stages: &[String], s: &str) -> bool {
    stages.iter().any(|x| x == s)
}

// spec: lifecycle-kit/SPEC.md §Multi-operator semantics — the iteration-scoped supersede set,
// derived here so check-merge-attrs and check-scratch-citation read one set rather than two
// rosters that could drift
pub fn supersede_set() -> Result<Vec<String>, String> {
    let mut out = vec![
        walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE")?,
        walk::knob_scalar("LIFECYCLE_KIT_LESSON_EVIDENCE_FILE")?,
        walk::knob_scalar("LIFECYCLE_KIT_SURVEY_RECORD_FILE")?,
    ];
    out.extend(walk::knob_array("LIFECYCLE_KIT_BOUNDARY_TRUNCATE")?);
    Ok(out)
}

// spec: lifecycle-kit/SPEC.md §The committed gap inbox — the union-merge set, distinct from
// the keep-ours supersede set above
pub fn union_set() -> Result<Vec<String>, String> {
    Ok(vec![walk::knob_scalar("LIFECYCLE_KIT_GAP_INBOX_FILE")?])
}

// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the resident registration block
// rendered from the live config, the derivation check-lifecycle-registration byte-compares
// against; the roster is the stage set as skill invocations, never hand-listed
pub fn registration_block() -> Result<String, String> {
    let queue = walk::knob_scalar("LIFECYCLE_KIT_QUEUE_FILE")?;
    let roster: Vec<String> = stages()?.iter().map(|s| format!("`/{}`", s)).collect();
    Ok(format!(
        "The repo runs lifecycle-kit's iteration state machine on `{}` — one\n\
         stage session per stage, each invoking its skill:\n\
         {}.\n\
         The state machine, its stamp protocol, and the per-stage contracts:\n\
         [lifecycle-kit/SPEC.md](lifecycle-kit/SPEC.md).",
        queue,
        roster.join(" ")
    ))
}

// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the .gitattributes merge-driver lines:
// one `<path> merge=iteration-scoped` per supersede member then one `<path> merge=union` per
// union member, off the two set derivations above, so writer and asserter cannot drift
pub fn merge_attrs_block() -> Result<String, String> {
    let mut out = String::new();
    for p in supersede_set()? {
        out.push_str(&format!("{} merge=iteration-scoped\n", p));
    }
    for p in union_set()? {
        out.push_str(&format!("{} merge=union\n", p));
    }
    Ok(out)
}

pub fn header(text: &str) -> Option<&str> {
    text.lines().find(|l| l.starts_with("## Iteration:"))
}

// spec: lifecycle-kit/SPEC.md §lib/stages.sh — the trailing-bracket strip is residual-field
// healing: a pre-upgrade header still carrying [stage:] yields the bare name
pub fn header_iter(hdr: &str) -> String {
    let mut s = hdr.strip_prefix("## Iteration:").unwrap_or(hdr);
    s = s.trim_start_matches([' ', '\t']);
    match s.find("[stage:") {
        Some(i) => s[..i].trim_end_matches([' ', '\t']).to_string(),
        None => s.to_string(),
    }
}

// spec: lifecycle-kit/SPEC.md §lib/stages.sh — the data lines of the state file: everything
// non-blank below the `---` separator, the one derivation every lifecycle reader shares
pub fn data_lines(text: &str) -> Vec<&str> {
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

// spec: lifecycle-kit/SPEC.md §lib/stages.sh — the cursor: the last data line's stage token.
// Empty for both no-cursor shapes (absent file, no data line yet) — "no cursor" is a
// legitimate state, not an error, and each caller decides what it means.
pub fn current_stage(text: &str) -> String {
    match data_lines(text).last() {
        Some(l) => l.split_whitespace().nth(1).unwrap_or("").to_string(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cursor_is_the_last_data_lines_stage_and_absent_shapes_are_empty() {
        assert_eq!(current_stage("# hdr\n---\n\na b c d\ne f g h\n"), "f");
        assert_eq!(current_stage("# hdr\na b c d\n"), "");
        assert_eq!(current_stage(""), "");
        assert_eq!(current_stage("---\nonlyonefield\n"), "");
    }

    // spec: lifecycle-kit/SPEC.md §lib/stages.sh — the residual [stage:] field is healed away
    // rather than reported, so a consumer upgrades mid-iteration without a red
    #[test]
    fn the_iteration_name_survives_a_residual_stage_field() {
        assert_eq!(header_iter("## Iteration: alpha"), "alpha");
        assert_eq!(header_iter("## Iteration:   alpha  [stage: build]"), "alpha");
        assert_eq!(header_iter("## Iteration: —"), "—");
    }
}
