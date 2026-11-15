// spec: gate-sdk/SPEC.md §Output contract — the clean line's grammar, read by the runner's result
// classification and the fixture runner's assertion alike

pub const NOT_APPLICABLE: &str = "not applicable: ";

// spec: gate-sdk/SPEC.md §Output contract — the canonical clean line, `^[A-Z][A-Z0-9-]*: clean
// \(.*\)$`: the name admits no space or colon, so the first separator is the only one.
pub fn clean_line(l: &str) -> bool {
    parenthetical(l).is_some()
}

fn parenthetical(l: &str) -> Option<&str> {
    let (name, rest) = l.split_once(": clean (")?;
    let inner = rest.strip_suffix(')')?;
    let mut cs = name.chars();
    match cs.next() {
        Some(c) if c.is_ascii_uppercase() => {}
        _ => return None,
    }
    if !cs.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-') {
        return None;
    }
    Some(inner)
}

// spec: gate-sdk/SPEC.md §Output contract — a clean line whose parenthetical opens
// `not applicable: ` says the gate evaluated no instance of its rule
pub fn not_applicable(l: &str) -> bool {
    parenthetical(l).is_some_and(|p| p.starts_with(NOT_APPLICABLE))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §Output contract — the clean line's grammar, which a fixture pair
    // cannot pin: the runner asserts it about *other* members and owes no pair of its own.
    #[test]
    fn the_clean_line_grammar_admits_only_the_canonical_shape() {
        assert!(clean_line("CHECK-FOO: clean (3 files)"));
        assert!(clean_line("A: clean ()"));
        assert!(!clean_line("check-foo: clean (3 files)"), "the name is upper-case");
        assert!(!clean_line("CHECK FOO: clean (3)"), "the name admits no space");
        assert!(!clean_line("CHECK-FOO: clean 3 files"), "the parenthetical is required");
        assert!(!clean_line("CHECK-FOO: clean (3) and more"), "the line ends at the paren");
    }

    // spec: gate-sdk/SPEC.md §Output contract — the not-applicable form is a clean line, told
    // apart by its parenthetical's opening alone
    #[test]
    fn a_not_applicable_line_is_a_clean_line_opening_its_parenthetical_so() {
        assert!(not_applicable("CHECK-FOO: clean (not applicable: CHECK_FOO_GLOBS is empty)"));
        assert!(clean_line("CHECK-FOO: clean (not applicable: CHECK_FOO_GLOBS is empty)"));
        assert!(!not_applicable("CHECK-FOO: clean (3 files; not applicable: none)"));
        assert!(!not_applicable("CHECK-FOO: clean (not applicable)"));
        assert!(!not_applicable("check-foo: clean (not applicable: x)"));
    }
}
