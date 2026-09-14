// spec: gate-sdk/SPEC.md §Layout and configuration — gate-sdk's static knob table: every layout and
// consumer knob its library defaulted, the locators and execution settings it refuses in a file
use super::{Kit, Resolve, Row, Shape, Value};

fn input_scalar(resolve: Resolve, name: &str) -> Result<String, String> {
    resolve(name).map(|(v, _)| v.wire())
}

fn in_gates_dir(resolve: Resolve, base: &str) -> Result<Value, String> {
    input_scalar(resolve, "GATE_SDK_GATES_DIR").map(|d| Value::Scalar(format!("{}/{}", d, base)))
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the crate root is normalized where the row resolves, once
fn crate_root(resolve: Resolve) -> Result<String, String> {
    input_scalar(resolve, "GATE_SDK_NATIVE_CRATE").map(|c| c.trim_end_matches('/').to_string())
}

fn in_crate(resolve: Resolve, base: &str) -> Result<Value, String> {
    crate_root(resolve).map(|c| Value::Scalar(format!("{}/{}", c, base)))
}

fn hooks_dir(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "git-hooks")
}

fn root_allowlist(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "root-allowlist.list")
}

fn core_files_file(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "core-files.list")
}

fn identity_file(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "identity.conf")
}

fn tests_dir(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "gate-tests")
}

fn graph_artifact(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "CHECK-GRAPH.html")
}

fn graph_theme_dir(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "graph-theme")
}

fn graph_vocab(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "graph-vocab.knobs")
}

fn msg_pattern_files(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "msg-patterns.list")
}

fn msg_pattern_files_local(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "msg-patterns.local.list")
}

fn portability_patterns(resolve: Resolve) -> Result<Value, String> {
    in_gates_dir(resolve, "portability-patterns.list")
}

// spec: gate-sdk/SPEC.md §check-exec-bit — the three kit globs, then the gates directory's own two
fn exec_globs(resolve: Resolve) -> Result<Value, String> {
    let g = input_scalar(resolve, "GATE_SDK_GATES_DIR")?;
    Ok(Value::Scalar(format!(
        "*/checks/*.sh */kpis/*.sh */bin/*.sh {0}/check-*.sh {0}/kpi-*.sh",
        g
    )))
}

fn native_src(resolve: Resolve) -> Result<Value, String> {
    in_crate(resolve, "src")
}

fn native_targets_file(resolve: Resolve) -> Result<Value, String> {
    in_crate(resolve, "targets.list")
}

fn native_runners_file(resolve: Resolve) -> Result<Value, String> {
    in_crate(resolve, "runners.list")
}

fn cargo_target_dir(resolve: Resolve) -> Result<Value, String> {
    in_crate(resolve, "target")
}

// spec: gate-sdk/SPEC.md §Layout and configuration — the host's executable suffix, the standard
// library's constant for the platform the binary was built for, which is the one it runs on
fn native_bin(_resolve: Resolve) -> Result<Value, String> {
    Ok(Value::Scalar(format!(
        "native/target/release/checkwright-gates{}",
        std::env::consts::EXE_SUFFIX
    )))
}

const GATES: &[&str] = &["GATE_SDK_GATES_DIR"];
const CRATE: &[&str] = &["GATE_SDK_NATIVE_CRATE"];

