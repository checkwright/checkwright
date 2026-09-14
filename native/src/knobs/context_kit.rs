// spec: context-kit/SPEC.md §Layout and configuration — context-kit's static knob table and validator
use super::{scalar, set_but_missing, Kit, Origin, Resolve, Row, Shape, Value, Values};

fn bridged_scalar(resolve: Resolve, name: &str) -> Result<(String, Origin), String> {
    resolve(name).map(|(v, o)| (v.wire(), o))
}

fn in_workflow_dir(resolve: Resolve, base: &str) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_WORKFLOW_DIR").map(|(d, _)| Value::Scalar(format!("{}/{}", d, base)))
}

fn in_gates_dir(resolve: Resolve, base: &str) -> Result<Value, String> {
    bridged_scalar(resolve, "GATE_SDK_GATES_DIR").map(|(d, _)| Value::Scalar(format!("{}/{}", d, base)))
}

fn baseline_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "always-loaded-baseline.txt")
}

fn state_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "WORKFLOW-STATE.txt")
}

fn ceiling_file(resolve: Resolve) -> Result<Value, String> {
    in_workflow_dir(resolve, "surface-ceiling.txt")
}

fn settings_pins(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "settings-pins.conf")
}

fn pub_lang_dir(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "pub-lang")
}

// spec: context-kit/SPEC.md §Layout and configuration — the battery front-end's queue index, found
// consumer-first over two candidates with file existence as the predicate, else empty; the roster's
// placeholder renders the first candidate rather than probe it
fn hook_cmd(resolve: Resolve) -> Result<Value, String> {
    let (gates, origin) = bridged_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    let first = format!("{}/run-gates.sh", gates);
    let argv = |c: String| {
        Value::Indexed(
            ["bash", c.as_str(), "--emit", "queue-index", "--collapse-deferred"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    };
    if origin == Origin::Placeholder || std::path::Path::new(&first).is_file() {
        return Ok(argv(first));
    }
    let (root, _) = bridged_scalar(resolve, "GATE_SDK_ROOT_HERE")?;
    let second = format!("{}/bin/run-gates.sh", root.trim_end_matches('/'));
    if std::path::Path::new(&second).is_file() {
        return Ok(argv(second));
    }
    Ok(Value::Indexed(Vec::new()))
}

pub const KIT: Kit = Kit {
    root: "context-kit",
    rows: &[
        Row::scalar("CONTEXT_KIT_SETTINGS_FILE", ".claude/settings.json"),
        Row::derived("CONTEXT_KIT_SETTINGS_PINS", Shape::Scalar, settings_pins, &["GATE_SDK_GATES_DIR"]),
        Row::scalar("CONTEXT_KIT_MEMORY_DIRS", ""),
        Row::scalar("CONTEXT_KIT_BREVITY_FILE", "CLAUDE.md"),
        Row::indexed("CONTEXT_KIT_BREVITY_SECTIONS", &["## Shared conventions"]),
        Row::scalar("CONTEXT_KIT_BREVITY_BUDGET", "4"),
        Row::scalar("CONTEXT_KIT_BREVITY_POINTER_RE", "§"),
        Row::indexed("CONTEXT_KIT_SURFACES", &["CLAUDE.md"]),
        Row::indexed("CONTEXT_KIT_GROWTH_PATHS", &["*.md"]),
        Row::derived("CONTEXT_KIT_BASELINE_FILE", Shape::Scalar, baseline_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("CONTEXT_KIT_STATE_FILE", Shape::Scalar, state_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("CONTEXT_KIT_CEILING_FILE", Shape::Scalar, ceiling_file, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::indexed("CONTEXT_KIT_RATCHET_PATHS", &[]),
        Row::derived(
            "CONTEXT_KIT_HOOK_CMD",
            Shape::Indexed,
            hook_cmd,
            &["GATE_SDK_GATES_DIR", "GATE_SDK_ROOT_HERE"],
        ),
        Row::scalar("CONTEXT_KIT_ENV_PROFILE_FILE", "ENV.local.md"),
        Row::indexed(
            "CONTEXT_KIT_PRUNE_DIRS",
            &[".git", "node_modules", "target", "dist", "build", "worktrees"],
        ),
        Row::derived("CONTEXT_KIT_PUB_LANG_DIR", Shape::Scalar, pub_lang_dir, &["GATE_SDK_GATES_DIR"]),
        Row::indexed("CONTEXT_KIT_PUB_LANGS", &[]),
    ],
    validate: Some(("context config", validate)),
    open_family: false,
    families: &[],
    retired: &[("CONTEXT_KIT_BREVITY_SECTION", "CONTEXT_KIT_BREVITY_SECTIONS")],
};

// spec: context-kit/SPEC.md §Layout and configuration — a broken context config gates nothing:
// emptiness, the budget's integer shape, and a settings file a consumer set to a path that is absent
fn validate(v: &Values) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    for n in ["CONTEXT_KIT_SETTINGS_FILE", "CONTEXT_KIT_SETTINGS_PINS", "CONTEXT_KIT_BREVITY_FILE"] {
        if scalar(v, n).is_some_and(str::is_empty) {
            errs.push(format!("{} is empty", n));
        }
    }
    errs.extend(set_but_missing(v, "CONTEXT_KIT_SETTINGS_FILE"));
    if let Some(b) = scalar(v, "CONTEXT_KIT_BREVITY_BUDGET") {
        if b.is_empty() || !b.bytes().all(|c| c.is_ascii_digit()) {
            errs.push(format!("CONTEXT_KIT_BREVITY_BUDGET must be an integer (got '{}')", b));
        }
    }
    errs
}
