// spec: evidence-kit/SPEC.md §Layout and configuration — evidence-kit's static knob table, its three
// declared families and its validator
use super::{indexed, is_suffix, scalar, Family, Kit, Origin, Resolve, Row, Shape, Value, Values};

fn input_scalar(resolve: Resolve, name: &str) -> Result<(String, Origin), String> {
    resolve(name).map(|(v, o)| (v.wire(), o))
}

fn under(resolve: Resolve, input: &str, base: &str) -> Result<Value, String> {
    input_scalar(resolve, input).map(|(d, _)| Value::Scalar(format!("{}/{}", d, base)))
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
    input_scalar(resolve, "GATE_SDK_QUEUE_FILE").map(|(q, _)| Value::Scalar(q))
}

fn tmp_dir(resolve: Resolve) -> Result<Value, String> {
    input_scalar(resolve, "GATE_SDK_TMP_DIR").map(|(t, _)| Value::Scalar(t))
}

fn lock_file(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "EVIDENCE_KIT_TMP_DIR", "run-validate.lock")
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the fixture suites over the kit roots and the gates dir; a
// placeholder input names no directory to probe, so the roster renders none
fn suites(resolve: Resolve) -> Result<Vec<(String, String, String)>, String> {
    let (root, origin) = input_scalar(resolve, "GATE_SDK_ROOT")?;
    let (dirs, _) = input_scalar(resolve, "GATE_SDK_KIT_DIRS")?;
    let (gates, _) = input_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    if origin == Origin::Placeholder {
        return Ok(Vec::new());
    }
    // spec: gate-sdk/SPEC.md §Layout and configuration — `fixture_suites_in` statts both joins and
    // its rows are the operands `--run-gate-tests` receives, so the roots arrive
    // working-directory-spelled, matching the `spelled_here` root `run_members` prints beside them
    let roots = crate::walk::kit_roots_from(&root, &dirs)?;
    Ok(crate::registry::fixture_suites_in(&roots, &gates))
}

fn fixture_suites(resolve: Resolve) -> Result<Value, String> {
    Ok(Value::Indexed(suites(resolve)?.into_iter().map(|(s, _, _)| s).collect()))
}

