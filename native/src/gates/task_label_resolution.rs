// spec: canon-kit/SPEC.md §check-task-label-resolution — every label a task list cites resolves to
// a definition in a markdown file beside it
use super::task_path_claim::{task_lists, valve_reason};
use crate::ere::EreCapture;
use crate::spec;
use crate::walk;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const LISTS_KNOB: &str = "CANON_KIT_TASK_LIST_GLOBS";
const CITES_KNOB: &str = "CANON_KIT_TASK_LABEL_CITES";
const DEFINES_KNOB: &str = "CANON_KIT_TASK_LABEL_DEFINES";
const VALVE: &str = "task-label-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-task-label-resolution: {}", e);
            2
        }
    }
}

struct Family {
    name: String,
    cite: EreCapture,
    define: EreCapture,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let families = families(&walk::knob_map(CITES_KNOB)?, &walk::knob_map(DEFINES_KNOB)?)?;
    let globs = spec::knob_array_pub(LISTS_KNOB)?;
    if families.is_empty() || globs.is_empty() {
        let empty = if families.is_empty() { CITES_KNOB } else { LISTS_KNOB };
        println!(
            "TASK-LABEL-RESOLUTION: clean (no label family or task list is configured: {} is empty)",
            empty
        );
        return Ok(0);
    }
    let lists = task_lists(root, &globs)?;

    let mut defined: BTreeMap<String, Vec<BTreeSet<String>>> = BTreeMap::new();
    let mut findings: Vec<(String, Finding)> = Vec::new();
    let mut tally = Tally::default();
    for list in &lists {
        let dir = Path::new(list).parent().map(|d| d.display().to_string()).unwrap_or_default();
        if !defined.contains_key(&dir) {
            defined.insert(dir.clone(), definitions(&dir, &families)?);
        }
        let text = read(list)?;
        let scan = scan(&text, &families, &defined[&dir]);
        tally.resolved += scan.tally.resolved;
        tally.valved += scan.tally.valved;
        findings.extend(scan.findings.into_iter().map(|f| (list.clone(), f)));
    }

    if !findings.is_empty() {
        println!(
            "check-task-label-resolution: {} cited label(s) with no definition beside the task list:",
            findings.len()
        );
        for (list, f) in &findings {
            let valve = if f.empty_valve {
                " (its task-label-exempt valve carries no reason)"
            } else {
                ""
            };
            println!(
                "  {}:{}: family `{}` cites `{}`, which no markdown file beside it defines{}",
                list, f.line, f.family, f.label, valve
            );
        }
        println!("  help: define the label in a document beside the task list, or cite a label that is defined.");
        println!(
            "  help: a deliberate keep takes '<!-- {} <reason> -->' on the citation's line or the one above.",
            VALVE
        );
        return Ok(1);
    }
    println!(
        "TASK-LABEL-RESOLUTION: clean ({} task list(s), {} label famil(ies), {} citation(s) resolved, {} valved; every cited label is defined beside its task list)",
        lists.len(),
        families.len(),
        tally.resolved,
        tally.valved
    );
    Ok(0)
}

fn read(path: &str) -> Result<String, String> {
    spec::read_text(Path::new(path)).map_err(|e| format!("cannot read {}: {}", path, e))
}

