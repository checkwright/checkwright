// Fixture stand-in carrying the second copy the derivation replaced: a literal
// per-kit roster in the installer, which a kit adding a gate can never update.
// The §check-install-disposition citation on this line registers nothing and is
// stripped before the match, so it is not a finding.
fn recipe_gates(kit: &str) -> &'static [&'static str] {
    match kit {
        "alpha-kit" => &["check-alpha", "check-delta"],
        _ => &[],
    }
}
