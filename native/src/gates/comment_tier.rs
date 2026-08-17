// spec: canon-kit/SPEC.md §check-comment-tier — every full-line comment on a governed surface
// is a machine/reason directive, rides a directive's bounded window, is comment-tier-exempt,
// or justifies a positional construct
use crate::spec;
use std::path::Path;

// spec: canon-kit/SPEC.md §check-comment-tier — the built-in kit-mechanism roster (the
// directive names the kits themselves carry); the CANON_KIT_COMMENT_* knobs append a
// consumer's extras
const SHELL_COLON: &[&str] = &[
    "graph:",
    "spec:",
    "contract:",
    "usage:",
    "install:",
    "exception-list:",
    "no-fixture:",
    "no-port:",
    "permanent:",
    "smoke-unregistered:",
    "comment-tier-exempt:",
    "TODO(task:",
    "TODO(spec-ambiguity)",
];
const SHELL_WORD: &[&str] = &["shellcheck", "assertion"];
// spec: canon-kit/SPEC.md §check-comment-tier — the state-file surface blesses only
// contract:/see headers, plus the universal exempt escape
const TXT_COLON: &[&str] = &["contract:", "comment-tier-exempt:"];
const TXT_WORD: &[&str] = &["see"];
const EXEMPT: &str = "comment-tier-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-comment-tier: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-comment-tier — the blessing alternation as the two predicates
// it is built from: a colon directive is a literal substring, a bare word is one bounded by a
// non-alphanumeric on each side. No pattern is interpreted, so no ERE engine is owed.
struct Bless {
    colons: Vec<String>,
    words: Vec<String>,
}

impl Bless {
    fn new(colons: &[&str], words: &[&str], extra: &[String]) -> Bless {
        let mut c: Vec<String> = colons.iter().map(|s| s.to_string()).collect();
        c.extend(extra.iter().cloned());
        Bless {
            colons: c.into_iter().filter(|s| !s.is_empty()).collect(),
            words: words.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn blesses(&self, body: &str) -> bool {
        if self.colons.is_empty() && self.words.is_empty() {
            return true;
        }
        if self.colons.iter().any(|c| body.contains(c.as_str())) {
            return true;
        }
        self.words.iter().any(|w| word_present(body, w))
    }
}

fn word_present(body: &str, word: &str) -> bool {
    let b = body.as_bytes();
    let w = word.as_bytes();
    if w.is_empty() || w.len() > b.len() {
        return false;
    }
    (0..=b.len() - w.len()).any(|i| {
        &b[i..i + w.len()] == w
            && (i == 0 || !b[i - 1].is_ascii_alphanumeric())
            && (i + w.len() == b.len() || !b[i + w.len()].is_ascii_alphanumeric())
    })
}

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Hash,
    Slash,
}

struct Rec {
    line: usize,
    text: String,
    flag: bool,
    exempt: bool,
    cspan: String,
}

struct Walk<'a> {
    file: String,
    style: Style,
    mdhead: bool,
    bless: &'a Bless,
    pos: &'a [String],
    cap: usize,
    grammar: &'a spec::CountGrammar,
    block: Vec<Rec>,
    window: usize,
    xwindow: usize,
    hd: String,
    inblock: bool,
    out: Vec<String>,
}

impl<'a> Walk<'a> {
    // spec: canon-kit/SPEC.md §check-comment-tier — a directive opens a CAP-wide window
    // blessing its line plus continuations; the count-shape override rides the shared adapter
    fn record(&mut self, body: &str, fnr: usize) {
        if self.bless.blesses(body) {
            self.window = self.cap;
        }
        if body.contains(EXEMPT) {
            self.xwindow = self.cap;
        }
        let blessed = self.window > 0;
        let exempt = self.xwindow > 0;
        if self.window > 0 {
            self.window -= 1;
        }
        if self.xwindow > 0 {
            self.xwindow -= 1;
        }
        let cspan = if exempt {
            String::new()
        } else {
            self.grammar.hit(body).unwrap_or_default()
        };
        self.block.push(Rec {
            line: fnr,
            text: body.trim_matches(|c: char| c == ' ' || c == '\t').to_string(),
            flag: !blessed,
            exempt,
            cspan,
        });
    }

