// spec: delegation-kit/SPEC.md §The tier binding — the roster's default, the one row parser and the
// one matcher every reader of a class table shares: `--emit agent-tiers`, `check-agent-tier-explicit`,
// the dispatch guard's D6, `--model-verdict`, the foreign executor and the table validator.

// spec: delegation-kit/SPEC.md §The tier binding — the roster a consumer's knob file leaves unset,
// highest first; a floor check reads "at or above" off the roster's order.
pub const DEFAULT_CLASSES: &[&str] = &["judgment", "routing", "mechanical"];

pub fn rank(roster: &[String], class: &str) -> Option<usize> {
    roster.iter().position(|c| c == class)
}

// spec: delegation-kit/SPEC.md §Layout and configuration — a class, a harness and an adapter are
// each a non-empty name within `[a-z0-9-]`
pub fn name(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

// spec: delegation-kit/SPEC.md §The tier binding — one `<key>=<value>` element, split at its first
// `=`; the validator refuses what does not split, so a reader past it meets only well-formed rows.
pub fn split(element: &str) -> Option<(&str, &str)> {
    element.split_once('=')
}

// spec: delegation-kit/SPEC.md §The tier binding — a row's value is `<model>[,<effort>]`, split at
// its first `,`
pub fn pair(value: &str) -> (&str, Option<&str>) {
    match value.split_once(',') {
        Some((model, effort)) => (model, Some(effort)),
        None => (value, None),
    }
}

fn row<'a>(table: &'a [String], key: &str) -> Option<(&'a str, Option<&'a str>)> {
    table.iter().filter_map(|e| split(e)).find(|(k, _)| *k == key).map(|(_, v)| pair(v))
}

// spec: delegation-kit/SPEC.md §The tier binding — the bound model of a class, or none for an
// unbound class.
pub fn bound<'a>(binding: &'a [String], class: &str) -> Option<&'a str> {
    row(binding, class).map(|(model, _)| model)
}

pub fn effort<'a>(binding: &'a [String], class: &str) -> Option<&'a str> {
    row(binding, class).and_then(|(_, effort)| effort)
}

// spec: delegation-kit/SPEC.md §The tier binding — the pair a foreign harness binds to a class
pub fn foreign_bound<'a>(rows: &'a [String], harness: &str, class: &str) -> Option<(&'a str, Option<&'a str>)> {
    rows.iter()
        .filter_map(|e| split(e))
        .find(|(k, _)| k.split_once('/') == Some((harness, class)))
        .map(|(_, v)| pair(v))
}

pub fn values(binding: &[String]) -> Vec<&str> {
    binding.iter().filter_map(|e| split(e)).map(|(_, v)| pair(v).0).collect()
}

fn leading(value: &str) -> &str {
    value.split('-').next().unwrap_or(value)
}

// spec: delegation-kit/SPEC.md §The tier binding — a running model id matches a bound value when the
// value equals the id or one of its `-`-separated tokens after the first. The leading token is the
// namespace every id shares, so a value equal to it matches nothing rather than every model.
pub fn matches(value: &str, id: &str) -> bool {
    value == id || id.split('-').skip(1).any(|t| t == value)
}

// spec: delegation-kit/SPEC.md §Layout and configuration — the roster's refusals
pub fn roster_refusals(roster: &[String]) -> Vec<String> {
    const KNOB: &str = "DELEGATION_KIT_TIER_CLASSES";
    let mut errs: Vec<String> = Vec::new();
    for (i, c) in roster.iter().enumerate() {
        if !name(c) {
            errs.push(format!("{} element '{}' is not a class name in [a-z0-9-]", KNOB, c));
        } else if roster[..i].contains(c) {
            errs.push(format!("{} names class '{}' twice", KNOB, c));
        }
    }
    errs
}

// spec: delegation-kit/SPEC.md §Layout and configuration — a value's refusals, shared by both
// tables: the model's on every row, the two matching spellings on the master table alone, and the
// effort's where the value carries a comma. A model that passes is returned for the cross-row check.
fn value_refusals<'a>(knob: &str, e: &'a str, value: &'a str, matched: bool, errs: &mut Vec<String>) -> Option<&'a str> {
    let (model, effort) = pair(value);
    match effort {
        Some("") => errs.push(format!("{} element '{}' has an empty effort", knob, e)),
        Some(f) if f.chars().any(char::is_whitespace) => {
            errs.push(format!("{} element '{}' carries whitespace in its effort", knob, e))
        }
        Some(f) if f.contains(',') => errs.push(format!("{} element '{}' carries a second comma", knob, e)),
        _ => {}
    }
    if model.is_empty() {
        errs.push(format!("{} element '{}' has an empty value", knob, e));
    } else if model.chars().any(char::is_whitespace) {
        errs.push(format!("{} element '{}' carries whitespace in its value", knob, e));
    } else if model == "inherit" {
        errs.push(format!(
            "{} element '{}' binds 'inherit', which is a dispatcher's tier and not a model — state it in the definition instead",
            knob, e
        ));
    } else if matched && model.bytes().all(|b| b.is_ascii_digit()) {
        errs.push(format!(
            "{} element '{}' is all digits, a version token that would match across families",
            knob, e
        ));
    } else {
        return Some(model);
    }
    None
}

