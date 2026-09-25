// spec: canon-kit/SPEC.md §check-provenance-seam — no kit SPEC carries a publisher-provenance
// marker or quotes a roster one consumer configured
use crate::ere::Ere;
use crate::registry;
use crate::spec::{self, compile_pattern as compile, Para, ProseSink};
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-provenance-seam: {}", e);
            2
        }
    }
}

fn rule(_args: &[String]) -> Result<i32, String> {
    let kit_on = spec::knob_pub("CANON_KIT_SCAN_KIT_ROOTS")? == "1";
    let surface_globs: Vec<String> = spec::knob_array_pub("CANON_KIT_SEAM_SURFACE_GLOBS")?
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    if !kit_on && surface_globs.is_empty() {
        println!("PROVENANCE-SEAM: clean (CANON_KIT_SCAN_KIT_ROOTS is 0: kit roots are a dependency's, so no kit SPEC is scanned; CANON_KIT_SEAM_SURFACE_GLOBS is empty, so no seam surface is either)");
        return Ok(0);
    }
    let floor_raw = spec::knob_pub("CANON_KIT_SEAM_SLUG_MIN_LEN")?;
    let floor = match floor_raw.parse::<usize>() {
        Ok(n) if n > 0 && floor_raw.bytes().all(|b| b.is_ascii_digit()) => n,
        _ => {
            return Err(format!(
                "CANON_KIT_SEAM_SLUG_MIN_LEN must be a positive integer (got '{}') — treating as failure (not clean)",
                floor_raw
            ))
        }
    };
    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — every consumer pattern compiles before the
    // first corpus line is read
    let mut markers: Vec<(String, Ere)> = Vec::new();
    for m in spec::vocabulary("CANON_KIT_SEAM_AUTHORITY_MARKERS", "CANON_KIT_SEAM_AUTHORITY_MARKERS_EXTRA")? {
        if m.is_empty() {
            continue;
        }
        let re = compile(&m, "CANON_KIT_SEAM_AUTHORITY_MARKERS")?;
        markers.push((m, re));
    }
    if markers.is_empty() {
        return Err("CANON_KIT_SEAM_AUTHORITY_MARKERS is empty — treating as failure (not clean)".to_string());
    }
    let agent_files: Vec<String> = spec::knob_array_pub("CANON_KIT_SEAM_AGENT_FILES")?
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let private: Vec<String> = spec::knob_array_pub("CANON_KIT_SEAM_PRIVATE_SURFACES")?
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();

    let roots = walk::kit_roots()?;
    let queue = spec::knob_pub("CANON_KIT_QUEUE_FILE")?;
    let slugs: Option<Vec<String>> = if Path::new(&queue).exists() {
        let text = spec::read_text(Path::new(&queue))?;
        let mechanism = Mechanism::load(&roots)?;
        let mut v: Vec<String> = lead_slugs(&text)
            .into_iter()
            .filter(|s| s.len() >= floor)
            .filter(|s| !mechanism.names(s))
            .collect();
        v.sort();
        v.dedup();
        Some(v)
    } else {
        None
    };

    let spec_name = spec::spec_name()?;
    let mut files: Vec<String> = Vec::new();
    for r in roots.iter().filter(|_| kit_on) {
        let r = r.trim_end_matches('/');
        if r.is_empty() {
            continue;
        }
        let f = format!("{}/{}", r, spec_name);
        if Path::new(&f).exists() {
            spec::read_text(Path::new(&f))?;
            files.push(spec::strip_dot_slash(&f));
        }
    }
    // spec: canon-kit/SPEC.md §check-provenance-seam — a file both sets reach is scanned once, as
    // a kit SPEC
    let mut surfaces: Vec<String> = walk::glob_corpus(Path::new("."), &surface_globs)?
        .into_iter()
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .filter(|f| !files.contains(f))
        .collect();
    surfaces.sort();
    surfaces.dedup();
    for f in &surfaces {
        spec::read_text(Path::new(f))?;
    }
    let kit_count = files.len();
    files.extend(surfaces.iter().cloned());

    let rosters: Vec<(&str, Vec<String>)> = crate::knobs::consumer_elements()?
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().filter(|e| e.contains('/') || e.contains('=')).collect::<Vec<_>>()))
        .filter(|(_, v)| v.len() >= 2)
        .collect();

    let mut sink = Seam {
        markers: &markers,
        agent_files: &agent_files,
        private: &private,
        slugs: slugs.as_deref().unwrap_or(&[]),
        rosters: &rosters,
        surfaces: &surfaces,
        out: Vec::new(),
    };
    spec::walk_prose_multi(&files, &[], &mut sink)?;

    if !sink.out.is_empty() {
        println!("check-provenance-seam: provenance-seam finding(s) in a kit SPEC or seam surface — a published rule is stated undated and impersonally, a pointer that resolves only in the publisher's tree is dead in every vendored copy, and a roster one consumer configured is that tree's content:");
        println!();
        for l in &sink.out {
            println!("{}", l);
        }
        println!("  help: dated-attribution — delete the attribution and keep the rule; dated — state the measurement undated as the rule's ground, or spell a specimen's date YYYY-MM-DD; agent-file-pointer or private-surface — state the rule the pointer stood for, or cite the kit SPEC section that owns it; queue-slug — name what the slug denoted, or rename a freshly filed slug; hex-reference — delete the object name and state the fact it labelled; consumer-roster — name the knob rather than quote its configured roster (gate-sdk/SPEC.md §The provenance seam)");
        return Ok(1);
    }
    let mut notes: Vec<&str> = Vec::new();
    if !kit_on {
        notes.push("kit SPECs off: CANON_KIT_SCAN_KIT_ROOTS is 0");
    }
    if surface_globs.is_empty() {
        notes.push("seam surfaces off: CANON_KIT_SEAM_SURFACE_GLOBS is empty");
    }
    if private.is_empty() {
        notes.push("private-surface arm off: CANON_KIT_SEAM_PRIVATE_SURFACES is empty");
    }
    if slugs.is_none() {
        notes.push("queue-slug arm off: no queue file");
    }
    let suffix = if notes.is_empty() {
        String::new()
    } else {
        format!("; {}", notes.join("; "))
    };
    println!(
        "PROVENANCE-SEAM: clean ({} kit SPEC(s), {} seam surface(s); no publisher-provenance marker or quoted consumer roster{})",
        kit_count,
        surfaces.len(),
        suffix
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-provenance-seam — a slug equal to a mechanism name the tree
// defines is structural, not a finding: a kit root's directory name, a gate declared in the
// gates directory or a kit's `checks/`, or a `gates.list` member
struct Mechanism {
    kits: Vec<String>,
    dirs: Vec<String>,
    members: Vec<String>,
}

impl Mechanism {
    fn load(roots: &[String]) -> Result<Self, String> {
        let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
        let kits = roots
            .iter()
            .map(|r| r.trim_end_matches('/'))
            .filter_map(|r| r.rsplit('/').next())
            .map(str::to_string)
            .collect();
        let members = match std::fs::read_to_string(registry::list_path(&gates_dir)) {
            Ok(text) => registry::members(&text),
            Err(_) => Vec::new(),
        };
        Ok(Mechanism { kits, dirs: registry::resolve_dirs(&gates_dir, roots), members })
    }

    fn names(&self, slug: &str) -> bool {
        self.kits.iter().any(|k| k == slug)
            || self.members.iter().any(|m| m == slug)
            || registry::resolve(slug, &self.dirs).is_some()
    }
}

// spec: canon-kit/SPEC.md §check-provenance-seam — every entry slug in every section: the
// `### <slug>` heading of an open entry and the bare `- slug` line of a finished one
fn lead_slugs(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in text.lines() {
        if let Some(s) = spec::entry_heading_slug(raw) {
            out.push(s.to_string());
            continue;
        }
        let Some(rest) = raw.strip_prefix("- ") else {
            continue;
        };
        let rest = rest.trim();
        if is_slug(rest) {
            out.push(rest.to_string());
        }
    }
    out
}

fn is_slug(s: &str) -> bool {
    let b = s.as_bytes();
    !b.is_empty()
        && (b[0].is_ascii_lowercase() || b[0].is_ascii_digit())
        && b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}

fn word(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'-'
}

struct Seam<'a> {
    markers: &'a [(String, Ere)],
    agent_files: &'a [String],
    private: &'a [String],
    slugs: &'a [String],
    rosters: &'a [(&'a str, Vec<String>)],
    surfaces: &'a [String],
    out: Vec<String>,
}