pub const KIT: Kit = Kit {
    root: "gate-sdk",
    rows: &[
        Row::scalar("GATE_SDK_WORKFLOW_DIR", ".workflow"),
        Row::scalar("GATE_SDK_TMP_DIR", ".tmp"),
        Row::scalar("GATE_SDK_QUEUE_FILE", "TASK-QUEUE.md"),
        Row::scalar("GATE_SDK_AGENT_FILE", "CLAUDE.md"),
        Row::scalar("GATE_SDK_REGISTRY_DOC", "README.md"),
        Row::scalar("GATE_SDK_RUNNER_DOC", "README.md"),
        Row::scalar("GATE_SDK_ENFORCE_SCAN_DIR", "."),
        Row::scalar("GATE_SDK_GIT_EMAIL_FILE", ""),
        Row::scalar("GATE_SDK_GIT_REMOTES_FILE", ""),
        Row::scalar("GATE_SDK_GH_HOSTS_FILE", ""),
        Row::scalar("GATE_SDK_GH_HOST", "github.com").empty_takes_default(),
        Row::scalar("GATE_SDK_UPGRADE_REPO", ""),
        Row::scalar("GATE_SDK_UPGRADE_FROM", ""),
        Row::scalar("GATE_SDK_UPGRADE_TO", "HEAD").empty_takes_default(),
        Row::scalar("GATE_SDK_GRAPH_MAX_EDGES", "100000").empty_takes_default(),
        Row::scalar("GATE_SDK_NATIVE_CRATE", "native").empty_takes_default(),
        Row::scalar("GATE_SDK_NATIVE_PUBLISH_WORKFLOW", ".github/workflows/publish.yml").empty_takes_default(),
        Row::derived("GATE_SDK_HOOKS_DIR", Shape::Scalar, hooks_dir, GATES),
        Row::derived("GATE_SDK_ROOT_ALLOWLIST", Shape::Scalar, root_allowlist, GATES),
        Row::derived("GATE_SDK_CORE_FILES_FILE", Shape::Scalar, core_files_file, GATES),
        Row::derived("GATE_SDK_IDENTITY_FILE", Shape::Scalar, identity_file, GATES).empty_takes_default(),
        Row::derived("GATE_SDK_TESTS_DIR", Shape::Scalar, tests_dir, GATES).empty_takes_default(),
        Row::derived("GATE_SDK_GRAPH_ARTIFACT", Shape::Scalar, graph_artifact, GATES).empty_takes_default(),
        Row::derived("GATE_SDK_GRAPH_THEME_DIR", Shape::Scalar, graph_theme_dir, GATES).empty_takes_default(),
        Row::derived("GATE_SDK_GRAPH_VOCAB", Shape::Scalar, graph_vocab, GATES).empty_takes_default(),
        Row::derived("GATE_SDK_NATIVE_SRC", Shape::Scalar, native_src, CRATE).empty_takes_default(),
        Row::derived("GATE_SDK_NATIVE_TARGETS_FILE", Shape::Scalar, native_targets_file, CRATE).empty_takes_default(),
        Row::derived("GATE_SDK_NATIVE_RUNNERS_FILE", Shape::Scalar, native_runners_file, CRATE).empty_takes_default(),
        Row::derived("GATE_SDK_CARGO_TARGET_DIR", Shape::Scalar, cargo_target_dir, CRATE).empty_takes_default(),
        Row::derived("GATE_SDK_NATIVE_BIN", Shape::Scalar, native_bin, &[]).empty_takes_default(),
        Row::indexed(
            "GATE_SDK_PROGRAM_FLOOR",
            &[
                "awk", "basename", "bash", "cat", "cd", "chmod", "cmp", "comm", "cp", "cut", "date", "diff",
                "dirname", "env", "find", "git", "grep", "head", "ln", "ls", "mkdir", "mktemp", "mv",
                "printf", "pwd", "realpath", "rm", "sed", "sh", "sort", "tail", "tee", "touch", "tr",
                "uniq", "wc", "xargs",
            ],
        ),
        Row::scalar("GATE_SDK_PRUNE_DIRS", "target .git node_modules .tmp gate-tests worktrees").empty_takes_default(),
        Row::scalar("GATE_SDK_PRUNE_EXTRA_DIRS", "").empty_takes_default(),
        Row::derived("GATE_SDK_EXEC_GLOBS", Shape::Scalar, exec_globs, GATES).empty_takes_default(),
        Row::scalar("GATE_SDK_EXEC_PRUNE", "gate-tests fixtures templates smoke").empty_takes_default(),
        Row::scalar("GATE_SDK_LINT_EXTRA_DIRS", "").empty_takes_default(),
        Row::scalar("GATE_SDK_GRAPH_EXTERNAL_REFS", "").empty_takes_default(),
        Row::derived("GATE_SDK_MSG_PATTERN_FILES", Shape::Scalar, msg_pattern_files, GATES).empty_takes_default(),
        Row::derived("GATE_SDK_MSG_PATTERN_FILES_LOCAL", Shape::Scalar, msg_pattern_files_local, GATES)
            .empty_takes_default(),
        Row::derived("GATE_SDK_PORTABILITY_PATTERNS", Shape::Scalar, portability_patterns, GATES).empty_takes_default(),
        Row::scalar("GATE_SDK_PORTABILITY_PATHS", "").empty_takes_default(),
        Row::scalar("GATE_SDK_KIT_DIRS", "").empty_takes_default(),
        Row::scalar("GATE_SDK_COMMIT_TYPES", "feat fix refactor perf docs test build ci chore style")
            .empty_takes_default(),
    ],
    validate: None,
    open_family: false,
    families: &[],
    retired: &[("GATE_SDK_GRAPH_THEME", "GATE_SDK_GRAPH_THEME_DIR")],
    env_only: &["GATE_SDK_GATES_DIR", "GATE_SDK_ROOT", "GATE_SDK_JOBS", "GATE_SDK_VERBOSE"],
};