fn unknown_class(knob: &str, e: &str, class: &str, roster: &[String]) -> String {
    format!("{} element '{}' names class '{}', not one of {}", knob, e, class, roster.join(", "))
}

// spec: delegation-kit/SPEC.md §Layout and configuration — the binding's refusals, every finding
// reported
pub fn refusals(binding: &[String], roster: &[String]) -> Vec<String> {
    const KNOB: &str = "DELEGATION_KIT_TIER_MODEL";
    let mut errs: Vec<String> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    let mut vals: Vec<&str> = Vec::new();
    for e in binding {
        let Some((class, value)) = split(e) else {
            errs.push(format!("{} element '{}' is not '<class>=<model>[,<effort>]'", KNOB, e));
            continue;
        };
        if rank(roster, class).is_none() {
            errs.push(unknown_class(KNOB, e, class, roster));
        } else if seen.contains(&class) {
            errs.push(format!("{} binds class '{}' twice", KNOB, class));
        } else {
            seen.push(class);
        }
        vals.extend(value_refusals(KNOB, e, value, true, &mut errs));
    }
    for v in &vals {
        if let Some(other) = vals.iter().find(|o| *o != v && leading(o) == *v) {
            errs.push(format!(
                "{} value '{}' is the leading token of '{}', a namespace that would hold no tier",
                KNOB, v, other
            ));
        }
    }
    errs
}

// spec: delegation-kit/SPEC.md §Layout and configuration — a foreign harness's rows: the master
// table's refusals less the two that guard a match no reader makes against a foreign row
pub fn foreign_refusals(rows: &[String], roster: &[String]) -> Vec<String> {
    const KNOB: &str = "DELEGATION_KIT_FOREIGN_TIER_MODEL";
    let mut errs: Vec<String> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for e in rows {
        let Some((key, value)) = split(e) else {
            errs.push(format!("{} element '{}' is not '<harness>/<class>=<model>[,<effort>]'", KNOB, e));
            continue;
        };
        match key.split_once('/') {
            Some((harness, class)) if name(harness) && name(class) => {
                if rank(roster, class).is_none() {
                    errs.push(unknown_class(KNOB, e, class, roster));
                } else if seen.contains(&key) {
                    errs.push(format!("{} binds '{}' twice", KNOB, key));
                } else {
                    seen.push(key);
                }
            }
            _ => errs.push(format!(
                "{} element '{}' keys '{}', not '<harness>/<class>' with both names in [a-z0-9-]",
                KNOB, e, key
            )),
        }
        value_refusals(KNOB, e, value, false, &mut errs);
    }
    errs
}

