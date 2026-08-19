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
fn prefix_pairs() -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for kr in walk::kit_roots_rel()? {
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

struct Sink {
    pairs: Vec<(String, String)>,
    spec_name: String,
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
        let bound = spec::DefaultGrammar { is_knobname: &|t: &str| self.has_prefix(t) }
            .default_bound(raw);
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
            if first.is_none() {
                first = Some((tok, owner));
            }
        }
        if bound {
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
                out.extend_from_slice(b"  ");
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
    println!("KNOB-CITATION: clean ({} manifest file(s); no kit knob stated with a value in prose outside the owning SPEC)", manifests.len());
    Ok(0)
}
