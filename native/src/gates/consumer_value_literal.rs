// spec: gate-sdk/SPEC.md §check-consumer-value-literal — every literal a gate's crate modules carry
// that names a tracked consumer path or heading is a knob or a declared site
use crate::gates::door_binding::rust_region;
use crate::registry::{self, GATE_MODULES};
use crate::{proc, programs, walk};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

const NAME: &str = "check-consumer-value-literal";

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — the site declaration; the reason is
// mandatory on the `door-contributor:` convention and an empty one is malformed, not exempting
const DECL: &str = "consumer-value-exempt:";

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — the knob tables and the descriptor knob
// lines are the sanctioned producers of a consumer value, so a file under this crate directory
// leaves the corpus
const KNOB_TABLES: &str = "knobs";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {} — the check could not run; treating as failure (not clean)", NAME, e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — one string literal as written, with the
// line its opening quote stands on
struct Literal {
    line: usize,
    text: String,
}

fn ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — the end of a `"…"` body opened at `at`,
// a backslash escaping the byte after it
fn quoted_end(b: &[u8], at: usize) -> usize {
    let mut j = at;
    while j < b.len() {
        match b[j] {
            b'\\' => j += 2,
            b'"' => return j,
            _ => j += 1,
        }
    }
    b.len()
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — the end of a raw body opened at `at`,
// closed by a quote followed by as many `#` as opened it
fn raw_end(b: &[u8], at: usize, hashes: usize) -> usize {
    let mut j = at;
    while j < b.len() {
        if b[j] == b'"' && b.len() - (j + 1) >= hashes && b[j + 1..j + 1 + hashes].iter().all(|c| *c == b'#') {
            return j;
        }
        j += 1;
    }
    b.len()
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — the literal reader: plain, raw and
// byte strings, each attributed to its opening line, with comments, char literals and lifetimes
// skipped
fn literals(src: &str) -> Vec<Literal> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut line = 1usize;
    let mut i = 0usize;
    let lines_in = |from: usize, to: usize| b[from..to.min(b.len())].iter().filter(|c| **c == b'\n').count();
    while i < b.len() {
        let c = b[i];
        if c == b'\n' {
            line += 1;
            i += 1;
            continue;
        }
        if c == b'/' && b.get(i + 1) == Some(&b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if c == b'/' && b.get(i + 1) == Some(&b'*') {
            let start = i;
            let mut depth = 1usize;
            i += 2;
            while i < b.len() && depth > 0 {
                if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
                    depth += 1;
                    i += 2;
                } else if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            line += lines_in(start, i);
            continue;
        }
        if c == b'\'' {
            i += char_or_lifetime(src, i);
            continue;
        }
        let boundary = i == 0 || !ident_byte(b[i - 1]);
        let prefix = if !boundary {
            0
        } else if b[i..].starts_with(b"br") {
            2
        } else if c == b'r' || c == b'b' {
            1
        } else {
            0
        };
        let raw = prefix > 0 && b[i + prefix - 1] == b'r';
        let mut k = i + prefix;
        let mut hashes = 0usize;
        if raw {
            while b.get(k) == Some(&b'#') {
                hashes += 1;
                k += 1;
            }
        }
        if b.get(k) == Some(&b'"') && (prefix > 0 || c == b'"') {
            let open = k + 1;
            let close = if raw { raw_end(b, open, hashes) } else { quoted_end(b, open) };
            out.push(Literal {
                line,
                text: src[open..close.min(b.len())].to_string(),
            });
            line += lines_in(i, close);
            i = (close + 1 + if raw { hashes } else { 0 }).min(b.len());
            continue;
        }
        if prefix > 0 || ident_byte(c) {
            while i < b.len() && ident_byte(b[i]) {
                i += 1;
            }
            continue;
        }
        i += src[i..].chars().next().map(char::len_utf8).unwrap_or(1);
    }
    out
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — a quote opens a char literal when it
// closes one character (or one escape) later, and a lifetime otherwise
fn char_or_lifetime(src: &str, at: usize) -> usize {
    let rest = &src[at + 1..];
    let mut chars = rest.char_indices();
    match chars.next() {
        Some((_, '\\')) => {
            let escaped = rest[1..].chars().next().map(char::len_utf8).unwrap_or(0);
            match rest[1 + escaped..].find('\'') {
                Some(p) => p + escaped + 3,
                None => 1,
            }
        }
        Some((_, ch)) => {
            let after = ch.len_utf8();
            if rest[after..].starts_with('\'') {
                after + 2
            } else {
                1
            }
        }
        None => 1,
    }
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — a line's declaration reason inside a `//`
// comment, or `None` where the line carries none
fn decl_reason(line: &str) -> Option<&str> {
    let at = line.find(DECL)?;
    line[..at].contains("//").then(|| line[at + DECL.len()..].trim())
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — the heading arm's index: each heading,
// a `#`-led line outside a fence, keyed by its whole line and by its text, with its file
struct Headings {
    marked: HashMap<String, String>,
    bare: HashMap<String, (String, String)>,
}

fn heading(line: &str) -> Option<&str> {
    let hashes = line.bytes().take_while(|c| *c == b'#').count();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    line[hashes..].strip_prefix(' ').map(str::trim).filter(|t| !t.is_empty())
}

fn index_headings(files: &[String]) -> Result<Headings, String> {
    let mut h = Headings {
        marked: HashMap::new(),
        bare: HashMap::new(),
    };
    for f in files {
        let text = read(f)?;
        let mut fenced = crate::spec::Fence::with_tilde();
        for raw in text.lines() {
            let line = raw.trim_end();
            if fenced.delimits(line) {
                continue;
            }
            if fenced.is_open() {
                continue;
            }
            let Some(body) = heading(line) else { continue };
            h.marked.entry(line.to_string()).or_insert_with(|| f.clone());
            if body.split_whitespace().count() >= 2 {
                h.bare
                    .entry(body.to_string())
                    .or_insert_with(|| (line.to_string(), f.clone()));
            }
        }
    }
    Ok(h)
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|_| format!("unreadable corpus file: {}", path))
}

fn under_any(roots: &[String], p: &str) -> bool {
    roots.iter().any(|r| walk::at_or_under(r.trim_end_matches('/'), p))
}

// spec: gate-sdk/SPEC.md §check-consumer-value-literal — the two arms, the path arm first; the
// answer is the finding's detail
fn arm(lit: &str, tracked: &BTreeSet<String>, dirs: &BTreeSet<String>, roots: &[String], h: &Headings) -> Option<String> {
    let p = lit.trim_end_matches('/');
    let pathlike = !p.is_empty()
        && p != "."
        && p != ".."
        && walk::path_root(p).is_none()
        && !p.split('/').any(|s| s.is_empty() || s == "." || s == "..");
    if pathlike && (tracked.contains(p) || dirs.contains(p)) && !under_any(roots, p) {
        return Some(format!("path arm: names the tracked path {}", p));
    }
    if lit.starts_with('#') {
        if let Some(f) = h.marked.get(lit) {
            return Some(format!("heading arm: the heading '{}' in {}", lit, f));
        }
    } else if let Some((line, f)) = h.bare.get(lit) {
        return Some(format!("heading arm: the heading '{}' in {}", line, f));
    }
    None
}

fn rule(args: &[String]) -> Result<i32, String> {
    if let Some(a) = args.first() {
        return Err(format!("unexpected argument: {}", a));
    }
    let src = walk::knob_scalar("GATE_SDK_NATIVE_SRC")?;
    let src = src.trim_end_matches('/').to_string();
    if src.is_empty() || !Path::new(&src).is_dir() {
        return Err(format!(
            "GATE_SDK_NATIVE_SRC names no crate source directory ('{}') — a gate that reads no crate cannot say the crate is clean",
            src
        ));
    }
    let kit_roots = walk::kit_roots()?;
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let dirs = registry::resolve_dirs(&gates_dir, &kit_roots);

    // spec: gate-sdk/SPEC.md §check-consumer-value-literal — the members: every registry row whose
    // descriptor resolves, under a kit root or in the gates dir
    let mut members = 0usize;
    let mut corpus: BTreeSet<String> = BTreeSet::new();
    let knob_dir = format!("{}/{}", src, KNOB_TABLES);
    for (member, _) in GATE_MODULES {
        if registry::resolve(member, &dirs).is_none() {
            continue;
        }
        members += 1;
        for f in registry::module_files(member, &dirs)? {
            if !walk::at_or_under(&knob_dir, &f) {
                corpus.insert(f);
            }
        }
    }
    if members == 0 {
        return Err("no member resolves under the gates dir or any kit root".to_string());
    }

    let listed = proc::run(&programs::GIT, &["ls-files"])?;
    let Some(out) = listed.stdout() else {
        return Err("git ls-files failed".to_string());
    };
    let tracked: BTreeSet<String> = String::from_utf8_lossy(out).lines().map(str::to_string).collect();
    let mut tracked_dirs: BTreeSet<String> = BTreeSet::new();
    for t in &tracked {
        let mut at = 0usize;
        while let Some(p) = t[at..].find('/') {
            tracked_dirs.insert(t[..at + p].to_string());
            at += p + 1;
        }
    }
    let prune = walk::prune_dirs()?;
    let md: Vec<String> = tracked
        .iter()
        .filter(|t| t.ends_with(".md") && !under_any(&kit_roots, t) && !walk::path_pruned(t, &prune))
        .cloned()
        .collect();
    let headings = index_headings(&md)?;

    let mut findings: Vec<String> = Vec::new();
    let mut malformed: Vec<String> = Vec::new();
    let (mut read_lits, mut valved) = (0usize, 0usize);
    for f in &corpus {
        let text = read(f)?;
        let region = rust_region(&text);
        let lines: Vec<&str> = region.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            if decl_reason(line) == Some("") {
                malformed.push(format!("{}:{}: {}", f, n + 1, line.trim()));
            }
        }
        let declared = |l: usize| {
            let at = |k: usize| lines.get(k).and_then(|s| decl_reason(s)).is_some_and(|r| !r.is_empty());
            at(l - 1) || (l >= 2 && at(l - 2))
        };
        for lit in literals(region) {
            read_lits += 1;
            let Some(detail) = arm(&lit.text, &tracked, &tracked_dirs, &kit_roots, &headings) else { continue };
            if declared(lit.line) {
                valved += 1;
                continue;
            }
            findings.push(format!("{}:{}: \"{}\" — {}", f, lit.line, lit.text, detail));
        }
    }

    if !findings.is_empty() || !malformed.is_empty() {
        if !findings.is_empty() {
            println!("{}: gate source carries a consumer value as a literal:", NAME);
            for x in &findings {
                println!("  {}", x);
            }
            println!("  help: read the value from a knob, the literal as its default, in the owning kit's");
            println!("        table or on the reading gate's descriptor (gate-sdk/SPEC.md §The knob file,");
            println!("        §The declaration cohort), or declare the site with");
            println!("        '// {} <reason>' on the literal's line or the line above.", DECL);
        }
        if !malformed.is_empty() {
            if !findings.is_empty() {
                println!();
            }
            println!("{}: '{}' with an empty reason is malformed (the reason is the audit trail):", NAME, DECL);
            for m in &malformed {
                println!("  {}", m);
            }
            println!("  help: write the reason that says why the literal is no consumer value, on the");
            println!("        'comment-tier-exempt:' convention. An empty reason reds rather than exempting.");
        }
        return Ok(1);
    }

    // spec: gate-sdk/SPEC.md §check-consumer-value-literal — the counts are a report with no floor
    println!(
        "CONSUMER-VALUE-LITERAL: clean ({} member(s) swept, {} crate file(s) read, {} literal(s) read, {} site(s) valved)",
        members,
        corpus.len(),
        read_lits,
        valved
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(src: &str) -> Vec<(usize, String)> {
        literals(src).into_iter().map(|l| (l.line, l.text)).collect()
    }

    #[test]
    fn the_reader_takes_every_string_form_and_skips_comments_chars_and_lifetimes() {
        let src = "// \"in a comment\"\nfn f<'a>(x: &'a str) {\n    let c = '\"';\n    let e = '\\'';\n    /* \"block\" /* \"nested\" */ */\n    let a = \"plain \\\" quote\";\n    let r = r#\"raw \"inner\" \"#;\n    let b = b\"bytes\";\n    let br = br\"raw bytes\";\n    let m = \"two\nlines\";\n    let after = \"x\";\n}\n";
        assert_eq!(
            texts(src),
            vec![
                (6, "plain \\\" quote".to_string()),
                (7, "raw \"inner\" ".to_string()),
                (8, "bytes".to_string()),
                (9, "raw bytes".to_string()),
                (10, "two\nlines".to_string()),
                (12, "x".to_string()),
            ]
        );
    }

    #[test]
    fn a_raw_identifier_and_a_multibyte_char_are_not_strings() {
        assert_eq!(texts("let r#type = '§'; let s = \"y\";"), vec![(1, "y".to_string())]);
    }

    #[test]
    fn a_declaration_counts_only_inside_a_comment_and_an_empty_reason_is_distinguishable() {
        assert_eq!(decl_reason("    // consumer-value-exempt: a kit constant"), Some("a kit constant"));
        assert_eq!(decl_reason("    // consumer-value-exempt:"), Some(""));
        assert_eq!(decl_reason("const D: &str = \"consumer-value-exempt:\";"), None);
    }

    #[test]
    fn a_bare_literal_matches_only_a_multi_word_heading_and_a_marked_one_any() {
        let mut h = Headings {
            marked: HashMap::new(),
            bare: HashMap::new(),
        };
        h.marked.insert("## Done".to_string(), "Q.md".to_string());
        h.bare.insert("Lessons Learned".to_string(), ("## Lessons Learned".to_string(), "Q.md".to_string()));
        let none = BTreeSet::new();
        assert!(arm("## Done", &none, &none, &[], &h).is_some());
        assert!(arm("Done", &none, &none, &[], &h).is_none());
        assert!(arm("Lessons Learned", &none, &none, &[], &h).is_some());
        assert_eq!(heading("#### Four words here"), Some("Four words here"));
        assert_eq!(heading("#!/bin/bash"), None);
    }

    #[test]
    fn the_path_arm_reads_a_tracked_file_or_directory_outside_every_kit_root() {
        let tracked: BTreeSet<String> = ["docs/a.md", "kit/x.sh"].iter().map(|s| s.to_string()).collect();
        let dirs: BTreeSet<String> = ["docs", "kit"].iter().map(|s| s.to_string()).collect();
        let roots = vec!["kit".to_string()];
        let h = Headings {
            marked: HashMap::new(),
            bare: HashMap::new(),
        };
        assert!(arm("docs/a.md", &tracked, &dirs, &roots, &h).is_some());
        assert!(arm("docs/", &tracked, &dirs, &roots, &h).is_some());
        assert!(arm("kit/x.sh", &tracked, &dirs, &roots, &h).is_none());
        assert!(arm(".", &tracked, &dirs, &roots, &h).is_none());
        assert!(arm("docs/b.md", &tracked, &dirs, &roots, &h).is_none());
    }
}
