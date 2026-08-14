// spec: canon-kit/SPEC.md §check-spec-pointer — every spec:/contract: directive on a governed
// source and every free-prose <path>.md §<heading> citation on a governed manifest resolves
use crate::proc;
use crate::spec;
use std::collections::HashMap;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-spec-pointer: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let probe = proc::run("git", &["-C", root, "rev-parse", "--git-dir"])?;
    if probe.stdout().is_none() {
        return Err(format!(
            "{} is not a git repository — cannot verify tracked targets",
            root
        ));
    }
    // spec: canon-kit/SPEC.md §check-spec-pointer — the tracked-file membership set, filled once
    // from a single `git ls-files` pass rather than a per-pointer `--error-unmatch` exec
    let ls = proc::run("git", &["-C", root, "ls-files", "-z"])?;
    let raw = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => return Err("git ls-files failed".to_string()),
    };
    let tracked: Vec<&str> = raw.split('\0').filter(|s| !s.is_empty()).collect();

    let whitelist = spec::knob_array_pub("CANON_KIT_COMMENT_WHITELIST")?;
    let mut headings: HeadingCache = HeadingCache::default();
    let mut errors: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    let mut pointers = 0usize;
    let mut markers = 0usize;

    for f in spec::comment_surface(root, false)? {
        let rel = spec::strip_dot_slash(f.strip_prefix(&format!("{}/", root)).unwrap_or(&f));
        if spec::comment_whitelisted(&rel, &whitelist) {
            continue;
        }
        scanned += 1;
        let text = spec::read_text(Path::new(&f))?;
        for (idx, line) in text.lines().enumerate() {
            let body = match directive_body(line) {
                Some(b) => b,
                None => continue,
            };
            let lineno = idx + 1;
            let is_contract = body.starts_with("contract:");
            let after = if is_contract {
                &body["contract:".len()..]
            } else {
                &body["spec:".len()..]
            };
            // spec: gate-sdk/SPEC.md §The workflow directory — contract: rules two payloads, and
            // the version-marker form names no path to resolve
            if is_contract && is_version_marker(after.trim()) {
                markers += 1;
                continue;
            }
            pointers += 1;
            let (path, frag) = target(after);
            if path.is_empty() || path.starts_with('§') {
                errors.push(format!(
                    "{}:{}: pointer directive carries no target path: {}",
                    rel, lineno, body
                ));
                continue;
            }
            if !Path::new(&format!("{}/{}", root, path)).is_file() {
                errors.push(format!(
                    "{}:{}: target file not found: {}",
                    rel, lineno, path
                ));
                continue;
            }
            if !tracked.iter().any(|t| *t == path) {
                errors.push(format!("{}:{}: target file untracked: {}", rel, lineno, path));
                continue;
            }
            if !frag.is_empty()
                && !headings.present(&format!("{}/{}", root, path), &frag, Mode::Exact)?
            {
                errors.push(format!(
                    "{}:{}: §heading not found in {}: §{}",
                    rel, lineno, path, frag
                ));
            }
        }
    }

    // spec: canon-kit/SPEC.md §check-spec-pointer — the prose-citation pass in prefix mode over
    // the manifest set; an untracked cited path is out of scope
    let manifest_files = spec::manifest_files(root)?;
    let mut prose_cites = 0usize;
    let manifests = manifest_files.len();
    for cite in prose_citations(&manifest_files)? {
        if !Path::new(&format!("{}/{}", root, cite.path)).is_file() {
            continue;
        }
        if !tracked.iter().any(|t| *t == cite.path) {
            continue;
        }
        prose_cites += 1;
        if cite.frag.is_empty() {
            continue;
        }
        if !headings.present(
            &format!("{}/{}", root, cite.path),
            &cite.frag,
            Mode::Prefix,
        )? {
            let prel = spec::strip_dot_slash(
                cite.file
                    .strip_prefix(&format!("{}/", root))
                    .unwrap_or(&cite.file),
            );
            let shown: String = cite.frag.chars().take(50).collect();
            errors.push(format!(
                "{}:{}: §heading not found in {}: §{}",
                prel, cite.line, cite.path, shown
            ));
        }
    }

    if !errors.is_empty() {
        println!("SPEC-POINTER: {} dangling pointer(s):", errors.len());
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: a spec:/contract: directive and a free-prose <path>.md §<heading> citation each bind a site to the requirement that governs it — the binding is only live if it resolves. Fix the <path> (repo-relative, tracked) or the §<heading> to name the current target, or drop the § fragment for a file-only pointer. A renamed heading updates every inbound pointer and citation in the same commit.");
        return Ok(1);
    }
    println!(
        "SPEC-POINTER: clean ({} directive pointer(s) across {} governed source(s), {} version-marker header(s) skipped as naming no path; {} prose citation(s) across {} manifest file(s); every target file tracked and named §heading present)",
        pointers, scanned, markers, prose_cites, manifests
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-spec-pointer — shape-only extraction of the directive lines
// across the hash and slash comment surfaces: the colon needs trailing whitespace or
// end-of-line, which parts a pointer from prose that merely opens with the keyword
fn directive_body(line: &str) -> Option<String> {
    let b = line.as_bytes();
    let mut i = lead_space(b, 0);
    let mut n = 0usize;
    if i < b.len() && b[i] == b'#' {
        while i < b.len() && b[i] == b'#' {
            i += 1;
            n += 1;
        }
    } else if i + 1 < b.len() && b[i] == b'/' && b[i + 1] == b'/' {
        while i < b.len() && b[i] == b'/' {
            i += 1;
            n += 1;
        }
        if n < 2 {
            return None;
        }
    } else if i < b.len() && b[i] == b'*' {
        i += 1;
        n = 1;
    }
    if n == 0 {
        return None;
    }
    let rest_at = lead_space(b, i);
    let rest = &line[rest_at..];
    for kw in ["spec:", "contract:"] {
        if let Some(after) = rest.strip_prefix(kw) {
            if after.is_empty() || after.starts_with(' ') || after.starts_with('\t') {
                return Some(rest.to_string());
            }
        }
    }
    None
}

fn lead_space(b: &[u8], mut i: usize) -> usize {
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
    }
    i
}