// spec: canon-kit/SPEC.md §check-provenance-seam — the paragraph rejoined across wraps with one
// space, each byte offset mapped back to the physical line it came from
struct Joined {
    text: String,
    starts: Vec<(usize, usize)>,
}

impl Joined {
    fn new(para: &Para) -> Self {
        let mut text = String::new();
        let mut starts = Vec::new();
        for (i, l) in para.line.iter().enumerate() {
            if i > 0 {
                text.push(' ');
            }
            starts.push((text.len(), para.fnr[i]));
            text.push_str(l);
        }
        Joined { text, starts }
    }

    fn line_at(&self, off: usize) -> usize {
        let mut ln = self.starts.first().map(|s| s.1).unwrap_or(0);
        for (o, f) in &self.starts {
            if *o <= off {
                ln = *f;
            }
        }
        ln
    }
}

impl ProseSink for Seam<'_> {
    fn on_pflush(&mut self, file: &str, para: &Para) {
        if para.len() == 0 {
            return;
        }
        let j = Joined::new(para);
        let mut hits: Vec<(usize, String)> = Vec::new();
        self.dated_attribution(&j, &mut hits);
        self.agent_file_pointer(&j, &mut hits);
        self.private_surface(&j, &mut hits);
        self.queue_slug(&j, &mut hits);
        hex_reference(&j, &mut hits);
        // spec: canon-kit/SPEC.md §check-provenance-seam — a seam surface is the consumer's own
        // record of its own configuration, so the consumer-roster arm does not judge it
        if !self.surfaces.iter().any(|s| s == file) {
            self.consumer_roster(&j, para.fnr[0], &mut hits);
        }
        hits.sort_by_key(|h| h.0);
        for (ln, msg) in hits {
            self.out.push(format!("  {}:{}  {}", file, ln, msg));
        }
    }
}

