// spec: delegation-kit/SPEC.md §The tier binding — the class vocabulary, the binding's parse and the
// one matcher every reader of the binding shares: `--emit agent-tiers`, `check-agent-tier-explicit`,
// the dispatch guard's D6, `--model-verdict` and the table validator.

// spec: delegation-kit/SPEC.md §The tier binding — the classes, highest first; a floor check reads
// "at or above" off this order.
pub const CLASSES: &[&str] = &["judgment", "routing", "mechanical"];

pub fn rank(class: &str) -> Option<usize> {
    CLASSES.iter().position(|c| *c == class)
}

// spec: delegation-kit/SPEC.md §The tier binding — one `<class>=<model>` element, split at its first
// `=`; the validator refuses what does not split, so a reader past it meets only well-formed pairs.
pub fn split(element: &str) -> Option<(&str, &str)> {
    element.split_once('=')
}

// spec: delegation-kit/SPEC.md §The tier binding — the bound value of a class, or none for an unbound
// class.
pub fn bound<'a>(binding: &'a [String], class: &str) -> Option<&'a str> {
    binding
        .iter()
        .filter_map(|e| split(e))
        .find(|(c, _)| *c == class)
        .map(|(_, v)| v)
}

pub fn values(binding: &[String]) -> Vec<&str> {
    binding.iter().filter_map(|e| split(e)).map(|(_, v)| v).collect()
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

// spec: delegation-kit/SPEC.md §Layout and configuration — the binding's refusals, every finding
// reported
pub fn refusals(binding: &[String]) -> Vec<String> {
    const KNOB: &str = "DELEGATION_KIT_TIER_MODEL";
    let mut errs: Vec<String> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    let mut vals: Vec<&str> = Vec::new();
    for e in binding {
        let Some((class, value)) = split(e) else {
            errs.push(format!("{} element '{}' is not '<class>=<model>'", KNOB, e));
            continue;
        };
        if rank(class).is_none() {
            errs.push(format!(
                "{} element '{}' names class '{}', not one of {}",
                KNOB,
                e,
                class,
                CLASSES.join(", ")
            ));
        } else if seen.contains(&class) {
            errs.push(format!("{} binds class '{}' twice", KNOB, class));
        } else {
            seen.push(class);
        }
        if value.is_empty() {
            errs.push(format!("{} element '{}' has an empty value", KNOB, e));
        } else if value.chars().any(char::is_whitespace) {
            errs.push(format!("{} element '{}' carries whitespace in its value", KNOB, e));
        } else if value == "inherit" {
            errs.push(format!(
                "{} element '{}' binds 'inherit', which is a dispatcher's tier and not a model — state it in the definition instead",
                KNOB, e
            ));
        } else if value.bytes().all(|b| b.is_ascii_digit()) {
            errs.push(format!(
                "{} element '{}' is all digits, a version token that would match across families",
                KNOB, e
            ));
        } else {
            vals.push(value);
        }
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

// spec: delegation-kit/SPEC.md §model-verdict — the classes whose bound value the id matches,
// highest first
pub fn classes_of(binding: &[String], id: &str) -> Vec<&'static str> {
    CLASSES
        .iter()
        .filter(|c| bound(binding, c).is_some_and(|v| matches(v, id)))
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(elems: &[&str]) -> Vec<String> {
        elems.iter().map(|s| s.to_string()).collect()
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
        assert_eq!(classes_of(&binding, "vendor-big-5"), vec!["judgment", "routing"]);
        assert_eq!(classes_of(&binding, "vendor-small-4"), vec!["mechanical"]);
        assert!(classes_of(&binding, "vendor-tiny-1").is_empty());
    }
}