    fn flush(&mut self, rescue: bool) {
        if self.block.is_empty() {
            return;
        }
        if rescue {
            if let Some(last) = self.block.last_mut() {
                last.flag = false;
            }
        }
        for r in &self.block {
            if r.flag {
                self.out.push(format!("{}:{}: {}", self.file, r.line, r.text));
            } else if !r.cspan.is_empty() {
                self.out.push(format!(
                    "{}:{}: restated collection total: {}",
                    self.file, r.line, r.cspan
                ));
            }
        }
        // spec: canon-kit/SPEC.md §check-comment-tier — the count override across a comment
        // wrap: an exempt line ends the join window
        let mut para = spec::Para::default();
        for r in &self.block {
            if r.exempt {
                wrapflush(&self.file, self.grammar, &mut para, &mut self.out);
                continue;
            }
            para.add(r.line, &r.text);
        }
        wrapflush(&self.file, self.grammar, &mut para, &mut self.out);
        self.block.clear();
        self.window = 0;
        self.xwindow = 0;
    }

    fn noncomment(&mut self, line: &str) {
        let blank = line.trim_matches(|c: char| c == ' ' || c == '\t').is_empty();
        let ispos = !blank && self.pos.iter().any(|p| line.contains(p.as_str()));
        self.flush(ispos);
        if self.style == Style::Hash {
            self.detect_hd(line);
        }
    }

    // spec: canon-kit/SPEC.md §check-comment-tier — heredoc bodies are not comments: the
    // opener's terminator is read off the last `<<`, with `<<<` neutralised first
    fn detect_hd(&mut self, line: &str) {
        if !line.contains("<<") {
            return;
        }
        let s = line.replace("<<<", "\u{1}");
        let at = match s.rfind("<<") {
            Some(v) => v,
            None => return,
        };
        let b = s.as_bytes();
        let mut i = at + 2;
        if b.get(i) == Some(&b'-') {
            i += 1;
        }
        while i < b.len() && matches!(b[i], b' ' | b'\t') {
            i += 1;
        }
        if matches!(b.get(i), Some(&b'\'') | Some(&b'"')) {
            i += 1;
        }
        if i >= b.len() || !(b[i].is_ascii_alphabetic() || b[i] == b'_') {
            return;
        }
        let mut e = i;
        while e < b.len() && (b[e].is_ascii_alphanumeric() || b[e] == b'_') {
            e += 1;
        }
        self.hd = s[i..e].to_string();
    }

    fn line(&mut self, raw: &str, fnr: usize) {
        if self.style == Style::Hash {
            if !self.hd.is_empty() {
                if raw.trim_matches(|c: char| c == ' ' || c == '\t') == self.hd {
                    self.hd = String::new();
                }
                self.flush(false);
                return;
            }
            if fnr == 1 && raw.starts_with("#!") {
                self.noncomment(raw);
                return;
            }
            let lead = raw.trim_start_matches([' ', '\t']);
            if self.mdhead && lead.starts_with("##") {
                self.noncomment(raw);
                return;
            }
            if let Some(body) = lead.strip_prefix('#') {
                self.record(body, fnr);
            } else {
                self.noncomment(raw);
            }
            return;
        }
        if self.inblock {
            let mut b = raw.to_string();
            if let Some(p) = b.find("*/") {
                b.truncate(p);
                self.inblock = false;
            }
            let body = b
                .trim_start_matches([' ', '\t'])
                .strip_prefix('*')
                .unwrap_or_else(|| b.trim_start_matches([' ', '\t']))
                .to_string();
            self.record(&body, fnr);
            // spec: canon-kit/SPEC.md §check-comment-tier — a block closing with code after it
            // is a non-comment line for the positional rescue: the test is that *some* `*/` has
            // non-blank content behind it, and what the rescue reads is the tail of the last one
            if !self.inblock && closer_has_tail(raw) {
                let p = raw.rfind("*/").expect("a closer was just detected");
                let tail = raw[p + 2..].to_string();
                self.noncomment(&tail);
            }
            return;
        }
        let lead = raw.trim_start_matches([' ', '\t']);
        if lead.starts_with("//") {
            let body = lead.trim_start_matches('/');
            self.record(body, fnr);
            return;
        }
        if let Some(rest) = lead.strip_prefix("/*") {
            let mut b = rest.to_string();
            match b.find("*/") {
                Some(p) => b.truncate(p),
                None => self.inblock = true,
            }
            self.record(&b, fnr);
            return;
        }
        self.noncomment(raw);
    }
}

fn closer_has_tail(raw: &str) -> bool {
    let b = raw.as_bytes();
    (0..b.len().saturating_sub(1)).any(|i| {
        if &b[i..i + 2] != b"*/" {
            return false;
        }
        let mut j = i + 2;
        while j < b.len() && matches!(b[j], b' ' | b'\t') {
            j += 1;
        }
        j < b.len()
    })
}

