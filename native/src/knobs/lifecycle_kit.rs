// spec: lifecycle-kit/SPEC.md §Layout and configuration — lifecycle-kit's static knob table and the
// stage machine's validator
use super::{indexed, keyed, scalar, Kit, Resolve, Row, Shape, Value, Values};

fn bridged_scalar(resolve: Resolve, name: &str) -> Result<String, String> {
    match resolve(name)? {
        (Value::Scalar(s), _) => Ok(s),
        (v, _) => Ok(v.wire()),
    }
}

fn queue_file(resolve: Resolve) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_QUEUE_FILE").map(Value::Scalar)
}

fn in_workflow_dir(resolve: Resolve, base: &str) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_WORKFLOW_DIR").map(|d| Value::Scalar(format!("{}/{}", d, base)))
}

fn state_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "WORKFLOW-STATE.txt")
}

fn lesson_evidence_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "lesson-evidence.txt")
}

fn gap_inbox_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "gap-inbox.md")
}

fn survey_record_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "survey-record.md")
}

// spec: lifecycle-kit/SPEC.md §The state machine — the journal pattern defers to the scratch dir's
// own knob rather than restating its literal
fn stage_journal_pattern(resolve: Resolve) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_TMP_DIR").map(|d| Value::Scalar(format!("{}/<stage>-journal.md", d)))
}

fn audit_stage(resolve: Resolve) -> Result<String, String> {
    bridged_scalar(resolve, "LIFECYCLE_KIT_AUDIT_STAGE")
}

fn audit_entry_stage(resolve: Resolve) -> Result<Value, String> {
    let a = audit_stage(resolve)?;
    Ok(Value::Scalar(if a.is_empty() { String::new() } else { "build".to_string() }))
}

fn waiver_token(resolve: Resolve) -> Result<Value, String> {
    let a = audit_stage(resolve)?;
    Ok(Value::Scalar(if a.is_empty() { a } else { format!("{}-waived", a) }))
}

// spec: lifecycle-kit/SPEC.md §The survey record — the queue file alone, the one permanent surface
// this kit owns
fn permanent_surface_globs(resolve: Resolve) -> Result<Value, String> {
    bridged_scalar(resolve, "LIFECYCLE_KIT_QUEUE_FILE").map(|q| Value::Indexed(vec![q]))
}

