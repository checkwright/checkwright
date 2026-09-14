// spec: canon-kit/SPEC.md §Layout and configuration — canon-kit's static knob table and validator
use super::{indexed, scalar, Kit, Resolve, Row, Shape, Value, Values};

fn queue_file(resolve: Resolve) -> Result<Value, String> {
    resolve("GATE_SDK_QUEUE_FILE").map(|(v, _)| v)
}

pub const KIT: Kit = Kit {
    root: "canon-kit",
    rows: &[
        Row::scalar("CANON_KIT_SPEC_NAME", "SPEC.md"),
        Row::scalar("CANON_KIT_AMENDMENT_GLOB", "SPEC-*.md"),
        Row::derived("CANON_KIT_QUEUE_FILE", Shape::Scalar, queue_file, &["GATE_SDK_QUEUE_FILE"]),
        Row::indexed("CANON_KIT_DEPRECATION_MARKERS", &[]),
        Row::indexed("CANON_KIT_FEATURE_SECTIONS", &["New Features"]),
        Row::indexed("CANON_KIT_ACTIVE_SECTIONS", &["New Features", "Technical Debt"]),
        Row::scalar("CANON_KIT_DEFERRED_SECTION", "Deferred"),
        Row::scalar("CANON_KIT_ICEBOX_SECTION", ""),
        Row::scalar("CANON_KIT_DOD_HEADING", "Definition of Done"),
        Row::scalar("CANON_KIT_DOD_MODE", "exactly-one"),
        Row::scalar("CANON_KIT_SCAN_KIT_ROOTS", "0"),
        Row::indexed(
            "CANON_KIT_BANNED_HEADINGS",
            &["Directory Structure", "Public API", "Cargo.toml Dependencies"],
        ),
        Row::scalar("CANON_KIT_DERIVABLE_DENSITY", "60"),
        Row::scalar("CANON_KIT_DERIVABLE_POINTER_REGEX", "pub-index|proto-index"),
        Row::scalar("CANON_KIT_EMBED_THRESHOLD", "0.70"),
        Row::scalar("CANON_KIT_EMBED_MINLINES", "8"),
        Row::indexed(
            "CANON_KIT_EMBED_LANGS",
            &[
                "rs|rust,rs|*.rs",
                "toml|toml|*.toml",
                "sql|sql|*.sql",
                "sh|bash,sh|*.sh",
                "yaml|yaml,yml|*.yaml,*.yml",
                "ts|typescript,ts,tsx|*.ts,*.tsx",
                "rego|rego|*.rego",
                "proto|proto,protobuf|*.proto",
                "dockerfile|dockerfile|Dockerfile",
            ],
        ),
        Row::indexed("CANON_KIT_EMBED_ILLUSTRATIVE", &["json"]),
        Row::scalar("CANON_KIT_EMBED_WIRE_KIND", "proto"),
        Row::scalar("CANON_KIT_GLOSSARY_FILE", "GLOSSARY.md"),
        Row::indexed("CANON_KIT_DUP_SURFACES", &["VISION.md"]),
        Row::indexed("CANON_KIT_MDREF_EXCLUDE", &[]),
        Row::indexed("CANON_KIT_RETIRED_SPELLING_EXCLUDE", &[]),
        Row::scalar("CANON_KIT_LINK_ROOT", "docs"),
        Row::scalar("CANON_KIT_DOCS_BLOB_REF", "master"),
        Row::indexed("CANON_KIT_MANIFEST_FILES", &[]),
        Row::indexed("CANON_KIT_PROSE_SURFACE_GLOBS", &[]),
        Row::indexed(
            "CANON_KIT_TEMPORAL_MARKERS",
            &[
                "previously",
                "formerly",
                "renamed from",
                "no longer",
                "used to be",
                "was (retired|removed|renamed|replaced)",
            ],
        ),
        Row::indexed("CANON_KIT_TEMPORAL_MARKERS_EXTRA", &[]),
        Row::indexed("CANON_KIT_TEMPORAL_EXEMPT_SECTIONS", &[]),
        Row::indexed("CANON_KIT_TEMPORAL_EXEMPT_PATHS", &[]),
        Row::indexed(
            "CANON_KIT_COUNT_COLLECTIONS",
            &["gates", "meta-gates", "checks", "kits", "stages", "rules", "KPIs"],
        ),
        Row::indexed("CANON_KIT_COUNT_ALLOWED_PHRASES", &[]),
        Row::scalar("CANON_KIT_COUNT_WEDGE_WORDS", "2"),
        Row::indexed("CANON_KIT_ENUM_SETS_CMD", &[]),
        Row::indexed("CANON_KIT_INSTALL_TRANSPORTS_CMD", &[]),
        Row::scalar("CANON_KIT_INSTALL_SECTION_RE", ""),
        Row::indexed("CANON_KIT_INSTALL_CLAIM_EXCLUDE", &[]),
        Row::indexed("CANON_KIT_PAYLOAD_CLAIMS_CMD", &[]),
        Row::indexed("CANON_KIT_PAYLOAD_CLAIM_EXCLUDE", &[]),
        Row::indexed("CANON_KIT_MEASURED_CLAIMS_CMD", &[]),
        Row::indexed("CANON_KIT_MEASURED_SURFACE_GLOBS", &[]),
        Row::indexed("CANON_KIT_CLAIM_CLASSES_CMD", &[]),
        Row::indexed("CANON_KIT_COMMENT_MACHINE", &[]),
        Row::indexed("CANON_KIT_COMMENT_REASON", &[]),
        Row::indexed("CANON_KIT_COMMENT_SURFACE", &[]),
        Row::indexed("CANON_KIT_COMMENT_POSITIONAL", &[]),
        Row::indexed("CANON_KIT_COMMENT_WHITELIST", &[]),
        Row::scalar("CANON_KIT_COMMENT_RUN_CAP", "3"),
        Row::indexed("CANON_KIT_PROSE_TELL_GLOBS", &[]),
        Row::scalar("CANON_KIT_PROSE_TELL_EMDASH_MAX", "2"),
        Row::scalar("CANON_KIT_PROSE_TELL_CONTRAST_MAX", "1"),
        Row::scalar("CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES", "4"),
        Row::scalar("CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN", "0.25"),
        Row::scalar("CANON_KIT_PROSE_TELL_TRICOLON_MAX", "2"),
        Row::indexed(
            "CANON_KIT_PROSE_TELL_PHRASES",
            &[
                "It's worth noting",
                "It is worth noting",
                "It's important to note",
                "That said",
                "Needless to say",
                "It goes without saying",
            ],
        ),
        Row::indexed("CANON_KIT_PROSE_TELL_PHRASES_EXTRA", &[]),
        Row::indexed(
            "CANON_KIT_PROSE_TELL_ABBR_ALLOW",
            &["API", "CLI", "URL", "HTML", "CSS", "JSON", "YAML", "CI", "SDK", "SSO", "DNS", "HTTPS"],
        ),
        Row::indexed("CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA", &[]),
    ],
    validate: Some(("spec config", validate)),
    open_family: false,
    families: &[],
    retired: &[],
};

