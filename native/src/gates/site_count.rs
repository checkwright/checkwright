// spec: gate-sdk/SPEC.md §check-site-count — every file's count of a configured site equals the count
// a person reviewed
use crate::count_baseline::{self, Counts};
use crate::ere::Ere;
use crate::exit::{self, Fault};
use crate::rule_blocks::{self, Grammar};
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-site-count";

// spec: gate-sdk/SPEC.md §check-site-count — §check-script-preflight's block grammar with one kind
const GRAMMAR: Grammar = Grammar {
    kinds: &[("site-count", &["site", "baseline"], &["paths"])],
    once: &["baseline", "paths"],
    tops: &[],
};

struct Rule {
    id: String,
    sites: Vec<Ere>,
    baseline: String,
    paths: Vec<String>,
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            let fault = fault.prefixed(NAME);
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

fn parse(file: &str, text: &str) -> Result<Vec<Rule>, Fault> {
    let (_, blocks) = rule_blocks::parse(file, text, &GRAMMAR)?;
    blocks
        .into_iter()
        .map(|b| {
            Ok(Rule {
                sites: b.eres(file, "site")?,
                baseline: b.one("baseline"),
                paths: b.words("paths"),
                id: b.id,
            })
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §check-site-count — non-overlapping matches, an empty one stepping past
// its position so a count always ends
fn matches(site: &Ere, line: &str) -> usize {
    let (mut n, mut at) = (0, 0);
    while let Some((start, end)) = site.find_from(line, at) {
        n += 1;
        at = if end > start { end } else { end + 1 };
        if at > line.len() {
            break;
        }
    }
    n
}

// spec: gate-sdk/SPEC.md §check-site-count — the sites on the lines before test code starts
fn count(text: &str, test_from: Option<usize>, sites: &[Ere]) -> usize {
    let shipped = test_from.map_or(usize::MAX, |n| n - 1);
    text.lines().take(shipped).map(|l| sites.iter().map(|s| matches(s, l)).sum::<usize>()).sum()
}

// spec: gate-sdk/SPEC.md §check-site-count — a file differing from its row, a file with no row
// counting 0 reviewed, so a new site, a removed one and a stale row all red
fn records(id: &str, found: &Counts, reviewed: &Counts) -> Vec<String> {
    let paths: BTreeSet<&String> = found.keys().chain(reviewed.keys()).collect();
    paths
        .into_iter()
        .filter_map(|p| {
            let (f, r) = (found.get(p).copied().unwrap_or(0), reviewed.get(p).copied().unwrap_or(0));
            (f != r).then(|| format!("{}: [{}] {}/{}", p, id, f, r))
        })
        .collect()
}

fn found(rule: &Rule, prune: &[String], globs: &[String]) -> Result<(Counts, usize), Fault> {
    let specs: Vec<&str> = if rule.paths.is_empty() { vec!["."] } else { rule.paths.iter().map(String::as_str).collect() };
    let listed = crate::listing::tracked(None, &specs).map_err(|e| Fault::tool(e.text()))?;
    let (mut counts, mut read) = (Counts::new(), 0);
    for p in listed.iter().filter(|p| !walk::path_pruned(p, prune) && Path::new(p).is_file()) {
        let bytes = std::fs::read(p).map_err(|e| Fault::input(format!("cannot read {}: {}", p, e)))?;
        if walk::binary(&bytes) {
            continue;
        }
        let text = String::from_utf8_lossy(&bytes);
        let test_from = walk::test_code(p, &text, globs);
        if test_from == Some(1) {
            continue;
        }
        read += 1;
        let n = count(&text, test_from, &rule.sites);
        if n > 0 {
            counts.insert(p.clone(), n);
        }
    }
    Ok((counts, read))
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    if !args.is_empty() {
        return Err(Fault::usage("takes no arguments — its rules are GATE_SDK_SITE_RULES, its corpus each rule's paths"));
    }
    let rules_file = walk::knob_scalar("GATE_SDK_SITE_RULES").map_err(Fault::refused)?;
    let rules = match exit::read_optional(&rules_file, "site rules file")? {
        Some(text) => parse(&rules_file, &text)?,
        None => Vec::new(),
    };
    if rules.is_empty() {
        println!("SITE-COUNT: clean (not applicable: no site rule configured)");
        return Ok(0);
    }
    if walk::toplevel_opt().map_err(Fault::tool)?.is_none() {
        return Err(Fault::refused("not a git repository — cannot enumerate tracked files"));
    }
    let prune = walk::prune_dirs().map_err(Fault::refused)?;
    let globs = walk::knob_words("GATE_SDK_TEST_GLOBS").map_err(Fault::refused)?;

    let (mut out, mut read) = (Vec::new(), 0);
    for rule in &rules {
        let reviewed = count_baseline::read(&rule.baseline)?;
        let (counts, n) = found(rule, &prune, &globs)?;
        read += n;
        out.extend(records(&rule.id, &counts, &reviewed));
    }
    if out.is_empty() {
        println!("SITE-COUNT: clean ({} file(s) read against {} rule(s) in {})", read, rules.len(), rules_file);
        return Ok(0);
    }
    println!("{}: file(s) whose site count differs from its reviewed count, rules in {}:", NAME, rules_file);
    for r in &out {
        println!("{}", r);
    }
    println!("  help: review every site in each named file, then set its row in the rule's baseline to the");
    println!("        found count (the first number); a file whose sites are gone loses its row.");
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(text: &str) -> Rule {
        parse("rules", text).expect("parses").remove(0)
    }

    // spec: gate-sdk/SPEC.md §check-site-count — non-overlapping matches of every site ERE,
    // summed over the lines before test code
    #[test]
    fn a_count_sums_every_site_before_test_code() {
        let r = rule("rule CLI site-count\n  site gh api\n  site ^gh \n  baseline b.tsv\n  paths src lib\n");
        assert_eq!((r.baseline.as_str(), r.paths.clone()), ("b.tsv", vec!["src".to_string(), "lib".to_string()]));
        let text = "gh api x; gh api y\ngh pr view\n#[cfg(test)]\ngh api z\n";
        assert_eq!(count(text, None, &r.sites), 6);
        assert_eq!(count(text, Some(3), &r.sites), 4);
        assert_eq!(matches(&Ere::compile("x*").expect("compiles"), "ab"), 3);
        assert_eq!(matches(&Ere::compile("aa").expect("compiles"), "aaaaa"), 2);
    }

    // spec: gate-sdk/SPEC.md §check-site-count — the comparison is exact, a missing row is 0
    #[test]
    fn every_difference_reds() {
        let found: Counts = [("over".to_string(), 3), ("new".to_string(), 1), ("same".to_string(), 2)].into();
        let reviewed: Counts = [("over".to_string(), 2), ("same".to_string(), 2), ("gone".to_string(), 1)].into();
        assert_eq!(records("ID", &found, &reviewed), ["gone: [ID] 0/1", "new: [ID] 1/0", "over: [ID] 3/2"]);
    }

    // spec: gate-sdk/SPEC.md §check-site-count — a block missing its baseline or holding a second
    // one refuses at its line
    #[test]
    fn a_malformed_rule_refuses() {
        let fault = parse("rules", "rule A site-count\n  site x\n").err().expect("refuses");
        assert!(fault.message.starts_with("rules:1: rule A lacks its 'baseline'"), "{}", fault.message);
        let fault = parse("rules", "rule A site-count\n  site x\n  baseline a\n  baseline b\n").err().expect("refuses");
        assert!(fault.message.starts_with("rules:4: key 'baseline' given twice"), "{}", fault.message);
        assert!(parse("rules", "waiver-min 3\n").is_err());
    }
}
