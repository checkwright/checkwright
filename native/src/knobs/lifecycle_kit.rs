// spec: lifecycle-kit/SPEC.md §Layout and configuration — lifecycle-kit's static knob table and the
// stage machine's validator
use super::{indexed, keyed, scalar, Kit, Resolve, Row, Shape, Value, Values};

fn input_scalar(resolve: Resolve, name: &str) -> Result<String, String> {
    match resolve(name)? {
        (Value::Scalar(s), _) => Ok(s),
        (v, _) => Ok(v.wire()),
    }
}

fn queue_file(resolve: Resolve) -> Result<Value, String> {
    input_scalar(resolve, "GATE_SDK_QUEUE_FILE").map(Value::Scalar)
}

fn in_workflow_dir(resolve: Resolve, base: &str) -> Result<Value, String> {
    input_scalar(resolve, "GATE_SDK_WORKFLOW_DIR").map(|d| Value::Scalar(format!("{}/{}", d, base)))
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

fn consult_inbox_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "consult-items.md")
}

fn survey_record_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "survey-record.md")
}

fn disposed_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "disposed-findings.txt")
}

// spec: lifecycle-kit/SPEC.md §The state machine — the journal pattern defers to the scratch dir's
// own knob rather than restating its literal
fn stage_journal_pattern(resolve: Resolve) -> Result<Value, String> {
    input_scalar(resolve, "GATE_SDK_TMP_DIR").map(|d| Value::Scalar(format!("{}/<stage>-journal.md", d)))
}

fn audit_stage(resolve: Resolve) -> Result<String, String> {
    input_scalar(resolve, "LIFECYCLE_KIT_AUDIT_STAGE")
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
    input_scalar(resolve, "LIFECYCLE_KIT_QUEUE_FILE").map(|q| Value::Indexed(vec![q]))
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
        Row::scalar("LIFECYCLE_KIT_MIRROR_ROOT", ""),
        Row::scalar("LIFECYCLE_KIT_SKILLS_DIR", ".claude/commands"),
        Row::scalar("LIFECYCLE_KIT_SESSION_BOUNDARY", "stage"),
        Row::scalar("LIFECYCLE_KIT_AGENT_FILE", "CLAUDE.md"),
        Row::scalar("LIFECYCLE_KIT_STAGE_CONTRACT_FRAME", "lifecycle-kit/templates/frames/stage-contract.md"),
        Row::indexed("LIFECYCLE_KIT_STAGE_EXECUTOR", &[]),
        Row::scalar("LIFECYCLE_KIT_CRITIQUE_ADAPTER", ""),
        Row::scalar("LIFECYCLE_KIT_CONSULT_CLASS", "judgment"),
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
            "LIFECYCLE_KIT_CONSULT_INBOX_FILE",
            Shape::Scalar,
            consult_inbox_file,
            &["GATE_SDK_WORKFLOW_DIR"],
        ),
        Row::derived(
            "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
            Shape::Scalar,
            survey_record_file,
            &["GATE_SDK_WORKFLOW_DIR"],
        ),
        Row::derived("LIFECYCLE_KIT_DISPOSED_FILE", Shape::Scalar, disposed_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::scalar("LIFECYCLE_KIT_RECURRENCE_THRESHOLD", "2"),
        Row::derived(
            "LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN",
            Shape::Scalar,
            stage_journal_pattern,
            &["GATE_SDK_TMP_DIR"],
        ),
        Row::scalar("LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE", "0"),
        Row::scalar("LIFECYCLE_KIT_LEAD_JOURNAL_FILE", "lead-journal.md"),
        Row::scalar("LIFECYCLE_KIT_DISPATCH_MARKER_FILE", "stage-dispatch.txt"),
        Row::indexed("LIFECYCLE_KIT_STAGE_SESSION_TYPES", &[]),
        Row::indexed("LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS", &["*/SPEC.md"]),
        Row::scalar("LIFECYCLE_KIT_RULING_RECORD", ""),
        Row::indexed("LIFECYCLE_KIT_RULING_CITERS", &[]),
        Row::scalar("LIFECYCLE_KIT_RULING_ORACLE_TIMEOUT", "10"),
        Row::scalar("LIFECYCLE_KIT_AUDIT_ROSTER_FILE", ""),
        Row::scalar("LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP", "1500"),
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
    families: &[],
    retired: &[],
    // spec: gate-sdk/SPEC.md §The knob file — LIFECYCLE_KIT_SESSIONS_DIR resolves from the process
    // environment only (`--emit session-id`); a knob file must never set a session identity, and
    // `smoke/` is withheld from the payload, so this is its one definition site.
    env_only: &["LIFECYCLE_KIT_SESSIONS_DIR"],
};

