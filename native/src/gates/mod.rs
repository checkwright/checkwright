// spec: gate-sdk/SPEC.md §The `# graph:` manifest — one module per ported gate, and
// the roster below is the binary's reported subcommand set. The subcommand name is
// the gate name: there is no mapping table, because a table would be a name that can
// drift from the thing it names. check-gate-substrate-parity assertion B holds this
// roster equal to the set of .gate descriptors across the resolve dirs.
pub mod action_pinning;

pub type GateFn = fn(&[String]) -> i32;

pub const REGISTRY: &[(&str, GateFn)] = &[("check-action-pinning", action_pinning::run)];

pub fn lookup(name: &str) -> Option<GateFn> {
    REGISTRY.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

pub fn names() -> Vec<&'static str> {
    REGISTRY.iter().map(|(n, _)| *n).collect()
}
