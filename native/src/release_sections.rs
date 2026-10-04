// spec: installer/SPEC.md §The upgrade contract — the release note's section roster, the one place a
// reader learns a section's name: the roles are kit constants, the headings and aliases the knobs'
use crate::declaration::TokenRule;
use crate::walk;

pub const SECTIONS_KNOB: &str = "GATE_SDK_RELEASE_SECTIONS";
pub const ALIASES_KNOB: &str = "GATE_SDK_RELEASE_SECTION_ALIASES";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Brief,
    Gates,
    Knobs,
    Authoring,
    Platforms,
    Behavior,
}

const ROLES: [(&str, Role); 6] = [
    ("brief", Role::Brief),
    ("gates", Role::Gates),
    ("knobs", Role::Knobs),
    ("authoring", Role::Authoring),
    ("platforms", Role::Platforms),
    ("behavior", Role::Behavior),
];

impl Role {
    pub fn token_rule(self) -> Option<TokenRule> {
        match self {
            Role::Brief => None,
            Role::Gates => Some(TokenRule::GateName),
            Role::Knobs | Role::Platforms => Some(TokenRule::Backticked),
            Role::Authoring | Role::Behavior => Some(TokenRule::Bolded),
        }
    }

    pub fn declaration_bearing(self) -> bool {
        self.token_rule().is_some()
    }
}

pub struct Section {
    pub role: Role,
    pub heading: String,
    pub aliases: Vec<String>,
}

impl Section {
    // spec: gate-sdk/SPEC.md §lib/declaration.sh — the name set a reader hands the holder
    pub fn names(&self) -> Vec<&str> {
        std::iter::once(self.heading.as_str()).chain(self.aliases.iter().map(String::as_str)).collect()
    }
}

// spec: installer/SPEC.md §The upgrade contract — `<role>: <heading>`, split at the first colon
fn element(e: &str) -> Result<(Role, String), String> {
    let (role, heading) = e.split_once(':').unwrap_or((e, ""));
    let role = role.trim();
    let heading = heading.trim();
    let Some((_, r)) = ROLES.iter().find(|(n, _)| *n == role) else {
        return Err(format!(
            "element '{}' names unknown role '{}' (roles: {})",
            e,
            role,
            ROLES.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(", ")
        ));
    };
    if heading.is_empty() {
        return Err(format!("element '{}' carries an empty heading", e));
    }
    Ok((*r, heading.to_string()))
}

fn role_name(r: Role) -> &'static str {
    ROLES.iter().find(|(_, x)| *x == r).map(|(n, _)| *n).unwrap_or("")
}

// spec: gate-sdk/SPEC.md §Layout and configuration — the validator's refusals, shared with the
// reader so the two hold one reading of an element
pub fn refusals(sections: &[String], aliases: &[String]) -> Vec<String> {
    let mut errs = Vec::new();
    let mut seen: Vec<(Role, String)> = Vec::new();
    for e in sections {
        match element(e) {
            Ok((r, h)) => {
                if seen.iter().any(|(x, _)| *x == r) {
                    errs.push(format!("{} gives role '{}' twice", SECTIONS_KNOB, role_name(r)));
                }
                seen.push((r, h));
            }
            Err(m) => errs.push(format!("{} {}", SECTIONS_KNOB, m)),
        }
    }
    for e in aliases {
        match element(e) {
            Ok((_, h)) if seen.iter().any(|(_, x)| *x == h) => errs.push(format!(
                "{} element '{}' names a heading {} already gives, so the section would read under two roles",
                ALIASES_KNOB, e, SECTIONS_KNOB
            )),
            Ok(_) => {}
            Err(m) => errs.push(format!("{} {}", ALIASES_KNOB, m)),
        }
    }
    errs
}

pub fn parse(sections: &[String], aliases: &[String]) -> Result<Vec<Section>, String> {
    let errs = refusals(sections, aliases);
    if !errs.is_empty() {
        return Err(errs.join("; "));
    }
    let mut out: Vec<Section> = Vec::new();
    for e in sections {
        let (role, heading) = element(e)?;
        out.push(Section { role, heading, aliases: Vec::new() });
    }
    for e in aliases {
        let (role, heading) = element(e)?;
        if let Some(s) = out.iter_mut().find(|s| s.role == role) {
            s.aliases.push(heading);
        }
    }
    Ok(out)
}

// spec: installer/SPEC.md §The upgrade contract — the roster in note order; a role the consumer
// omits is absent from every reader's grammar
pub fn roster() -> Result<Vec<Section>, String> {
    parse(&walk::knob_array(SECTIONS_KNOB)?, &walk::knob_array(ALIASES_KNOB)?)
}

pub fn find(roster: &[Section], role: Role) -> Option<&Section> {
    roster.iter().find(|s| s.role == role)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(e: &[&str]) -> Vec<String> {
        e.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_default_roster_resolves_in_note_order_with_its_aliases() {
        let r = parse(
            &v(&["brief: B", "gates: G", "knobs: K"]),
            &v(&["gates: Old G", "knobs: Old K", "knobs: Older K"]),
        )
        .expect("refused");
        assert_eq!(r.iter().map(|s| s.role).collect::<Vec<_>>(), vec![Role::Brief, Role::Gates, Role::Knobs]);
        assert_eq!(r[1].names(), vec!["G", "Old G"]);
        assert_eq!(r[2].names(), vec!["K", "Old K", "Older K"]);
        assert!(!r[0].role.declaration_bearing());
    }

    #[test]
    fn each_validator_refusal_fires() {
        let one = |s: &[&str], a: &[&str], needle: &str| {
            let e = refusals(&v(s), &v(a));
            assert!(e.iter().any(|m| m.contains(needle)), "{:?} lacks {}", e, needle);
        };
        one(&["bogus: X"], &[], "unknown role 'bogus'");
        one(&["gates X"], &[], "unknown role");
        one(&[], &["bogus: X"], "unknown role 'bogus'");
        one(&["gates: G", "gates: H"], &[], "gives role 'gates' twice");
        one(&["gates:  "], &[], "empty heading");
        one(&[], &["gates:"], "empty heading");
        one(&["gates: G", "knobs: K"], &["gates: K"], "already gives");
        assert!(refusals(&v(&["gates: G"]), &v(&["gates: H"])).is_empty());
    }
}
