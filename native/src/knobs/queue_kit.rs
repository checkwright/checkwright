// spec: queue-kit/SPEC.md §Layout and configuration — queue-kit's static knob table and validator
use super::{indexed, scalar, Kit, Resolve, Row, Shape, Value, Values};

fn queue_file(resolve: Resolve) -> Result<Value, String> {
    resolve("GATE_SDK_QUEUE_FILE").map(|(v, _)| v)
}

pub const REQUIRED_SECTIONS: &[&str] = &[
    "Iteration:",
    "New Features",
    "Technical Debt",
    "Deferred",
    "Done",
    "Lessons Learned",
];

pub const KIT: Kit = Kit {
    root: "queue-kit",
    rows: &[
        Row::derived("QUEUE_KIT_QUEUE_FILE", Shape::Scalar, queue_file, &["GATE_SDK_QUEUE_FILE"]),
        Row::indexed("QUEUE_KIT_ACTIVE_SECTIONS", &["New Features", "Technical Debt"]),
        Row::scalar("QUEUE_KIT_DEFERRED_SECTION", "Deferred"),
        Row::scalar("QUEUE_KIT_ICEBOX_SECTION", ""),
        Row::scalar("QUEUE_KIT_DONE_SECTION", "Done"),
        Row::scalar("QUEUE_KIT_WRAP_BUDGET", "100"),
        Row::scalar("QUEUE_KIT_ENTRY_LINE_CAP", "50"),
        Row::scalar("QUEUE_KIT_ICEBOX_AGE_DAYS", "30"),
        Row::indexed("QUEUE_KIT_REQUIRED_SECTIONS", REQUIRED_SECTIONS),
        Row::indexed("QUEUE_KIT_PROSE_LEADS", &["Protocol:"]),
        Row::indexed("QUEUE_KIT_PROSE_SURFACE_GLOBS", &[]),
        Row::scalar(
            "QUEUE_KIT_PRECONDITION_REGEX",
            "revisit when|once [^.]*(lands|ships|is (done|ready|merged))|gated on|contingent on|waiting on|pending [a-z]|blocked on",
        ),
        Row::indexed("QUEUE_KIT_LESSON_TAGS", &[]),
        Row::keyed("QUEUE_KIT_LESSON_SINKS", &[]),
        Row::scalar("QUEUE_KIT_ATTEND_CAP", "3"),
        Row::indexed("QUEUE_KIT_HORIZONS", &[]),
        Row::indexed("QUEUE_KIT_TRACKS", &[]),
        Row::scalar("QUEUE_KIT_ROADMAP_FILE", ""),
        Row::scalar("QUEUE_KIT_ROADMAP_MARKER", "roadmap"),
    ],
    validate: Some(("queue config", validate)),
    open_family: false,
    families: &[],
    retired: &[],
    env_only: &[],
};

fn positive(v: &str) -> bool {
    !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()) && !v.trim_start_matches('0').is_empty()
}

// spec: queue-kit/SPEC.md §Layout and configuration — a broken grammar gates nothing: emptiness,
// positive integers, the icebox apart from the deferred section, and the roadmap vocabulary as a pair
fn validate(v: &Values) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();
    let empty_list = |n: &str| indexed(v, n).is_some_and(|e| e.is_empty());
    let empty = |n: &str| scalar(v, n).is_some_and(str::is_empty);
    if empty_list("QUEUE_KIT_ACTIVE_SECTIONS") {
        errs.push("QUEUE_KIT_ACTIVE_SECTIONS is empty".to_string());
    }
    for n in ["QUEUE_KIT_DEFERRED_SECTION", "QUEUE_KIT_DONE_SECTION"] {
        if empty(n) {
            errs.push(format!("{} is empty", n));
        }
    }
    for n in [
        "QUEUE_KIT_WRAP_BUDGET",
        "QUEUE_KIT_ATTEND_CAP",
        "QUEUE_KIT_ENTRY_LINE_CAP",
        "QUEUE_KIT_ICEBOX_AGE_DAYS",
    ] {
        if let Some(s) = scalar(v, n) {
            if !positive(s) {
                errs.push(format!("{} must be a positive integer (got '{}')", n, s));
            }
        }
    }
    if scalar(v, "QUEUE_KIT_ICEBOX_SECTION") == scalar(v, "QUEUE_KIT_DEFERRED_SECTION") {
        errs.push("QUEUE_KIT_ICEBOX_SECTION must not name the deferred section".to_string());
    }
    if empty("QUEUE_KIT_PRECONDITION_REGEX") {
        errs.push("QUEUE_KIT_PRECONDITION_REGEX is empty".to_string());
    }
    if empty_list("QUEUE_KIT_REQUIRED_SECTIONS") && empty("QUEUE_KIT_ICEBOX_SECTION") {
        errs.push("QUEUE_KIT_REQUIRED_SECTIONS is empty".to_string());
    }
    if empty("QUEUE_KIT_ROADMAP_MARKER") {
        errs.push("QUEUE_KIT_ROADMAP_MARKER is empty".to_string());
    }
    let (h, t) = (empty_list("QUEUE_KIT_HORIZONS"), empty_list("QUEUE_KIT_TRACKS"));
    if !h && t {
        errs.push("QUEUE_KIT_HORIZONS is set but QUEUE_KIT_TRACKS is empty — the roadmap vocabulary is configured as a pair".to_string());
    }
    if h && !t {
        errs.push("QUEUE_KIT_TRACKS is set but QUEUE_KIT_HORIZONS is empty — the roadmap vocabulary is configured as a pair".to_string());
    }
    errs
}