impl Seam<'_> {
    // spec: canon-kit/SPEC.md §check-provenance-seam — arm A: an ISO date and an authority marker
    // in one sentence, the sentence ending at `.`, `?`, `!` or `;` before whitespace; a dated
    // sentence with no marker is the `dated` arm's
    fn dated_attribution(&self, j: &Joined, hits: &mut Vec<(usize, String)>) {
        let b = j.text.as_bytes();
        let mut start = 0usize;
        let mut i = 0usize;
        while i <= b.len() {
            let end = i == b.len()
                || (matches!(b[i], b'.' | b'?' | b'!' | b';') && b.get(i + 1).is_some_and(|c| c.is_ascii_whitespace()));
            if end {
                let stop = (i + 1).min(b.len());
                let sentence = &j.text[start..stop];
                if let Some(d) = iso_date(sentence) {
                    let low = sentence.to_ascii_lowercase();
                    let lead = start + sentence.len() - sentence.trim_start().len();
                    let msg = match self.markers.iter().find(|(_, re)| re.is_match(&low)) {
                        Some((name, _)) => format!("dated-attribution: {} with marker '{}'", d, name),
                        None => format!("dated: {}", d),
                    };
                    hits.push((j.line_at(lead), msg));
                }
                start = stop;
            }
            i += 1;
        }
    }

    // spec: canon-kit/SPEC.md §check-provenance-seam — arm B: an agent-file name followed by a
    // section citation or a possessive; a closing backtick may sit between
    fn agent_file_pointer(&self, j: &Joined, hits: &mut Vec<(usize, String)>) {
        let t = j.text.as_str();
        for name in self.agent_files {
            for (at, _) in t.match_indices(name.as_str()) {
                if at > 0 && (word(t.as_bytes()[at - 1]) || t.as_bytes()[at - 1] == b'.') {
                    continue;
                }
                let mut rest = &t[at + name.len()..];
                rest = rest.strip_prefix('`').unwrap_or(rest);
                let possessive = rest.starts_with("'s") || rest.starts_with("\u{2019}s");
                let r2 = rest.strip_prefix(',').unwrap_or(rest);
                let r2 = r2.strip_prefix(' ').unwrap_or(r2);
                if possessive || r2.starts_with('§') {
                    let shape = if possessive { "possessive" } else { "section citation" };
                    hits.push((j.line_at(at), format!("agent-file-pointer: {} ({})", name, shape)));
                }
            }
        }
    }

    // spec: canon-kit/SPEC.md §check-provenance-seam — arm C: a configured private path as a whole
    // path token, so a longer name containing it does not match
    fn private_surface(&self, j: &Joined, hits: &mut Vec<(usize, String)>) {
        let t = j.text.as_bytes();
        for p in self.private {
            for (at, _) in j.text.match_indices(p.as_str()) {
                let before_ok = at == 0 || !(word(t[at - 1]) || t[at - 1] == b'.' || t[at - 1] == b'/');
                let e = at + p.len();
                let after_ok = e >= t.len()
                    || !(word(t[e]) || t[e] == b'/' || (t[e] == b'.' && t.get(e + 1).is_some_and(|c| c.is_ascii_alphanumeric())));
                if before_ok && after_ok {
                    hits.push((j.line_at(at), format!("private-surface: {}", p)));
                }
            }
        }
    }

    // spec: canon-kit/SPEC.md §check-provenance-seam — arm D: a live queue slug with
    // `[A-Za-z0-9_-]` as word characters on both sides
    fn queue_slug(&self, j: &Joined, hits: &mut Vec<(usize, String)>) {
        let t = j.text.as_bytes();
        for s in self.slugs {
            for (at, _) in j.text.match_indices(s.as_str()) {
                let e = at + s.len();
                if (at == 0 || !word(t[at - 1])) && (e >= t.len() || !word(t[e])) {
                    hits.push((j.line_at(at), format!("queue-slug: {}", s)));
                }
            }
        }
    }
}

