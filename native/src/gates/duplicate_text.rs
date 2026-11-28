// spec: canon-kit/SPEC.md §check-duplicate-text — no run of the configured word count is stated in
// two declared files
use crate::exit::Fault;
use crate::spec;
use crate::walk;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;

const GLOBS_KNOB: &str = "CANON_KIT_DUPLICATE_TEXT_GLOBS";
const EXCLUDE_KNOB: &str = "CANON_KIT_DUPLICATE_TEXT_EXCLUDE";
const MIN_KNOB: &str = "CANON_KIT_DUPLICATE_TEXT_MIN_WORDS";
const WITHIN_KNOB: &str = "CANON_KIT_DUPLICATE_TEXT_WITHIN_FILE";
const VALVE: &str = "duplicate-text-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-duplicate-text: {}", e);
            e.status()
        }
    }
}

struct Doc {
    rel: String,
    units: Vec<Vec<(String, usize)>>,
    reasonless: HashSet<usize>,
}

// spec: canon-kit/SPEC.md §check-duplicate-text — the phrase stream's units, the front-matter span
// held out, and a reasoned valve holding out its line and the one below; a reasonless one holds nothing
fn read_doc(full: &str, rel: &str) -> Result<Doc, Fault> {
    let text = spec::read_text(Path::new(full)).map_err(Fault::input)?;
    let raw: Vec<&str> = text.lines().collect();
    let mut sink = spec::UnitSink::default();
    sink.front_matter = spec::front_matter_span(&raw);
    let mut reasonless: HashSet<usize> = HashSet::new();
    for (i, l) in raw.iter().enumerate() {
        match super::task_path_claim::valve_reason(l, VALVE) {
            Some(r) if r.is_empty() => reasonless.extend([i + 1, i + 2]),
            Some(_) => sink.held.extend([i + 1, i + 2]),
            None => {}
        }
    }
    spec::walk_prose_multi(&[full.to_string()], &[], &mut sink).map_err(Fault::input)?;
    let units = sink
        .units
        .iter()
        .map(spec::phrase_words_lined)
        .filter(|w| !w.is_empty())
        .collect();
    Ok(Doc { rel: rel.to_string(), units, reasonless })
}

struct Finding {
    doc: usize,
    line: usize,
    first_doc: usize,
    first_line: usize,
    len: usize,
    quote: String,
    reasonless: bool,
}

fn window_hash(ids: &[u32]) -> u64 {
    let mut h = DefaultHasher::new();
    ids.hash(&mut h);
    h.finish()
}

// spec: canon-kit/SPEC.md §check-duplicate-text — a window's first occurrence is the earliest file's
// earliest unit; a later window matching one in another unit (another file, unless within-file is on)
// seeds a run, extended while both units agree and reported once at its full length
fn find_runs(docs: &[Doc], n: usize, within_file: bool) -> Vec<Finding> {
    let mut intern: HashMap<&str, u32> = HashMap::new();
    type Interned<'a> = (usize, &'a [(String, usize)], Vec<u32>);
    let mut units: Vec<Interned> = Vec::new();
    for (d, doc) in docs.iter().enumerate() {
        for u in &doc.units {
            let ids = u
                .iter()
                .map(|(w, _)| {
                    let next = intern.len() as u32;
                    *intern.entry(w.as_str()).or_insert(next)
                })
                .collect();
            units.push((d, u.as_slice(), ids));
        }
    }
    let mut first: HashMap<u64, (usize, usize)> = HashMap::new();
    for (g, (_, _, ids)) in units.iter().enumerate() {
        for j in 0..(ids.len() + 1).saturating_sub(n) {
            first.entry(window_hash(&ids[j..j + n])).or_insert((g, j));
        }
    }
    let mut out = Vec::new();
    for (g, (d, words, ids)) in units.iter().enumerate() {
        let mut k = 0usize;
        while k + n <= ids.len() {
            let (fg, fj) = first[&window_hash(&ids[k..k + n])];
            let (fd, fwords, fids) = &units[fg];
            if fg == g || (!within_file && fd == d) || fids[fj..fj + n] != ids[k..k + n] {
                k += 1;
                continue;
            }
            let mut len = n;
            while k + len < ids.len() && fj + len < fids.len() && ids[k + len] == fids[fj + len] {
                len += 1;
            }
            let lines = |ws: &[(String, usize)], doc: usize| ws.iter().any(|(_, l)| docs[doc].reasonless.contains(l));
            out.push(Finding {
                doc: *d,
                line: words[k].1,
                first_doc: *fd,
                first_line: fwords[fj].1,
                len,
                quote: words[k..k + n].iter().map(|w| w.0.as_str()).collect::<Vec<_>>().join(" "),
                reasonless: lines(&words[k..k + len], *d) || lines(&fwords[fj..fj + len], *fd),
            });
            k += len - n + 1;
        }
    }
    out
}

fn expand(root: &str, globs: &[String]) -> Result<Vec<(String, String)>, Fault> {
    let globs: Vec<String> = globs.iter().filter(|g| !g.is_empty()).cloned().collect();
    if globs.is_empty() {
        return Ok(Vec::new());
    }
    let mut out: Vec<(String, String)> = walk::glob_corpus(Path::new(root), &globs)
        .map_err(Fault::input)?
        .into_iter()
        .map(|p| {
            let full = p.display().to_string();
            let rel = match walk::rel_under(root, &full) {
                Some(r) => r.to_string(),
                None => spec::strip_dot_slash(&full),
            };
            (full, rel)
        })
        .collect();
    out.sort_by(|a, b| a.1.cmp(&b.1));
    out.dedup_by(|a, b| a.1 == b.1);
    Ok(out)
}

