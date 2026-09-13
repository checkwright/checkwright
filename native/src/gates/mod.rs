// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one module per ported gate; the
// subcommand name is the gate name, so no mapping table exists to drift
pub mod action_gh_repo;
pub mod action_permissions;
pub mod action_pinning;
pub mod action_run_shell;
pub mod amendment_queue;
pub mod amendment_retired_spelling;
pub mod amendment_update_target;
pub mod agent_tier_explicit;
pub mod assertion_strength;
pub mod brevity;
pub mod close_surfaces;
pub mod comment_tier;
pub mod commit_subject;
pub mod doctrine_registration;
pub mod deferred_board_tags;
pub mod deprecation_task;
pub mod docs_cmd;
pub mod gate_assertions;
pub mod gate_exemption_tasks;
pub mod gate_fail_closed;
pub mod gate_binary_fresh;
pub mod gate_substrate_parity;
pub mod gate_fixture_coverage;
pub mod gate_output;
pub mod gate_tamper;
pub mod graph;
pub mod kit_enum;
pub mod docs_cname_parity;
pub mod exec_bit;
pub mod core_files;
pub mod crate_arms;
pub mod battery_roster;
pub mod commit_msg;
pub mod docs_link_convention;
pub mod docs_kit_parity;
pub mod docs_mirror_fresh;
pub mod docs_render_fidelity;
pub mod docs_nav_reachable;
pub mod enforcement_fresh;
pub mod evidence_baseline;
pub mod evidence_manifest;
pub mod footprint_fresh;
pub mod install_platforms;
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
pub mod install_disposition;
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
pub mod path_dialect;
pub mod payload_claim;
pub mod portability_floor;
pub mod producer_liveness;
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
pub mod shellcheck;
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
pub mod surface_duplication;
pub mod surface_ratchet;
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

// spec: gate-sdk/SPEC.md §check-reads-couples — one declared walk root: the root, its filter, its
// prune, and the ground a `?` root is declared on — empty on every other root, and the const
// assertion below the registry refuses either mistake at compile time.
pub type RootDecl = (&'static str, &'static str, &'static str, &'static str);

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
    &'static [RootDecl],
    &'static [&'static str],
    &'static str,
    &'static [(&'static str, &'static str)],
);

// spec: canon-kit/SPEC.md §lib/spec.sh — `spec::manifest_files`' branch set: the configured corpus
// and the prose surface it adds, declared once for the members that call it
// spec: gate-sdk/SPEC.md §check-reads-couples — the kit-literal fallback keeps `?`, because the
// conservative authoring rule it would be held to is the part of the `couples=` semantics that
// section files as unsettled, and a kit cannot demand an adopter satisfy it over an unseen tree
const MANIFEST_ROOTS: &[RootDecl] = &[
    (".", "glob:knob:CANON_KIT_MANIFEST_FILES", "", ""),
    ("?", "", "", "fallback@src/spec.rs:214 via const MANIFEST_ROOTS"),
    ("?", "", "", "fallback@src/spec.rs:220 via const MANIFEST_ROOTS"),
    (".", "glob:knob:CANON_KIT_PROSE_SURFACE_GLOBS", "", ""),
];

// spec: canon-kit/SPEC.md §lib/spec.sh — `spec::comment_surface`'s two runtime branches: the
// configured one declares, and the kit-literal fallback keeps `?` on the ground above
const COMMENT_SURFACE_ROOTS: &[RootDecl] = &[
    (".", "glob:knob:CANON_KIT_COMMENT_SURFACE", "", ""),
    ("?", "", "", "fallback@src/spec.rs:243 via const COMMENT_SURFACE_ROOTS"),
];

// spec: gate-sdk/SPEC.md §check-reads-couples — `MANIFEST_ROOTS` and `COMMENT_SURFACE_ROOTS`
// together: this member walks twice, through both shared helpers, and a declaration per member
// rather than per walk is what drops the second walk silently.
const SPEC_POINTER_ROOTS: &[RootDecl] = &[
    (".", "glob:knob:CANON_KIT_MANIFEST_FILES", "", ""),
    ("?", "", "", "fallback@src/spec.rs:214 via const SPEC_POINTER_ROOTS"),
    ("?", "", "", "fallback@src/spec.rs:220 via const SPEC_POINTER_ROOTS"),
    (".", "glob:knob:CANON_KIT_PROSE_SURFACE_GLOBS", "", ""),
    (".", "glob:knob:CANON_KIT_COMMENT_SURFACE", "", ""),
    ("?", "", "", "fallback@src/spec.rs:243 via const SPEC_POINTER_ROOTS"),
];