// spec: delegation-kit/SPEC.md §model-verdict — the classes whose bound value the id matches,
// highest first
pub fn classes_of<'a>(binding: &[String], roster: &'a [String], id: &str) -> Vec<&'a str> {
    roster
        .iter()
        .map(String::as_str)
        .filter(|c| bound(binding, c).is_some_and(|v| matches(v, id)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(elems: &[&str]) -> Vec<String> {
        elems.iter().map(|s| s.to_string()).collect()
    }

    fn roster() -> Vec<String> {
        b(DEFAULT_CLASSES)
    }

    // spec: delegation-kit/SPEC.md §The tier binding — an exact id matches only itself, an alias
    // matches an id carrying it after the first token, and the leading token matches nothing
    #[test]
    fn the_matcher_reads_every_token_but_the_namespace() {
        assert!(matches("vendor-big-5-5", "vendor-big-5-5"));
        assert!(!matches("vendor-big-5-5", "vendor-big-5-6"));
        assert!(matches("big", "vendor-big-5-5"));
        assert!(!matches("small", "vendor-big-5-5"));
        assert!(!matches("vendor", "vendor-big-5-5"));
        assert!(!matches("vendor", "vendor-small-4"));
        assert!(matches("small", "small"));
    }

    // spec: delegation-kit/SPEC.md §The tier binding — a class reads its bound value, and an unbound
    // class reads none
    #[test]
    fn a_class_reads_its_bound_value() {
        let binding = b(&["mechanical=small", "judgment=big", "routing=big"]);
        assert_eq!(bound(&binding, "routing"), Some("big"));
        assert_eq!(values(&binding), vec!["small", "big", "big"]);
        assert_eq!(bound(&b(&["judgment=big"]), "mechanical"), None);
        assert_eq!(classes_of(&binding, &roster(), "vendor-big-5"), vec!["judgment", "routing"]);
        assert_eq!(classes_of(&binding, &roster(), "vendor-small-4"), vec!["mechanical"]);
        assert!(classes_of(&binding, &roster(), "vendor-tiny-1").is_empty());
    }

    // spec: delegation-kit/SPEC.md §The tier binding — a value splits at its first comma, a value
    // with none binds a model and no effort, and every reader of the model half reads past the effort
    #[test]
    fn a_row_binds_a_model_and_an_optional_effort() {
        let binding = b(&["judgment=big,deep", "mechanical=small"]);
        assert_eq!((bound(&binding, "judgment"), effort(&binding, "judgment")), (Some("big"), Some("deep")));
        assert_eq!((bound(&binding, "mechanical"), effort(&binding, "mechanical")), (Some("small"), None));
        assert_eq!(effort(&binding, "routing"), None);
        assert_eq!(values(&binding), vec!["big", "small"]);
        assert_eq!(classes_of(&binding, &roster(), "vendor-big-5"), vec!["judgment"]);
        assert!(refusals(&binding, &roster()).is_empty());
        for (want, bad) in [
            ("has an empty effort", "judgment=big,"),
            ("carries whitespace in its effort", "judgment=big,very deep"),
            ("carries a second comma", "judgment=big,deep,x"),
            ("has an empty value", "judgment=,deep"),
        ] {
            let errs = refusals(&b(&[bad]), &roster());
            assert!(errs.len() == 1 && errs[0].contains(want), "{}: {:?}", bad, errs);
        }
    }

    // spec: delegation-kit/SPEC.md §The tier binding — the roster is the consumer's: its order is the
    // ranking, a class outside it is unknown to both tables, and its own two refusals
    #[test]
    fn the_roster_is_read_from_its_knob() {
        let four = b(&["expert", "judgment", "mechanical", "trivial"]);
        assert_eq!(rank(&four, "expert"), Some(0));
        assert_eq!(rank(&four, "routing"), None);
        assert!(refusals(&b(&["expert=huge,deep", "trivial=tiny"]), &four).is_empty());
        let errs = refusals(&b(&["routing=big"]), &four);
        assert!(errs.len() == 1 && errs[0].contains("not one of expert, judgment, mechanical, trivial"), "{:?}", errs);
        assert!(roster_refusals(&four).is_empty());
        let errs = roster_refusals(&b(&["judgment", "Expert", "judgment", ""]));
        assert_eq!(errs.len(), 3, "{:?}", errs);
        assert!(errs[0].contains("'Expert' is not a class name") && errs[1].contains("names class 'judgment' twice"), "{:?}", errs);
    }

    // spec: delegation-kit/SPEC.md §The tier binding — a foreign harness's rows resolve by harness and
    // class, and take the value's refusals less the two matching ones
    #[test]
    fn a_foreign_row_resolves_by_harness_and_class() {
        let rows = b(&["vend/judgment=g-9,deep", "vend/mechanical=5", "other/judgment=vend"]);
        assert!(foreign_refusals(&rows, &roster()).is_empty(), "{:?}", foreign_refusals(&rows, &roster()));
        assert_eq!(foreign_bound(&rows, "vend", "judgment"), Some(("g-9", Some("deep"))));
        assert_eq!(foreign_bound(&rows, "vend", "mechanical"), Some(("5", None)));
        assert_eq!(foreign_bound(&rows, "vend", "routing"), None);
        assert_eq!(foreign_bound(&rows, "nope", "judgment"), None);
        for (want, bad) in [
            ("is not '<harness>/<class>=<model>[,<effort>]'", &["vend/judgment"][..]),
            ("not '<harness>/<class>' with both names", &["judgment=g"]),
            ("not '<harness>/<class>' with both names", &["Vend/judgment=g"]),
            ("not '<harness>/<class>' with both names", &["vend/a/b=g"]),
            ("names class 'premium'", &["vend/premium=g"]),
            ("binds 'vend/judgment' twice", &["vend/judgment=g", "vend/judgment=h"]),
            ("has an empty value", &["vend/judgment="]),
            ("carries whitespace in its value", &["vend/judgment=g 9"]),
            ("binds 'inherit'", &["vend/judgment=inherit"]),
            ("has an empty effort", &["vend/judgment=g,"]),
            ("carries whitespace in its effort", &["vend/judgment=g,x y"]),
            ("carries a second comma", &["vend/judgment=g,x,y"]),
        ] {
            let errs = foreign_refusals(&b(bad), &roster());
            assert!(errs.len() == 1 && errs[0].contains(want), "{:?}: {:?}", bad, errs);
        }
    }
}
