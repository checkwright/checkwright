// spec: context-kit/SPEC.md §Index-first reading — one Markdown section by heading.
// spec: gate-sdk/SPEC.md §The non-gate arm — the class's first member whose table row exists for
// reachability rather than for configuration, its declared roster being empty.
use crate::section;

pub const KNOBS: &[&str] = &[];

// spec: context-kit/SPEC.md §Index-first reading — the query normalisation: a leading `§` is
// tolerated so a spec citation pastes directly, surrounding blanks go, and the compare is
// case-folded. The section walk beside this is `section::sections`; this is the matcher it is not.
fn needle(query: &str) -> String {
    query
        .trim_matches([' ', '\t'])
        .trim_start_matches('§')
        .trim_matches([' ', '\t'])
        .to_lowercase()
}

// spec: context-kit/SPEC.md §Index-first reading — a heading's *text*: the hashes and the blanks
// around them stripped, case-folded. Comparing text rather than a line prefix is what makes
// `## Testing` unreachable from a query for `Test`.
fn heading_text(line: &str) -> String {
    let level = section::heading_level(line);
    line[level..].trim_matches([' ', '\t']).to_lowercase()
}

// spec: context-kit/SPEC.md §Index-first reading — the fence mask, which is what lets the
// fence-aware contract reuse `section::sections` rather than widen it for its other callers.
fn fence_mask<'a>(lines: &[&'a str]) -> Vec<&'a str> {
    let mut out: Vec<&str> = Vec::with_capacity(lines.len());
    let mut in_fence = false;
    for line in lines {
        let l = line.trim_start_matches([' ', '\t']);
        let marker = l.starts_with("```") || l.starts_with("~~~");
        if marker || in_fence {
            out.push("");
        } else {
            out.push(line);
        }
        if marker {
            in_fence = !in_fence;
        }
    }
    out
}

// spec: context-kit/SPEC.md §Index-first reading — the located section as a half-open line range
// over the input's own lines: the matcher finds the opening heading, and the walk bounds it.
fn locate(lines: &[&str], query: &str) -> Option<(usize, usize)> {
    let want = needle(query);
    let masked = fence_mask(lines);
    let at = masked
        .iter()
        .position(|l| section::heading_level(l) > 0 && heading_text(l) == want)?;
    let bounded = section::sections(&masked[at..], masked[at]);
    let first = bounded.first()?;
    Some((at, at + first.end))
}

// spec: context-kit/SPEC.md §Index-first reading — the three refusals, all exit 2 once the
// dispatcher maps an error arm; that section carries which of them moved and why no caller read it.
pub fn emit(args: &[String]) -> Result<String, String> {
    let (file, query) = match (args.first(), args.get(1)) {
        (Some(f), Some(q)) if !f.is_empty() && !q.is_empty() => (f, q),
        _ => return Err("usage: --emit md-section <file.md> \"<heading>\"".to_string()),
    };
    if !std::path::Path::new(file).is_file() {
        return Err(format!("file not found: {}", file));
    }
    let text = super::read_text(file)?;
    let lines = section::split_lines(&text);
    let (start, end) = locate(&lines, query).ok_or(format!("no heading matched: {}", query))?;
    let mut out = String::new();
    for line in &lines[start..end] {
        out.push_str(line);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = "# Top\n\n## Alpha\n\nbody a\n\n```\n## NotAHeading\n# alsoNot\n```\n\nstill alpha\n\n## Beta\n\nbody b\n";

    // spec: context-kit/SPEC.md §Index-first reading — the three ways this matcher is not
    // `line.starts_with(name)`: case folds, a leading `§` is tolerated, and the compare is against
    // the heading's text rather than the line's prefix
    #[test]
    fn the_match_is_case_folded_section_tolerant_and_against_the_headings_text() {
        let lines = section::split_lines(DOC);
        assert!(locate(&lines, "Alpha").is_some());
        assert_eq!(locate(&lines, "alpha"), locate(&lines, "Alpha"));
        assert_eq!(locate(&lines, "\u{a7} ALPHA "), locate(&lines, "Alpha"));
        assert!(
            locate(&lines, "Alph").is_none(),
            "a prefix matched where the contract compares whole heading text"
        );
        assert!(
            locate(&lines, "## Alpha").is_none(),
            "the query is compared against the heading's text, so a hash-prefixed query \
             matches nothing — which is why a spec citation and not a raw heading line is \
             what pastes in"
        );
        assert!(locate(&lines, "Top").is_some());
    }

    // spec: context-kit/SPEC.md §Index-first reading — headings inside a fence are not structure at
    // either end: one does not open a section, and one does not close the section containing it
    #[test]
    fn a_fenced_heading_neither_opens_a_section_nor_closes_the_one_around_it() {
        let lines = section::split_lines(DOC);
        assert!(
            locate(&lines, "NotAHeading").is_none(),
            "a heading inside a fence opened a section"
        );
        let (start, end) = locate(&lines, "Alpha").expect("the Alpha section is absent");
        let body: Vec<&str> = lines[start..end].to_vec();
        assert_eq!(body.first(), Some(&"## Alpha"));
        assert!(
            body.contains(&"still alpha"),
            "the fenced heading closed the section early: {:?}",
            body
        );
        assert!(!body.contains(&"## Beta"), "the closing heading was printed");
    }

    // spec: context-kit/SPEC.md §Index-first reading — the walk's own rule, reused rather than
    // restated: a deeper heading stays inside and a same-or-shallower one ends the section
    #[test]
    fn a_deeper_heading_stays_inside_and_a_shallower_one_ends_the_section() {
        let doc = "# A\n\n## B\n\nb\n\n### C\n\nc\n\n## D\n\nd\n";
        let lines = section::split_lines(doc);
        let (start, end) = locate(&lines, "B").expect("the B section is absent");
        assert_eq!(&lines[start..end], &["## B", "", "b", "", "### C", "", "c", ""]);
        let (start, end) = locate(&lines, "A").expect("the A section is absent");
        assert_eq!(end - start, lines.len(), "a level-1 section ran short");
    }
}
