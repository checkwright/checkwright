// spec: canon-kit/SPEC.md §check-rule-id-coverage — every rule id the code declares is cited by
// exactly one spec file, and every rule id a spec cites is declared
use super::task_label_resolution::labels;
use super::task_path_claim::valve_reason;
use crate::ere::EreCapture;
use crate::knobs::canon_kit::{rule_id_refusals, RuleIdConfig};
use crate::spec;
use crate::walk;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const DECLARE_GLOBS: &str = "CANON_KIT_RULE_ID_DECLARE_GLOBS";
const SPEC_GLOBS: &str = "CANON_KIT_RULE_ID_SPEC_GLOBS";
const DECLARES: &str = "CANON_KIT_RULE_ID_DECLARES";
const CITES: &str = "CANON_KIT_RULE_ID_CITES";
const HOMES: &str = "CANON_KIT_RULE_ID_HOMES";
const VALVE: &str = "rule-id-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-rule-id-coverage: {}", e);
            2
        }
    }
}

struct Family {
    name: String,
    declare: EreCapture,
    cite: EreCapture,
}

type Pick = fn(&Family) -> &EreCapture;

fn declare_side(f: &Family) -> &EreCapture {
    &f.declare
}

fn cite_side(f: &Family) -> &EreCapture {
    &f.cite
}

#[derive(Clone, Copy, PartialEq)]
enum Valve {
    None,
    Exempt,
    Empty,
}

struct Occurrence {
    family: usize,
    id: String,
    file: String,
    line: usize,
    valve: Valve,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let (declares, cites) = (walk::knob_map(DECLARES)?, walk::knob_map(CITES)?);
    let (declare_globs, spec_globs) = (spec::knob_array_pub(DECLARE_GLOBS)?, spec::knob_array_pub(SPEC_GLOBS)?);
    let homes = spec::knob_pub(HOMES)?;
    let refusals = rule_id_refusals(&RuleIdConfig {
        declares: &declares,
        cites: &cites,
        declare_globs: &declare_globs,
        spec_globs: &spec_globs,
        homes: &homes,
    });
    if !refusals.is_empty() {
        return Err(refusals.join("; "));
    }
    if declares.is_empty() {
        println!(
            "RULE-ID-COVERAGE: clean (not applicable: no rule family is configured: {} is empty)",
            DECLARES
        );
        return Ok(0);
    }
    let families = families(&declares, &cites)?;
    let declaring = files(root, &declare_globs)?;
    let specs = files(root, &spec_globs)?;
    let decls = occurrences(&declaring, &families, declare_side)?;
    let cites = occurrences(&specs, &families, cite_side)?;
    let report = judge(&families, &decls, &cites, homes == "one");