pub const KIT: Kit = Kit {
    root: "lifecycle-kit",
    rows: &[
        Row::indexed("LIFECYCLE_KIT_STAGES", &["scope", "align", "build", "validate", "close"]),
        Row::keyed(
            "LIFECYCLE_KIT_PREDECESSOR",
            &[("align", "scope"), ("build", "scope"), ("validate", "build"), ("close", "validate")],
        ),
        Row::scalar("LIFECYCLE_KIT_FIRST_STAGE", "scope"),
        Row::scalar("LIFECYCLE_KIT_DRAIN_STAGE", "validate"),
        Row::indexed("LIFECYCLE_KIT_ACTIVE_SECTIONS", &["New Features", "Technical Debt"]),
        Row::scalar("LIFECYCLE_KIT_AUDIT_STAGE", "align"),
        Row::derived("LIFECYCLE_KIT_AUDIT_ENTRY_STAGE", Shape::Scalar, audit_entry_stage, &["LIFECYCLE_KIT_AUDIT_STAGE"]),
        Row::derived("LIFECYCLE_KIT_WAIVER_TOKEN", Shape::Scalar, waiver_token, &["LIFECYCLE_KIT_AUDIT_STAGE"]),
        Row::scalar("LIFECYCLE_KIT_AMENDMENT_GLOB", "SPEC-*.md"),
        Row::scalar("LIFECYCLE_KIT_ROSTER_BASENAME", "SPEC.md"),
        Row::indexed("LIFECYCLE_KIT_CONTRACT_TOKENS", &["SPEC.md", "proto/"]),
        Row::scalar("LIFECYCLE_KIT_SKILLS_DIR", ".claude/commands"),
        Row::scalar("LIFECYCLE_KIT_SESSION_BOUNDARY", "stage"),
        Row::scalar("LIFECYCLE_KIT_AGENT_FILE", "CLAUDE.md"),
        Row::scalar("LIFECYCLE_KIT_SHIM_NGRAM", "9"),
        Row::indexed("LIFECYCLE_KIT_SHIM_DEDUP_CORPUS", &[]),
        Row::derived("LIFECYCLE_KIT_QUEUE_FILE", Shape::Scalar, queue_file, &["GATE_SDK_QUEUE_FILE"]),
        Row::derived("LIFECYCLE_KIT_STATE_FILE", Shape::Scalar, state_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived(
            "LIFECYCLE_KIT_LESSON_EVIDENCE_FILE",
            Shape::Scalar,
            lesson_evidence_file,
            &["GATE_SDK_WORKFLOW_DIR"],
        ),
        Row::derived("LIFECYCLE_KIT_GAP_INBOX_FILE", Shape::Scalar, gap_inbox_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived(
            "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
            Shape::Scalar,
            survey_record_file,
            &["GATE_SDK_WORKFLOW_DIR"],
        ),
        Row::scalar("LIFECYCLE_KIT_RECURRENCE_THRESHOLD", "2"),
        Row::derived(
            "LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN",
            Shape::Scalar,
            stage_journal_pattern,
            &["GATE_SDK_TMP_DIR"],
        ),
        Row::scalar("LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE", "0"),
        Row::scalar("LIFECYCLE_KIT_LEAD_JOURNAL_FILE", "lead-journal.md"),
        Row::indexed("LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS", &["*/SPEC.md"]),
        Row::scalar("LIFECYCLE_KIT_RULING_RECORD", ""),
        Row::indexed("LIFECYCLE_KIT_RULING_CITERS", &[]),
        Row::scalar("LIFECYCLE_KIT_RULING_ORACLE_TIMEOUT", "10"),
        Row::derived(
            "LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS",
            Shape::Indexed,
            permanent_surface_globs,
            &["LIFECYCLE_KIT_QUEUE_FILE"],
        ),
        Row::indexed("LIFECYCLE_KIT_BOUNDARY_TRUNCATE", &[]),
        Row::indexed("LIFECYCLE_KIT_BOUNDARY_REQUIRE", &[]),
        Row::indexed("LIFECYCLE_KIT_BOUNDARY_PRESERVE", &[]),
        Row::scalar("LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK", "1"),
        Row::scalar("LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE", ""),
        Row::indexed("LIFECYCLE_KIT_ENTRY_PREFLIGHT", &[]),
        Row::scalar("LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE", ""),
    ],
    validate: Some(("stage-machine config", validate)),
    open_family: false,
    retired: &[],
};

fn positive(v: &str) -> bool {
    v.bytes().next().is_some_and(|b| (b'1'..=b'9').contains(&b)) && v.bytes().all(|b| b.is_ascii_digit())
}

// spec: lifecycle-kit/SPEC.md §The stage-machine adapters — a compiled lock-reason pattern is read off
// bash's own `[[ =~ ]]` status, the engine the consumer's pattern is matched with: 2 is a pattern it
// could not compile, 0 and 1 both mean it compiled
fn ere_compiles(re: &str) -> bool {
    match crate::proc::run("bash", &["-c", "[[ \"\" =~ $1 ]]", "bash", re]) {
        Ok(c) => c.code().is_some_and(|rc| rc <= 1),
        Err(_) => false,
    }
}

// spec: lifecycle-kit/SPEC.md §The stage-machine adapters — an escaped parenthesis is literal, so a
// capture group is an unescaped `(`
fn declares_group(re: &str) -> bool {
    let mut chars = re.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                chars.next();
            }
            '(' => return true,
            _ => {}
        }
    }
    false
}

