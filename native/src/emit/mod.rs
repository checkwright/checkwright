// spec: gate-sdk/SPEC.md §The non-gate arm — the ported arms. Each owes no descriptor, no
// registration and no fixture pair, and owes a named caller instead: a regen command, a
// comparator calling `emit()`, a stage step, a gate reaching it in process.
pub mod cite_survey;
pub mod close_surfaces;
pub mod docs_mirror;
pub mod drift_report;
pub mod enforcement_map;
pub mod env_probe;
pub mod file_survey;
pub mod footprint;
pub mod graph;
pub mod install_lifecycle;
pub mod kpi;
pub mod lesson_sink;
pub mod md_index;
pub mod md_section;
pub mod port_blockers;
pub mod pub_index;
pub mod pub_lang;
pub mod queue_counts;
pub mod queue_edges;
pub mod queue_index;
pub mod stage_rules;
pub mod roadmap;
pub mod session_id;
pub mod trajectory;
pub mod upgrade_smoke;
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

// spec: context-kit/SPEC.md §Index-first reading — the index walk both index arms share, sited
// here on `self_repo_prefix`'s reading: one traversal-exclusion set, so one copy of the walk.
// spec: context-kit/SPEC.md §Index-first reading — the targets, spelled as given because the
// empty-case message names them back.
pub fn targets(args: &[String]) -> Result<Vec<String>, String> {
    if args.is_empty() {
        return Ok(vec![match crate::walk::toplevel_opt()? {
            Some(t) => t,
            None => crate::walk::cwd()?,
        }]);
    }
    Ok(args.to_vec())
}

// spec: context-kit/SPEC.md §Index-first reading — `find <targets> -name <glob> -not -path
// "*/<prune>/*"`, in that section's traversal and order, down to what a target that is neither a
// file nor a directory contributes.
pub fn corpus(targets: &[String], globs: &[&str]) -> Result<Vec<String>, String> {
    let prune = crate::walk::knob_array("CONTEXT_KIT_PRUNE_DIRS")?;
    let mut out: Vec<String> = Vec::new();
    for t in targets {
        let path = std::path::Path::new(t);
        if path.is_dir() {
            for p in crate::walk::find_link_entries_with_prune(path, &|n| prune.iter().any(|d| d == n))?
            {
                out.push(p.display().to_string());
            }
        } else if path.is_file() {
            out.push(t.clone());
        }
    }
    out.retain(|p| {
        let base = p.rsplit('/').next().unwrap_or(p);
        globs.iter().any(|g| crate::walk::pattern_match(g, base))
            && !crate::walk::path_pruned(p, &prune)
    });
    out.sort_unstable();
    Ok(out)
}

