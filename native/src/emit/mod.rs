// spec: gate-sdk/SPEC.md §The non-gate arm — the ported emitters. Each owes no descriptor, no
// registration and no fixture pair, and owes a named reader instead: the regen command in
// docs/site-architecture.md §Generated projections, and the comparator calling `emit()`.
pub mod footprint;

pub type EmitFn = fn() -> Result<String, String>;

// spec: gate-sdk/SPEC.md §The non-gate arm — the projection's own name keys the arm, so no
// mapping table exists to drift. The third element is the arm's bridged knob reads, the data
// `--knobs` prints for it.
pub const EMITTERS: &[(&str, EmitFn, &[&str])] = &[(
    "footprint",
    footprint::emit,
    &["CONTEXT_KIT_SURFACES"],
)];

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
