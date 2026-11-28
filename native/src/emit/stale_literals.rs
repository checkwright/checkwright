// spec: gate-sdk/SPEC.md §The since-base corpus — `--emit stale-literals`, the test lines that may
// assert a string the branch removed: a worklist, never a verdict
use crate::exit::Fault;
use crate::literals::{self, Floor, Literal};
use crate::since_base::{self, Base};
use crate::walk;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

pub const KNOBS: &[&str] = &[since_base::KNOB, "GATE_SDK_TEST_GLOBS", CHARS_KNOB, LETTERS_KNOB];

const CHARS_KNOB: &str = "GATE_SDK_STALE_LITERAL_CHARS";
const LETTERS_KNOB: &str = "GATE_SDK_STALE_LITERAL_LETTERS";

pub const USAGE: &str = "usage: --emit stale-literals [--base <ref>] [--tools <dir>]";

const NAME: &str = "stale-literals";

struct Args {
    base: Option<String>,
    tools: Option<String>,
}

fn parse(args: &[String]) -> Result<Args, Fault> {
    let mut a = Args { base: None, tools: None };
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let slot = match flag.as_str() {
            "--base" => &mut a.base,
            "--tools" => &mut a.tools,
            _ => return Err(Fault::usage(USAGE)),
        };
        match it.next() {
            Some(v) if slot.is_none() && !v.is_empty() => *slot = Some(v.clone()),
            _ => return Err(Fault::usage(USAGE)),
        }
    }
    Ok(a)
}

// spec: gate-sdk/SPEC.md §The since-base corpus — a source site: the merge base's path and line
// a changed fragment was removed from
type Site = (String, usize);

fn read_text(path: &Path) -> Result<Option<String>, Fault> {
    if !path.is_file() {
        return Ok(None);
    }
    let bytes = std::fs::read(path).map_err(|e| Fault::input(format!("cannot read {}: {}", path.display(), e)))?;
    Ok((!walk::binary(&bytes)).then(|| String::from_utf8_lossy(&bytes).into_owned()))
}

fn shipped(test_from: Option<usize>, line: usize) -> bool {
    !matches!(test_from, Some(t) if line >= t)
}

// spec: gate-sdk/SPEC.md §The since-base corpus — a literal is removed where any line it spans is,
// so an edit inside a multi-line literal is collected
fn touched(removed: &[(usize, usize)], lit: &Literal) -> bool {
    let last = lit.line + lit.body.matches('\n').count();
    (lit.line..=last).any(|l| since_base::within(removed, l))
}

