// spec: canon-kit/SPEC.md §check-provenance-seam — no kit SPEC carries a publisher-provenance
// marker: a dated attribution, an agent-file pointer, a private-surface mention or a live queue slug
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
    if spec::knob_pub("CANON_KIT_SCAN_KIT_ROOTS")? != "1" {
        println!("PROVENANCE-SEAM: clean (CANON_KIT_SCAN_KIT_ROOTS is 0: kit roots are a dependency's, so no kit SPEC is scanned)");
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
    for r in &roots {
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

    let mut sink = Seam {
        markers: &markers,
        agent_files: &agent_files,
        private: &private,
        slugs: slugs.as_deref().unwrap_or(&[]),
        out: Vec::new(),
    };
    spec::walk_prose_multi(&files, &[], &mut sink)?;

    if !sink.out.is_empty() {
        println!("check-provenance-seam: publisher-provenance marker(s) in a kit SPEC — a kit SPEC states its rule undated and impersonally, and a pointer that resolves only in the publisher's tree is dead in every vendored copy:");
        println!();
        for l in &sink.out {
            println!("{}", l);
        }
        println!("  help: dated-attribution — delete the attribution and keep the rule; agent-file-pointer or private-surface — state the rule the pointer stood for, or cite the kit SPEC section that owns it; queue-slug — name what the slug denoted, or rename a freshly filed slug (gate-sdk/SPEC.md §The provenance seam)");
        return Ok(1);
    }
    let mut notes: Vec<&str> = Vec::new();
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
        "PROVENANCE-SEAM: clean ({} kit SPEC(s); no publisher-provenance marker{})",
        files.len(),
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

// spec: canon-kit/SPEC.md §check-provenance-seam — every lead-line slug in every section: the
// bold `- **slug**` lead of an open entry and the bare `- slug` line of a finished one
fn lead_slugs(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let Some(rest) = raw.strip_prefix("- ") else {
            continue;
        };
        let rest = rest.trim_start();
        if let Some(b) = rest.strip_prefix("**") {
            let end = b.find("**").unwrap_or(0);
            if end > 0 && is_slug(&b[..end]) {
                out.push(b[..end].to_string());
            }
        } else if is_slug(rest.trim_end()) {
            out.push(rest.trim_end().to_string());
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
        hits.sort_by_key(|h| h.0);
        for (ln, msg) in hits {
            self.out.push(format!("  {}:{}  {}", file, ln, msg));
        }
    }
}

impl Seam<'_> {
    // spec: canon-kit/SPEC.md §check-provenance-seam — arm A: an ISO date and an authority marker
    // in one sentence, the sentence ending at `.`, `?`, `!` or `;` before whitespace
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
                    if let Some((name, _)) = self.markers.iter().find(|(_, re)| re.is_match(&low)) {
                        let lead = start + sentence.len() - sentence.trim_start().len();
                        hits.push((
                            j.line_at(lead),
                            format!("dated-attribution: {} with marker '{}'", d, name),
                        ));
                    }
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

    #[test]
    fn lead_slugs_read_both_the_open_and_the_finished_lead() {
        let q = "## A\n\n- **widget-rework-unit** [x] — body\n  - **not-a-lead** nested\n\n## Done\n\n- finished-unit-slug\n- Prose line\n";
        let s = lead_slugs(q);
        assert!(s.contains(&"widget-rework-unit".to_string()));
        assert!(s.contains(&"finished-unit-slug".to_string()));
        assert!(!s.iter().any(|x| x == "Prose line" || x == "not-a-lead"));
    }
}
