// spec: canon-kit/SPEC.md §check-prose-bounds — governed prose keeps its sentences and units
// within a bound and states a repeated rule once, under an optional per-file ceiling
use crate::spec;
use crate::walk;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-prose-bounds: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-prose-bounds — an inline code span is one word, spelled as a
// byte no prose carries so neither the sentence split nor the phrase walk reads inside it
const CODE: u8 = 0x01;

// spec: canon-kit/SPEC.md §Layout and configuration — every bound is validated by the kit's knob
// table before this gate reads it, so a parse failure here is a validator bug
fn bound(name: &str) -> Result<Option<usize>, String> {
    let v = walk::knob_scalar(name)?;
    if v == "off" {
        return Ok(None);
    }
    v.parse::<usize>()
        .map(Some)
        .map_err(|e| format!("{} is '{}', not an integer despite table validation: {}", name, v, e))
}

struct Bounds {
    sentence_max: Option<usize>,
    paragraph_max: Option<usize>,
    repeat_words: Option<usize>,
    repeat_min: usize,
}

struct Unit {
    fnr: usize,
    text: String,
    line_of: Vec<usize>,
}

impl Unit {
    fn line_at(&self, byte: usize) -> usize {
        self.line_of.get(byte).or(self.line_of.last()).copied().unwrap_or(self.fnr)
    }
}

// spec: canon-kit/SPEC.md §check-prose-bounds — the unit's lines rejoined with one space, each byte
// mapped to its physical line, and each inline code span collapsed to one word
fn build_unit(lines: &[(usize, String)]) -> Unit {
    let mut joined: Vec<u8> = Vec::new();
    let mut lines_of: Vec<usize> = Vec::new();
    for (k, (fnr, l)) in lines.iter().enumerate() {
        if k > 0 {
            joined.push(b' ');
            lines_of.push(*fnr);
        }
        joined.extend_from_slice(l.as_bytes());
        lines_of.extend(std::iter::repeat(*fnr).take(l.len()));
    }
    let mut out: Vec<u8> = Vec::with_capacity(joined.len());
    let mut line_of: Vec<usize> = Vec::with_capacity(joined.len());
    let mut i = 0usize;
    while i < joined.len() {
        if joined[i] == b'`' {
            if let Some(off) = joined[i + 1..].iter().position(|&c| c == b'`') {
                out.push(CODE);
                line_of.push(lines_of[i]);
                i = i + 1 + off + 1;
                continue;
            }
        }
        out.push(joined[i]);
        line_of.push(lines_of[i]);
        i += 1;
    }
    Unit {
        fnr: lines.first().map(|l| l.0).unwrap_or(1),
        text: String::from_utf8_lossy(&out).into_owned(),
        line_of,
    }
}

// spec: canon-kit/SPEC.md §check-prose-bounds — a link target is dropped with the rest of the
// non-prose, so a shared citation does not count as a repeated phrase
fn drop_link_targets(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(at) = rest.find("](") {
        out.push_str(&rest[..at + 1]);
        let after = &rest[at + 2..];
        match after.find(')') {
            Some(close) => rest = &after[close + 1..],
            None => {
                rest = after;
                break;
            }
        }
    }
    out.push_str(rest);
    out
}

// spec: canon-kit/SPEC.md §check-prose-bounds — a token carrying a separator, or a dot between two
// alphanumerics, names a path or a file rather than a word of prose
fn path_shaped(tok: &str) -> bool {
    if tok.contains('/') || tok.contains('\\') {
        return true;
    }
    let b = tok.as_bytes();
    (1..b.len().saturating_sub(1))
        .any(|i| b[i] == b'.' && b[i - 1].is_ascii_alphanumeric() && b[i + 1].is_ascii_alphanumeric())
}

// spec: canon-kit/SPEC.md §check-prose-bounds — assertion C's word stream: case folded, and code
// spans, link targets, path-shaped tokens and punctuation dropped first
fn phrase_words(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for tok in drop_link_targets(text).split_whitespace() {
        if tok.as_bytes().contains(&CODE) || path_shaped(tok) {
            continue;
        }
        let kept: String = tok
            .chars()
            .filter(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '\''))
            .collect();
        let kept = kept.trim_matches(|c| matches!(c, '-' | '_' | '\''));
        if kept.is_empty() {
            continue;
        }
        out.push(kept.to_lowercase());
    }
    out
}

#[derive(Default)]
struct Sink {
    held: HashSet<usize>,
    in_gen: bool,
    headings: Vec<(usize, String)>,
    units: Vec<Unit>,
}

// spec: canon-kit/SPEC.md §check-prose-bounds — a comment-only line is markup, not prose
fn comment_only(s: &str) -> bool {
    let t = s.trim();
    t.starts_with("<!--") && t.ends_with("-->")
}

