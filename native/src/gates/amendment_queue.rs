// spec: canon-kit/SPEC.md §check-amendment-queue — the Task↔amendment bidirectional rule and
// spec-readiness: the queue's section filing on one pass, then the on-disk pairing in both
// directions
use crate::spec;
use std::path::Path;

// spec: canon-kit/SPEC.md §check-amendment-queue — arm (d): the retired design-pending tag, red on
// any line of a feature, active or design-pending section
const RETIRED_TAG: &str = "[design-pending]";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-amendment-queue: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §The shared spec adapters — `SPEC_SECTION_RE` is `^## ` and nothing more, and the
// three classifying patterns are `^## (<name>|…)[[:space:]]*$` over an unescaped `|` join, so a
// heading's class is decided by comparing the trimmed name against the configured sets
enum Sec {
    Feature,
    Active,
    Deferred,
    Other,
}

struct Sets {
    feature: Vec<String>,
    active: Vec<String>,
    deferred: String,
    icebox: String,
}

fn classify(line: &str, s: &Sets) -> Option<Sec> {
    let name = trim_end_space(line.strip_prefix("## ")?);
    // spec: canon-kit/SPEC.md §check-amendment-queue — feature is tested first, so a section
    // named in both sets classifies as feature; the order is load-bearing, not incidental
    if s.feature.iter().any(|n| n == name) {
        return Some(Sec::Feature);
    }
    if s.active.iter().any(|n| n == name) {
        return Some(Sec::Active);
    }
    if name == s.deferred || (!s.icebox.is_empty() && name == s.icebox) {
        return Some(Sec::Deferred);
    }
    Some(Sec::Other)
}

fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\r' || c == 0x0b || c == 0x0c
}

fn trim_end_space(s: &str) -> &str {
    let b = s.as_bytes();
    let mut e = b.len();
    while e > 0 && is_space(b[e - 1]) {
        e -= 1;
    }
    &s[..e]
}

fn trim_space(s: &str) -> &str {
    let b = s.as_bytes();
    let mut i = 0usize;
    while i < b.len() && is_space(b[i]) {
        i += 1;
    }
    trim_end_space(&s[i..])
}

fn basename(p: &str) -> &str {
    match p.rfind('/') {
        Some(i) => &p[i + 1..],
        None => p,
    }
}

// spec: canon-kit/SPEC.md §check-amendment-queue — `grep -oE '\[spec:[[:space:]]*[^]]+\]'`
// piped through the two `sed` strips: every non-overlapping occurrence on the line, its
// content being whatever lies between the tag and the first `]` after at least one character
fn spec_refs_in(line: &str) -> Vec<String> {
    let b = line.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if !b[i..].starts_with(b"[spec:") {
            i += 1;
            continue;
        }
        let mut j = i + 6;
        while j < b.len() && is_space(b[j]) {
            j += 1;
        }
        // spec: canon-kit/SPEC.md §check-amendment-queue — `[^]]+` needs one character, and it
        // cannot cross a `]`, so the match ends at the first `]` at or after that character
        let mut k = j;
        while k < b.len() && b[k] != b']' {
            k += 1;
        }
        if k == j || k >= b.len() {
            i += 1;
            continue;
        }
        out.push(trim_space(&line[i + 6..k]).to_string());
        i = k + 1;
    }
    out
}