fn digits(v: &str) -> bool {
    !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit())
}

fn positive(v: &str) -> bool {
    digits(v) && v.bytes().any(|b| b != b'0')
}

// spec: canon-kit/SPEC.md §Layout and configuration — `^0?\.[0-9]+$|^1(\.0+)?$`
fn fraction(v: &str) -> bool {
    let zero = v.strip_prefix('0').unwrap_or(v);
    if let Some(d) = zero.strip_prefix('.') {
        return digits(d);
    }
    match v.strip_prefix('1') {
        Some("") => true,
        Some(rest) => rest.strip_prefix('.').is_some_and(|z| !z.is_empty() && z.bytes().all(|b| b == b'0')),
        None => false,
    }
}

// spec: canon-kit/SPEC.md §Layout and configuration — a broken spec config gates nothing: emptiness,
// the enumerated switches, the numeric ranges, and the icebox apart from the deferred section
fn validate(v: &Values) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    let empty = |n: &str| scalar(v, n).is_some_and(str::is_empty);
    let empty_list = |n: &str| indexed(v, n).is_some_and(|e| e.is_empty());
    for n in ["CANON_KIT_SPEC_NAME", "CANON_KIT_AMENDMENT_GLOB"] {
        if empty(n) {
            errs.push(format!("{} is empty", n));
        }
    }
    for n in ["CANON_KIT_FEATURE_SECTIONS", "CANON_KIT_ACTIVE_SECTIONS"] {
        if empty_list(n) {
            errs.push(format!("{} is empty", n));
        }
    }
    if empty("CANON_KIT_DEFERRED_SECTION") {
        errs.push("CANON_KIT_DEFERRED_SECTION is empty".to_string());
    }
    if scalar(v, "CANON_KIT_ICEBOX_SECTION") == scalar(v, "CANON_KIT_DEFERRED_SECTION") {
        errs.push("CANON_KIT_ICEBOX_SECTION must not name the deferred section".to_string());
    }
    if empty("CANON_KIT_DOD_HEADING") {
        errs.push("CANON_KIT_DOD_HEADING is empty".to_string());
    }
    if let Some(m) = scalar(v, "CANON_KIT_DOD_MODE").filter(|m| *m != "exactly-one" && *m != "at-most-one") {
        errs.push(format!("CANON_KIT_DOD_MODE must be exactly-one|at-most-one (got '{}')", m));
    }
    if let Some(s) = scalar(v, "CANON_KIT_SCAN_KIT_ROOTS").filter(|s| *s != "0" && *s != "1") {
        errs.push(format!("CANON_KIT_SCAN_KIT_ROOTS must be 0|1 (got '{}')", s));
    }
    if let Some(d) = scalar(v, "CANON_KIT_DERIVABLE_DENSITY") {
        if !digits(d) || d.parse::<u64>().map_or(true, |n| n > 100) {
            errs.push(format!("CANON_KIT_DERIVABLE_DENSITY must be 0..100 (got '{}')", d));
        }
    }
    if let Some(t) = scalar(v, "CANON_KIT_EMBED_THRESHOLD").filter(|t| !fraction(t)) {
        errs.push(format!("CANON_KIT_EMBED_THRESHOLD must be a 0..1 fraction (got '{}')", t));
    }
    for n in ["CANON_KIT_EMBED_MINLINES", "CANON_KIT_COUNT_WEDGE_WORDS", "CANON_KIT_COMMENT_RUN_CAP"] {
        if let Some(s) = scalar(v, n).filter(|s| !positive(s)) {
            errs.push(format!("{} must be a positive integer (got '{}')", n, s));
        }
    }
    for n in ["CANON_KIT_GLOSSARY_FILE", "CANON_KIT_DOCS_BLOB_REF"] {
        if empty(n) {
            errs.push(format!("{} is empty", n));
        }
    }
    // spec: canon-kit/SPEC.md §Layout and configuration — the marker set a reader sees is the base
    // followed by its extra, so the set is empty only when both are
    if empty_list("CANON_KIT_TEMPORAL_MARKERS") && empty_list("CANON_KIT_TEMPORAL_MARKERS_EXTRA") {
        errs.push("CANON_KIT_TEMPORAL_MARKERS is empty".to_string());
    }
    if empty_list("CANON_KIT_COUNT_COLLECTIONS") {
        errs.push("CANON_KIT_COUNT_COLLECTIONS is empty".to_string());
    }
    errs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embed_threshold_is_a_fraction_in_the_unit_interval() {
        for ok in ["0.70", ".5", "1", "1.0", "1.00", "0.0"] {
            assert!(fraction(ok), "{}", ok);
        }
        for bad in ["", "0", "1.5", "2", "0.", "1.", "a.1", "00.1"] {
            assert!(!fraction(bad), "{}", bad);
        }
    }
}
