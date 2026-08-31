// spec: guard-kit/SPEC.md §escalation-guard — the PreToolUse(SendMessage) advisory member: it
// nudges a headerless escalation to the lead toward the decision shape and never blocks.
use crate::hook;
use serde_json::Value;

const NAME: &str = "escalation-guard";

// spec: guard-kit/SPEC.md §escalation-guard — the four decision-shape headers, in the order the
// advisory names them back
const HEADERS: &[&str] = &["Question", "Options", "Recommendation", "Evidence"];

pub fn run(payload: Option<&Value>) -> i32 {
    // spec: guard-kit/SPEC.md §escalation-guard — advisory posture: never block; an uninspectable
    // payload passes, which is the whole of this member's degraded path
    let to = hook::field(payload, &["tool_input", "to"]);
    // spec: guard-kit/SPEC.md §escalation-guard — only an upward message (to the lead as "main")
    // is an escalation
    if to != "main" {
        return 0;
    }
    let message = hook::field(payload, &["tool_input", "message"]);
    let missing: Vec<&str> = HEADERS
        .iter()
        .copied()
        .filter(|h| !contains_word(&message, h))
        .collect();
    if missing.is_empty() {
        return 0;
    }
    // spec: guard-kit/SPEC.md §escalation-guard — the advisory cites the *containing* section, as
    // every `spec:` line of the shell original did; the port preserves the text it shows a session
    // rather than improving a citation, which is a user-facing change this cut does not carry.
    hook::advise(&format!(
        "{}: this message to the lead is missing the decision-shape header(s): {}. An escalation batches every open question as Question / Options / Recommendation / Evidence so the lead can rule and resume you in place; routine narration belongs in the resume journal, not the message channel. (guard-kit/SPEC.md §wakeup-guard)",
        NAME,
        missing.join(" ")
    ))
}

// spec: guard-kit/SPEC.md §escalation-guard — `grep -qiw`: a case-insensitive match bounded by
// non-word characters on both sides, so `Questions` and `unquestioned` are not the header. The
// word class is grep's own, `[A-Za-z0-9_]`.
fn contains_word(haystack: &str, needle: &str) -> bool {
    let hay: Vec<char> = haystack.chars().collect();
    let pat: Vec<char> = needle.to_lowercase().chars().collect();
    if pat.is_empty() || hay.len() < pat.len() {
        return false;
    }
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    for start in 0..=hay.len() - pat.len() {
        let end = start + pat.len();
        if !hay[start..end]
            .iter()
            .zip(&pat)
            .all(|(a, b)| a.to_ascii_lowercase() == *b)
        {
            continue;
        }
        if start > 0 && is_word(hay[start - 1]) {
            continue;
        }
        if end < hay.len() && is_word(hay[end]) {
            continue;
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: guard-kit/SPEC.md §escalation-guard — the header test is `grep -qiw`, so case does not
    // matter and a word boundary does; a substring hit inside a longer word is not the header
    #[test]
    fn the_header_match_is_case_insensitive_and_word_bounded() {
        assert!(contains_word("Question: what now?", "Question"));
        assert!(contains_word("**question**", "Question"));
        assert!(!contains_word("unquestioned premise", "Question"));
        assert!(!contains_word("Questions", "Question"));
        assert!(contains_word("a Question", "Question"));
        assert!(!contains_word("", "Question"));
    }

    // spec: guard-kit/SPEC.md §escalation-guard — the kit's own advisory table, read from disk
    // rather than transcribed into Rust literals: the table is reviewable test data and a copy
    // here would trade that review for a recompile. This test replaces its shell driver.
    #[test]
    fn the_kits_advisory_table_routes_every_case() {
        let table = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../guard-kit/guard-tests/escalation-cases.tsv");
        let text = std::fs::read_to_string(&table).expect("the kit's advisory table must be read");
        let mut ran = 0usize;
        for line in text.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            assert!(cols.len() >= 3, "malformed case row: {}", line);
            let (want, to, message) = (cols[0], cols[1], cols[2]);
            let fires = to == "main" && HEADERS.iter().any(|h| !contains_word(message, h));
            let got = if fires { "advise" } else { "fallthrough" };
            assert_eq!(got, want, "case [-> {}: {}]", to, message);
            ran += 1;
        }
        assert_eq!(ran, 3, "the table's case count moved; read it before changing this");
    }

    // spec: guard-kit/SPEC.md §escalation-guard — only an upward message is an escalation, and an
    // uninspectable payload is not one either
    #[test]
    fn only_an_upward_message_is_inspected() {
        let sideways: Value = serde_json::from_str(r#"{"tool_input":{"to":"worker-1"}}"#)
            .expect("the fixture must parse");
        assert_eq!(run(Some(&sideways)), 0);
        assert_eq!(run(None), 0);
    }
}