// spec: lifecycle-kit/SPEC.md §Layout and configuration — a broken machine gates nothing: the stage
// relations, the switches, the journal placeholder and the lock-reason pattern
fn validate(v: &Values) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    let stages: &[String] = indexed(v, "LIFECYCLE_KIT_STAGES").unwrap_or(&[]);
    let known = |s: &str| stages.iter().any(|x| x == s);
    if stages.is_empty() {
        errs.push("LIFECYCLE_KIT_STAGES is empty".to_string());
    }
    for n in [
        "LIFECYCLE_KIT_SKILLS_DIR",
        "LIFECYCLE_KIT_LESSON_EVIDENCE_FILE",
        "LIFECYCLE_KIT_GAP_INBOX_FILE",
        "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
        "LIFECYCLE_KIT_LEAD_JOURNAL_FILE",
    ] {
        if scalar(v, n).is_some_and(str::is_empty) {
            errs.push(format!("{} is empty", n));
        }
    }
    for n in ["LIFECYCLE_KIT_SHIM_NGRAM", "LIFECYCLE_KIT_RECURRENCE_THRESHOLD"] {
        if let Some(s) = scalar(v, n) {
            if !positive(s) {
                errs.push(format!("{} '{}' is not a positive integer", n, s));
            }
        }
    }
    if let Some(b) = scalar(v, "LIFECYCLE_KIT_SESSION_BOUNDARY") {
        if b != "stage" && b != "iteration" {
            errs.push(format!("LIFECYCLE_KIT_SESSION_BOUNDARY '{}' is neither 'stage' nor 'iteration'", b));
        }
    }
    if let Some(f) = scalar(v, "LIFECYCLE_KIT_FIRST_STAGE") {
        if !known(f) {
            errs.push(format!("LIFECYCLE_KIT_FIRST_STAGE '{}' is not in LIFECYCLE_KIT_STAGES", f));
        }
    }
    let pred: &[(String, String)] = keyed(v, "LIFECYCLE_KIT_PREDECESSOR").unwrap_or(&[]);
    for (k, p) in pred {
        if !known(k) {
            errs.push(format!("LIFECYCLE_KIT_PREDECESSOR key '{}' is not in LIFECYCLE_KIT_STAGES", k));
        }
        if !known(p) {
            errs.push(format!("LIFECYCLE_KIT_PREDECESSOR[{}]='{}' is not in LIFECYCLE_KIT_STAGES", k, p));
        }
    }
    if let Some(d) = scalar(v, "LIFECYCLE_KIT_DRAIN_STAGE").filter(|d| !d.is_empty()) {
        if !known(d) {
            errs.push(format!("LIFECYCLE_KIT_DRAIN_STAGE '{}' is not in LIFECYCLE_KIT_STAGES", d));
        }
        // spec: lifecycle-kit/SPEC.md §check-stage-entry — a terminal drain stage is fail-closed
        // config: a [drain-exempt:] tag with no reachable successor backstop is a permanent exemption
        if !pred.iter().any(|(_, p)| p == d) {
            errs.push(format!(
                "LIFECYCLE_KIT_DRAIN_STAGE '{}' is terminal (no LIFECYCLE_KIT_PREDECESSOR entry names it) — the drain-exempt backstop would never run",
                d
            ));
        }
    }
    for n in ["LIFECYCLE_KIT_AUDIT_STAGE", "LIFECYCLE_KIT_AUDIT_ENTRY_STAGE"] {
        if let Some(s) = scalar(v, n).filter(|s| !s.is_empty()) {
            if !known(s) {
                errs.push(format!("{} '{}' is not in LIFECYCLE_KIT_STAGES", n, s));
            }
        }
    }
    if let Some(w) = scalar(v, "LIFECYCLE_KIT_WAIVER_TOKEN").filter(|w| !w.is_empty()) {
        if known(w) {
            errs.push(format!("LIFECYCLE_KIT_WAIVER_TOKEN '{}' collides with a stage name", w));
        }
    }
    for pf in indexed(v, "LIFECYCLE_KIT_ENTRY_PREFLIGHT").unwrap_or(&[]) {
        match pf.split_once('=') {
            None => errs.push(format!("LIFECYCLE_KIT_ENTRY_PREFLIGHT entry '{}' lacks the '<stage>=<command>' shape", pf)),
            Some((s, _)) if !known(s) => errs.push(format!(
                "LIFECYCLE_KIT_ENTRY_PREFLIGHT stage key '{}' is not in LIFECYCLE_KIT_STAGES",
                s
            )),
            Some(_) => {}
        }
    }
    for n in ["LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK", "LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE"] {
        if let Some(s) = scalar(v, n) {
            if s != "0" && s != "1" {
                errs.push(format!("{} must be 0|1 (got '{}')", n, s));
            }
        }
    }
    // spec: lifecycle-kit/SPEC.md §The stage-machine adapters — a pattern with no placeholder names one
    // file for every stage, so the entry assertion would read the wrong session's journal and pass
    if let Some(p) = scalar(v, "LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN") {
        if !p.contains("<stage>") {
            errs.push(format!("LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN '{}' carries no '<stage>' placeholder", p));
        }
    }
    if let Some(re) = scalar(v, "LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE").filter(|r| !r.is_empty()) {
        if !ere_compiles(re) {
            errs.push(format!("LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE '{}' is not a valid POSIX ERE", re));
        } else if !declares_group(re) {
            errs.push(format!(
                "LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE '{}' declares no capture group — the group is the holder's pid",
                re
            ));
        }
    }
    errs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lock_pattern_is_compiled_by_bash_and_needs_an_unescaped_group() {
        assert!(ere_compiles("^held by pid ([0-9]+)$"));
        assert!(!ere_compiles("(["));
        assert!(declares_group("a ([0-9]+)"));
        assert!(!declares_group("a \\([0-9]+\\)"));
    }
}
