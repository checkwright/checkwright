// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one module per ported gate; the
// subcommand name is the gate name, so no mapping table exists to drift
pub mod action_pinning;

pub type GateFn = fn(&[String]) -> i32;

pub const REGISTRY: &[(&str, GateFn)] = &[("check-action-pinning", action_pinning::run)];

pub fn lookup(name: &str) -> Option<GateFn> {
    REGISTRY.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

pub fn names() -> Vec<&'static str> {
    REGISTRY.iter().map(|(n, _)| *n).collect()
}
