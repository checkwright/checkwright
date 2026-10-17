// spec: canon-kit/SPEC.md §check-delta-agreement — every configured delta agrees with its base on
// which items exist
use crate::ere::EreCapture;
use crate::section::heading_level;
use crate::spec;
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const PATH_KNOB: &str = "CANON_KIT_DELTA_PATH";
const BASE_DIR_KNOB: &str = "CANON_KIT_DELTA_BASE_DIR";
const BASE_ITEM_KNOB: &str = "CANON_KIT_DELTA_BASE_ITEM";
const PRESENT_KNOB: &str = "CANON_KIT_DELTA_PRESENT";
const ABSENT_KNOB: &str = "CANON_KIT_DELTA_ABSENT";
const ITEM_KNOB: &str = "CANON_KIT_DELTA_ITEM";
const REQUIRED_KNOB: &str = "CANON_KIT_DELTA_BASE_REQUIRED";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-delta-agreement: {}", e);
            2
        }
    }
}

struct Family {
    name: String,
    heading: crate::ere::Ere,
    item: EreCapture,
    present: bool,
    required: bool,
}

struct Config {
    path: EreCapture,
    base_dir: String,
    base_item: EreCapture,
    families: Vec<Family>,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let path = walk::knob_scalar(PATH_KNOB)?;
    if path.is_empty() {
        println!("DELTA-AGREEMENT: clean (no delta is configured: {} is empty)", PATH_KNOB);
        return Ok(0);
    }
    let cfg = config(
        &path,
        &walk::knob_scalar(BASE_DIR_KNOB)?,
        &walk::knob_scalar(BASE_ITEM_KNOB)?,
        &walk::knob_map(PRESENT_KNOB)?,
        &walk::knob_map(ABSENT_KNOB)?,
        &walk::knob_map(ITEM_KNOB)?,
        &walk::knob_array(REQUIRED_KNOB)?,
    )?;

    let mut deltas: Vec<(String, String)> = Vec::new();
    for p in walk::find_files(Path::new(root), &["md"])? {
        let shown = spec::strip_dot_slash(&p.display().to_string());
        let rel = walk::rel_under(root, &shown).unwrap_or(&shown).trim_start_matches('/').to_string();
        if let Some((gs, ge)) = cfg.path.capture(&rel) {
            let base = join(&cfg.base_dir, &span(&rel, gs, ge));
            deltas.push((rel, base));
        }
    }
    deltas.sort();

    let mut findings: Vec<(String, Finding)> = Vec::new();
    let mut agreed = 0usize;
    for (delta, base) in &deltas {
        let text = read(&walk::child(Path::new(root), delta))?;
        let base_file = walk::child(Path::new(root), base);
        let items = if base_file.is_file() {
            Some(base_items(&read(&base_file)?, &cfg.base_item))
        } else {
            None
        };
        let scan = scan(&text, &cfg.families, items.as_ref());
        agreed += scan.agreed;
        findings.extend(scan.findings.into_iter().map(|f| (delta.clone(), f)));
    }

    if !findings.is_empty() {
        println!("check-delta-agreement: {} delta item(s) disagree with their base:", findings.len());
        for (delta, f) in &findings {
            let base = &deltas.iter().find(|(d, _)| d == delta).map(|(_, b)| b.as_str()).unwrap_or("");
            let verdict = match f.why {
                Why::Lacks => format!("which {} lacks", base),
                Why::Carries => format!("which {} already carries", base),
                Why::NoBase => format!("and its base {} does not exist", base),
            };
            println!("  {}:{}: family `{}` names `{}`, {}", delta, f.line, f.family, f.item, verdict);
        }
        println!("  help: name the item as the base spells it, move it to the family that matches what the base holds, or add the base the delta changes.");
        return Ok(1);
    }
    println!(
        "DELTA-AGREEMENT: clean ({} delta(s), {} famil(ies), {} item(s) agree with their base)",
        deltas.len(),
        cfg.families.len(),
        agreed
    );
    Ok(0)
}

fn read(path: &Path) -> Result<String, String> {
    spec::read_text(path).map_err(|e| format!("cannot read {}: {}", path.display(), e))
}

fn join(dir: &str, rest: &str) -> String {
    let d = dir.trim_end_matches('/');
    if d.is_empty() || d == "." {
        rest.to_string()
    } else {
        format!("{}/{}", d, rest)
    }
}