// spec: gate-sdk/SPEC.md §The workflow directory — `^[a-z0-9-]+ v[0-9]+$`, the wire-format
// version its owning gate parses
fn is_version_marker(payload: &str) -> bool {
    let (name, ver) = match payload.split_once(' ') {
        Some(v) => v,
        None => return false,
    };
    if name.is_empty()
        || !name
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
    {
        return false;
    }
    match ver.strip_prefix('v') {
        Some(d) => !d.is_empty() && d.bytes().all(|c| c.is_ascii_digit()),
        None => false,
    }
}

// spec: canon-kit/SPEC.md §check-spec-pointer — the directive's payload: the significant head
// ends at the em-dash prose tail, the first whitespace-separated word is the path, and a §
// fragment may ride either that word or the words after it
fn target(after: &str) -> (String, String) {
    let mut sig = after;
    if let Some(p) = sig.find(" — ") {
        sig = &sig[..p];
    }
    if let Some(p) = sig.find(" -- ") {
        sig = &sig[..p];
    }
    let sig = sig.trim_matches(|c: char| c == ' ' || c == '\t');
    let (mut path, prest) = match sig.find([' ', '\t']) {
        Some(p) => (
            sig[..p].to_string(),
            sig[p..].trim_matches(|c: char| c == ' ' || c == '\t').to_string(),
        ),
        None => (sig.to_string(), String::new()),
    };
    let mut frag = String::new();
    if let Some(p) = path.find('§') {
        frag = path[p + '§'.len_utf8()..].to_string();
        path = path[..p].to_string();
    } else if let Some(p) = prest.find('§') {
        frag = prest[p + '§'.len_utf8()..].to_string();
    }
    (
        path,
        frag.trim_matches(|c: char| c == ' ' || c == '\t').to_string(),
    )
}

struct Cite {
    file: String,
    line: usize,
    path: String,
    frag: String,
}

// spec: canon-kit/SPEC.md §check-spec-pointer — shape-only extraction of prose <path>.md §
// citations over the blank-line paragraph join, fenced code skipped; the fragment is the whole
// tail after the §, which the prefix-mode resolver reads a heading off
fn prose_citations(files: &[std::path::PathBuf]) -> Result<Vec<Cite>, String> {
    let mut out: Vec<Cite> = Vec::new();
    for p in files {
        let f = p.display().to_string();
        let text = spec::read_text(p)?;
        let mut fence = false;
        let mut para: Vec<(usize, String)> = Vec::new();
        for (idx, raw) in text.lines().enumerate() {
            if spec::is_fence_line(raw) {
                flush(&f, &mut para, &mut out);
                fence = !fence;
                continue;
            }
            if fence || spec::is_blank(raw) {
                flush(&f, &mut para, &mut out);
                continue;
            }
            para.push((idx + 1, raw.to_string()));
        }
        flush(&f, &mut para, &mut out);
    }
    Ok(out)
}

fn flush(file: &str, para: &mut Vec<(usize, String)>, out: &mut Vec<Cite>) {
    if para.is_empty() {
        return;
    }
    let mut joined = String::new();
    let mut lstart: Vec<usize> = Vec::new();
    // spec: canon-kit/SPEC.md §check-spec-pointer — the line's start offset is taken *before*
    // the joining space is appended, so a citation opening a continued line reports on that
    // line; the asymmetry is the awk driver's and is reproduced rather than tidied
    for (i, (_, text)) in para.iter().enumerate() {
        lstart.push(joined.len());
        if i > 0 {
            joined.push(' ');
        }
        joined.push_str(text);
    }
    let b = joined.as_bytes();
    let mut scan = 0usize;
    while scan < b.len() {
        let (ms, me, path) = match md_ref(b, scan) {
            Some(v) => v,
            None => break,
        };
        let mut li = 0usize;
        for (i, s) in lstart.iter().enumerate() {
            if *s <= ms {
                li = i;
            }
        }
        out.push(Cite {
            file: file.to_string(),
            line: para[li].0,
            path,
            frag: joined[me..].to_string(),
        });
        scan = me;
    }
    para.clear();
}