fn rule(args: &[String]) -> Result<i32, Fault> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(Fault::refused(format!("not a directory: {}", root)));
    }
    // spec: canon-kit/SPEC.md §Layout and configuration — both scalars are validated by the kit's
    // knob table before this gate reads them, so a parse failure here is a validator bug
    let raw_n = walk::knob_scalar(MIN_KNOB).map_err(Fault::refused)?;
    let n: usize = raw_n
        .parse()
        .map_err(|e| Fault::refused(format!("{} is '{}', not an integer despite table validation: {}", MIN_KNOB, raw_n, e)))?;
    let within_file = walk::knob_scalar(WITHIN_KNOB).map_err(Fault::refused)? == "on";
    let excluded: HashSet<String> = expand(root, &walk::knob_array(EXCLUDE_KNOB).map_err(Fault::refused)?)?
        .into_iter()
        .map(|f| f.1)
        .collect();
    let files: Vec<(String, String)> = expand(root, &walk::knob_array(GLOBS_KNOB).map_err(Fault::refused)?)?
        .into_iter()
        .filter(|f| !excluded.contains(&f.1))
        .collect();
    if files.is_empty() {
        println!("DUPLICATE-TEXT: clean (not applicable: 0 file(s); nothing compared)");
        return Ok(0);
    }
    let docs: Vec<Doc> = files.iter().map(|(full, rel)| read_doc(full, rel)).collect::<Result<_, _>>()?;
    let unit_count: usize = docs.iter().map(|d| d.units.len()).sum();
    let mut found = find_runs(&docs, n, within_file);
    let scope = if within_file { "two units" } else { "two files" };
    if found.is_empty() {
        println!(
            "DUPLICATE-TEXT: clean ({} file(s), {} unit(s); no run of {} words or more stated in {})",
            docs.len(),
            unit_count,
            n,
            scope
        );
        return Ok(0);
    }
    found.sort_by_key(|f| (f.doc, f.line));
    println!(
        "check-duplicate-text: {} run(s) of {} words or more stated in {}, across {} file(s):",
        found.len(),
        n,
        scope,
        docs.len()
    );
    for f in &found {
        let more = if f.len > n { " …" } else { "" };
        let valve = if f.reasonless { " (its duplicate-text-exempt valve carries no reason)" } else { "" };
        println!(
            "{}:{}: a run of {} words first stated at {}:{}: \"{}{}\"{}",
            docs[f.doc].rel, f.line, f.len, docs[f.first_doc].rel, f.first_line, f.quote, more, valve
        );
    }
    println!("  help: state it once, in the file whose tier owns it, and point to that home from every other: a link or a section citation. For a deliberate keep, tag '<!-- duplicate-text-exempt: <reason> -->' on the copy's line or the one above. The run length is {} (canon-kit/SPEC.md §check-duplicate-text).", MIN_KNOB);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(rel: &str, units: &[&[(&str, usize)]]) -> Doc {
        Doc {
            rel: rel.to_string(),
            units: units.iter().map(|u| u.iter().map(|(w, l)| (w.to_string(), *l)).collect()).collect(),
            reasonless: HashSet::new(),
        }
    }

    fn words(s: &str, line: usize) -> Vec<(&str, usize)> {
        s.split(' ').map(|w| (w, line)).collect()
    }

    // spec: canon-kit/SPEC.md §check-duplicate-text — overlapping windows are one run at its full
    // length, and a third file's copy names the earliest occurrence
    #[test]
    fn a_shared_run_is_one_finding_per_later_copy_at_its_full_length() {
        let a = words("x a b c d e y", 1);
        let b = words("a b c d e z", 4);
        let c = words("q a b c", 9);
        let docs = [doc("a.md", &[&a]), doc("b.md", &[&b]), doc("c.md", &[&c])];
        let f = find_runs(&docs, 3, false);
        assert_eq!(f.len(), 2);
        assert_eq!((f[0].doc, f[0].line, f[0].first_doc, f[0].first_line, f[0].len), (1, 4, 0, 1, 5));
        assert_eq!(f[0].quote, "a b c");
        assert_eq!((f[1].doc, f[1].first_doc, f[1].len), (2, 0, 3));
    }

    // spec: canon-kit/SPEC.md §check-duplicate-text — two units of one file pair only under
    // within-file, and a run repeated inside one unit is never read
    #[test]
    fn within_file_pairs_two_units_of_one_file_only_when_on() {
        let u1 = words("a b c d", 1);
        let u2 = words("a b c d", 3);
        let same = words("a b c a b c", 5);
        let docs = [doc("a.md", &[&u1, &u2]), doc("b.md", &[&same])];
        assert!(find_runs(&docs[..1], 3, false).is_empty());
        assert_eq!(find_runs(&docs[..1], 3, true).len(), 1);
        assert!(find_runs(&docs[1..], 3, true).is_empty());
    }

    // spec: canon-kit/SPEC.md §check-duplicate-text — a window never crosses a unit boundary
    #[test]
    fn a_run_split_across_two_units_is_no_run() {
        let a = words("a b c d e f", 1);
        let b1 = words("a b c", 3);
        let b2 = words("d e f", 4);
        let docs = [doc("a.md", &[&a]), doc("b.md", &[&b1, &b2])];
        assert!(find_runs(&docs, 4, false).is_empty());
    }
}
