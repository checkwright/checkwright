// spec: site-kit/SPEC.md §check-docs-render-fidelity — every tracked docs markdown page, rendered
// through the Pages parser, leaks no code-span corruption symptom into text, promotes no
// code-fenced heading, and renders no fewer tables than its source GFM table starts
use crate::fresh;
use crate::proc;
use crate::walk;
use std::path::Path;

// spec: site-kit/SPEC.md §check-docs-render-fidelity — the known-HTML-element set is a kit
// built-in, not a config seam
const HTML_ELEMENTS: &[&str] = &[
    "a", "abbr", "address", "area", "article", "aside", "audio", "b", "base", "bdi", "bdo",
    "blockquote", "body", "br", "button", "canvas", "caption", "cite", "code", "col", "colgroup",
    "data", "datalist", "dd", "del", "details", "dfn", "dialog", "div", "dl", "dt", "em", "embed",
    "fieldset", "figcaption", "figure", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6",
    "head", "header", "hgroup", "hr", "html", "i", "iframe", "img", "input", "ins", "kbd", "label",
    "legend", "li", "link", "main", "map", "mark", "menu", "meta", "meter", "nav", "noscript",
    "object", "ol", "optgroup", "option", "output", "p", "param", "picture", "pre", "progress",
    "q", "rp", "rt", "ruby", "s", "samp", "script", "search", "section", "select", "slot", "small",
    "source", "span", "strong", "style", "sub", "summary", "sup", "table", "tbody", "td",
    "template", "textarea", "tfoot", "th", "thead", "time", "title", "tr", "track", "u", "ul",
    "var", "video", "wbr",
];

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — a renderer knob is an argv, so the shell
// form's `"${KNOB[@]}"` is a program plus its arguments; an empty array is the branch the batch
// knob's conditional default produces and never a program named the empty string
fn spawn_filter(argv: &[String], input: &[u8], stderr: proc::Stderr) -> Result<(i32, Vec<u8>), String> {
    let Some((program, rest)) = argv.split_first() else {
        // spec: site-kit/SPEC.md §check-docs-render-fidelity — bash runs an empty expansion as the
        // null command: status 0 and no output, which the caller's own emptiness test then reds
        return Ok((0, Vec::new()));
    };
    let args: Vec<&str> = rest.iter().map(String::as_str).collect();
    let s = proc::run_streamed(program, &args, input, stderr)?;
    Ok((s.code(), s.stdout().to_vec()))
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — `read -r -d ''`: a record is what a NUL
// *terminates*, so a trailing fragment the stream never terminated is not a document. That is the
// property the count assertion grades a truncated renderer by.
fn nul_records(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut cur: Vec<u8> = Vec::new();
    for b in bytes {
        if *b == 0 {
            out.push(std::mem::take(&mut cur));
        } else {
            cur.push(*b);
        }
    }
    out
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — the shell form's `${html%$'\n'}` loop and
// `$(…)` both drop *every* trailing newline, so the two render paths write the same page file
fn trim_trailing_newlines(s: &str) -> &str {
    s.trim_end_matches('\n')
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — the framing's unforgeability rests on the
// reader having dropped any NUL before framing happens; bash could not hold one, a compiled
// reader can, so it drops them on purpose
fn strip_nuls(s: &str) -> String {
    s.chars().filter(|c| *c != '\0').collect()
}

fn inner(args: &[String]) -> Result<i32, String> {
    let docs_knob = walk::knob_scalar("SITE_KIT_DOCS_DIR").map_err(|e| format!("{}: {}", NAME, e))?;
    let docs = fresh::strip_trailing_slash(fresh::positional(args, 0, &docs_knob)).to_string();

    let probe = proc::run("git", &["rev-parse", "--git-dir"]).map_err(|e| format!("{}: {}", NAME, e))?;
    if probe.stdout().is_none() {
        return Err(format!(
            "{}: not a git repository — cannot enumerate tracked pages",
            NAME
        ));
    }
    if !fresh::is_dir(&docs) {
        return Err(format!("{}: docs dir not found: {}", NAME, docs));
    }

    let batch = walk::knob_array("SITE_KIT_RENDERER_BATCH").map_err(|e| format!("{}: {}", NAME, e))?;
    let renderer = walk::knob_array("SITE_KIT_RENDERER").map_err(|e| format!("{}: {}", NAME, e))?;

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — probe the oracle the gate will actually
    // run, and only that one: a set batch knob is the consumer's statement of which parser is
    // authoritative
    if !batch.is_empty() {
        let (_, out) = spawn_filter(&batch, b"# probe one\n\0# probe two\n\0", proc::Stderr::Discard)?;
        // spec: site-kit/SPEC.md §check-docs-render-fidelity — two documents in, exactly two
        // *non-empty* documents back. Emptiness is bash's `${d//[[:space:]]/}`, whose C-locale
        // class carries the vertical tab Rust's `is_ascii_whitespace` leaves out.
        let bprobe = nul_records(&out)
            .iter()
            .filter(|d| d.iter().any(|b| !posix_space(*b)))
            .count();
        if bprobe != 2 {
            return Err(format!(
                "{}: batch renderer '{}' failed its probe\n  (2 documents in, {} non-empty document(s) back)\n  help: SITE_KIT_RENDERER_BATCH must read NUL-terminated documents from stdin and write\n        one NUL-terminated HTML document per input, in order; unset it to fall back to\n        the per-document SITE_KIT_RENDERER contract",
                NAME,
                batch.join(" "),
                bprobe
            ));
        }
    } else {
        let (pst, out) = spawn_filter(&renderer, b"# probe\n", proc::Stderr::Discard)?;
        let text = String::from_utf8_lossy(&out).into_owned();
        if pst != 0 || trim_trailing_newlines(&text).is_empty() {
            return Err(format!(
                "{}: renderer '{}' could not run (exit {})\n  help: install the Pages parser — ruby plus the kramdown-parser-gfm gem — or point\n        SITE_KIT_RENDERER at a stdin->stdout GFM-to-HTML command",
                NAME,
                renderer.join(" "),
                pst
            ));
        }
    }

    let ls = proc::run("git", &["ls-files", "--", &docs]).map_err(|e| format!("{}: {}", NAME, e))?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            return Err(format!(
                "DOCS-RENDER-FIDELITY: {}",
                fresh::fail_closed("git-ls-files", ls.code())
            ))
        }
    };

    let prune = walk::prune_dirs().map_err(|e| format!("{}: {}", NAME, e))?;
    let mut pages: Vec<String> = Vec::new();
    for p in listing.lines() {
        if p.is_empty() || !p.ends_with(".md") || jekyll_internal(p) || walk::path_pruned(p, &prune) {
            continue;
        }
        if Path::new(p).is_file() {
            pages.push(p.to_string());
        }
    }

    if pages.is_empty() {
        println!(
            "DOCS-RENDER-FIDELITY: clean (0 tracked markdown page(s) under {})",
            docs
        );
        return Ok(0);
    }

    let mut bodies: Vec<String> = Vec::new();
    for p in &pages {
        let raw = std::fs::read(p)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("{}: cannot read {}: {}", NAME, p, e))?;
        bodies.push(strip_front_matter(&raw));
    }

    let mut htmls: Vec<String> = Vec::new();
    if !batch.is_empty() {
        let mut stream: Vec<u8> = Vec::new();
        for b in &bodies {
            stream.extend_from_slice(render_input(b).as_bytes());
            stream.push(0);
        }
        let (_, out) = spawn_filter(&batch, &stream, proc::Stderr::Inherit)?;
        for d in nul_records(&out) {
            let text = String::from_utf8_lossy(&d).into_owned();
            htmls.push(trim_trailing_newlines(&text).to_string());
        }
        // spec: site-kit/SPEC.md §check-docs-render-fidelity — the count assertion stands in for
        // the per-page exit status the shell form's process substitution discarded, so it catches
        // renderer death, truncation and framing error alike
        if htmls.len() != pages.len() {
            return Err(format!(
                "{}: batch renderer returned {} document(s) for {} page(s)\n  help: SITE_KIT_RENDERER_BATCH must write exactly one NUL-terminated HTML document per\n        NUL-terminated input document, in order; a short count is a renderer that died\n        mid-stream, truncated its output, or framed it wrongly",
                NAME,
                htmls.len(),
                pages.len()
            ));
        }
    } else {
        for b in &bodies {
            let (rst, out) = spawn_filter(&renderer, render_input(b).as_bytes(), proc::Stderr::Inherit)?;
            if rst != 0 {
                return Err(format!(
                    "DOCS-RENDER-FIDELITY: {}",
                    fresh::fail_closed("renderer", Some(rst))
                ));
            }
            let text = String::from_utf8_lossy(&out).into_owned();
            htmls.push(trim_trailing_newlines(&text).to_string());
        }
    }

    let mut findings: Vec<String> = Vec::new();
    for (i, page) in pages.iter().enumerate() {
        let r = scan_rendered(&htmls[i]);
        let scount = count_source_headings(&bodies[i]);
        let stbl = count_source_tables(&bodies[i]);

        if r.leak {
            findings.push(format!("{}: a code-span corruption symptom (a stray backtick, or a raw non-HTML-element tag) leaked into rendered text — a code span or fenced block failed to parse", page));
        }
        if r.headings > scount {
            findings.push(format!(
                "{}: {} rendered heading(s) exceed {} source heading(s) outside code — a code-fenced '#' line was promoted",
                page, r.headings, scount
            ));
        }
        if r.tables < stbl {
            findings.push(format!(
                "{}: {} rendered table(s) fall short of {} source GFM table start(s) — a table collapsed into literal-pipe paragraph text (a row abutting a non-blank line)",
                page, r.tables, stbl
            ));
        }
    }

    if !findings.is_empty() {
        println!("{}: rendered docs page(s) diverge from source under the Pages parser:", NAME);
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: restructure the offending block so kramdown's GFM parser renders it faithfully — an");
        println!("        indented (4-space) code block avoids the consecutive-fence and unclosed-fence leakage");
        println!("        class; a doubled-backtick code span kept on one line (never split across a newline");
        println!("        before a <word> token) avoids the severed-span class.");
        return Ok(1);
    }
    println!(
        "DOCS-RENDER-FIDELITY: clean ({} tracked markdown page(s) under {} render with no span-corruption, heading, or table leakage)",
        pages.len(),
        docs
    );
    Ok(0)
}

