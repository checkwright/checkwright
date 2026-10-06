// spec: canon-kit/SPEC.md §check-docs-page-repeat — no declared page states a relative link target,
// or a sentence of the configured length, twice
use super::md_refs::line_link_targets;
use crate::spec;
use crate::walk;
use std::collections::HashMap;
use std::path::Path;

const PAGES_KNOB: &str = "CANON_KIT_PAGE_REPEAT_PAGES";
const MIN_WORDS_KNOB: &str = "CANON_KIT_PAGE_REPEAT_MIN_WORDS";
const VALVE: &str = "page-repeat-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-docs-page-repeat: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let min_words = parse_min(&spec::knob_pub(MIN_WORDS_KNOB)?)?;
    let globs = spec::knob_array_pub(PAGES_KNOB)?;
    let mut pages: Vec<String> = if globs.is_empty() {
        Vec::new()
    } else {
        walk::glob_corpus(Path::new(root), &globs)?
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| spec::strip_dot_slash(&p.display().to_string()))
            .collect()
    };
    pages.sort();
    pages.dedup();

    let mut findings: Vec<Finding> = Vec::new();
    let (mut nlinks, mut nsentences) = (0usize, 0usize);
    for page in &pages {
        let scan = scan(&read_page(page)?, min_words);
        nlinks += scan.links;
        nsentences += scan.sentences;
        for f in scan.findings {
            findings.push(Finding {
                page: page.clone(),
                ..f
            });
        }
    }

    if !findings.is_empty() {
        println!(
            "check-docs-page-repeat: {} repeat(s) on the declared pages:",
            findings.len()
        );
        for f in &findings {
            let what = match f.arm {
                Arm::Link => format!("link target `{}`", f.subject),
                Arm::Sentence => format!("sentence \"{}\"", f.subject),
            };
            let valve = if f.empty_valve {
                " (its page-repeat-exempt valve carries no reason)"
            } else {
                ""
            };
            println!(
                "  {}:{}: {} first stated at line {}{}",
                f.page, f.line, what, f.first, valve
            );
        }
        if findings.iter().any(|f| matches!(f.arm, Arm::Link)) {
            println!("  help: a repeated link — link the first mention and make the later one plain text or an in-page anchor.");
        }
        if findings.iter().any(|f| matches!(f.arm, Arm::Sentence)) {
            println!("  help: a repeated sentence — state it once and point back to it.");
        }
        println!(
            "  help: a deliberate keep takes '<!-- {} <reason> -->' on the later line or the one above.",
            VALVE
        );
        return Ok(1);
    }
    println!(
        "DOCS-PAGE-REPEAT: clean ({} page(s), {} link(s), {} sentence(s) of {} word(s) or more, none stated twice on one page)",
        pages.len(),
        nlinks,
        nsentences,
        min_words
    );
    Ok(0)
}

fn parse_min(raw: &str) -> Result<usize, String> {
    match raw.trim().parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("{} is '{}', not a positive integer", MIN_WORDS_KNOB, raw)),
    }
}

fn read_page(page: &str) -> Result<String, String> {
    spec::read_text(Path::new(page)).map_err(|e| format!("cannot read declared page {}: {}", page, e))
}

#[derive(Clone, Copy)]
enum Arm {
    Link,
    Sentence,
}

struct Finding {
    page: String,
    line: usize,
    first: usize,
    arm: Arm,
    subject: String,
    empty_valve: bool,
}