fn wrapflush(
    file: &str,
    grammar: &spec::CountGrammar,
    para: &mut spec::Para,
    out: &mut Vec<String>,
) {
    if let Some((fnr, span)) = spec::para_wrapped(grammar, para) {
        out.push(format!(
            "{}:{}: restated collection total: {}",
            file, fnr, span
        ));
    }
    para.reset();
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let machine = spec::knob_array_pub("CANON_KIT_COMMENT_MACHINE")?;
    let reason = spec::knob_array_pub("CANON_KIT_COMMENT_REASON")?;
    let mut extra = machine;
    extra.extend(reason);
    let shell_bless = Bless::new(SHELL_COLON, SHELL_WORD, &extra);
    let txt_bless = Bless::new(TXT_COLON, TXT_WORD, &extra);
    let pos: Vec<String> = spec::knob_array_pub("CANON_KIT_COMMENT_POSITIONAL")?
        .into_iter()
        .filter(|p| !p.is_empty())
        .collect();
    let whitelist = spec::knob_array_pub("CANON_KIT_COMMENT_WHITELIST")?;
    let cap: usize = spec::knob_pub("CANON_KIT_COMMENT_RUN_CAP")?
        .parse()
        .map_err(|_| "CANON_KIT_COMMENT_RUN_CAP is not a number".to_string())?;
    let grammar = spec::CountGrammar::resolve()?;
    let wf = spec::knob_pub("GATE_SDK_WORKFLOW_DIR")?;
    let no_pos: Vec<String> = Vec::new();

    // spec: canon-kit/SPEC.md §check-comment-tier — the governed surface via the shared
    // primitive; the with-templates variant adds the templates/ sources this gate governs
    // where check-spec-pointer exempts them
    let surface = spec::comment_surface(root, true)?;
    let mut errors: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    for f in &surface {
        let rel = spec::strip_dot_slash(f.strip_prefix(&format!("{}/", root)).unwrap_or(f));
        if spec::comment_whitelisted(&rel, &whitelist) {
            continue;
        }
        let (style, bless, linepos, mdhead) = classify(&rel, &wf, &shell_bless, &txt_bless);
        scanned += 1;
        let mut w = Walk {
            file: rel.clone(),
            style,
            mdhead,
            bless,
            pos: if linepos { &pos } else { &no_pos },
            cap,
            grammar: &grammar,
            block: Vec::new(),
            window: 0,
            xwindow: 0,
            hd: String::new(),
            inblock: false,
            out: Vec::new(),
        };
        let text = spec::read_text(Path::new(f))?;
        for (idx, raw) in text.lines().enumerate() {
            w.line(raw, idx + 1);
        }
        w.flush(false);
        errors.extend(w.out);
    }

    if !errors.is_empty() {
        println!("COMMENT-TIER: {} violation(s):", errors.len());
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: code is the WHAT, its SPEC the WHY — lead the block with a bare 'spec: <SPEC> §<section>' pointer (or another roster directive), delete prose that restates the code or paraphrases the SPEC it points at, or tag '# comment-tier-exempt: <reason>' for a genuinely-local fact below SPEC altitude. A directive blesses only its own window (its line plus continuations up to CANON_KIT_COMMENT_RUN_CAP={} physical comment lines, blank '#' lines included); prose beyond the window re-anchors on its own directive, trims, or exempts. A 'restated collection total' is flagged inside a directive window too — a directive's blessing covers its own wording, never a pinned total: delete the count or cite the owning collection. Not-yet-swept components ride the COMMENT_TIER_WHITELIST with a '# until:' drain task.", cap);
        return Ok(1);
    }
    println!(
        "COMMENT-TIER: clean ({} governed source(s); every full-line comment is a directive, rides a directive window, is exempt, or justifies a positional construct)",
        scanned
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-comment-tier — the per-surface comment style, blessing roster
// and positional allowance; a workflow-dir markdown member's '##' headings are that surface's
// own block grammar rather than comments
fn classify<'a>(
    rel: &str,
    wf: &str,
    shell: &'a Bless,
    txt: &'a Bless,
) -> (Style, &'a Bless, bool, bool) {
    let wfpfx = format!("{}/", wf);
    if rel.starts_with(&wfpfx) {
        return (Style::Hash, txt, false, rel.ends_with(".md"));
    }
    if rel.ends_with(".sh") || rel.ends_with(".bash") {
        return (Style::Hash, shell, true, false);
    }
    if rel.ends_with(".txt") {
        return (Style::Hash, txt, false, false);
    }
    for ext in [".rs", ".ts", ".tsx", ".js", ".go", ".c", ".h", ".rego"] {
        if rel.ends_with(ext) {
            return (Style::Slash, shell, true, false);
        }
    }
    (Style::Hash, shell, true, false)
}
