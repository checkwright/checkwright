// spec: delegation-kit/SPEC.md §Layout and configuration — delegation-kit's static knob table and
// validator
use super::{indexed, scalar, Kit, Resolve, Row, Shape, Value, Values};

fn input_scalar(resolve: Resolve, name: &str) -> Result<String, String> {
    resolve(name).map(|(v, _)| v.wire())
}

// spec: delegation-kit/SPEC.md §Layout and configuration — beside a set usage file, `${file%/*}`'s
// directory; empty beside the empty usage file, which `usage::paths` fills at the reader
fn cred_file(resolve: Resolve) -> Result<Value, String> {
    let usage = input_scalar(resolve, "DELEGATION_KIT_USAGE_FILE")?;
    if usage.is_empty() {
        return Ok(Value::Scalar(String::new()));
    }
    let dir = usage.rsplit_once('/').map_or(usage.as_str(), |(d, _)| d);
    Ok(Value::Scalar(format!("{}/.credentials.json", dir)))
}

fn stop_log(resolve: Resolve) -> Result<Value, String> {
    input_scalar(resolve, "GATE_SDK_WORKFLOW_DIR").map(|d| Value::Scalar(format!("{}/subagent-stop-liveness.log", d)))
}

fn gate_files(resolve: Resolve) -> Result<Value, String> {
    let g = input_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    Ok(Value::Indexed(vec![
        format!("{}/check-*.sh", g),
        format!("{}/check-*.gate", g),
    ]))
}

fn meta_paths(resolve: Resolve) -> Result<Value, String> {
    let g = input_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    let w = input_scalar(resolve, "GATE_SDK_WORKFLOW_DIR")?;
    // path-dialect-exempt: this knob's declared value shape — a roster of directory prefixes
    // spelled with a trailing separator, beside the `.claude/` literal that fixes the convention;
    // its reader (`check-gate-tamper`) trims and compares by component
    Ok(Value::Indexed(vec![format!("{}/", g), format!("{}/", w), ".claude/".to_string()]))
}

