// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one module per ported gate; the
// subcommand name is the gate name, so no mapping table exists to drift
pub mod action_gh_repo;
pub mod action_pinning;
pub mod docs_cmd;
pub mod knob_citation;
pub mod manifest_count;
pub mod md_refs;
pub mod measured_claim;
pub mod prose_enum;
pub mod queue_entry_budget;
pub mod queue_hygiene;
pub mod queue_sections;
pub mod queue_slug_liveness;
pub mod queue_wrap;
pub mod spec_fence_balance;
pub mod tag_lead_line;
pub mod task_conservation;
pub mod task_names;
pub mod tracking_claim;

pub type GateFn = fn(&[String]) -> i32;

// spec: gate-sdk/SPEC.md §check-reads-couples — the third element is the member's declared
// walk roots, the data `--reads` prints. A member added without them fails to compile, so
// the declaration cannot be silently omitted.
// spec: gate-sdk/SPEC.md §lib/gate.sh — the fourth element is the member's declared knob
// reads, the data `--knobs` prints and the config bridge resolves. Un-omittable by the same
// construction, so no member can read a knob the bridge was never asked to carry.
// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the fifth element is the owning kit's
// directory basename, `--list`'s second column, by which assertion B scopes its roster half to
// the kits a consumer vendored. Un-omittable by the same construction, and held to the tree by
// the unit test below rather than trusted as a self-declaration.
pub type GateEntry = (
    &'static str,
    GateFn,
    &'static [&'static str],
    &'static [&'static str],
    &'static str,
);

pub const REGISTRY: &[GateEntry] = &[
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because each member's scan root is
    // its own first argument with a default, the same variable-first-argument shape the
    // shell parser calls undecidable and skips-and-counts.
    (
        "check-action-pinning",
        action_pinning::run,
        &["?"],
        &["GATE_PRUNE_DIRS"],
        "gate-sdk",
    ),
    (
        "check-action-gh-repo",
        action_gh_repo::run,
        &["?"],
        &["GATE_PRUNE_DIRS"],
        "gate-sdk",
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
    ),
    (
        "check-queue-wrap",
        queue_wrap::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_WRAP_BUDGET"],
        "queue-kit",
    ),
    (
        "check-queue-hygiene",
        queue_hygiene::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_PROSE_LEADS"],
        "queue-kit",
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
    ),
    // spec: canon-kit/SPEC.md §lib/spec.sh — the canon-kit cohort's members all derive their
    // corpus from `spec::manifest_files`, so each declares that derivation's whole knob set
    // beside its own: the bridge carries what the shared derivation reads, not what the
    // member's own rule reads.
    (
        "check-manifest-count",
        manifest_count::run,
        &["?"],
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
    ),
    // spec: canon-kit/SPEC.md §check-prose-enum — the vocabulary is a bridged *value*, two
    // index-aligned arrays because the wire format's own separator is the tab; the command
    // knob rides too, telling "none configured" from "configured, declared nothing"
    (
        "check-prose-enum",
        prose_enum::run,
        &["?"],
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
    ),
    // spec: canon-kit/SPEC.md §check-measured-claim — born native, so it derives its corpus
    // from its own glob surface rather than from `spec::manifest_files`: the knob set is its
    // two knobs plus the two bridged arrays the emitter's roster crosses as
    (
        "check-measured-claim",
        measured_claim::run,
        &["?"],
        &[
            "CANON_KIT_MEASURED_CLAIMS_CMD",
            "CANON_KIT_MEASURED_SURFACE_GLOBS",
            "CANON_KIT_MEASURED_KEYS",
            "CANON_KIT_MEASURED_VALUES",
        ],
        "canon-kit",
    ),
    // spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — a
    // substrate-sensitive member by reverse trigger only: its `couples=` reaches gate
    // declaration paths, but the corpus it scans is the governed-doc set
    (
        "check-docs-cmd",
        docs_cmd::run,
        &["?"],
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
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` for the reason spelled out at
    // check-spec-fence-balance below: the walk root does not bound the read set
    (
        "check-md-refs",
        md_refs::run,
        &["?"],
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
    ),
    // spec: gate-sdk/SPEC.md §Fail-closed contract — a git-spawning member, reaching its
    // child through `proc::run` alone
    (
        "check-tracking-claim",
        tracking_claim::run,
        &["?"],
        &[
            "GATE_PRUNE_DIRS",
            "GATE_KIT_ROOTS_HERE",
            "CANON_KIT_SPEC_NAME",
            "CANON_KIT_SCAN_KIT_ROOTS",
            "CANON_KIT_MANIFEST_FILES",
            "CANON_KIT_PROSE_SURFACE_GLOBS",
        ],
        "canon-kit",
    ),
    // spec: canon-kit/SPEC.md §check-knob-citation — the second consumer of the kit-root
    // mechanism inside this cohort: it calls it directly for its prefix roster, not only
    // through the manifest derivation, which is why the Rust form is a shared function
    (
        "check-knob-citation",
        knob_citation::run,
        &["?"],
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
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` rather than the literal `.` the walk
    // starts from: a concrete root asserts the member's `couples=` covers every tracked file
    // under it, and these members read a filtered subset the root does not bound
    (
        "check-spec-fence-balance",
        spec_fence_balance::run,
        &["?"],
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
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because the scan root is the member's
    // own first argument with a default, the variable-first-argument shape the shell parser
    // calls undecidable
    (
        "check-queue-slug-liveness",
        queue_slug_liveness::run,
        &["?"],
        &[
            "QUEUE_KIT_QUEUE_FILE",
            "QUEUE_KIT_PROSE_SURFACE_GLOBS",
            "QUEUE_KIT_ACTIVE_SECTIONS",
            "QUEUE_KIT_DEFERRED_SECTION",
            "QUEUE_KIT_ICEBOX_SECTION",
        ],
        "queue-kit",
    ),
];

pub fn lookup(name: &str) -> Option<GateFn> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _)| *n == name)
        .map(|(_, f, _, _, _)| *f)
}

