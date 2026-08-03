// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one module per ported gate; the
// subcommand name is the gate name, so no mapping table exists to drift
pub mod action_pinning;

pub type GateFn = fn(&[String]) -> i32;

// spec: gate-sdk/SPEC.md §check-reads-couples — the third element is the member's declared
// walk roots, the data `--reads` prints. A member added without them fails to compile, so
// the declaration cannot be silently omitted.
pub const REGISTRY: &[(&str, GateFn, &[&str])] = &[
    // spec: gate-sdk/SPEC.md §check-reads-couples — `?` because the scan root is this
    // gate's first argument with a default, the same variable-first-argument shape the
    // shell parser calls undecidable and skips-and-counts.
    ("check-action-pinning", action_pinning::run, &["?"]),
];

pub fn lookup(name: &str) -> Option<GateFn> {
    REGISTRY.iter().find(|(n, _, _)| *n == name).map(|(_, f, _)| *f)
}

pub fn roots(name: &str) -> Option<&'static [&'static str]> {
    REGISTRY.iter().find(|(n, _, _)| *n == name).map(|(_, _, r)| *r)
}

pub fn names() -> Vec<&'static str> {
    REGISTRY.iter().map(|(n, _, _)| *n).collect()
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
        let mut cases_run = 0usize;
        let mut roots_observed = 0usize;
        for (name, f, declared) in REGISTRY {
            for case in walk::fixture_case_dirs(name) {
                let args = case_args(&case);
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

    // spec: gate-sdk/SPEC.md §run-gate-tests — the case's `args` file, read on the runner's
    // own terms (`#` lines stripped) so this test and the runner cannot disagree.
    fn case_args(case: &std::path::Path) -> Vec<String> {
        let p = case.join("args");
        let text = match std::fs::read_to_string(&p) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };
        text.lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(String::from)
            .collect()
    }
}