// spec: canon-kit/SPEC.md §check-task-label-resolution — the two knobs pair by key, each value a
// one-group ERE; the table validator's refusals are the one home of both rules
fn families(cites: &[(String, String)], defines: &[(String, String)]) -> Result<Vec<Family>, String> {
    let refusals = crate::knobs::canon_kit::task_label_refusals(cites, defines);
    if !refusals.is_empty() {
        return Err(refusals.join("; "));
    }
    let compile = |p: &str| EreCapture::compile(p).map_err(|e| e.to_string());
    let mut out: Vec<Family> = Vec::new();
    for (k, c) in cites {
        if let Some((_, d)) = defines.iter().find(|(dk, _)| dk == k) {
            out.push(Family {
                name: k.clone(),
                cite: compile(c)?,
                define: compile(d)?,
            });
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

// spec: canon-kit/SPEC.md §check-task-label-resolution — the line is matched whole: each
// leftmost-longest match in turn, its group captured within that match's span
fn labels(cap: &EreCapture, line: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while from <= line.len() {
        let Some(((_, e), (gs, ge))) = cap.capture_from(line, from) else {
            break;
        };
        out.push(String::from_utf8_lossy(&line.as_bytes()[gs..ge]).into_owned());
        from = if e > from { e } else { from + 1 };
    }
    out
}

// spec: canon-kit/SPEC.md §check-task-label-resolution — every `*.md` file in the task list's own
// directory, the task list included, each family's defined labels
fn definitions(dir: &str, families: &[Family]) -> Result<Vec<BTreeSet<String>>, String> {
    let at = if dir.is_empty() { "." } else { dir };
    let mut out: Vec<BTreeSet<String>> = vec![BTreeSet::new(); families.len()];
    for (name, is_dir) in walk::list_dir(Path::new(at))? {
        if is_dir || !name.ends_with(".md") {
            continue;
        }
        let text = read(&walk::child(Path::new(at), &name).display().to_string())?;
        for line in text.lines() {
            for (f, set) in families.iter().zip(out.iter_mut()) {
                set.extend(labels(&f.define, line));
            }
        }
    }
    Ok(out)
}

#[derive(Debug)]
struct Finding {
    line: usize,
    family: String,
    label: String,
    empty_valve: bool,
}

#[derive(Default)]
struct Tally {
    resolved: usize,
    valved: usize,
}

struct Scan {
    findings: Vec<Finding>,
    tally: Tally,
}

fn scan(text: &str, families: &[Family], defined: &[BTreeSet<String>]) -> Scan {
    let raw: Vec<&str> = text.lines().collect();
    let mut out = Scan {
        findings: Vec::new(),
        tally: Tally::default(),
    };
    let mut fence = false;
    for (i, line) in raw.iter().enumerate() {
        if spec::is_fence_line(line) {
            fence = !fence;
            continue;
        }
        if fence {
            continue;
        }
        let valve = [Some(i), i.checked_sub(1)]
            .iter()
            .flatten()
            .find_map(|&k| valve_reason(raw[k], VALVE));
        let exempt = valve.as_deref().is_some_and(|r| !r.is_empty());
        for (f, set) in families.iter().zip(defined) {
            for label in labels(&f.cite, line) {
                if set.contains(&label) {
                    out.tally.resolved += 1;
                } else if exempt {
                    out.tally.valved += 1;
                } else {
                    out.findings.push(Finding {
                        line: i + 1,
                        family: f.name.clone(),
                        label,
                        empty_valve: valve.as_deref() == Some(""),
                    });
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn story() -> Vec<Family> {
        let pair = |a: &str, b: &str| vec![(a.to_string(), b.to_string())];
        families(
            &pair("story", "\\[US([0-9]+)\\]"),
            &pair("story", "^### User Story ([0-9]+)[ ]"),
        )
        .expect("the family compiles")
    }

    #[test]
    fn every_citation_on_a_line_resolves_or_reds_and_a_fence_is_skipped() {
        let fams = story();
        let defined = vec![["1".to_string()].into_iter().collect::<BTreeSet<String>>()];
        let text = "- [ ] T1 [US1] and [US2]\n```\n[US9]\n```\n<!-- task-label-exempt: moved -->\n- [ ] T2 [US3]\n";
        let s = scan(text, &fams, &defined);
        let got: Vec<(usize, &str)> = s.findings.iter().map(|f| (f.line, f.label.as_str())).collect();
        assert_eq!(got, vec![(1, "2")]);
        assert_eq!((s.tally.resolved, s.tally.valved), (1, 1));
    }

    #[test]
    fn a_define_pattern_reads_the_heading_label() {
        let fams = story();
        assert_eq!(labels(&fams[0].define, "### User Story 12 - Export"), vec!["12"]);
        assert!(labels(&fams[0].define, "### User Story 12").is_empty());
    }

    // spec: canon-kit/SPEC.md §check-task-label-resolution — the exit-2 path, which a `bad/`
    // fixture held to exit 1 cannot express
    #[test]
    fn differing_key_sets_are_a_refusal() {
        let pair = |a: &str, b: &str| vec![(a.to_string(), b.to_string())];
        let e = families(&pair("story", "(a)"), &pair("req", "(b)")).err().expect("refused");
        assert!(e.contains("'req'") && e.contains("one key set"), "{}", e);
    }
}
