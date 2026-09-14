// spec: drift-kit/SPEC.md §Layout and configuration — drift-kit's static knob table and validator
use super::{set_but_missing, Kit, Origin, Resolve, Row, Shape, Value, Values};

fn bridged_scalar(resolve: Resolve, name: &str) -> Result<(String, Origin), String> {
    resolve(name).map(|(v, o)| (v.wire(), o))
}

fn under(resolve: Resolve, input: &str, base: &str) -> Result<Value, String> {
    bridged_scalar(resolve, input).map(|(d, _)| Value::Scalar(format!("{}/{}", d, base)))
}

// spec: drift-kit/SPEC.md §Layout and configuration — the registry at its default path when that file
// exists, else empty, the not-adopted answer; the roster's placeholder renders the path unprobed
fn kpis_file(resolve: Resolve) -> Result<Value, String> {
    let (gates, origin) = bridged_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    let path = format!("{}/kpis.list", gates);
    if origin == Origin::Placeholder || std::path::Path::new(&path).is_file() {
        return Ok(Value::Scalar(path));
    }
    Ok(Value::Scalar(String::new()))
}

// spec: drift-kit/SPEC.md §Layout and configuration — one scalar, `<state-file> <evidence-file>`
fn trajectory_surfaces(resolve: Resolve) -> Result<Value, String> {
    let (w, _) = bridged_scalar(resolve, "GATE_SDK_WORKFLOW_DIR")?;
    Ok(Value::Scalar(format!("{0}/WORKFLOW-STATE.txt {0}/validate-evidence.txt", w)))
}

fn gates_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_GATES_DIR", "gates.list")
}

fn price_table(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_GATES_DIR", "price-table.tsv")
}

fn kpi_dirs(resolve: Resolve) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_GATES_DIR").map(|(g, _)| Value::Indexed(vec![g]))
}

fn queue_file(resolve: Resolve) -> Result<Value, String> {
    resolve("GATE_SDK_QUEUE_FILE").map(|(v, _)| v)
}

fn knowledge_log(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "knowledge-friction.log")
}

fn state_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "WORKFLOW-STATE.txt")
}

fn timings_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_TMP_DIR", "gate-timings.txt")
}

fn tmp_dir(resolve: Resolve) -> Result<Value, String> {
    resolve("GATE_SDK_TMP_DIR").map(|(v, _)| v)
}

fn overhead_log(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "DRIFT_KIT_METRIC_DIR", "overhead-log.txt")
}

fn stage_economics_log(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "DRIFT_KIT_METRIC_DIR", "stage-economics-log.txt")
}

pub const KIT: Kit = Kit {
    root: "drift-kit",
    rows: &[
        Row::derived("DRIFT_KIT_KPIS_FILE", Shape::Scalar, kpis_file, &["GATE_SDK_GATES_DIR"]),
        Row::derived(
            "DRIFT_KIT_TRAJECTORY_SURFACES",
            Shape::Scalar,
            trajectory_surfaces,
            &["GATE_SDK_WORKFLOW_DIR"],
        ),
        Row::derived("DRIFT_KIT_GATES_FILE", Shape::Scalar, gates_file, &["GATE_SDK_GATES_DIR"]),
        Row::indexed("DRIFT_KIT_STAGES", &["scope", "align", "build", "validate", "close"]),
        Row::derived("DRIFT_KIT_QUEUE_FILE", Shape::Scalar, queue_file, &["GATE_SDK_QUEUE_FILE"]),
        Row::derived("DRIFT_KIT_KNOWLEDGE_LOG", Shape::Scalar, knowledge_log, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("DRIFT_KIT_TIMINGS_FILE", Shape::Scalar, timings_file, &["GATE_SDK_TMP_DIR"]),
        Row::derived("DRIFT_KIT_TMP_DIR", Shape::Scalar, tmp_dir, &["GATE_SDK_TMP_DIR"]),
        Row::scalar("DRIFT_KIT_METRIC_DIR", ".metric"),
        Row::scalar("DRIFT_KIT_DONE_SECTION", "Done"),
        Row::scalar("DRIFT_KIT_DEFERRED_SECTION", "Deferred"),
        Row::scalar("DRIFT_KIT_ICEBOX_SECTION", ""),
        Row::scalar("DRIFT_KIT_SESSIONS_DIR", ""),
        Row::derived("DRIFT_KIT_OVERHEAD_LOG", Shape::Scalar, overhead_log, &["DRIFT_KIT_METRIC_DIR"]),
        Row::derived(
            "DRIFT_KIT_STAGE_ECONOMICS_LOG",
            Shape::Scalar,
            stage_economics_log,
            &["DRIFT_KIT_METRIC_DIR"],
        ),
        Row::derived("DRIFT_KIT_STATE_FILE", Shape::Scalar, state_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::scalar("DRIFT_KIT_SUPERVISION_LABEL", "supervision"),
        Row::scalar("DRIFT_KIT_FANOUT_SUFFIX", "+fanout"),
        Row::derived("DRIFT_KIT_PRICE_TABLE", Shape::Scalar, price_table, &["GATE_SDK_GATES_DIR"]),
        Row::derived("DRIFT_KIT_KPI_DIRS", Shape::Indexed, kpi_dirs, &["GATE_SDK_GATES_DIR"]),
    ],
    validate: Some(("drift config", validate)),
    open_family: true,
    retired: &[],
};

// spec: drift-kit/SPEC.md §Layout and configuration — the registry a consumer set to a path that is
// absent is adopted-but-broken; an empty value stays the not-adopted answer
fn validate(v: &Values) -> Vec<String> {
    set_but_missing(v, "DRIFT_KIT_KPIS_FILE").into_iter().collect()
}