impl Seam<'_> {
    // spec: canon-kit/SPEC.md §check-provenance-seam — arm E: two or more distinct candidates of one
    // knob in one paragraph, each a whole inline-code span, reported at the paragraph's first line
    fn consumer_roster(&self, j: &Joined, first: usize, hits: &mut Vec<(usize, String)>) {
        let spans = code_spans(&j.text);
        for (knob, candidates) in self.rosters {
            let mut matched: Vec<&str> = candidates
                .iter()
                .filter(|c| spans.contains(&c.as_str()))
                .map(String::as_str)
                .collect();
            matched.sort();
            matched.dedup();
            if matched.len() >= 2 {
                let quoted: Vec<String> = matched.iter().map(|m| format!("`{}`", m)).collect();
                hits.push((first, format!("consumer-roster: {} {}", knob, quoted.join(", "))));
            }
        }
    }
}

// spec: canon-kit/SPEC.md §check-provenance-seam — hex-reference: a 7-40 run of lowercase hex with
// no letter, digit or `_` on either side, carrying a digit and a letter `a`-`f`
fn hex_reference(j: &Joined, hits: &mut Vec<(usize, String)>) {
    let b = j.text.as_bytes();
    let tok = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = 0usize;
    while i < b.len() {
        if !tok(b[i]) {
            i += 1;
            continue;
        }
        let s = i;
        while i < b.len() && tok(b[i]) {
            i += 1;
        }
        let run = &b[s..i];
        if (7..=40).contains(&run.len())
            && run.iter().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(c))
            && run.iter().any(u8::is_ascii_digit)
            && run.iter().any(|c| c.is_ascii_lowercase())
        {
            hits.push((j.line_at(s), format!("hex-reference: {}", &j.text[s..i])));
        }
    }
}

// spec: canon-kit/SPEC.md §check-provenance-seam — each single-backtick inline-code span's content
fn code_spans(t: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = t;
    while let Some(a) = rest.find('`') {
        let after = &rest[a + 1..];
        let Some(b) = after.find('`') else { break };
        out.push(&after[..b]);
        rest = &after[b + 1..];
    }
    out
}

// spec: canon-kit/SPEC.md §check-provenance-seam — `[0-9]{4}-[0-9]{2}-[0-9]{2}` with no digit on
// either side, a kit literal that hand-compiles
fn iso_date(s: &str) -> Option<&str> {
    let b = s.as_bytes();
    let shape = |w: &[u8]| {
        w.len() == 10
            && w.iter().enumerate().all(|(k, c)| if k == 4 || k == 7 { *c == b'-' } else { c.is_ascii_digit() })
    };
    let mut i = 0usize;
    while i + 10 <= b.len() {
        if shape(&b[i..i + 10])
            && (i == 0 || !b[i - 1].is_ascii_digit())
            && b.get(i + 10).map_or(true, |c| !c.is_ascii_digit())
        {
            return Some(&s[i..i + 10]);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_iso_date_needs_its_whole_shape_and_no_abutting_digit() {
        assert_eq!(iso_date("on 2026-09-16, ruled"), Some("2026-09-16"));
        assert_eq!(iso_date("12026-09-16"), None);
        assert_eq!(iso_date("2026-9-16"), None);
    }

    fn hexes(t: &str) -> Vec<String> {
        let j = Joined { text: t.to_string(), starts: vec![(0, 1)] };
        let mut h = Vec::new();
        hex_reference(&j, &mut h);
        h.into_iter().map(|x| x.1).collect()
    }

    #[test]
    fn a_hex_reference_needs_a_digit_a_letter_and_a_clean_boundary() {
        assert_eq!(hexes("at `a1b2c3d4` and 0fedcba9."), vec!["hex-reference: a1b2c3d4", "hex-reference: 0fedcba9"]);
        assert!(hexes("id 2147483646, word defaced, short ab12cd").is_empty());
        assert!(hexes("Xa1b2c3d4 a1b2c3d4g _a1b2c3d4 A1B2C3D4").is_empty());
        assert_eq!(hexes("uuid 1b4e28ba-2fa1").len(), 1);
    }

    #[test]
    fn code_spans_read_each_backticked_content() {
        assert_eq!(code_spans("a `x/y` and `k=v` then `open"), vec!["x/y", "k=v"]);
    }

    #[test]
    fn lead_slugs_read_both_the_open_and_the_finished_lead() {
        let q = "## A\n\n### widget-rework-unit\n\n[x]\n\n- **not-a-lead** a body list item\n\n## Done\n\n- finished-unit-slug\n- Prose line\n";
        let s = lead_slugs(q);
        assert!(s.contains(&"widget-rework-unit".to_string()));
        assert!(s.contains(&"finished-unit-slug".to_string()));
        assert!(!s.iter().any(|x| x == "Prose line" || x == "not-a-lead"));
    }
}
