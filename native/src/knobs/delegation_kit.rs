// spec: delegation-kit/SPEC.md §Layout and configuration — delegation-kit's static knob table and
// validator
use super::{indexed, scalar, Kit, Resolve, Row, Shape, Value, Values};

fn bridged_scalar(resolve: Resolve, name: &str) -> Result<String, String> {
    resolve(name).map(|(v, _)| v.wire())
}

// spec: delegation-kit/SPEC.md §Layout and configuration — beside a set usage file, `${file%/*}`'s
// directory; empty beside the empty usage file, which `usage::paths` fills at the reader
fn cred_file(resolve: Resolve) -> Result<Value, String> {
    let usage = bridged_scalar(resolve, "DELEGATION_KIT_USAGE_FILE")?;
    if usage.is_empty() {
        return Ok(Value::Scalar(String::new()));
    }
    let dir = usage.rsplit_once('/').map_or(usage.as_str(), |(d, _)| d);
    Ok(Value::Scalar(format!("{}/.credentials.json", dir)))
}

fn stop_log(resolve: Resolve) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_WORKFLOW_DIR").map(|d| Value::Scalar(format!("{}/subagent-stop-liveness.log", d)))
}

fn gate_files(resolve: Resolve) -> Result<Value, String> {
    let g = bridged_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    Ok(Value::Indexed(vec![
        format!("{}/check-*.sh", g),
        format!("{}/check-*.gate", g),
        format!("{}/lib/gate.sh", g),
    ]))
}

fn meta_paths(resolve: Resolve) -> Result<Value, String> {
    let g = bridged_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    let w = bridged_scalar(resolve, "GATE_SDK_WORKFLOW_DIR")?;
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
// the numeric shapes, the positive fan width, and the emptiness of the roster and the two path sets
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
    if scalar(v, "DELEGATION_KIT_AGENT_DIR").is_some_and(str::is_empty) {
        errs.push("DELEGATION_KIT_AGENT_DIR is empty".to_string());
    }
    for n in ["DELEGATION_KIT_GATE_FILES", "DELEGATION_KIT_META_PATHS"] {
        if indexed(v, n).is_some_and(|e| e.is_empty()) {
            errs.push(format!("{} is empty", n));
        }
    }
    errs
}