// spec: context-kit/SPEC.md §Index-first reading — the path a block is headed by is repo-relative
// where the walk stayed inside the repository and the walked spelling otherwise, the shell form's
// prefix strip; the toplevel is resolved once per run rather than per file.
pub fn relative(root: &Option<String>, path: &str) -> String {
    match root {
        Some(r) => path
            .strip_prefix(&format!("{}/", r))
            .unwrap_or(path)
            .to_string(),
        None => path.to_string(),
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — an arm's own read of one file, refusing with the path
// rather than through a gate's fail-closed diagnostic: this class returns a document, not a verdict
pub fn read_text(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the arm's own argv tail, so a projection whose
// generator has a write-in-place mode takes it as a flag rather than needing a second arm. The
// returned string is what the arm prints: the document, or the action a write performed.
pub type EmitFn = fn(&[String]) -> Result<String, String>;

// spec: gate-sdk/SPEC.md §The non-gate arm — a bridged arm either renders a document or returns an
// exit code; the class the table keys is *bridged*, not *emitting*. The variant is a return shape
// and nothing else — no declared-knob union keys on it.
pub enum Arm {
    Emit(EmitFn),
    Run(fn(&[String]) -> i32),
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the registry union sentinel, owned beside `knobs`
// rather than beside the first member that spelled it: the expansion is the mechanism's.
pub const EVERY_REGISTERED_KNOB: &str = "@every-registered-knob";

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
    // spec: queue-kit/SPEC.md §The queue-counts arm — the four knobs `Sections::active_and_deferred`
    // resolves, plus the queue file. `QUEUE_KIT_DONE_SECTION` deliberately not: Done is not a task
    // section, and the arm must not acquire a read it does not make.
    (
        "--emit-queue-counts",
        Arm::Emit(queue_counts::emit),
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
    ),
    // spec: queue-kit/SPEC.md §The queue-edges arm — the same section vocabulary read over entry
    // *bodies* rather than lead lines: a different question over the same file, so the two stay
    // two arms rather than one with a fourth mode.
    (
        "--emit-queue-edges",
        Arm::Emit(queue_edges::emit),
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
    ),
    // spec: context-kit/SPEC.md §Index-first reading — the markdown structural index. A table
    // member rather than a hardcoded flag because it resolves a consumer knob, and a hardcoded flag
    // receives no consumer override at all.
    (
        "--emit-md-index",
        Arm::Emit(md_index::emit),
        md_index::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §The non-gate arm — the class's first member whose row exists for
    // reachability rather than for configuration, its declared roster being empty.
    (
        "--emit-md-section",
        Arm::Emit(md_section::emit),
        md_section::KNOBS,
    ),
    // spec: context-kit/SPEC.md §Index-first reading — the public-surface dispatcher: the extractor
    // seam survives the port, so the two knobs that resolve it cross the bridge beside the prune set.
    (
        "--emit-pub-index",
        Arm::Emit(pub_index::emit),
        pub_index::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §port-blockers — the port oracle: three arms over two corpora, whose
    // `--tree` owed count is the port track's completion predicate. A table member rather than a
    // hardcoded flag because it reads five structural knobs and an arbitrary sixth.
    (
        "--emit-port-blockers",
        Arm::Emit(port_blockers::emit),
        port_blockers::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The report skeleton — the collator, a *bridged* arm rather than a
    // top-level flag: a hardcoded flag receives no consumer override, which for a kit whose whole
    // surface is overridable knobs would be a functional regression rather than a porting detail.
    (
        "--emit-drift-report",
        Arm::Emit(drift_report::emit),
        drift_report::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §The survey record — the capture affordance, whose free-text
    // argv keeps the shape refusal and the `--` escape across the port while its help arm retires
    // to the front-end: the hazard belongs to the argument and the help belongs to the substrate.
    (
        "--emit-file-survey",
        Arm::Emit(file_survey::emit),
        file_survey::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §The survey record — the citation affordance: it derives no stage
    // and stamps no rev, so its sibling's state-file knob is deliberately off this roster.
    (
        "--emit-cite-survey",
        Arm::Emit(cite_survey::emit),
        cite_survey::KNOBS,
    ),
    // spec: doctrine-kit/SPEC.md §stage-rules — an `Arm::Emit` because the contract is a document
    // and both its failures are already exit 2, which is the variant's own collapse; reached
    // through the generic `--emit <name>` composer rather than a front-end branch of its own
    (
        "--emit-stage-rules",
        Arm::Emit(stage_rules::emit),
        stage_rules::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — an empty-roster member whose roster must
    // stay empty rather than merely happening to be: neither name it reads is defined in
    // lifecycle-kit's `lib/stages.sh`, so a declared row would fail-close through the bridge
    (
        "--emit-session-id",
        Arm::Emit(session_id::emit),
        session_id::KNOBS,
    ),
    // spec: context-kit/SPEC.md §bin/env-probe — an action that reports, so an `Arm::Emit`: both
    // its failures are already exit 2, which is the variant's own collapse. Its one declared knob
    // is resolved out of `lib/context.sh`, the config bridge's sole resolver for that family.
    (
        "--emit-env-probe",
        Arm::Emit(env_probe::emit),
        env_probe::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §run-gates — the battery runner: the class's first bridged member
    // that returns a verdict rather than a document, and the reason the table is keyed by flag
    ("--run", Arm::Run(crate::runner::run), crate::runner::KNOBS),
    // spec: gate-sdk/SPEC.md §The non-gate arm — the one dispatching harness-integration arm; its
    // roster is a sentinel because the answer is one member's knobs, scoped by its own argv
    (
        "--hook",
        Arm::Run(crate::hook::run),
        &[crate::hook::EVERY_HOOK_KNOB],
    ),
    // spec: gate-sdk/SPEC.md §The non-gate arm — the two harness-integration arms that are not
    // members of `--hook`: neither speaks the hook protocol, so each names its own caller and
    // carries its own fixed roster rather than folding into an arm whose contract it would void
    (
        "--statusline",
        Arm::Run(crate::hook::statusline::run),
        crate::hook::statusline::KNOBS,
    ),
    (
        "--usage-poll",
        Arm::Run(crate::hook::poll::run),
        crate::hook::poll::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §upgrade-smoke — the two-phase upgrade proof: an `Arm::Run` because
    // its contract is the 1-versus-2 split of its exit status, which an emitting arm collapses, and
    // a table member because it resolves six knobs a hardcoded flag would silently ignore
    (
        "--upgrade-smoke",
        Arm::Run(upgrade_smoke::run),
        upgrade_smoke::KNOBS,
    ),
    // spec: queue-kit/SPEC.md §The lesson-sink arm — an `Arm::Run` and not an `--emit-` member:
    // its contract is the sink's exit status, which an emitting arm cannot carry. Its caller is a
    // stage step, so it is not a harness-integration arm either
    (
        "--lesson-sink",
        Arm::Run(lesson_sink::run),
        lesson_sink::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — an `Arm::Run` because the member
    // mutates two files and a git config key and emits no document; the `--install <op>` family is
    // refused with cause there, its unbridged contract being unable to resolve these eight knobs
    (
        "--install-lifecycle",
        Arm::Run(install_lifecycle::run),
        install_lifecycle::KNOBS,
    ),
];

pub fn lookup(arm: &str) -> Option<&'static Arm> {
    BRIDGED_ARMS
        .iter()
        .find(|(a, _, _)| *a == arm)
        .map(|(_, f, _)| f)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — an arm's declared reads are its own roster with every
// sentinel in it expanded, derived rather than maintained. One expansion for every member: the
// `Arm` variant is not consulted at all.
pub fn knobs(arm: &str, rest: &[String]) -> Option<Vec<&'static str>> {
    let (_, _, own) = BRIDGED_ARMS.iter().find(|(a, _, _)| *a == arm)?;
    Some(expand(own, rest))
}

// spec: gate-sdk/SPEC.md §The non-gate arm — an arm carrying no sentinel keeps exactly its own
// roster, argv for argv; one carrying either is bridged that sentinel's expansion instead, and the
// result is sorted and deduped because two sentinels can name one knob.
fn expand(own: &'static [&'static str], rest: &[String]) -> Vec<&'static str> {
    if !own.iter().any(|k| is_sentinel(k)) {
        return own.to_vec();
    }
    let mut all: Vec<&'static str> = own.iter().copied().filter(|k| !is_sentinel(k)).collect();
    if own.contains(&EVERY_REGISTERED_KNOB) {
        for name in registered_members(rest) {
            if let Some(k) = crate::gates::knobs(&name) {
                all.extend_from_slice(k);
            }
        }
    }
    if own.contains(&crate::hook::EVERY_HOOK_KNOB) {
        all.extend_from_slice(&hook_knobs(rest));
    }
    all.sort_unstable();
    all.dedup();
    all
}

fn is_sentinel(knob: &str) -> bool {
    knob == EVERY_REGISTERED_KNOB || knob == crate::hook::EVERY_HOOK_KNOB
}

// spec: gate-sdk/SPEC.md §The non-gate arm — `--knobs --hook <member>` answers that member's own
// roster and `--knobs --hook` with no member the union over the table: the per-member answer is
// what makes the bridge resolve one guard's configuration rather than six.
fn hook_knobs(rest: &[String]) -> Vec<&'static str> {
    match rest.first().and_then(|m| crate::hook::knobs(m)) {
        Some(k) => k.to_vec(),
        None => crate::hook::HOOKS
            .iter()
            .flat_map(|(_, _, k)| k.iter().copied())
            .collect(),
    }
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the member set the registry sentinel scopes to, and
// the crate's one implementation of that scan: the scope arrives as argv, so `--gates-dir` is read
// out of whatever grammar the arm carries and its absence under-reports rather than fails open.
fn registered_members(args: &[String]) -> Vec<String> {
    let Some(dir) = args
        .iter()
        .position(|a| a == "--gates-dir")
        .and_then(|i| args.get(i + 1))
    else {
        return Vec::new();
    };
    match std::fs::read_to_string(crate::registry::list_path(dir)) {
        Ok(t) => crate::registry::members(&t),
        Err(_) => Vec::new(),
    }
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
        assert_eq!(
            knobs("--emit-footprint", &[]),
            Some(vec!["CONTEXT_KIT_SURFACES"])
        );
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the dispatching arm's roster is derived and
    // registry-scoped: it carries its own knobs always, and a member's knobs only where that
    // member is registered in the tree the caller named
    #[test]
    fn the_dispatching_arms_roster_is_scoped_to_what_the_tree_registers() {
        let bare = knobs("--run", &[]).expect("--run is not in the bridged-arm table");
        assert!(bare.contains(&"GATE_SDK_TMP_DIR"), "the arm's own knob is missing");
        assert!(
            !bare.contains(&"CANON_KIT_SPEC_NAME"),
            "an unregistered member's knob rode an unscoped union"
        );
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let here = vec![
            "--gates-dir".to_string(),
            root.join("scripts").display().to_string(),
        ];
        let scoped = knobs("--run", &here).expect("--run is not in the bridged-arm table");
        assert!(
            scoped.contains(&"CANON_KIT_SPEC_NAME"),
            "a registered member's knob is missing from the scoped union"
        );
        let mut sorted = scoped.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(scoped, sorted, "the union is not deterministic");
    }
}