#[cfg(test)]
mod tests {
    use super::super::{reset, resolve, Value};
    use crate::knobenv;
    use std::path::Path;

    // spec: gate-sdk/SPEC.md §lib/gate.sh — one pre-binary accessor's answer, read out of a bash that
    // sources the library under this process's environment
    fn accessor(call: &str) -> String {
        let lib = Path::new(env!("CARGO_MANIFEST_DIR")).join("../gate-sdk/lib/gate.sh");
        let out = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("source \"$1\" && {}", call))
            .arg("bash")
            .arg(&lib)
            .output()
            .expect("cannot run the shell library");
        assert!(out.status.success(), "{} failed: {}", call, String::from_utf8_lossy(&out.stderr));
        String::from_utf8_lossy(&out.stdout).trim_end_matches('\n').to_string()
    }

    fn resolved(name: &str) -> String {
        match resolve(name).unwrap_or_else(|e| panic!("{}: {}", name, e)).0 {
            Value::Scalar(s) => s,
            v => v.wire(),
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the pre-binary accessors are a second holder of six values,
    // held to the table under an environment value, an overlay value, a tracked value, an empty value
    // and nothing
    #[test]
    fn the_pre_binary_accessors_answer_what_the_table_resolves() {
        let env = knobenv::lock();
        let dir = std::env::temp_dir().join(format!("checkwright-prebinary.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        let d = dir.display().to_string();
        let names = [
            ("GATE_SDK_NATIVE_BIN", "gate_native_bin"),
            ("GATE_SDK_NATIVE_CRATE", "gate_native_crate"),
            ("GATE_SDK_NATIVE_TARGETS_FILE", "gate_native_targets_file"),
            ("GATE_SDK_NATIVE_RUNNERS_FILE", "gate_native_runners_file"),
        ];
        let clear = |env: &knobenv::KnobEnv| {
            for n in ["GATE_SDK_NATIVE_BIN", "GATE_SDK_NATIVE_CRATE", "GATE_SDK_NATIVE_TARGETS_FILE", "GATE_SDK_NATIVE_RUNNERS_FILE", "GATE_SDK_MSG_PATTERN_FILES", "GATE_SDK_MSG_PATTERN_FILES_LOCAL", "GATE_SDK_KNOB_FILE"] {
                env.remove(n);
            }
            let _ = std::fs::remove_file(dir.join("gate-sdk-config.knobs"));
            let _ = std::fs::remove_file(dir.join("gate-sdk-config.local.knobs"));
            reset(env);
        };
        env.set("GATE_SDK_GATES_DIR", &d);
        let mut checked = 0usize;
        for case in ["nothing", "environment", "overlay", "tracked", "empty"] {
            clear(&env);
            for (name, _) in names.iter().copied().chain([("GATE_SDK_MSG_PATTERN_FILES", ""), ("GATE_SDK_MSG_PATTERN_FILES_LOCAL", "")]) {
                let value = format!("{}/{}-from-{}", d, name.to_ascii_lowercase(), case);
                match case {
                    "environment" => env.set(name, &value),
                    "empty" => env.set(name, ""),
                    "overlay" | "tracked" => {
                        let file = if case == "overlay" { "gate-sdk-config.local.knobs" } else { "gate-sdk-config.knobs" };
                        let p = dir.join(file);
                        let mut text = std::fs::read_to_string(&p).unwrap_or_default();
                        text.push_str(&format!("{} = {}\n", name, value));
                        std::fs::write(&p, text).expect("write");
                    }
                    _ => {}
                }
            }
            reset(&env);
            for (name, call) in names {
                assert_eq!(accessor(call), resolved(name), "{} under {}", call, case);
                checked += 1;
            }
            let files: Vec<String> = [resolved("GATE_SDK_MSG_PATTERN_FILES"), resolved("GATE_SDK_MSG_PATTERN_FILES_LOCAL")]
                .iter()
                .flat_map(|v| v.split_whitespace().map(String::from).collect::<Vec<_>>())
                .collect();
            for f in &files {
                if let Some(parent) = Path::new(f).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(f, "");
            }
            assert_eq!(accessor("gate_msg_pattern_files"), files.join("\n"), "gate_msg_pattern_files under {}", case);
            for f in &files {
                if f.starts_with(&d) {
                    let _ = std::fs::remove_file(f);
                }
            }
            checked += 1;
        }
        clear(&env);
        env.remove("GATE_SDK_GATES_DIR");
        reset(&env);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(checked, 25, "an accessor case went unchecked");
    }
}