struct Scan {
    findings: Vec<Finding>,
    links: usize,
    sentences: usize,
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — the valve's reason, on the line or the one above
fn valve_reason(line: &str) -> Option<String> {
    let at = line.find(VALVE)?;
    let rest = &line[at + VALVE.len()..];
    let rest = rest.split("-->").next().unwrap_or(rest);
    Some(rest.trim().to_string())
}

fn scan(text: &str, min_words: usize) -> Scan {
    let raw: Vec<&str> = text.lines().collect();
    let mut out = Scan {
        findings: Vec::new(),
        links: 0,
        sentences: 0,
    };
    let mut seen_links: HashMap<String, usize> = HashMap::new();
    let mut seen_sentences: HashMap<String, usize> = HashMap::new();
    for (idx, prose) in prose_lines(&raw) {
        let ln = idx + 1;
        let valve = [Some(idx), idx.checked_sub(1)]
            .iter()
            .flatten()
            .find_map(|&i| valve_reason(raw[i]));
        let exempt = valve.as_deref().is_some_and(|r| !r.is_empty());
        let empty_valve = valve.as_deref() == Some("");
        let mut record = |arm: Arm, key: String, subject: String, seen: &mut HashMap<String, usize>| {
            match seen.get(&key) {
                Some(&first) if !exempt => out.findings.push(Finding {
                    page: String::new(),
                    line: ln,
                    first,
                    arm,
                    subject,
                    empty_valve,
                }),
                Some(_) => {}
                None => {
                    seen.insert(key, ln);
                }
            }
        };

        let mut targets: Vec<String> = Vec::new();
        line_link_targets(&prose, &mut targets);
        for t in targets {
            let Some(t) = relative_target(&t) else {
                continue;
            };
            out.links += 1;
            record(Arm::Link, t.clone(), t, &mut seen_links);
        }

        let plain = unlinked(&prose);
        let body = strip_marker(&plain);
        for (s, e) in spec::sentence_spans(body) {
            let folded = fold(&body[s..e]);
            if spec::word_count(&folded) < min_words {
                continue;
            }
            out.sentences += 1;
            record(Arm::Sentence, folded.clone(), folded, &mut seen_sentences);
        }
    }
    out
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — prose only: the front-matter block, fenced
// blocks, HTML comments and table rows are skipped, the rest of a commented line kept
fn prose_lines(raw: &[&str]) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut front = raw.first().map(|l| l.trim_end() == "---").unwrap_or(false);
    let mut fence = spec::Fence::default();
    let mut comment = false;
    for (i, line) in raw.iter().enumerate() {
        if front {
            if i > 0 && line.trim_end() == "---" {
                front = false;
            }
            continue;
        }
        if !comment && fence.delimits(line) {
            continue;
        }
        if fence.is_open() {
            continue;
        }
        let mut kept = String::new();
        let mut rest: &str = line;
        loop {
            if comment {
                match rest.find("-->") {
                    Some(e) => {
                        rest = &rest[e + 3..];
                        comment = false;
                    }
                    None => break,
                }
            } else {
                match rest.find("<!--") {
                    Some(s) => {
                        kept.push_str(&rest[..s]);
                        rest = &rest[s + 4..];
                        comment = true;
                    }
                    None => {
                        kept.push_str(rest);
                        break;
                    }
                }
            }
        }
        if kept.trim().is_empty() || spec::is_table_row(&kept) {
            continue;
        }
        out.push((i, kept));
    }
    out
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — a scheme, a `mailto:` target or a pure
// `#anchor` is out of scope, and a leading `./` is dropped before the compare
fn relative_target(t: &str) -> Option<String> {
    if t.starts_with('#') {
        return None;
    }
    let b = t.as_bytes();
    if let Some(colon) = t.find(':') {
        let scheme = &b[..colon];
        if !scheme.is_empty()
            && scheme[0].is_ascii_alphabetic()
            && scheme
                .iter()
                .all(|&c| c.is_ascii_alphanumeric() || matches!(c, b'+' | b'.' | b'-'))
        {
            return None;
        }
    }
    Some(t.strip_prefix("./").unwrap_or(t).to_string())
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — a link reduces to its text
fn unlinked(line: &str) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        let Some(mid) = after.find("](") else {
            break;
        };
        let Some(close) = after[mid + 2..].find(')') else {
            break;
        };
        out.push_str(&rest[..open]);
        out.push_str(&after[..mid]);
        rest = &after[mid + 2 + close + 1..];
    }
    out.push_str(rest);
    out
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — a heading, list or quote marker is dropped
fn strip_marker(line: &str) -> &str {
    let t = line.trim_start();
    if let Some(r) = t.strip_prefix("> ") {
        return strip_marker(r);
    }
    let level = spec::prose_heading_level(t);
    if level > 0 {
        return t[level..].trim_start();
    }
    if spec::is_list_item(t) {
        let at = t.find([' ', '\t']).unwrap_or(0);
        return t[at..].trim_start();
    }
    t
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — case and runs of whitespace are folded
fn fold(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_splits_into_sentences_and_each_is_folded() {
        let body = strip_marker("- One Two  three.  Four five? Six");
        let got: Vec<String> = spec::sentence_spans(body)
            .into_iter()
            .map(|(s, e)| fold(&body[s..e]))
            .collect();
        assert_eq!(got, vec!["one two three", "four five", "six"]);
    }

    #[test]
    fn a_link_reduces_to_its_text_and_a_quote_marker_drops() {
        assert_eq!(unlinked("see [the page](a.md#b) and [x](y)"), "see the page and x");
        assert_eq!(strip_marker("> > quoted text"), "quoted text");
        assert_eq!(strip_marker("## A heading"), "A heading");
    }

    #[test]
    fn only_a_relative_target_is_in_scope() {
        assert_eq!(relative_target("./a.md#b"), Some("a.md#b".to_string()));
        assert_eq!(relative_target("https://x.dev/a"), None);
        assert_eq!(relative_target("mailto:a@b.c"), None);
        assert_eq!(relative_target("#here"), None);
    }

    #[test]
    fn front_matter_fences_comments_and_tables_are_never_prose() {
        let raw = [
            "---",
            "title: [a](b.md)",
            "---",
            "kept <!-- dropped",
            "still dropped --> tail",
            "```",
            "[a](b.md)",
            "```",
            "| [a](b.md) |",
        ];
        let got: Vec<String> = prose_lines(&raw).into_iter().map(|(_, l)| l).collect();
        assert_eq!(got, vec!["kept ", " tail"]);
    }

    #[test]
    fn a_repeat_reds_unless_a_reasoned_valve_sits_on_it_or_above() {
        let page = "[a](b.md) one two three four five six seven eight.\n\n[a](b.md#x)\n\n<!-- page-repeat-exempt: kept -->\n[a](b.md)\n\n[a](b.md) One  two three four five six seven EIGHT.\n\n<!-- page-repeat-exempt: -->\n[a](b.md)\n";
        let s = scan(page, 8);
        let got: Vec<(usize, usize, bool)> = s.findings.iter().map(|f| (f.line, f.first, f.empty_valve)).collect();
        assert_eq!(got, vec![(8, 1, false), (8, 1, false), (11, 1, true)]);
        assert_eq!(s.links, 5);
    }

    // spec: canon-kit/SPEC.md §check-docs-page-repeat — both exit-2 paths, which a `bad/` fixture
    // held to exit 1 cannot express
    #[test]
    fn an_unreadable_page_and_a_bad_threshold_are_refusals() {
        assert!(read_page("no/such/page-repeat.md")
            .unwrap_err()
            .contains("cannot read declared page"));
        for bad in ["0", "-1", "eight", ""] {
            assert!(parse_min(bad).unwrap_err().contains("not a positive integer"));
        }
        assert_eq!(parse_min(" 8 "), Ok(8));
    }
}