    if !report.findings.is_empty() {
        println!(
            "check-rule-id-coverage: {} rule id(s) not covered both ways:",
            report.findings.len()
        );
        for f in &report.findings {
            println!("  {}", f);
        }
        println!("  help: cite each declared id from one spec file, and declare each id a spec cites, or correct the stale side.");
        println!("  help: a deliberate keep takes '{} <reason>' on the site's line or the one above.", VALVE);
        return Ok(1);
    }
    let one = if homes == "one" { "exactly one" } else { "at least one" };
    println!(
        "RULE-ID-COVERAGE: clean ({} declaring file(s), {} spec file(s), {} rule famil(ies), {} declared id(s), {} citation(s), {} valved; every declared id is cited by {} spec file and every cited id is declared)",
        declaring.len(),
        specs.len(),
        families.len(),
        report.declared,
        report.cited,
        report.valved,
        one
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-rule-id-coverage — the two knobs pair by key, each value a
// one-group ERE; the table validator's refusals are the one home of both rules
fn families(declares: &[(String, String)], cites: &[(String, String)]) -> Result<Vec<Family>, String> {
    let compile = |p: &str| EreCapture::compile(p).map_err(|e| e.to_string());
    let mut out: Vec<Family> = Vec::new();
    for (k, d) in declares {
        if let Some((_, c)) = cites.iter().find(|(ck, _)| ck == k) {
            out.push(Family {
                name: k.clone(),
                declare: compile(d)?,
                cite: compile(c)?,
            });
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn files(root: &str, globs: &[String]) -> Result<Vec<String>, String> {
    let prune = walk::prune_dirs()?;
    let mut out: Vec<String> = walk::glob_files_pruned(Path::new(root), globs, &|n| prune.iter().any(|d| d == n))?
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    out.sort();
    out.dedup();
    Ok(out)
}

// spec: canon-kit/SPEC.md §check-rule-id-coverage — fenced code is skipped in a markdown file only,
// and the valve rides the shared exempt window
fn occurrences(
    files: &[String],
    families: &[Family],
    pick: Pick,
) -> Result<Vec<Occurrence>, String> {
    let mut out: Vec<Occurrence> = Vec::new();
    for file in files {
        let text = spec::read_text(Path::new(file)).map_err(|e| format!("cannot read {}: {}", file, e))?;
        out.extend(scan(file, &text, families, pick));
    }
    Ok(out)
}

fn scan(file: &str, text: &str, families: &[Family], pick: Pick) -> Vec<Occurrence> {
    let markdown = file.ends_with(".md");
    let raw: Vec<&str> = text.lines().collect();
    let mut out: Vec<Occurrence> = Vec::new();
    let mut fence = spec::Fence::default();
    for (i, line) in raw.iter().enumerate() {
        if markdown && (fence.delimits(line) || fence.is_open()) {
            continue;
        }
        let reason = [Some(i), i.checked_sub(1)]
            .iter()
            .flatten()
            .find_map(|&k| valve_reason(raw[k], VALVE));
        let valve = match reason.as_deref() {
            None => Valve::None,
            Some("") => Valve::Empty,
            Some(_) => Valve::Exempt,
        };
        for (n, f) in families.iter().enumerate() {
            for id in labels(pick(f), line) {
                out.push(Occurrence {
                    family: n,
                    id,
                    file: file.to_string(),
                    line: i + 1,
                    valve,
                });
            }
        }
    }
    out
}

struct Report {
    findings: Vec<String>,
    declared: usize,
    cited: usize,
    valved: usize,
}

fn judge(families: &[Family], decls: &[Occurrence], cites: &[Occurrence], one_home: bool) -> Report {
    let mut report = Report {
        findings: Vec::new(),
        declared: 0,
        cited: 0,
        valved: 0,
    };
    let empty_note = |v: Valve| if v == Valve::Empty { " (its rule-id-exempt valve carries no reason)" } else { "" };
    for (n, fam) in families.iter().enumerate() {
        let declared: BTreeSet<&str> = decls.iter().filter(|d| d.family == n).map(|d| d.id.as_str()).collect();
        let mut first: BTreeMap<&str, &Occurrence> = BTreeMap::new();
        for d in decls.iter().filter(|d| d.family == n) {
            if d.valve == Valve::Exempt {
                report.valved += 1;
            } else {
                first.entry(d.id.as_str()).or_insert(d);
            }
        }
        let mut homes: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
        for c in cites.iter().filter(|c| c.family == n) {
            if c.valve == Valve::Exempt {
                report.valved += 1;
            } else if declared.contains(c.id.as_str()) {
                report.cited += 1;
                homes.entry(c.id.as_str()).or_default().entry(c.file.as_str()).or_insert(c.line);
            } else {
                report.findings.push(format!(
                    "{}:{}: undeclared: family `{}` cites `{}`, which no declaring file declares{}",
                    c.file,
                    c.line,
                    fam.name,
                    c.id,
                    empty_note(c.valve)
                ));
            }
        }
        for (id, d) in &first {
            report.declared += 1;
            match homes.get(id) {
                None => report.findings.push(format!(
                    "{}:{}: uncited: family `{}` declares `{}`, which no spec file cites{}",
                    d.file,
                    d.line,
                    fam.name,
                    id,
                    empty_note(d.valve)
                )),
                Some(h) if one_home && h.len() > 1 => {
                    let at: Vec<String> = h.iter().map(|(f, l)| format!("{}:{}", f, l)).collect();
                    report.findings.push(format!(
                        "{}:{}: cited twice: family `{}` declares `{}`, cited by {} spec files: {}",
                        d.file,
                        d.line,
                        fam.name,
                        id,
                        h.len(),
                        at.join(", ")
                    ));
                }
                Some(_) => {}
            }
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule_family() -> Vec<Family> {
        let pair = |a: &str, b: &str| vec![(a.to_string(), b.to_string())];
        families(&pair("rule", "rule\\(\"(R-[0-9]+)\"\\)"), &pair("rule", "\\[(R-[0-9]+)\\]"))
            .expect("the family compiles")
    }

    fn judged(decl: &str, specs: &[(&str, &str)], one_home: bool) -> Report {
        let fams = rule_family();
        let decls = scan("src/rules.rs", decl, &fams, declare_side);
        let cites: Vec<Occurrence> = specs.iter().flat_map(|(file, text)| scan(file, text, &fams, cite_side)).collect();
        judge(&fams, &decls, &cites, one_home)
    }

    #[test]
    fn each_finding_kind_reds_and_one_file_is_one_home() {
        let decl = "rule(\"R-1\")\nrule(\"R-2\")\nrule(\"R-3\")\n";
        let r = judged(
            decl,
            &[("a.md", "[R-1] and again [R-1]\n[R-3]\n"), ("b.md", "[R-3]\n```\n[R-8]\n```\n[R-9]\n")],
            true,
        );
        assert_eq!(
            r.findings,
            vec![
                "b.md:5: undeclared: family `rule` cites `R-9`, which no declaring file declares",
                "src/rules.rs:2: uncited: family `rule` declares `R-2`, which no spec file cites",
                "src/rules.rs:3: cited twice: family `rule` declares `R-3`, cited by 2 spec files: a.md:2, b.md:1",
            ]
        );
    }

    #[test]
    fn any_home_admits_several_files_and_the_valve_exempts_each_side() {
        let decl = "rule(\"R-1\")\n// rule-id-exempt: reserved for the next release\nrule(\"R-2\")\nrule(\"R-3\") // rule-id-exempt:\n";
        let r = judged(decl, &[("a.md", "[R-1]\n<!-- rule-id-exempt: a retired id -->\n[R-7]\n"), ("b.md", "[R-1]\n")], false);
        assert_eq!(
            r.findings,
            vec!["src/rules.rs:4: uncited: family `rule` declares `R-3`, which no spec file cites (its rule-id-exempt valve carries no reason)"]
        );
        assert_eq!((r.declared, r.cited, r.valved), (2, 2, 2));
    }

    #[test]
    fn a_fence_is_skipped_in_markdown_alone() {
        let fams = rule_family();
        let text = "```\n[R-1]\n```\n";
        assert!(scan("a.md", text, &fams, cite_side).is_empty());
        assert_eq!(scan("a.txt", text, &fams, cite_side).len(), 1);
    }
}
