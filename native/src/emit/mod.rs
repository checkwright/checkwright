// spec: gate-sdk/SPEC.md §The non-gate arm — the ported arms. Each owes no descriptor, no
// registration and no fixture pair; it owes a test module in its own file, and a named caller: a
// regen command, a comparator calling `emit()`, a stage step, a gate reaching it in process.
use crate::programs;
pub mod agents_md_smoke;
pub mod always_loaded;
pub mod capture_drain;
pub mod cite_survey;
pub mod close_surfaces;
pub mod compare_settings_allow;
pub mod csmoke;
pub mod demo;
pub mod diff_baseline;
pub mod docs_mirror;
pub mod drift_report;
pub mod enforcement_map;
pub mod enter_stage;
pub mod entry_history;
pub mod enum_sets;
pub mod env_probe;
pub mod file_gap;
pub mod file_install;
pub mod file_survey;
pub mod footprint;
pub mod front_end_parity;
pub mod git_hooks;
pub mod graph;
pub mod install_evidence;
pub mod install_hooks;
pub mod install_lifecycle;
pub mod kfric;
pub mod kpi;
pub mod lesson_sink;
pub mod md_index;
pub mod md_section;
pub mod md_unwrap;
pub mod parse_gates_log;
pub mod overhead_meter;
pub mod pack_installer;
pub mod parse_smoke_log;
pub mod projection_witness;
pub mod port_blockers;
pub mod pub_index;
pub mod pub_lang;
pub mod queue_counts;
pub mod queue_edges;
pub mod queue_flow;
pub mod queue_index;
pub mod queue_migrate;
pub mod reads_census;
pub mod rewrite;
pub mod scan_prompts;
pub mod stage_economics;
pub mod stage_rules;
pub mod roadmap;
pub mod ruling_staleness;
pub mod run_consumer_smoke;
pub mod run_gate_tests;
pub mod run_guard_tests;
pub mod run_index_tests;
pub mod run_validate;
pub mod scratch_run;
pub mod session_id;
pub mod trajectory;
pub mod upgrade_smoke;
pub mod usage_trend;
pub mod value_rollup;
pub mod wait_probe;

