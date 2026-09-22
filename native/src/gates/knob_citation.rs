// spec: canon-kit/SPEC.md §check-knob-citation — no kit knob stated with its value in
// manifest prose outside the owning kit's SPEC
use crate::spec;
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-knob-citation: {}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §check-knob-citation — the knob-token vocabulary is derived from
// the kit roots, never listed: two prefix forms per root map to the owning kit, so the gate
// ships no term list and the provenance seam is untouched
// spec: gate-sdk/SPEC.md §Layout and configuration — the prefix derives from the basename, which
// either spelling yields alike, while the pair's other element is a path: joined in `is_owner`
// against a walked file and named to the user as the value's home, so it takes `kit_roots`
fn prefix_pairs() -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for kr in walk::kit_roots()? {
        let kr = kr.trim_end_matches('/');
        if kr.is_empty() {
            continue;
        }
        let base = kr.rsplit('/').next().unwrap_or(kr);
        out.push((format!("{}_", base.to_ascii_uppercase().replace('-', "_")), kr.to_string()));
        if let Some(stem) = base.strip_suffix("-kit") {
            out.push((
                format!("{}_", stem.to_ascii_uppercase().replace('-', "_")),
                kr.to_string(),
            ));
        }
    }
    Ok(out)
}

// spec: canon-kit/SPEC.md §check-knob-citation — `CANON_KIT_KNOB_CITATION_REACH`: how far a
// "default" marker may follow the token it binds
#[derive(Clone, Copy, Debug, PartialEq)]
enum Reach {
    Count(usize),
    Sentence,
    Off,
}

fn positive(raw: &str) -> Option<usize> {
    raw.parse::<usize>().ok().filter(|&n| n > 0 && raw.bytes().all(|b| b.is_ascii_digit()))
}

fn reach() -> Result<Reach, String> {
    let raw = spec::knob_pub("CANON_KIT_KNOB_CITATION_REACH")?;
    match raw.as_str() {
        "sentence" => Ok(Reach::Sentence),
        "off" => Ok(Reach::Off),
        _ => positive(&raw).map(Reach::Count).ok_or_else(|| {
            format!("CANON_KIT_KNOB_CITATION_REACH must be a positive integer, sentence or off (got '{}')", raw)
        }),
    }
}

fn span() -> Result<usize, String> {
    let raw = spec::knob_pub("CANON_KIT_KNOB_CITATION_LITERAL_SPAN")?;
    positive(&raw)
        .ok_or_else(|| format!("CANON_KIT_KNOB_CITATION_LITERAL_SPAN must be a positive integer (got '{}')", raw))
}

struct Sink {
    pairs: Vec<(String, String)>,
    spec_name: String,
    reach: Reach,
    span: usize,
    out: Vec<String>,
}

impl Sink {
    fn has_prefix(&self, t: &str) -> bool {
        self.pairs.iter().any(|(p, _)| t.starts_with(p.as_str()))
    }

    fn is_owner(&self, file: &str, kit: &str) -> bool {
        let sp = format!("{}/{}", kit, self.spec_name);
        let f = spec::strip_dot_slash(file);
        f == sp || f.ends_with(&format!("/{}", sp))
    }

    // spec: canon-kit/SPEC.md §check-knob-citation — a knob stated inside its own owning
    // SPEC is where the value belongs, so it is not a citation defect
    fn token_owner(&self, t: &str, file: &str) -> String {
        let mut matched = false;
        let mut this_file_owns = false;
        let mut owner = String::new();
        for (p, kit) in &self.pairs {
            if t.starts_with(p.as_str()) {
                matched = true;
                if owner.is_empty() {
                    owner = kit.clone();
                }
                if self.is_owner(file, kit) {
                    this_file_owns = true;
                }
            }
        }
        if !matched || this_file_owns {
            return String::new();
        }
        owner
    }
}