// spec: gate-sdk/SPEC.md §The since-base corpus — each fragment on a non-test line the branch
// removed, with every site it was removed from
fn removed(base: &Base, changed: &[String], hunks: &HashMap<String, since_base::Hunks>, globs: &[String], floor: Floor) -> Result<BTreeMap<String, BTreeSet<Site>>, Fault> {
    let mut out: BTreeMap<String, BTreeSet<Site>> = BTreeMap::new();
    for p in changed {
        let h = &hunks[p];
        if h.removed.is_empty() {
            continue;
        }
        let bytes = base.blob(p)?;
        if walk::binary(&bytes) {
            continue;
        }
        let text = String::from_utf8_lossy(&bytes);
        let test_from = walk::test_code(p, &text, globs);
        for lit in literals::literals(&text, literals::lang(p)) {
            if !shipped(test_from, lit.line) || !touched(&h.removed, &lit) {
                continue;
            }
            for f in literals::fragments(lit.body, lit.escapes, floor) {
                out.entry(f.to_string()).or_default().insert((p.clone(), lit.line));
            }
        }
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The since-base corpus — a fragment some non-test working-tree line still
// holds is no change; the branch's own additions are working-tree lines, so a moved string is held
fn drop_held(base: &Base, corpus: &[String], globs: &[String], fragments: &mut BTreeMap<String, BTreeSet<Site>>) -> Result<(), Fault> {
    for p in corpus {
        if fragments.is_empty() {
            break;
        }
        let Some(text) = read_text(&base.path(p))? else { continue };
        let test_from = walk::test_code(p, &text, globs);
        for (i, line) in text.lines().enumerate() {
            if !shipped(test_from, i + 1) {
                break;
            }
            fragments.retain(|f, _| !line.contains(f.as_str()));
        }
    }
    Ok(())
}

fn overlaps(a: &str, b: &str) -> bool {
    a.contains(b) || b.contains(a)
}

// spec: gate-sdk/SPEC.md §The since-base corpus — a test line's first source site among the changed
// fragments its listable literals overlap
fn listed(text: &str, path: &str, test_from: usize, added: &[(usize, usize)], changed: &BTreeMap<String, BTreeSet<Site>>, floor: Floor) -> BTreeMap<usize, Site> {
    let mut out: BTreeMap<usize, Site> = BTreeMap::new();
    let lits: Vec<Literal> = literals::literals(text, literals::lang(path));
    for lit in lits.iter().filter(|l| l.line >= test_from && !l.message && !since_base::within(added, l.line)) {
        for f in literals::fragments(lit.body, lit.escapes, floor) {
            for site in changed.iter().filter(|(c, _)| overlaps(f, c)).filter_map(|(_, s)| s.first()) {
                let slot = out.entry(lit.line).or_insert_with(|| site.clone());
                if site < slot {
                    *slot = site.clone();
                }
            }
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §The since-base corpus — the counting floor, both halves the consumer's
fn floor() -> Result<Floor, Fault> {
    let read = |name: &str, parse: fn(&str) -> Option<usize>| -> Result<usize, Fault> {
        let raw = walk::knob_scalar(name).map_err(Fault::refused)?;
        parse(&raw).ok_or_else(|| Fault::refused(format!("{} must be an integer count (got '{}')", name, raw)))
    };
    Ok(Floor {
        chars: read(CHARS_KNOB, crate::knobs::gate_sdk::positive)?,
        letters: read(LETTERS_KNOB, crate::knobs::gate_sdk::count)?,
    })
}

pub fn emit(args: &[String]) -> Result<String, Fault> {
    let args = parse(args)?;
    inner(&args).map_err(|e| e.prefixed(NAME))
}

fn inner(args: &Args) -> Result<String, Fault> {
    if let Some(t) = &args.tools {
        if !Path::new(t).is_dir() {
            return Err(Fault::refused(format!("--tools: '{}' is not a directory", t)));
        }
    }
    let base = since_base::resolve(args.base.as_deref())?;
    let globs = walk::knob_words("GATE_SDK_TEST_GLOBS").map_err(Fault::refused)?;
    let floor = floor()?;
    let changed = base.changed(&[])?;
    let mut hunks: HashMap<String, since_base::Hunks> = HashMap::new();
    for p in &changed {
        hunks.insert(p.clone(), base.hunks(p)?);
    }
    let mut fragments = removed(&base, &changed, &hunks, &globs, floor)?;
    let tracked = base.tracked()?;
    if !fragments.is_empty() {
        let mut corpus = tracked.clone();
        corpus.extend(base.untracked()?);
        drop_held(&base, &corpus, &globs, &mut fragments)?;
    }

    let mut rows: Vec<(String, usize, Site)> = Vec::new();
    if !fragments.is_empty() {
        let none = Vec::new();
        for p in &tracked {
            let Some(text) = read_text(&base.path(p))? else { continue };
            let Some(test_from) = walk::test_code(p, &text, &globs) else { continue };
            let added = hunks.get(p).map_or(&none, |h| &h.added);
            rows.extend(listed(&text, p, test_from, added, &fragments, floor).into_iter().map(|(l, s)| (p.clone(), l, s)));
        }
        if let Some(dir) = &args.tools {
            let files = walk::find_with_prune(Path::new(dir), &|n: &str| n == ".git").map_err(Fault::input)?;
            for f in files {
                let p = f.display().to_string().replace('\\', "/");
                let Some(text) = read_text(&f)? else { continue };
                rows.extend(listed(&text, &p, 1, &[], &fragments, floor).into_iter().map(|(l, s)| (p.clone(), l, s)));
            }
        }
    }

    let mut out = String::new();
    for (test, line, (src, at)) in &rows {
        out.push_str(&format!("LITERAL {}:{} (was {}:{})\n", test, line, src, at));
    }
    out.push_str(&format!(
        "{}: {} test line(s) listed against merge base {}, {} changed fragment(s)\n",
        NAME,
        rows.len(),
        base.short(),
        fragments.len()
    ));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §The non-gate arm — an unknown flag, a missing value or a repeated
    // flag refuses with the usage before any git is read
    #[test]
    fn a_malformed_argv_refuses_with_the_usage() {
        let _knobs = crate::knobenv::lock();
        for args in [&["--write"][..], &["--base"], &["--base", "a", "--base", "b"], &["main"]] {
            let argv: Vec<String> = args.iter().map(|a| a.to_string()).collect();
            let fault = emit(&argv).expect_err("a malformed argv must refuse");
            assert_eq!(fault.message, USAGE, "{:?}", args);
            assert_eq!(fault.status(), crate::exit::USAGE, "{:?}", args);
        }
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — an unchanged test line's literal overlapping
    // a changed fragment either way is listed; a message, an added line and test-code-free text are not
    #[test]
    fn a_test_line_lists_by_overlap_either_way() {
        let changed: BTreeMap<String, BTreeSet<Site>> = [
            ("cannot read the config".to_string(), [("src/a.rs".to_string(), 7)].into()),
            ("unknown flag given".to_string(), [("src/b.rs".to_string(), 2), ("src/a.rs".to_string(), 9)].into()),
        ]
        .into();
        let text = "fn f() {}\n#[cfg(test)]\nassert!(e.contains(\"read the config\"));\nassert!(x, \"cannot read the config\");\nlet s = \"error: unknown flag given here\";\nlet t = \"cannot read the config\";\n";
        let got = listed(text, "src/c.rs", 2, &[(6, 7)], &changed, Floor { chars: 8, letters: 4 });
        assert_eq!(got, BTreeMap::from([(3, ("src/a.rs".to_string(), 7)), (5, ("src/a.rs".to_string(), 9))]));
    }

    // spec: gate-sdk/SPEC.md §The since-base corpus — an edit on any line a literal spans removes it,
    // a multi-line raw string and a triple-quoted one alike
    #[test]
    fn a_literal_is_removed_on_any_line_it_spans() {
        let rs = "let a = 1;\nlet s = r#\"first line\nsecond line\nthird\"#;\nlet b = 2;\n";
        let py = "a = 1\ns = \"\"\"first\nsecond\n\"\"\"\nb = 2\n";
        for (text, path) in [(rs, "a.rs"), (py, "a.py")] {
            let lits = literals::literals(text, literals::lang(path));
            let lit = lits.iter().find(|l| l.body.contains("first")).expect("the literal");
            assert_eq!(lit.line, 2, "{}", path);
            assert!(touched(&[(3, 4)], lit) && touched(&[(2, 3)], lit), "{}", path);
            assert!(!touched(&[(1, 2)], lit) && !touched(&[(5, 6)], lit), "{}", path);
        }
    }
}