// spec: canon-kit/SPEC.md §check-delta-agreement — the table validator's refusals are the one home
// of the key-set and shape rules; the gate compiles what they admit
fn config(
    path: &str,
    base_dir: &str,
    base_item: &str,
    present: &[(String, String)],
    absent: &[(String, String)],
    item: &[(String, String)],
    required: &[String],
) -> Result<Config, String> {
    let refusals = crate::knobs::canon_kit::delta_refusals(path, base_item, present, absent, item, required);
    if !refusals.is_empty() {
        return Err(refusals.join("; "));
    }
    let cap = |p: &str| EreCapture::compile(p).map_err(|e| e.to_string());
    let mut families: Vec<Family> = Vec::new();
    for (k, i) in item {
        let (heading, is_present) = match present.iter().find(|(pk, _)| pk == k) {
            Some((_, h)) => (h, true),
            None => match absent.iter().find(|(ak, _)| ak == k) {
                Some((_, h)) => (h, false),
                None => continue,
            },
        };
        families.push(Family {
            name: k.clone(),
            heading: crate::ere::Ere::compile(heading).map_err(|e| format!("{}[{}]: {}", if is_present { PRESENT_KNOB } else { ABSENT_KNOB }, k, e))?,
            item: cap(i)?,
            present: is_present,
            required: required.iter().any(|r| r == k),
        });
    }
    families.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Config {
        path: cap(path)?,
        base_dir: base_dir.to_string(),
        base_item: cap(base_item)?,
        families,
    })
}

// spec: canon-kit/SPEC.md §check-delta-agreement — an item name compares case-sensitively after
// trimming and collapsing each run of blanks to one space
fn normalize(name: &str) -> String {
    name.split([' ', '\t']).filter(|w| !w.is_empty()).collect::<Vec<_>>().join(" ")
}

// spec: gate-sdk/SPEC.md §The POSIX ERE matcher — its offsets are bytes and need not fall on a
// character boundary, so a group is read off the bytes
fn span(text: &str, s: usize, e: usize) -> String {
    String::from_utf8_lossy(&text.as_bytes()[s..e]).into_owned()
}

fn captured(cap: &EreCapture, line: &str) -> Option<String> {
    cap.capture(line).map(|(s, e)| normalize(&span(line, s, e)))
}

fn base_items(text: &str, cap: &EreCapture) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut fence = spec::Fence::default();
    for line in text.lines() {
        if fence.delimits(line) || fence.is_open() {
            continue;
        }
        out.extend(captured(cap, line));
    }
    out
}

#[derive(Debug, PartialEq)]
enum Why {
    Lacks,
    Carries,
    NoBase,
}

#[derive(Debug)]
struct Finding {
    line: usize,
    family: String,
    item: String,
    why: Why,
}

struct Scan {
    findings: Vec<Finding>,
    agreed: usize,
}

