// spec: canon-kit/SPEC.md §check-spec-embedded-source — a canonical spec's fenced block must not
// verbatim-copy a tracked source file
use crate::spec;
use crate::walk;
use std::collections::BTreeMap;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-spec-embedded-source: {}", e);
            2
        }
    }
}

fn trim(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_ascii_whitespace())
}

// spec: canon-kit/SPEC.md §check-spec-embedded-source — a punctuation-only line is not evidence
// of a copy, so it counts toward neither side of the fraction
fn trivial(s: &str) -> bool {
    matches!(s, "" | "{" | "}" | "(" | ")" | "..." | "};" | "});" | "//" | "#")
}

struct Langs {
    lang2kind: BTreeMap<String, String>,
    ext2kind: BTreeMap<String, String>,
    base2kind: BTreeMap<String, String>,
    globs: Vec<String>,
}

// spec: canon-kit/SPEC.md §check-spec-embedded-source — the language roster is consumer config,
// each entry `<kind>|<aliases>|<globs>`; the illustrative list then drops a fence language from
// the map while leaving its file globs in the candidate index
fn langs() -> Result<Langs, String> {
    let mut l = Langs {
        lang2kind: BTreeMap::new(),
        ext2kind: BTreeMap::new(),
        base2kind: BTreeMap::new(),
        globs: Vec::new(),
    };
    for entry in walk::knob_array("CANON_KIT_EMBED_LANGS")? {
        let mut it = entry.splitn(3, '|');
        let kind = it.next().unwrap_or("").to_string();
        let aliases = it.next().unwrap_or("").to_string();
        let globs = it.next().unwrap_or("").to_string();
        for a in aliases.split(',') {
            if !a.is_empty() {
                l.lang2kind.insert(a.to_string(), kind.clone());
            }
        }
        for g in globs.split(',') {
            if g.is_empty() {
                continue;
            }
            l.globs.push(g.to_string());
            match g.strip_prefix("*.") {
                Some(ext) if !ext.is_empty() => {
                    l.ext2kind.insert(ext.to_string(), kind.clone());
                }
                _ => {
                    l.base2kind.insert(g.to_string(), kind.clone());
                }
            }
        }
    }
    for i in walk::knob_array("CANON_KIT_EMBED_ILLUSTRATIVE")? {
        l.lang2kind.remove(&i);
    }
    Ok(l)
}

impl Langs {
    fn file_kind(&self, path: &str) -> String {
        let b = path.rsplit('/').next().unwrap_or(path);
        if let Some(k) = self.base2kind.get(b) {
            return k.clone();
        }
        let e = match b.rfind('.') {
            Some(i) => &b[i + 1..],
            None => b,
        };
        self.ext2kind.get(e).cloned().unwrap_or_default()
    }
}

fn is_spec_name(path: &str) -> bool {
    let b = path.rsplit('/').next().unwrap_or(path);
    b.contains("SPEC") && b.ends_with(".md")
}

// spec: canon-kit/SPEC.md §check-spec-embedded-source — the opening fence carries a bare
// alphabetic info string and nothing else; anything richer is not an opener at all
fn fence_open(line: &str) -> Option<String> {
    let t = line.trim_start_matches(|c: char| c.is_ascii_whitespace());
    let rest = t.strip_prefix("```")?;
    let rest = rest.trim_start_matches(|c: char| c.is_ascii_whitespace());
    let mut i = 0usize;
    let b = rest.as_bytes();
    while i < b.len() && (b[i].is_ascii_alphabetic() || b[i] == b'+') {
        i += 1;
    }
    let lang = &rest[..i];
    if !rest[i..]
        .chars()
        .all(|c: char| c.is_ascii_whitespace())
    {
        return None;
    }
    Some(lang.to_ascii_lowercase())
}

fn fence_close(line: &str) -> bool {
    let t = line.trim_start_matches(|c: char| c.is_ascii_whitespace());
    match t.strip_prefix("```") {
        Some(rest) => rest.chars().all(|c: char| c.is_ascii_whitespace()),
        None => false,
    }
}

