// spec: canon-kit/SPEC.md §Layout and configuration — canon-kit's static knob table and validator
use super::{indexed, scalar, Kit, Packing, Resolve, Row, Shape, Value, Values};

// spec: canon-kit/SPEC.md §Layout and configuration — `CANON_KIT_EMBED_LANGS`' element packing
const EMBED_LANGS_PACKING: Packing = Packing {
    sep: '|',
    fields: &[("kind", None), ("fence-langs", Some(',')), ("file-globs", Some(','))],
};

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
        )
        .packed(&EMBED_LANGS_PACKING),
        Row::indexed("CANON_KIT_EMBED_ILLUSTRATIVE", &["json"]),
        Row::scalar("CANON_KIT_EMBED_WIRE_KIND", "proto"),
        Row::scalar("CANON_KIT_GLOSSARY_FILE", "GLOSSARY.md"),
        Row::indexed("CANON_KIT_DUP_SURFACES", &["VISION.md"]),
        Row::indexed("CANON_KIT_MDREF_EXCLUDE", &[]),
        Row::indexed("CANON_KIT_UNWRAP_GLOBS", &[]),
        Row::indexed("CANON_KIT_UNWRAP_EXCLUDE", &[]),
        Row::indexed("CANON_KIT_UNWRAP_DECLARATION_LEADS", &[]),
        Row::indexed("CANON_KIT_RESTATEMENT_PAGES", &[]),
        Row::indexed(
            "CANON_KIT_FENCE_PROGRAMS",
            &[
                "awk", "basename", "bash", "cat", "chmod", "cmp", "cp", "cut", "date", "diff",
                "dirname", "env", "expr", "find", "git", "grep", "head", "ln", "ls", "mkdir",
                "mktemp", "mv", "nohup", "od", "paste", "rm", "rmdir", "sed", "sh", "sleep",
                "sort", "tail", "tar", "tee", "touch", "tr", "uname", "uniq", "wc", "xargs",
            ],
        ),
        Row::indexed("CANON_KIT_FENCE_PROGRAMS_EXTRA", &[]),
        Row::indexed("CANON_KIT_FENCE_RUN_PROGRAMS", &["git"]),
        Row::indexed("CANON_KIT_RETIRED_SPELLING_EXCLUDE", &[]),
        Row::scalar("CANON_KIT_LINK_ROOT", "docs"),
        Row::scalar("CANON_KIT_MIRROR_ROOT", "docs"),
        Row::scalar("CANON_KIT_DOCS_BLOB_REF", "master"),
        Row::indexed("CANON_KIT_MANIFEST_FILES", &[]),
        Row::indexed("CANON_KIT_PROSE_SURFACE_GLOBS", &[]),
        Row::scalar("CANON_KIT_KNOB_CITATION_REACH", "100"),
        Row::scalar("CANON_KIT_KNOB_CITATION_LITERAL_SPAN", "24"),
        Row::scalar("CANON_KIT_DEFAULT_COUPLING_WINDOW", "400"),
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
            "CANON_KIT_SEAM_AUTHORITY_MARKERS",
            &[
                "operator( |-)(ruled|ruling|direction|directed|decided|ratified|approved|chose)",
                "lead, ",
                "own-authority",
                "ruled",
                "ratified",
                "consult",
            ],
        ),
        Row::indexed("CANON_KIT_SEAM_AUTHORITY_MARKERS_EXTRA", &[]),
        Row::indexed("CANON_KIT_SEAM_AGENT_FILES", &["CLAUDE.md"]),
        Row::indexed("CANON_KIT_SEAM_PRIVATE_SURFACES", &[]),
        Row::scalar("CANON_KIT_SEAM_SLUG_MIN_LEN", "12"),
        Row::indexed("CANON_KIT_SEAM_SURFACE_GLOBS", &[]),
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
        Row::scalar("CANON_KIT_MEASURED_SPAN", "paragraph"),
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
        Row::scalar("CANON_KIT_PROSE_TELL_ABBR_MIN_LEN", "3"),
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
    env_only: &[],
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