pub const KIT: Kit = Kit {
    root: "delegation-kit",
    rows: &[
        Row::scalar("DELEGATION_KIT_USAGE_FILE", ""),
        Row::derived("DELEGATION_KIT_CRED_FILE", Shape::Scalar, cred_file, &["DELEGATION_KIT_USAGE_FILE"]),
        Row::scalar("DELEGATION_KIT_PAUSE_PCT", "80"),
        Row::scalar("DELEGATION_KIT_PAUSE_PCT_7D", "95"),
        Row::scalar("DELEGATION_KIT_STALE_AGE", "600"),
        Row::scalar("DELEGATION_KIT_LOGIN_WINDOW", "600"),
        Row::indexed("DELEGATION_KIT_REFRESH_CMD", &[]),
        Row::scalar("DELEGATION_KIT_REFRESH_MIN_AGE", "60"),
        Row::scalar("DELEGATION_KIT_USAGE_HISTORY", ""),
        Row::scalar("DELEGATION_KIT_FAN_WIDTH", "2"),
        Row::scalar("DELEGATION_KIT_AGENT_DIR", ".claude/agents"),
        Row::scalar("DELEGATION_KIT_ACCOUNT_CONFIG", ""),
        Row::scalar("DELEGATION_KIT_USAGE_ENDPOINT", "https://api.anthropic.com/api/oauth/usage"),
        Row::derived("DELEGATION_KIT_STOP_LOG", Shape::Scalar, stop_log, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::indexed("DELEGATION_KIT_LIVENESS_CMD", &[]),
        Row::indexed("DELEGATION_KIT_READONLY_TYPES", &[]),
        Row::indexed("DELEGATION_KIT_MUTATING_TYPES", &[]),
        Row::scalar("DELEGATION_KIT_REQUIRE_TIER", "off"),
        Row::indexed("DELEGATION_KIT_TIER_MODEL", &[]),
        Row::scalar("DELEGATION_KIT_SESSIONS_DIR", ""),
        Row::indexed("DELEGATION_KIT_STATUSLINE_INBOXES", &[]),
        Row::derived("DELEGATION_KIT_GATE_FILES", Shape::Indexed, gate_files, &["GATE_SDK_GATES_DIR"]),
        Row::derived(
            "DELEGATION_KIT_META_PATHS",
            Shape::Indexed,
            meta_paths,
            &["GATE_SDK_GATES_DIR", "GATE_SDK_WORKFLOW_DIR"],
        ),
    ],
    validate: Some(("delegation config", validate)),
    open_family: false,
    families: &[],
    retired: &[],
    env_only: &[],
};

fn digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

// spec: delegation-kit/SPEC.md §Layout and configuration — `^[0-9]+(\.[0-9]+)?$`
fn numeric(s: &str) -> bool {
    match s.split_once('.') {
        Some((i, f)) => digits(i) && digits(f),
        None => digits(s),
    }
}

// spec: delegation-kit/SPEC.md §Layout and configuration — a broken delegation config runs no tool:
// the numeric shapes, the positive fan width, D5's on|off switch, the tier binding, and the
// emptiness of the agent dir and the two path sets
fn validate(v: &Values) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    for n in ["DELEGATION_KIT_PAUSE_PCT", "DELEGATION_KIT_PAUSE_PCT_7D"] {
        if let Some(s) = scalar(v, n).filter(|s| !numeric(s)) {
            errs.push(format!("{} must be numeric (got '{}')", n, s));
        }
    }
    for n in ["DELEGATION_KIT_STALE_AGE", "DELEGATION_KIT_LOGIN_WINDOW", "DELEGATION_KIT_REFRESH_MIN_AGE"] {
        if let Some(s) = scalar(v, n).filter(|s| !digits(s)) {
            errs.push(format!("{} must be a non-negative integer (got '{}')", n, s));
        }
    }
    if let Some(s) = scalar(v, "DELEGATION_KIT_FAN_WIDTH").filter(|s| !digits(s) || s.bytes().all(|b| b == b'0')) {
        errs.push(format!("DELEGATION_KIT_FAN_WIDTH must be a positive integer (got '{}')", s));
    }
    if let Some(s) = scalar(v, "DELEGATION_KIT_REQUIRE_TIER").filter(|s| !matches!(*s, "on" | "off")) {
        errs.push(format!("DELEGATION_KIT_REQUIRE_TIER must be on|off (got '{}')", s));
    }
    if scalar(v, "DELEGATION_KIT_AGENT_DIR").is_some_and(str::is_empty) {
        errs.push("DELEGATION_KIT_AGENT_DIR is empty".to_string());
    }
    // spec: delegation-kit/SPEC.md §Layout and configuration — each inbox counter is `<label>=<path>`,
    // neither half empty
    for e in indexed(v, "DELEGATION_KIT_STATUSLINE_INBOXES").unwrap_or(&[]) {
        match e.split_once('=') {
            Some((l, p)) if !l.is_empty() && !p.is_empty() => {}
            _ => errs.push(format!(
                "DELEGATION_KIT_STATUSLINE_INBOXES element '{}' is not '<label>=<path>' with both halves non-empty",
                e
            )),
        }
    }
    // spec: delegation-kit/SPEC.md §The tier binding — the refusals are the matcher module's, so
    // the validator and every reader share one reading of an element
    errs.extend(crate::tier::refusals(indexed(v, "DELEGATION_KIT_TIER_MODEL").unwrap_or(&[])));
    for n in ["DELEGATION_KIT_GATE_FILES", "DELEGATION_KIT_META_PATHS"] {
        if indexed(v, n).is_some_and(|e| e.is_empty()) {
            errs.push(format!("{} is empty", n));
        }
    }
    errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knobs::Origin;

    fn binding(elems: &[&str]) -> Vec<String> {
        let mut v: Values = Values::new();
        v.insert(
            "DELEGATION_KIT_TIER_MODEL",
            (Value::Indexed(elems.iter().map(|s| s.to_string()).collect()), Origin::Tracked),
        );
        validate(&v)
    }

    // spec: delegation-kit/SPEC.md §Layout and configuration — one firing row per tier-binding refusal
    #[test]
    fn every_tier_binding_refusal_fires_on_its_own_shape() {
        let cases: &[(&str, &[&str])] = &[
            ("is not '<class>=<model>'", &["judgment"]),
            ("not one of judgment, routing, mechanical", &["premium=big"]),
            ("binds class 'judgment' twice", &["judgment=big", "judgment=bigger"]),
            ("has an empty value", &["judgment="]),
            ("carries whitespace", &["judgment=big model"]),
            ("binds 'inherit'", &["judgment=inherit"]),
            ("is all digits", &["mechanical=5"]),
            ("is the leading token of 'vendor-big-5'", &["judgment=vendor-big-5", "mechanical=vendor"]),
        ];
        for (want, elems) in cases {
            let errs = binding(elems);
            assert!(errs.iter().any(|e| e.contains(want)), "{:?} did not refuse with '{}': {:?}", elems, want, errs);
        }
    }

    // spec: delegation-kit/SPEC.md §Layout and configuration — a binding refusing on several findings
    // reports every one; three aliases, one alias bound to two classes, and an alias beside an exact
    // id all pass
    #[test]
    fn a_well_formed_binding_passes_and_a_broken_one_reports_every_finding() {
        assert!(binding(&["judgment=big", "routing=big", "mechanical=small"]).is_empty());
        assert!(binding(&["judgment=vendor-big-5-5", "mechanical=small"]).is_empty());
        assert!(binding(&[]).is_empty());
        let errs = binding(&["judgment", "premium=x", "mechanical=inherit"]);
        assert_eq!(errs.len(), 3, "{:?}", errs);
    }
}
