// spec: canon-kit/SPEC.md §check-docs-page-repeat — no declared page states a relative link target,
// or a sentence of the configured length, twice, nor restates the site chrome around it
use super::md_refs::line_link_targets;
use crate::spec;
use crate::walk;
use std::collections::HashMap;
use std::path::Path;

const PAGES_KNOB: &str = "CANON_KIT_PAGE_REPEAT_PAGES";
const MIN_WORDS_KNOB: &str = "CANON_KIT_PAGE_REPEAT_MIN_WORDS";
const CHROME_KNOB: &str = "CANON_KIT_PAGE_REPEAT_CHROME";
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
    let pages = expand(root, &spec::knob_array_pub(PAGES_KNOB)?)?;
    let mut chrome = Chrome::default();
    for file in expand(root, &spec::knob_array_pub(CHROME_KNOB)?)? {
        let text = spec::read_text(Path::new(&file))
            .map_err(|e| format!("cannot read chrome file {}: {}", file, e))?;
        chrome.add(&file, &text, min_words);
    }

    let mut findings: Vec<Finding> = Vec::new();
    let (mut nlinks, mut nsentences) = (0usize, 0usize);
    for page in &pages {
        let scan = scan(&read_page(page)?, min_words, &chrome);
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
                Arm::Link | Arm::ChromeLink => format!("link target `{}`", f.subject),
                Arm::Sentence | Arm::ChromeSentence => format!("sentence \"{}\"", f.subject),
            };
            let valve = if f.empty_valve {
                " (its page-repeat-exempt valve carries no reason)"
            } else {
                ""
            };
            println!(
                "  {}:{}: {} first stated at {}{}",
                f.page, f.line, what, f.first, valve
            );
        }
        if findings.iter().any(|f| matches!(f.arm, Arm::Link)) {
            println!("  help: a repeated link — link the first mention and make the later one plain text or an in-page anchor.");
        }
        if findings.iter().any(|f| matches!(f.arm, Arm::Sentence)) {
            println!("  help: a repeated sentence — state it once and point back to it.");
        }
        if findings
            .iter()
            .any(|f| matches!(f.arm, Arm::ChromeLink | Arm::ChromeSentence))
        {
            println!("  help: a chrome restatement — the site chrome already states it on every page: drop the page's copy.");
        }
        println!(
            "  help: a deliberate keep takes '<!-- {} <reason> -->' on the later line or the one above.",
            VALVE
        );
        return Ok(1);
    }
    println!(
        "DOCS-PAGE-REPEAT: clean ({} page(s), {} link(s), {} sentence(s) of {} word(s) or more, none stated twice on one page; {} chrome file(s), none restated on a page)",
        pages.len(),
        nlinks,
        nsentences,
        min_words,
        chrome.files
    );
    Ok(0)
}

