// spec: gate-sdk/SPEC.md §The non-gate arm — the ported arms. Each owes no descriptor, no
// registration and no fixture pair, and owes a named caller instead: a regen command, a
// comparator calling `emit()`, a stage step, a gate reaching it in process.
pub mod close_surfaces;
pub mod docs_mirror;
pub mod enforcement_map;
pub mod footprint;
pub mod graph;
pub mod queue_index;
pub mod roadmap;
pub mod trajectory;
pub mod value_rollup;

// spec: gate-sdk/SPEC.md §lib/gate.sh — gate_self_repo_prefix, degrading to nothing on no origin
// or an unrecognised form. It sits on the family because two arms render self-repo links, and a
// second copy of the normalisation is a second identity to disagree about.
pub fn self_repo_prefix(reference: &str) -> String {
    let origin = match crate::proc::run("git", &["remote", "get-url", "origin"]) {
        Ok(c) => match c.stdout() {
            Some(o) => String::from_utf8_lossy(o).trim().to_string(),
            None => return String::new(),
        },
        Err(_) => return String::new(),
    };
    if origin.is_empty() {
        return String::new();
    }
    let id = origin
        .strip_suffix(".git")
        .unwrap_or(&origin)
        .trim_end_matches('/')
        .to_string();
    let id = if let Some(rest) = id.strip_prefix("git@") {
        match rest.split_once(':') {
            Some((host, path)) => format!("https://{}/{}", host, path),
            None => return String::new(),
        }
    } else if id.starts_with("https://") || id.starts_with("http://") {
        id
    } else {
        return String::new();
    };
    format!("{}/blob/{}/", id, reference)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's own argv tail, so a projection whose
// generator has a write-in-place mode takes it as a flag rather than needing a second arm. The
// returned string is what the arm prints: the document, or the action a write performed.
pub type EmitFn = fn(&[String]) -> Result<String, String>;

// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged arm either renders a document or returns an
// exit code; the class the table keys is *bridged*, not *emitting*, so both shapes sit in it.
pub enum Arm {
    Emit(EmitFn),
    Run(fn(&[String]) -> i32),
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the **bridged-arm table**, keyed by the arm's own flag
// spelling: `--knobs` publishes each member's roster and a front-end resolves it, which is the
// property the members share. `--emit-` is one arm family's spelling, not the table's name.
pub const BRIDGED_ARMS: &[(&str, Arm, &[&str])] = &[
    (
        "--emit-footprint",
        Arm::Emit(footprint::emit),
        &["CONTEXT_KIT_SURFACES"],
    ),
    // spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the class's first member that is
    // not a stored projection: the roster's value is that it is recomputed at the moment close
    // reads it, so there is no comparator and must not be.
    (
        "--emit-close-surfaces",
        Arm::Emit(close_surfaces::emit),
        &[
            "GATE_KIT_ROOTS_REL",
            "LIFECYCLE_KIT_ROSTER_BASENAME",
            "LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS",
            "GATE_SDK_WORKFLOW_DIR",
        ],
    ),
    (
        "--emit-enforcement-map",
        Arm::Emit(enforcement_map::emit),
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
        "--emit-value-rollup",
        Arm::Emit(value_rollup::emit),
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
    // spec: canon-kit/SPEC.md §The reference-link grammar — the source set is derived from the
    // tracked tree rather than enumerated, so the only configured value is the blob ref
    (
        "--emit-docs-mirror",
        Arm::Emit(docs_mirror::emit),
        &["CANON_KIT_DOCS_BLOB_REF"],
    ),
    // spec: drift-kit/SPEC.md §The published-evidence extractor — the stage roster and the
    // evidence-surface pair are this consumer's vocabulary, so they cross the bridge as knobs; a
    // stage name in the crate would ship one project's lifecycle as everyone's
    (
        "--emit-trajectory",
        Arm::Emit(trajectory::emit),
        &[
            "DRIFT_KIT_CONFIG_FILE",
            "DRIFT_KIT_TRAJECTORY_SURFACES",
            "DRIFT_KIT_GATES_FILE",
            "DRIFT_KIT_STAGES",
            "GATE_SDK_WORKFLOW_DIR",
        ],
    ),
    // spec: queue-kit/SPEC.md §The roadmap arm — the consumer's editorial vocabulary plus the
    // section trio that scopes the scan. TRACKS rides although the arm only prints it verbatim:
    // one table entry serves check-roadmap-fresh's caller too, and that one validates it.
    (
        "--emit-roadmap",
        Arm::Emit(roadmap::emit),
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
            "QUEUE_KIT_HORIZONS",
            "QUEUE_KIT_TRACKS",
            "QUEUE_KIT_ROADMAP_FILE",
            "QUEUE_KIT_ROADMAP_MARKER",
        ],
    ),
    // spec: gate-sdk/SPEC.md §check-graph — the theme crosses as a *path* rather than as content:
    // the bridge refuses any element carrying a newline and a stylesheet is newline-bearing by
    // construction. Values cross the bridge; documents cross as a path.
    (
        "--emit-graph",
        Arm::Emit(graph::emit),
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_KIT_ROOTS_REL",
            "GATE_SDK_GRAPH_ARTIFACT",
            "GATE_SDK_GRAPH_THEME_DIR",
            "GATE_SDK_GRAPH_MAX_EDGES",
            "GRAPH_LAYERS",
            "GRAPH_LAYER_RULES",
            "GRAPH_LAYER_DEFAULT",
        ],
    ),
    // spec: queue-kit/SPEC.md §The queue-index arm — the class's first *query* member as well as a
    // generator, and configured: a hardcoded flag receives no configuration
    (
        "--emit-queue-index",
        Arm::Emit(queue_index::emit),
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
            "QUEUE_KIT_ATTEND_CAP",
            "QUEUE_KIT_ICEBOX_AGE_DAYS",
        ],
    ),
    // spec: gate-sdk/SPEC.md §run-gates — the battery runner: the class's first bridged member
    // that returns a verdict rather than a document, and the reason the table is keyed by flag.
    (
        "--run",
        Arm::Run(crate::runner::run),
        crate::runner::KNOBS,
    ),
];

