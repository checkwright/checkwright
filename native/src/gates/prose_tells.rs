// spec: canon-kit/SPEC.md §check-prose-tells — the mechanical AI-prose tells over the
// consumer-configured prose surfaces, each threshold-gated, with the prose-tell-exempt valve
use crate::ere::Ere;
use crate::spec;
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-prose-tells: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-prose-tells — every threshold is validated by the kit's knob
// table (§Layout and configuration) before this gate ever reads it, so a numeric parse here
// cannot fail — a failure is a bug in that validator, not a malformed consumer value
fn numeric(s: &str) -> Result<f64, String> {
    s.parse::<f64>()
        .map_err(|e| format!("{} is not numeric despite table validation: {}", s, e))
}

fn abbr_min(s: &str) -> Result<Option<usize>, String> {
    if s == "off" {
        return Ok(None);
    }
    s.parse::<usize>()
        .map(Some)
        .map_err(|e| format!("{} is not an integer despite table validation: {}", s, e))
}

struct Thresholds {
    emdash_max: f64,
    emdash_max_raw: String,
    contrast_max: f64,
    contrast_max_raw: String,
    rhythm_min: f64,
    rhythm_cv_min: f64,
    tricolon_max: f64,
    tricolon_max_raw: String,
    abbr_min: Option<usize>,
}

// spec: canon-kit/SPEC.md §check-prose-tells — three non-prose surfaces are held out of what
// the tells measure: inline code spans, markdown table rows, and generated regions
fn strip_code(s: &str) -> String {
    let b = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'`' {
            if let Some(off) = b[i + 1..].iter().position(|&c| c == b'`') {
                out.push(b' ');
                i = i + 1 + off + 1;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn lstrip(s: &str) -> &str {
    s.trim_start_matches([' ', '\t'])
}

fn is_table(s: &str) -> bool {
    lstrip(s).starts_with('|')
}

// spec: canon-kit/SPEC.md §check-prose-tells — a markdown list item is its own unit rather than
// flowing prose, so the paragraph and section assertions never lump two items into one span
fn is_list_item(s: &str) -> bool {
    let t = lstrip(s).as_bytes();
    if t.is_empty() {
        return false;
    }
    if matches!(t[0], b'-' | b'*' | b'+') {
        return t.len() > 1 && (t[1] == b' ' || t[1] == b'\t');
    }
    let mut i = 0usize;
    while i < t.len() && t[i].is_ascii_digit() {
        i += 1;
    }
    i > 0 && i + 1 < t.len() && t[i] == b'.' && (t[i + 1] == b' ' || t[i + 1] == b'\t')
}

fn heading_level(s: &str) -> usize {
    let t = lstrip(s).as_bytes();
    let mut i = 0usize;
    while i < t.len() && t[i] == b'#' {
        i += 1;
    }
    if i == 0 || i > 6 || i >= t.len() || !(t[i] == b' ' || t[i] == b'\t') {
        return 0;
    }
    i
}

// spec: gate-sdk/SPEC.md §lib/inject.sh — a marker is the line's whole trimmed content; prose naming one opens nothing.
fn gen_marker(s: &str, suffix: &str) -> bool {
    let inner = match s
        .trim()
        .strip_prefix("<!--")
        .and_then(|t| t.strip_suffix("-->"))
    {
        Some(t) => t.trim(),
        None => return false,
    };
    match inner.strip_suffix(suffix) {
        Some(name) => {
            !name.is_empty()
                && name
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
        }
        None => false,
    }
}

fn count_matches(re: &Ere, hay: &str) -> usize {
    let mut n = 0usize;
    let mut pos = 0usize;
    while pos <= hay.len() {
        let rest = match hay.get(pos..) {
            Some(r) => r,
            None => {
                pos += 1;
                continue;
            }
        };
        match re.find(rest) {
            Some((s, e)) => {
                n += 1;
                pos += if e > s { e } else { s + 1 };
            }
            None => break,
        }
    }
    n
}

// spec: canon-kit/SPEC.md §check-prose-tells — assertion E's sentence split: a run of terminal
// punctuation followed by whitespace, or ending the span
fn split_sentences(p: &str) -> Vec<usize> {
    let b = p.as_bytes();
    let mut segs: Vec<String> = Vec::new();
    let mut cur: Vec<u8> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if matches!(b[i], b'.' | b'!' | b'?') {
            let mut j = i;
            while j < b.len() && matches!(b[j], b'.' | b'!' | b'?') {
                j += 1;
            }
            if j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
                segs.push(String::from_utf8_lossy(&cur).into_owned());
                cur.clear();
                i = j + 1;
                continue;
            }
            let mut k = j;
            while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                k += 1;
            }
            if k == b.len() {
                segs.push(String::from_utf8_lossy(&cur).into_owned());
                cur.clear();
                i = b.len();
                continue;
            }
        }
        cur.push(b[i]);
        i += 1;
    }
    segs.push(String::from_utf8_lossy(&cur).into_owned());
    let mut out = Vec::new();
    for seg in segs {
        let t = seg.trim_matches([' ', '\t']);
        if t.is_empty() {
            continue;
        }
        out.push(t.split([' ', '\t']).filter(|w| !w.is_empty()).count().max(1));
    }
    out
}

