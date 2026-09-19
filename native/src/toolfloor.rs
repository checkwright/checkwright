// spec: context-kit/SPEC.md §bin/env-probe — the crate's holder of the probe roster's grammar and
// its floor predicate. Promoted here from check-install-toolchain rather than copied, so the
// roster keeps exactly one crate-side parser and the gate and the arm cannot disagree about it.
// spec: context-kit/SPEC.md §bin/env-probe — the roster itself, beside the predicate that reads it.
// It carries no knob and never did, on that section's own ground, so holding it here rather than in
// a sourceable file loses no affordance the kit ever offered.
pub const PROBE_SET: &[&str] = &[
    "bash:4.3::context-kit+delegation-kit+drift-kit+guard-kit+lifecycle-kit",
    "git",
    "jq:::contributor",
    "curl:::delegation-kit",
    "shellcheck:::registered",
    "cargo:1.71::contributor",
];

// spec: context-kit/SPEC.md §bin/env-probe — the two spelled audience values that name no kit
pub const CONTRIBUTOR: &str = "contributor";
pub const REGISTERED: &str = "registered";
// spec: context-kit/SPEC.md §bin/env-probe — a kit-list audience joins its kit names with this
pub const KIT_JOIN: char = '+';

// spec: context-kit/SPEC.md §bin/env-probe — a kit-valued audience read as its kit names
pub fn audience_kits(audience: &str) -> Vec<&str> {
    audience.split(KIT_JOIN).collect()
}

// spec: context-kit/SPEC.md §bin/env-probe — what a consumer-side reader has selected: the kit set
// and the registered gate set, each read by one arm of the owed-predicate
pub struct Selection {
    pub kits: Vec<String>,
    pub gates: Vec<String>,
}

// spec: context-kit/SPEC.md §bin/env-probe — the owed-predicate's closed answer set
#[derive(Debug, PartialEq, Eq)]
pub enum Owing {
    Owed,
    NotOwed,
    Undecided,
}

