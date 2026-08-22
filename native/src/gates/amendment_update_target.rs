// spec: canon-kit/SPEC.md §check-amendment-update-target — every entry under an amendment's
// `## Existing sections updated` cites at least one delta, and every cited delta is defined
// under `## What changes` in the same amendment
use crate::spec;
use std::path::Path;

const EXEMPT: &str = "update-target-exempt:";

// spec: canon-kit/SPEC.md §check-amendment-update-target — the two heading names are kit
// constants, not config: they are canon-kit's own template's headings, and a consumer editing
// them has edited the artifact rather than configured it
const WHAT_CHANGES: &str = "## What changes";
const UPDATED: &str = "## Existing sections updated";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-amendment-update-target: {}", e);
            2
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Sec {
    What,
    Updated,
    Other,
}

// spec: canon-kit/SPEC.md §check-amendment-update-target — arm A's grammar: `### (<N>) <title>`,
// `<N>` a positive decimal integer written without a leading zero, `<title>` non-empty. A
// heading under `## What changes` that this returns `None` for is the arm-A violation.
fn delta_number(heading: &str) -> Option<usize> {
    let rest = heading.strip_prefix("### ")?.trim_start();
    let rest = rest.strip_prefix('(')?;
    let close = rest.find(')')?;
    let num = &rest[..close];
    if num.is_empty() || !num.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if num.len() > 1 && num.starts_with('0') {
        return None;
    }
    let n: usize = num.parse().ok()?;
    if n == 0 {
        return None;
    }
    if rest[close + 1..].trim().is_empty() {
        return None;
    }
    Some(n)
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn bounded_at(b: &[u8], at: usize, word: &[u8]) -> bool {
    if !b[at..].starts_with(word) {
        return false;
    }
    if at > 0 && is_word_byte(b[at - 1]) {
        return false;
    }
    let end = at + word.len();
    end == b.len() || !is_word_byte(b[end])
}

fn skip_sp(b: &[u8], mut i: usize) -> usize {
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
    }
    i
}

// spec: canon-kit/SPEC.md §check-amendment-update-target — a citation's integer list: the first
// integer, then any further integer reached through a comma, the word `and`, or both; a trailing
// possessive closes an integer without closing the list
fn integer_list(b: &[u8], start: usize, out: &mut Vec<(usize, usize)>) {
    let mut i = start;
    loop {
        let s = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == s {
            return;
        }
        match std::str::from_utf8(&b[s..i]).ok().and_then(|d| d.parse().ok()) {
            Some(n) => out.push((n, s)),
            None => return,
        }
        if i < b.len() && b[i] == b'\'' {
            i += 1;
            if i < b.len() && b[i] == b's' {
                i += 1;
            }
        }
        let mut j = skip_sp(b, i);
        let mut sep = false;
        if j < b.len() && b[j] == b',' {
            sep = true;
            j = skip_sp(b, j + 1);
        }
        if bounded_at(b, j, b"and") {
            sep = true;
            j = skip_sp(b, j + 3);
        }
        if !sep || j >= b.len() || !b[j].is_ascii_digit() {
            return;
        }
        i = j;
    }
}

// spec: canon-kit/SPEC.md §check-amendment-update-target — the citation grammar over one
// whitespace-normalized entry: `delta`/`deltas` plus an integer list, case-insensitively, and
// the literal `all deltas` standing for every delta the amendment defines
fn citations(text: &str) -> (Vec<(usize, usize)>, Option<usize>) {
    let lower = text.to_ascii_lowercase();
    let b = lower.as_bytes();
    let mut nums: Vec<(usize, usize)> = Vec::new();
    let mut all: Option<usize> = None;
    let mut i = 0usize;
    while i < b.len() {
        if all.is_none() && bounded_at(b, i, b"all deltas") {
            all = Some(i);
        }
        let word: &[u8] = if bounded_at(b, i, b"deltas") {
            b"deltas"
        } else if bounded_at(b, i, b"delta") {
            b"delta"
        } else {
            i += 1;
            continue;
        };
        let after = skip_sp(b, i + word.len());
        integer_list(b, after, &mut nums);
        i += word.len();
    }
    (nums, all)
}

struct Entry {
    line: usize,
    exempt: bool,
    flat: spec::FlatPara,
}

struct Scan {
    deltas: Vec<(usize, usize)>,
    malformed: Vec<(usize, String)>,
    entries: Vec<Entry>,
    has_what: bool,
    has_updated: bool,
}