// spec: canon-kit/SPEC.md §check-spec-pointer — `[A-Za-z0-9._/-]+\.md[[:space:]]*§`, leftmost;
// the returned end is one past the § so the fragment is the remainder of the joined paragraph
fn md_ref(b: &[u8], from: usize) -> Option<(usize, usize, String)> {
    let tok = |c: u8| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'_' | b'/' | b'-');
    let mut i = from;
    while i < b.len() {
        if !tok(b[i]) || (i > 0 && tok(b[i - 1]) && i > from) {
            i += 1;
            continue;
        }
        let mut j = i;
        let mut best: Option<usize> = None;
        while j < b.len() && tok(b[j]) {
            j += 1;
            if j >= 3 && &b[j - 3..j] == b".md" {
                best = Some(j);
            }
        }
        if let Some(end) = best {
            let mut k = end;
            while k < b.len() && matches!(b[k], b' ' | b'\t') {
                k += 1;
            }
            if b[k..].starts_with("§".as_bytes()) {
                return Some((
                    i,
                    k + "§".len(),
                    String::from_utf8_lossy(&b[i..end]).into_owned(),
                ));
            }
        }
        i += 1;
    }
    None
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Exact,
    Prefix,
}

// spec: canon-kit/SPEC.md §check-spec-pointer — one resolver, two callers: the directive pass
// matches the fragment whole, the prose pass as a boundary-anchored prefix; both tolerate a
// trailing "(qualifier)"
#[derive(Default)]
struct HeadingCache {
    files: HashMap<String, Vec<(String, String)>>,
}

impl HeadingCache {
    fn headings(&mut self, file: &str) -> Result<&Vec<(String, String)>, String> {
        if !self.files.contains_key(file) {
            let text = spec::read_text(Path::new(file))?;
            let mut v: Vec<(String, String)> = Vec::new();
            for line in text.lines() {
                if let Some(h) = heading_text(line) {
                    let hs = strip_qualifier(&h);
                    v.push((h, hs));
                }
            }
            self.files.insert(file.to_string(), v);
        }
        Ok(self.files.get(file).expect("heading set just inserted"))
    }

    fn present(&mut self, file: &str, frag: &str, mode: Mode) -> Result<bool, String> {
        let stripped = strip_qualifier(frag);
        let hs = self.headings(file)?;
        Ok(hs.iter().any(|(h, hstripped)| {
            if mode == Mode::Prefix {
                is_prefix(frag, h) || is_prefix(frag, hstripped)
            } else {
                h == frag || hstripped == frag || *h == stripped || *hstripped == stripped
            }
        }))
    }
}

fn heading_text(line: &str) -> Option<String> {
    let b = line.as_bytes();
    let mut n = 0usize;
    while n < b.len() && b[n] == b'#' {
        n += 1;
    }
    if n == 0 || n > 6 || n >= b.len() || !matches!(b[n], b' ' | b'\t') {
        return None;
    }
    let mut s = n;
    while s < b.len() && matches!(b[s], b' ' | b'\t') {
        s += 1;
    }
    Some(
        line[s..]
            .trim_end_matches([' ', '\t'])
            .to_string(),
    )
}

// spec: canon-kit/SPEC.md §check-spec-pointer — `[[:space:]]*\([^)]*\)[[:space:]]*$`, awk's
// leftmost match of that anchored shape
fn strip_qualifier(s: &str) -> String {
    let b = s.as_bytes();
    for i in 0..b.len() {
        let mut j = i;
        while j < b.len() && matches!(b[j], b' ' | b'\t') {
            j += 1;
        }
        if b.get(j) != Some(&b'(') {
            continue;
        }
        j += 1;
        while j < b.len() && b[j] != b')' {
            j += 1;
        }
        if b.get(j) != Some(&b')') {
            continue;
        }
        j += 1;
        let mut k = j;
        while k < b.len() && matches!(b[k], b' ' | b'\t') {
            k += 1;
        }
        if k == b.len() {
            return s[..i].to_string();
        }
    }
    s.to_string()
}

fn is_prefix(text: &str, h: &str) -> bool {
    if h.is_empty() || !text.starts_with(h) {
        return false;
    }
    match text[h.len()..].chars().next() {
        None => true,
        Some(c) => !c.is_alphanumeric(),
    }
}
