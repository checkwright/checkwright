// spec: gate-sdk/SPEC.md §run-gates — the release stamp `build.rs` computes, pure over its two
// inputs so the crate's tests can reach it: `build.rs` includes this file by `#[path]`, since
// `cargo test` never compiles a build script's own tests

// spec: gate-sdk/SPEC.md §run-gates — the build input outranks the describe answer, each loses a
// leading `v`; outside the version grammar the build input is refused and describe empties
#[cfg_attr(not(test), allow(dead_code))]
pub fn stamp(build_input: Option<&str>, describe: Option<&str>) -> Result<(String, bool), String> {
    if let Some(v) = build_input.map(str::trim).filter(|v| !v.is_empty()) {
        let s = strip_v(v);
        if !version_token(s) {
            return Err(format!(
                "CHECKWRIGHT_BUILD_VERSION '{}' is not a release version: after a leading 'v' it takes ASCII letters, digits, '.', '+' and '-' only",
                v
            ));
        }
        return Ok((s.to_string(), true));
    }
    match describe.map(str::trim).map(strip_v).filter(|d| version_token(d)) {
        Some(s) => Ok((s.to_string(), !has_distance(s))),
        None => Ok((String::new(), false)),
    }
}

// spec: gate-sdk/SPEC.md §run-gates — the version grammar the stamp and `--verdict-lookup`'s
// `<kit-ver>` share
pub fn version_token(v: &str) -> bool {
    !v.is_empty() && v.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'+' | b'-'))
}

fn strip_v(s: &str) -> &str {
    s.strip_prefix('v').unwrap_or(s)
}

// spec: gate-sdk/SPEC.md §run-gates — `git describe`'s distance suffix, `-<n>-g<hash>`
pub fn has_distance(s: &str) -> bool {
    let mut parts = s.rsplitn(3, '-');
    let (Some(hash), Some(n), Some(_)) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    let hex = hash.strip_prefix('g').unwrap_or("");
    !n.is_empty()
        && n.bytes().all(|b| b.is_ascii_digit())
        && !hex.is_empty()
        && hex.bytes().all(|b| b.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §run-gates — the build input wins over describe, stripped of its `v`,
    // and is exact whatever describe answered
    #[test]
    fn the_build_input_outranks_describe_and_is_exact() {
        assert_eq!(stamp(Some("v0.36.0"), Some("v0.35.0-6-gabc1234")), Ok(("0.36.0".into(), true)));
        assert_eq!(stamp(Some("0.36.0"), None), Ok(("0.36.0".into(), true)));
        assert_eq!(stamp(Some(""), Some("v0.36.0")), Ok(("0.36.0".into(), true)));
    }

    // spec: gate-sdk/SPEC.md §run-gates — a describe answer is exact only with no distance suffix
    #[test]
    fn a_describe_answer_is_exact_only_on_the_tag_itself() {
        assert_eq!(stamp(None, Some("v0.36.0")), Ok(("0.36.0".into(), true)));
        assert_eq!(stamp(None, Some("v0.36.0-6-gabc1234\n")), Ok(("0.36.0-6-gabc1234".into(), false)));
        assert_eq!(stamp(None, Some("v1.0.0-rc.1")), Ok(("1.0.0-rc.1".into(), true)));
        assert_eq!(stamp(None, Some("v1.0.0-rc-12-g0f3")), Ok(("1.0.0-rc-12-g0f3".into(), false)));
    }

    // spec: gate-sdk/SPEC.md §run-gates — no input and no describe answer is an empty stamp
    #[test]
    fn no_answer_is_an_empty_inexact_stamp() {
        assert_eq!(stamp(None, None), Ok((String::new(), false)));
        assert_eq!(stamp(None, Some("  ")), Ok((String::new(), false)));
    }

    // spec: gate-sdk/SPEC.md §run-gates — a build input outside the grammar is refused by name,
    // so no stamped build prints a key the lookup refuses
    #[test]
    fn a_build_input_outside_the_grammar_is_refused_by_name() {
        let err = stamp(Some("v1.2.3_review"), Some("v1.2.3")).expect_err("refused");
        assert!(err.contains("'v1.2.3_review'"), "{}", err);
        assert!(stamp(Some("v"), None).is_err());
        assert!(stamp(Some("1.2.3 rc"), None).is_err());
    }

    // spec: gate-sdk/SPEC.md §run-gates — a describe answer outside the grammar empties the stamp
    #[test]
    fn a_describe_answer_outside_the_grammar_empties_the_stamp() {
        assert_eq!(stamp(None, Some("v1.2.3_review")), Ok((String::new(), false)));
        assert_eq!(stamp(None, Some("v1.2.3/x-2-gabc")), Ok((String::new(), false)));
        assert_eq!(stamp(None, Some("v")), Ok((String::new(), false)));
    }

    // spec: gate-sdk/SPEC.md §run-gates — every stamp a build accepts is a lookup operand
    #[test]
    fn every_accepted_stamp_is_a_version_token() {
        for (input, describe) in [(Some("v0.1.0+b.7"), None), (None, Some("v0.1.0-3-gabc"))] {
            let (s, _) = stamp(input, describe).expect("accepted");
            assert!(version_token(&s), "{}", s);
        }
    }
}
