// spec: gate-sdk/SPEC.md §The non-gate arm — the ported arms. Each owes no descriptor, no
// registration and no fixture pair, and owes a named caller instead: a regen command, a
// comparator calling `emit()`, a stage step, a gate reaching it in process.
pub mod close_surfaces;
pub mod enforcement_map;
pub mod footprint;
pub mod queue_index;
pub mod value_rollup;

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's own argv tail, so a projection whose
// generator has a write-in-place mode takes it as a flag rather than needing a second arm. The
// returned string is what the arm prints: the document, or the action a write performed.
pub type EmitFn = fn(&[String]) -> Result<String, String>;

// spec: gate-sdk/SPEC.md §The non-gate arm — the projection's own name keys the arm, so no
// mapping table exists to drift. The third element is the arm's bridged knob reads, the data
// `--knobs` prints for it.
pub const EMITTERS: &[(&str, EmitFn, &[&str])] = &[
    ("footprint", footprint::emit, &["CONTEXT_KIT_SURFACES"]),
    // spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the class's first member that is
    // not a stored projection: the roster's value is that it is recomputed at the moment close
    // reads it, so there is no comparator and must not be.
    (
        "close-surfaces",
        close_surfaces::emit,
        &[
            "GATE_KIT_ROOTS_REL",
            "LIFECYCLE_KIT_ROSTER_BASENAME",
            "LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS",
            "GATE_SDK_WORKFLOW_DIR",
        ],
    ),
    (
        "enforcement-map",
        enforcement_map::emit,
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_ENFORCE_SCAN_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_PRUNE_DIRS",
            "DRIFT_KIT_KPIS_FILE",
            "CONTEXT_KIT_SETTINGS_FILE",
            "CANON_KIT_DOCS_BLOB_REF",
            "EVIDENCE_KIT_SUITES",
            "EVIDENCE_KIT_RUN_*",
        ],
    ),
    // spec: docs/site-architecture.md §Generated projections and their freshness gates — the join
    // reads both sibling emitters live, so it declares the union of what they read.
    (
        "value-rollup",
        value_rollup::emit,
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_ENFORCE_SCAN_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_PRUNE_DIRS",
            "DRIFT_KIT_KPIS_FILE",
            "CONTEXT_KIT_SETTINGS_FILE",
            "CANON_KIT_DOCS_BLOB_REF",
            "EVIDENCE_KIT_SUITES",
            "EVIDENCE_KIT_RUN_*",
            "CONTEXT_KIT_SURFACES",
        ],
    ),
    // spec: queue-kit/SPEC.md §The queue-index arm — the class's first *query* member as well as a
    // generator, and the table's most configured one: a hardcoded flag receives no configuration
    (
        "queue-index",
        queue_index::emit,
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
            "QUEUE_KIT_ATTEND_CAP",
            "QUEUE_KIT_ICEBOX_AGE_DAYS",
        ],
    ),
];

// spec: gate-sdk/SPEC.md §The non-gate arm — resolved before the registry lookup and absent from
// `--list`, which is what keeps check-gate-substrate-parity assertion B's equality true in both
// directions.
pub fn arm_name(projection: &str) -> String {
    format!("--emit-{}", projection)
}

pub fn lookup(arm: &str) -> Option<EmitFn> {
    EMITTERS
        .iter()
        .find(|(p, _, _)| arm_name(p) == arm)
        .map(|(_, f, _)| *f)
}

pub fn knobs(arm: &str) -> Option<&'static [&'static str]> {
    EMITTERS
        .iter()
        .find(|(p, _, _)| arm_name(p) == arm)
        .map(|(_, _, k)| *k)
}

pub fn projections() -> Vec<&'static str> {
    EMITTERS.iter().map(|(p, _, _)| *p).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The non-gate arm — the arm spelling is derived from the projection
    // name, so a typo cannot resolve to a different member and no mapping table exists to drift
    #[test]
    fn an_arm_resolves_only_under_its_derived_spelling() {
        assert!(lookup("--emit-footprint").is_some());
        assert!(lookup("footprint").is_none());
        assert!(lookup("--emit-footprints").is_none());
        assert_eq!(knobs("--emit-footprint"), Some(&["CONTEXT_KIT_SURFACES"][..]));
    }
}
