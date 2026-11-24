// spec: gate-sdk/SPEC.md §run-gates — the release stamp `build.rs` computes, pure over its two
// inputs so the crate's tests can reach it: `build.rs` includes this file by `#[path]`, since
// `cargo test` never compiles a build script's own tests

// spec: gate-sdk/SPEC.md §run-gates — the build input outranks the describe answer, each loses a
// leading `v`, and the stamp is exact for a build input or a describe answer with no distance
#[cfg_attr(not(test), allow(dead_code))]
pub fn stamp(build_input: Option<&str>, describe: Option<&str>) -> (String, bool) {
    if let Some(v) = build_input.map(str::trim).filter(|v| !v.is_empty()) {
        return (strip_v(v).to_string(), true);
    }
    match describe.map(str::trim).filter(|d| !d.is_empty()) {
        Some(d) => {
            let s = strip_v(d);
            (s.to_string(), !has_distance(s))
        }
        None => (String::new(), false),
    }
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
        assert_eq!(stamp(Some("v0.36.0"), Some("v0.35.0-6-gabc1234")), ("0.36.0".into(), true));
        assert_eq!(stamp(Some("0.36.0"), None), ("0.36.0".into(), true));
        assert_eq!(stamp(Some(""), Some("v0.36.0")), ("0.36.0".into(), true));
    }

    // spec: gate-sdk/SPEC.md §run-gates — a describe answer is exact only with no distance suffix
    #[test]
    fn a_describe_answer_is_exact_only_on_the_tag_itself() {
        assert_eq!(stamp(None, Some("v0.36.0")), ("0.36.0".into(), true));
        assert_eq!(stamp(None, Some("v0.36.0-6-gabc1234\n")), ("0.36.0-6-gabc1234".into(), false));
        assert_eq!(stamp(None, Some("v1.0.0-rc.1")), ("1.0.0-rc.1".into(), true));
        assert_eq!(stamp(None, Some("v1.0.0-rc-12-g0f3")), ("1.0.0-rc-12-g0f3".into(), false));
    }

    // spec: gate-sdk/SPEC.md §run-gates — no input and no describe answer is an empty stamp
    #[test]
    fn no_answer_is_an_empty_inexact_stamp() {
        assert_eq!(stamp(None, None), (String::new(), false));
        assert_eq!(stamp(None, Some("  ")), (String::new(), false));
    }
}