// spec: canon-kit/SPEC.md §check-delta-agreement — a family's section runs from its heading to the
// next heading of that level or shallower; two families may share one heading
fn scan(text: &str, families: &[Family], base: Option<&BTreeSet<String>>) -> Scan {
    let mut out = Scan {
        findings: Vec::new(),
        agreed: 0,
    };
    let mut open: Vec<Option<usize>> = vec![None; families.len()];
    let mut fence = spec::Fence::default();
    for (i, line) in text.lines().enumerate() {
        if fence.delimits(line) || fence.is_open() {
            continue;
        }
        let level = heading_level(line);
        if level > 0 {
            for (f, at) in families.iter().zip(open.iter_mut()) {
                if at.is_some_and(|l| level <= l) {
                    *at = None;
                }
                if f.heading.is_match(line) {
                    *at = Some(level);
                }
            }
        }
        for (f, at) in families.iter().zip(&open) {
            if at.is_none() {
                continue;
            }
            let Some(item) = captured(&f.item, line) else {
                continue;
            };
            let why = match base {
                None if f.required => Some(Why::NoBase),
                None => None,
                Some(set) if f.present && !set.contains(&item) => Some(Why::Lacks),
                Some(set) if !f.present && set.contains(&item) => Some(Why::Carries),
                Some(_) => None,
            };
            match why {
                Some(why) => out.findings.push(Finding {
                    line: i + 1,
                    family: f.name.clone(),
                    item,
                    why,
                }),
                None => out.agreed += 1,
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const REQ: &str = "^### Requirement:[[:space:]]*(.+)$";

    fn pairs(v: &[(&str, &str)]) -> Vec<(String, String)> {
        v.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
    }

    fn openspec() -> Config {
        config(
            "^openspec/changes/[^/]+/specs/(.+/spec\\.md)$",
            "openspec/specs",
            REQ,
            &pairs(&[
                ("modified", "^## MODIFIED Requirements"),
                ("removed", "^## REMOVED Requirements"),
                ("renamed", "^## RENAMED Requirements"),
            ]),
            &pairs(&[("added", "^## ADDED Requirements"), ("renamed-to", "^## RENAMED Requirements")]),
            &pairs(&[
                ("added", REQ),
                ("modified", REQ),
                ("removed", REQ),
                ("renamed", "^- FROM: `### Requirement:[[:space:]]*(.+)`"),
                ("renamed-to", "^- TO: `### Requirement:[[:space:]]*(.+)`"),
            ]),
            &["modified".to_string(), "removed".to_string(), "renamed".to_string()],
        )
        .expect("the OpenSpec binding compiles")
    }

    fn base() -> BTreeSet<String> {
        ["Export one release", "Entry length"].iter().map(|s| s.to_string()).collect()
    }

    fn got(s: &Scan) -> Vec<(usize, &str, &str, &Why)> {
        s.findings.iter().map(|f| (f.line, f.family.as_str(), f.item.as_str(), &f.why)).collect()
    }

    #[test]
    fn each_finding_shape_against_an_existing_base() {
        let text = "## ADDED Requirements\n### Requirement: Entry length\n### Requirement: Digest\n\n\
                    ## MODIFIED Requirements\n### Requirement: Entry  length \n### Requirement: entry length\n\n\
                    ## REMOVED Requirements\n### Requirement: Gone\n";
        let s = scan(text, &openspec().families, Some(&base()));
        assert_eq!(
            got(&s),
            vec![
                (2, "added", "Entry length", &Why::Carries),
                (7, "modified", "entry length", &Why::Lacks),
                (10, "removed", "Gone", &Why::Lacks),
            ]
        );
        assert_eq!(s.agreed, 2);
    }

    #[test]
    fn a_shared_heading_carries_two_families_and_a_fence_is_skipped() {
        let text = "## RENAMED Requirements\n- FROM: `### Requirement: Entry length`\n- TO: `### Requirement: Export one release`\n\
                    ```\n## REMOVED Requirements\n### Requirement: Absent\n```\n# Next\n### Requirement: Absent\n";
        let s = scan(text, &openspec().families, Some(&base()));
        assert_eq!(got(&s), vec![(3, "renamed-to", "Export one release", &Why::Carries)]);
        assert_eq!(s.agreed, 1);
    }

    #[test]
    fn a_missing_base_reds_only_a_base_required_family() {
        let text = "## ADDED Requirements\n### Requirement: New\n## MODIFIED Requirements\n### Requirement: Old\n";
        let s = scan(text, &openspec().families, None);
        assert_eq!(got(&s), vec![(4, "modified", "Old", &Why::NoBase)]);
        assert_eq!(s.agreed, 1);
    }

    #[test]
    fn a_base_item_is_normalized_and_its_fences_skipped() {
        let cap = EreCapture::compile(REQ).unwrap();
        let items = base_items("### Requirement:Entry\tlength  \n```\n### Requirement: Quoted\n```\n", &cap);
        assert_eq!(items.into_iter().collect::<Vec<_>>(), vec!["Entry length"]);
    }

    #[test]
    fn a_group_ending_inside_a_multibyte_character_does_not_panic() {
        let cap = EreCapture::compile("^### Requirement: (.).*$").unwrap();
        assert!(captured(&cap, "### Requirement: \u{e9}t\u{e9}").is_some_and(|g| !g.is_empty()));
        let path = EreCapture::compile("^d/(.)").unwrap();
        let (s, e) = path.capture("d/\u{e9}.md").unwrap();
        assert!(!span("d/\u{e9}.md", s, e).is_empty());
    }

    // spec: canon-kit/SPEC.md §check-delta-agreement — the exit-2 path, which a `bad/` fixture held
    // to exit 1 cannot express
    #[test]
    fn a_family_outside_the_key_sets_is_a_refusal() {
        let e = config("^(.+)$", "", REQ, &pairs(&[("a", "^## A")]), &[], &pairs(&[("b", REQ)]), &[])
            .err()
            .expect("refused");
        assert!(e.contains("'a'") && e.contains("'b'"), "{}", e);
    }
}
