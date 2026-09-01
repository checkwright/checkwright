// spec: context-kit/SPEC.md §Index-first reading — the markdown structural index: heading
// hierarchy with line numbers, each heading's first sentence, and a per-file line count. The walk's
// one exclusion source is CONTEXT_KIT_PRUNE_DIRS and the arm declares nothing else.
use super::{corpus, read_text, relative, targets};
use crate::section;
use crate::walk;

pub const KNOBS: &[&str] = &["CONTEXT_KIT_PRUNE_DIRS"];

// spec: context-kit/SPEC.md §Index-first reading — this index's own heading rule, `^#{1,6}[ \t]+`,
// narrower than `section::heading_level`'s unbounded run and POSIX-space class and local for that
// reason: widening the shared rule would move every caller of it.
fn heading_level(line: &str) -> usize {
    let b = line.as_bytes();
    let mut n = 0usize;
    while n < b.len() && b[n] == b'#' {
        n += 1;
    }
    if n == 0 || n > 6 {
        return 0;
    }
    match b.get(n) {
        Some(b' ') | Some(b'\t') => n,
        _ => 0,
    }
}

// spec: context-kit/SPEC.md §Index-first reading — `wc -l`'s model: the count is newline bytes, so
// a file whose last line is unterminated counts one fewer than it has lines.
fn newlines(text: &str) -> usize {
    text.bytes().filter(|&b| b == b'\n').count()
}

// spec: context-kit/SPEC.md §Index-first reading — one link span measured from just past its
// opening bracket: the text length and the whole span's length. Both halves must be non-empty,
// which is what keeps `[]()` and a bare `[EDIT ME]` literal.
fn link_span(after: &str) -> Option<(usize, usize)> {
    let close = after.find(']').filter(|c| *c > 0)?;
    let inner = after[close + 1..].strip_prefix('(')?;
    let end = inner.find(')').filter(|e| *e > 0)?;
    Some((close, close + end + 3))
}

// spec: context-kit/SPEC.md §Index-first reading — the first-sentence rule's link reduction:
// `[text](target)` collapses to `text`, leftmost first, and anything that is not a whole link
// passes through as written.
fn strip_links(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        match link_span(after) {
            Some((text_len, span)) => {
                out.push_str(&rest[..open]);
                out.push_str(&after[..text_len]);
                rest = &after[span..];
            }
            None => {
                out.push_str(&rest[..open + 1]);
                rest = &rest[open + 1..];
            }
        }
    }
    out.push_str(rest);
    out
}

// spec: context-kit/SPEC.md §Index-first reading — the first sentence is the first non-blank line
// below the heading that is outside a fence and is neither a heading nor a `---` rule; emphasis and
// code marks are stripped and the text is cut at the first `.`, `!` or `?`, else at 120 characters.
fn first_sentence(lines: &[&str], from: usize) -> String {
    let mut in_fence = false;
    for line in lines.iter().skip(from) {
        let l = line.trim_matches([' ', '\t']);
        if l.starts_with("```") || l.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if heading_level(l) > 0 || l.starts_with("---") {
            break;
        }
        if l.is_empty() {
            continue;
        }
        let text: String = strip_links(l)
            .chars()
            .filter(|c| !matches!(c, '*' | '_' | '`'))
            .collect();
        let cut = match text.find(['.', '!', '?']) {
            Some(i) => text[..i + 1].to_string(),
            None => text.chars().take(120).collect(),
        };
        return cut.trim_matches([' ', '\t']).to_string();
    }
    String::new()
}

// spec: context-kit/SPEC.md §Index-first reading — one indented row per heading, two spaces per
// level below the first. The heading scan is fence-blind where the sentence search above is not,
// which that section rules as contract rather than oversight.
fn index_one(text: &str) -> String {
    let lines = section::split_lines(text);
    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        let level = heading_level(line);
        if level == 0 {
            continue;
        }
        let heading = line[level..].trim_matches([' ', '\t']);
        let first = first_sentence(&lines, i + 1);
        let indent = "  ".repeat(level - 1);
        let hashes = "#".repeat(level);
        out.push_str(&format!("{}{} {}:{}", indent, hashes, heading, i + 1));
        if !first.is_empty() {
            out.push_str(&format!("  \u{2014} {}", first));
        }
        out.push('\n');
    }
    out
}

