// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one module per ported gate; the
// subcommand name is the gate name, so no mapping table exists to drift
pub mod action_gh_repo;
pub mod action_pinning;
pub mod amendment_queue;
pub mod amendment_update_target;
pub mod agent_tier_explicit;
pub mod assertion_strength;
pub mod brevity;
pub mod close_surfaces;
pub mod comment_tier;
pub mod commit_subject;
pub mod doctrine_registration;
pub mod deprecation_task;
pub mod docs_cmd;
pub mod gate_assertions;
pub mod gate_exemption_tasks;
pub mod gate_fail_closed;
pub mod gate_binary_fresh;
pub mod gate_fixture_coverage;
pub mod gate_output;
pub mod gate_tamper;
pub mod graph;
pub mod kit_enum;
pub mod docs_cname_parity;
pub mod exec_bit;
pub mod core_files;
pub mod battery_roster;
pub mod commit_msg;
pub mod docs_link_convention;
pub mod docs_kit_parity;
pub mod docs_mirror_fresh;
pub mod docs_nav_reachable;
pub mod enforcement_fresh;
pub mod evidence_baseline;
pub mod evidence_manifest;
pub mod footprint_fresh;
pub mod install_toolchain;
pub mod installer_no_deps;
pub mod kit_ref_liveness;
pub mod npm_publish_spec;
pub mod release_channel_parity;
pub mod trajectory_fresh;
pub mod value_rollup_fresh;
pub mod gap_inbox_neutrality;
pub mod hook_exec_bit;
pub mod identity;
pub mod install_claim;
pub mod kit_registration;
pub mod knob_citation;
pub mod knob_default_coupling;
pub mod lesson_disposition;
pub mod lifecycle_registration;
pub mod manifest_count;
pub mod manifest_temporal;
pub mod md_refs;
pub mod measured_claim;
pub mod memory_off;
pub mod merge_attrs;
pub mod payload_claim;
pub mod prose_enum;
pub mod prose_tells;
pub mod queue_entry_budget;
pub mod queue_hygiene;
pub mod queue_prose_precondition;
pub mod queue_sections;
pub mod queue_slug_liveness;
pub mod queue_wrap;
pub mod readme_roster;
pub mod reads_couples;
pub mod release_bump;
pub mod roadmap_fresh;
pub mod root_tiering;
pub mod rule_citation;
pub mod scratch_citation;
pub mod settings_paths;
pub mod settings_pins;
pub mod settings_pins_parity;
pub mod shim_restatement;
pub mod skill_binding;
pub mod smoke_entry_guard;
pub mod spec_derivable_section;
pub mod spec_dod_singleton;
pub mod spec_embedded_source;
pub mod spec_fence_balance;
pub mod spec_pointer;
pub mod stage_entry;
pub mod stage_evidence;
pub mod stage_skill_coverage;
pub mod survey_record;
pub mod tag_lead_line;
pub mod task_conservation;
pub mod todo_task_liveness;
pub mod task_names;
pub mod template_copy_parity;
pub mod template_registry_parity;
pub mod test_hermetic;
pub mod tightened_gates_grammar;
pub mod tightened_gates_note_parity;
pub mod tracking_claim;
pub mod tree_terms;
pub mod unmarked_claim;
pub mod workflow_tiering;

pub type GateFn = fn(&[String]) -> i32;

// spec: gate-sdk/SPEC.md §check-reads-couples — the third element is the member's declared
// walk roots, the data `--reads` prints, each paired with the name of the knob whose value
// filters that root by basename — empty for an unfiltered root, un-omittable by construction.
// spec: gate-sdk/SPEC.md §lib/gate.sh — the fourth element is the member's declared knob
// reads, the data `--knobs` prints and the config bridge resolves. Un-omittable by the same
// construction, so no member can read a knob the bridge was never asked to carry.
// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the fifth element is the declaring
// root, `--list`'s second column: a kit's directory basename, or `-` where the consumer's own
// gates directory declares the member. Un-omittable, and held to the tree by the test below.
// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the sixth element is the member's external
// program requirements, the data `--needs` prints, in the third element's own two-field shape.
// Un-omittable by that construction, and held to executed behavior by unit test A below.
pub type GateEntry = (
    &'static str,
    GateFn,
    &'static [(&'static str, &'static str)],
    &'static [&'static str],
    &'static str,
    &'static [(&'static str, &'static str)],
);