// spec: canon-kit/SPEC.md §Layout and configuration — a non-negative decimal, `^[0-9]+$|^[0-9]*\.[0-9]+$`:
// unlike `fraction`, unbounded above one, since a coefficient of variation may exceed it
fn decimal(v: &str) -> bool {
    match v.split_once('.') {
        Some((int, frac)) => (int.is_empty() || digits(int)) && digits(frac),
        None => digits(v),
    }
}

// spec: canon-kit/SPEC.md §Layout and configuration — an integer of at least two: a coefficient of
// variation over one sentence is zero by construction, so a floor of one would red every
// one-sentence paragraph
fn min_sentences(v: &str) -> bool {
    digits(v) && v.parse::<u64>().is_ok_and(|n| n >= 2)
}

// spec: canon-kit/SPEC.md §Layout and configuration — at one letter every capital is an
// abbreviation, so the floor is two
fn abbr_min_len(v: &str) -> bool {
    v == "off" || (digits(v) && v.parse::<usize>().is_ok_and(|n| n >= 2))
}

// spec: canon-kit/SPEC.md §Layout and configuration — the mirror root is one tracked directory
// below the repository root: a root mirror would overwrite its own sources, and one outside the
// tree cannot be tracked
pub(crate) fn mirror_root_refusal(v: &str) -> Option<&'static str> {
    let t = v.trim_end_matches('/');
    let t = t.strip_prefix("./").unwrap_or(t);
    if t.is_empty() || t == "." {
        return Some("must name a directory below the repository root");
    }
    if crate::walk::path_root(t).is_some() {
        return Some("must be repository-relative, not absolute");
    }
    if t.split(['/', '\\']).any(|seg| seg == "..") {
        return Some("must not carry a '..' segment");
    }
    None
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
    if let Some(s) = scalar(v, "CANON_KIT_MEASURED_SPAN").filter(|s| !matches!(*s, "paragraph" | "sentence" | "off")) {
        errs.push(format!("CANON_KIT_MEASURED_SPAN must be paragraph|sentence|off (got '{}')", s));
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
    for n in [
        "CANON_KIT_EMBED_MINLINES",
        "CANON_KIT_COUNT_WEDGE_WORDS",
        "CANON_KIT_COMMENT_RUN_CAP",
        "CANON_KIT_SEAM_SLUG_MIN_LEN",
        "CANON_KIT_KNOB_CITATION_LITERAL_SPAN",
    ] {
        if let Some(s) = scalar(v, n).filter(|s| !positive(s)) {
            errs.push(format!("{} must be a positive integer (got '{}')", n, s));
        }
    }
    if let Some(s) = scalar(v, "CANON_KIT_KNOB_CITATION_REACH").filter(|s| !positive(s) && !matches!(*s, "sentence" | "off")) {
        errs.push(format!("CANON_KIT_KNOB_CITATION_REACH must be a positive integer, sentence or off (got '{}')", s));
    }
    if let Some(s) = scalar(v, "CANON_KIT_DEFAULT_COUPLING_WINDOW").filter(|s| !positive(s) && *s != "off") {
        errs.push(format!("CANON_KIT_DEFAULT_COUPLING_WINDOW must be a positive integer or off (got '{}')", s));
    }
    for n in ["CANON_KIT_GLOSSARY_FILE", "CANON_KIT_DOCS_BLOB_REF"] {
        if empty(n) {
            errs.push(format!("{} is empty", n));
        }
    }
    if let Some((s, why)) = scalar(v, "CANON_KIT_MIRROR_ROOT").and_then(|s| mirror_root_refusal(s).map(|w| (s, w))) {
        errs.push(format!("CANON_KIT_MIRROR_ROOT {} (got '{}')", why, s));
    }
    // spec: canon-kit/SPEC.md §Layout and configuration — the marker set a reader sees is the base
    // followed by its extra, so the set is empty only when both are
    if empty_list("CANON_KIT_TEMPORAL_MARKERS") && empty_list("CANON_KIT_TEMPORAL_MARKERS_EXTRA") {
        errs.push("CANON_KIT_TEMPORAL_MARKERS is empty".to_string());
    }
    if empty_list("CANON_KIT_SEAM_AUTHORITY_MARKERS") && empty_list("CANON_KIT_SEAM_AUTHORITY_MARKERS_EXTRA") {
        errs.push("CANON_KIT_SEAM_AUTHORITY_MARKERS is empty".to_string());
    }
    if empty_list("CANON_KIT_COUNT_COLLECTIONS") {
        errs.push("CANON_KIT_COUNT_COLLECTIONS is empty".to_string());
    }
    // spec: canon-kit/SPEC.md §check-prose-tells — the thresholds are validated here, not
    // coerced by the gate: a malformed one refuses every canon-kit gate at exit 2 with the knob
    // named, never reads as zero
    for n in [
        "CANON_KIT_PROSE_TELL_EMDASH_MAX",
        "CANON_KIT_PROSE_TELL_CONTRAST_MAX",
        "CANON_KIT_PROSE_TELL_TRICOLON_MAX",
    ] {
        if let Some(s) = scalar(v, n).filter(|s| !digits(s)) {
            errs.push(format!("{} must be a non-negative integer (got '{}')", n, s));
        }
    }
    if let Some(s) = scalar(v, "CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES").filter(|s| !min_sentences(s)) {
        errs.push(format!(
            "CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES must be an integer >= 2 (got '{}')",
            s
        ));
    }
    if let Some(s) = scalar(v, "CANON_KIT_PROSE_TELL_ABBR_MIN_LEN").filter(|s| !abbr_min_len(s)) {
        errs.push(format!(
            "CANON_KIT_PROSE_TELL_ABBR_MIN_LEN must be an integer >= 2 or off (got '{}')",
            s
        ));
    }
    if let Some(s) = scalar(v, "CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN").filter(|s| !decimal(s)) {
        errs.push(format!(
            "CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN must be a non-negative decimal (got '{}')",
            s
        ));
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

    // spec: canon-kit/SPEC.md §Layout and configuration — a non-negative decimal is unbounded
    // above one, unlike the embed threshold's unit fraction
    #[test]
    fn a_decimal_is_non_negative_and_unbounded_above_one() {
        for ok in ["0", "2", "0.25", ".5", "10.5", "007"] {
            assert!(decimal(ok), "{}", ok);
        }
        for bad in ["", "-1", "1.", ".", "a.1", "1.a"] {
            assert!(!decimal(bad), "{}", bad);
        }
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — the em-dash/contrast/tricolon maxes: zero is a
    // legitimate ban, and a decimal or negative value is refused rather than coerced to it
    #[test]
    fn a_prose_tell_max_is_a_non_negative_integer() {
        for ok in ["0", "2", "007"] {
            assert!(digits(ok), "{}", ok);
        }
        for bad in ["", "-1", "1.5", "a", "2 "] {
            assert!(!digits(bad), "{}", bad);
        }
    }

    // spec: canon-kit/SPEC.md §Layout and configuration — the mirror root refuses the root itself,
    // an absolute path and a `..` segment
    #[test]
    fn the_mirror_root_is_a_directory_below_the_repository_root() {
        for ok in ["docs", "docs/ref", "./site/", "site/ref/"] {
            assert!(mirror_root_refusal(ok).is_none(), "{}", ok);
        }
        for bad in ["", ".", "./", "/docs", "C:/docs", "../docs", "docs/../x"] {
            assert!(mirror_root_refusal(bad).is_some(), "{}", bad);
        }
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — the rhythm floor is an integer of at least two
    #[test]
    fn the_rhythm_floor_is_an_integer_of_at_least_two() {
        for ok in ["2", "4", "10"] {
            assert!(min_sentences(ok), "{}", ok);
        }
        for bad in ["", "0", "1", "-1", "2.0", "a"] {
            assert!(!min_sentences(bad), "{}", bad);
        }
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — the abbreviation floor is two or more, or off
    #[test]
    fn the_abbreviation_floor_is_at_least_two_or_off() {
        for ok in ["2", "3", "10", "off"] {
            assert!(abbr_min_len(ok), "{}", ok);
        }
        for bad in ["", "0", "1", "-1", "2.0", "OFF", "a"] {
            assert!(!abbr_min_len(bad), "{}", bad);
        }
    }
}
