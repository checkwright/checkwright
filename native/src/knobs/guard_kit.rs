// spec: guard-kit/SPEC.md §Layout and configuration — guard-kit's static knob table and validator
use super::{scalar, Kit, Resolve, Row, Shape, Value, Values};

fn under(resolve: Resolve, input: &str, base: &str) -> Result<Value, String> {
    resolve(input).map(|(d, _)| Value::Scalar(format!("{}/{}", d.wire(), base)))
}

fn log(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "prompt-friction.log")
}

fn wakeup_log(resolve: Resolve) -> Result<Value, String> {
    under(resolve, "GATE_SDK_WORKFLOW_DIR", "wakeup-attempts.log")
}

pub const KIT: Kit = Kit {
    root: "guard-kit",
    rows: &[
        Row::derived("GUARD_KIT_LOG", Shape::Scalar, log, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::derived("GUARD_KIT_WAKEUP_LOG", Shape::Scalar, wakeup_log, &["GATE_SDK_WORKFLOW_DIR"]),
        Row::scalar("GUARD_KIT_SETTINGS", ".claude/settings.json"),
        Row::scalar("GUARD_KIT_SETTINGS_LOCAL", ".claude/settings.local.json"),
        Row::indexed("GUARD_KIT_BREADTH_PROBES", &[]),
        Row::indexed("GUARD_KIT_DOOR_ROOTS", &[]),
        Row::keyed("GUARD_KIT_BREADTH_DECLARED", &[]),
        Row::indexed("GUARD_KIT_RO_SCRIPTS", &["check-*.sh"]),
        Row::indexed("GUARD_KIT_SCRATCH_DIRS", &[".tmp"]),
        Row::indexed(
            "GUARD_KIT_RO_BINS",
            &[
                "grep", "egrep", "fgrep", "rg", "head", "tail", "cat", "wc", "sort", "uniq", "cut", "tr", "nl",
                "rev", "tac", "paste", "comm", "column", "diff", "jq", "find", "ls", "xargs",
            ],
        ),
        Row::keyed("GUARD_KIT_RO_FORMS", &[]),
        Row::indexed("GUARD_KIT_APPEND_BINS", &["cat", "printf", "echo"]),
        Row::indexed("GUARD_KIT_SEARCH_TOOLS", &["Glob", "Grep"]),
        Row::indexed(
            "GUARD_KIT_SCRIPT_INTERPRETERS",
            &["python", "python3", "node", "deno", "ruby", "perl", "php", "zsh"],
        ),
        Row::scalar("GUARD_KIT_WORKTREE_READS", "read-only"),
        Row::indexed("GUARD_KIT_CONSUMER_RULES_CMD", &[]),
    ],
    validate: Some(("guard config", validate)),
    open_family: false,
    families: &[],
    retired: &[],
    env_only: &[],
};

// spec: guard-kit/SPEC.md §Layout and configuration — guard-kit rule `worktree_confinement`'s admission selector takes its two
// values only, so a misspelt `off` refuses rather than reads as the default
fn validate(v: &Values) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    if let Some(s) = scalar(v, "GUARD_KIT_WORKTREE_READS").filter(|s| !matches!(*s, "read-only" | "off")) {
        errs.push(format!("GUARD_KIT_WORKTREE_READS must be read-only|off (got '{}')", s));
    }
    errs
}