struct Block {
    lang: String,
    kind: String,
    start: usize,
    nb: usize,
    lines: Vec<String>,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("scan root not found: {}", root));
    }
    let l = langs()?;
    let threshold: f64 = walk::knob_scalar("CANON_KIT_EMBED_THRESHOLD")?
        .parse()
        .unwrap_or(0.0);
    let minlines: usize = walk::knob_scalar("CANON_KIT_EMBED_MINLINES")?
        .parse()
        .unwrap_or(0);
    let wirekind = walk::knob_scalar("CANON_KIT_EMBED_WIRE_KIND")?;

    let prune = walk::prune_dirs()?;
    let mut candidates: Vec<String> = walk::find_with_prune(Path::new(root), &|n| {
        prune.iter().any(|d| d == n)
    })
    .unwrap_or_default()
    .into_iter()
    .filter(|p| {
        let b = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        l.globs.iter().any(|g| walk::pattern_match(g, b))
    })
    .map(|p| spec::strip_dot_slash(&p.display().to_string()))
    .collect();
    candidates.sort();
    candidates.dedup();

    let mut amendfiles: Vec<String> = spec::amendments(root)?
        .into_iter()
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    amendfiles.sort();
    amendfiles.dedup();

    let mut specs: Vec<String> = spec::canonical_specs(root)?
        .into_iter()
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    specs.extend(amendfiles.iter().cloned());
    specs.retain(|s| !s.is_empty());
    specs.sort();
    specs.dedup();

    if specs.is_empty() {
        println!("SPEC-EMBEDDED-SOURCE: clean (0 spec files found)");
        return Ok(0);
    }

    // spec: canon-kit/SPEC.md §check-spec-embedded-source — the candidate index: one entry per
    // distinct non-trivial trimmed line, naming every source file carrying it
    let mut idx: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut fkind: BTreeMap<String, String> = BTreeMap::new();
    for f in &candidates {
        if is_spec_name(f) {
            continue;
        }
        let text = match std::fs::read(Path::new(f)) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => return Err(format!("cannot read {}: {}", f, e)),
        };
        let mut seen: Vec<String> = Vec::new();
        for line in text.lines() {
            let s = trim(line);
            if trivial(s) {
                continue;
            }
            if !seen.iter().any(|x| x == s) {
                seen.push(s.to_string());
                idx.entry(s.to_string()).or_default().push(f.clone());
            }
            fkind
                .entry(f.clone())
                .or_insert_with(|| l.file_kind(f));
        }
    }

    let mut flagged: Vec<String> = Vec::new();
    for f in &specs {
        let text = match std::fs::read(Path::new(f)) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(e) => return Err(format!("cannot read {}: {}", f, e)),
        };
        // spec: canon-kit/SPEC.md §check-spec-embedded-source — the exemption is a path-identity
        // test against the amendment set rather than a name pattern, which is what re-arms it the
        // moment the amendment is deleted
        let is_amendment = amendfiles.iter().any(|a| a == f);
        let mut block: Option<Block> = None;
        let mut skipblock = false;
        let mut lastnb = String::new();
        for (idx0, line) in text.lines().enumerate() {
            let fnr = idx0 + 1;
            match block.as_mut() {
                None => {
                    if let Some(lang) = fence_open(line) {
                        skipblock = lastnb.contains("spec-embedded-source-exempt:");
                        let kind = l.lang2kind.get(&lang).cloned().unwrap_or_default();
                        block = Some(Block {
                            lang,
                            kind,
                            start: fnr,
                            nb: 0,
                            lines: Vec::new(),
                        });
                        continue;
                    }
                    if !trim(line).is_empty() {
                        lastnb = line.to_string();
                    }
                }
                Some(b) => {
                    if fence_close(line) {
                        if !skipblock {
                            if let Some(hit) = emit_block(
                                b, f, &idx, &fkind, &candidates, minlines, threshold, &wirekind,
                                is_amendment,
                            ) {
                                flagged.push(hit);
                            }
                        }
                        block = None;
                        lastnb = String::new();
                        continue;
                    }
                    let s = trim(line);
                    if trivial(s) {
                        continue;
                    }
                    b.nb += 1;
                    if !b.lines.iter().any(|x| x == s) {
                        b.lines.push(s.to_string());
                    }
                }
            }
        }
    }

    if !flagged.is_empty() {
        println!("check-spec-embedded-source: spec fenced block(s) verbatim-copy a tracked source");
        println!("file (the file is the home of the body); cite the path instead, or bless a");
        println!("genuinely-illustrative block:");
        println!();
        for l in &flagged {
            println!("{}", l);
        }
        println!("  help: replace the embedded body with a path reference to the cited source file, or, if the block is a genuine illustrative shape/schema example, add '<!-- spec-embedded-source-exempt: <reason> -->' on the line directly above the opening fence");
        return Ok(1);
    }
    println!(
        "SPEC-EMBEDDED-SOURCE: clean ({} spec file(s) scanned; no fenced block copies a tracked source)",
        specs.len()
    );
    Ok(0)
}

#[allow(clippy::too_many_arguments)]
fn emit_block(
    b: &Block,
    file: &str,
    idx: &BTreeMap<String, Vec<String>>,
    fkind: &BTreeMap<String, String>,
    candidates: &[String],
    minlines: usize,
    threshold: f64,
    wirekind: &str,
    is_amendment: bool,
) -> Option<String> {
    if b.kind.is_empty() || b.nb < minlines {
        return None;
    }
    // spec: canon-kit/SPEC.md §check-spec-embedded-source — the amendment wire-delta exemption is
    // scoped to the configured wire kind, never to amendments generally
    if b.kind == wirekind && is_amendment {
        return None;
    }
    let nd = b.lines.len();
    if nd < minlines {
        return None;
    }
    let mut hits: BTreeMap<&str, usize> = BTreeMap::new();
    for line in &b.lines {
        if let Some(files) = idx.get(line) {
            for f in files {
                if fkind.get(f).map(|k| k == &b.kind).unwrap_or(false) {
                    *hits.entry(f.as_str()).or_insert(0) += 1;
                }
            }
        }
    }
    // spec: canon-kit/SPEC.md §check-spec-embedded-source — the best-match scan runs the candidate
    // set in its own sorted order, so a tie resolves to one named file rather than to whichever
    // one a hash happened to yield first
    let mut best = 0.0f64;
    let mut bestf = "";
    for c in candidates {
        if let Some(n) = hits.get(c.as_str()) {
            let frac = *n as f64 / nd as f64;
            if frac > best {
                best = frac;
                bestf = c.as_str();
            }
        }
    }
    if best >= threshold {
        return Some(format!(
            "  {}:{}  [{}] ~{}% of {} lines copied from {}",
            file,
            b.start,
            b.lang,
            (best * 100.0 + 0.5) as i64,
            nd,
            bestf
        ));
    }
    None
}