pub fn roots(name: &str) -> Option<&'static [&'static str]> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _)| *n == name)
        .map(|(_, _, r, _, _)| *r)
}

pub fn knobs(name: &str) -> Option<&'static [&'static str]> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _, _)| *n == name)
        .map(|(_, _, _, k, _)| *k)
}

// spec: gate-sdk/SPEC.md §check-gate-substrate-parity — `--list`'s two columns, the second
// naming the kit whose `checks/` declares the member. Emitted together because a consumer
// reading the roster needs the owner to know whether it vendored the member at all.
pub fn names_with_owners() -> Vec<(&'static str, &'static str)> {
    REGISTRY.iter().map(|(n, _, _, _, o)| (*n, *o)).collect()
}

// spec: gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — each `?`
// absorbs one unmatched observed root, so the declaration is held to its arity. Pure, so
// the concrete-root branch is provable without a member that declares one.
#[cfg(test)]
fn declaration_covers(declared: &[&str], observed: &[String]) -> Result<(), String> {
    let mut wildcards = declared.iter().filter(|d| **d == "?").count();
    let mut undeclared: Vec<&str> = Vec::new();
    for o in observed {
        if declared.iter().any(|d| *d != "?" && d == o) {
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
        "walked {:?} but declares {:?}",
        undeclared, declared
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walk;

    #[test]
    fn a_concrete_root_matches_by_equality_and_a_leftover_is_undeclared() {
        assert!(declaration_covers(&["corpus"], &["corpus".into()]).is_ok());
        assert!(declaration_covers(&["corpus"], &["corpus".into(), "other".into()]).is_err());
        assert!(declaration_covers(&[], &["corpus".into()]).is_err());
    }

    #[test]
    fn each_question_mark_absorbs_exactly_one_unbounded_root() {
        assert!(declaration_covers(&["?"], &["anything".into()]).is_ok());
        assert!(declaration_covers(&["?"], &["a".into(), "b".into()]).is_err());
        assert!(declaration_covers(&["corpus", "?"], &["corpus".into(), "x".into()]).is_ok());
    }

    // spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the owner column is registry data
    // held to executed behavior: a declared owner must be the kit whose `checks/` carries the
    // descriptor.
    #[test]
    fn every_registry_member_declares_the_kit_that_carries_its_descriptor() {
        assert!(!REGISTRY.is_empty(), "no member to assert over");
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        for (name, _, _, _, owner) in REGISTRY {
            let declared = repo.join(owner).join("checks").join(format!("{}.gate", name));
            assert!(
                declared.is_file(),
                "{} declares owner {}, but {} is not a file",
                name,
                owner,
                declared.display()
            );
        }
    }

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test A: the declared roots are
    // held to what the code does, by running each member over its own fixture cases with
    // the walk recorder on. Nothing else makes a self-declared read set trustworthy.
    #[test]
    fn every_registry_member_declares_the_roots_it_walks() {
        assert!(!REGISTRY.is_empty(), "no member to assert over");
        walk::bridge_declared_knobs();
        let mut cases_run = 0usize;
        let mut roots_observed = 0usize;
        for (name, f, declared, knobs, _) in REGISTRY {
            for case in walk::fixture_case_dirs(name) {
                let args = case_args(&case);
                // spec: gate-sdk/SPEC.md §run-gate-tests — the member's knobs are bridged from
                // the case dir before it runs, or a bridged member exits 2 on an unresolved
                // knob and this test asserts over a run that never reached its rule
                walk::bridge_case_knobs(&case, name, knobs);
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
                if let Err(e) = declaration_covers(declared, &observed) {
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
