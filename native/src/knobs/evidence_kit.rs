// spec: evidence-kit/SPEC.md §Layout and configuration — evidence-kit's static knob table, its two
// declared families and its validator
use super::{indexed, is_suffix, scalar, Family, Kit, Origin, Resolve, Row, Shape, Value, Values};

fn bridged_scalar(resolve: Resolve, name: &str) -> Result<(String, Origin), String> {
    resolve(name).map(|(v, o)| (v.wire(), o))
}

fn under(resolve: Resolve, input: &str, base: &str) -> Result<Value, String> {
    bridged_scalar(resolve, input).map(|(d, _)| Value::Scalar(format!("{}/{}", d, base)))
}

fn baseline_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "validate-baseline.txt")
}

fn manifest_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "validate-evidence.txt")
}

fn skip_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "validate-skips.txt")
}

fn state_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "WORKFLOW-STATE.txt")
}

fn queue_file(resolve: Resolve) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_QUEUE_FILE").map(|(q, _)| Value::Scalar(q))
}

fn tmp_dir(resolve: Resolve) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_TMP_DIR").map(|(t, _)| Value::Scalar(t))
}

fn lock_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "EVIDENCE_KIT_TMP_DIR", "run-validate.lock")
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the fixture suites over the kit roots and the gates dir; a
// placeholder input names no directory to probe, so the roster renders none
fn suites(resolve: Resolve) -> Result<Vec<(String, String, String)>, String> {
    let (roots, origin) = bridged_scalar(resolve, "GATE_KIT_ROOTS_REL")?;
    let (gates, _) = bridged_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    if origin == Origin::Placeholder {
        return Ok(Vec::new());
    }
    let roots: Vec<String> = roots.split('\t').filter(|r| !r.is_empty()).map(str::to_string).collect();
    Ok(crate::registry::fixture_suites_in(&roots, &gates))
}

fn fixture_suites(resolve: Resolve) -> Result<Value, String> {
    Ok(Value::Indexed(suites(resolve)?.into_iter().map(|(s, _, _)| s).collect()))
}

// spec: evidence-kit/SPEC.md §Layout and configuration — one run member per fixture suite, the
// fixture runner over its tests dir and, where one exists, its checks dir
fn run_members(resolve: Resolve) -> Result<Vec<(String, String)>, String> {
    let (root, _) = bridged_scalar(resolve, "GATE_SDK_ROOT_HERE")?;
    let root = root.trim_end_matches('/').to_string();
    Ok(suites(resolve)?
        .into_iter()
        .map(|(s, tests, checks)| {
            let mut cmd = format!("bash {}/bin/run-gates.sh --run-gate-tests {}", root, tests);
            if !checks.is_empty() {
                cmd.push(' ');
                cmd.push_str(&checks);
            }
            (s, cmd)
        })
        .collect())
}

pub const KIT: Kit = Kit {
    root: "evidence-kit",
    rows: &[
        Row::indexed("EVIDENCE_KIT_SUITES", &[]).with_referents(&["EVIDENCE_KIT_FIXTURE_SUITES"]),
        Row::scalar("EVIDENCE_KIT_PARSER", "exit-code"),
        Row::derived("EVIDENCE_KIT_BASELINE_FILE", Shape::Scalar, baseline_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("EVIDENCE_KIT_MANIFEST_FILE", Shape::Scalar, manifest_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("EVIDENCE_KIT_SKIP_FILE", Shape::Scalar, skip_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("EVIDENCE_KIT_QUEUE_FILE", Shape::Scalar, queue_file, &["GATE_SDK_QUEUE_FILE"]),
        Row::derived("EVIDENCE_KIT_STATE_FILE", Shape::Scalar, state_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("EVIDENCE_KIT_TMP_DIR", Shape::Scalar, tmp_dir, &["GATE_SDK_TMP_DIR"]),
        Row::derived("EVIDENCE_KIT_LOCK_FILE", Shape::Scalar, lock_file, &["EVIDENCE_KIT_TMP_DIR"]),
        Row::scalar("EVIDENCE_KIT_RUN_ID", ""),
        Row::scalar("EVIDENCE_KIT_PRE_HOOK", ""),
        Row::scalar("EVIDENCE_KIT_RUNNER_DOC", "README.md"),
        Row::keyed("EVIDENCE_KIT_SCENARIO_GLOBS", &[]),
        Row::indexed("EVIDENCE_KIT_PERMANENT_SLUGS", &[]),
        Row::derived(
            "EVIDENCE_KIT_FIXTURE_SUITES",
            Shape::Indexed,
            fixture_suites,
            &["GATE_KIT_ROOTS_REL", "GATE_SDK_GATES_DIR"],
        ),
    ],
    validate: Some(("evidence config", validate)),
    open_family: false,
    families: &[
        Family {
            prefix: "EVIDENCE_KIT_RUN_",
            derive: Some(run_members),
            inputs: &["GATE_KIT_ROOTS_REL", "GATE_SDK_GATES_DIR", "GATE_SDK_ROOT_HERE"],
        },
        Family { prefix: "EVIDENCE_KIT_PARSER_", derive: None, inputs: &[] },
    ],
    retired: &[],
};

// spec: evidence-kit/SPEC.md §Layout and configuration — the four paths and the parser non-empty,
// and every suite a valid family suffix
fn validate(v: &Values) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for name in [
        "EVIDENCE_KIT_PARSER",
        "EVIDENCE_KIT_BASELINE_FILE",
        "EVIDENCE_KIT_MANIFEST_FILE",
        "EVIDENCE_KIT_QUEUE_FILE",
    ] {
        if scalar(v, name).is_some_and(str::is_empty) {
            out.push(format!("{} is empty", name));
        }
    }
    for s in indexed(v, "EVIDENCE_KIT_SUITES").unwrap_or(&[]) {
        if !is_suffix(s) {
            out.push(format!("suite name '{}' is not a valid EVIDENCE_KIT_RUN_<suite> suffix", s));
        }
    }
    out
}