fn scan(text: &str) -> Scan {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Scan {
        deltas: Vec::new(),
        malformed: Vec::new(),
        entries: Vec::new(),
        has_what: false,
        has_updated: false,
    };
    let mut fence = false;
    let mut sec = Sec::Other;
    let mut i = 0usize;
    while i < lines.len() {
        let raw = lines[i];
        // spec: canon-kit/SPEC.md §check-amendment-update-target — a fenced block is skipped
        // whole: the template sanctions an embedded wire-contract delta, whose content must not
        // read as a heading or an update target
        if spec::is_fence_line(raw) {
            fence = !fence;
            i += 1;
            continue;
        }
        if fence {
            i += 1;
            continue;
        }
        if raw.starts_with("## ") {
            let name = raw.trim_end();
            sec = if name == WHAT_CHANGES {
                out.has_what = true;
                Sec::What
            } else if name == UPDATED {
                out.has_updated = true;
                Sec::Updated
            } else {
                Sec::Other
            };
            i += 1;
            continue;
        }
        match sec {
            Sec::What => {
                if raw.starts_with("### ") {
                    match delta_number(raw) {
                        Some(n) => out.deltas.push((n, i + 1)),
                        None => out.malformed.push((i + 1, raw.trim_end().to_string())),
                    }
                }
                i += 1;
            }
            Sec::Updated => {
                if !raw.starts_with("- ") {
                    i += 1;
                    continue;
                }
                // spec: canon-kit/SPEC.md §lib/spec.sh — the shared exempt window, the line or
                // the one above
                let exempt = raw.contains(EXEMPT) || (i > 0 && lines[i - 1].contains(EXEMPT));
                let mut para = spec::Para::default();
                para.add(i + 1, raw);
                let mut j = i + 1;
                // spec: canon-kit/SPEC.md §check-amendment-update-target — the entry window is
                // the bullet plus its indented continuation, so a citation that wrapped is still
                // one subject; it ends at the first non-blank line back at column 0
                while j < lines.len() {
                    let l = lines[j];
                    if spec::is_blank(l) {
                        j += 1;
                        continue;
                    }
                    if spec::is_fence_line(l) || !(l.starts_with(' ') || l.starts_with('\t')) {
                        break;
                    }
                    para.add(j + 1, l);
                    j += 1;
                }
                out.entries.push(Entry {
                    line: i + 1,
                    exempt,
                    flat: spec::flatten_para(&para),
                });
                i = j;
            }
            Sec::Other => i += 1,
        }
    }
    out
}