const NAME: &str = "check-docs-render-fidelity";

// spec: site-kit/SPEC.md §check-docs-render-fidelity — every underscore-prefixed directory
// segment excluded, those being Jekyll internals rather than published pages. The shell form's
// `case "/$p/" in */_*/*)` brackets the path with slashes, so the *basename* is a segment too.
fn jekyll_internal(p: &str) -> bool {
    p.split('/').any(|seg| seg.starts_with('_'))
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — strip Jekyll front matter so the gate
// renders exactly the body kramdown sees: a leading `---` opens it and the next `---` closes it,
// and every kept record is newline-terminated the way awk's `print` wrote it
fn strip_front_matter(raw: &str) -> String {
    let mut out = String::new();
    let mut fm = false;
    for (i, line) in fresh::file_lines(raw).iter().enumerate() {
        if i == 0 && *line == "---" {
            fm = true;
            continue;
        }
        if fm {
            if *line == "---" {
                fm = false;
            }
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — what the shell form handed the renderer:
// the body read through a substitution that drops NULs and every trailing newline, then written
// back with exactly one
fn render_input(body: &str) -> String {
    let mut s = strip_nuls(body);
    while s.ends_with('\n') {
        s.pop();
    }
    s.push('\n');
    s
}

struct RenderedScan {
    leak: bool,
    headings: usize,
    tables: usize,
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — the rendered-page scan: heading and table
// element counts, and the span-corruption symptom taken from the text left once `<pre>` blocks and
// inline `<code>` spans are removed
fn scan_rendered(html: &str) -> RenderedScan {
    let mut out = RenderedScan {
        leak: false,
        headings: 0,
        tables: 0,
    };
    let mut inpre = false;
    let mut foreign: usize = 0;
    for line in fresh::file_lines(&format!("{}\n", html)) {
        out.headings += count_open_tags(line, "h", true);
        out.tables += count_open_tags(line, "table", false);
        if inpre {
            if line.contains("</pre>") {
                inpre = false;
            }
            continue;
        }
        if line.contains("<pre") {
            inpre = true;
            continue;
        }
        let s = strip_code_spans(line);
        for tag in tags(&s) {
            let closing = tag.starts_with("</");
            let selfclose = tag.ends_with("/>");
            let name = tag_name(tag);
            // spec: site-kit/SPEC.md §check-docs-render-fidelity — foreign-content subtree,
            // tracked by depth so an unknown name inside it is legitimate SVG/MathML vocabulary
            // rather than a leaked placeholder
            if name == "svg" || name == "math" {
                if closing {
                    foreign = foreign.saturating_sub(1);
                } else if !selfclose {
                    foreign += 1;
                }
                continue;
            }
            if foreign > 0 {
                continue;
            }
            if !HTML_ELEMENTS.contains(&name.as_str()) {
                out.leak = true;
            }
        }
        // spec: site-kit/SPEC.md §check-docs-render-fidelity — symptom (a), any literal backtick,
        // not only a fence run
        if strip_tags(&s).contains('`') {
            out.leak = true;
        }
    }
    out
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — awk's `while (match($0, /<h[1-6][ >]/))`
// and its `<table[ >]` twin: every occurrence on the line, the scan resuming past each match
fn count_open_tags(line: &str, stem: &str, numbered: bool) -> usize {
    let b = line.as_bytes();
    let open = format!("<{}", stem);
    let ob = open.as_bytes();
    let mut n = 0usize;
    let mut i = 0usize;
    while i + ob.len() < b.len() {
        if &b[i..i + ob.len()] == ob {
            let mut j = i + ob.len();
            if numbered {
                if j >= b.len() || !(b'1'..=b'6').contains(&b[j]) {
                    i += 1;
                    continue;
                }
                j += 1;
            }
            if j < b.len() && (b[j] == b' ' || b[j] == b'>') {
                n += 1;
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    n
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — `gsub(/<code[^>]*>[^<]*<\/code>/, "", s)`:
// a legitimate backtick renders inside `<code>` and a legitimate embedded tag renders as escaped
// entities inside it, so both are excluded before the check
fn strip_code_spans(line: &str) -> String {
    let b = line.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if b[i..].starts_with(b"<code") {
            if let Some(gt) = find_byte(b, i + 5, b'>') {
                if let Some(lt) = find_byte(b, gt + 1, b'<') {
                    if b[lt..].starts_with(b"</code>") {
                        i = lt + 7;
                        continue;
                    }
                }
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn posix_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r')
}

fn find_byte(b: &[u8], from: usize, needle: u8) -> Option<usize> {
    (from..b.len()).find(|&k| b[k] == needle)
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — symptom (b)'s scan,
// `/<\/?[A-Za-z][A-Za-z0-9]*[^>]*>/`: `[^>]*` cannot cross a `>`, so a tag runs from its `<` to
// the first `>` after the element name and the leftmost-longest span is that one
fn tags(s: &str) -> Vec<&str> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'<' {
            let mut j = i + 1;
            if j < b.len() && b[j] == b'/' {
                j += 1;
            }
            if j < b.len() && b[j].is_ascii_alphabetic() {
                if let Some(gt) = find_byte(b, j, b'>') {
                    out.push(&s[i..gt + 1]);
                    i = gt + 1;
                    continue;
                }
                return out;
            }
        }
        i += 1;
    }
    out
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — matched by name, so an attribute-bearing
// legitimate tag (`<a href…>`) is excluded and a placeholder token (`<verdict>`) is not
fn tag_name(tag: &str) -> String {
    let t = tag.trim_start_matches('<').trim_start_matches('/');
    t.chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — `gsub(/<[^>]*>/, "", s)`: what is left is
// the rendered text content the backtick symptom is read from
fn strip_tags(s: &str) -> String {
    let b = s.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'<' {
            if let Some(gt) = find_byte(b, i + 1, b'>') {
                i = gt + 1;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — the source-side scans' shared fence state:
// cmark's rules, a fence opened by three or more backticks or tildes indented at most three
// spaces and closed by a run of at least that length of the same character
struct Fence {
    inside: bool,
    ch: u8,
    len: usize,
}

struct Line<'a> {
    raw: &'a str,
    body: &'a str,
    indent: usize,
    fence_ch: u8,
    fence_len: usize,
}

fn read_line(raw: &str) -> Line<'_> {
    let indent = raw.len() - raw.trim_start_matches(' ').len();
    let body = &raw[indent..];
    let (mut ch, mut len) = (0u8, 0usize);
    if indent <= 3 {
        let first = body.as_bytes().first().copied().unwrap_or(0);
        if first == b'`' || first == b'~' {
            let run = body.as_bytes().iter().take_while(|c| **c == first).count();
            if run >= 3 {
                ch = first;
                len = run;
            }
        }
    }
    Line {
        raw,
        body,
        indent,
        fence_ch: ch,
        fence_len: len,
    }
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — the closing-fence test the awk programs
// build as `"^" fchar "{" flen ",}[ \t]*$"`: the same character, at least as long, and nothing
// after the run but spaces and tabs
fn closes(f: &Fence, l: &Line) -> bool {
    l.fence_len > 0
        && l.fence_ch == f.ch
        && l.fence_len >= f.len
        && l.body[l.fence_len..].bytes().all(|c| c == b' ' || c == b'\t')
}

fn blank(raw: &str) -> bool {
    raw.bytes().all(|c| c == b' ' || c == b'\t')
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — the count of source heading lines the
// gate's own fence-aware scan places outside any code context: ATX and setext, both skipped
// inside a fenced or `~`-fenced block
fn count_source_headings(body: &str) -> usize {
    let mut f = Fence {
        inside: false,
        ch: 0,
        len: 0,
    };
    let mut count = 0usize;
    let mut prevblank = true;
    let mut previsatx = false;
    for raw in fresh::file_lines(body) {
        let l = read_line(raw);
        if f.inside {
            if closes(&f, &l) {
                f.inside = false;
            }
            prevblank = blank(l.raw);
            previsatx = false;
            continue;
        }
        if l.fence_len > 0 {
            f = Fence {
                inside: true,
                ch: l.fence_ch,
                len: l.fence_len,
            };
            prevblank = false;
            previsatx = false;
            continue;
        }
        if l.indent <= 3 && setext(l.body) && !prevblank && !previsatx {
            count += 1;
            prevblank = true;
            previsatx = false;
            continue;
        }
        if l.indent <= 3 && atx(l.body) {
            count += 1;
            prevblank = false;
            previsatx = true;
            continue;
        }
        prevblank = blank(l.raw);
        previsatx = false;
    }
    count
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — `/^=+[ \t]*$/` and `/^-+[ \t]*$/`
fn setext(body: &str) -> bool {
    for marker in *b"=-" {
        let run = body.bytes().take_while(|c| *c == marker).count();
        if run > 0 && body.as_bytes()[run..].iter().all(|c| *c == b' ' || *c == b'\t') {
            return true;
        }
    }
    false
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — `/^#{1,6}([ \t]|$)/`: a seventh `#` leaves
// no split of the bounded repeat that a space or an end-of-line can follow, so it is not a heading
fn atx(body: &str) -> bool {
    let run = body.bytes().take_while(|c| *c == b'#').count();
    if !(1..=6).contains(&run) {
        return false;
    }
    match body.as_bytes().get(run) {
        None => true,
        Some(c) => *c == b' ' || *c == b'\t',
    }
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — count source GFM table starts (a pipe row
// followed by a `| --- |` delimiter row) outside code fences
fn count_source_tables(body: &str) -> usize {
    let mut f = Fence {
        inside: false,
        ch: 0,
        len: 0,
    };
    let mut count = 0usize;
    let mut prevpipe = false;
    for raw in fresh::file_lines(body) {
        let l = read_line(raw);
        if f.inside {
            if closes(&f, &l) {
                f.inside = false;
            }
            prevpipe = false;
            continue;
        }
        if l.fence_len > 0 {
            f = Fence {
                inside: true,
                ch: l.fence_ch,
                len: l.fence_len,
            };
            prevpipe = false;
            continue;
        }
        if l.indent <= 3 && delimiter_row(l.body) && prevpipe {
            count += 1;
            prevpipe = false;
            continue;
        }
        if blank(l.raw) {
            prevpipe = false;
            continue;
        }
        prevpipe = l.raw.contains('|');
    }
    count
}

// spec: site-kit/SPEC.md §check-docs-render-fidelity — `/^\|?[ \t:|-]*-[ \t:|-]*$/` conjoined
// with `/\|/`: `|` is itself in the class, so the leading alternative adds nothing and the rule is
// exactly "every byte is a delimiter-row byte, and at least one `-` and one `|` are present"
fn delimiter_row(body: &str) -> bool {
    body.bytes()
        .all(|c| matches!(c, b' ' | b'\t' | b':' | b'|' | b'-'))
        && body.contains('-')
        && body.contains('|')
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — the exemption follows the subtree by
    // open/close depth rather than a name list, so a bare `<path>` outside any `<svg>` still reds
    #[test]
    fn the_foreign_content_exemption_is_scoped_to_the_subtree() {
        assert!(!scan_rendered("<p><svg><circle r=\"1\"/></svg></p>").leak);
        assert!(scan_rendered("<p><path d=\"M0\"/></p>").leak);
        assert!(scan_rendered("<p>the <verdict> token</p>").leak);
        assert!(!scan_rendered("<p>see <a href=\"x\">x</a></p>").leak);
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — a legitimate backtick renders inside
    // `<code>` and is excluded; one surviving in text is a span that failed to form
    #[test]
    fn a_backtick_reds_only_outside_a_code_span() {
        assert!(!scan_rendered("<p><code>`</code></p>").leak);
        assert!(scan_rendered("<p>a ` stray</p>").leak);
        assert!(!scan_rendered("<pre><code>` in a block\n</code></pre>").leak);
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — the counts the two leakage assertions
    // compare, taken before the `<pre>` skip so a promoted heading inside one is still counted
    #[test]
    fn headings_and_tables_are_counted_by_element() {
        let r = scan_rendered("<h1>a</h1><h2 id=\"b\">b</h2>\n<table><tr><td>c</td></tr></table>");
        assert_eq!(r.headings, 2);
        assert_eq!(r.tables, 1);
        assert_eq!(scan_rendered("<h7>x</h7><hr>").headings, 0);
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — cmark's rules: ATX and setext, both
    // skipped inside a fenced block, and a seventh `#` is not a heading
    #[test]
    fn the_source_heading_scan_is_fence_aware_and_bounds_the_atx_run() {
        assert_eq!(count_source_headings("# one\n\n## two\n"), 2);
        assert_eq!(count_source_headings("```\n# fenced\n```\n"), 0);
        assert_eq!(count_source_headings("~~~\n# fenced\n~~~\n"), 0);
        assert_eq!(count_source_headings("####### seven\n"), 0);
        assert_eq!(count_source_headings("###### six\n"), 1);
        assert_eq!(count_source_headings("title\n=====\n"), 1);
        assert_eq!(count_source_headings("\n=====\n"), 0);
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — a table start is a pipe-carrying row
    // immediately followed by a delimiter row, outside code
    #[test]
    fn the_source_table_scan_needs_a_pipe_row_then_a_delimiter_row() {
        assert_eq!(count_source_tables("| a | b |\n| --- | --- |\n| 1 | 2 |\n"), 1);
        assert_eq!(count_source_tables("| a | b |\n\n| --- | --- |\n"), 0);
        assert_eq!(count_source_tables("```\n| a |\n| --- |\n```\n"), 0);
        assert_eq!(count_source_tables("a b\n| --- |\n"), 0);
        assert_eq!(count_source_tables("| a |\n| :-: |\n"), 1);
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — front matter is stripped so the gate
    // renders exactly the body kramdown sees, and only a *leading* `---` opens it
    #[test]
    fn front_matter_is_stripped_only_when_it_opens_the_page() {
        assert_eq!(strip_front_matter("---\ntitle: x\n---\nbody\n"), "body\n");
        assert_eq!(strip_front_matter("body\n---\nmore\n"), "body\n---\nmore\n");
        assert_eq!(strip_front_matter(""), "");
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — `NUL` is a *terminator* rather than a
    // separator, so N documents produce exactly N NULs and an unterminated tail is not a document
    #[test]
    fn only_a_nul_terminated_record_is_a_document() {
        assert_eq!(nul_records(b"a\0b\0").len(), 2);
        assert_eq!(nul_records(b"a\0b").len(), 1);
        assert_eq!(nul_records(b"").len(), 0);
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — the framing is unforgeable because the
    // gate's own reader drops any NUL in the page before framing happens
    #[test]
    fn a_page_carrying_a_nul_cannot_forge_a_frame() {
        assert_eq!(render_input("a\0b\n"), "ab\n");
        assert_eq!(render_input("a\n\n\n"), "a\n");
        assert_eq!(render_input(""), "\n");
    }

    // spec: site-kit/SPEC.md §check-docs-render-fidelity — every underscore-prefixed directory
    // segment excluded; the shell form brackets the path so the basename is a segment too
    #[test]
    fn every_underscore_prefixed_segment_is_a_jekyll_internal() {
        assert!(jekyll_internal("docs/_layouts/default.md"));
        assert!(jekyll_internal("docs/_x.md"));
        assert!(!jekyll_internal("docs/a_b/c.md"));
        assert!(!jekyll_internal("docs/install.md"));
    }
}