// spec: context-kit/SPEC.md §bin/env-probe — a registered name with no `REGISTRY` row is a
// consumer-declared shell gate and contributes nothing
pub fn owed(element: &str, selection: Option<&Selection>) -> Owing {
    let e = parse(element);
    let audience = e.audience.as_str();
    if audience.is_empty() {
        return Owing::Owed;
    }
    if audience == CONTRIBUTOR {
        return Owing::NotOwed;
    }
    let Some(sel) = selection else {
        return Owing::Undecided;
    };
    let hit = if audience == REGISTERED {
        sel.gates.iter().any(|g| {
            crate::gates::needs(g).is_some_and(|reqs| reqs.iter().any(|(p, _)| *p == e.name))
        })
    } else {
        let owners = audience_kits(audience);
        sel.kits.iter().any(|k| owners.contains(&k.as_str()))
    };
    if hit {
        Owing::Owed
    } else {
        Owing::NotOwed
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — the roster's home, named for the reports that cite
// where a verdict came from; a caller handed no override reads `PROBE_SET` above directly.
pub const ROSTER: &str = "native/src/toolfloor.rs";

// spec: context-kit/SPEC.md §bin/env-probe — `<name>[:<min-version>[:<impl-token>[:<audience>]]]`,
// positional, an empty field meaning unconstrained on that axis exactly as an omitted trailing one
// does, so `jq`, `jq:`, `jq::` and `jq:::` parse to one member.
pub struct Element {
    pub name: String,
    pub min: String,
    pub imp: String,
    pub audience: String,
}

pub fn parse(element: &str) -> Element {
    let mut it = element.splitn(4, ':');
    let name = it.next().unwrap_or("").to_string();
    let min = it.next().unwrap_or("").to_string();
    let imp = it.next().unwrap_or("").to_string();
    // spec: context-kit/SPEC.md §bin/env-probe — the shell reader takes `${_rest%%:*}` at every
    // step, so a fifth field is dropped rather than folded into the audience
    let audience = it
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();
    Element {
        name,
        min,
        imp,
        audience,
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — the grammar is parsed rather than sourced: a fixture
// path is untrusted input, so a reader of the array must not be made to execute the file. `None`
// is *no array at all*, which each caller reports against its own path.
pub fn probe_set(text: &str) -> Option<Vec<String>> {
    let line = text.lines().find(|l| l.starts_with("PROBE_SET=("))?;
    let inner = line
        .split_once('(')
        .map(|(_, r)| r)
        .unwrap_or("")
        .split(')')
        .next()
        .unwrap_or("");
    Some(inner.split_whitespace().map(String::from).collect())
}

// spec: context-kit/SPEC.md §bin/env-probe — the closed verdict set. `uncomparable` is the
// fail-closed arm and carries no field: its one cause is a banner or token the predicate cannot
// compare, whose remedy is to stop trusting the comparison.
pub enum Verdict {
    Ok,
    Absent,
    Below { found: String, floor: String },
    WrongImpl { found: String },
    Uncomparable,
}

impl Verdict {
    // spec: context-kit/SPEC.md §bin/env-probe — the shell predicate's stdout word-for-word, which
    // is what a caller reading it with `read -r _kind _found _floor` consumes.
    pub fn rendered(&self) -> String {
        match self {
            Verdict::Ok => "ok".to_string(),
            Verdict::Absent => "absent".to_string(),
            Verdict::Below { found, floor } => format!("below {} {}", found, floor),
            Verdict::WrongImpl { found } => format!("wrong-impl {}", found),
            Verdict::Uncomparable => "uncomparable".to_string(),
        }
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — the banner's first dotted-version token, bash's
// `[[ $b =~ ([0-9]+(\.[0-9]+)+) ]]`; the crate's POSIX matcher reports the same leftmost-longest
// span, so the two holders agree on a banner carrying several candidates.
pub fn version(banner: &str) -> String {
    let re = match crate::ere::Ere::compile("[0-9]+(\\.[0-9]+)+") {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    match re.find(banner) {
        Some((s, e)) => banner[s..e].to_string(),
        None => String::new(),
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — the first word of the banner, bash's
// `${banner%%[[:space:]]*}`: the implementation's own name, which is what a `wrong-impl` verdict
// reports back.
fn first_word(banner: &str) -> String {
    match banner.find(char::is_whitespace) {
        Some(i) => banner[..i].to_string(),
        None => banner.to_string(),
    }
}

// spec: context-kit/SPEC.md §bin/env-probe — field-wise numeric, the shorter token padded with
// zero fields; a field that is not an ASCII digit run, or overflows `u64`, answers `None`.
pub fn floor_met(min: &str, found: &str) -> Option<bool> {
    let fields = |t: &str| -> Option<Vec<u64>> {
        t.split('.')
            .map(|f| {
                if f.is_empty() || !f.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                f.parse::<u64>().ok()
            })
            .collect()
    };
    let (m, f) = (fields(min)?, fields(found)?);
    for i in 0..m.len().max(f.len()) {
        let (a, b) = (m.get(i).copied().unwrap_or(0), f.get(i).copied().unwrap_or(0));
        if a != b {
            return Some(b > a);
        }
    }
    Some(true)
}

// spec: context-kit/SPEC.md §bin/env-probe — `tool_floor_check <element> <banner>`, arm for arm
// and in its order: an empty banner is `absent`, the implementation token is a substring test on
// the banner, an unconstrained member is `ok`, and every comparison failure is `uncomparable`.
pub fn check(element: &str, banner: &str) -> Verdict {
    let e = parse(element);
    if banner.is_empty() {
        return Verdict::Absent;
    }
    if !e.imp.is_empty() && !banner.contains(&e.imp) {
        return Verdict::WrongImpl {
            found: first_word(banner),
        };
    }
    if e.min.is_empty() {
        return Verdict::Ok;
    }
    let found = version(banner);
    if found.is_empty() {
        return Verdict::Uncomparable;
    }
    match floor_met(&e.min, &found) {
        None => Verdict::Uncomparable,
        Some(true) => Verdict::Ok,
        Some(false) => Verdict::Below {
            found,
            floor: e.min,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selection(kits: &[&str], gates: &[&str]) -> Selection {
        Selection {
            kits: kits.iter().map(|s| s.to_string()).collect(),
            gates: gates.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn owed_names(sel: Option<&Selection>, want: Owing) -> Vec<String> {
        PROBE_SET
            .iter()
            .filter(|e| owed(e, sel) == want)
            .map(|e| parse(e).name)
            .collect()
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the owed-predicate over each arm: an unselected
    // kit and a gate set naming no requirement owe the unconditional members alone
    #[test]
    fn a_conditional_member_is_owed_only_where_the_selection_reaches_it() {
        let base = selection(&["gate-sdk"], &["check-core-files", "check-no-such-gate"]);
        assert_eq!(owed_names(Some(&base), Owing::Owed), vec!["git"]);
        let prose = selection(&["gate-sdk", "canon-kit"], &[]);
        assert_eq!(owed_names(Some(&prose), Owing::Owed), vec!["git"]);
        let guarded = selection(&["gate-sdk", "guard-kit"], &[]);
        assert_eq!(owed_names(Some(&guarded), Owing::Owed), vec!["bash", "git"]);
        let staged = selection(&["gate-sdk", "lifecycle-kit"], &[]);
        assert_eq!(owed_names(Some(&staged), Owing::Owed), vec!["bash", "git"]);
        let linted = selection(&["gate-sdk"], &["check-action-run-shell"]);
        assert_eq!(owed_names(Some(&linted), Owing::Owed), vec!["git", "shellcheck"]);
        assert_eq!(owed_names(None, Owing::Undecided), vec!["bash", "curl", "shellcheck"]);
        assert_eq!(owed_names(None, Owing::NotOwed), vec!["jq", "cargo"]);
    }

    // spec: context-kit/SPEC.md §bin/env-probe — a kit list is the union of its names, and a name
    // that is only a substring of a listed one owes nothing
    #[test]
    fn a_kit_list_audience_is_owed_where_any_listed_kit_is_selected() {
        let e = "tool:::alpha-kit+beta-kit";
        assert_eq!(owed(e, Some(&selection(&["beta-kit"], &[]))), Owing::Owed);
        assert_eq!(owed(e, Some(&selection(&["alpha-kit", "gamma-kit"], &[]))), Owing::Owed);
        assert_eq!(owed(e, Some(&selection(&["alpha", "beta-kit-x"], &[]))), Owing::NotOwed);
        assert_eq!(owed(e, None), Owing::Undecided);
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the audience value set is closed: each kit name a
    // value lists names a kit root the authoring tree carries, so a misspelling cannot leave every floor
    #[test]
    fn every_audience_value_is_closed_over_the_kit_roots() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the crate sits under the repo root");
        let sdk = repo.join("gate-sdk").display().to_string();
        let kits: Vec<String> = crate::walk::kit_roots_rel_from(&sdk, "")
            .expect("cannot derive the kit roots")
            .iter()
            .map(|r| r.rsplit('/').next().unwrap_or(r).to_string())
            .collect();
        assert!(kits.iter().any(|k| k == "guard-kit"), "the kit-root derivation found {:?}", kits);
        let offenders: Vec<String> = PROBE_SET
            .iter()
            .map(|e| parse(e).audience)
            .filter(|a| !a.is_empty() && a != CONTRIBUTOR && a != REGISTERED)
            .flat_map(|a| audience_kits(&a).into_iter().map(String::from).collect::<Vec<_>>())
            .filter(|k| !kits.contains(k))
            .collect();
        assert!(offenders.is_empty(), "audience values naming no kit root: {:?}", offenders);
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the three spellings of an unconstrained member
    // are one member, and a fifth field is dropped rather than folded into the audience
    #[test]
    fn every_empty_trailing_field_parses_to_the_same_member() {
        for e in ["jq", "jq:", "jq::", "jq:::"] {
            let p = parse(e);
            assert_eq!((p.name.as_str(), p.min.as_str(), p.imp.as_str(), p.audience.as_str()), ("jq", "", "", ""));
        }
        let p = parse("cargo:1.71::contributor");
        assert_eq!((p.min.as_str(), p.imp.as_str(), p.audience.as_str()), ("1.71", "", "contributor"));
        let p = parse("sort::coreutils:contributor:extra");
        assert_eq!((p.imp.as_str(), p.audience.as_str()), ("coreutils", "contributor"));
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the array is read out of the library as text, and
    // a file carrying no array at all is distinguishable from one carrying an empty one
    #[test]
    fn the_roster_reads_out_of_the_librarys_array_line() {
        assert_eq!(
            probe_set("x=1\nPROBE_SET=(bash:4.3 git sort::coreutils)\ny=2\n"),
            Some(vec!["bash:4.3".to_string(), "git".to_string(), "sort::coreutils".to_string()])
        );
        assert_eq!(probe_set("PROBE_SET=()\n"), Some(Vec::new()));
        assert_eq!(probe_set("nothing here\n"), None);
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the version token is leftmost-longest, so a
    // banner whose first digit run carries no dot does not shadow the version further along it
    #[test]
    fn the_version_token_is_the_first_dotted_run() {
        assert_eq!(version("GNU Awk 5.3.1, API 4.0"), "5.3.1");
        assert_eq!(version("GNU bash, version 5.2.37(1)-release"), "5.2.37");
        assert_eq!(version("mawk 1.3.4 20240905"), "1.3.4");
        assert_eq!(version("GNU bash, no version here"), "");
        assert_eq!(version("12 items, 3.4 left"), "3.4");
    }

    // spec: context-kit/SPEC.md §bin/env-probe — numeric fields rather than a lexical comparison,
    // and a token outside the digit-run grammar is uncomparable rather than ordered
    #[test]
    fn the_floor_compares_numeric_fields_padded_with_zeros() {
        assert_eq!(floor_met("4.3", "5.2.21"), Some(true));
        assert_eq!(floor_met("4.3", "4.2.46"), Some(false));
        assert_eq!(check("bash:4.3", "GNU bash, version 4.2.46(1)-release").rendered(), "below 4.2.46 4.3");
        assert_eq!(floor_met("1.71", "1.71"), Some(true));
        assert_eq!(floor_met("4.3", "4.3.0"), Some(true));
        assert_eq!(floor_met("4.10", "4.9"), Some(false));
        assert_eq!(floor_met("4.3", "4.99999999999999999999999"), None);
        assert_eq!(floor_met("4.3", "4.3-rc1"), None);
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the closed verdict set, each arm reached: the
    // fail-closed one on a banner the predicate cannot parse
    #[test]
    fn the_predicate_answers_from_the_closed_verdict_set() {
        assert_eq!(check("bash:4.3", "").rendered(), "absent");
        assert_eq!(check("sort::coreutils", "sort (GNU coreutils) 9.4").rendered(), "ok");
        assert_eq!(check("sort::coreutils", "BusyBox v1.36.1").rendered(), "wrong-impl BusyBox");
        assert_eq!(check("awk", "mawk 1.3.4").rendered(), "ok");
        assert_eq!(check("git", "git version 2.4").rendered(), "ok");
        assert_eq!(
            check("bash:4.3", "GNU bash, version 3.2.57(1)-release").rendered(),
            "below 3.2.57 4.3"
        );
        assert_eq!(check("bash:4.3", "GNU bash, no version").rendered(), "uncomparable");
    }
}
