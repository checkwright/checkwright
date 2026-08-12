// spec: canon-kit/SPEC.md §lib/spec.sh — the binary side of the corpus primitive the
// manifest-narration gate family shares: ported once and proved N times, so a per-gate
// copy of the derivation has no place to exist
use crate::walk;
use std::path::{Path, PathBuf};

// spec: gate-sdk/SPEC.md §lib/gate.sh — every knob read is bridged; the crate holds no
// default, so an unset variable is a harness error rather than a fallback.
fn knob(name: &str) -> Result<String, String> {
    std::env::var(format!("GATE_SDK_KNOB_{}", name)).map_err(|_| {
        format!(
            "GATE_SDK_KNOB_{} is unset — the gate was invoked without the config bridge \
             gate_command emits, so {} could not be resolved",
            name, name
        )
    })
}

fn knob_array(name: &str) -> Result<Vec<String>, String> {
    let raw = knob(name)?;
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    Ok(raw.split('\t').map(String::from).collect())
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the finders skip `templates/` stubs; the shell
// filters the emitted path with a fixed substring, so the port matches on the same one
fn under_templates(p: &str) -> bool {
    p.contains("/templates/")
}

// spec: canon-kit/SPEC.md §check-spec-pointer — a prose-surface candidate joins the
// manifest set iff slot-free. That pattern is a kit literal rather than consumer config,
// so the port hand-writes the two shapes and owes no regex engine.
fn slot_free(path: &Path) -> Result<bool, String> {
    let text = read_text(path)?;
    for line in text.lines() {
        if line.starts_with("CONSUMER BINDING") {
            return Ok(false);
        }
        if has_binding_slot(line) {
            return Ok(false);
        }
    }
    Ok(true)
}

// spec: canon-kit/SPEC.md §check-spec-pointer — the binding-slot half of that pattern
fn has_binding_slot(line: &str) -> bool {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i + 2 < b.len() {
        if b[i] == b'*' && b[i + 1] == b'<' && b[i + 2].is_ascii_lowercase() {
            let mut j = i + 3;
            while j < b.len() && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-')
            {
                j += 1;
            }
            if j < b.len() && b[j] == b':' {
                return true;
            }
        }
        i += 1;
    }
    false
}

pub fn read_text(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {}", path.display(), e))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn cwd() -> String {
    std::env::current_dir()
        .map(|c| c.display().to_string())
        .unwrap_or_else(|_| ".".to_string())
        .trim_end_matches('/')
        .to_string()
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the scan root's absolute form on
// `_spec_prune_kit_roots`' own four cases; a bare `.` is the cwd itself, and appending it
// as a component instead makes every prefix test below fail silently
fn root_to_abs(p: &str) -> String {
    let abs = match p {
        _ if p.starts_with('/') => p.to_string(),
        "." => cwd(),
        _ if p.starts_with("./") => format!("{}/{}", cwd(), &p[2..]),
        _ => format!("{}/{}", cwd(), p),
    };
    // spec: gate-sdk/SPEC.md §lib/gate.sh — a bridged root is spelled relative to the
    // invoking directory, so it may climb out with `..`; normalising here recovers the
    // absolute path the shell compares, without one ever crossing the bridge
    normalize(abs.trim_end_matches('/'))
}

fn file_to_abs(p: &str) -> String {
    if p.starts_with('/') {
        return p.to_string();
    }
    format!("{}/{}", cwd(), p.strip_prefix("./").unwrap_or(p))
}

// spec: canon-kit/SPEC.md §lib/spec.sh — `_spec_prune_kit_roots`: exclude a file whose
// absolute path falls under a kit root that is a strict descendant of the scan root. An
// ancestor root never prunes; `walk::prune_dirs` is the other, by-directory-name rule.
pub fn prune_kit_roots(root: &str, files: Vec<PathBuf>) -> Result<Vec<PathBuf>, String> {
    if knob("CANON_KIT_SCAN_KIT_ROOTS")? == "1" {
        return Ok(files);
    }
    let root_abs = root_to_abs(root);
    let mut roots: Vec<String> = Vec::new();
    for r in walk::kit_roots()? {
        if r.is_empty() {
            continue;
        }
        let rabs = root_to_abs(&r);
        if rabs.starts_with(&format!("{}/", root_abs)) {
            roots.push(rabs);
        }
    }
    if roots.is_empty() {
        return Ok(files);
    }
    Ok(files
        .into_iter()
        .filter(|f| {
            let fabs = file_to_abs(&f.display().to_string());
            !roots.iter().any(|r| fabs.starts_with(&format!("{}/", r)))
        })
        .collect())
}

// spec: canon-kit/SPEC.md §lib/spec.sh — `spec_manifest_files`, all three branches in the
// one place the cohort calls. The `CLAUDE.md` find is neither `templates/`-filtered nor
// kit-root pruned while the other two are; the asymmetry is reproduced, not tidied.
pub fn manifest_files(root: &str) -> Result<Vec<PathBuf>, String> {
    let mut out: Vec<PathBuf> = Vec::new();
    let manifest_globs = knob_array("CANON_KIT_MANIFEST_FILES")?;
    let rootp = Path::new(root);
    if !manifest_globs.is_empty() {
        for f in walk::glob_files(rootp, &manifest_globs)? {
            if f.is_file() {
                out.push(f);
            }
        }
    } else {
        let spec_name = knob("CANON_KIT_SPEC_NAME")?;
        let specs = walk::find_named(rootp, &[spec_name.as_str()])?
            .into_iter()
            .filter(|p| !under_templates(&p.display().to_string()))
            .collect();
        out.extend(prune_kit_roots(root, specs)?);

        let readmes = walk::find_named(rootp, &["README.md"])?
            .into_iter()
            .filter(|p| !under_templates(&p.display().to_string()))
            .collect();
        out.extend(prune_kit_roots(root, readmes)?);

        out.extend(walk::find_named(rootp, &["CLAUDE.md"])?);
    }
    let prose_globs = knob_array("CANON_KIT_PROSE_SURFACE_GLOBS")?;
    if !prose_globs.is_empty() {
        for f in walk::glob_files(rootp, &prose_globs)? {
            if f.is_file() && slot_free(&f)? {
                out.push(f);
            }
        }
    }
    Ok(out)
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the manifest set as the members that sort it read
// it: `spec_manifest_files … | sed 's#^\./##' | sort -u`
pub fn manifest_files_sorted_stripped(root: &str) -> Result<Vec<String>, String> {
    let mut v: Vec<String> = manifest_files(root)?
        .into_iter()
        .map(|p| strip_dot_slash(&p.display().to_string()))
        .collect();
    v.sort();
    v.dedup();
    Ok(v)
}

pub fn spec_name() -> Result<String, String> {
    knob("CANON_KIT_SPEC_NAME")
}

// spec: canon-kit/SPEC.md §check-md-refs — `realpath -m --relative-to=. -- <p>`, shared by
// the two members that resolve a doc-relative token so neither carries a path algebra of
// its own
pub fn relative_to_cwd(p: &str) -> String {
    let cwd = std::env::current_dir()
        .map(|c| c.display().to_string())
        .unwrap_or_else(|_| "/".to_string());
    let abs = if p.starts_with('/') {
        p.to_string()
    } else {
        format!("{}/{}", cwd, p)
    };
    let norm = normalize(&abs);
    let base = normalize(&cwd);
    if norm == base {
        return ".".to_string();
    }
    let n: Vec<&str> = norm.split('/').filter(|s| !s.is_empty()).collect();
    let bs: Vec<&str> = base.split('/').filter(|s| !s.is_empty()).collect();
    let common = n.iter().zip(bs.iter()).take_while(|(a, b)| a == b).count();
    let mut parts: Vec<String> = Vec::new();
    for _ in common..bs.len() {
        parts.push("..".to_string());
    }
    for seg in &n[common..] {
        parts.push((*seg).to_string());
    }
    if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — one normaliser, in the module that owns the bridged
// root spelling this rule exists for; a second copy here would be the drift the kit-roots
// cohort's own criterion-6 discharge argues against
fn normalize(abs: &str) -> String {
    walk::normalize_abs(abs)
}

pub fn knob_pub(name: &str) -> Result<String, String> {
    knob(name)
}

pub fn knob_array_pub(name: &str) -> Result<Vec<String>, String> {
    knob_array(name)
}

pub fn strip_dot_slash(s: &str) -> String {
    s.strip_prefix("./").unwrap_or(s).to_string()
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the paragraph accumulator: physical lines in,
// a logical window back out
#[derive(Default)]
pub struct Para {
    pub fnr: Vec<usize>,
    pub line: Vec<String>,
}

impl Para {
    fn reset(&mut self) {
        self.fnr.clear();
        self.line.clear();
    }
    fn add(&mut self, fnr: usize, text: &str) {
        self.fnr.push(fnr);
        self.line.push(text.to_string());
    }
    pub fn len(&self) -> usize {
        self.line.len()
    }
    // spec: canon-kit/SPEC.md §lib/spec.sh — `_sk_join(lo, hi)`, one-based inclusive
    pub fn join(&self, lo: usize, hi: usize) -> String {
        let mut s = String::new();
        for k in lo..=hi {
            if k > lo {
                s.push(' ');
            }
            s.push_str(&self.line[k - 1]);
        }
        s
    }
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the manifest-prose walk driver: fence tracking,
// the blank-line paragraph reset, and the per-site exempt window (the line or the one
// above), whose marker is a kit literal and so a substring test
// spec: canon-kit/SPEC.md §lib/spec.sh — the two hooks the awk driver calls, as one sink
// rather than two callbacks: the member's finding list is the state both write, and a pair
// of closures cannot share it
pub trait ProseSink {
    fn on_line(&mut self, _file: &str, _fnr: usize, _raw: &str) {}
    fn on_pflush(&mut self, _file: &str, _para: &Para) {}
}

pub fn walk_prose(
    files: &[String],
    exempt: &str,
    sink: &mut dyn ProseSink,
) -> Result<(), String> {
    walk_prose_multi(files, &[exempt], sink)
}

// spec: canon-kit/SPEC.md §check-manifest-count — the same walk over more than one per-site
// marker, so a member whose ban has a second sanctioned discharge names both windows rather
// than carrying a second walk of its own
pub fn walk_prose_multi(
    files: &[String],
    exempts: &[&str],
    sink: &mut dyn ProseSink,
) -> Result<(), String> {
    let marked = |s: &str| exempts.iter().any(|e| s.contains(e));
    let mut para = Para::default();
    let mut curfile = String::new();
    for f in files {
        let text = read_text(Path::new(f))?;
        let mut fence = false;
        let mut prev = String::new();
        let mut first = true;
        for (idx, raw) in text.lines().enumerate() {
            let fnr = idx + 1;
            if first {
                // spec: canon-kit/SPEC.md §lib/spec.sh — the FNR==1 flush closes the
                // *previous* file's paragraph, so it still reports under that file's name
                sink.on_pflush(&curfile, &para);
                para.reset();
                fence = false;
                prev = String::new();
                first = false;
            }
            curfile = f.clone();
            if is_fence_line(raw) {
                sink.on_pflush(&curfile, &para);
                para.reset();
                fence = !fence;
                prev = raw.to_string();
                continue;
            }
            if fence || marked(raw) || marked(&prev) || is_blank(raw) {
                sink.on_pflush(&curfile, &para);
                para.reset();
                prev = raw.to_string();
                continue;
            }
            sink.on_line(&curfile, fnr, raw);
            para.add(fnr, raw);
            prev = raw.to_string();
        }
    }
    sink.on_pflush(&curfile, &para);
    para.reset();
    Ok(())
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the fence and blank-line shapes the awk driver
// tests, byte-wise on POSIX space as awk matches them
pub fn is_fence_line(line: &str) -> bool {
    lstrip_space(line.as_bytes()).starts_with(b"```")
}

pub fn is_blank(line: &str) -> bool {
    lstrip_space(line.as_bytes()).is_empty()
}

fn lstrip_space(b: &[u8]) -> &[u8] {
    let mut i = 0usize;
    while i < b.len() && is_space(b[i]) {
        i += 1;
    }
    &b[i..]
}

// spec: canon-kit/SPEC.md §lib/spec.sh — the cardinal alternation `SPEC_COUNT_CARDINAL_RE`
// fixes for the count grammar below
const CARDINAL_WORDS: &[&str] = &[
    "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve",
];

// spec: canon-kit/SPEC.md §check-measured-claim — the same alternation read as a value, so
// arm C's "does this cardinal appear in the claim" and check-manifest-count's "is there a
// cardinal here" are one grammar with one spelling
pub fn cardinal_word_value(w: &str) -> Option<String> {
    CARDINAL_WORDS
        .iter()
        .position(|c| *c == w)
        .map(|i| (i + 2).to_string())
}

// spec: canon-kit/SPEC.md §check-measured-claim — the marker's opening literal, shared by the
// gate that reads it and by check-manifest-count, whose ban the marker discharges
pub const MEASURED_MARKER: &str = "<!-- measured:";

pub struct CountGrammar {
    nouns: Vec<String>,
    wedge: usize,
    phrases: Vec<String>,
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}

fn is_alnum(c: u8) -> bool {
    c.is_ascii_alphanumeric()
}

impl CountGrammar {
    pub fn resolve() -> Result<Self, String> {
        let nouns: Vec<String> = knob_array("CANON_KIT_COUNT_COLLECTIONS")?
            .into_iter()
            .filter(|n| !n.is_empty())
            .map(|n| n.to_ascii_lowercase())
            .collect();
        let wedge: usize = knob("CANON_KIT_COUNT_WEDGE_WORDS")?
            .parse()
            .map_err(|_| "CANON_KIT_COUNT_WEDGE_WORDS is not a number".to_string())?;
        let phrases: Vec<String> = knob_array("CANON_KIT_COUNT_ALLOWED_PHRASES")?
            .into_iter()
            .filter(|p| !p.is_empty())
            .map(|p| p.to_ascii_lowercase())
            .collect();
        Ok(CountGrammar {
            nouns,
            wedge,
            phrases,
        })
    }

    // spec: canon-kit/SPEC.md §check-manifest-count — `sk_count_hit`: the quantifier shape
    // first, then the range shape; a cardinal inside inline code is a meta-reference and is
    // blanked before either runs.
    pub fn hit(&self, text: &str) -> Option<String> {
        // spec: canon-kit/SPEC.md §check-manifest-count — byte-wise because awk's
        // substr/tolower are; a char-wise port shifts every offset on a multi-byte glyph
        let scan = strip_inline_code(text.as_bytes());
        let low: Vec<u8> = scan.iter().map(|c| c.to_ascii_lowercase()).collect();
        if let Some(s) = self.span(&low, &scan, true) {
            return Some(s);
        }
        self.span(&low, &scan, false)
    }

    // spec: canon-kit/SPEC.md §check-manifest-count — `_sk_span`: leftmost-longest, then
    // the boundary rule, then the exemptions
    fn span(&self, lb: &[u8], scan: &[u8], quantifier: bool) -> Option<String> {
        let mut start = 0usize;
        while start < lb.len() {
            let m = (start..lb.len()).find_map(|i| {
                let e = if quantifier {
                    self.match_quantifier(lb, i)
                } else {
                    self.match_range(lb, i)
                };
                e.map(|end| (i, end))
            });
            let (ms, me) = m?;
            if self.accept(lb, ms, me, quantifier) {
                return Some(String::from_utf8_lossy(&scan[ms..me]).into_owned());
            }
            start = ms + 1;
        }
        None
    }

    fn accept(&self, lb: &[u8], ms: usize, me: usize, quantifier: bool) -> bool {
        let bc = if ms > 0 { lb[ms - 1] } else { b' ' };
        let ac = if me < lb.len() { lb[me] } else { b' ' };
        if is_alnum(bc) {
            return false;
        }
        if is_alnum(ac) || ac == b'-' {
            return false;
        }
        if self.phrase_exempt(lb, ms, me) {
            return false;
        }
        if !quantifier {
            return true;
        }
        let prefix = &lb[..ms];
        let m = &lb[ms..me];
        let suffix = &lb[me..];
        if ends_with_comparator(prefix) {
            return false;
        }
        if trimmed_space_end_ends_with(prefix, b"all but") {
            return false;
        }
        if prefix_of_partitive(prefix) {
            return false;
        }
        if contains_bare_of(m) {
            return false;
        }
        if suffix_is_per(suffix) {
            return false;
        }
        true
    }

    fn phrase_exempt(&self, lb: &[u8], ms: usize, me: usize) -> bool {
        for p in &self.phrases {
            let pb = p.as_bytes();
            let mut from = 0usize;
            while from + pb.len() <= lb.len() {
                match find_sub(&lb[from..], pb) {
                    Some(off) => {
                        let pp = from + off;
                        if ms >= pp && me <= pp + pb.len() {
                            return true;
                        }
                        from = pp + 1;
                    }
                    None => break,
                }
            }
        }
        false
    }

    // spec: canon-kit/SPEC.md §check-manifest-count — `spec_count_quantifier_re`, whose
    // wedge groups are optional, so bare adjacency is this shape's zero-wedge case
    fn match_quantifier(&self, b: &[u8], i: usize) -> Option<usize> {
        let mut best: Option<usize> = None;
        for c_end in cardinal_ends(b, i) {
            self.wedge_then_noun(b, c_end, 0, &mut best);
        }
        best
    }

    fn wedge_then_noun(&self, b: &[u8], p: usize, k: usize, best: &mut Option<usize>) {
        for s in class_ends(b, p, is_space) {
            for n in &self.nouns {
                if b[s..].starts_with(n.as_bytes()) {
                    let e = s + n.len();
                    if best.map(|x| e > x).unwrap_or(true) {
                        *best = Some(e);
                    }
                }
            }
            if k < self.wedge {
                for a in class_ends(b, s, |c| is_alnum(c) || c == b'_' || c == b'-') {
                    self.wedge_then_noun(b, a, k + 1, best);
                }
            }
        }
    }

    // spec: canon-kit/SPEC.md §check-manifest-count — `spec_count_range_re`
    fn match_range(&self, b: &[u8], i: usize) -> Option<usize> {
        let mut best: Option<usize> = None;
        for n in &self.nouns {
            if !b[i..].starts_with(n.as_bytes()) {
                continue;
            }
            for s in class_ends(b, i + n.len(), is_space) {
                for d1 in class_ends(b, s, |c| c.is_ascii_digit()) {
                    if b.get(d1) != Some(&b'-') {
                        continue;
                    }
                    for d2 in class_ends(b, d1 + 1, |c| c.is_ascii_digit()) {
                        if best.map(|x| d2 > x).unwrap_or(true) {
                            best = Some(d2);
                        }
                    }
                }
            }
        }
        best
    }
}

fn class_ends(b: &[u8], i: usize, f: impl Fn(u8) -> bool) -> Vec<usize> {
    let mut n = i;
    while n < b.len() && f(b[n]) {
        n += 1;
    }
    (i + 1..=n).rev().collect()
}

fn cardinal_ends(b: &[u8], i: usize) -> Vec<usize> {
    let mut out: Vec<usize> = class_ends(b, i, |c| c.is_ascii_digit());
    for w in CARDINAL_WORDS {
        if b[i..].starts_with(w.as_bytes()) {
            out.push(i + w.len());
        }
    }
    out
}

fn find_sub(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > hay.len() {
        return None;
    }
    (0..=hay.len() - needle.len()).find(|&i| &hay[i..i + needle.len()] == needle)
}

fn trim_space_end(b: &[u8]) -> &[u8] {
    let mut e = b.len();
    while e > 0 && is_space(b[e - 1]) {
        e -= 1;
    }
    &b[..e]
}

fn trimmed_space_end_ends_with(prefix: &[u8], lit: &[u8]) -> bool {
    trim_space_end(prefix).ends_with(lit)
}

// spec: canon-kit/SPEC.md §check-manifest-count — a threshold, not a total: the comparator
// prefix `_sk_span` exempts
fn ends_with_comparator(prefix: &[u8]) -> bool {
    let t = trim_space_end(prefix);
    const LITS: &[&str] = &[
        "≥",
        "≤",
        ">",
        "<",
        "at least",
        "at most",
        "up to",
        "more than",
        "fewer than",
    ];
    LITS.iter().any(|l| t.ends_with(l.as_bytes()))
}

// spec: canon-kit/SPEC.md §check-manifest-count — a partition, not a total. The shape's
// space run is required, so the trailing whitespace belongs to the match, not to trim.
fn prefix_of_partitive(prefix: &[u8]) -> bool {
    let mut e = prefix.len();
    let mut saw_space = false;
    while e > 0 && is_space(prefix[e - 1]) {
        e -= 1;
        saw_space = true;
    }
    if !saw_space {
        return false;
    }
    let mut head = &prefix[..e];
    if head.ends_with(b"the") {
        let h2 = &head[..head.len() - 3];
        let mut e2 = h2.len();
        let mut saw2 = false;
        while e2 > 0 && is_space(h2[e2 - 1]) {
            e2 -= 1;
            saw2 = true;
        }
        if saw2 {
            head = &h2[..e2];
        }
    }
    if !head.ends_with(b"of") {
        return false;
    }
    let before = head.len() - 2;
    before == 0 || !is_alnum(head[before - 1])
}

// spec: canon-kit/SPEC.md §check-manifest-count — a proportion, not a total
fn contains_bare_of(m: &[u8]) -> bool {
    let mut i = 0usize;
    while i + 2 <= m.len() {
        if &m[i..i + 2] == b"of" {
            let ok_before = i == 0 || is_space(m[i - 1]);
            let ok_after = i + 2 == m.len() || is_space(m[i + 2]);
            if ok_before && ok_after {
                return true;
            }
        }
        i += 1;
    }
    false
}

// spec: canon-kit/SPEC.md §check-manifest-count — a rate, not a total
fn suffix_is_per(suffix: &[u8]) -> bool {
    let mut i = 0usize;
    while i < suffix.len() && is_space(suffix[i]) {
        i += 1;
    }
    if i == 0 {
        return false;
    }
    if !suffix[i..].starts_with(b"per") {
        return false;
    }
    let after = i + 3;
    after == suffix.len() || is_space(suffix[after])
}

// spec: canon-kit/SPEC.md §check-manifest-count — a cardinal in inline code is a
// meta-reference, not a restated total
pub fn strip_inline_code(b: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'`' {
            if let Some(off) = b[i + 1..].iter().position(|&c| c == b'`') {
                i = i + 1 + off + 1;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    out
}

// spec: canon-kit/SPEC.md §check-manifest-count — `sk_para_wrapped`: the first total whose
// span crosses a line boundary, reported at the span's first physical line. A same-line
// span returns nothing, because the per-line scan already owns it.
pub fn para_wrapped(g: &CountGrammar, para: &Para) -> Option<(usize, String)> {
    if para.len() < 2 {
        return None;
    }
    let mut comp_k = 0usize;
    let mut hit = String::new();
    for k in 1..=para.len() {
        if let Some(h) = g.hit(&para.join(1, k)) {
            comp_k = k;
            hit = h;
            break;
        }
    }
    if comp_k == 0 {
        return None;
    }
    let mut start_k = 1usize;
    for k in 2..=comp_k {
        if g.hit(&para.join(k, comp_k)).is_some() {
            start_k = k;
        } else {
            break;
        }
    }
    if start_k == comp_k {
        return None;
    }
    Some((para.fnr[start_k - 1], hit))
}