fn positive(v: &str) -> bool {
    v.bytes().next().is_some_and(|b| (b'1'..=b'9').contains(&b)) && v.bytes().all(|b| b.is_ascii_digit())
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
        "LIFECYCLE_KIT_CONSULT_INBOX_FILE",
        "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
        "LIFECYCLE_KIT_LEAD_JOURNAL_FILE",
        "LIFECYCLE_KIT_DISPATCH_MARKER_FILE",
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
    if let Some(s) = scalar(v, "LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP").filter(|s| *s != "off" && !positive(s)) {
        errs.push(format!("LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP '{}' is neither a positive integer nor off", s));
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
    // spec: lifecycle-kit/SPEC.md §Layout and configuration — each executor element is
    // `<stage>=<adapter>`, a configured stage bound once to a name in the shape delegation-kit's
    // validator holds an adapter name to; the adapter table itself is that kit's to read
    let mut bound: Vec<&str> = Vec::new();
    for e in indexed(v, "LIFECYCLE_KIT_STAGE_EXECUTOR").unwrap_or(&[]) {
        let Some((s, a)) = e.split_once('=') else {
            errs.push(format!("LIFECYCLE_KIT_STAGE_EXECUTOR element '{}' lacks the '<stage>=<adapter>' shape", e));
            continue;
        };
        if !known(s) {
            errs.push(format!("LIFECYCLE_KIT_STAGE_EXECUTOR stage key '{}' is not in LIFECYCLE_KIT_STAGES", s));
        }
        if bound.contains(&s) {
            errs.push(format!("LIFECYCLE_KIT_STAGE_EXECUTOR binds stage '{}' twice", s));
        }
        bound.push(s);
        if a.is_empty() {
            errs.push(format!("LIFECYCLE_KIT_STAGE_EXECUTOR element '{}' has an empty adapter", e));
        } else if !super::delegation_kit::adapter_name(a) {
            errs.push(format!("LIFECYCLE_KIT_STAGE_EXECUTOR adapter '{}' is outside [a-z0-9-]", a));
        }
    }
    if let Some(a) = scalar(v, "LIFECYCLE_KIT_CRITIQUE_ADAPTER").filter(|a| !a.is_empty()) {
        if !super::delegation_kit::adapter_name(a) {
            errs.push(format!("LIFECYCLE_KIT_CRITIQUE_ADAPTER '{}' is outside [a-z0-9-]", a));
        }
    }
    // spec: lifecycle-kit/SPEC.md §Layout and configuration — the consultation's class is one name in
    // the shape delegation-kit holds a class to; that kit's roster is its own to read
    if let Some(c) = scalar(v, "LIFECYCLE_KIT_CONSULT_CLASS") {
        if c.is_empty() {
            errs.push("LIFECYCLE_KIT_CONSULT_CLASS is empty".to_string());
        } else if !crate::tier::name(c) {
            errs.push(format!("LIFECYCLE_KIT_CONSULT_CLASS '{}' is outside [a-z0-9-]", c));
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
    // spec: lifecycle-kit/SPEC.md §The stage-machine adapters — the lock-reason pattern is judged by
    // the engine that will match it, and refused for any shape outside its one-group capture
    if let Some(re) = scalar(v, "LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE").filter(|r| !r.is_empty()) {
        if let Err(e) = crate::ere::EreCapture::compile(re) {
            errs.push(format!(
                "LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE '{}' is refused — the group is the holder's pid: {}",
                re, e
            ));
        }
    }
    errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ere::EreCapture;
    use crate::knobs::Origin;

    // spec: lifecycle-kit/SPEC.md §Layout and configuration — one firing row per executor-binding
    // refusal, every finding of a table reported together, beside a table that passes
    #[test]
    fn every_executor_binding_refusal_fires_on_its_own_shape() {
        let run = |elems: &[&str]| {
            let list = |xs: &[&str]| (Value::Indexed(xs.iter().map(|s| s.to_string()).collect()), Origin::Tracked);
            let mut v: Values = Values::new();
            v.insert("LIFECYCLE_KIT_STAGES", list(&["scope", "align", "build"]));
            v.insert("LIFECYCLE_KIT_STAGE_EXECUTOR", list(elems));
            validate(&v)
        };
        let cases: &[(&str, &[&str])] = &[
            ("lacks the '<stage>=<adapter>' shape", &["align"]),
            ("stage key 'polish' is not in LIFECYCLE_KIT_STAGES", &["polish=demo"]),
            ("binds stage 'align' twice", &["align=demo", "align=other"]),
            ("element 'align=' has an empty adapter", &["align="]),
            ("adapter 'Demo_1' is outside [a-z0-9-]", &["align=Demo_1"]),
        ];
        for (want, elems) in cases {
            let errs = run(elems);
            assert!(errs.iter().any(|e| e.contains(want)), "{:?} did not refuse with '{}': {:?}", elems, want, errs);
        }
        let all = run(&["align", "polish=demo", "scope="]);
        assert_eq!(all.iter().filter(|e| e.contains("LIFECYCLE_KIT_STAGE_EXECUTOR")).count(), 3, "{:?}", all);
        let clean = run(&["align=demo-1", "scope=other"]);
        assert!(!clean.iter().any(|e| e.contains("LIFECYCLE_KIT_STAGE_EXECUTOR")), "{:?}", clean);
        assert!(!run(&[]).iter().any(|e| e.contains("LIFECYCLE_KIT_STAGE_EXECUTOR")));
    }

    // spec: lifecycle-kit/SPEC.md §Layout and configuration — the critique adapter is one name in
    // the adapter-name shape, and empty is off
    #[test]
    fn a_critique_adapter_outside_the_name_shape_is_refused() {
        let run = |name: &str| {
            let mut v: Values = Values::new();
            v.insert("LIFECYCLE_KIT_CRITIQUE_ADAPTER", (Value::Scalar(name.to_string()), Origin::Tracked));
            validate(&v).into_iter().filter(|e| e.contains("LIFECYCLE_KIT_CRITIQUE_ADAPTER")).collect::<Vec<_>>()
        };
        for bad in ["Demo_1", "demo expert", "demo=x"] {
            assert_eq!(run(bad), vec![format!("LIFECYCLE_KIT_CRITIQUE_ADAPTER '{}' is outside [a-z0-9-]", bad)]);
        }
        assert!(run("demo-1").is_empty() && run("").is_empty());
    }

    // spec: lifecycle-kit/SPEC.md §Layout and configuration — the consultation's class is one name in
    // the class-name shape, never empty, and a class no roster holds is not this validator's to refuse
    #[test]
    fn a_consult_class_outside_the_name_shape_is_refused() {
        let run = |name: &str| {
            let mut v: Values = Values::new();
            v.insert("LIFECYCLE_KIT_CONSULT_CLASS", (Value::Scalar(name.to_string()), Origin::Tracked));
            validate(&v).into_iter().filter(|e| e.contains("LIFECYCLE_KIT_CONSULT_CLASS")).collect::<Vec<_>>()
        };
        assert_eq!(run(""), vec!["LIFECYCLE_KIT_CONSULT_CLASS is empty".to_string()]);
        for bad in ["Expert", "two words", "a=b"] {
            assert_eq!(run(bad), vec![format!("LIFECYCLE_KIT_CONSULT_CLASS '{}' is outside [a-z0-9-]", bad)]);
        }
        assert!(run("judgment").is_empty() && run("a-class-no-roster-holds").is_empty());
    }

    #[test]
    fn a_lock_pattern_needs_one_group_the_engine_compiles() {
        let no_group = EreCapture::compile("a \\([0-9]+\\)").err().expect("an escaped group compiled");
        assert!(no_group.to_string().contains("no capture group"), "{}", no_group);
        assert!(EreCapture::compile("([").is_err());
    }

    // spec: lifecycle-kit/SPEC.md §Layout and configuration — every pattern this tree ships must stay
    // admitted and keep capturing its pid, so an engine change that would refuse one reds here
    #[test]
    fn every_shipped_lock_pattern_is_admitted_and_captures_its_pid() {
        let knobs_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../scripts/lifecycle-config.knobs");
        let knobs = std::fs::read_to_string(knobs_file)
            .expect("cannot read this repo's lifecycle knob file");
        let repo = knobs
            .lines()
            .find_map(|l| l.strip_prefix("LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE = "))
            .expect("the repo knob file sets no lock pattern");
        for (re, reason) in [
            (repo, "claude agent a1b2 (pid 4321 start 99)"),
            ("^held by pid ([0-9]+)$", "held by pid 4321"),
            ("^testharness \\(pid ([0-9]+)\\)$", "testharness (pid 4321)"),
        ] {
            let cap = EreCapture::compile(re).unwrap_or_else(|e| panic!("{:?} refused: {}", re, e));
            assert_eq!(cap.capture(reason).map(|(i, j)| &reason[i..j]), Some("4321"), "{:?}", re);
        }
    }
}
