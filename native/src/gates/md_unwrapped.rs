// spec: canon-kit/SPEC.md §check-md-unwrapped — no paragraph in the governed markdown set is
// broken across physical lines; the block scanner here is the one `--emit md-unwrap` joins by
use crate::spec;
use crate::{proc, programs};

const NAME: &str = "check-md-unwrapped";

// spec: canon-kit/SPEC.md §check-md-unwrapped — one line's verdict: `soft` marks a paragraph
// continuation, and `content` is the byte offset its text starts at once the quote prefix and
// the indentation are dropped, which is what the join keeps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub soft: bool,
    pub content: usize,
}

fn indent(s: &str) -> usize {
    let mut n = 0usize;
    for c in s.chars() {
        match c {
            ' ' => n += 1,
            '\t' => n += 4 - n % 4,
            _ => break,
        }
    }
    n
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — the block-quote prefix: each `>` (with up to three
// spaces before it and one optional space after) is one level of quoting
fn strip_quotes(line: &str) -> (usize, usize) {
    let b = line.as_bytes();
    let mut depth = 0usize;
    let mut at = 0usize;
    loop {
        let mut i = at;
        while i < b.len() && i - at < 4 && b[i] == b' ' {
            i += 1;
        }
        if i < b.len() && b[i] == b'>' && i - at < 4 {
            depth += 1;
            at = i + 1;
            if at < b.len() && b[at] == b' ' {
                at += 1;
            }
        } else {
            return (depth, at);
        }
    }
}

fn fence_marker(t: &str) -> Option<(u8, usize)> {
    let b = t.as_bytes();
    let c = *b.first()?;
    if c != b'`' && c != b'~' {
        return None;
    }
    let n = b.iter().take_while(|&&x| x == c).count();
    if n >= 3 {
        Some((c, n))
    } else {
        None
    }
}

fn is_atx(t: &str) -> bool {
    let n = t.bytes().take_while(|&b| b == b'#').count();
    (1..=6).contains(&n) && matches!(t.as_bytes().get(n), None | Some(b' ') | Some(b'\t'))
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — a thematic break or setext underline: one of
// `-`, `*`, `_`, `=` repeated at least three times, spaces allowed between
fn is_rule(t: &str) -> bool {
    let body: Vec<u8> = t.bytes().filter(|&b| b != b' ' && b != b'\t').collect();
    match body.first() {
        Some(&c) if matches!(c, b'-' | b'*' | b'_' | b'=') => body.len() >= 3 && body.iter().all(|&b| b == c),
        _ => false,
    }
}

fn is_list_marker(t: &str) -> bool {
    let b = t.as_bytes();
    if b.is_empty() {
        return false;
    }
    let after = if matches!(b[0], b'-' | b'*' | b'+') {
        1
    } else {
        let d = b.iter().take_while(|x| x.is_ascii_digit()).count();
        if d == 0 || d > 9 || !matches!(b.get(d), Some(b'.') | Some(b')')) {
            return false;
        }
        d + 1
    };
    matches!(b.get(after), None | Some(b' ') | Some(b'\t'))
}

fn is_html_start(t: &str) -> bool {
    let rest = match t.strip_prefix('<') {
        Some(r) => r,
        None => return false,
    };
    // path-dialect-exempt: markdown source text, where `</` opens a closing HTML tag
    rest.starts_with("!--") || rest.starts_with('/') || rest.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
}

fn is_link_def(t: &str) -> bool {
    t.starts_with('[') && t.find("]:").is_some_and(|i| i > 1)
}

const LEADS_KNOB: &str = "CANON_KIT_UNWRAP_DECLARATION_LEADS";

// spec: canon-kit/SPEC.md §check-md-unwrapped — the one block start taken from configuration
// rather than from CommonMark, tested on the same content the CommonMark starts are
fn is_declared_lead(t: &str, leads: &[String]) -> bool {
    leads.iter().any(|lead| t.starts_with(lead.as_str()))
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — an empty member is malformed config, refused at
// exit 2 rather than silently opening a block at every line
fn validate_leads(leads: Vec<String>) -> Result<Vec<String>, String> {
    match leads.iter().position(String::is_empty) {
        Some(i) => Err(format!(
            "{} member {} is empty — a lead is the text a declaration line starts with, \
             and an empty one opens a block at every line",
            LEADS_KNOB,
            i + 1
        )),
        None => Ok(leads),
    }
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — the gate and `--emit md-unwrap` resolve the lead
// set through this one reader, so the two can never disagree about what opens a block
pub fn declaration_leads() -> Result<Vec<String>, String> {
    validate_leads(spec::knob_array_pub(LEADS_KNOB)?)
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — a hard break: two trailing spaces, or a trailing
// backslash outside an open code span
fn hard_break(line: &str) -> bool {
    if line.ends_with("  ") {
        return true;
    }
    line.ends_with('\\') && line.bytes().filter(|&b| b == b'`').count() % 2 == 0
}

enum Html {
    Comment,
    Block,
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — the block scanner, a CommonMark subset: one
// verdict per input line, in order
pub fn scan(text: &str, leads: &[String]) -> Vec<Line> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<Line> = Vec::with_capacity(lines.len());
    let mut fence: Option<(u8, usize)> = None;
    let mut html: Option<Html> = None;
    let mut para: Option<usize> = None;
    let mut list_open = false;
    let mut exempt_next = false;
    let mut start = 0usize;
    if lines.first().map(|l| l.trim_end()) == Some("---") {
        if let Some(end) = lines.iter().skip(1).position(|l| l.trim_end() == "---") {
            for _ in 0..end + 2 {
                out.push(Line { soft: false, content: 0 });
            }
            start = end + 2;
        }
    }
    for line in lines.iter().skip(start) {
        let (depth, qat) = strip_quotes(line);
        let body = &line[qat..];
        let ind = indent(body);
        let t = body.trim_start_matches([' ', '\t']);
        let content = line.len() - t.len();
        let mut verdict = Line { soft: false, content };
        if let Some((c, n)) = fence {
            if fence_marker(t).is_some_and(|(c2, n2)| c2 == c && n2 >= n) && t.trim_end().bytes().all(|b| b == c) {
                fence = None;
            }
            out.push(verdict);
            continue;
        }
        if let Some(h) = &html {
            match h {
                Html::Comment => {
                    if line.contains("-->") {
                        html = None;
                    }
                }
                Html::Block => {
                    if t.is_empty() {
                        html = None;
                    }
                }
            }
            out.push(verdict);
            continue;
        }
        if t.is_empty() {
            para = None;
            exempt_next = false;
            out.push(verdict);
            continue;
        }
        let continues = para.is_some_and(|d| depth <= d);
        let code = !list_open && !continues && ind >= 4;
        let opens = !code
            && (is_atx(t)
                || fence_marker(t).is_some()
                || t.starts_with('|')
                || is_rule(t)
                || is_list_marker(t)
                || is_html_start(t)
                || is_link_def(t)
                || is_declared_lead(t, leads));
        let quote_start = para.is_some_and(|d| depth > d);
        if continues && !opens && !quote_start {
            verdict.soft = !exempt_next;
            exempt_next = hard_break(line);
            out.push(verdict);
            continue;
        }
        exempt_next = false;
        para = None;
        if code {
            out.push(verdict);
            continue;
        }
        if let Some(f) = fence_marker(t) {
            fence = Some(f);
        } else if is_html_start(t) {
            if t.starts_with("<!--") {
                if !t.contains("-->") {
                    html = Some(Html::Comment);
                }
            } else {
                html = Some(Html::Block);
            }
        } else if is_list_marker(t) && !is_rule(t) {
            list_open = true;
            para = Some(depth);
            exempt_next = hard_break(line);
        } else if !(is_atx(t) || t.starts_with('|') || is_rule(t) || is_link_def(t)) {
            if ind == 0 && depth == 0 {
                list_open = false;
            }
            para = Some(depth);
            exempt_next = hard_break(line);
        } else if ind == 0 && depth == 0 {
            list_open = false;
        }
        out.push(verdict);
    }
    out
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — the join the arm performs: each soft break onto
// its predecessor with one space, the continuation's indentation and quote prefix dropped
pub fn unwrap(text: &str, leads: &[String]) -> String {
    let verdicts = scan(text, leads);
    let mut out: Vec<String> = Vec::with_capacity(verdicts.len());
    for (line, v) in text.lines().zip(&verdicts) {
        match (v.soft, out.last_mut()) {
            (true, Some(prev)) => {
                let kept = prev.trim_end_matches([' ', '\t']).len();
                prev.truncate(kept);
                prev.push(' ');
                prev.push_str(&line[v.content..]);
            }
            _ => out.push(line.to_string()),
        }
    }
    let mut s = out.join("\n");
    if text.ends_with('\n') {
        s.push('\n');
    }
    s
}

// spec: canon-kit/SPEC.md §check-md-unwrapped — the corpus: tracked files matching the include
// globs less the exclude globs, both as git pathspecs
fn corpus(include: &[String], exclude: &[String]) -> Result<Vec<String>, String> {
    let mut argv: Vec<String> = vec!["ls-files".into(), "-z".into(), "--".into()];
    argv.extend(include.iter().cloned());
    argv.extend(exclude.iter().map(|e| format!(":(exclude){}", e)));
    let refs: Vec<&str> = argv.iter().map(String::as_str).collect();
    let ls = proc::run(&programs::GIT, &refs)?;
    match ls.stdout() {
        Some(o) => Ok(String::from_utf8_lossy(o)
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()),
        None => Err(format!(
            "git ls-files exited {} — the governed markdown set could not be enumerated",
            ls.code().unwrap_or(-1)
        )),
    }
}

fn head(s: &str) -> String {
    let h: String = s.chars().take(40).collect();
    if s.chars().count() > 40 {
        format!("{}…", h)
    } else {
        h
    }
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let leads = declaration_leads()?;
    let files = if !args.is_empty() {
        args.to_vec()
    } else {
        let include = spec::knob_array_pub("CANON_KIT_UNWRAP_GLOBS")?;
        if include.is_empty() {
            println!("MD-UNWRAPPED: clean (CANON_KIT_UNWRAP_GLOBS is unset; nothing scanned)");
            return Ok(0);
        }
        let exclude = spec::knob_array_pub("CANON_KIT_UNWRAP_EXCLUDE")?;
        corpus(&include, &exclude)?
    };
    let mut findings: Vec<String> = Vec::new();
    for f in &files {
        let text = std::fs::read_to_string(f).map_err(|e| format!("cannot read {}: {}", f, e))?;
        for (i, (line, v)) in text.lines().zip(scan(&text, &leads)).enumerate() {
            if v.soft {
                findings.push(format!("  {}:{}: {}", f, i + 1, head(&line[v.content..])));
            }
        }
    }
    if !findings.is_empty() {
        println!("{}: paragraph(s) broken across lines (each line continues the one above):", NAME);
        for f in &findings {
            println!("{}", f);
        }
        println!("  help: join each paragraph onto one line —");
        println!("        bash gate-sdk/bin/run-gates.sh --emit md-unwrap --write <file>…");
        return Ok(1);
    }
    println!("MD-UNWRAPPED: clean ({} file(s); no paragraph broken across lines)", files.len());
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn soft(text: &str) -> Vec<usize> {
        soft_with(text, &[])
    }

    fn soft_with(text: &str, leads: &[String]) -> Vec<usize> {
        scan(text, leads).iter().enumerate().filter(|(_, v)| v.soft).map(|(i, _)| i + 1).collect()
    }

    fn leads(of: &[&str]) -> Vec<String> {
        of.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_wrapped_paragraph_list_item_and_quote_are_soft_breaks() {
        assert_eq!(soft("one\ntwo\n"), vec![2]);
        assert_eq!(soft("- one\n  two\n"), vec![2]);
        assert_eq!(soft("> one\n> two\n"), vec![2]);
        assert_eq!(soft("> one\ntwo\n"), vec![2]);
        assert_eq!(soft("- one\nlazy\n"), vec![2]);
    }

    #[test]
    fn a_block_start_never_continues_a_paragraph() {
        for next in ["# h", "```", "| a |", "---", "- x", "1. x", "2) x", "<div>", "<!-- c -->", "[a]: b", "> # q"] {
            let text = format!("para\n{}\n", next);
            assert!(soft(&text).is_empty(), "{:?}", next);
        }
    }

    #[test]
    fn exempt_interiors_are_outside_the_grammar() {
        assert!(soft("```\na\nb\n```\n").is_empty());
        assert!(soft("<!--\na\nb\n-->\n").is_empty());
        assert!(soft("<div>\na\nb\n</div>\n").is_empty());
        assert!(soft("---\ntitle: a\nb: c\n---\n").is_empty());
        assert!(soft("    code\n    more\n").is_empty());
        assert!(soft("a  \nb\n").is_empty());
        assert!(soft("a\\\nb\n").is_empty());
    }

    #[test]
    fn a_nested_list_continuation_joins_the_innermost_item() {
        assert_eq!(soft("- a\n  - b\n    c\n"), vec![3]);
        assert_eq!(soft("- a\n\n      more\n"), Vec::<usize>::new());
    }

    #[test]
    fn the_join_is_the_gate_s_clean_verdict_and_deletes_no_word() {
        let text = "Intro line\n  continues here.\n\n- item one\n  wraps\n    again\n> quoted\n> more\n\n```\nkeep\nme\n```\n";
        let joined = unwrap(text, &[]);
        assert_eq!(
            joined,
            "Intro line continues here.\n\n- item one wraps again\n> quoted more\n\n```\nkeep\nme\n```\n"
        );
        assert!(soft(&joined).is_empty());
        let words = |s: &str| s.split_whitespace().filter(|w| *w != ">").map(String::from).collect::<Vec<_>>();
        assert_eq!(words(text), words(&joined));
    }

    #[test]
    fn the_join_holds_its_postcondition_over_both_fixture_trees() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for case in ["good", "bad"] {
            let p = root.join("../canon-kit/gate-tests/check-md-unwrapped").join(case).join("doc.md");
            let text = std::fs::read_to_string(&p).expect("fixture");
            for set in [leads(&[]), leads(&["note:"])] {
                assert!(soft_with(&unwrap(&text, &set), &set).is_empty(), "{} {:?}", p.display(), set);
            }
        }
    }

    #[test]
    fn a_declared_lead_opens_a_block_and_the_join_leaves_its_line_alone() {
        let text = "A paragraph.\nnote: a declaration the reader keys on.\n";
        assert_eq!(soft(text), vec![2]);
        assert_eq!(unwrap(text, &[]), "A paragraph. note: a declaration the reader keys on.\n");
        let set = leads(&["note:"]);
        assert!(soft_with(text, &set).is_empty());
        assert_eq!(unwrap(text, &set), text);
        // spec: canon-kit/SPEC.md §check-md-unwrapped — the lead is read after the indentation and
        // the block-quote markers, and it opens a paragraph a later continuation still reds
        assert!(soft_with("A paragraph.\n>   note: indented, quoted.\n", &set).is_empty());
        assert_eq!(soft_with("note: a declaration\nwrapped onto a second line.\n", &set), vec![2]);
        assert_eq!(soft_with(text, &leads(&["ruling:"])), vec![2]);
    }

    #[test]
    fn an_empty_declaration_lead_is_malformed_config() {
        let e = validate_leads(leads(&["note:", ""])).expect_err("empty member");
        assert!(e.contains(LEADS_KNOB) && e.contains("member 2"), "{}", e);
        assert_eq!(validate_leads(leads(&["note:"])).expect("valid"), leads(&["note:"]));
    }
}