impl spec::ProseSink for Sink {
    // spec: canon-kit/SPEC.md §check-prose-bounds — headings, table rows, generated regions and
    // comment-only lines are held out; a heading also opens the next section
    fn on_line(&mut self, _file: &str, fnr: usize, raw: &str) {
        if spec::is_gen_marker(raw, ":begin") {
            self.in_gen = true;
            self.held.insert(fnr);
            return;
        }
        if spec::is_gen_marker(raw, ":end") {
            self.in_gen = false;
            self.held.insert(fnr);
            return;
        }
        if self.in_gen {
            self.held.insert(fnr);
            return;
        }
        if spec::prose_heading_level(raw) > 0 {
            let text = raw.trim().trim_start_matches('#').trim().to_string();
            self.headings.push((fnr, text));
            self.held.insert(fnr);
            return;
        }
        if spec::is_table_row(raw) || comment_only(raw) {
            self.held.insert(fnr);
        }
    }

    // spec: canon-kit/SPEC.md §check-prose-bounds — each list item and its continuation lines is a
    // unit of its own, and a held-out line closes the unit it interrupts
    fn on_pflush(&mut self, _file: &str, para: &spec::Para) {
        let mut cur: Vec<(usize, String)> = Vec::new();
        for (k, line) in para.line.iter().enumerate() {
            let fnr = para.fnr[k];
            if self.held.contains(&fnr) {
                if !cur.is_empty() {
                    self.units.push(build_unit(&cur));
                    cur.clear();
                }
                continue;
            }
            if spec::is_list_item(line) && !cur.is_empty() {
                self.units.push(build_unit(&cur));
                cur.clear();
            }
            cur.push((fnr, line.clone()));
        }
        if !cur.is_empty() {
            self.units.push(build_unit(&cur));
        }
    }
}

struct Finding {
    line: usize,
    text: String,
}

impl Sink {
    fn section_of(&self, fnr: usize) -> usize {
        self.headings.iter().filter(|h| h.0 < fnr).count()
    }

    fn section_name(&self, idx: usize) -> String {
        match idx {
            0 => "before the first heading".to_string(),
            n => format!("§{}", self.headings[n - 1].1),
        }
    }

    fn judge(&self, b: &Bounds) -> Vec<Finding> {
        let mut out: Vec<Finding> = Vec::new();
        for u in &self.units {
            let sec = self.section_name(self.section_of(u.fnr));
            // spec: canon-kit/SPEC.md §check-prose-bounds — assertion A, per sentence
            if let Some(max) = b.sentence_max {
                for (s, e) in spec::sentence_spans(&u.text) {
                    let n = spec::word_count(&u.text[s..e]);
                    if n > max {
                        out.push(Finding {
                            line: u.line_at(s),
                            text: format!("[A] sentence of {} words > {} ({})", n, max, sec),
                        });
                    }
                }
            }
            // spec: canon-kit/SPEC.md §check-prose-bounds — assertion B, per unit
            if let Some(max) = b.paragraph_max {
                let n = spec::word_count(&u.text);
                if n > max {
                    out.push(Finding {
                        line: u.fnr,
                        text: format!("[B] unit of {} words > {} ({})", n, max, sec),
                    });
                }
            }
        }
        if let Some(w) = b.repeat_words {
            out.extend(self.repeats(w, b.repeat_min));
        }
        out.sort_by_key(|f| f.line);
        out
    }

    // spec: canon-kit/SPEC.md §check-prose-bounds — assertion C, one finding per section at the
    // first unit carrying a phrase recurring in at least the floor's count of its units
    fn repeats(&self, w: usize, min: usize) -> Vec<Finding> {
        let mut by_section: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (i, u) in self.units.iter().enumerate() {
            by_section.entry(self.section_of(u.fnr)).or_default().push(i);
        }
        let mut out = Vec::new();
        for (sec, members) in by_section {
            if members.len() < min {
                continue;
            }
            let mut seen: BTreeMap<String, Vec<usize>> = BTreeMap::new();
            let mut order: Vec<String> = Vec::new();
            for &ui in &members {
                let words = phrase_words(&self.units[ui].text);
                if words.len() < w {
                    continue;
                }
                for k in 0..=words.len() - w {
                    let key = words[k..k + w].join(" ");
                    let at = seen.entry(key.clone()).or_default();
                    if at.last() != Some(&ui) {
                        if at.is_empty() {
                            order.push(key);
                        }
                        at.push(ui);
                    }
                }
            }
            let hot: Vec<&String> = order.iter().filter(|k| seen[*k].len() >= min).collect();
            if hot.is_empty() {
                continue;
            }
            let first = hot.iter().map(|k| seen[*k][0]).min().unwrap_or(members[0]);
            let merged = merge_overlaps(&hot);
            let shown: Vec<String> = merged.iter().take(3).map(|k| format!("\"{}\"", k)).collect();
            let more = if merged.len() > 3 { format!(" (+{} more)", merged.len() - 3) } else { String::new() };
            out.push(Finding {
                line: self.units[first].fnr,
                text: format!(
                    "[C] a phrase of {} or more words recurs in {} or more units: {}{} ({})",
                    w,
                    min,
                    shown.join(", "),
                    more,
                    self.section_name(sec)
                ),
            });
        }
        out
    }
}

