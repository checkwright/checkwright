// spec: canon-kit/SPEC.md §check-prose-bounds — governed prose keeps its sentences and units
// within a bound and states a repeated rule once, under an optional per-file ceiling
use crate::exit::Fault;
use crate::spec;
use crate::walk;
use std::collections::BTreeMap;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-prose-bounds: {}", e);
            e.status()
        }
    }
}

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
struct Finding {
    line: usize,
    text: String,
}

type Sink = spec::UnitSink;

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
                let words = spec::phrase_words(&self.units[ui].text);
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
fn ceiling(path: &str) -> Result<BTreeMap<String, usize>, Fault> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        Fault::reading(
            path,
            format!("CANON_KIT_PROSE_BOUND_CEILING_FILE names '{}', which cannot be read: {} — treating as failure (not clean)", path, e),
        )
    })?;
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
            return Err(Fault::input(format!("{}:{}: '{}' is not a '<n> <path>' row with a positive count — treating as failure (not clean)", path, i + 1, l)));
        };
        if rows.insert(p.clone(), n).is_some() {
            return Err(Fault::input(format!("{}:{}: a second row for '{}' — treating as failure (not clean)", path, i + 1, p)));
        }
    }
    Ok(rows)
}

fn rule(args: &[String]) -> Result<i32, Fault> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(Fault::refused(format!("not a directory: {}", root)));
    }
    let b = Bounds {
        sentence_max: bound("CANON_KIT_PROSE_BOUND_SENTENCE_MAX").map_err(Fault::refused)?,
        paragraph_max: bound("CANON_KIT_PROSE_BOUND_PARAGRAPH_MAX").map_err(Fault::refused)?,
        repeat_words: bound("CANON_KIT_PROSE_BOUND_REPEAT_WORDS").map_err(Fault::refused)?,
        repeat_min: bound("CANON_KIT_PROSE_BOUND_REPEAT_MIN").map_err(Fault::refused)?.unwrap_or(usize::MAX),
    };
    let ceiling_file = walk::knob_scalar("CANON_KIT_PROSE_BOUND_CEILING_FILE").map_err(Fault::refused)?;
    let rows = if ceiling_file.is_empty() { None } else { Some(ceiling(&ceiling_file)?) };

    let globs: Vec<String> = walk::knob_array("CANON_KIT_PROSE_BOUND_GLOBS").map_err(Fault::refused)?
        .into_iter()
        .filter(|g| !g.is_empty())
        .collect();
    let mut files: Vec<(String, String)> = Vec::new();
    if !globs.is_empty() {
        for p in walk::glob_corpus(Path::new(root), &globs).map_err(Fault::input)? {
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
        println!("PROSE-BOUNDS: clean (not applicable: 0 governed file(s); nothing scanned)");
        return Ok(0);
    }

    let mut per_file: Vec<(String, Vec<Finding>)> = Vec::new();
    for (full, rel) in &files {
        let mut sink = Sink::default();
        spec::walk_prose(std::slice::from_ref(full), "prose-bound-exempt:", &mut sink).map_err(Fault::input)?;
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
        println!("  help: a pass that removed findings records the lowered row in the same commit, so the slack cannot hide a later regression. The worklist is 'CANON_KIT_PROSE_BOUND_CEILING_FILE= {}'.", super::door_command("--only check-prose-bounds").map_err(Fault::refused)?);
    }
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

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