pub const REGISTRY: &[GateEntry] = &[
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because each member's scan root is
    // its own first argument with a default, the same variable-first-argument shape the
    // shell parser calls undecidable and skips-and-counts.
    (
        "check-action-pinning",
        action_pinning::run,
        &[("?", "")],
        &["GATE_PRUNE_DIRS"],
        "gate-sdk",
        &[],
    ),
    (
        "check-action-gh-repo",
        action_gh_repo::run,
        &[("?", "")],
        &["GATE_PRUNE_DIRS"],
        "gate-sdk",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — the queue-kit cohort reads named files
    // rather than walking a tree, so each member declares an empty walk-root set: there is no
    // root for the recorder to observe, and unit test A holds that to executed behavior.
    (
        "check-queue-sections",
        queue_sections::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_REQUIRED_SECTIONS"],
        "queue-kit",
        &[],
    ),
    (
        "check-queue-wrap",
        queue_wrap::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_WRAP_BUDGET"],
        "queue-kit",
        &[],
    ),
    (
        "check-queue-hygiene",
        queue_hygiene::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_PROSE_LEADS"],
        "queue-kit",
        &[],
    ),
    (
        "check-queue-prose-precondition",
        queue_prose_precondition::run,
        &[],
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_PRECONDITION_REGEX",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
        "queue-kit",
        &[],
    ),
    // spec: queue-kit/SPEC.md §lib/queue.sh — a member reading a derived section matcher declares
    // every knob that matcher is computed from, since the Rust side derives them from the
    // bridged values exactly as lib/queue.sh derives its regexes
    (
        "check-tag-lead-line",
        tag_lead_line::run,
        &[],
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_LESSON_TAGS",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
        "queue-kit",
        &[],
    ),
    (
        "check-task-names",
        task_names::run,
        &[],
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
            "QUEUE_KIT_DONE_SECTION",
        ],
        "queue-kit",
        &[],
    ),
    (
        "check-queue-entry-budget",
        queue_entry_budget::run,
        &[],
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ENTRY_LINE_CAP",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
        "queue-kit",
        &[],
    ),
    // spec: queue-kit/SPEC.md §check-task-conservation — the HEAD side comes out of the git object store
    // rather than off the filesystem, so this member walks nothing and declares the same empty
    // set its file-reading siblings above declare
    (
        "check-task-conservation",
        task_conservation::run,
        &[],
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
            "QUEUE_KIT_DONE_SECTION",
        ],
        "queue-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §lib/spec.sh — the canon-kit cohort's members all derive their
    // corpus from `spec::manifest_files`, so each declares that derivation's whole knob set
    // beside its own: the bridge carries what the shared derivation reads, not what the
    // member's own rule reads.
    (
        "check-manifest-count",
        manifest_count::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_COUNT_COLLECTIONS",
            "CANON_KIT_COUNT_WEDGE_WORDS",
            "CANON_KIT_COUNT_ALLOWED_PHRASES",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-prose-enum — the vocabulary is a bridged *value*, two
    // index-aligned arrays because the wire format's own separator is the tab; the command
    // knob rides too, telling "none configured" from "configured, declared nothing"
    (
        "check-prose-enum",
        prose_enum::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_ENUM_SETS_CMD",
            "CANON_KIT_ENUM_SET_NAMES",
            "CANON_KIT_ENUM_SET_MEMBERS",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-measured-claim — born native, so it derives its corpus
    // from its own glob surface rather than from `spec::manifest_files`: the knob set is its
    // two knobs plus the two bridged arrays the emitter's roster crosses as
    (
        "check-measured-claim",
        measured_claim::run,
        &[("?", "")],
        &[
            "CANON_KIT_MEASURED_CLAIMS_CMD",
            "CANON_KIT_MEASURED_SURFACE_GLOBS",
            "CANON_KIT_MEASURED_KEYS",
            "CANON_KIT_MEASURED_VALUES",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The canonical-spec `spec_canonical_specs` cohort — two members
    // sharing one corpus derivation, so each declares that derivation's knob set beside its
    // own rule's knobs and nothing else
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because the scan root is the member's
    // own first argument with a `.` default, the variable-first-argument shape the shell
    // parser calls undecidable
    (
        "check-spec-dod-singleton",
        spec_dod_singleton::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_DOD_HEADING",
            "CANON_KIT_DOD_MODE",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    (
        "check-spec-derivable-section",
        spec_derivable_section::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_BANNED_HEADINGS",
            "CANON_KIT_DERIVABLE_DENSITY",
            "CANON_KIT_DERIVABLE_POINTER_REGEX",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the ERE cohort: the marker vocabulary
    // is a consumer ERE array, so this member declares it beside the shared corpus
    // derivation's own knob set and compiles every pattern through the engine
    (
        "check-manifest-temporal",
        manifest_temporal::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_TEMPORAL_MARKERS",
            "CANON_KIT_TEMPORAL_EXEMPT_SECTIONS",
            "CANON_KIT_TEMPORAL_EXEMPT_PATHS",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §lib/spec.sh — the emitter-backed vocabularies ride as bridged
    // id/pattern pairs, and the command knob rides too: it is what tells "none configured"
    // from "configured, declared nothing", the two clean skips this member reports apart
    (
        "check-install-claim",
        install_claim::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_MDREF_EXCLUDE",
            "CANON_KIT_INSTALL_CLAIM_EXCLUDE",
            "CANON_KIT_INSTALL_TRANSPORTS_CMD",
            "CANON_KIT_INSTALL_SECTION_RE",
            "CANON_KIT_INSTALL_TRANSPORT_IDS",
            "CANON_KIT_INSTALL_TRANSPORT_PATTERNS",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-unmarked-claim — born native beside its family, and it
    // derives its corpus from check-measured-claim's glob surface rather than from
    // `spec::manifest_files`: the knob set is that surface plus the class command and the two
    // bridged arrays the roster crosses as
    (
        "check-unmarked-claim",
        unmarked_claim::run,
        &[("?", "")],
        &[
            "CANON_KIT_CLAIM_CLASSES_CMD",
            "CANON_KIT_MEASURED_SURFACE_GLOBS",
            "CANON_KIT_CLAIM_CLASS_IDS",
            "CANON_KIT_CLAIM_CLASS_PATTERNS",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    (
        "check-payload-claim",
        payload_claim::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_MDREF_EXCLUDE",
            "CANON_KIT_PAYLOAD_CLAIM_EXCLUDE",
            "CANON_KIT_PAYLOAD_CLAIMS_CMD",
            "CANON_KIT_PAYLOAD_CLAIM_IDS",
            "CANON_KIT_PAYLOAD_CLAIM_PATTERNS",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — a
    // substrate-sensitive member by reverse trigger only: its `couples=` reaches gate
    // declaration paths, but the corpus it scans is the governed-doc set
    (
        "check-docs-cmd",
        docs_cmd::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "GATE_KIT_ROOTS_REL",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_MDREF_EXCLUDE",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` for the reason spelled out at
    // check-spec-fence-balance below: the walk root does not bound the read set
    (
        "check-md-refs",
        md_refs::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_MDREF_EXCLUDE",
            "CANON_KIT_DOCS_BLOB_REF",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §Fail-closed contract — a git-spawning member, reaching its
    // child through `proc::run` alone
    (
        "check-tracking-claim",
        tracking_claim::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — an empty read-root set: this member reads
    // named files and one directory listing per resolve dir, and every corpus it covers comes out
    // of `git ls-files` rather than a walk. The sentinel is what carries the filter-knob union.
    (
        "check-reads-couples",
        reads_couples::run,
        &[],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_KIT_ROOTS_REL",
            "GATE_PRUNE_DIRS",
            EVERY_FILTER_KNOB,
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — an empty read-root set: the pairing is one
    // pathname expansion over the root plus a named-file probe per pair, and neither descends
    (
        "check-template-copy-parity",
        template_copy_parity::run,
        &[],
        &["GATE_SDK_GATES_DIR"],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because the scan root is the member's own
    // first argument with a default; the candidate walk, the canonical-spec find and the
    // amendment find all anchor at that one root
    (
        "check-spec-embedded-source",
        spec_embedded_source::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_AMENDMENT_GLOB",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_EMBED_LANGS",
            "CANON_KIT_EMBED_ILLUSTRATIVE",
            "CANON_KIT_EMBED_THRESHOLD",
            "CANON_KIT_EMBED_MINLINES",
            "CANON_KIT_EMBED_WIRE_KIND",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — an empty read-root set: the member resolves
    // its corpus by one-level pathname expansion over each listed directory and descends into
    // none, so there is no walk root for the recorder to observe
    (
        "check-gate-exemption-tasks",
        gate_exemption_tasks::run,
        &[],
        &[
            "GATE_SDK_QUEUE_FILE",
            "GATE_SDK_GATES_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_SDK_NATIVE_CRATE",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-prose-tells — `?` because the scan root is the member's own
    // first argument with a default; the consumer-extended vocabularies cross as the *merged*
    // arrays their kit library unions before the bridge reads them, never as the extension names
    (
        "check-prose-tells",
        prose_tells::run,
        &[("?", "")],
        &[
            "CANON_KIT_PROSE_TELL_GLOBS",
            "CANON_KIT_PROSE_TELL_PHRASES",
            "CANON_KIT_PROSE_TELL_ABBR_ALLOW",
            "CANON_KIT_PROSE_TELL_EMDASH_MAX",
            "CANON_KIT_PROSE_TELL_CONTRAST_MAX",
            "CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES",
            "CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN",
            "CANON_KIT_PROSE_TELL_TRICOLON_MAX",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-knob-default-coupling — the kit-root walk is one `?` and
    // not one per kit: the roster's members and its arity both come from a knob, so no literal
    // root here could be anything but a second spelling of that knob's resolved value.
    (
        "check-knob-default-coupling",
        knob_default_coupling::run,
        &[("?", "")],
        &["GATE_PRUNE_DIRS", "GATE_KIT_ROOTS_REL", "CANON_KIT_SPEC_NAME"],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-knob-citation — the second consumer of the kit-root
    // mechanism inside this cohort: it calls it directly for its prefix roster, not only
    // through the manifest derivation, which is why the Rust form is a shared function
    (
        "check-knob-citation",
        knob_citation::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "GATE_KIT_ROOTS_REL",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` rather than the literal `.` the walk
    // starts from: a concrete root asserts the member's `couples=` covers every tracked file
    // under it, and these members read a filtered subset the root does not bound
    (
        "check-spec-fence-balance",
        spec_fence_balance::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_QUEUE_FILE",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §lib/spec.sh — the `spec_comment_surface` cohort: four members on
    // one corpus derivation, so each declares that derivation's whole knob set beside its own,
    // `GATE_SDK_WORKFLOW_DIR` among them because the corpus takes that directory's tracked tier
    // spec: gate-sdk/SPEC.md §check-reads-couples — one `?` for the derivation's two walk call
    // sites: both are anchored at the member's own first argument, so the recorder observes one
    (
        "check-comment-tier",
        comment_tier::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "GATE_SDK_WORKFLOW_DIR",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_COMMENT_SURFACE",
            "CANON_KIT_COMMENT_MACHINE",
            "CANON_KIT_COMMENT_REASON",
            "CANON_KIT_COMMENT_POSITIONAL",
            "CANON_KIT_COMMENT_WHITELIST",
            "CANON_KIT_COMMENT_RUN_CAP",
            "CANON_KIT_COUNT_COLLECTIONS",
            "CANON_KIT_COUNT_WEDGE_WORDS",
            "CANON_KIT_COUNT_ALLOWED_PHRASES",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-spec-pointer — the second corpus this member derives is the
    // manifest set, so both derivations' knob sets ride together
    (
        "check-spec-pointer",
        spec_pointer::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "GATE_SDK_WORKFLOW_DIR",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
            "CANON_KIT_COMMENT_SURFACE",
            "CANON_KIT_COMMENT_WHITELIST",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §lib/spec.sh — the queue-resolution pass both liveness members
    // read is a second shared derivation, so each declares the section vocabulary it is
    // computed from beside the corpus knobs
    (
        "check-todo-task-liveness",
        todo_task_liveness::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "GATE_SDK_WORKFLOW_DIR",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_COMMENT_SURFACE",
            "CANON_KIT_QUEUE_FILE",
            "CANON_KIT_ACTIVE_SECTIONS",
            "CANON_KIT_DEFERRED_SECTION",
            "CANON_KIT_ICEBOX_SECTION",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-amendment-queue — the amendment finder is the walk, and its
    // scan root is this member's *second* argument with a `.` default, so the root stays the `?`
    // the cohort declares; the feature set joins the vocabulary because only this member reads it
    (
        "check-amendment-queue",
        amendment_queue::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_AMENDMENT_GLOB",
            "CANON_KIT_QUEUE_FILE",
            "CANON_KIT_FEATURE_SECTIONS",
            "CANON_KIT_ACTIVE_SECTIONS",
            "CANON_KIT_DEFERRED_SECTION",
            "CANON_KIT_ICEBOX_SECTION",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-amendment-update-target — the amendment finder is the walk
    // and the scan root is this member's own first argument with a `.` default, so the root stays
    // the `?` the cohort declares; the knob set is `spec::amendments`' and nothing else, because
    // the two heading names are kit constants rather than config
    (
        "check-amendment-update-target",
        amendment_update_target::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_AMENDMENT_GLOB",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the marker vocabulary is a consumer array
    // joined into an alternation and interpreted, so this member compiles it through the engine
    (
        "check-deprecation-task",
        deprecation_task::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "GATE_SDK_WORKFLOW_DIR",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_COMMENT_SURFACE",
            "CANON_KIT_DEPRECATION_MARKERS",
            "CANON_KIT_QUEUE_FILE",
            "CANON_KIT_ACTIVE_SECTIONS",
            "CANON_KIT_DEFERRED_SECTION",
            "CANON_KIT_ICEBOX_SECTION",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because the scan root is the member's
    // own first argument with a default, the variable-first-argument shape the shell parser
    // calls undecidable
    (
        "check-roadmap-fresh",
        roadmap_fresh::run,
        &[],
        // spec: queue-kit/SPEC.md §check-roadmap-fresh — the consumer's lane vocabulary crosses
        // the bridge; the crate ships the <horizon>/<track> grammar and not one configured value.
        // The section trio comes with the shared adapter, which scopes its scan to live entries.
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
        "queue-kit",
        &[("date", ""), ("git", "")],
    ),
    (
        "check-queue-slug-liveness",
        queue_slug_liveness::run,
        &[("?", "")],
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_PROSE_SURFACE_GLOBS",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
        "queue-kit",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — five members sharing one
    // corpus derivation, `GATE_KIT_ROOTS_HERE`/`_REL`, so each declares the bridged spelling its
    // own rule reads and nothing more
    // spec: gate-sdk/SPEC.md §check-reads-couples — an empty walk-root set for the two members
    // that probe fixed literal paths rather than listing a directory: there is no root for the
    // recorder to observe, the shape the queue-kit cohort's file readers already declare.
    (
        "check-kit-registration",
        kit_registration::run,
        &[],
        &[
            "GATE_KIT_ROOTS_REL",
            "GATE_SDK_REGISTRY_DOC",
            "GATE_SDK_RUNNER_DOC",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-smoke-entry-guard",
        smoke_entry_guard::run,
        &[],
        &["GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because the listed directory set is the
    // member's own positional arguments with a kit-root-derived default: neither the count nor
    // the paths are statically bounded, which is the undecidable answer this line kind exists for
    (
        "check-test-hermetic",
        test_hermetic::run,
        &[("?", "")],
        &["GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[],
    ),
    (
        "check-assertion-strength",
        assertion_strength::run,
        &[("?", "")],
        &["GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — one `?` and not one per kit: the whole
    // `templates/*.list` listing goes through a single walk anchored at the scan root, which is
    // the member's own first argument with a default
    (
        "check-template-registry-parity",
        template_registry_parity::run,
        &[("?", "")],
        &["GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — the
    // lifecycle-kit cohort shares a config surface rather than a corpus walk, so each member
    // declares its own knobs plus every knob a shared derivation it calls is computed from
    (
        "check-stage-skill-coverage",
        stage_skill_coverage::run,
        &[("?", "")],
        &["LIFECYCLE_KIT_SKILLS_DIR", "LIFECYCLE_KIT_STAGES"],
        "lifecycle-kit",
        &[],
    ),
    (
        "check-skill-binding",
        skill_binding::run,
        &[("?", "")],
        &["LIFECYCLE_KIT_SKILLS_DIR"],
        "lifecycle-kit",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — an empty walk-root set for the members
    // that read named files rather than listing a directory: there is no root for the
    // recorder to observe, the shape the queue-kit cohort's file readers declare
    (
        "check-lifecycle-registration",
        lifecycle_registration::run,
        &[],
        &[
            "LIFECYCLE_KIT_AGENT_FILE",
            "LIFECYCLE_KIT_STAGES",
            "LIFECYCLE_KIT_QUEUE_FILE",
        ],
        "lifecycle-kit",
        &[],
    ),
    (
        "check-gap-inbox-neutrality",
        gap_inbox_neutrality::run,
        &[],
        &["LIFECYCLE_KIT_GAP_INBOX_FILE"],
        "lifecycle-kit",
        &[],
    ),
    (
        "check-merge-attrs",
        merge_attrs::run,
        &[],
        &[
            "LIFECYCLE_KIT_STATE_FILE",
            "LIFECYCLE_KIT_LESSON_EVIDENCE_FILE",
            "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
            "LIFECYCLE_KIT_BOUNDARY_TRUNCATE",
            "LIFECYCLE_KIT_GAP_INBOX_FILE",
        ],
        "lifecycle-kit",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because the walk's root is decided by
    // the glob knob's values, which the queue-kit cohort's prose-surface member already rules
    // undecidable statically for the same reason.
    (
        "check-evidence-baseline",
        evidence_baseline::run,
        &[("?", "")],
        &[
            "EVIDENCE_KIT_BASELINE_FILE",
            "EVIDENCE_KIT_QUEUE_FILE",
            "EVIDENCE_KIT_SCENARIO_GLOBS",
            "EVIDENCE_KIT_PERMANENT_SLUGS",
        ],
        "evidence-kit",
        &[("bash", "")],
    ),
    // spec: evidence-kit/SPEC.md §check-evidence-manifest — three named-file reads and no walk,
    // so the declared root set is empty and unit test A holds that to executed behavior
    (
        "check-evidence-manifest",
        evidence_manifest::run,
        &[],
        &[
            "EVIDENCE_KIT_MANIFEST_FILE",
            "EVIDENCE_KIT_QUEUE_FILE",
            "EVIDENCE_KIT_STATE_FILE",
            "EVIDENCE_KIT_SUITES",
        ],
        "evidence-kit",
        &[("bash", "")],
    ),
    // spec: delegation-kit/SPEC.md §Verify after every agent commit — the corpus is the git
    // index and two named fixture lists, so nothing is walked and the declared root set is empty
    (
        "check-gate-tamper",
        gate_tamper::run,
        &[],
        &["DELEGATION_KIT_GATE_FILES", "DELEGATION_KIT_META_PATHS"],
        "delegation-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — the filter-knob arm's first live instance:
    // assertion C's two whole-tree scans are one root selected by two knob values, and a literal
    // pattern here would be a second spelling of a knob's own default.
    (
        "check-stage-entry",
        stage_entry::run,
        &[
            (".", "LIFECYCLE_KIT_ROSTER_BASENAME"),
            (".", "LIFECYCLE_KIT_AMENDMENT_GLOB"),
        ],
        &[
            "LIFECYCLE_KIT_QUEUE_FILE",
            "LIFECYCLE_KIT_STATE_FILE",
            "LIFECYCLE_KIT_STAGES",
            "LIFECYCLE_KIT_PREDECESSOR",
            "LIFECYCLE_KIT_DRAIN_STAGE",
            "LIFECYCLE_KIT_ACTIVE_SECTIONS",
            "LIFECYCLE_KIT_AUDIT_STAGE",
            "LIFECYCLE_KIT_AUDIT_ENTRY_STAGE",
            "LIFECYCLE_KIT_WAIVER_TOKEN",
            "LIFECYCLE_KIT_ROSTER_BASENAME",
            "LIFECYCLE_KIT_AMENDMENT_GLOB",
            "LIFECYCLE_KIT_CONTRACT_TOKENS",
            "GATE_PRUNE_DIRS",
        ],
        "lifecycle-kit",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §The twelfth cohort — both walks hang off a base that is this
    // member's own first argument with a default, so both take the undecidable marker.
    (
        "check-close-surfaces",
        close_surfaces::run,
        &[("?", ""), ("?", "")],
        &[
            "GATE_KIT_ROOTS_REL",
            "LIFECYCLE_KIT_ROSTER_BASENAME",
            "LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS",
            "GATE_SDK_WORKFLOW_DIR",
        ],
        "lifecycle-kit",
        &[("date", ""), ("git", "")],
    ),
    (
        "check-stage-evidence",
        stage_evidence::run,
        &[],
        // spec: lifecycle-kit/SPEC.md §check-stage-evidence — the four boundary knobs ride
        // because the purity assertion's exemption set is derived from them
        &[
            "LIFECYCLE_KIT_QUEUE_FILE",
            "LIFECYCLE_KIT_STATE_FILE",
            "LIFECYCLE_KIT_STAGES",
            "LIFECYCLE_KIT_FIRST_STAGE",
            "LIFECYCLE_KIT_WAIVER_TOKEN",
            "LIFECYCLE_KIT_SESSION_BOUNDARY",
            "LIFECYCLE_KIT_LESSON_EVIDENCE_FILE",
            "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
            "LIFECYCLE_KIT_BOUNDARY_TRUNCATE",
            "LIFECYCLE_KIT_GAP_INBOX_FILE",
        ],
        "lifecycle-kit",
        &[("git", "")],
    ),
    (
        "check-lesson-disposition",
        lesson_disposition::run,
        &[],
        &[
            "LIFECYCLE_KIT_QUEUE_FILE",
            "LIFECYCLE_KIT_LESSON_EVIDENCE_FILE",
        ],
        "lifecycle-kit",
        &[("git", "")],
    ),
    (
        "check-survey-record",
        survey_record::run,
        &[],
        &["LIFECYCLE_KIT_SURVEY_RECORD_FILE"],
        "lifecycle-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — two `?` and not one: the skills-dir listing
    // and the per-kit templates walk are separate call sites with separately unbounded roots,
    // which is the arity the shell parser's own skipped-and-counted accounting has
    (
        "check-shim-restatement",
        shim_restatement::run,
        &[("?", ""), ("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_REL",
            "LIFECYCLE_KIT_SKILLS_DIR",
            "LIFECYCLE_KIT_SHIM_NGRAM",
            "LIFECYCLE_KIT_SHIM_DEDUP_CORPUS",
            "LIFECYCLE_KIT_AGENT_FILE",
        ],
        "lifecycle-kit",
        &[],
    ),
    // spec: lifecycle-kit/SPEC.md §check-scratch-citation — the surface globs expand from the
    // invoking directory through one `**`-capable listing, so one unbounded root; the four
    // supersede knobs ride because the forbidden-target set is derived from them
    (
        "check-scratch-citation",
        scratch_citation::run,
        &[("?", "")],
        &[
            "LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS",
            "LIFECYCLE_KIT_STATE_FILE",
            "LIFECYCLE_KIT_LESSON_EVIDENCE_FILE",
            "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
            "LIFECYCLE_KIT_BOUNDARY_TRUNCATE",
        ],
        "lifecycle-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency — both
    // members read named files rather than walking, so each declares an empty walk-root set,
    // the shape the queue-kit cohort's file readers already declare
    (
        "check-settings-paths",
        settings_paths::run,
        &[],
        &["CONTEXT_KIT_SETTINGS_FILE"],
        "context-kit",
        &[],
    ),
    (
        "check-settings-pins",
        settings_pins::run,
        &[],
        &["CONTEXT_KIT_SETTINGS_FILE", "CONTEXT_KIT_SETTINGS_PINS"],
        "context-kit",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §The declaration cohort — the consumer sentinel's first members: each
    // walks one directory relocated by the gate's own positional argument, so the honest
    // declaration is a single `?`, and a subprocess read enters no walk roster in either substrate.
    // spec: gate-sdk/SPEC.md §The declaration cohort — none declares a knob, which keeps
    // `gate_command` on its zero-knob path and the config-bridge question unasked.
    (
        "check-release-bump",
        release_bump::run,
        &[("?", "")],
        &[],
        "-",
        &[("git", "")],
    ),
    (
        "check-tightened-gates-grammar",
        tightened_gates_grammar::run,
        &[("?", "")],
        &[],
        "-",
        &[],
    ),
    (
        "check-tightened-gates-note-parity",
        tightened_gates_note_parity::run,
        &[("?", "")],
        &[],
        "-",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the rest of the consumer's own
    // gates directory, every member on the `-` sentinel. The two that declare a knob declare
    // what they execute: a shared derivation's whole knob set.
    (
        "check-docs-kit-parity",
        docs_kit_parity::run,
        &[("?", "")],
        &[
            "GATE_KIT_ROOTS_REL",
            "GATE_SDK_REGISTRY_DOC",
            "GATE_SDK_RUNNER_DOC",
        ],
        "-",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` for each member whose walk root is its
    // own positional argument with a default, the variable-first-argument shape the shell parser
    // calls undecidable; an empty set for the members that read named files and list nothing.
    (
        "check-docs-mirror-fresh",
        docs_mirror_fresh::run,
        // spec: gate-sdk/SPEC.md §check-reads-couples — **two** unbounded roots off the scan-root
        // argument: the generator's source-set listing of `<root>`, and the orphan sweep over
        // `<root>/docs`. The first was invisible while the generator was a subprocess.
        &[("?", ""), ("?", "")],
        // spec: gate-sdk/SPEC.md §The non-gate arm — the generator it now calls in-process reads
        // the blob ref, so the comparator declares what its callee reads: a knob the bridge does
        // not carry is a knob the emission cannot resolve.
        &["CANON_KIT_DOCS_BLOB_REF"],
        "-",
        &[("date", ""), ("git", "")],
    ),
    (
        "check-docs-nav-reachable",
        docs_nav_reachable::run,
        &[("?", "")],
        &[],
        "-",
        &[("git", "")],
    ),
    (
        "check-install-toolchain",
        install_toolchain::run,
        &[],
        &[],
        "-",
        &[("git", "")],
    ),
    (
        "check-installer-no-deps",
        installer_no_deps::run,
        &[],
        &[],
        "-",
        &[("git", "")],
    ),
    (
        "check-kit-ref-liveness",
        kit_ref_liveness::run,
        &[],
        &[
            "GATE_KIT_ROOTS_REL",
            "GATE_PRUNE_DIRS",
            "GATE_SDK_QUEUE_FILE",
        ],
        "-",
        &[("git", "")],
    ),
    (
        "check-npm-publish-spec",
        npm_publish_spec::run,
        &[("?", "")],
        &[],
        "-",
        &[("git", "")],
    ),
    (
        "check-release-channel-parity",
        release_channel_parity::run,
        &[],
        &[],
        "-",
        &[("git", "")],
    ),
    (
        "check-trajectory-fresh",
        trajectory_fresh::run,
        &[],
        // spec: gate-sdk/SPEC.md §The non-gate arm — the extractor it now calls in-process reads
        // the stage roster and the evidence surfaces, so the comparator declares them
        &[
            "DRIFT_KIT_CONFIG_FILE",
            "DRIFT_KIT_TRAJECTORY_SURFACES",
            "DRIFT_KIT_GATES_FILE",
            "DRIFT_KIT_STAGES",
            "GATE_SDK_WORKFLOW_DIR",
        ],
        "-",
        &[("date", ""), ("git", "")],
    ),
    (
        "check-value-rollup-fresh",
        value_rollup_fresh::run,
        &[("?", "")],
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
        "-",
        &[("date", ""), ("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — no walk root: the comparator reads one named
    // projection file, and the emitter it calls in-process declares its own reads through the knob
    // below rather than through a root the recorder could observe.
    (
        "check-footprint-fresh",
        footprint_fresh::run,
        &[],
        &["CONTEXT_KIT_SURFACES"],
        "context-kit",
        &[("date", ""), ("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — the monitor walk's root is the emitter's own
    // scan knob, declared below; the comparator itself reads one named projection file.
    (
        "check-enforcement-fresh",
        enforcement_fresh::run,
        &[("?", "")],
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
        "gate-sdk",
        &[("date", ""), ("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — a budget
    // batch's members carry no joint proof, so each declares its own reads and nothing shared:
    // both read named files rather than listing a directory, the empty-walk-root shape.
    (
        "check-brevity",
        brevity::run,
        &[],
        &[
            "CONTEXT_KIT_BREVITY_FILE",
            "CONTEXT_KIT_BREVITY_BUDGET",
            "CONTEXT_KIT_BREVITY_SECTION",
            "CONTEXT_KIT_BREVITY_POINTER_RE",
        ],
        "context-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — the batch's
    // second walker-riding member, sequenced behind the first rather than cohorted with it: it
    // reads named files rather than listing a directory, so the walk-root set is empty.
    (
        "check-doctrine-registration",
        doctrine_registration::run,
        &[],
        &[
            "DOCTRINE_KIT_AGENT_FILE",
            "DOCTRINE_KIT_DOCTRINE_FILE",
            "DOCTRINE_KIT_DIGEST_SECTION",
        ],
        "doctrine-kit",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — the first
    // budget batch's remaining four members, each its own unit with no joint proof: a `?` for a
    // positional scan root the shell parser calls undecidable, an empty set for named-file readers.
    (
        "check-hook-exec-bit",
        hook_exec_bit::run,
        &[("?", "")],
        &["GATE_SDK_HOOKS_DIR"],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-agent-tier-explicit",
        agent_tier_explicit::run,
        &[("?", "")],
        &["DELEGATION_KIT_AGENT_DIR", "GATE_PRUNE_DIRS"],
        "delegation-kit",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — the two
    // file paths are positional arguments with hardcoded defaults and no env knob, so this
    // member declares the empty knob list.
    (
        "check-rule-citation",
        rule_citation::run,
        &[],
        &[],
        "delegation-kit",
        &[],
    ),
    (
        "check-workflow-tiering",
        workflow_tiering::run,
        &[("?", "")],
        &["GATE_SDK_WORKFLOW_DIR"],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The second budget batch — two members with no joint proof, each its
    // own unit: the first reads one message file and walks nothing, the empty-walk-root shape; the
    // second lists every kit's `checks/` through one glob anchored at its own positional scan root.
    (
        "check-commit-subject",
        commit_subject::run,
        &[],
        &["GATE_SDK_COMMIT_TYPES"],
        "gate-sdk",
        &[],
    ),
    (
        "check-readme-roster",
        readme_roster::run,
        &[("?", "")],
        &["GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §The third budget batch — two members with no joint proof, each its
    // own unit: the first walks each resolved memory dir, one in every case a fixture can build;
    // the second's tracked set is a subprocess read, which enters no walk roster in either.
    (
        "check-memory-off",
        memory_off::run,
        &[("?", "")],
        &[
            "CONTEXT_KIT_MEMORY_DIRS",
            "CONTEXT_KIT_SETTINGS_FILE",
            "CONTEXT_KIT_SETTINGS_PINS",
        ],
        "context-kit",
        &[("git", "")],
    ),
    (
        "check-root-tiering",
        root_tiering::run,
        &[],
        &[
            "GATE_SDK_ROOT_ALLOWLIST",
            "GATE_SDK_QUEUE_FILE",
            "GATE_SDK_AGENT_FILE",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The fourth budget batch — members with no joint proof, each its own
    // unit. None declares a walk root: two resolve a corpus by pathname expansion and the third
    // by `git ls-files`, and §check-reads-couples rules both outside the walk class.
    // spec: gate-sdk/SPEC.md §check-graph — the coupling-graph auditor. `?` as its walk root for
    // the reason its shell original was classified `?`: assertion G's scan root is the member's own
    // first argument with a default, the variable-first-argument shape §check-reads-couples calls
    // undecidable. It declares GATE_SDK_ROOT_HERE because assertion D spawns the hook generator,
    // which stays shell (§gen-pre-commit), and a compiled member has no BASH_SOURCE to find it by.
    (
        "check-graph",
        graph::run,
        &[("?", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_HOOKS_DIR",
            "GATE_SDK_ROOT_HERE",
            "GATE_KIT_ROOTS_HERE",
            "GATE_KIT_ROOTS_REL",
            "GATE_SDK_GRAPH_ARTIFACT",
            "GATE_SDK_GRAPH_THEME_DIR",
            "GATE_SDK_GRAPH_MAX_EDGES",
            "GATE_GRAPH_EXTERNAL_REFS",
            "GRAPH_VOCAB",
            "GRAPH_LEADING",
            "GRAPH_LAGGING",
            "GRAPH_LAYERS",
            "GRAPH_LAYER_RULES",
            "GRAPH_LAYER_DEFAULT",
        ],
        "gate-sdk",
        &[("bash", ""), ("date", ""), ("git", "")],
    ),
    (
        "check-gate-fail-closed",
        gate_fail_closed::run,
        &[],
        &["GATE_SDK_GATES_DIR", "GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-gate-binary-fresh",
        gate_binary_fresh::run,
        &[],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_NATIVE_BIN",
            "GATE_SDK_NATIVE_CRATE",
            "GATE_KIT_ROOTS_REL",
        ],
        "gate-sdk",
        &[("git", ""), ("?", "GATE_SDK_NATIVE_BIN")],
    ),
    // spec: gate-sdk/SPEC.md §check-gate-assertions — no walk root: the corpus is the kit SPEC
    // set at fixed paths, and each heading resolves through the registry rather than through a
    // tree walk. The three knobs are §check-gate-output's, because the resolution is the same one.
    (
        "check-gate-assertions",
        gate_assertions::run,
        &[],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_SDK_NATIVE_CRATE",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-gate-fixture-coverage",
        gate_fixture_coverage::run,
        &[],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_TESTS_DIR",
            "GATE_KIT_ROOTS_HERE",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-gate-output",
        gate_output::run,
        &[],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_SDK_NATIVE_CRATE",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-kit-enum",
        kit_enum::run,
        &[],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_KIT_ROOTS_REL",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-docs-cname-parity",
        docs_cname_parity::run,
        &[],
        &[
            "SITE_KIT_SCAN_ROOT",
            "SITE_KIT_CNAME",
            "SITE_KIT_ALIASES",
            "SITE_KIT_EXEMPT_PATHS",
            "GATE_PRUNE_DIRS",
        ],
        "site-kit",
        &[("git", "")],
    ),
    (
        "check-identity",
        identity::run,
        &[],
        &[
            "GATE_SDK_IDENTITY_FILE",
            "GATE_SDK_GIT_EMAIL_FILE",
            "GATE_SDK_GIT_REMOTES_FILE",
            "GATE_SDK_GH_HOSTS_FILE",
            "GATE_SDK_GH_HOST",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-exec-bit",
        exec_bit::run,
        &[],
        &["GATE_EXEC_GLOBS", "GATE_EXEC_PRUNE"],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-core-files",
        core_files::run,
        &[],
        &["GATE_SDK_CORE_FILES_FILE", "GATE_KIT_ROOTS_REL"],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-battery-roster",
        battery_roster::run,
        &[],
        &[
            "EVIDENCE_KIT_RUNNER_DOC",
            "EVIDENCE_KIT_SUITES",
            "EVIDENCE_KIT_RUN_*",
        ],
        "evidence-kit",
        &[("git", "")],
    ),
    (
        "check-commit-msg",
        commit_msg::run,
        &[],
        &["GATE_MSG_PATTERN_FILES", "GATE_MSG_PATTERN_FILES_LOCAL"],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-docs-link-convention",
        docs_link_convention::run,
        &[("?", "")],
        &["CANON_KIT_LINK_ROOT"],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-tree-terms — no walk root: the corpus is `git ls-files`,
    // which §check-reads-couples rules outside the walk class. The two pattern-file knobs are
    // check-commit-msg's, declared here too because both halves read one resolved source.
    (
        "check-tree-terms",
        tree_terms::run,
        &[],
        &[
            "GATE_MSG_PATTERN_FILES",
            "GATE_MSG_PATTERN_FILES_LOCAL",
            "GATE_PRUNE_DIRS",
        ],
        "gate-sdk",
        &[("git", "")],
    ),
];

pub fn lookup(name: &str) -> Option<GateFn> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _, _)| *n == name)
        .map(|(_, f, _, _, _, _)| *f)
}

pub fn roots(name: &str) -> Option<&'static [(&'static str, &'static str)]> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _, _)| *n == name)
        .map(|(_, _, r, _, _, _)| *r)
}

// spec: gate-sdk/SPEC.md §check-reads-couples — the union sentinel a member declares when the
// knob names it must resolve are not known until run time; it expands to every filter-knob name
// the registry carries
pub const EVERY_FILTER_KNOB: &str = "@every-filter-knob";

// spec: gate-sdk/SPEC.md §check-reads-couples — the expansion is a crate-internal carrier: the
// arm's own output stays one knob name per line, and no `.gate` descriptor field moves.
fn expanded_knobs() -> &'static [(&'static str, Vec<&'static str>)] {
    static EXPANDED: std::sync::OnceLock<Vec<(&'static str, Vec<&'static str>)>> =
        std::sync::OnceLock::new();
    EXPANDED.get_or_init(|| {
        let mut filters: Vec<&'static str> = REGISTRY
            .iter()
            .flat_map(|(_, _, roots, _, _, _)| roots.iter().map(|(_, k)| *k))
            .filter(|k| !k.is_empty())
            .collect();
        filters.sort();
        filters.dedup();
        REGISTRY
            .iter()
            .filter(|(_, _, _, k, _, _)| k.contains(&EVERY_FILTER_KNOB))
            .map(|(n, _, _, k, _, _)| {
                let mut out: Vec<&'static str> =
                    k.iter().copied().filter(|x| *x != EVERY_FILTER_KNOB).collect();
                for f in &filters {
                    if !out.contains(f) {
                        out.push(f);
                    }
                }
                (*n, out)
            })
            .collect()
    })
}

pub fn knobs(name: &str) -> Option<&'static [&'static str]> {
    if let Some((_, v)) = expanded_knobs().iter().find(|(n, _)| *n == name) {
        return Some(v.as_slice());
    }
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _, _)| *n == name)
        .map(|(_, _, _, k, _, _)| *k)
}

// spec: gate-sdk/SPEC.md §The `# graph:` manifest — the requirement set `--needs` prints. A
// lookup by name rather than a roster, because the reader asks about one member at a time,
// exactly as `--reads` and `--knobs` are asked.
pub fn needs(name: &str) -> Option<&'static [(&'static str, &'static str)]> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _, _)| *n == name)
        .map(|(_, _, _, _, _, r)| *r)
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — `--list`'s two columns, the second
// naming the root whose declaration carries the member: a kit's `checks/`, or the consumer
// sentinel. Emitted together because the reader needs the owner to scope the roster at all.
pub fn names_with_owners() -> Vec<(&'static str, &'static str)> {
    REGISTRY.iter().map(|(n, _, _, _, o, _)| (*n, *o)).collect()
}

// spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — each `?`
// absorbs one unmatched observed root, so the declaration is held to its arity. Pure, so
// the concrete-root branch is provable without a member that declares one.
// spec: gate-sdk/SPEC.md §The `# graph:` manifest — `--needs`' declaration is covered by this
// same function and not a second one: both arms declare in one two-field shape where `?` means
// *the registry cannot name this literal*, so one covering rule holds both to behavior.
#[cfg(test)]
fn declaration_covers(
    verb: &str,
    declared: &[(&str, &str)],
    observed: &[String],
) -> Result<(), String> {
    let mut wildcards = declared.iter().filter(|(d, _)| *d == "?").count();
    let mut undeclared: Vec<&str> = Vec::new();
    for o in observed {
        if declared.iter().any(|(d, _)| *d != "?" && d == o) {
            continue;
        }
        if wildcards > 0 {
            wildcards -= 1;
            continue;
        }
        undeclared.push(o.as_str());
    }
    if undeclared.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} {:?} but declares {:?}",
        verb, undeclared, declared
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walk;

    #[test]
    fn a_concrete_root_matches_by_equality_and_a_leftover_is_undeclared() {
        assert!(declaration_covers("walked", &[("corpus", "")], &["corpus".into()]).is_ok());
        assert!(
            declaration_covers("walked", &[("corpus", "")], &["corpus".into(), "other".into()]).is_err()
        );
        assert!(declaration_covers("walked", &[], &["corpus".into()]).is_err());
    }

    #[test]
    fn each_question_mark_absorbs_exactly_one_unbounded_root() {
        assert!(declaration_covers("walked", &[("?", "")], &["anything".into()]).is_ok());
        assert!(declaration_covers("walked", &[("?", "")], &["a".into(), "b".into()]).is_err());
        assert!(
            declaration_covers("walked", &[("corpus", ""), ("?", "")], &["corpus".into(), "x".into()])
                .is_ok()
        );
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — the filter-knob half is a declaration about
    // the root, never a second root: a member declaring one root twice under two filters is
    // covered by one observation of that root, which is what the ported walk actually does.
    #[test]
    fn a_filter_knob_narrows_a_root_without_multiplying_it() {
        assert!(declaration_covers(
            "walked",
            &[(".", "KIT_A"), (".", "KIT_B")],
            &[".".into()]
        )
        .is_ok());
        assert!(declaration_covers("walked", &[(".", "KIT_A")], &["other".into()]).is_err());
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the owner column is registry data
    // held to executed behavior: the declared root must carry the descriptor, over both root
    // shapes, which is why the name says root rather than kit.
    #[test]
    fn every_registry_member_declares_the_root_that_carries_its_descriptor() {
        assert!(!REGISTRY.is_empty(), "no member to assert over");
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut gates_dir: Option<String> = None;
        for (name, _, _, _, owner, _) in REGISTRY {
            let declared = if *owner == "-" {
                let dir = gates_dir
                    .get_or_insert_with(|| resolve_gates_dir(&repo))
                    .clone();
                repo.join(dir).join(format!("{}.gate", name))
            } else {
                repo.join(owner).join("checks").join(format!("{}.gate", name))
            };
            assert!(
                declared.is_file(),
                "{} declares owner {}, but {} is not a file",
                name,
                owner,
                declared.display()
            );
        }
    }

    // spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — the sentinel's
    // declaring root is a layout, resolved by the one owner of that value so the crate carries
    // no gates-directory default of its own.
    fn resolve_gates_dir(repo: &std::path::Path) -> String {
        let at = repo.display().to_string();
        let completed = crate::proc::run(
            "bash",
            &[
                "-c",
                "cd \"$1\" || exit 2; . gate-sdk/lib/gate.sh; gate_sdk_gates_dir",
                "bash",
                &at,
            ],
        )
        .expect("cannot run the shell library's gates-directory resolution");
        let out = completed
            .stdout()
            .expect("gate-sdk/lib/gate.sh could not resolve the gates directory");
        let dir = String::from_utf8_lossy(out).trim().to_string();
        assert!(
            !dir.is_empty(),
            "the shell library resolved no gates directory — a sentinel member's descriptor \
             would be looked for at the repo root"
        );
        dir
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test A: the declared roots are
    // held to what the code does, by running each member over its own fixture cases with
    // the walk recorder on. Nothing else makes a self-declared read set trustworthy.
    #[test]
    fn every_registry_member_declares_the_roots_it_walks() {
        assert!(!REGISTRY.is_empty(), "no member to assert over");
        let env = crate::knobenv::lock();
        walk::bridge_declared_knobs(&env);
        let mut cases_run = 0usize;
        let mut roots_observed = 0usize;
        for (name, f, declared, _, _, _) in REGISTRY {
            // spec: gate-sdk/SPEC.md §check-reads-couples — resolved through `knobs`, not off the
            // tuple, so a member declaring the union sentinel is bridged the expansion the
            // dispatcher would bridge rather than the sentinel itself
            let member_knobs = knobs(name).unwrap_or(&[]);
            for case in walk::fixture_case_dirs(name) {
                let args = case_args(&case);
                // spec: gate-sdk/SPEC.md §run-gate-tests — the member's knobs are bridged from
                // the case dir before it runs, or a bridged member exits 2 on an unresolved
                // knob and this test asserts over a run that never reached its rule
                walk::bridge_case_knobs(&env, &case, name, member_knobs);
                let prev = std::env::current_dir().expect("cannot read cwd");
                // spec: gate-sdk/SPEC.md §check-reads-couples — the case is entered exactly
                // as run-gate-tests.sh enters it, so an observed root is the same string the
                // gate would walk from the repo root in the battery.
                std::env::set_current_dir(&case)
                    .unwrap_or_else(|e| panic!("cannot enter {}: {}", case.display(), e));
                walk::recorder::start();
                let rc = (*f)(&args);
                let observed = walk::recorder::stop();
                std::env::set_current_dir(&prev).expect("cannot restore cwd");
                assert_ne!(
                    rc, 2,
                    "{} errored on {} — an observation taken from a run that never walked \
                     would pass this test by being empty",
                    name,
                    case.display()
                );
                if let Err(e) = declaration_covers("walked", declared, &observed) {
                    panic!("{} on {}: {}", name, case.display(), e);
                }
                cases_run += 1;
                roots_observed += observed.len();
            }
        }
        assert!(cases_run > 0, "no fixture case found for any registry member");
        assert!(
            roots_observed > 0,
            "no member walked anything — the subset assertion above held over nothing"
        );
    }

    // spec: gate-sdk/SPEC.md §The `# graph:` manifest — unit test A for `--needs`: each member
    // runs over its own fixture cases with the spawn recorder on, and observed must be a subset
    // of declared. That section owns why the direction is a subset and not an equality.
    // comment-tier-exempt: every offender is collected before the panic rather than the first,
    // which is a property of this test's loop and of nothing the SPEC describes
    #[test]
    fn every_registry_member_declares_the_programs_it_spawns() {
        assert!(!REGISTRY.is_empty(), "no member to assert over");
        let env = crate::knobenv::lock();
        walk::bridge_declared_knobs(&env);
        let mut cases_run = 0usize;
        let mut offenders: Vec<String> = Vec::new();
        for (name, f, _, _, _, declared) in REGISTRY {
            let member_knobs = knobs(name).unwrap_or(&[]);
            for case in walk::fixture_case_dirs(name) {
                let args = case_args(&case);
                walk::bridge_case_knobs(&env, &case, name, member_knobs);
                let prev = std::env::current_dir().expect("cannot read cwd");
                std::env::set_current_dir(&case)
                    .unwrap_or_else(|e| panic!("cannot enter {}: {}", case.display(), e));
                crate::proc::recorder::start();
                let rc = (*f)(&args);
                let observed = crate::proc::recorder::stop();
                std::env::set_current_dir(&prev).expect("cannot restore cwd");
                assert_ne!(
                    rc, 2,
                    "{} errored on {} — an observation taken from a run that never spawned \
                     would pass this test by being empty",
                    name,
                    case.display()
                );
                if let Err(e) = declaration_covers("spawned", declared, &observed) {
                    offenders.push(format!("{} on {}: {}", name, case.display(), e));
                }
                cases_run += 1;
            }
        }
        assert!(cases_run > 0, "no fixture case found for any registry member");
        assert!(
            offenders.is_empty(),
            "member(s) spawned a program their registry entry does not declare, so `--needs` \
             would under-report what a consumer's machine has to carry:\n  {}",
            offenders.join("\n  ")
        );
    }

    // spec: gate-sdk/SPEC.md §run-gate-tests — the case's `args` file on the runner's own
    // terms: drop lines starting `#`, then split the rest on whitespace as its unquoted
    // expansion does, so this test and the runner cannot disagree about one case's argv.
    fn case_args(case: &std::path::Path) -> Vec<String> {
        let p = case.join("args");
        let text = match std::fs::read_to_string(&p) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };
        text.lines()
            .filter(|l| !l.starts_with('#'))
            .flat_map(str::split_whitespace)
            .map(String::from)
            .collect()
    }
}
