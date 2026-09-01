// spec: lifecycle-kit/SPEC.md §The survey record — the citation affordance: one record block as an
// inline-ready snippet, so carrying a finding onto a permanent surface is one command rather than a
// pointer that dies at the next boundary.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and not a hardcoded flag, because the
// arm resolves the record knob and a hardcoded flag receives no consumer override at all.
use super::file_survey::{anchored, positionals};

pub const KNOBS: &[&str] = &["LIFECYCLE_KIT_SURVEY_RECORD_FILE"];

const USAGE: &str = "usage: --emit cite-survey [--] \"<heading-substring>\"\n  emits the one matching block of the survey record as an inline-ready snippet";

// spec: lifecycle-kit/SPEC.md §The survey record — the block's five fields in record order; a line
// outside this set is not part of the snippet, which is what keeps a pasted citation the witness
// rather than whatever prose happened to follow the heading.
const FIELDS: &[&str] = &["corpus", "oracle", "rev", "edges", "finding"];

fn is_field(line: &str) -> bool {
    FIELDS
        .iter()
        .any(|f| line.starts_with(&format!("- {}: ", f)))
}

// spec: lifecycle-kit/SPEC.md §The survey record — the discovery key is the `## ` heading, and the
// match is a plain substring of it: a later stage searches by the question it is about to ask.
fn headings<'a>(text: &'a str, needle: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|l| l.starts_with("## ") && l.contains(needle))
        .collect()
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let rest = positionals(args, "substring")?;
    let needle = match rest.first() {
        Some(n) if rest.len() == 1 && !n.is_empty() => n,
        _ => return Err(USAGE.to_string()),
    };
    let (record, spelled) = anchored("LIFECYCLE_KIT_SURVEY_RECORD_FILE")?;
    if !std::path::Path::new(&record).is_file() {
        return Err(format!("no survey record at {} — nothing to cite.", spelled));
    }
    let text = super::read_text(&record)?;
    let matched = headings(&text, needle);

    // spec: lifecycle-kit/SPEC.md §The survey record — both refusals keep printing what the author
    // needs to narrow: the record's own headings when nothing matched, every match when several
    // did. A silently-chosen sibling would be pasted onto a permanent surface as the one they read.
    if matched.is_empty() {
        let all = headings(&text, "");
        let listed = if all.is_empty() {
            "  (none)".to_string()
        } else {
            all.join("\n")
        };
        return Err(format!(
            "no block heading in {} contains: {}\n  the record carries these headings —\n{}",
            spelled, needle, listed
        ));
    }
    if matched.len() > 1 {
        return Err(format!(
            "{} block headings contain \"{}\" — narrow the substring:\n  {}",
            matched.len(),
            needle,
            matched.join("\n  ")
        ));
    }

    let head = matched[0];
    let mut out = format!(
        "**Carried survey — {}**\n",
        head.strip_prefix("## ").unwrap_or(head)
    );
    for line in text
        .lines()
        .skip_while(|l| *l != head)
        .skip(1)
        .take_while(|l| !l.starts_with("## "))
    {
        if is_field(line) {
            out.push_str(line);
            out.push('\n');
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REC: &str = "# contract: seeded\n\n## 2026-01-01 scope — alpha question\n- corpus: a\n- oracle: b\n- rev: c\n- edges: d\n- finding: e\nstray prose\n\n## 2026-01-02 build — beta question\n- corpus: f\n- oracle: g\n- rev: h\n- edges: i\n- finding: j\n";

    // spec: lifecycle-kit/SPEC.md §The survey record — the snippet is the heading reformatted plus
    // the five fields in record order, bounded by the next block's heading and nothing else
    #[test]
    fn the_snippet_is_the_heading_and_its_five_fields_bounded_by_the_next_block() {
        let one = headings(REC, "alpha");
        assert_eq!(one.len(), 1);
        let mut argvless = REC.lines().skip_while(|l| *l != one[0]).skip(1);
        assert_eq!(argvless.next(), Some("- corpus: a"));
        assert!(is_field("- finding: e"));
        assert!(!is_field("stray prose"), "unfielded prose entered the snippet");
        assert!(!is_field("- notafield: x"));
    }

    // spec: lifecycle-kit/SPEC.md §The survey record — an ambiguous substring is a refusal rather
    // than a first-match guess, and a substring matching nothing is one too
    #[test]
    fn an_ambiguous_or_unmatched_substring_selects_nothing() {
        assert_eq!(headings(REC, "question").len(), 2);
        assert!(headings(REC, "gamma").is_empty());
        assert!(
            headings(REC, "corpus").is_empty(),
            "a field line was read as a heading"
        );
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — this member carries one free-text positional
    // and the shape refusal reaches it too, `-h`/`--help` included
    #[test]
    fn its_one_positional_carries_the_shape_refusal() {
        let flagged: Vec<String> = vec!["--help".to_string()];
        assert!(positionals(&flagged, "substring").is_err());
        let escaped: Vec<String> = vec!["--".to_string(), "-alpha".to_string()];
        assert_eq!(
            positionals(&escaped, "substring").expect("the separator did not end option processing"),
            &escaped[1..]
        );
    }
}