fn rule(args: &[String]) -> Result<i32, String> {
    let queue = match args.first() {
        Some(q) => q.clone(),
        None => spec::knob_pub("CANON_KIT_QUEUE_FILE")?,
    };
    let root = args.get(1).map(String::as_str).unwrap_or(".");
    if !Path::new(&queue).is_file() {
        return Err(format!("file not found: {}", queue));
    }
    let sets = Sets {
        feature: spec::knob_array_pub("CANON_KIT_FEATURE_SECTIONS")?,
        active: spec::knob_array_pub("CANON_KIT_ACTIVE_SECTIONS")?,
        deferred: spec::knob_pub("CANON_KIT_DEFERRED_SECTION")?,
        icebox: spec::knob_pub("CANON_KIT_ICEBOX_SECTION")?,
    };
    let text = spec::read_text(Path::new(&queue))?;

    let mut missing: Vec<String> = Vec::new();
    let mut dready: Vec<String> = Vec::new();
    let mut mready: Vec<String> = Vec::new();
    let mut retired: Vec<String> = Vec::new();

    // spec: canon-kit/SPEC.md §check-amendment-queue — awk's `sec` is unset until the first
    // heading, so a line above every section is classified by no arm; `Other` is that state
    let mut sec = Sec::Other;
    for (idx, line) in text.lines().enumerate() {
        if let Some(c) = classify(line, &sets) {
            sec = c;
            continue;
        }
        if matches!(sec, Sec::Other) {
            continue;
        }
        let at = format!("{}:{}: {}", queue, idx + 1, line);
        if line.contains(RETIRED_TAG) {
            retired.push(at.clone());
        }
        if !line.starts_with("- ") {
            continue;
        }
        match sec {
            Sec::Feature if !line.contains("[spec:") => missing.push(at),
            Sec::Active if line.contains("[spec:") => mready.push(at),
            Sec::Deferred if line.contains("[spec:") => dready.push(at),
            _ => {}
        }
    }

    let mut errors = String::new();
    let mut block = |head: &str, items: &[String]| {
        if items.is_empty() {
            return;
        }
        errors.push_str(head);
        errors.push('\n');
        for it in items {
            errors.push_str("  ");
            errors.push_str(it);
            errors.push('\n');
        }
    };
    block(
        "feature-section entries without [spec:] (spec-writing is an authoring-stage activity — write the amendment, then promote):",
        &missing,
    );
    block(
        "design-pending-section entries already carrying [spec:] (promote to a feature section):",
        &dready,
    );
    block(
        "[spec:]-tagged entries misfiled in an active non-feature section (a spec-ready entry belongs in a feature section):",
        &mready,
    );
    block(
        "[design-pending] is retired — section membership is the state; delete the tag:",
        &retired,
    );

    // spec: canon-kit/SPEC.md §check-amendment-queue — bidirectional pairing on disk. The
    // amendment set is enumerated once and reused for both directions, where the shell re-runs
    // the finder per reference; the walk is the same walk and the verdict cannot differ.
    let disk: Vec<String> = spec::amendments(root)?
        .into_iter()
        .map(|p| p.display().to_string())
        .collect();

    let mut refs: Vec<String> = Vec::new();
    for line in text.lines() {
        refs.extend(spec_refs_in(line));
    }
    refs.sort();
    refs.dedup();

    let mut ref_bases: Vec<String> = Vec::new();
    for r in &refs {
        if r.is_empty() {
            continue;
        }
        ref_bases.push(basename(r).to_string());
        if r.contains('/') {
            if !Path::new(r).is_file() {
                errors.push_str(&format!(
                    "queue references [spec: {}] but no such file exists at that path\n",
                    r
                ));
            }
        } else if !disk.iter().any(|f| basename(f) == r.as_str()) {
            errors.push_str(&format!(
                "queue references [spec: {}] but no amendment file named {} exists on disk\n",
                r, r
            ));
        }
    }

    for f in &disk {
        let base = basename(f);
        if !ref_bases.iter().any(|b| b == base) {
            errors.push_str(&format!(
                "amendment on disk with no queue entry: {} (expected a task tagged [spec: {}])\n",
                f, base
            ));
        }
    }

    if !errors.is_empty() {
        println!("check-amendment-queue: Task↔amendment bidirectional-rule violation(s):");
        println!();
        print!("{}", errors);
        println!("  help: pair every amendment with a [spec: …] queue entry and vice versa; delete every retired [design-pending] tag (section membership is the state); give every feature entry a [spec:] ref");
        return Ok(1);
    }

    println!("AMENDMENT-QUEUE: clean (every amendment ↔ a queue entry; feature entries spec-ready; no retired design-pending tag)");
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spec_ref_is_the_content_between_the_tag_and_the_first_bracket() {
        assert_eq!(spec_refs_in("- **a** [spec: SPEC-x.md] — p"), vec!["SPEC-x.md"]);
        assert_eq!(
            spec_refs_in("[spec:a] and [spec:  b/c.md  ]"),
            vec!["a", "b/c.md"]
        );
        assert!(spec_refs_in("[spec:]").is_empty());
        assert!(spec_refs_in("[spec: unterminated").is_empty());
    }

    // spec: canon-kit/SPEC.md §check-amendment-queue — a section named in both the feature and
    // the active set is a feature section, because the feature set is tested first
    #[test]
    fn the_feature_set_wins_over_the_active_set_on_a_shared_name() {
        let s = Sets {
            feature: vec!["New Features".into()],
            active: vec!["New Features".into(), "Technical Debt".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
        };
        assert!(matches!(
            classify("## New Features", &s),
            Some(Sec::Feature)
        ));
        assert!(matches!(
            classify("## Technical Debt  ", &s),
            Some(Sec::Active)
        ));
        assert!(matches!(classify("## Deferred", &s), Some(Sec::Deferred)));
        assert!(matches!(classify("## Done", &s), Some(Sec::Other)));
        assert!(classify("- **x** — y", &s).is_none());
    }

    // spec: canon-kit/SPEC.md §check-amendment-queue — an empty icebox knob omits the term
    // rather than matching a section whose heading is empty
    #[test]
    fn an_empty_icebox_knob_matches_no_section() {
        let s = Sets {
            feature: Vec::new(),
            active: Vec::new(),
            deferred: "Deferred".into(),
            icebox: String::new(),
        };
        assert!(matches!(classify("## ", &s), Some(Sec::Other)));
    }
}