// spec: evidence-kit/SPEC.md §Layout and configuration — one run member per fixture suite, the
// fixture runner over its tests dir and, where one exists, its checks dir, on the binary door
fn run_members(resolve: Resolve) -> Result<Vec<(String, String)>, String> {
    let (bin, _) = input_scalar(resolve, "GATE_SDK_NATIVE_BIN")?;
    let door = crate::installer::init::command_token(&bin);
    Ok(suites(resolve)?
        .into_iter()
        .map(|(s, tests, checks)| {
            let mut cmd = format!("{} --run-gate-tests {}", door, tests);
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
        Row::indexed("EVIDENCE_KIT_SUITES", &[]),
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
        Row::scalar("EVIDENCE_KIT_STATEMENT_FILE", ""),
        Row::indexed("EVIDENCE_KIT_STATEMENT_CMD", &[]),
        Row::indexed("EVIDENCE_KIT_STATEMENT_WRITE_CMD", &[]),
        Row::scalar("EVIDENCE_KIT_VERIFY_REQUIRE", ""),
        Row::scalar("EVIDENCE_KIT_TREE_UNTRACKED", "ignore"),
        Row::scalar("EVIDENCE_KIT_IMPORTED_RUNS", "refuse"),
        Row::indexed("EVIDENCE_KIT_CONTROLS", &[]),
        Row::scalar("EVIDENCE_KIT_CONTROL_SINCE", ""),
        Row::scalar("EVIDENCE_KIT_RUNNER_DOC", "README.md"),
        Row::keyed("EVIDENCE_KIT_SCENARIO_GLOBS", &[]),
        Row::indexed("EVIDENCE_KIT_PERMANENT_SLUGS", &[]),
        Row::scalar("EVIDENCE_KIT_DONE_SECTION", "Done"),
        Row::derived(
            "EVIDENCE_KIT_FIXTURE_SUITES",
            Shape::Indexed,
            fixture_suites,
            &["GATE_SDK_ROOT", "GATE_SDK_KIT_DIRS", "GATE_SDK_GATES_DIR"],
        ),
    ],
    validate: Some(("evidence config", validate)),
    open_family: false,
    families: &[
        Family {
            prefix: "EVIDENCE_KIT_RUN_",
            derive: Some(run_members),
            inputs: &["GATE_SDK_ROOT", "GATE_SDK_KIT_DIRS", "GATE_SDK_GATES_DIR", "GATE_SDK_NATIVE_BIN"],
        },
        Family { prefix: "EVIDENCE_KIT_PARSER_", derive: None, inputs: &[] },
        Family { prefix: "EVIDENCE_KIT_CONTROL_", derive: None, inputs: &[] },
    ],
    retired: &[],
    env_only: &[],
};

// spec: evidence-kit/SPEC.md §Layout and configuration — the paths, the parser and the Done section
// non-empty, every suite and control a valid family suffix, the import and untracked selections one
// of their two words each, and every verify requirement a known word
fn validate(v: &Values) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for name in [
        "EVIDENCE_KIT_PARSER",
        "EVIDENCE_KIT_BASELINE_FILE",
        "EVIDENCE_KIT_MANIFEST_FILE",
        "EVIDENCE_KIT_QUEUE_FILE",
        "EVIDENCE_KIT_DONE_SECTION",
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
    for c in indexed(v, "EVIDENCE_KIT_CONTROLS").unwrap_or(&[]) {
        if !is_suffix(c) {
            out.push(format!("control name '{}' is not a valid EVIDENCE_KIT_CONTROL_<name> suffix", c));
        }
    }
    if let Some(sel) = scalar(v, "EVIDENCE_KIT_IMPORTED_RUNS").filter(|s| !["refuse", "accept"].contains(s)) {
        out.push(format!("EVIDENCE_KIT_IMPORTED_RUNS is '{}' (want refuse or accept)", sel));
    }
    if let Some(sel) = scalar(v, "EVIDENCE_KIT_TREE_UNTRACKED").filter(|s| !["ignore", "count"].contains(s)) {
        out.push(format!("EVIDENCE_KIT_TREE_UNTRACKED is '{}' (want ignore or count)", sel));
    }
    for word in scalar(v, "EVIDENCE_KIT_VERIFY_REQUIRE").unwrap_or("").split_whitespace() {
        if !VERIFY_REQUIREMENTS.contains(&word) {
            out.push(format!("EVIDENCE_KIT_VERIFY_REQUIRE word '{}' is not one of head, verdict", word));
        }
    }
    out
}

pub const VERIFY_REQUIREMENTS: &[&str] = &["head", "verdict"];

#[cfg(test)]
mod tests {
    use super::*;

    fn one(name: &'static str, value: &str) -> Vec<String> {
        let mut v: Values = Values::new();
        v.insert(name, (Value::Scalar(value.to_string()), Origin::Tracked));
        validate(&v)
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — the untracked-file selection is one of
    // its two words, spelled exactly
    #[test]
    fn the_untracked_selection_admits_only_its_two_words() {
        assert!(one("EVIDENCE_KIT_TREE_UNTRACKED", "ignore").is_empty());
        assert!(one("EVIDENCE_KIT_TREE_UNTRACKED", "count").is_empty());
        for bad in ["", "off", "Count", "all"] {
            let e = one("EVIDENCE_KIT_TREE_UNTRACKED", bad);
            assert!(e.len() == 1 && e[0].contains("want ignore or count"), "{:?} -> {:?}", bad, e);
        }
    }

    // spec: evidence-kit/SPEC.md §Layout and configuration — the verify requirements are a set of
    // known words, empty selecting none
    #[test]
    fn the_verify_requirements_admit_only_known_words() {
        for ok in ["", "head", "verdict", "head verdict", " verdict  head "] {
            assert!(one("EVIDENCE_KIT_VERIFY_REQUIRE", ok).is_empty(), "{:?}", ok);
        }
        let e = one("EVIDENCE_KIT_VERIFY_REQUIRE", "head ahead Verdict");
        assert_eq!(e.len(), 2, "{:?}", e);
        assert!(e[0].contains("'ahead'") && e[1].contains("'Verdict'") && e[0].contains("head, verdict"), "{:?}", e);
    }
}