pub const REGISTRY: &[GateEntry] = &[
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because each member's scan root is
    // its own first argument with a default, the same variable-first-argument shape the
    // shell parser calls undecidable and skips-and-counts.
    (
        "check-action-pinning",
        action_pinning::run,
        &[(".", "ext:lit:yml,yaml", "", "")],
        &["GATE_PRUNE_DIRS"],
        "gate-sdk",
        &[],
    ),
    // spec: gate-sdk/SPEC.md §check-action-run-shell — the same `?` scan root as its cohort
    // sibling above, and `shellcheck` as the declared dependency criterion 7's wrapper ruling
    // keeps: the program is the rule, so it stays off the payload and on the floor of what the
    // member refuses without.
    (
        "check-action-run-shell",
        action_run_shell::run,
        &[(".", "ext:lit:yml,yaml", "", "")],
        &["GATE_PRUNE_DIRS"],
        "gate-sdk",
        &[("shellcheck", "")],
    ),
    (
        "check-action-gh-repo",
        action_gh_repo::run,
        &[(".", "ext:lit:yml,yaml", "", "")],
        &["GATE_PRUNE_DIRS"],
        "gate-sdk",
        &[],
    ),
    (
        "check-action-permissions",
        action_permissions::run,
        &[(".", "ext:lit:yml,yaml", "", "")],
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
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_WRAP_BUDGET",
            "QUEUE_KIT_DEFERRED_SECTION",
        ],
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
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` for the one-level root listing, which is
    // the member's second argument with a default; the queue itself is a named file, not a walk
    (
        "check-deferred-board-tags",
        deferred_board_tags::run,
        &[],
        &[
            "QUEUE_KIT_QUEUE_FILE",
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
        MANIFEST_ROOTS,
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
        MANIFEST_ROOTS,
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
        &[(".", "glob:knob:CANON_KIT_MEASURED_SURFACE_GLOBS", "", "")],
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
        &[(".", "name:knob:CANON_KIT_SPEC_NAME", "**/templates,docs/*", "")],
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
        &[(".", "name:knob:CANON_KIT_SPEC_NAME", "**/templates,docs/*", "")],
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
        MANIFEST_ROOTS,
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
        MANIFEST_ROOTS,
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
        &[(".", "glob:knob:CANON_KIT_MEASURED_SURFACE_GLOBS", "", "")],
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
        MANIFEST_ROOTS,
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
        MANIFEST_ROOTS,
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
        MANIFEST_ROOTS,
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
        MANIFEST_ROOTS,
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
            crate::registry::EVERY_COUPLES_KNOB,
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
    // spec: gate-sdk/SPEC.md §check-reads-couples — three walks at one root, declared per walk: two
    // name their bounding knobs, and the source-candidate walk keeps `?` because its filter is a
    // *projection* out of `CANON_KIT_EMBED_LANGS`' packed elements rather than a knob's value
    (
        "check-spec-embedded-source",
        spec_embedded_source::run,
        &[(".", "name:knob:CANON_KIT_AMENDMENT_GLOB", "", ""), (".", "name:knob:CANON_KIT_SPEC_NAME", "**/templates,docs/*", ""), ("?", "", "", "projection@src/gates/spec_embedded_source.rs:147")],
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
            "GATE_PRUNE_DIRS",
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
        &[(".", "glob:knob:CANON_KIT_PROSE_TELL_GLOBS", "", "")],
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
        &[("?", "", "", "dynamic@src/gates/knob_default_coupling.rs:353")],
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
        MANIFEST_ROOTS,
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
        MANIFEST_ROOTS,
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
        COMMENT_SURFACE_ROOTS,
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
        SPEC_POINTER_ROOTS,
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
        COMMENT_SURFACE_ROOTS,
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
        &[(".", "name:knob:CANON_KIT_AMENDMENT_GLOB", "", "")],
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
        &[(".", "name:knob:CANON_KIT_AMENDMENT_GLOB", "", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_AMENDMENT_GLOB",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the amendment finder is the
    // walk behind the cohort's `?`; the `git ls-files` corpus adds the dependency, not a root
    (
        "check-amendment-retired-spelling",
        amendment_retired_spelling::run,
        &[(".", "name:knob:CANON_KIT_AMENDMENT_GLOB", "", "")],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_AMENDMENT_GLOB",
            "CANON_KIT_RETIRED_SPELLING_EXCLUDE",
        ],
        "canon-kit",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the marker vocabulary is a consumer array
    // joined into an alternation and interpreted, so this member compiles it through the engine
    (
        "check-deprecation-task",
        deprecation_task::run,
        COMMENT_SURFACE_ROOTS,
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
        &[(".", "glob:knob:QUEUE_KIT_PROSE_SURFACE_GLOBS", "", "")],
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
        &[("?", "", "", "dynamic@src/gates/test_hermetic.rs:42")],
        &["GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[],
    ),
    (
        "check-assertion-strength",
        assertion_strength::run,
        &[("?", "", "", "dynamic@src/gates/assertion_strength.rs:375")],
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
        &[("?", "", "", "dynamic@src/gates/template_registry_parity.rs:89")],
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
        &[("?", "", "", "dynamic@src/gates/stage_skill_coverage.rs:79")],
        &["LIFECYCLE_KIT_SKILLS_DIR", "LIFECYCLE_KIT_STAGES"],
        "lifecycle-kit",
        &[],
    ),
    (
        "check-skill-binding",
        skill_binding::run,
        &[("?", "", "", "dynamic@src/gates/skill_binding.rs:97")],
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
        &[(".", "glob:knob:EVIDENCE_KIT_SCENARIO_GLOBS", "", "")],
        &[
            "EVIDENCE_KIT_BASELINE_FILE",
            "EVIDENCE_KIT_QUEUE_FILE",
            "EVIDENCE_KIT_SCENARIO_GLOBS",
            "EVIDENCE_KIT_PERMANENT_SLUGS",
            "EVIDENCE_KIT_SUITES",
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
            (".", "name:knob:LIFECYCLE_KIT_ROSTER_BASENAME", "", ""),
            (".", "name:knob:LIFECYCLE_KIT_AMENDMENT_GLOB", "", ""),
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
    // spec: gate-sdk/SPEC.md §The twelfth cohort — the surface glob hangs off a base that is this
    // member's own first argument with a git-toplevel default, so it takes the undecidable marker;
    // the workflow directory's listing is single-level, outside the analyzed class.
    (
        "check-close-surfaces",
        close_surfaces::run,
        &[("?", "", "", "dynamic@src/emit/close_surfaces.rs:135 via emit::close_surfaces::derive")],
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
        // spec: lifecycle-kit/SPEC.md §check-stage-evidence — the boundary knobs and the valve
        // ledger ride because the purity assertion's exemption set is derived from them
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
            "LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE",
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
        &[
            ("?", "", "", "dynamic@src/gates/shim_restatement.rs:122"),
            ("?", "", "", "dynamic@src/gates/shim_restatement.rs:161"),
        ],
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
        &[(".", "glob:knob:LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS", "", "")],
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
        &[("docs/posts", "glob:lit:*.md", "", "")],
        &[],
        "-",
        &[("git", "")],
    ),
    (
        "check-tightened-gates-grammar",
        tightened_gates_grammar::run,
        &[("docs/posts", "glob:lit:*.md", "", "")],
        &[],
        "-",
        &[],
    ),
    (
        "check-tightened-gates-note-parity",
        tightened_gates_note_parity::run,
        &[("docs/posts", "glob:lit:*.md", "", "")],
        &[],
        "-",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the rest of the consumer's own
    // gates directory, every member on the `-` sentinel. The two that declare a knob declare
    // what they execute: a shared derivation's whole knob set.
    // spec: gate-sdk/SPEC.md §check-reads-couples — a declared root, not `?`: the index glob hangs
    // off the directory of a positional whose default is a literal, so the root is decidable.
    (
        "check-docs-kit-parity",
        docs_kit_parity::run,
        &[("docs", "glob:lit:*/index.md", "", "")],
        &[
            "GATE_KIT_ROOTS_REL",
            "GATE_SDK_REGISTRY_DOC",
            "GATE_SDK_RUNNER_DOC",
        ],
        "-",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — a declared root where the walk hangs off a
    // positional with a literal default, and an empty set for the members that read named files
    // and list nothing; the mirror emitter's kit listing is single-level, outside the class.
    (
        "check-docs-mirror-fresh",
        docs_mirror_fresh::run,
        &[("docs", "name:lit:SPEC.md,README.md,DOCTRINE.md", "", "")],
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
        &[("docs", "name:lit:*.md", "", "")],
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
    // spec: gate-sdk/SPEC.md §The port-candidate criteria — arm D counts the registry members
    // that dispatch to the binary, so it reads the gates dir and the kit roots the resolve set is
    // built from; both operands stay positional and neither is a knob.
    (
        "check-install-platforms",
        install_platforms::run,
        &[],
        &["GATE_SDK_GATES_DIR", "GATE_KIT_ROOTS_REL"],
        "-",
        &[],
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
        &[(".github/workflows", "glob:lit:*.yml,*.yaml", "", "")],
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
        &[("?", "", "", "dynamic@src/emit/enforcement_map.rs:336 via emit::value_rollup::emit")],
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
        &[("?", "", "", "dynamic@src/emit/enforcement_map.rs:336 via emit::enforcement_map::emit")],
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
            "CONTEXT_KIT_BREVITY_SECTIONS",
            "CONTEXT_KIT_BREVITY_POINTER_RE",
        ],
        "context-kit",
        &[("git", "")],
    ),
    // spec: context-kit/SPEC.md §The surface ratchet — born native beside the meter whose writer
    // it reads: named files and one `git ls-files` over the ratchet pathspecs, so the declared
    // walk-root set is empty and `git` is the one declared program.
    (
        "check-surface-ratchet",
        surface_ratchet::run,
        &[],
        &[
            "CONTEXT_KIT_CEILING_FILE",
            "CONTEXT_KIT_SURFACES",
            "CONTEXT_KIT_RATCHET_PATHS",
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
    // budget batch's remaining four members: a `?` for a positional scan root the shell parser calls
    // undecidable, an empty set for named-file readers and for a `git ls-files` mode-bit reader.
    (
        "check-hook-exec-bit",
        hook_exec_bit::run,
        &[],
        &["GATE_SDK_HOOKS_DIR"],
        "gate-sdk",
        &[("git", "")],
    ),
    (
        "check-agent-tier-explicit",
        agent_tier_explicit::run,
        &[("?", "", "", "dynamic@src/gates/agent_tier_explicit.rs:59")],
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
        &[],
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
        &[("?", "", "", "dynamic@src/gates/readme_roster.rs:101")],
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
        &[("?", "", "", "dynamic@src/gates/memory_off.rs:116")],
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
    // spec: gate-sdk/SPEC.md §gen-pre-commit — `bash` is the whole declared requirement: assertion
    // D's two generator arms are this member's only spawns, measured, and the generator's own
    // programs ride the floor behind them rather than joining this element.
    (
        "check-graph",
        graph::run,
        &[(".", "name:lit:SPEC-*.md", "", "")],
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
            crate::registry::EVERY_COUPLES_KNOB,
        ],
        "gate-sdk",
        &[("bash", "")],
    ),
    (
        "check-gate-fail-closed",
        gate_fail_closed::run,
        &[],
        &["GATE_SDK_GATES_DIR", "GATE_KIT_ROOTS_HERE"],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — two unbounded roots: the
    // implementation tree assertion D sweeps, and the kit-root sweep assertion E runs once per
    // vendored root, whose count is a knob's value rather than a literal this entry could name.
    (
        "check-gate-substrate-parity",
        gate_substrate_parity::run,
        &[
            ("?", "", "", "dynamic@src/gates/gate_substrate_parity.rs:701"),
            ("?", "", "", "dynamic@src/gates/gate_substrate_parity.rs:726"),
        ],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_SDK_ROOT_HERE",
            "GATE_KIT_ROOTS_REL",
            "GATE_SDK_NATIVE_CRATE",
            "GATE_SDK_NATIVE_SRC",
            "GATE_SDK_NATIVE_TARGETS_FILE",
            "GATE_SDK_NATIVE_PUBLISH_WORKFLOW",
            "GATE_PRUNE_DIRS",
            crate::registry::EVERY_COUPLES_KNOB,
        ],
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
    // spec: gate-sdk/SPEC.md §check-crate-arms — no walk root: the corpus is a crate cargo is
    // handed by manifest path, and the tracked-source stamp it caches on comes from git rather
    // than from a walk this crate performs.
    // spec: gate-sdk/SPEC.md §check-crate-arms — three declared programs where criterion 7's
    // report counts two, because `git` reaches this member through the shared source-stamp
    // helper and sits on the program floor.
    (
        "check-crate-arms",
        crate_arms::run,
        &[],
        &[
            "GATE_SDK_NATIVE_CRATE",
            "GATE_SDK_CARGO_TARGET_DIR",
            "GATE_SDK_TMP_DIR",
        ],
        "gate-sdk",
        &[("cargo", ""), ("git", ""), ("rustc", "")],
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
    // spec: gate-sdk/SPEC.md §check-shellcheck — an empty root set: each derived directory is
    // expanded one level for `*.sh`, a single-level listing outside §check-reads-couples' analyzed
    // class. `shellcheck` is the declared dependency criterion 7's wrapper ruling
    // keeps: the program is the rule, so it stays off the payload and on the floor of what the
    // member refuses without.
    (
        "check-shellcheck",
        shellcheck::run,
        &[],
        &[
            "GATE_SDK_GATES_DIR",
            "GATE_KIT_ROOTS_HERE",
            "GATE_LINT_EXTRA_DIRS",
        ],
        "gate-sdk",
        &[("shellcheck", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-install-disposition — one `?` for the one root per kit root,
    // a set the registry cannot name concretely; `git` for the toplevel the positional root falls
    // back to, which the fixture cases pass their own root instead of reaching.
    (
        "check-install-disposition",
        install_disposition::run,
        &[("?", "", "", "dynamic@src/gates/install_disposition.rs:153")],
        &["GATE_KIT_ROOTS_HERE"],
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
        "check-docs-render-fidelity",
        docs_render_fidelity::run,
        &[],
        &[
            "SITE_KIT_DOCS_DIR",
            "SITE_KIT_RENDERER",
            "SITE_KIT_RENDERER_BATCH",
            "GATE_PRUNE_DIRS",
        ],
        "site-kit",
        &[
            ("git", ""),
            ("?", "SITE_KIT_RENDERER_BATCH"),
            ("?", "SITE_KIT_RENDERER"),
        ],
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
        &[("?", "", "", "dynamic@src/gates/docs_link_convention.rs:132")],
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
    // spec: gate-sdk/SPEC.md §check-portability-floor — no walk root: the corpus is
    // `git ls-files` over the configured pathspecs, which §check-reads-couples rules outside the
    // walk class, and a pathspec roster is a knob's value rather than a root a registry can name.
    // spec: gate-sdk/SPEC.md §check-portability-floor — the knob pair is this member's alone,
    // deliberately not the leak ban's
    (
        "check-portability-floor",
        portability_floor::run,
        &[],
        &["GATE_PORTABILITY_PATTERNS", "GATE_PORTABILITY_PATHS"],
        "gate-sdk",
        &[("git", "")],
    ),
    // spec: canon-kit/SPEC.md §check-surface-duplication — the one walk is the canonical-spec find,
    // so the bounding knob is the spec name and **not** `CANON_KIT_DUP_SURFACES`, whose members this
    // member reads as named files rather than walking for
    (
        "check-surface-duplication",
        surface_duplication::run,
        &[(".", "name:knob:CANON_KIT_SPEC_NAME", "**/templates,docs/*", "")],
        &[
            "CANON_KIT_GLOSSARY_FILE",
            "CANON_KIT_DUP_SURFACES",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "GATE_KIT_ROOTS_HERE",
            "GATE_PRUNE_DIRS",
        ],
        "canon-kit",
        &[],
    ),
    // spec: evidence-kit/SPEC.md §check-producer-liveness — no walk root: both modes read named
    // files, set mode's `*.run` glob resolving a corpus rather than reading one. `bash` carries the
    // `kill -0` builtin and is on the floor; `ps` is the fallback leg's, and the one the report counts.
    (
        "check-producer-liveness",
        producer_liveness::run,
        &[],
        &["EVIDENCE_KIT_LOCK_FILE"],
        "evidence-kit",
        &[("bash", ""), ("ps", "")],
    ),
    // spec: gate-sdk/SPEC.md §check-path-dialect — one unbounded root, the crate source tree,
    // whose location is a knob's value rather than a literal this entry could name. The shell
    // corpus is `git ls-files`, which §check-reads-couples rules outside the walk class.
    (
        "check-path-dialect",
        path_dialect::run,
        &[("?", "", "", "dynamic@src/gates/path_dialect.rs:350")],
        &["GATE_SDK_NATIVE_SRC", "GATE_PRUNE_DIRS"],
        "gate-sdk",
        &[("git", "")],
    ),
];

// spec: gate-sdk/SPEC.md §check-reads-couples — the three ground classes, each named by what retires
// its `?`; a fourth is refused, since a `?` on a literal or literal-default root whose filter the
// field can express declares the root instead
pub const GROUND_CLASSES: &[&str] = &["fallback", "dynamic", "projection"];

const fn ground_opens_with(ground: &str, class: &str) -> bool {
    let (g, c) = (ground.as_bytes(), class.as_bytes());
    if g.len() <= c.len() || g[c.len()] != b'@' {
        return false;
    }
    let mut i = 0;
    while i < c.len() {
        if g[i] != c[i] {
            return false;
        }
        i += 1;
    }
    true
}

// spec: gate-sdk/SPEC.md §check-reads-couples — a `?` carries a classed ground and no filter or
// prune, which is what lets `--reads` print the ground in the second column; any other root carries
// no ground. Where the locator points is the registry unit tests' to hold, not the compiler's.
const fn root_declaration_holds(root: &str, filter: &str, prune: &str, ground: &str) -> bool {
    let undecidable = root.len() == 1 && root.as_bytes()[0] == b'?';
    if !undecidable {
        return ground.is_empty();
    }
    if !filter.is_empty() || !prune.is_empty() {
        return false;
    }
    let mut k = 0;
    while k < GROUND_CLASSES.len() {
        if ground_opens_with(ground, GROUND_CLASSES[k]) {
            return true;
        }
        k += 1;
    }
    false
}

const _: () = {
    let mut i = 0;
    while i < REGISTRY.len() {
        let roots = REGISTRY[i].2;
        let mut j = 0;
        while j < roots.len() {
            let (r, f, p, g) = roots[j];
            assert!(
                root_declaration_holds(r, f, p, g),
                "a `?` walk root needs a ground `<class>@<path>:<line>`, the class one of `fallback`, \
                 `dynamic` or `projection`, and no filter or prune, and any other root carries no ground \
                 (gate-sdk/SPEC.md §check-reads-couples)"
            );
            j += 1;
        }
        i += 1;
    }
};

pub fn lookup(name: &str) -> Option<GateFn> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _, _)| *n == name)
        .map(|(_, f, _, _, _, _)| *f)
}

pub fn roots(name: &str) -> Option<&'static [RootDecl]> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _, _)| *n == name)
        .map(|(_, _, r, _, _, _)| *r)
}

// spec: gate-sdk/SPEC.md §check-reads-couples — the optional kind prefix a declared filter field
// carries, which is what makes the interpretation the member's own walker's: the reader stands for a
// walk it cannot see, so the walker's matching discipline travels on the field or nowhere.
pub const FILTER_KINDS: &[&str] = &["glob:", "name:", "ext:"];

// spec: gate-sdk/SPEC.md §check-reads-couples — the else-guard prefix: `else:<SELECTOR>:` marks a
// declaration the walk takes only where `<SELECTOR>` resolves empty. It names the knob and never the
// knob's state, so a kit descriptor carries no consumer's configuration.
pub const FILTER_ELSE: &str = "else:";

// spec: gate-sdk/SPEC.md §check-reads-couples — a declaration's guard, and the field without it: the
// selector knob whose emptiness selects this branch, `None` for an unguarded declaration
pub fn filter_guard(spec: &str) -> (Option<&str>, &str) {
    match spec.strip_prefix(FILTER_ELSE).and_then(|r| r.split_once(':')) {
        Some((sel, rest)) => (Some(sel), rest),
        None => (None, spec),
    }
}

// spec: gate-sdk/SPEC.md §check-reads-couples — the field with its guard and kind prefixes removed. A
// field carrying neither keeps its original meaning, a basename pattern, so every pre-existing
// declaration reads exactly as it did.
pub fn filter_source(spec: &str) -> &str {
    let (_, rest) = filter_guard(spec);
    FILTER_KINDS
        .iter()
        .find_map(|k| rest.strip_prefix(k))
        .unwrap_or(rest)
}

// spec: gate-sdk/SPEC.md §check-reads-couples — the knob a declared filter field names, read from
// here by both its readers: `None` for the two forms that name no knob, the omitted field (an
// unfiltered walk) and `lit:<list>` (a kit literal the crate owns at the walk site)
pub fn filter_knob(spec: &str) -> Option<&str> {
    let src = filter_source(spec);
    if src.is_empty() || src.starts_with("lit:") {
        return None;
    }
    Some(src.strip_prefix("knob:").unwrap_or(src))
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
        // spec: gate-sdk/SPEC.md §check-reads-couples — a guard's selector knob is read by the
        // resolver exactly as the filter knob is, so the union carries both or a guarded declaration
        // cannot be evaluated at all
        let mut filters: Vec<&'static str> = REGISTRY
            .iter()
            .flat_map(|(_, _, roots, _, _, _)| {
                roots
                    .iter()
                    .flat_map(|(_, f, _, _)| [filter_knob(f), filter_guard(f).0])
                    .flatten()
            })
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

    // spec: gate-sdk/SPEC.md §run-gate-tests — the case's own positional arguments, which are the
    // only anchors a case may relocate a declared walk to. A flag is not an anchor.
    fn relocations(args: &[String]) -> Vec<String> {
        args.iter()
            .filter(|a| !a.starts_with('-') && !a.is_empty())
            .cloned()
            .collect()
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — `declaration_covers` with the case's relocation
    // set: an observed root the case itself supplied as an anchor satisfies the declaration it
    // relocates, so the assertion keeps its force — an undeclared root no case argument names
    // still reds — without a `?` standing where a resolvable root could.
    // spec: gate-sdk/SPEC.md §check-reads-couples — the root/filter projection of a root declaration,
    // because the arity rule is about roots and the prune is a third dimension held separately
    fn two(declared: &[RootDecl]) -> Vec<(&'static str, &'static str)> {
        declared.iter().map(|(r, f, _, _)| (*r, *f)).collect()
    }

    fn declaration_covers_relocated(
        verb: &str,
        declared: &[RootDecl],
        observed: &[String],
        relocations: &[String],
    ) -> Result<(), String> {
        if declared.iter().all(|(d, _, _, _)| *d != "?") && !relocations.is_empty() {
            let kept: Vec<String> = observed
                .iter()
                .filter(|o| {
                    !relocations
                        .iter()
                        .any(|r| *o == r || o.starts_with(&format!("{}/", r)))
                })
                .cloned()
                .collect();
            return declaration_covers(verb, &two(declared), &kept);
        }
        declaration_covers(verb, &two(declared), observed)
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
        let mut prunes_held = 0usize;
        for (name, _, declared, _, _, _) in REGISTRY {
            // spec: gate-sdk/SPEC.md §check-reads-couples — resolved through `knobs`, not off the
            // tuple, so a member declaring the union sentinel is bridged the expansion the
            // dispatcher would bridge rather than the sentinel itself
            let member_knobs = knobs(name).unwrap_or(&[]);
            for case in walk::fixture_case_dirs(name) {
                // spec: gate-sdk/SPEC.md §run-gate-tests — the member's knobs are bridged from
                // the case dir before it runs, or a bridged member exits 2 on an unresolved
                // knob and this test asserts over a run that never reached its rule
                walk::bridge_case_knobs(&env, &case, name, member_knobs);
                // spec: gate-sdk/SPEC.md §check-reads-couples — the case is set on the observer's
                // spawn as the --run-gate-tests arm sets it, so an observed root is the same string
                // the gate would walk from the repo root in the battery.
                let run = observe_in_case(name, &case);
                assert_ne!(
                    run.rc, 2,
                    "{} errored on {} — an observation taken from a run that never walked \
                     would pass this test by being empty",
                    name,
                    case.display()
                );
                // spec: gate-sdk/SPEC.md §check-reads-couples — a declared root names the *deployed*
                // invocation's root, which the generated hook dispatches with no trailing argv; a
                // case passing a positional relocates that same walk to a case-local anchor
                let relocated = relocations(&case_args(&case));
                // spec: gate-sdk/SPEC.md §The path-dialect contract — `./docs` and `docs` are one
                // root, so both sides of the comparison are spelled the way a declaration spells it
                let walked: Vec<String> =
                    run.walked.iter().map(|r| normalize_root(r)).collect();
                if let Err(e) =
                    declaration_covers_relocated("walked", declared, &walked, &relocated)
                {
                    panic!("{} on {}: {}", name, case.display(), e);
                }
                // spec: gate-sdk/SPEC.md §check-reads-couples — the prune half runs in the inverse
                // direction to the roots': a declared prune narrows the demand, so the risk is
                // declaring one the walk does not apply — **declared ⊆ observed**
                for (r, fspec, pspec, _) in declared.iter() {
                    // spec: gate-sdk/SPEC.md §check-reads-couples — a guarded branch the case's own
                    // configuration does not select describes a walk that did not run, so it applied
                    // no prune. The resolver skips such a declaration for the same reason.
                    if filter_guard(fspec)
                        .0
                        .is_some_and(|sel| {
                            std::env::var(format!("GATE_SDK_KNOB_{}", sel))
                                .is_ok_and(|v| !v.is_empty())
                        })
                    {
                        continue;
                    }
                    // spec: gate-sdk/SPEC.md §check-reads-couples — and a case that ran the walk is
                    // the only case that can hold its prune: an invocation taking a file list instead
                    // of a root walks nothing, so there is nothing to be held to
                    if !walked.iter().any(|o| o == r) {
                        continue;
                    }
                    for g in pspec.split(',').filter(|g| !g.is_empty()) {
                        prunes_held += 1;
                        assert!(
                            run.pruned.iter().any(|o| o == g),
                            "{} on {} declares root '{}' pruning '{}', but its walk applied no such \
                             prune — a declared prune narrows the coverage demand, so an unapplied \
                             one hides exactly the gap this gate exists to catch (observed: {:?})",
                            name,
                            case.display(),
                            r,
                            g,
                            run.pruned
                        );
                    }
                }
                cases_run += 1;
                roots_observed += run.walked.len();
            }
        }
        assert!(cases_run > 0, "no fixture case found for any registry member");
        assert!(
            roots_observed > 0,
            "no member walked anything — the subset assertion above held over nothing"
        );
        // spec: gate-sdk/SPEC.md §check-reads-couples — the prune half cannot pass by holding over
        // nothing: a declared prune is the one form here whose failure direction is under-demand
        assert!(
            prunes_held > 0,
            "no declared prune was held to an executed walk — read the prune half as unverified \
             rather than as clean, because an unheld narrowing is what it exists to refuse"
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
        for (name, _, _, _, _, declared) in REGISTRY {
            let member_knobs = knobs(name).unwrap_or(&[]);
            for case in walk::fixture_case_dirs(name) {
                walk::bridge_case_knobs(&env, &case, name, member_knobs);
                let run = observe_in_case(name, &case);
                assert_ne!(
                    run.rc, 2,
                    "{} errored on {} — an observation taken from a run that never spawned \
                     would pass this test by being empty",
                    name,
                    case.display()
                );
                if let Err(e) = declaration_covers("spawned", declared, &run.spawned) {
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

    const OBSERVER_MARKER: &str = "CHECKWRIGHT_REGISTRY_OBSERVER";
    const OBSERVER_MEMBER: &str = "CHECKWRIGHT_REGISTRY_OBSERVER_MEMBER";
    const OBSERVER_SENTINEL: &str = "checkwright-registry-observer";

    struct Observation {
        rc: i32,
        walked: Vec<String>,
        spawned: Vec<String>,
        pruned: Vec<String>,
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — a case directory is set on a spawn and never entered,
    // so the member runs in a child of this test binary whose working directory is the case
    fn observe_in_case(name: &str, case: &std::path::Path) -> Observation {
        let exe = std::env::current_exe().expect("a running test has an executable");
        let exe = exe.display().to_string();
        let module = module_path!()
            .split_once("::")
            .map_or(module_path!(), |(_, rest)| rest);
        let observer = format!("{}::a_registry_member_run_inside_its_case_dir", module);
        let env = vec![
            (OBSERVER_MARKER.to_string(), "1".to_string()),
            (OBSERVER_MEMBER.to_string(), name.to_string()),
        ];
        let merged = crate::proc::run_merged_in(
            &exe,
            &[&observer, "--exact", "--ignored", "--nocapture"],
            &env,
            Some(case),
        )
        .unwrap_or_else(|e| panic!("{} on {}: {}", name, case.display(), e));
        let output = String::from_utf8_lossy(merged.output());
        assert_eq!(
            merged.code(),
            Some(0),
            "the observer child for {} on {} did not pass:\n{}",
            name,
            case.display(),
            output
        );
        let prefix = format!("{}\t", OBSERVER_SENTINEL);
        let mut rcs: Vec<i32> = Vec::new();
        let mut walked: Vec<String> = Vec::new();
        let mut spawned: Vec<String> = Vec::new();
        let mut pruned: Vec<String> = Vec::new();
        for line in output.lines() {
            let Some(rest) = line.strip_prefix(&prefix) else {
                continue;
            };
            match rest.split_once('\t') {
                Some(("walked", root)) => walked.push(root.to_string()),
                Some(("spawned", program)) => spawned.push(program.to_string()),
                Some(("pruned", glob)) => pruned.push(glob.to_string()),
                Some(("exit", code)) => rcs.push(
                    code.parse()
                        .unwrap_or_else(|_| panic!("unparseable exit line: {}", line)),
                ),
                _ => panic!("unparseable observation line: {}", line),
            }
        }
        // spec: gate-sdk/SPEC.md §lib/gate.sh — a filter matching no test exits 0 and prints
        // nothing, so the exit line is what proves the observer ran at all
        assert_eq!(
            rcs.len(),
            1,
            "the observer child for {} on {} printed {} exit lines, not one:\n{}",
            name,
            case.display(),
            rcs.len(),
            output
        );
        Observation {
            rc: rcs[0],
            walked,
            spawned,
            pruned,
        }
    }

    // spec: gate-sdk/SPEC.md §lib/gate.sh — the child `observe_in_case` spawns inside one fixture
    // case; ignored so a plain run skips it, and refusing without the marker so a run including
    // ignored cases cannot execute it in the test binary's own working directory
    #[test]
    #[ignore = "the registry coverage tests' child, run inside a fixture case"]
    fn a_registry_member_run_inside_its_case_dir() {
        assert!(
            std::env::var_os(OBSERVER_MARKER).is_some(),
            "this case runs only as the registry coverage tests' child — in-process it would \
             observe a member in the test binary's working directory rather than a fixture case"
        );
        let name = std::env::var(OBSERVER_MEMBER).expect("the parent names the member to run");
        let entry = REGISTRY
            .iter()
            .find(|entry| entry.0 == name)
            .unwrap_or_else(|| panic!("no registry member named {}", name));
        let args = case_args(std::path::Path::new("."));
        walk::recorder::start();
        crate::proc::recorder::start();
        let rc = (entry.1)(&args);
        let spawned = crate::proc::recorder::stop();
        let pruned = walk::recorder::stop_prunes();
        let walked = walk::recorder::stop();
        // spec: gate-sdk/SPEC.md §lib/gate.sh — the block opens on a newline, so a member's
        // unterminated last line cannot absorb the first sentinel
        let mut block = String::from("\n");
        for root in walked {
            block.push_str(&format!("{}\twalked\t{}\n", OBSERVER_SENTINEL, root));
        }
        for program in spawned {
            block.push_str(&format!("{}\tspawned\t{}\n", OBSERVER_SENTINEL, program));
        }
        for glob in pruned {
            block.push_str(&format!("{}\tpruned\t{}\n", OBSERVER_SENTINEL, glob));
        }
        block.push_str(&format!("{}\texit\t{}\n", OBSERVER_SENTINEL, rc));
        use std::io::Write;
        let mut out = std::io::stdout().lock();
        out.write_all(block.as_bytes())
            .and_then(|_| out.flush())
            .expect("cannot write the observation block");
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

    // spec: gate-sdk/SPEC.md §check-reads-couples — every entry point whose *first* argument is a
    // **recursive** walk's root. `list_dir` is absent for the reason it is absent from the recorder:
    // the invariant is over a recursive walk and a single-level listing descends nothing.
    const ROOT_ENTRY_POINTS: &[&str] = &[
        "find_files",
        "find_named",
        "find_with_prune",
        "find_link_entries_with_prune",
        "glob_files",
        "manifest_files",
        "manifest_files_sorted_stripped",
        "canonical_specs",
        "canonical_specs_sorted",
        "amendments",
        "amendments_strict",
        "comment_surface",
        "governed_docs",
        "prune_kit_roots",
    ];

    // spec: gate-sdk/SPEC.md §check-reads-couples — value-preserving wrappers a root passes through
    const ROOT_WRAPPERS: &[&str] = &[
        "Path::new",
        "PathBuf::from",
        "fresh::strip_trailing_slash",
        "strip_trailing_slash",
        "spec::strip_dot_slash",
        "strip_dot_slash",
    ];

    // spec: gate-sdk/SPEC.md §check-reads-couples — value-preserving method tails, same reason
    const ROOT_TAILS: &[&str] = &[
        ".as_str()",
        ".to_string()",
        ".clone()",
        ".as_path()",
        ".display().to_string()",
        ".trim_end_matches('/')",
        ".trim_start_matches(\"./\")",
        ".to_owned()",
    ];

    fn ident_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — one expression's text: up to the first top-level
    // comma, semicolon or unmatched closer, with string literals skipped so a comma inside one does
    // not cut it. Shared by the call-argument reader and the `let`-binding reader.
    fn first_arg(after_open: &str) -> Option<&str> {
        let b = after_open.as_bytes();
        let (mut depth, mut i, mut quote) = (0i32, 0usize, 0u8);
        while i < b.len() {
            let c = b[i];
            if quote != 0 {
                if c == b'\\' {
                    i += 2;
                    continue;
                }
                if c == quote {
                    quote = 0;
                }
                i += 1;
                continue;
            }
            match c {
                b'"' | b'\'' => quote = c,
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' => {
                    if depth == 0 {
                        return Some(&after_open[..i]);
                    }
                    depth -= 1;
                }
                b',' | b';' if depth == 0 => return Some(&after_open[..i]),
                _ => {}
            }
            i += 1;
        }
        None
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — a root spelled repo-relative the way a
    // declaration spells it, so a resolved root and a declared one compare by equality
    fn normalize_root(lit: &str) -> String {
        let t = lit.trim_start_matches("./").trim_end_matches('/');
        if t.is_empty() { ".".to_string() } else { t.to_string() }
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — the first `let <id> …= <expr>;` binding in the
    // module, at any depth: an identifier reaching a walk in one function is bound in another
    fn bindings_of<'a>(id: &str, text: &'a str) -> Vec<&'a str> {
        let mut out: Vec<&'a str> = Vec::new();
        for pat in [format!("let {} ", id), format!("let mut {} ", id), format!("let {}:", id)] {
            let mut from = 0usize;
            while let Some(at) = text[from..].find(&pat) {
                let start = from + at;
                from = start + pat.len();
                let Some(eq) = text[start..].find('=') else { continue };
                let rest = &text[start + eq + 1..];
                if let Some(end) = first_arg(rest) {
                    out.push(end);
                }
            }
        }
        out
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — delta 1's predicate implemented as a *property*
    // test: does this expression resolve to a literal path without running the gate? Every form it
    // cannot evaluate answers `None`, which is why the assertion below cannot false-fire upward.
    fn resolve_static(expr: &str, text: &str, depth: usize) -> Option<String> {
        if depth > 8 {
            return None;
        }
        // spec: gate-sdk/SPEC.md §check-reads-couples — a method chain broken across lines is one
        // expression, so the space a line break leaves before its `.` is removed rather than kept
        let flat = expr.split_whitespace().collect::<Vec<_>>().join(" ").replace(" .", ".");
        let mut e = flat.trim().to_string();
        while e.starts_with('&') || e.starts_with('*') {
            e = e[1..].trim().to_string();
        }
        e = e.trim_end_matches('?').trim().to_string();
        if e.len() >= 2 && e.starts_with('"') && e.ends_with('"') && !e[1..e.len() - 1].contains('"')
        {
            return Some(e[1..e.len() - 1].to_string());
        }
        for w in ROOT_WRAPPERS {
            if let Some(rest) = e.strip_prefix(&format!("{}(", w)) {
                return first_arg(rest).and_then(|a| resolve_static(a, text, depth + 1));
            }
        }
        for t in ROOT_TAILS {
            if let Some(head) = e.strip_suffix(t) {
                return resolve_static(head, text, depth + 1);
            }
        }
        // spec: gate-sdk/SPEC.md §check-reads-couples — the positional-root helper and the four
        // hand-spelled argv idioms beside it: an argument whose default is a literal resolves to
        // that literal, because the generated hook dispatches with no trailing argv
        for p in ["fresh::positional(", "positional("] {
            if let Some(rest) = e.strip_prefix(p) {
                let mut args = rest;
                for _ in 0..2 {
                    let a = first_arg(args)?;
                    args = &args[a.len() + 1..];
                }
                return first_arg(args).and_then(|d| resolve_static(d, text, depth + 1));
            }
        }
        if e.contains("args.") || e.starts_with("match ") {
            for tail in ["unwrap_or(", "unwrap_or_else(|| "] {
                if let Some(at) = e.rfind(tail) {
                    let inner = &e[at + tail.len()..];
                    if let Some(a) = first_arg(inner) {
                        if let Some(r) = resolve_static(a, text, depth + 1) {
                            return Some(r);
                        }
                    }
                }
            }
            if let Some(at) = e.find("None =>") {
                let arm = &e[at + "None =>".len()..];
                if let Some(a) = first_arg(arm) {
                    if let Some(r) = resolve_static(a, text, depth + 1) {
                        return Some(r);
                    }
                }
            }
            return None;
        }
        if let Some(rest) = e.strip_prefix("format!(") {
            return resolve_format(rest, text, depth);
        }
        if !e.is_empty() && e.chars().all(ident_char) {
            if let Some(c) = const_literal(&e, text) {
                return Some(c);
            }
            for b in bindings_of(&e, text) {
                if let Some(r) = resolve_static(b, text, depth + 1) {
                    return Some(r);
                }
            }
        }
        None
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — a `const NAME: &str = "…";` the crate owns, which
    // is as statically known as an inline literal
    fn const_literal(name: &str, text: &str) -> Option<String> {
        let at = text.find(&format!("const {}:", name))?;
        let eq = text[at..].find('=')?;
        let arg = first_arg(&text[at + eq + 1..])?;
        let t = arg.trim();
        if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') {
            return Some(t[1..t.len() - 1].to_string());
        }
        None
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — a root composed by `format!` resolves only when
    // every operand does, which is what keeps a knob-valued path component out of the population
    fn resolve_format(rest: &str, text: &str, depth: usize) -> Option<String> {
        let tmpl = first_arg(rest)?.trim().to_string();
        if !(tmpl.starts_with('"') && tmpl.ends_with('"')) {
            return None;
        }
        let mut args = &rest[tmpl.len() + 1..];
        let mut parts: Vec<String> = Vec::new();
        while let Some(a) = first_arg(args) {
            if a.trim().is_empty() {
                break;
            }
            parts.push(resolve_static(a, text, depth + 1)?);
            if args.len() <= a.len() {
                break;
            }
            args = &args[a.len() + 1..];
        }
        let body = &tmpl[1..tmpl.len() - 1];
        let mut out = String::new();
        let mut used = 0usize;
        for (i, seg) in body.split("{}").enumerate() {
            if i > 0 {
                out.push_str(parts.get(used)?);
                used += 1;
            }
            out.push_str(seg);
        }
        Some(out)
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — the module's own rule text, with its `#[cfg(test)]`
    // block and its comments removed: a literal in either is not a root the deployed gate walks
    fn rule_text(src: &str) -> String {
        let body = match src.find("#[cfg(test)]\nmod tests {") {
            Some(at) => &src[..at],
            None => src,
        };
        body.lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — every statically resolvable walk root in one
    // module, per walk rather than per module: a member may walk several roots of different shapes
    fn static_roots(text: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for name in ROOT_ENTRY_POINTS {
            let needle = format!("{}(", name);
            let mut from = 0usize;
            while let Some(at) = text[from..].find(&needle) {
                let start = from + at;
                from = start + needle.len();
                if start > 0 && text[..start].chars().next_back().is_some_and(ident_char) {
                    continue;
                }
                let Some(arg) = first_arg(&text[from..]) else { continue };
                if let Some(lit) = resolve_static(arg, text, 0) {
                    let r = normalize_root(&lit);
                    if !out.contains(&r) {
                        out.push(r);
                    }
                }
            }
        }
        out.sort();
        out
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test C, the refusal: `?` is not an available
    // answer for a walk whose root resolves statically. Per walk, never per member — a member with
    // one resolvable and one unresolvable walk declares the first and keeps `?` for the second.
    #[test]
    fn no_member_answers_a_statically_resolvable_walk_root_with_a_question_mark() {
        assert!(!REGISTRY.is_empty(), "no member to assert over");
        let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/gates");
        let mut offenders: Vec<String> = Vec::new();
        let mut resolved = 0usize;
        for (name, _, declared, _, _, _) in REGISTRY {
            let module = src_dir.join(format!("{}.rs", name.trim_start_matches("check-").replace('-', "_")));
            let raw = std::fs::read_to_string(&module)
                .unwrap_or_else(|e| panic!("cannot read {}: {}", module.display(), e));
            for root in static_roots(&rule_text(&raw)) {
                resolved += 1;
                if !declared.iter().any(|(d, _, _, _)| *d == root) {
                    offenders.push(format!(
                        "{} walks the statically resolvable root '{}' but its registry entry \
                         declares {:?}",
                        name, root, declared
                    ));
                }
            }
        }
        assert!(
            resolved > 0,
            "no module resolved a static root — the refusal below held over nothing, so read it \
             as unverified rather than as clean"
        );
        assert!(
            offenders.is_empty(),
            "'?' means a root that cannot be bounded statically, and a resolvable root claiming it \
             is the foreclosed opt-out moved into the registry — declare the root (and the knob or \
             kit literal that bounds its walk) instead:\n  {}",
            offenders.join("\n  ")
        );
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — one `?` declaration site: the shared root const
    // whose body holds it, else the home module its member's dispatch function lives in
    #[derive(Clone)]
    struct Site {
        line: usize,
        ground: String,
        konst: Option<String>,
        home: Option<String>,
    }

    #[derive(Debug, PartialEq)]
    enum Placement {
        InModule,
        OffModule,
    }

    fn crate_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — the sites are read off the registry's source text
    // rather than off `REGISTRY`, because a shared const is one site however many members use it and
    // the site is the unit a ground is authored at
    fn question_mark_sites(src: &str) -> Result<Vec<Site>, String> {
        const OPEN: &str = "(\"?\", \"\", \"\", \"";
        let body = src.find("#[cfg(test)]\nmod tests {").map_or(src, |at| &src[..at]);
        let (mut konst, mut home): (Option<String>, Option<String>) = (None, None);
        let mut out: Vec<Site> = Vec::new();
        for (i, line) in body.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("//") {
                continue;
            }
            if let Some((name, tail)) = t.strip_prefix("const ").and_then(|r| r.split_once(':')) {
                if tail.contains("RootDecl") {
                    konst = Some(name.to_string());
                }
            }
            if t == "];" {
                konst = None;
            }
            if let Some(m) = t.strip_suffix("::run,").filter(|m| m.chars().all(ident_char)) {
                home = Some(format!("src/gates/{}.rs", m));
            }
            let mut from = 0usize;
            while let Some(at) = line[from..].find(OPEN) {
                let start = from + at + OPEN.len();
                let end = start
                    + line[start..]
                        .find('"')
                        .ok_or_else(|| format!("mod.rs:{}: an unterminated ground", i + 1))?;
                out.push(Site {
                    line: i + 1,
                    ground: line[start..end].to_string(),
                    konst: konst.clone(),
                    home: if konst.is_some() { None } else { home.clone() },
                });
                from = end;
            }
        }
        Ok(out)
    }

    fn preceded_by_ident(text: &str, at: usize) -> bool {
        text[..at].chars().next_back().is_some_and(ident_char)
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — a call of a root-entry roster member, never its
    // definition and never a comment
    fn calls_root_entry_point(line: &str) -> bool {
        if line.trim_start().starts_with("//") {
            return false;
        }
        ROOT_ENTRY_POINTS.iter().any(|name| {
            let needle = format!("{}(", name);
            line.match_indices(&needle).any(|(at, _)| {
                !preceded_by_ident(line, at) && !line[..at].trim_end().ends_with("fn")
            })
        })
    }

    fn names_symbol(text: &str, symbol: &str) -> bool {
        text.match_indices(symbol).any(|(at, _)| {
            !preceded_by_ident(text, at)
                && !text[at + symbol.len()..].chars().next().is_some_and(ident_char)
        })
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — the `via` clause's first hop and no further: the
    // symbol resolves, and the home module names it; the chain from it to the walk line is recorded,
    // never proven
    fn via_verdict(site: &Site, via: &str, root: &std::path::Path) -> Result<(), String> {
        if let Some(name) = via.strip_prefix("const ") {
            return match &site.konst {
                Some(k) if k == name => Ok(()),
                _ => Err(format!("`via const {}` at a site outside that const's body", name)),
            };
        }
        let (Some(home), None) = (&site.home, &site.konst) else {
            return Err(format!(
                "a site inside const {} takes `via const {0}`, not `via {}`",
                site.konst.as_deref().unwrap_or("?"),
                via
            ));
        };
        let well_formed = |s: &str| !s.is_empty() && s.chars().all(ident_char);
        let (module, func) = via
            .rsplit_once("::")
            .filter(|(m, f)| m.split("::").all(well_formed) && well_formed(f))
            .ok_or_else(|| format!("`via {}` is not `<module>::<fn>` or `const <NAME>`", via))?;
        let rel = module.replace("::", "/");
        let defining = [format!("src/{}.rs", rel), format!("src/{}/mod.rs", rel)]
            .iter()
            .find_map(|p| std::fs::read_to_string(root.join(p)).ok())
            .unwrap_or_default();
        if !names_symbol(&defining, &format!("fn {}", func)) {
            return Err(format!("`via {}` has no definition in module {}", via, module));
        }
        let home_text = std::fs::read_to_string(root.join(home))
            .map_err(|e| format!("home module {} does not resolve: {}", home, e))?;
        let last = module.rsplit("::").next().unwrap_or(module);
        if !names_symbol(&home_text, &format!("{}::{}", last, func)) {
            return Err(format!("home module {} does not reference `via {}`", home, via));
        }
        Ok(())
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — presence and placement, which a gate can decide;
    // whether the class is the right one is not, and nothing here reads it past its spelling
    fn locator_verdict(site: &Site, root: &std::path::Path) -> Result<Placement, String> {
        let g = site.ground.as_str();
        let (class, rest) = g
            .split_once('@')
            .ok_or_else(|| format!("`{}` is not `<class>@<path>:<line>`", g))?;
        if !GROUND_CLASSES.contains(&class) {
            return Err(format!("`{}` is not a ground class ({:?})", class, GROUND_CLASSES));
        }
        let (loc, via) = match rest.split_once(" via ") {
            Some((l, v)) => (l, Some(v)),
            None => (rest, None),
        };
        let (path, n) = loc
            .rsplit_once(':')
            .and_then(|(p, n)| n.parse::<usize>().ok().map(|n| (p, n)))
            .ok_or_else(|| format!("`{}` is not `<path>:<line>`", loc))?;
        let text = std::fs::read_to_string(root.join(path))
            .map_err(|e| format!("{} does not resolve: {}", loc, e))?;
        let walk_line = text
            .lines()
            .nth(n.wrapping_sub(1))
            .ok_or_else(|| format!("{} does not resolve: {} has no line {}", loc, path, n))?;
        if !calls_root_entry_point(walk_line) {
            return Err(format!("{} calls no root-entry roster member: {}", loc, walk_line.trim()));
        }
        match (site.home.as_deref() == Some(path), via) {
            (true, None) => Ok(Placement::InModule),
            (true, Some(v)) => Err(format!("`via {}` is forbidden in-module", v)),
            (false, None) => Err(format!("{} is off its site's home module and carries no via clause", loc)),
            (false, Some(v)) => via_verdict(site, v, root).map(|_| Placement::OffModule),
        }
    }

    fn live_sites() -> Vec<Site> {
        let module = crate_root().join("src/gates/mod.rs");
        let src = std::fs::read_to_string(&module)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", module.display(), e));
        question_mark_sites(&src).unwrap_or_else(|e| panic!("{}", e))
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test D, the locator assertion: every `?` site
    // resolves on a line calling a root-entry roster member, its `via` clause present exactly where
    // the line lies off the site's home module. Its enumeration is what reports the site counts.
    #[test]
    fn every_question_mark_ground_locates_a_root_entry_call_at_its_placement() {
        let root = crate_root();
        let sites = live_sites();
        assert!(!sites.is_empty(), "no `?` declaration site found — the assertion held over nothing");
        let from_text: std::collections::BTreeSet<&str> =
            sites.iter().map(|s| s.ground.as_str()).collect();
        let from_registry: std::collections::BTreeSet<&str> = REGISTRY
            .iter()
            .flat_map(|(_, _, roots, _, _, _)| roots.iter())
            .filter(|(r, _, _, _)| *r == "?")
            .map(|(_, _, _, g)| *g)
            .collect();
        assert_eq!(
            from_text, from_registry,
            "the registry's `?` grounds and the sites read off its source disagree — a `?` spelled \
             other than `(\"?\", \"\", \"\", \"<ground>\")` on one line escapes the locator assertion"
        );
        let (mut in_module, mut off_module) = (0usize, 0usize);
        let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
        let mut offenders: Vec<String> = Vec::new();
        for s in &sites {
            *by_class.entry(s.ground.split('@').next().unwrap_or("")).or_default() += 1;
            match locator_verdict(s, &root) {
                Ok(Placement::InModule) => in_module += 1,
                Ok(Placement::OffModule) => off_module += 1,
                Err(e) => offenders.push(format!("mod.rs:{}: {} — {}", s.line, s.ground, e)),
            }
        }
        println!(
            "`?` declaration sites: {} ({} in-module, {} off-module); by class: {:?}",
            sites.len(),
            in_module,
            off_module,
            by_class
        );
        assert!(
            offenders.is_empty(),
            "a `?` ground's locator names where its walk is, and these do not (gate-sdk/SPEC.md \
             §check-reads-couples):\n  {}",
            offenders.join("\n  ")
        );
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — each bad kind seeded from a live site, so the
    // refusal is proven against the tree it guards rather than against a line number that moves
    #[test]
    fn a_seeded_bad_locator_of_each_kind_is_refused() {
        let root = crate_root();
        let sites = live_sites();
        let pick = |want: Placement, konst: bool| {
            sites
                .iter()
                .find(|s| s.konst.is_some() == konst && locator_verdict(s, &root).as_ref() == Ok(&want))
                .cloned()
                .expect("a live site of each placement")
        };
        let inm = pick(Placement::InModule, false);
        let off = pick(Placement::OffModule, false);
        let shared = pick(Placement::OffModule, true);
        let (inm_loc, _) = inm.ground.split_once(" via ").unwrap_or((&inm.ground, ""));
        let (off_loc, off_via) = off.ground.split_once(" via ").expect("an off-module ground has a via");
        let (_, shared_via) = shared.ground.split_once(" via ").expect("a const ground has a via");
        let (inm_path, _) = inm_loc.rsplit_once(':').expect("a live locator has a line");
        let (off_module, _) = off_via.rsplit_once("::").expect("an inline off-module via names a fn");
        let seeded = |base: &Site, ground: String, kind: &str| {
            let site = Site { ground: ground.clone(), ..base.clone() };
            match locator_verdict(&site, &root) {
                Ok(p) => panic!("seeded {} locator `{}` was accepted as {:?}", kind, ground, p),
                Err(e) => assert!(e.contains(kind), "seeded `{}` refused for the wrong reason: {}", ground, e),
            }
        };
        seeded(&inm, format!("{}:999999", inm_path), "does not resolve");
        seeded(&inm, format!("{}:1", inm_path), "calls no root-entry roster member");
        let (_, inm_rest) = inm_loc.split_once('@').expect("a live locator has a class");
        seeded(&inm, format!("static@{}", inm_rest), "not a ground class");
        seeded(&off, off_loc.to_string(), "carries no via clause");
        seeded(&inm, format!("{} via {}", inm_loc, off_via), "forbidden in-module");
        seeded(&off, format!("{} via {}::no_such_fn_seeded", off_loc, off_module), "has no definition");
        seeded(&inm, off.ground.clone(), "does not reference");
        seeded(&off, format!("{} via {}", off_loc, shared_via), "outside that const's body");
        seeded(&shared, format!("{} via {}", off_loc, off_via), "takes `via const");
    }
}