fn lead(text: &str) -> String {
    let mut s: String = text.chars().take(72).collect();
    if text.chars().count() > 72 {
        s.push('…');
    }
    s
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    // spec: canon-kit/SPEC.md §check-amendment-update-target — the strict finder, where
    // §check-amendment-queue takes the best-effort one: an empty amendment set hides every
    // violation here, because this gate has no second surface to contradict it
    let files: Vec<String> = spec::amendments_strict(root)?
        .into_iter()
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();

    let mut bad_grammar: Vec<String> = Vec::new();
    let mut bad_order: Vec<String> = Vec::new();
    let mut uncited: Vec<String> = Vec::new();
    let mut dangling: Vec<String> = Vec::new();
    let mut deltas = 0usize;
    let mut targets = 0usize;

    for f in &files {
        let text = spec::read_text(Path::new(f))?;
        let s = scan(&text);
        if s.has_updated && !s.has_what {
            return Err(format!(
                "{}: carries '{}' but no '{}' — no delta can own an update target here",
                f, UPDATED, WHAT_CHANGES
            ));
        }
        for (line, heading) in &s.malformed {
            bad_grammar.push(format!("  {}:{}: {}", f, line, heading));
        }
        // spec: canon-kit/SPEC.md §check-amendment-update-target — 1..n unique and ascending is
        // one assertion, and only the first breach is reported: an inserted delta shifts every
        // number after it, so reporting each would bury the one edit that caused them
        for (k, (n, line)) in s.deltas.iter().enumerate() {
            if *n != k + 1 {
                bad_order.push(format!(
                    "  {}:{}: delta ({}) stands where ({}) is owed",
                    f,
                    line,
                    n,
                    k + 1
                ));
                break;
            }
        }
        deltas += s.deltas.len();

        let defined: Vec<usize> = s.deltas.iter().map(|(n, _)| *n).collect();
        for e in &s.entries {
            if e.exempt {
                continue;
            }
            targets += 1;
            let (nums, all) = citations(&e.flat.text);
            if nums.is_empty() && all.is_none() {
                uncited.push(format!("  {}:{}: {}", f, e.line, lead(&e.flat.text)));
                continue;
            }
            if let Some(at) = all {
                if defined.is_empty() {
                    dangling.push(format!(
                        "  {}:{}: 'all deltas', but the amendment defines none",
                        f,
                        e.flat.line_at(at)
                    ));
                }
            }
            let mut seen: Vec<usize> = Vec::new();
            for (n, at) in &nums {
                if defined.contains(n) || seen.contains(n) {
                    continue;
                }
                seen.push(*n);
                dangling.push(format!(
                    "  {}:{}: cites delta {}, which no '### ({})' heading defines",
                    f,
                    e.flat.line_at(*at),
                    n,
                    n
                ));
            }
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
            errors.push_str(it);
            errors.push('\n');
        }
    };
    block(
        "delta headings outside the '### (<N>) <title>' form (arm A — a grammar B and C cannot read):",
        &bad_grammar,
    );
    block(
        "delta numbers that are not 1..n unique and ascending (arm A — a gap or a repeat means a delta moved and its citations did not):",
        &bad_order,
    );
    block(
        "update targets citing no delta (arm B — an orphan a build batch adopts on its own authority):",
        &uncited,
    );
    block(
        "citations naming a delta the amendment does not define (arm C — renumbered out from under the target):",
        &dangling,
    );

    if !errors.is_empty() {
        println!("check-amendment-update-target: an update target no delta owns, or a citation no delta defines:");
        println!();
        print!("{}", errors);
        println!("  help: give each '{}' entry a '(delta <N>)' citation naming the delta under '{}' that owns it — 'deltas 2, 3' and 'all deltas' are citations too; number deltas '### (<N>) <title>', 1..n with no gap or repeat, moving every citation with a renumbered delta. For a target deliberately owned by no delta, tag '<!-- update-target-exempt: <reason> -->' on the bullet's first line or the one above (a reason is mandatory).", UPDATED, WHAT_CHANGES);
        return Ok(1);
    }
    println!(
        "AMENDMENT-UPDATE-TARGET: clean ({} amendment(s), {} delta(s) defined; {} update target(s), each citing a delta the same amendment defines)",
        files.len(),
        deltas,
        targets
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_delta_heading_grammar_is_a_parenthesised_positive_integer_and_a_title() {
        assert_eq!(delta_number("### (1) The grammar"), Some(1));
        assert_eq!(delta_number("### (12) x"), Some(12));
        assert_eq!(delta_number("### (1) `code` title"), Some(1));
        assert_eq!(delta_number("### A1. bold-ish"), None);
        assert_eq!(delta_number("### (i) roman"), None);
        assert_eq!(delta_number("### (0) zero is not positive"), None);
        assert_eq!(delta_number("### (01) leading zero"), None);
        assert_eq!(delta_number("### (2)"), None);
        assert_eq!(delta_number("### (3) "), None);
        assert_eq!(delta_number("#### (1) wrong level"), None);
    }

    // spec: canon-kit/SPEC.md §check-amendment-update-target — the citation grammar, including
    // the plural, the comma/`and` list, the possessive and the whole-amendment literal
    #[test]
    fn a_citation_is_the_word_plus_an_integer_list() {
        let n = |s: &str| -> Vec<usize> { citations(s).0.into_iter().map(|(n, _)| n).collect() };
        assert_eq!(n("… (delta 1)."), vec![1]);
        assert_eq!(n("… (deltas 2, 3)."), vec![2, 3]);
        assert_eq!(n("… (deltas 1, 2 and 3)."), vec![1, 2, 3]);
        assert_eq!(n("… (deltas 4 and 5)."), vec![4, 5]);
        assert_eq!(n("per delta 5's table (delta 5)."), vec![5, 5]);
        assert_eq!(n("Delta 7 capitalised."), vec![7]);
        assert!(n("no delta claims this target.").is_empty());
        assert!(n("the deltas above.").is_empty());
        assert!(citations("… (all deltas).").1.is_some());
        assert!(citations("… (delta 1).").1.is_none());
    }

    // spec: canon-kit/SPEC.md §check-amendment-update-target — the entry window is the bullet
    // plus its indented continuation, so a wrapped citation is still one subject, and a fenced
    // block reads as neither a heading nor a target
    #[test]
    fn an_entry_spans_its_indented_continuation_and_a_fence_is_skipped_whole() {
        let text = "## What changes\n\n### (1) one\n\n```\n### (9) fenced\n- **fenced target**\n```\n\n## Existing sections updated\n\n- **a** — wrapped across\n  a newline (delta\n  1).\n- **b** — uncited.\n\n## Definition of Done\n";
        let s = scan(text);
        assert_eq!(s.deltas.len(), 1);
        assert!(s.malformed.is_empty());
        assert_eq!(s.entries.len(), 2);
        assert_eq!(citations(&s.entries[0].flat.text).0.len(), 1);
        assert!(citations(&s.entries[1].flat.text).0.is_empty());
        assert!(s.has_what && s.has_updated);
    }

    #[test]
    fn the_exempt_window_is_the_bullet_line_or_the_one_above() {
        let text = "## What changes\n\n### (1) one\n\n## Existing sections updated\n\n<!-- update-target-exempt: r -->\n- **a** — uncited.\n- **b** — uncited.\n";
        let s = scan(text);
        assert_eq!(s.entries.len(), 2);
        assert!(s.entries[0].exempt);
        assert!(!s.entries[1].exempt);
    }
}