// spec: gate-sdk/SPEC.md §lib/gate.sh — gate_self_repo_prefix, degrading to nothing on no origin
// or an unrecognised form. It sits on the family because two arms render self-repo links, and a
// second copy of the normalisation is a second identity to disagree about.
pub fn self_repo_prefix(reference: &str) -> String {
    let origin = match crate::proc::run(&programs::GIT, &["remote", "get-url", "origin"]) {
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
// spec: gate-sdk/SPEC.md §The bin/-tool contract — the paths are free text, so a dash-led one is
// a shape refusal carrying the calling member's usage.
pub fn targets(args: &[String], usage: &str) -> Result<Vec<String>, String> {
    let args = file_survey::positionals(args, "path").map_err(|e| format!("{}\n{}", e, usage))?;
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
        Some(r) => crate::walk::rel_under(r, path).unwrap_or(path).to_string(),
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

// spec: gate-sdk/SPEC.md §The non-gate arm — a non-gate arm either renders a document or returns an
// exit code; the class the table keys is *non-gate*, not *emitting*. The variant is a return shape,
// and an emitting one carries its argument grammar — no declared-knob union keys on either.
pub enum Arm {
    Emit(EmitFn, Grammar),
    Run(fn(&[String]) -> i32),
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — an `--emit-` member's argument grammar: `Flags`
// names every token the member reads, none for one taking no argument; `Parsed` is a member parsing
// its own argv, with the usage block its shape refusal prints.
pub enum Grammar {
    Flags(&'static [&'static str]),
    Parsed(&'static str),
}

impl Grammar {
    pub fn usage(&self, arm: &str) -> String {
        let name = arm.strip_prefix("--emit-").unwrap_or(arm);
        match self {
            Grammar::Parsed(block) => block.to_string(),
            Grammar::Flags([]) => format!("usage: --emit {}   (it takes no argument)", name),
            Grammar::Flags(flags) => {
                let opts: Vec<String> = flags.iter().map(|f| format!("[{}]", f)).collect();
                format!("usage: --emit {} {}", name, opts.join(" "))
            }
        }
    }
}

// spec: gate-sdk/SPEC.md §The bin/-tool contract — a token a `Flags` grammar does not name is
// refused before the member runs; a `Parsed` member's refusal prints as the member spelled it.
pub fn dispatch(arm: &str, f: EmitFn, grammar: &Grammar, args: &[String]) -> Result<String, String> {
    if let Grammar::Flags(flags) = grammar {
        if let Some(a) = args.iter().find(|a| !flags.contains(&a.as_str())) {
            return Err(format!("{}: unrecognized argument: {}\n{}", arm, a, grammar.usage(arm)));
        }
    }
    f(args).map_err(|e| format!("{}: {}", arm, e))
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the registry union sentinel, owned beside `knobs`
// rather than beside the first member that spelled it: the expansion is the mechanism's.
pub const EVERY_REGISTERED_KNOB: &str = "@every-registered-knob";

// spec: gate-sdk/SPEC.md §The non-gate arm — the **arm table**, keyed by the arm's own flag spelling,
// each row carrying the arm's declared knob roster. `--emit-` is one arm family's spelling, not the
// table's name.
pub const ARMS: &[(&str, Arm, &[&str])] = &[
    (
        "--emit-footprint",
        Arm::Emit(footprint::emit, Grammar::Flags(&[])),
        &["CONTEXT_KIT_SURFACES"],
    ),
    // spec: lifecycle-kit/SPEC.md §The close-surfaces emit arm — the class's first member that is
    // not a stored projection: the roster's value is that it is recomputed at the moment close
    // reads it, so there is no comparator and must not be.
    (
        "--emit-close-surfaces",
        Arm::Emit(close_surfaces::emit, Grammar::Parsed(close_surfaces::USAGE)),
        &[
            "GATE_SDK_KIT_DIRS",
            "GATE_SDK_PRUNE_DIRS",
            "GATE_SDK_PRUNE_EXTRA_DIRS",
            "LIFECYCLE_KIT_ROSTER_BASENAME",
            "LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS",
            "GATE_SDK_WORKFLOW_DIR",
        ],
    ),
    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — a reporting member whose callers
    // are the closing stage's repair step and a session running it by hand before relying on a
    // ruling; nothing stores its two reports, for the roster arm's reason above.
    (
        "--emit-ruling-staleness",
        Arm::Emit(ruling_staleness::emit, Grammar::Parsed(ruling_staleness::USAGE)),
        ruling_staleness::KNOBS,
    ),
    (
        "--emit-enforcement-map",
        Arm::Emit(enforcement_map::emit, Grammar::Flags(&[])),
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_ENFORCE_SCAN_DIR",
            "GATE_SDK_KIT_DIRS",
            "GATE_SDK_PRUNE_DIRS",
            "GATE_SDK_PRUNE_EXTRA_DIRS",
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
        Arm::Emit(value_rollup::emit, Grammar::Flags(&["--write"])),
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_ENFORCE_SCAN_DIR",
            "GATE_SDK_KIT_DIRS",
            "GATE_SDK_PRUNE_DIRS",
            "GATE_SDK_PRUNE_EXTRA_DIRS",
            "DRIFT_KIT_KPIS_FILE",
            "CONTEXT_KIT_SETTINGS_FILE",
            "CANON_KIT_DOCS_BLOB_REF",
            "EVIDENCE_KIT_SUITES",
            "EVIDENCE_KIT_RUN_*",
            "CONTEXT_KIT_SURFACES",
        ],
    ),
    // spec: canon-kit/SPEC.md §The reference-link grammar — the source set is derived from the
    // tracked tree rather than enumerated, so the configured values are the blob ref and the
    // mirror root
    (
        "--emit-docs-mirror",
        Arm::Emit(docs_mirror::emit, Grammar::Parsed(docs_mirror::USAGE)),
        &["CANON_KIT_DOCS_BLOB_REF", "CANON_KIT_MIRROR_ROOT"],
    ),
    // spec: drift-kit/SPEC.md §The published-evidence extractor — the stage roster and the
    // evidence-surface pair are this consumer's vocabulary, so they are knobs; a
    // stage name in the crate would ship one project's lifecycle as everyone's
    (
        "--emit-trajectory",
        Arm::Emit(trajectory::emit, Grammar::Flags(&["--human"])),
        &[
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
        Arm::Emit(roadmap::emit, Grammar::Parsed(roadmap::USAGE)),
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
    // spec: gate-sdk/SPEC.md §check-graph — the theme is configured as a *path* rather than as
    // content: a knob value cannot carry a newline and a stylesheet is newline-bearing by
    // construction. Values are knobs; documents are paths.
    (
        "--emit-graph",
        Arm::Emit(graph::emit, Grammar::Flags(&[])),
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_KIT_DIRS",
            "GATE_SDK_GRAPH_ARTIFACT",
            "GATE_SDK_GRAPH_THEME_DIR",
            "GATE_SDK_GRAPH_MAX_EDGES",
            "GATE_SDK_GRAPH_VOCAB",
            // spec: gate-sdk/SPEC.md §lib/gate.sh — this arm expands every member's `couples=`, so
            // it needs the knobs those tokens name rather than only its own: the sentinel stands
            // for them, read off the descriptor corpus.
            crate::registry::EVERY_COUPLES_KNOB,
        ],
    ),
    // spec: gate-sdk/SPEC.md §gen-pre-commit — an `Arm::Emit` whose operand names the hook, and
    // whose `--write` operand writes both, on `--emit-docs-mirror`'s precedent
    (
        "--emit-git-hooks",
        Arm::Emit(git_hooks::emit, Grammar::Parsed(git_hooks::USAGE)),
        git_hooks::KNOBS,
    ),
    // spec: queue-kit/SPEC.md §The queue-index arm — the class's first *query* member as well as a
    // generator, and configured: a hardcoded flag would hide its reads from the knob-file derivation
    (
        "--emit-queue-index",
        Arm::Emit(queue_index::emit, Grammar::Parsed(queue_index::USAGE)),
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
            "QUEUE_KIT_ATTEND_CAP",
            "QUEUE_KIT_ICEBOX_AGE_DAYS",
            "QUEUE_KIT_ENTRY_CAP",
        ],
    ),
    // spec: queue-kit/SPEC.md §check-queue-entry-budget — an `Arm::Emit` on the family's own test:
    // the product is a report and the exit contract is two-valued, 0 with the report and 2 on a
    // usage error, the absence of a 1 being the member's whole point
    (
        "--emit-entry-history",
        Arm::Emit(entry_history::emit, Grammar::Parsed(entry_history::USAGE)),
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ENTRY_CAP",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
    ),
    // spec: queue-kit/SPEC.md §The queue-counts arm — the four knobs `Sections::active_and_deferred`
    // resolves, plus the queue file. `QUEUE_KIT_DONE_SECTION` deliberately not: Done is not a task
    // section, and the arm must not acquire a read it does not make.
    (
        "--emit-queue-counts",
        Arm::Emit(queue_counts::emit, Grammar::Parsed(queue_counts::USAGE)),
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
        Arm::Emit(queue_edges::emit, Grammar::Parsed(queue_edges::USAGE)),
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
    ),
    // spec: queue-kit/SPEC.md §The queue-migrate arm — the grammar migration a consumer upgrading
    // queue-kit runs once, configured by the section vocabulary it rewrites within
    (
        "--emit-queue-migrate",
        Arm::Emit(queue_migrate::emit, Grammar::Parsed(queue_migrate::USAGE)),
        queue_migrate::KNOBS,
    ),
    // spec: context-kit/SPEC.md §Index-first reading — the markdown structural index. A table
    // member rather than a hardcoded flag because it reads a consumer knob, which a hardcoded flag
    // would hide from the knob-file derivation.
    (
        "--emit-md-index",
        Arm::Emit(md_index::emit, Grammar::Parsed(md_index::USAGE)),
        md_index::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §The non-gate arm — the class's first member whose row exists for
    // reachability rather than for configuration, its declared roster being empty.
    (
        "--emit-md-section",
        Arm::Emit(md_section::emit, Grammar::Parsed(md_section::USAGE)),
        md_section::KNOBS,
    ),
    // spec: canon-kit/SPEC.md §check-md-unwrapped — the gate's remedy, reading no knob: its
    // operands are the files, and the scanner is the gate's own
    (
        "--emit-md-unwrap",
        Arm::Emit(md_unwrap::emit, Grammar::Parsed(md_unwrap::USAGE)),
        md_unwrap::KNOBS,
    ),
    // spec: context-kit/SPEC.md §Index-first reading — the public-surface dispatcher: the extractor
    // seam survives the port, so the two knobs that resolve it are declared beside the prune set.
    (
        "--emit-pub-index",
        Arm::Emit(pub_index::emit, Grammar::Parsed(pub_index::USAGE)),
        pub_index::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §port-blockers — the port oracle: three arms over two corpora, whose
    // `--tree` owed count is the port track's completion predicate. A table member rather than a
    // hardcoded flag because it reads five structural knobs and an arbitrary sixth.
    (
        "--emit-port-blockers",
        Arm::Emit(port_blockers::emit, Grammar::Parsed(port_blockers::USAGE)),
        port_blockers::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — the `?` census, a projection over registry
    // field 2. It reads no knob and is a table member anyway: the family the table keys is what
    // `no_such_arm` prints, so an arm outside it is one a mistyped `--emit` cannot be steered to.
    (
        "--emit-reads-census",
        Arm::Emit(reads_census::emit, Grammar::Flags(&[])),
        reads_census::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The report skeleton — the collator, a table member rather than a
    // top-level flag: it reads its own knobs and its members' guard-kit knobs.
    (
        "--emit-drift-report",
        Arm::Emit(drift_report::emit, Grammar::Flags(&["--trend"])),
        drift_report::KNOBS,
    ),
    // spec: guard-kit/SPEC.md §scan-prompts — the ranker, a table member on the forced-family
    // test rather than by resemblance: it reads three consumer knobs a hardcoded top-level
    // flag would hide from the knob-file derivation, and its free-text positional keeps the shape refusal.
    (
        "--emit-scan-prompts",
        Arm::Emit(scan_prompts::emit, Grammar::Parsed(scan_prompts::USAGE)),
        scan_prompts::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §The workflow directory — the rotation drain every close triage runs
    // before reading a capture log, a table member so the front end's one `--emit` grant covers it
    (
        "--emit-capture-drain",
        Arm::Emit(capture_drain::emit, Grammar::Parsed(capture_drain::USAGE)),
        capture_drain::KNOBS,
    ),
    // spec: guard-kit/SPEC.md §compare-settings-allow — the settings-allow advisory, a table
    // member on the forced-family test: its four knobs are consumer configuration.
    (
        "--emit-compare-settings-allow",
        Arm::Emit(compare_settings_allow::emit, Grammar::Parsed(compare_settings_allow::USAGE)),
        compare_settings_allow::KNOBS,
    ),
    // spec: context-kit/SPEC.md §The always-loaded meter — the context meter, a table member
    // because it reads four consumer knobs a hardcoded flag would hide from the knob-file derivation;
    // its three modes arrive as operands, the shape `--hook` and `--wait-probe` already carry.
    (
        "--emit-always-loaded",
        Arm::Emit(always_loaded::emit, Grammar::Parsed(always_loaded::USAGE)),
        always_loaded::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §The survey record — the capture affordance, whose free-text
    // argv keeps the shape refusal and the `--` escape across the port while its help arm retires
    // to the front-end: the hazard belongs to the argument and the help belongs to the substrate.
    (
        "--emit-file-survey",
        Arm::Emit(file_survey::emit, Grammar::Parsed(file_survey::USAGE)),
        file_survey::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — the mid-iteration capture affordance,
    // riding the same argv-shape split as `--emit-file-survey`: the refusal and the `--` escape
    // cross the port with the argument, the help arm retires to the front-end.
    (
        "--emit-file-gap",
        Arm::Emit(file_gap::emit, Grammar::Parsed(file_gap::USAGE)),
        file_gap::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The knowledge-friction loop — the capture affordance, riding the same
    // argv-shape split as `--emit-file-survey`: the refusal and the `--` escape cross the port with
    // the argument, the help arm retires to the front-end.
    (
        "--emit-kfric",
        Arm::Emit(kfric::emit, Grammar::Parsed(kfric::USAGE)),
        kfric::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The install-observation record — the observation-capture affordance,
    // riding the same argv-shape split as `--emit-kfric`: the refusal and the `--` escape cross the
    // port with the argument, the help arm retires to the front-end.
    (
        "--emit-file-install",
        Arm::Emit(file_install::emit, Grammar::Parsed(file_install::USAGE)),
        file_install::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The install-evidence projection — the public half of the same
    // channel, an aggregate-only projection a consumer's freshness gate byte-compares; it declares
    // the gates file because it classifies a red's gate name against that roster.
    (
        "--emit-install-evidence",
        Arm::Emit(install_evidence::emit, Grammar::Flags(&["--human"])),
        install_evidence::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The overhead meter — an `Arm::Emit` on the variant's own test: the
    // meter is advisory and exit is always 0, so no `1` is load-bearing and the `{0, 2}` collapse
    // costs nothing. `--emit-kfric`'s shape — a document returned, a line appended beside it.
    (
        "--emit-overhead-meter",
        Arm::Emit(overhead_meter::emit, Grammar::Parsed(overhead_meter::USAGE)),
        overhead_meter::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The stage-economics meter — an `Arm::Emit` on the variant's own
    // test: the meter is advisory and exit is always 0, so no `1` is load-bearing. Its roster is
    // the meter's own seven knobs, every one of them a row of drift-kit's table.
    (
        "--emit-stage-economics",
        Arm::Emit(stage_economics::emit, Grammar::Flags(&[])),
        stage_economics::KNOBS,
    ),
    // spec: drift-kit/SPEC.md §The queue-flow arm — an `Arm::Emit` because exit is always 0 bar a
    // usage error, so no `1` is load-bearing; its roster is the KPI's pool knobs plus the state
    // file whose history the windows are read from.
    (
        "--emit-queue-flow",
        Arm::Emit(queue_flow::emit, Grammar::Parsed(queue_flow::USAGE)),
        queue_flow::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §The survey record — the citation affordance: it derives no stage
    // and stamps no rev, so its sibling's state-file knob is deliberately off this roster.
    (
        "--emit-cite-survey",
        Arm::Emit(cite_survey::emit, Grammar::Parsed(cite_survey::USAGE)),
        cite_survey::KNOBS,
    ),
    // spec: doctrine-kit/SPEC.md §stage-rules — an `Arm::Emit` because the contract is a document
    // and both its failures are already exit 2, which is the variant's own collapse; reached
    // through the generic `--emit <name>` composer rather than a front-end branch of its own
    (
        "--emit-stage-rules",
        Arm::Emit(stage_rules::emit, Grammar::Parsed(stage_rules::USAGE)),
        stage_rules::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §bin/session-id.sh — an empty-roster member whose roster must
    // stay empty rather than merely happening to be: neither name it reads is a row in
    // lifecycle-kit's static table, so a declared row would be an undeclared-name refusal
    (
        "--emit-session-id",
        Arm::Emit(session_id::emit, Grammar::Flags(&[])),
        session_id::KNOBS,
    ),
    // spec: canon-kit/SPEC.md §check-prose-enum — the bundled enum-set emitter, an `Arm::Emit`:
    // the contract is a document and every failure is already exit 2 spec: gate-sdk/SPEC.md §The
    // non-gate arm — a **two-kit** declared roster
    (
        "--emit-enum-sets",
        Arm::Emit(enum_sets::emit, Grammar::Flags(&[])),
        enum_sets::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §The non-gate arm — the static knob table published, an `Arm::Emit`
    // whose roster is empty by construction: the arm takes no knob and reads no file
    ("--emit-knob-roster", Arm::Emit(crate::knobs::emit, Grammar::Flags(&[])), &[]),
    // spec: gate-sdk/SPEC.md §The non-gate arm — the resolved static values, the one producer the
    // shell couples expander and every shell knob read take; its roster is its argv's closure
    (
        "--emit-knob-values",
        Arm::Emit(crate::knobs::values, Grammar::Parsed(crate::knobs::VALUES_USAGE)),
        &[crate::knobs::ARGV_STATIC_KNOBS],
    ),
    // spec: gate-sdk/SPEC.md §lib/gate.sh — the fixture-suite roster the workflows loop over
    (
        "--emit-fixture-suites",
        Arm::Emit(crate::registry::emit_fixture_suites, Grammar::Flags(&[])),
        &["GATE_SDK_KIT_DIRS", "GATE_SDK_GATES_DIR"],
    ),
    // spec: gate-sdk/SPEC.md §The non-gate arm — the kit roots, derived from the gate-sdk root
    // locator, for the shell callers that still need the set
    (
        "--emit-kit-roots",
        Arm::Emit(crate::walk::emit_kit_roots, Grammar::Flags(&[])),
        &["GATE_SDK_KIT_DIRS"],
    ),
    // spec: evidence-kit/SPEC.md §Layout and configuration — the two parser adapters, reached as
    // the *value* of `EVIDENCE_KIT_PARSER_<suite>` rather than as a named adapter spec: gate-
    // sdk/SPEC.md §The non-gate arm — both rosters are empty of the `--emit-md-section` kind, not
    (
        "--emit-parse-gates-log",
        Arm::Emit(parse_gates_log::emit, Grammar::Parsed(parse_gates_log::USAGE)),
        parse_gates_log::KNOBS,
    ),
    (
        "--emit-parse-smoke-log",
        Arm::Emit(parse_smoke_log::emit, Grammar::Parsed(parse_smoke_log::USAGE)),
        parse_smoke_log::KNOBS,
    ),
    // spec: context-kit/SPEC.md §bin/env-probe — an action that reports, so an `Arm::Emit`: both
    // its failures are already exit 2, which is the variant's own collapse. Its one declared knob
    // is a row of context-kit's table.
    (
        "--emit-env-probe",
        Arm::Emit(env_probe::emit, Grammar::Flags(&[])),
        env_probe::KNOBS,
    ),
    // spec: delegation-kit/SPEC.md §Trend reporter — an `Arm::Emit` settled by the *absence* of a
    // 1 in the declared contract, the exact inverse of `--usage-verdict` below; the `--emit-`
    // spelling rides with the family, so no new front-end `case` arm is owed.
    (
        "--emit-usage-trend",
        Arm::Emit(usage_trend::emit, Grammar::Parsed(usage_trend::USAGE)),
        usage_trend::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §run-gates — the battery runner: the class's first member
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
    // spec: delegation-kit/SPEC.md §usage-verdict — an `Arm::Run` because its three-state exit
    // status carries a 1 a hook grades, which an emitting arm collapses to {0, 2}; its callers are
    // a session brief, a kit smoke and a gate in process, so it is no harness-integration arm
    (
        "--usage-verdict",
        Arm::Run(crate::hook::verdict::run),
        crate::hook::verdict::KNOBS,
    ),
    // spec: guard-kit/SPEC.md §scratch-run — an `Arm::Run` on two independent grounds: the runner
    // passes the child's exit code through verbatim, and its stdout must reach the terminal as the
    // child produces it rather than as a string returned at the end.
    (
        "--scratch-run",
        Arm::Run(scratch_run::run),
        scratch_run::KNOBS,
    ),
    // spec: guard-kit/SPEC.md §rewrite — an `Arm::Run` because its contract is the 0/1/2 split an
    // emitting arm collapses, and a table member because it resolves `LIFECYCLE_KIT_STATE_FILE`
    (
        "--rewrite",
        Arm::Run(rewrite::run),
        rewrite::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §upgrade-smoke — the two-phase upgrade proof: an `Arm::Run` because
    // its contract is the 1-versus-2 split of its exit status, which an emitting arm collapses, and
    // a table member because it reads six knobs a hardcoded flag would hide from the knob-file derivation
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
    // refused with cause there, a hardcoded flag having no row to declare these eight knobs on
    (
        "--install-lifecycle",
        Arm::Run(install_lifecycle::run),
        install_lifecycle::KNOBS,
    ),
    // spec: doctrine-kit/SPEC.md §install-doctrine — an `Arm::Run` on the member above's own
    // precedent, and it reads two knobs the rowless `--install <op>` family could not declare. Its
    // callers are the adopter's install step, the kit's smoke, and `--init`, in-process.
    (
        "--install-doctrine",
        Arm::Run(crate::doctrine::run),
        crate::doctrine::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §install-hooks — an `Arm::Run` because `check-identity`'s 1 surfaces
    // through this member's own status; it also declines the `--install <op>` family, whose
    // rowless contract could declare neither of the two knobs this one reads from kit config
    (
        "--install-hooks",
        Arm::Run(install_hooks::run),
        install_hooks::KNOBS,
    ),
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — an `Arm::Run` because the exit contract
    // is three-state and every code is load-bearing: 0 a stamp or a reported no-op, 1 a refusal,
    // 2 a usage or configuration error, and the state machine reads that difference.
    (
        "--enter-stage",
        Arm::Run(enter_stage::run),
        enter_stage::KNOBS,
    ),
    // spec: delegation-kit/SPEC.md §bin/wait-probe — an `Arm::Run` because its exit contract is
    // three-state: `report` returns 1 on an empty evidence file, an honest empty reading an emitting
    // arm would rewrite to the misuse code. Its subcommand word is an operand on `--hook`'s precedent
    (
        "--wait-probe",
        Arm::Run(wait_probe::run),
        wait_probe::KNOBS,
    ),
    // spec: evidence-kit/SPEC.md §bin/run-validate.sh — an `Arm::Run` because the contract is
    // three-state and the difference is the member's whole product: the `--emit-` collapse makes
    // *a suite regressed* indistinguishable from *the run could not start*
    (
        "--run-validate",
        Arm::Run(run_validate::run),
        run_validate::KNOBS,
    ),
    // spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — an `Arm::Run` although it does emit
    // findings on stdout, which is why the ground is stated: the family is excluded by the exit
    // contract, 1 being "NEW failures" and 2 the misuse code, not by the document test
    (
        "--diff-baseline",
        Arm::Run(diff_baseline::run),
        diff_baseline::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §run-gate-tests — an `Arm::Run` because the contract is a three-valued
    // exit — 0 clean, 1 a logic failure, 2 a harness or fixture error — and a table member because
    // it reads three knobs a hardcoded flag would hide from the knob-file derivation
    (
        "--run-gate-tests",
        Arm::Run(run_gate_tests::run),
        run_gate_tests::KNOBS,
    ),
    // spec: guard-kit/SPEC.md §Testing — an `Arm::Run` because the contract is a three-valued exit
    // an emitting arm collapses, and a table member because the arm needs the vendored guard-kit
    // root, a read a hardcoded flag would hide from the knob-file derivation
    (
        "--run-guard-tests",
        Arm::Run(run_guard_tests::run),
        run_guard_tests::KNOBS,
    ),
    // spec: context-kit/SPEC.md §Testing — an `Arm::Run` because the contract is the three-valued
    // exit the `index_tests` validate suite reads, and a table member because it reaches two kit
    // roots, a read a hardcoded flag would hide from the knob-file derivation
    (
        "--run-index-tests",
        Arm::Run(run_index_tests::run),
        run_index_tests::KNOBS,
    ),
    // spec: context-kit/SPEC.md §Testing — an `Arm::Run` because the contract is the verdict, which
    // an emitting arm cannot carry, and a table member because the vendoring reads the consumer's
    // kit roots and the binary placement the consumer's own `GATE_SDK_NATIVE_BIN`
    (
        "--agents-md-smoke",
        Arm::Run(agents_md_smoke::run),
        agents_md_smoke::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §Consumer smoke — an `Arm::Run` because the contract is the 1-versus-2
    // split of its exit status, which an emitting arm collapses, and a table member because the
    // vendoring and the binary placement both read the consumer's own knobs
    ("--run-demo", Arm::Run(demo::run), demo::KNOBS),
    // spec: gate-sdk/SPEC.md §Consumer smoke — an `Arm::Run` because the contract is the 1-versus-2
    // split of its exit status, and a table member because the vendoring and the probes read the
    // consumer's kit roots and binary path
    (
        "--run-consumer-smoke",
        Arm::Run(run_consumer_smoke::run),
        run_consumer_smoke::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §projection-witness — an `Arm::Run` because the contract is the 0/1/2
    // split of clean, a gate red outside its trigger, and a witness that could not run
    (
        "--projection-witness",
        Arm::Run(projection_witness::run),
        projection_witness::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §Consumer payload — the payload assembler, an `Arm::Run` because its
    // product is a tarball plus a receipt rather than a document, and a table member because all
    // three of its inputs are knobs a hardcoded flag would hide from the knob-file derivation
    (
        "--pack-installer",
        Arm::Run(pack_installer::run),
        pack_installer::KNOBS,
    ),
    // spec: gate-sdk/SPEC.md §run-gates — the front-end twin's held comparison, an `Arm::Run`
    // because its contract is the 0/1/2 split of identical, divergent and could-not-run
    (
        "--run-front-end-parity",
        Arm::Run(front_end_parity::run),
        front_end_parity::KNOBS,
    ),
];

// spec: gate-sdk/SPEC.md §The harness-integration arm — the fail-open set, the arms a front-end
// stub exits 0 for when the binary is absent; this declaration is authoritative and each stub's
// declaration line is its copy, held to it by check-front-end-fail-open
pub const FAIL_OPEN_ARMS: &[&str] = &["--hook", "--statusline"];

// spec: gate-sdk/SPEC.md §The non-gate arm — the fence-safe arm set; the `--emit-` family and every
// registered gate join by derivation in `fence_safe`, so only the rest are spelled here
pub const FENCE_SAFE_ARMS: &[&str] = &["--help", "--list", "--run", "--run-gate-tests"];

pub fn fence_safe(arm: &str) -> bool {
    FENCE_SAFE_ARMS.contains(&arm)
        || emit_names().contains(&arm)
        || crate::gates::lookup(arm).is_some()
}

pub fn lookup(arm: &str) -> Option<&'static Arm> {
    ARMS
        .iter()
        .find(|(a, _, _)| *a == arm)
        .map(|(_, f, _)| f)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the `--emit-` family's roster, read off the table
// rather than maintained beside it; the filter is the family the `--emit <name>` operand composes,
// which that section's refusal paragraph is why.
pub fn emit_names() -> Vec<&'static str> {
    ARMS
        .iter()
        .map(|(a, _, _)| *a)
        .filter(|a| a.starts_with("--emit-"))
        .collect()
}

pub fn arms() -> Vec<&'static str> {
    ARMS.iter().map(|(a, _, _)| *a).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The non-gate arm — the table is keyed by the arm's own flag, so a
    // near-miss spelling resolves to nothing rather than to a different member
    #[test]
    fn an_arm_resolves_only_under_its_own_flag() {
        assert!(lookup("--emit-enum-sets").is_some());
        assert!(lookup("enum-sets").is_none());
        assert!(lookup("--emit-enum-set").is_none());
    }

    // spec: gate-sdk/SPEC.md §The harness-integration arm — a renamed or deleted fail-open arm reds
    // here, before a stub's copy can name an arm the binary no longer dispatches
    // spec: gate-sdk/SPEC.md §The non-gate arm — the crate's network spawners, the arms reaching
    // `curl` and `npm`, are never fence-safe: admitting one reds here rather than in an adopter's
    // scratch
    #[test]
    fn no_network_spawning_arm_is_fence_safe() {
        const NETWORK_ARMS: &[&str] = &["--usage-poll", "--pack-installer"];
        for arm in NETWORK_ARMS {
            assert!(lookup(arm).is_some(), "{} names no arm-table row", arm);
            assert!(!fence_safe(arm), "{} spawns a network program and is fence-safe", arm);
        }
        for arm in FENCE_SAFE_ARMS {
            assert!(
                lookup(arm).is_some() || ["--help", "--list"].contains(arm),
                "{} is fence-safe and names no arm",
                arm
            );
        }
        assert!(fence_safe("--emit-knob-roster"));
        assert!(!fence_safe("--emit"));
    }

    fn never_runs(_: &[String]) -> Result<String, String> {
        panic!("the dispatcher ran a member whose argv it should have refused")
    }


    // spec: gate-sdk/SPEC.md §The bin/-tool contract — a surplus token is refused before the member
    // runs, and a declared flag is not
    #[test]
    fn a_flags_grammar_refuses_every_token_it_does_not_name() {
        let none = Grammar::Flags(&[]);
        let err = dispatch("--emit-x", never_runs, &none, &["bogus".to_string()]).expect_err("surplus");
        assert!(err.contains("unrecognized argument: bogus"), "{}", err);
        assert!(err.ends_with("usage: --emit x   (it takes no argument)"), "{}", err);
        let one = Grammar::Flags(&["--write"]);
        let err = dispatch("--emit-x", never_runs, &one, &["--write".into(), "-w".into()]).expect_err("-w");
        assert!(err.contains("unrecognized argument: -w\nusage: --emit x [--write]"), "{}", err);
        let ok = |_: &[String]| Ok("doc".to_string());
        assert_eq!(dispatch("--emit-x", ok, &one, &["--write".to_string()]), Ok("doc".to_string()));
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — a `Parsed` member's shape refusal carries the
    // usage block it spelled, once, and its runtime refusal prints its cause alone
    #[test]
    fn a_parsed_refusal_prints_as_the_member_spelled_it() {
        let g = Grammar::Parsed("usage: --emit x <arg>");
        let shape = |_: &[String]| Err("unrecognized option: --help\nusage: --emit x <arg>".to_string());
        let err = dispatch("--emit-x", shape, &g, &["--help".to_string()]).expect_err("refusal");
        assert_eq!(err, "--emit-x: unrecognized option: --help\nusage: --emit x <arg>");
        let runtime = |_: &[String]| Err("not a git repository".to_string());
        let err = dispatch("--emit-x", runtime, &g, &[]).expect_err("refusal");
        assert_eq!(err, "--emit-x: not a git repository");
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — every emitting row declares a usage block that
    // names its own member, so no row's refusal can print another member's grammar
    #[test]
    fn every_emitting_row_declares_its_own_usage() {
        for (arm, variant, _) in ARMS {
            if let Arm::Emit(_, g) = variant {
                let name = arm.strip_prefix("--emit-").expect("an Arm::Emit row spells --emit-<name>");
                let first = g.usage(arm).lines().next().unwrap_or_default().to_string();
                assert!(
                    first.starts_with("usage:") && first.contains(&format!("--emit {} ", name))
                        || first.ends_with(&format!("--emit {}", name)),
                    "{} declares a usage block naming another grammar: {}",
                    arm,
                    first
                );
            }
        }
    }

    // spec: gate-sdk/SPEC.md §The non-gate arm — the testing floor's census: each row's function
    // resolves to its file, and that file carries a test module, and so does each `--hook` member's.
    // Presence, never coverage.
    #[test]
    fn every_arm_table_row_resolves_to_a_file_with_a_test_module() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let src = std::fs::read_to_string(root.join("emit/mod.rs")).expect("the arm table's file reads");
        let start = src.find("pub const ARMS:").expect("the arm table's declaration");
        let end = start + src[start..].find("\n];").expect("the arm table's close");
        let table = &src[start..end];
        let mut rows = 0;
        let mut offending: Vec<String> = Vec::new();
        for token in ["Arm::Emit(", "Arm::Run("] {
            for (at, _) in table.match_indices(token) {
                rows += 1;
                let path: String = table[at + token.len()..]
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
                    .collect();
                let segs: Vec<&str> = path.split("::").collect();
                let module = match segs.split_last() {
                    Some((_, ["crate", rest @ ..])) => rest.join("/"),
                    Some((_, rest)) => format!("emit/{}", rest.join("/")),
                    None => String::new(),
                };
                let file = [format!("{}.rs", module), format!("{}/mod.rs", module)]
                    .into_iter()
                    .map(|f| root.join(f))
                    .find(|f| f.is_file());
                let tested = file.as_ref().and_then(|f| std::fs::read_to_string(f).ok()).is_some_and(|t| {
                    t.lines().any(|l| l.trim_start().starts_with("#[cfg(test)]"))
                });
                if !tested {
                    let shown = file.map_or_else(
                        || format!("{}.rs (absent)", module),
                        |f| f.strip_prefix(&root).unwrap_or(&f).display().to_string(),
                    );
                    offending.push(format!("{} -> {}", path, shown));
                }
            }
        }
        assert_eq!(rows, ARMS.len(), "the census read {} rows of the {} the table holds", rows, ARMS.len());
        let hooks = std::fs::read_to_string(root.join("hook/mod.rs")).expect("the member table's file reads");
        let start = hooks.find("pub const HOOKS:").expect("the member table's declaration");
        let end = start + hooks[start..].find("\n];").expect("the member table's close");
        let mut members = 0;
        for (at, _) in hooks[start..end].match_indices("::run") {
            members += 1;
            let module: String = hooks[start..start + at]
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            let file = root.join(format!("hook/{}.rs", module));
            let tested = std::fs::read_to_string(&file)
                .is_ok_and(|t| t.lines().any(|l| l.trim_start().starts_with("#[cfg(test)]")));
            if !tested {
                offending.push(format!("--hook {}::run -> hook/{}.rs", module, module));
            }
        }
        assert_eq!(
            members,
            crate::hook::HOOKS.len(),
            "the census read {} members of the {} the hook table holds",
            members,
            crate::hook::HOOKS.len()
        );
        assert!(
            offending.is_empty(),
            "arm-table rows and --hook members whose file carries no #[cfg(test)] module:\n  {}",
            offending.join("\n  ")
        );
    }

    #[test]
    fn every_fail_open_arm_is_an_arm_table_row() {
        for arm in FAIL_OPEN_ARMS {
            assert!(lookup(arm).is_some(), "{} names no arm-table row", arm);
        }
    }
}
