// spec: context-kit/SPEC.md §bin/env-probe — the crate's holder of the probe roster's grammar and
// its floor predicate. Promoted here from check-install-toolchain rather than copied, so the
// roster keeps exactly one crate-side parser and the gate and the arm cannot disagree about it.
use crate::proc;

// spec: context-kit/SPEC.md §bin/env-probe — the roster is *read* from the library that owns it,
// never restated: baking the elements into Rust would ship one project's dependency set as a kit
// literal, and the file is consumer-visible precisely so a second reader can obtain it.
pub const ROSTER: &str = "context-kit/lib/toolfloor.sh";

// spec: context-kit/SPEC.md §bin/env-probe — `<name>[:<min-version>[:<impl-token>[:<audience>]]]`,
// positional, an empty field meaning unconstrained on that axis exactly as an omitted trailing one
// does, so `awk`, `awk:`, `awk::` and `awk:::` parse to one member.
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
// fail-closed arm and carries no field: an unparseable banner and a `sort` without `-V` are one
// verdict because the remedy is the same, which is to stop trusting the comparison.
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

// spec: context-kit/SPEC.md §bin/env-probe — numeric comparison is `sort -V`, kept as a spawn
// rather than replaced by a native comparator: `uncomparable`'s second cause is *a `sort` without
// `-V`*, which no in-process comparison can reach, and the golden pins that cause.
fn floor_met(min: &str, found: &str) -> Option<bool> {
    let body = format!("{}\n{}\n", min, found);
    let out = proc::run_streamed("sort", &["-V"], body.as_bytes(), proc::Stderr::Discard).ok()?;
    if out.code() != 0 {
        return None;
    }
    let text = String::from_utf8_lossy(out.stdout()).to_string();
    Some(text.lines().next().unwrap_or("") == min)
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
        let p = parse("awk::GNU:contributor:extra");
        assert_eq!((p.imp.as_str(), p.audience.as_str()), ("GNU", "contributor"));
    }

    // spec: context-kit/SPEC.md §bin/env-probe — the array is read out of the library as text, and
    // a file carrying no array at all is distinguishable from one carrying an empty one
    #[test]
    fn the_roster_reads_out_of_the_librarys_array_line() {
        assert_eq!(
            probe_set("x=1\nPROBE_SET=(bash:4.3 git awk::GNU)\ny=2\n"),
            Some(vec!["bash:4.3".to_string(), "git".to_string(), "awk::GNU".to_string()])
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

    // spec: context-kit/SPEC.md §bin/env-probe — the closed verdict set, each arm reached: the
    // fail-closed one on a banner the predicate cannot parse
    #[test]
    fn the_predicate_answers_from_the_closed_verdict_set() {
        assert_eq!(check("bash:4.3", "").rendered(), "absent");
        assert_eq!(check("awk::GNU", "GNU Awk 5.3.1").rendered(), "ok");
        assert_eq!(check("awk::GNU", "mawk 1.3.4").rendered(), "wrong-impl mawk");
        assert_eq!(check("git", "git version 2.4").rendered(), "ok");
        assert_eq!(
            check("bash:4.3", "GNU bash, version 3.2.57(1)-release").rendered(),
            "below 3.2.57 4.3"
        );
        assert_eq!(check("bash:4.3", "GNU bash, no version").rendered(), "uncomparable");
    }
}