// spec: canon-kit/SPEC.md §check-prose-bounds — C lists the phrases, so a run of overlapping
// windows over one longer phrase is shown as that phrase
fn merge_overlaps(keys: &[&String]) -> Vec<String> {
    let mut out: Vec<Vec<&str>> = Vec::new();
    for k in keys {
        let words: Vec<&str> = k.split(' ').collect();
        if let Some(last) = out.last_mut() {
            if last.len() >= words.len() && last[last.len() + 1 - words.len()..] == words[..words.len() - 1] {
                last.push(words[words.len() - 1]);
                continue;
            }
        }
        out.push(words);
    }
    out.into_iter().map(|w| w.join(" ")).collect()
}

// spec: canon-kit/SPEC.md §check-prose-bounds — the ceiling: a `# contract:` header, then one
// `<n> <path>` row per governed file with findings; a malformed row or a repeated path refuses
fn ceiling(path: &str) -> Result<BTreeMap<String, usize>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("CANON_KIT_PROSE_BOUND_CEILING_FILE names '{}', which cannot be read: {} — treating as failure (not clean)", path, e))?;
    let mut rows: BTreeMap<String, usize> = BTreeMap::new();
    for (i, raw) in text.lines().enumerate() {
        let l = raw.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let parsed = l.split_once(' ').and_then(|(n, p)| {
            let p = p.trim();
            match n.parse::<usize>() {
                Ok(v) if v > 0 && n.bytes().all(|c| c.is_ascii_digit()) && !p.is_empty() && !p.contains(' ') => {
                    Some((v, spec::strip_dot_slash(p)))
                }
                _ => None,
            }
        });
        let Some((n, p)) = parsed else {
            return Err(format!("{}:{}: '{}' is not a '<n> <path>' row with a positive count — treating as failure (not clean)", path, i + 1, l));
        };
        if rows.insert(p.clone(), n).is_some() {
            return Err(format!("{}:{}: a second row for '{}' — treating as failure (not clean)", path, i + 1, p));
        }
    }
    Ok(rows)
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let b = Bounds {
        sentence_max: bound("CANON_KIT_PROSE_BOUND_SENTENCE_MAX")?,
        paragraph_max: bound("CANON_KIT_PROSE_BOUND_PARAGRAPH_MAX")?,
        repeat_words: bound("CANON_KIT_PROSE_BOUND_REPEAT_WORDS")?,
        repeat_min: bound("CANON_KIT_PROSE_BOUND_REPEAT_MIN")?.unwrap_or(usize::MAX),
    };
    let ceiling_file = walk::knob_scalar("CANON_KIT_PROSE_BOUND_CEILING_FILE")?;
    let rows = if ceiling_file.is_empty() { None } else { Some(ceiling(&ceiling_file)?) };

    let globs: Vec<String> = walk::knob_array("CANON_KIT_PROSE_BOUND_GLOBS")?
        .into_iter()
        .filter(|g| !g.is_empty())
        .collect();
    let mut files: Vec<(String, String)> = Vec::new();
    if !globs.is_empty() {
        for p in walk::glob_corpus(Path::new(root), &globs)? {
            let full = p.display().to_string();
            let rel = match walk::rel_under(root, &full) {
                Some(r) => r.to_string(),
                None => spec::strip_dot_slash(&full),
            };
            files.push((full, rel));
        }
    }
    files.sort_by(|a, b| a.1.cmp(&b.1));
    files.dedup_by(|a, b| a.1 == b.1);
    if files.is_empty() {
        println!("PROSE-BOUNDS: clean (0 governed file(s); nothing scanned)");
        return Ok(0);
    }

    let mut per_file: Vec<(String, Vec<Finding>)> = Vec::new();
    for (full, rel) in &files {
        let mut sink = Sink::default();
        spec::walk_prose(std::slice::from_ref(full), "prose-bound-exempt:", &mut sink)?;
        per_file.push((rel.clone(), sink.judge(&b)));
    }
    let total: usize = per_file.iter().map(|f| f.1.len()).sum();

    let help = "  help: split the long sentence at its colons and semicolons, break the long unit into paragraphs or bullets, and state a rule repeated across a section's units once, above them — or, for a deliberate keep, tag '<!-- prose-bound-exempt: <reason> -->' on the flagged line or the one above. The bounds are the CANON_KIT_PROSE_BOUND_* knobs (canon-kit/SPEC.md §check-prose-bounds).";
    let Some(rows) = rows else {
        if total == 0 {
            println!("PROSE-BOUNDS: clean ({} governed file(s); no sentence, unit or repeated phrase over its bound)", files.len());
            return Ok(0);
        }
        println!("check-prose-bounds: {} finding(s) across {} governed file(s):", total, files.len());
        for (rel, fs) in &per_file {
            for f in fs {
                println!("{}:{}: {}", rel, f.line, f.text);
            }
        }
        println!("{}", help);
        return Ok(1);
    };

    let mut over: Vec<String> = Vec::new();
    let mut under: Vec<String> = Vec::new();
    let mut held = 0usize;
    for (rel, fs) in &per_file {
        let n = fs.len();
        match rows.get(rel) {
            Some(&r) if n == r => held += 1,
            Some(&r) if n > r => {
                over.push(format!("{}: {} finding(s) against a ceiling row of {} — shorten the prose:", rel, n, r));
                over.extend(fs.iter().map(|f| format!("{}:{}: {}", rel, f.line, f.text)));
            }
            Some(&r) if n == 0 => under.push(format!("{}: no finding against a ceiling row of {} — delete its row", rel, r)),
            Some(&r) => under.push(format!("{}: {} finding(s) against a ceiling row of {} — lower the row to '{} {}'", rel, n, r, n, rel)),
            None if n > 0 => {
                over.push(format!("{}: {} finding(s) and no ceiling row — shorten the prose:", rel, n));
                over.extend(fs.iter().map(|f| format!("{}:{}: {}", rel, f.line, f.text)));
            }
            None => {}
        }
    }
    if over.is_empty() && under.is_empty() {
        println!(
            "PROSE-BOUNDS: clean ({} governed file(s); {} finding(s) held at the ceiling rows of {} file(s) in {})",
            files.len(),
            total,
            held,
            ceiling_file
        );
        return Ok(0);
    }
    println!("check-prose-bounds: the findings disagree with the ceiling in {} — a row must equal its file's count:", ceiling_file);
    for l in over.iter().chain(under.iter()) {
        println!("{}", l);
    }
    if !over.is_empty() {
        println!("{}", help);
    }
    if !under.is_empty() {
        println!("  help: a pass that removed findings records the lowered row in the same commit, so the slack cannot hide a later regression. The worklist is 'CANON_KIT_PROSE_BOUND_CEILING_FILE= bash gate-sdk/bin/run-gates.sh --only check-prose-bounds'.");
    }
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_code_span_is_one_word_and_its_dots_split_nothing() {
        let u = build_unit(&[(3, "Run `a. b. c` now.".to_string()), (4, "Then stop.".to_string())]);
        let spans = spec::sentence_spans(&u.text);
        assert_eq!(spans.len(), 2);
        assert_eq!(spec::word_count(&u.text[spans[0].0..spans[0].1]), 3);
        assert_eq!(u.line_at(spans[1].0), 4);
    }

    #[test]
    fn the_phrase_stream_drops_code_links_paths_and_punctuation() {
        let u = build_unit(&[(1, "The [rule](a/b.md#x) in `k` and SPEC.md, per canon-kit/SPEC.md — stated.".to_string())]);
        let w = phrase_words(&u.text);
        assert_eq!(w, vec!["the", "rule", "in", "and", "per", "stated"]);
    }

    #[test]
    fn overlapping_windows_show_as_one_phrase() {
        let a = "a b c".to_string();
        let b = "b c d".to_string();
        let c = "x y z".to_string();
        assert_eq!(merge_overlaps(&[&a, &b, &c]), vec!["a b c d", "x y z"]);
    }

    #[test]
    fn a_ceiling_row_is_a_positive_count_and_one_path() {
        let dir = std::env::temp_dir().join(format!("prose-bounds-ceiling-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let f = dir.join("c.txt");
        let p = f.display().to_string();
        std::fs::write(&f, "# contract: x\n3 a.md\n1 ./b.md\n").expect("write");
        let rows = ceiling(&p).expect("parses");
        assert_eq!(rows.get("b.md"), Some(&1));
        for bad in ["0 a.md\n", "x a.md\n", "3\n", "1 a.md\n2 a.md\n"] {
            std::fs::write(&f, bad).expect("write");
            assert!(ceiling(&p).is_err(), "{:?}", bad);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
