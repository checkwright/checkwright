// spec: lifecycle-kit/SPEC.md §The stage-machine adapters — the sole holder of the stage machine's
// shared surface: the derived stage roster, the two boundary sets, the registration block
use crate::{proc, programs};
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
        "The repo runs lifecycle-kit's iteration state machine on `{}` — one \
         stage session per stage, each invoking its skill: \
         {}. \
         The state machine, its stamp protocol, and the per-stage contracts: \
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

// spec: lifecycle-kit/SPEC.md §The stage-machine adapters — the trailing-bracket strip is residual-field
// healing: a pre-upgrade header still carrying [stage:] yields the bare name
pub fn header_iter(hdr: &str) -> String {
    let mut s = hdr.strip_prefix("## Iteration:").unwrap_or(hdr);
    s = s.trim_start_matches([' ', '\t']);
    match s.find("[stage:") {
        Some(i) => s[..i].trim_end_matches([' ', '\t']).to_string(),
        None => s.to_string(),
    }
}

// spec: lifecycle-kit/SPEC.md §The stage-machine adapters — the data lines of the state file: everything
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

// spec: lifecycle-kit/SPEC.md §The stage-machine adapters — the cursor: the last data line's stage token.
// Empty for both no-cursor shapes (absent file, no data line yet) — "no cursor" is a
// legitimate state, not an error, and each caller decides what it means.
pub fn current_stage(text: &str) -> String {
    match data_lines(text).last() {
        Some(l) => l.split_whitespace().nth(1).unwrap_or("").to_string(),
        None => String::new(),
    }
}

// spec: lifecycle-kit/SPEC.md §The stage-machine adapters — the iteration-start read's pure half: field five of
// the first data line, and only in the abbreviated-hex shape `--enter-stage` writes, so `none`, a
// four-field line and the no-cursor shapes all read as no commit
pub fn first_head(text: &str) -> String {
    let head = data_lines(text)
        .first()
        .and_then(|l| l.split_whitespace().nth(4))
        .unwrap_or("");
    let hex = head.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c));
    if (7..=40).contains(&head.len()) && hex {
        head.to_string()
    } else {
        String::new()
    }
}

// spec: lifecycle-kit/SPEC.md §check-stamp-subject — the stamp a write adds: the last data line of
// `after` absent from `before` whose stage field is a roster member. The writer names its subject
// from it and the gate asserts against it, so the two cannot disagree about which stage decides.
pub fn last_added_stamp<'a>(after: &'a str, before: &str, stages: &[String]) -> Option<&'a str> {
    last_added_stamp_over(after, &[before], stages)
}

// spec: lifecycle-kit/SPEC.md §check-stamp-subject — a merge inherits the stamps its parents carry,
// so the prior set is the union of every prior version's data lines
pub fn last_added_stamp_over<'a>(after: &'a str, befores: &[&str], stages: &[String]) -> Option<&'a str> {
    let prior: Vec<&str> = befores.iter().flat_map(|b| data_lines(b)).collect();
    data_lines(after)
        .into_iter()
        .rfind(|l| !prior.contains(l) && stage_known(stages, stamp_stage(l)))
}

fn marker_raw(text: &str) -> Vec<&str> {
    text.lines().map(str::trim).filter(|l| !l.is_empty()).collect()
}