struct Sink<'a> {
    th: &'a Thresholds,
    contrast_re: &'a Ere,
    tricolon_re: &'a Ere,
    phrases: &'a [(String, String)],
    allow: &'a [String],
    out: Vec<String>,
    sec_startline: usize,
    sec_buf: String,
    file_buf: String,
    in_gen: bool,
}

impl Sink<'_> {
    fn emit(&mut self, file: &str, ln: usize, code: &str, msg: &str) {
        self.out.push(format!("{}:{}: [{}] {}", file, ln, code, msg));
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — assertion A, em-dash density over a paragraph
    fn a_emdash(&mut self, file: &str, para: &str, ln: usize) {
        let n = para.matches('—').count();
        if n as f64 > self.th.emdash_max {
            let m = format!(
                "em-dash density ({} em-dashes > {}) in a paragraph",
                n, self.th.emdash_max_raw
            );
            self.emit(file, ln, "A", &m);
        }
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — assertion B, any bundled or consumer-supplied
    // throat-clearing phrase, case-insensitive
    fn b_phrases(&mut self, file: &str, para: &str, ln: usize) {
        let lc = para.to_ascii_lowercase();
        let hits: Vec<String> = self
            .phrases
            .iter()
            .filter(|(_, low)| lc.contains(low.as_str()))
            .map(|(orig, _)| format!("throat-clearing phrase \"{}\"", orig))
            .collect();
        for m in hits {
            self.emit(file, ln, "B", &m);
        }
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — assertion E, sentence-rhythm variance
    fn e_rhythm(&mut self, file: &str, para: &str, ln: usize) {
        let words = split_sentences(para);
        let ns = words.len();
        if (ns as f64) < self.th.rhythm_min {
            return;
        }
        let sum: usize = words.iter().sum();
        let mean = sum as f64 / ns as f64;
        if mean <= 0.0 {
            return;
        }
        let sq: f64 = words
            .iter()
            .map(|w| (*w as f64 - mean) * (*w as f64 - mean))
            .sum();
        let cv = (sq / ns as f64).sqrt() / mean;
        if cv < self.th.rhythm_cv_min {
            let m = format!(
                "metronomic rhythm ({} sentences, word-count CV {:.3} < {:.2})",
                ns, cv, self.th.rhythm_cv_min
            );
            self.emit(file, ln, "E", &m);
        }
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — assertions C and F, flushed at each level-1/2
    // heading and at the end of the file
    fn flush_section(&mut self, file: &str) {
        if self.sec_buf.is_empty() {
            return;
        }
        let contrast_re = self.contrast_re;
        let tricolon_re = self.tricolon_re;
        let lc = self.sec_buf.to_ascii_lowercase();
        let n = count_matches(contrast_re, &lc);
        if n as f64 > self.th.contrast_max {
            let m = format!(
                "contrast cadence ({} \"not X, it's Y\" turns > {}) in a section",
                n, self.th.contrast_max_raw
            );
            let at = self.sec_startline;
            self.emit(file, at, "C", &m);
        }
        let sec = self.sec_buf.clone();
        let n = count_matches(tricolon_re, &sec);
        if n as f64 > self.th.tricolon_max {
            let m = format!(
                "tricolon density ({} \"A, B, and C\" triples > {}) in a section",
                n, self.th.tricolon_max_raw
            );
            let at = self.sec_startline;
            self.emit(file, at, "F", &m);
        }
        self.sec_buf.clear();
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — assertion D, an all-caps token never expanded
    // anywhere in the file and absent from the allow-list
    fn flush_file(&mut self, file: &str) {
        // spec: canon-kit/SPEC.md §check-prose-tells — `off` skips assertion D alone
        let Some(min) = self.th.abbr_min else { return };
        let allow = self.allow.to_vec();
        let buf = self.file_buf.clone();
        let b = buf.as_bytes();
        let mut seen: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < b.len() {
            if !b[i].is_ascii_uppercase() {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j < b.len() && (b[j].is_ascii_uppercase() || b[j].is_ascii_digit()) {
                j += 1;
            }
            if j - i < min {
                i += 1;
                continue;
            }
            let before = if i > 0 { b[i - 1] } else { b' ' };
            let tok = String::from_utf8_lossy(&b[i..j]).into_owned();
            i = j;
            if before.is_ascii_alphanumeric() {
                continue;
            }
            if seen.contains(&tok) {
                continue;
            }
            seen.push(tok.clone());
            if allow.contains(&tok) {
                continue;
            }
            if is_defined(&buf, &tok) {
                continue;
            }
            let m = format!(
                "undefined abbreviation \"{}\" (never expanded in-file, not in the allow-list)",
                tok
            );
            self.emit(file, 1, "D", &m);
        }
    }
}

// spec: canon-kit/SPEC.md §check-prose-tells — an expansion is either a parenthesized gloss
// opening with the token, or the token immediately preceding an opening parenthesis
fn is_defined(buf: &str, tok: &str) -> bool {
    if buf.contains(&format!("({}", tok)) {
        return true;
    }
    let b = buf.as_bytes();
    let t = tok.as_bytes();
    let mut i = 0usize;
    while i + t.len() <= b.len() {
        match b[i..].windows(t.len()).position(|w| w == t) {
            Some(off) => {
                let mut j = i + off + t.len();
                while j < b.len() && b[j] == b' ' {
                    j += 1;
                }
                if j < b.len() && b[j] == b'(' {
                    return true;
                }
                i = i + off + 1;
            }
            None => return false,
        }
    }
    false
}

impl spec::ProseSink for Sink<'_> {
    // spec: canon-kit/SPEC.md §check-prose-tells — a heading is never prose: a level-1/2 head
    // closes the section span, and any head stays out of the paragraph and file buffers
    fn on_line(&mut self, file: &str, fnr: usize, raw: &str) {
        let s = self;
        if gen_marker(raw, ":begin") {
            s.in_gen = true;
            return;
        }
        if gen_marker(raw, ":end") {
            s.in_gen = false;
            return;
        }
        if s.in_gen {
            return;
        }
        let lvl = heading_level(raw);
        if lvl > 0 {
            if lvl <= 2 {
                let f = file.to_string();
                s.flush_section(&f);
                s.sec_startline = fnr;
            }
            return;
        }
        if is_table(raw) {
            return;
        }
        let clean = strip_code(raw);
        if is_list_item(raw) {
            s.sec_buf.push_str(" .");
            s.file_buf.push_str(" .");
        }
        s.sec_buf.push(' ');
        s.sec_buf.push_str(&clean);
        s.file_buf.push(' ');
        s.file_buf.push_str(&clean);
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — each blank-line paragraph is unitized: the run
    // before the first list item, and every list item with its continuations, is its own A/B/E unit
    fn on_pflush(&mut self, file: &str, para: &spec::Para) {
        let n = para.len();
        if n == 0 || self.in_gen {
            return;
        }
        let first = &para.line[0];
        if heading_level(first) > 0 || is_table(first) {
            return;
        }
        let mut ustart = 1usize;
        let mut units: Vec<(usize, usize)> = Vec::new();
        for i in 2..=n {
            if is_list_item(&para.line[i - 1]) {
                units.push((ustart, i - 1));
                ustart = i;
            }
        }
        units.push((ustart, n));
        for (lo, hi) in units {
            let text = strip_code(&para.join(lo, hi));
            let ln = para.fnr[lo - 1];
            self.a_emdash(file, &text, ln);
            self.b_phrases(file, &text, ln);
            self.e_rhythm(file, &text, ln);
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — the scanned surfaces are the consumer's
    // glob set; empty means nothing scanned, the unconfigured-consumer no-op
    let globs = walk::knob_array("CANON_KIT_PROSE_TELL_GLOBS")?;
    let mut files: Vec<String> = Vec::new();
    if !globs.is_empty() {
        for p in walk::glob_corpus(Path::new(root), &globs)? {
            files.push(p.display().to_string());
        }
    }
    if files.is_empty() {
        println!("PROSE-TELLS: clean (0 configured surface(s); nothing scanned)");
        return Ok(0);
    }
    files.sort();
    files.dedup();

    let phrases: Vec<(String, String)> = crate::spec::vocabulary("CANON_KIT_PROSE_TELL_PHRASES", "CANON_KIT_PROSE_TELL_PHRASES_EXTRA")?
        .into_iter()
        .filter(|p| !p.is_empty())
        .map(|p| {
            let low = p.to_ascii_lowercase();
            (p, low)
        })
        .collect();
    let allow: Vec<String> = crate::spec::vocabulary("CANON_KIT_PROSE_TELL_ABBR_ALLOW", "CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA")?
        .into_iter()
        .filter(|a| !a.is_empty())
        .collect();

    let emdash_max_raw = walk::knob_scalar("CANON_KIT_PROSE_TELL_EMDASH_MAX")?;
    let contrast_max_raw = walk::knob_scalar("CANON_KIT_PROSE_TELL_CONTRAST_MAX")?;
    let tricolon_max_raw = walk::knob_scalar("CANON_KIT_PROSE_TELL_TRICOLON_MAX")?;
    let th = Thresholds {
        emdash_max: numeric(&emdash_max_raw)?,
        emdash_max_raw,
        contrast_max: numeric(&contrast_max_raw)?,
        contrast_max_raw,
        rhythm_min: numeric(&walk::knob_scalar("CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES")?)?,
        rhythm_cv_min: numeric(&walk::knob_scalar("CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN")?)?,
        tricolon_max: numeric(&tricolon_max_raw)?,
        tricolon_max_raw,
        abbr_min: abbr_min(&walk::knob_scalar("CANON_KIT_PROSE_TELL_ABBR_MIN_LEN")?)?,
    };

    let contrast_re = Ere::compile("not[^.]*(—|, but)[^.]*it('s| is)")
        .map_err(|e| format!("the contrast-cadence expression failed to compile: {}", e))?;
    let tricolon_re = Ere::compile("[A-Za-z][^,.]*, [^,.]*, and [^,.]*[A-Za-z]")
        .map_err(|e| format!("the tricolon expression failed to compile: {}", e))?;

    let mut findings: Vec<String> = Vec::new();
    // spec: canon-kit/SPEC.md §check-prose-tells — the walk runs per file, because assertion D
    // is an in-file assertion and a shared walk would pool every file's tokens into one buffer
    for f in &files {
        let mut sink = Sink {
            th: &th,
            contrast_re: &contrast_re,
            tricolon_re: &tricolon_re,
            phrases: &phrases,
            allow: &allow,
            out: Vec::new(),
            sec_startline: 1,
            sec_buf: String::new(),
            file_buf: String::new(),
            in_gen: false,
        };
        spec::walk_prose(std::slice::from_ref(f), "prose-tell-exempt:", &mut sink)?;
        sink.flush_section(f);
        sink.flush_file(f);
        findings.extend(sink.out);
    }

    findings.retain(|l| !l.trim().is_empty());
    findings.sort();
    findings.dedup();
    if !findings.is_empty() {
        println!(
            "check-prose-tells: {} mechanical AI-prose tell(s) across {} configured surface(s):",
            findings.len(),
            files.len()
        );
        for l in &findings {
            println!("{}", l);
        }
        println!("  help: rewrite the flagged prose — break the em-dash-dense paragraph, cut the throat-clearing opener, vary sentence length, spell out the abbreviation once, thin the contrast/tricolon cadence — or, for a deliberate keep, tag '<!-- prose-tell-exempt: <reason> -->' on the flagged line or directly above it (a reason is mandatory). Thresholds are the CANON_KIT_PROSE_TELL_* knobs (canon-kit/SPEC.md §Layout and configuration).");
        return Ok(1);
    }
    println!(
        "PROSE-TELLS: clean ({} configured surface(s); no mechanical AI-prose tell tripped)",
        files.len()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn abbreviations(buf: &str, min: &str) -> Vec<String> {
        let th = Thresholds {
            emdash_max: 0.0,
            emdash_max_raw: String::new(),
            contrast_max: 0.0,
            contrast_max_raw: String::new(),
            rhythm_min: 0.0,
            rhythm_cv_min: 0.0,
            tricolon_max: 0.0,
            tricolon_max_raw: String::new(),
            abbr_min: abbr_min(min).expect("a validated value"),
        };
        let re = Ere::compile("x").expect("compiles");
        let mut sink = Sink {
            th: &th,
            contrast_re: &re,
            tricolon_re: &re,
            phrases: &[],
            allow: &[],
            out: Vec::new(),
            sec_startline: 1,
            sec_buf: String::new(),
            file_buf: buf.to_string(),
            in_gen: false,
        };
        sink.flush_file("f.md");
        sink.out
            .iter()
            .map(|l| l.split('"').nth(1).unwrap_or("").to_string())
            .collect()
    }

    // spec: canon-kit/SPEC.md §check-prose-tells — assertion D reads all-caps tokens at least
    // CANON_KIT_PROSE_TELL_ABBR_MIN_LEN long, and `off` skips it
    #[test]
    fn the_abbreviation_floor_is_the_knob_and_off_skips_the_assertion() {
        let buf = " An OK run, a TLA, and a K8S node. ";
        assert_eq!(abbreviations(buf, "2"), vec!["OK", "TLA", "K8S"]);
        assert_eq!(abbreviations(buf, "3"), vec!["TLA", "K8S"]);
        assert!(abbreviations(buf, "off").is_empty());
    }
}