impl spec::ProseSink for Sink {
    fn on_line(&mut self, file: &str, fnr: usize, raw: &str) {
        // spec: canon-kit/SPEC.md §check-knob-citation — a knob named inside a `${…}` shell
        // expansion is a name citation, never a value statement of itself; blank the
        // expansions before the token scan
        let tokline = blank_expansions(raw);
        let markers = match self.reach {
            Reach::Off => Vec::new(),
            _ => spec::DefaultGrammar { is_knobname: &|t: &str| self.has_prefix(t) }
                .default_bound_at(raw, self.span),
        };
        // spec: canon-kit/SPEC.md §check-knob-citation — a token binds a marker that follows it
        // within the reach, so an unwrapped paragraph does not bind a knob to another knob's
        // default
        let rb = raw.as_bytes();
        let near = |at: usize| {
            markers.iter().any(|&m| {
                m > at
                    && match self.reach {
                        Reach::Count(n) => raw.get(at..m).is_some_and(|s| s.chars().count() <= n),
                        Reach::Sentence => !(at..m).any(|t| spec::sentence_end(rb, t)),
                        Reach::Off => false,
                    }
            })
        };
        let mut first: Option<(String, String)> = None;
        let b = tokline.as_bytes();
        let mut pos = 0usize;
        while pos < b.len() {
            let start = match (pos..b.len()).find(|&i| b[i].is_ascii_uppercase()) {
                Some(i) => i,
                None => break,
            };
            let mut j = start + 1;
            while j < b.len() && (b[j].is_ascii_uppercase() || b[j].is_ascii_digit() || b[j] == b'_')
            {
                j += 1;
            }
            let eq = j < b.len() && b[j] == b'=';
            if eq {
                j += 1;
            }
            let before = if start > 0 { b[start - 1] } else { b' ' };
            pos = j;
            if before.is_ascii_alphanumeric() || before == b'_' {
                continue;
            }
            let tok = String::from_utf8_lossy(&b[start..if eq { j - 1 } else { j }]).into_owned();
            let owner = self.token_owner(&tok, file);
            if owner.is_empty() {
                continue;
            }
            if eq {
                self.out.push(format!(
                    "  {}:{}  {} stated with an '=' value — the value belongs in {}/{}",
                    spec::strip_dot_slash(file),
                    fnr,
                    tok,
                    owner,
                    self.spec_name
                ));
                return;
            }
            if first.is_none() && near(start) {
                first = Some((tok, owner));
            }
        }
        {
            if let Some((tok, owner)) = first {
                self.out.push(format!(
                    "  {}:{}  {} stated with a default value — the value belongs in {}/{}",
                    spec::strip_dot_slash(file),
                    fnr,
                    tok,
                    owner,
                    self.spec_name
                ));
            }
        }
    }
}

fn blank_expansions(line: &str) -> String {
    let b = line.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'$' && i + 1 < b.len() && b[i + 1] == b'{' {
            if let Some(off) = b[i + 2..].iter().position(|&c| c == b'}') {
                out.resize(out.len() + off + 3, b' ');
                i = i + 2 + off + 1;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let pairs = prefix_pairs()?;
    let manifests = spec::manifest_files_sorted_stripped(root)?;
    if manifests.is_empty() {
        println!("KNOB-CITATION: clean (0 manifest file(s) found)");
        return Ok(0);
    }
    let mut sink = Sink {
        pairs,
        spec_name: spec::spec_name()?,
        reach: reach()?,
        span: span()?,
        out: Vec::new(),
    };
    spec::walk_prose(&manifests, "knob-citation-exempt:", &mut sink)?;

    if !sink.out.is_empty() {
        println!("check-knob-citation: kit knob(s) stated with a value in manifest prose outside the owning SPEC — a knob's value has one home, and a restated copy drifts silently:");
        println!();
        for l in &sink.out {
            println!("{}", l);
        }
        println!("  help: cite the knob by bare name and point at the owning kit's SPEC roster, which owns the value; a genuine local restatement takes a 'knob-citation-exempt: <reason>' comment on the line or the one above");
        return Ok(1);
    }
    if sink.reach == Reach::Off {
        println!("KNOB-CITATION: clean ({} manifest file(s); no kit knob stated with an '=' value in prose outside the owning SPEC; the default-marker leg is off under CANON_KIT_KNOB_CITATION_REACH, so a stated default was not read)", manifests.len());
        return Ok(0);
    }
    println!("KNOB-CITATION: clean ({} manifest file(s); no kit knob stated with a value in prose outside the owning SPEC)", manifests.len());
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::ProseSink;

    fn fired_at(line: &str, reach: Reach) -> usize {
        let mut s = Sink {
            pairs: vec![("QUEUE_KIT_".to_string(), "queue-kit".to_string())],
            spec_name: "SPEC.md".to_string(),
            reach,
            span: 24,
            out: Vec::new(),
        };
        s.on_line("lifecycle-kit/SPEC.md", 1, line);
        s.out.len()
    }

    // spec: canon-kit/SPEC.md §check-knob-citation — the default marker binds the knob it follows
    // within reach, never a knob it precedes or one a paragraph away; `sentence` bounds the reach
    // by a terminator and `off` binds nothing
    #[test]
    fn a_default_marker_binds_only_the_knob_it_follows_within_reach() {
        let near = "`QUEUE_KIT_ENTRY_CAP` — default `4300cp`.";
        let before = "default `1500`. A policy, `QUEUE_KIT_ENTRY_CAP`'s posture.";
        let far = format!("`QUEUE_KIT_ENTRY_CAP` is cited. {} Another knob has default `7`.", "x".repeat(120));
        let at = |l: &str| [Reach::Count(100), Reach::Sentence, Reach::Off].map(|r| fired_at(l, r));
        assert_eq!(at(near), [1, 1, 0]);
        assert_eq!(at(before), [0, 0, 0]);
        assert_eq!(at(&far), [0, 0, 0]);
    }
}