fn expand(root: &str, globs: &[String]) -> Result<Vec<String>, String> {
    if globs.is_empty() {
        return Ok(Vec::new());
    }
    let mut out: Vec<String> = walk::glob_corpus(Path::new(root), globs)?
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    out.sort();
    out.dedup();
    Ok(out)
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — arm C's chrome: each literal target and each
// long sentence, keyed to the `<file>:<line>` of its first occurrence across the chrome files
#[derive(Default)]
struct Chrome {
    files: usize,
    targets: HashMap<String, String>,
    sentences: HashMap<String, String>,
}

impl Chrome {
    fn add(&mut self, file: &str, text: &str, min_words: usize) {
        self.files += 1;
        let mut markup = blank_spans(text, "<!--", "-->");
        for el in ["script", "style"] {
            markup = blank_spans(&markup, &format!("<{}", el), &format!("</{}>", el));
        }
        let templated = blank_spans(&blank_spans(&markup, "{{", "}}"), "{%", "%}");
        for (idx, (line, raw)) in templated.lines().zip(markup.lines()).enumerate() {
            for t in literal_hrefs(line, raw) {
                if let Some(t) = chrome_target(&t) {
                    self.targets
                        .entry(t)
                        .or_insert_with(|| format!("{}:{}", file, idx + 1));
                }
            }
        }
        let bare = strip_tags(&templated);
        for (idx, line) in bare.lines().enumerate() {
            for folded in long_sentences(line, min_words) {
                self.sentences
                    .entry(folded)
                    .or_insert_with(|| format!("{}:{}", file, idx + 1));
            }
        }
    }
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — a dropped span keeps its newlines, so every
// later line keeps its number; the open literal is matched case-insensitively, and an unclosed
// span runs to the end
fn blank_spans(text: &str, open: &str, close: &str) -> String {
    let low = text.to_ascii_lowercase();
    let mut b = text.as_bytes().to_vec();
    let mut from = 0usize;
    while let Some(off) = low[from..].find(open) {
        let s = from + off;
        let e = low[s + open.len()..]
            .find(close)
            .map_or(b.len(), |o| s + open.len() + o + close.len());
        for c in &mut b[s..e] {
            if *c != b'\n' {
                *c = b' ';
            }
        }
        from = e;
    }
    String::from_utf8_lossy(&b).into_owned()
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — a tag drops whole, its newlines kept; a `<`
// opening no tag name is text
fn strip_tags(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        let opens = c == '<' && chars.peek().is_some_and(|n| n.is_ascii_alphabetic() || *n == '/');
        if !opens {
            out.push(c);
            continue;
        }
        for t in chars.by_ref() {
            if t == '>' {
                break;
            }
            if t == '\n' {
                out.push('\n');
            }
        }
    }
    out
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — a literal `href` value: quoted, and holding
// no template expression; `line` is `raw` with template expressions blanked byte for byte
fn literal_hrefs(line: &str, raw: &str) -> Vec<String> {
    let low = line.to_ascii_lowercase();
    let b = line.as_bytes();
    let skip_ws = |mut i: usize| {
        while b.get(i).is_some_and(u8::is_ascii_whitespace) {
            i += 1;
        }
        i
    };
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(off) = low[from..].find("href") {
        let at = from + off;
        from = at + 4;
        if at > 0 && !b[at - 1].is_ascii_whitespace() {
            continue;
        }
        let eq = skip_ws(at + 4);
        if b.get(eq) != Some(&b'=') {
            continue;
        }
        let qi = skip_ws(eq + 1);
        let Some(&q) = b.get(qi).filter(|&&q| q == b'"' || q == b'\'') else {
            continue;
        };
        let start = qi + 1;
        let Some(len) = line[start..].find(q as char) else {
            continue;
        };
        let v = raw[start..start + len].trim();
        from = start + len + 1;
        if !v.is_empty() && !v.contains("{{") && !v.contains("{%") {
            out.push(v.to_string());
        }
    }
    out
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — arm C compares a scheme target too, chrome
// links being commonly absolute; `mailto:` and a pure `#anchor` stay out, as in arm A
fn chrome_target(t: &str) -> Option<String> {
    if t.starts_with('#') || t.to_ascii_lowercase().starts_with("mailto:") {
        return None;
    }
    Some(t.strip_prefix("./").unwrap_or(t).to_string())
}

fn long_sentences(body: &str, min_words: usize) -> Vec<String> {
    spec::sentence_spans(body)
        .into_iter()
        .map(|(s, e)| fold(&body[s..e]))
        .filter(|f| spec::word_count(f) >= min_words)
        .collect()
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
    ChromeLink,
    ChromeSentence,
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — `first` is the first occurrence: `line <n>` on
// the page for arms A and B, the chrome's `<file>:<line>` for arm C
struct Finding {
    page: String,
    line: usize,
    first: String,
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

fn scan(text: &str, min_words: usize, chrome: &Chrome) -> Scan {
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
        let mut found: Vec<(Arm, String, String)> = Vec::new();

        let mut targets: Vec<String> = Vec::new();
        line_link_targets(&prose, &mut targets);
        for t in targets {
            if let Some(at) = chrome_target(&t).and_then(|c| chrome.targets.get(&c)) {
                found.push((Arm::ChromeLink, at.clone(), t.clone()));
            }
            let Some(t) = relative_target(&t) else {
                continue;
            };
            out.links += 1;
            if let Some(first) = seen_before(&mut seen_links, &t, ln) {
                found.push((Arm::Link, format!("line {}", first), t));
            }
        }

        let plain = unlinked(&prose);
        for folded in long_sentences(strip_marker(&plain), min_words) {
            out.sentences += 1;
            if let Some(at) = chrome.sentences.get(&folded) {
                found.push((Arm::ChromeSentence, at.clone(), folded.clone()));
            }
            if let Some(first) = seen_before(&mut seen_sentences, &folded, ln) {
                found.push((Arm::Sentence, format!("line {}", first), folded));
            }
        }

        if !exempt {
            out.findings
                .extend(found.into_iter().map(|(arm, first, subject)| Finding {
                    page: String::new(),
                    line: ln,
                    first,
                    arm,
                    subject,
                    empty_valve,
                }));
        }
    }
    out
}

fn seen_before(seen: &mut HashMap<String, usize>, key: &str, ln: usize) -> Option<usize> {
    match seen.get(key) {
        Some(&first) => Some(first),
        None => {
            seen.insert(key.to_string(), ln);
            None
        }
    }
}

// spec: canon-kit/SPEC.md §check-docs-page-repeat — prose only: the front-matter block, fenced
// blocks, HTML comments and table rows are skipped, the rest of a commented line kept
fn prose_lines(raw: &[&str]) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let body = spec::front_matter_span(raw);
    let mut fence = spec::Fence::default();
    let mut comment = false;
    for (i, line) in raw.iter().enumerate() {
        if i < body {
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
        let s = scan(page, 8, &Chrome::default());
        let got: Vec<(usize, &str, bool)> = s.findings.iter().map(|f| (f.line, f.first.as_str(), f.empty_valve)).collect();
        assert_eq!(got, vec![(8, "line 1", false), (8, "line 1", false), (11, "line 1", true)]);
        assert_eq!(s.links, 5);
    }

    #[test]
    fn a_page_restating_the_chrome_reds_at_the_chrome_locator() {
        let mut chrome = Chrome::default();
        chrome.add(
            "_layouts/default.html",
            "<a href=\"{{ '/' | relative_url }}\">Home</a>\n<script>\n  var s = 'one two three four five six seven eight.';\n</script>\n<footer>Built by the same small team that ships it.\n  <a href=\"https://example.dev/repo\">Source</a>.</footer>\n",
            8,
        );
        assert_eq!(chrome.targets.len(), 1);
        assert_eq!(chrome.targets["https://example.dev/repo"], "_layouts/default.html:6");
        assert_eq!(chrome.sentences.len(), 1);
        let page = "See [the repo](https://example.dev/repo).\n\nOne two three four five six seven eight.\n\nBuilt by the same small team that ships it.\n\n[home](/)\n";
        let s = scan(page, 8, &chrome);
        let got: Vec<(usize, &str, &str)> = s.findings.iter().map(|f| (f.line, f.first.as_str(), f.subject.as_str())).collect();
        assert_eq!(
            got,
            vec![
                (1, "_layouts/default.html:6", "https://example.dev/repo"),
                (5, "_layouts/default.html:5", "built by the same small team that ships it"),
            ]
        );
    }

    #[test]
    fn a_spaced_href_is_read_and_one_inside_a_template_expression_is_not() {
        let mut chrome = Chrome::default();
        chrome.add(
            "_layouts/default.html",
            "<a href = \"https://example.dev/widget\">Source</a>\n{% assign x = '<a href=\"https://example.dev/other\">O</a>' %}\n<a href=\"/x/{{ page.url }}\">Self</a>\n<a\n  data-href=\"https://example.dev/data\" HREF='https://example.dev/caps'>C</a>\n",
            8,
        );
        let mut got: Vec<(&str, &str)> = chrome.targets.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        got.sort();
        assert_eq!(
            got,
            vec![
                ("https://example.dev/caps", "_layouts/default.html:5"),
                ("https://example.dev/widget", "_layouts/default.html:1"),
            ]
        );
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