pub fn lookup(arm: &str) -> Option<&'static Arm> {
    BRIDGED_ARMS
        .iter()
        .find(|(a, _, _)| *a == arm)
        .map(|(_, f, _)| f)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — a dispatching arm's declared reads are the **union** of
// its own with every registry member's and every sibling arm's, derived from the registry and this
// table rather than maintained beside them, which is the data both already hold.
pub fn knobs(arm: &str) -> Option<Vec<&'static str>> {
    let (_, f, own) = BRIDGED_ARMS.iter().find(|(a, _, _)| *a == arm)?;
    match f {
        Arm::Emit(_) => Some(own.to_vec()),
        Arm::Run(_) => Some(dispatch_union(own)),
    }
}

fn dispatch_union(own: &'static [&'static str]) -> Vec<&'static str> {
    let mut all: Vec<&'static str> = own.to_vec();
    for (name, _, _, _, _) in crate::gates::REGISTRY {
        if let Some(k) = crate::gates::knobs(name) {
            all.extend_from_slice(k);
        }
    }
    for (a, _, k) in BRIDGED_ARMS {
        if *a != "--run" {
            all.extend_from_slice(k);
        }
    }
    all.sort_unstable();
    all.dedup();
    all
}

pub fn arms() -> Vec<&'static str> {
    BRIDGED_ARMS.iter().map(|(a, _, _)| *a).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The non-gate arm — the table is keyed by the arm's own flag, so a
    // near-miss spelling resolves to nothing rather than to a different member
    #[test]
    fn an_arm_resolves_only_under_its_own_flag() {
        assert!(lookup("--emit-footprint").is_some());
        assert!(lookup("footprint").is_none());
        assert!(lookup("--emit-footprints").is_none());
        assert_eq!(knobs("--emit-footprint"), Some(vec!["CONTEXT_KIT_SURFACES"]));
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the dispatching arm's roster is derived, so it
    // carries a knob no arm declares the moment a registry member declares it
    #[test]
    fn the_dispatching_arms_roster_is_the_union_it_will_have_to_carry() {
        let u = knobs("--run").expect("--run is not in the bridged-arm table");
        assert!(u.contains(&"GATE_SDK_TMP_DIR"), "the arm's own knob is missing");
        assert!(u.contains(&"GATE_PRUNE_DIRS"), "a registry member's knob is missing");
        assert!(u.contains(&"CONTEXT_KIT_SURFACES"), "a sibling arm's knob is missing");
        let mut sorted = u.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(u, sorted, "the union is not deterministic");
    }
}