fn marker_stage(line: &str) -> &str {
    line.split_whitespace().next().unwrap_or("")
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — a waiver-bearing line is `<stage>
// <waiver-token> <reason>`; `Some(reason)` for one, `None` for a bare line or an empty token
fn marker_reason(line: &str, token: &str) -> Option<String> {
    let f: Vec<&str> = line.split_whitespace().collect();
    (!token.is_empty() && f.get(1) == Some(&token)).then(|| f[2..].join(" "))
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the stage each marker line names, read off its
// first field; its writer and check-dispatch-entry read this one spelling
pub fn marker_lines(text: &str) -> Vec<&str> {
    marker_raw(text).into_iter().map(marker_stage).collect()
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the line a discharge of `stage` removes: a
// waiver-bearing one before a bare one
fn marker_pick(lines: &[&str], stage: &str, token: &str) -> Option<usize> {
    let named = |l: &&str| marker_stage(l) == stage;
    lines
        .iter()
        .position(|l| named(l) && marker_reason(l, token).is_some())
        .or_else(|| lines.iter().position(named))
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the waiver reason the entry of `stage` would
// consume, read without removing anything
pub fn marker_waiver(text: &str, stage: &str, token: &str) -> Option<String> {
    let lines = marker_raw(text);
    marker_pick(&lines, stage, token).and_then(|at| marker_reason(lines[at], token))
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the marker with one line naming `stage`
// removed, and that line's waiver reason if it bore one; `None` when no line names it
pub fn marker_without(text: &str, stage: &str, token: &str) -> Option<(Vec<String>, Option<String>)> {
    let lines = marker_raw(text);
    let at = marker_pick(&lines, stage, token)?;
    let kept = lines
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != at)
        .map(|(_, l)| l.to_string())
        .collect();
    Some((kept, marker_reason(lines[at], token)))
}

pub fn stamp_stage(line: &str) -> &str {
    line.split_whitespace().nth(1).unwrap_or("")
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the subjects the writer prints, one per
// stamp-writing path, each scoped to the stage the gate will read off the added stamp
pub fn entry_subject(stage: &str) -> String {
    format!("chore({}): stamp the {} stage entry", stage, stage)
}

pub fn boundary_subject(stage: &str) -> String {
    format!("{} at the iteration boundary", entry_subject(stage))
}

pub fn rename_subject(stage: &str, name: &str) -> String {
    format!("chore({}): name the iteration {}", stage, name)
}

pub fn commit_resolves(commit: &str) -> bool {
    proc::run(
        &programs::GIT,
        &["rev-parse", "-q", "--verify", &format!("{}^{{commit}}", commit)],
    )
    .map(|c| c.stdout().is_some())
    .unwrap_or(false)
}

// spec: lifecycle-kit/SPEC.md §The state machine — the iteration-start commit, empty in every
// no-commit case that section lists; the caller hands in the path its own knob resolved
pub fn iteration_start(state_file: &str) -> String {
    let text = match std::fs::read(state_file) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => return String::new(),
    };
    let head = first_head(&text);
    if head.is_empty() || !commit_resolves(&head) {
        return String::new();
    }
    head
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the header run stops at a markdown '## '
// section heading as well as at the first data line: on a markdown surface whose blocks are '## '
// headings (the survey record) a bare /^#/ predicate reads the first block's heading as header
fn is_header_line(l: &str) -> bool {
    let mut b = l.bytes();
    b.next() == Some(b'#') && b.next() != Some(b'#')
}

pub fn truncate_to_header(text: &str) -> String {
    let mut out = String::new();
    let mut pend = String::new();
    for l in text.lines() {
        if l.trim().is_empty() {
            pend.push_str(l);
            pend.push('\n');
            continue;
        }
        if is_header_line(l) {
            out.push_str(&pend);
            pend.clear();
            out.push_str(l);
            out.push('\n');
            continue;
        }
        break;
    }
    out
}

// spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — `empty` is the truncate's own header
// run read as a predicate, so a surface the boundary truncated reads as drained
pub fn header_only(text: &str) -> bool {
    text.lines().all(|l| l.trim().is_empty() || is_header_line(l))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — a discharge removes one line naming the
    // stage, never every such line, and leaves a marker naming no such line alone
    #[test]
    fn a_marker_discharge_removes_exactly_one_line_naming_the_stage() {
        assert_eq!(marker_lines("build\n\n  spec \n"), vec!["build", "spec"]);
        assert_eq!(marker_without("build\nspec\nbuild\n", "build", "align-waived"), Some((vec!["spec".to_string(), "build".to_string()], None)));
        assert_eq!(marker_without("build\n", "build", "align-waived"), Some((Vec::new(), None)));
        assert_eq!(marker_without("spec\n", "build", "align-waived"), None);
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — a waiver-bearing line names its stage in
    // its first field, a discharge takes it ahead of a bare line naming the same stage, and an
    // empty token reads every line as bare
    #[test]
    fn a_waiver_bearing_line_is_read_by_its_first_field_and_discharged_first() {
        let m = "build\nbuild align-waived the operator ruled it\nspec\n";
        assert_eq!(marker_lines(m), vec!["build", "build", "spec"]);
        assert_eq!(marker_waiver(m, "build", "align-waived").as_deref(), Some("the operator ruled it"));
        assert_eq!(marker_waiver(m, "spec", "align-waived"), None);
        assert_eq!(
            marker_without(m, "build", "align-waived"),
            Some((vec!["build".to_string(), "spec".to_string()], Some("the operator ruled it".to_string())))
        );
        assert_eq!(marker_waiver(m, "build", ""), None);
    }

    // spec: lifecycle-kit/SPEC.md §The state machine — the start is the *first* stamp's head, never
    // the cursor's, and every malformed or absent head reads as no commit
    #[test]
    fn the_iteration_start_is_the_first_stamps_head_and_the_no_commit_shapes_are_empty() {
        let hdr = "# hdr\n---\n\n";
        assert_eq!(
            first_head(&format!("{}a scope s1 2026-09-14 d57c4fec\na build s2 2026-09-14 fe2e2066\n", hdr)),
            "d57c4fec"
        );
        assert_eq!(first_head(""), "");
        assert_eq!(first_head("# hdr\n---\n\n"), "");
        assert_eq!(first_head(&format!("{}a scope s1 2026-09-14 none\n", hdr)), "");
        assert_eq!(first_head(&format!("{}a scope s1 2026-09-14\n", hdr)), "");
        assert_eq!(first_head(&format!("{}a scope s1 2026-09-14 D57C4FEC\n", hdr)), "");
        assert_eq!(first_head(&format!("{}a scope s1 2026-09-14 d57c4f\n", hdr)), "");

        let dir = std::env::temp_dir().join(format!("cw-start-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("cannot make the fixture dir");
        let absent = dir.join("absent.txt");
        assert_eq!(iteration_start(&absent.display().to_string()), "");
        let unresolvable = dir.join("state.txt");
        std::fs::write(&unresolvable, format!("{}a scope s1 2026-09-14 0000000\n", hdr))
            .expect("cannot write the fixture state file");
        assert_eq!(iteration_start(&unresolvable.display().to_string()), "");
        let head = proc::run(&programs::GIT, &["rev-parse", "--short", "HEAD"])
            .ok()
            .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
            .expect("the crate's tests run inside a work tree");
        std::fs::write(&unresolvable, format!("{}a scope s1 2026-09-14 {}\n", hdr, head))
            .expect("cannot write the fixture state file");
        assert_eq!(iteration_start(&unresolvable.display().to_string()), head);
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the header run stops at a '## ' heading as
    // well as at the first data line, and the retained blank run does not grow by one per boundary
    #[test]
    fn the_truncate_stops_at_a_section_heading_and_holds_blanks_pending() {
        let rec = "# contract: x\n\n## 2026-01-01 scope — q?\n- finding: body\n";
        assert_eq!(truncate_to_header(rec), "# contract: x\n");
        assert_eq!(truncate_to_header(&truncate_to_header(rec)), "# contract: x\n");
        let two = "# a\n# b\n\ndata\n";
        assert_eq!(truncate_to_header(two), "# a\n# b\n");
        assert!(header_only(&truncate_to_header(rec)));
        assert!(header_only(""));
        assert!(!header_only(rec));
        assert!(!header_only(two));
    }

    #[test]
    fn the_cursor_is_the_last_data_lines_stage_and_absent_shapes_are_empty() {
        assert_eq!(current_stage("# hdr\n---\n\na b c d\ne f g h\n"), "f");
        assert_eq!(current_stage("# hdr\na b c d\n"), "");
        assert_eq!(current_stage(""), "");
        assert_eq!(current_stage("---\nonlyonefield\n"), "");
    }

    // spec: lifecycle-kit/SPEC.md §The stage-machine adapters — the residual [stage:] field is healed away
    // rather than reported, so a consumer upgrades mid-iteration without a red
    #[test]
    fn the_iteration_name_survives_a_residual_stage_field() {
        assert_eq!(header_iter("## Iteration: alpha"), "alpha");
        assert_eq!(header_iter("## Iteration:   alpha  [stage: build]"), "alpha");
        assert_eq!(header_iter("## Iteration: —"), "—");
    }
}
