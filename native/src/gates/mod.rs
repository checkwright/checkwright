// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one module per ported gate; the
// subcommand name is the gate name, so no mapping table exists to drift
pub mod action_gh_repo;
pub mod action_pinning;
pub mod queue_entry_budget;
pub mod queue_hygiene;
pub mod queue_sections;
pub mod queue_slug_liveness;
pub mod queue_wrap;
pub mod tag_lead_line;
pub mod task_names;

pub type GateFn = fn(&[String]) -> i32;

// spec: gate-sdk/SPEC.md §check-reads-couples — the third element is the member's declared
// walk roots, the data `--reads` prints. A member added without them fails to compile, so
// the declaration cannot be silently omitted.
// spec: gate-sdk/SPEC.md §lib/gate.sh — the fourth element is the member's declared knob
// reads, the data `--knobs` prints and the config bridge resolves. Un-omittable by the same
// construction, so no member can read a knob the bridge was never asked to carry.
pub const REGISTRY: &[(&str, GateFn, &[&str], &[&str])] = &[
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because each member's scan root is
    // its own first argument with a default, the same variable-first-argument shape the
    // shell parser calls undecidable and skips-and-counts.
    (
        "check-action-pinning",
        action_pinning::run,
        &["?"],
        &["GATE_PRUNE_DIRS"],
    ),
    (
        "check-action-gh-repo",
        action_gh_repo::run,
        &["?"],
        &["GATE_PRUNE_DIRS"],
    ),
    // spec: gate-sdk/SPEC.md §check-reads-couples — the queue-kit cohort reads named files
    // rather than walking a tree, so each member declares an empty walk-root set: there is no
    // root for the recorder to observe, and unit test A holds that to executed behavior.
    (
        "check-queue-sections",
        queue_sections::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_REQUIRED_SECTIONS"],
    ),
    (
        "check-queue-wrap",
        queue_wrap::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_WRAP_BUDGET"],
    ),
    (
        "check-queue-hygiene",
        queue_hygiene::run,
        &[],
        &["QUEUE_KIT_QUEUE_FILE", "QUEUE_KIT_PROSE_LEADS"],
    ),
    // spec: gate-sdk/SPEC-queue-cohort.md — a member reading a derived section matcher declares
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
    ),
];

pub fn lookup(name: &str) -> Option<GateFn> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _)| *n == name)
        .map(|(_, f, _, _)| *f)
}

pub fn roots(name: &str) -> Option<&'static [&'static str]> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _)| *n == name)
        .map(|(_, _, r, _)| *r)
}

pub fn knobs(name: &str) -> Option<&'static [&'static str]> {
    REGISTRY
        .iter()
        .find(|(n, _, _, _)| *n == name)
        .map(|(_, _, _, k)| *k)
}

pub fn names() -> Vec<&'static str> {
    REGISTRY.iter().map(|(n, _, _, _)| *n).collect()
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

    // spec: gate-sdk/SPEC.md §check-reads-couples — unit test A: the declared roots are
    // held to what the code does, by running each member over its own fixture cases with
    // the walk recorder on. Nothing else makes a self-declared read set trustworthy.
    #[test]
    fn every_registry_member_declares_the_roots_it_walks() {
        assert!(!REGISTRY.is_empty(), "no member to assert over");
        walk::bridge_declared_knobs();
        let mut cases_run = 0usize;
        let mut roots_observed = 0usize;
        for (name, f, declared, knobs) in REGISTRY {
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