// spec: context-kit/SPEC.md §Index-first reading — the per-file block: the path and its `wc -l`
// count, the heading rows, then a blank line — after the last block too, which the shell form's
// unconditional trailing `echo ""` is what makes true.
pub fn emit(args: &[String]) -> Result<String, String> {
    let paths = targets(args)?;
    let root = walk::toplevel_opt()?;
    let mut out = String::new();
    for file in corpus(&paths, &["*.md"])? {
        // spec: context-kit/SPEC.md §Index-first reading — a file the walk reached but the reader
        // cannot open contributes nothing, the shell form's `awk … 2>/dev/null || true`: the index
        // is advisory, so an unreadable entry is a gap in it rather than a refusal of the run
        let Ok(text) = read_text(&file) else { continue };
        let rows = index_one(&text);
        if rows.is_empty() {
            continue;
        }
        out.push_str(&format!(
            "{}  ({}L)\n",
            relative(&root, &file),
            newlines(&text)
        ));
        out.push_str(&rows);
        out.push('\n');
    }
    if out.is_empty() {
        out.push_str(&format!(
            "No Markdown files found in {}\n",
            paths.join(" ")
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: context-kit/SPEC.md §Index-first reading — the first-sentence rule's four clauses on
    // one input: a link reduced to its text, emphasis stripped, the cut at the first terminator,
    // and the 120-character fallback where the line carries none.
    #[test]
    fn the_first_sentence_reduces_links_strips_marks_and_cuts_at_a_terminator() {
        let lines = vec![
            "# H",
            "",
            "Intro with a [link](https://example.org/p) and *more*. Second is dropped.",
        ];
        assert_eq!(first_sentence(&lines, 1), "Intro with a link and more.");
        let long = "x".repeat(200);
        let lines = vec!["# H", long.as_str()];
        assert_eq!(first_sentence(&lines, 1).chars().count(), 120);
        // spec: context-kit/SPEC.md §Index-first reading — a bracket that opens no well-formed
        // link is literal, so prose carrying `[EDIT ME]` survives the reduction intact
        assert_eq!(strip_links("an [EDIT ME] gap"), "an [EDIT ME] gap");
    }

    // spec: context-kit/SPEC.md §Index-first reading — the sentence search skips fenced content
    // and stops at the next heading or `---` rule, so a heading with nothing quotable below it
    // renders bare rather than borrowing the next section's prose
    #[test]
    fn the_sentence_search_skips_fences_and_stops_at_the_next_heading() {
        let lines = vec!["## A", "```", "fenced prose", "```", "after the fence."];
        assert_eq!(first_sentence(&lines, 1), "after the fence.");
        let lines = vec!["## A", "", "## B", "body."];
        assert_eq!(first_sentence(&lines, 1), "");
        let lines = vec!["## A", "", "---", "body."];
        assert_eq!(first_sentence(&lines, 1), "");
    }

    // spec: context-kit/SPEC.md §Index-first reading — the fence-blind heading scan, and its
    // stated bounds: six hashes, and a blank as the separator.
    #[test]
    fn a_heading_inside_a_fence_is_a_row_and_the_hash_run_is_bounded_at_six() {
        let got = index_one("# Top\n\n```\n## Fenced\n```\n");
        assert!(got.contains("# Top:1"));
        assert!(
            got.contains("  ## Fenced:4"),
            "the fence-blind heading scan moved: {}",
            got
        );
        assert_eq!(heading_level("###### x"), 6);
        assert_eq!(heading_level("####### x"), 0);
        assert_eq!(heading_level("##x"), 0);
    }

    // spec: context-kit/SPEC.md §Index-first reading — `wc -l` counts newlines, so an unterminated
    // final line is not counted; the block header would otherwise read one high for every file
    // whose last byte is not a newline
    #[test]
    fn the_line_count_is_newlines_and_not_lines() {
        assert_eq!(newlines("a\nb\n"), 2);
        assert_eq!(newlines("a\nb"), 1);
        assert_eq!(newlines(""), 0);
    }
}
