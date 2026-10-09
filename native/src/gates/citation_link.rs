// spec: canon-kit/SPEC.md §check-citation-link — on a declared page every section citation sits in a
// link's text, and a link whose text carries a citation targets the section it names
use super::spec_pointer::{opens_heading, paragraphs, sites};
use crate::spec;
use crate::walk;
use std::path::Path;

const PAGES_KNOB: &str = "CANON_KIT_CITATION_LINK_PAGES";
const VALVE: &str = "citation-link-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-citation-link: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let globs = spec::knob_array_pub(PAGES_KNOB)?;
    let mut pages: Vec<String> = if globs.is_empty() {
        Vec::new()
    } else {
        walk::glob_corpus(Path::new(root), &globs)?
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| spec::strip_dot_slash(&p.display().to_string()))
            .collect()
    };
    pages.sort();
    pages.dedup();

    let mut findings: Vec<(String, Finding)> = Vec::new();
    let mut tally = Tally::default();
    for page in &pages {
        let scan = scan(&read_page(page)?);
        tally.linked += scan.tally.linked;
        tally.later += scan.tally.later;
        tally.valved += scan.tally.valved;
        findings.extend(scan.findings.into_iter().map(|f| (page.clone(), f)));
    }

    if !findings.is_empty() {
        println!(
            "check-citation-link: {} citation(s) on the declared pages not linked to their section:",
            findings.len()
        );
        for (page, f) in &findings {
            let what = match f.arm {
                Arm::Unlinked => "unlinked citation",
                Arm::NoAnchor => "citation link carries no #anchor",
                Arm::WrongAnchor => "citation link's #anchor names another section",
            };
            let valve = if f.empty_valve {
                " (its citation-link-exempt valve carries no reason)"
            } else {
                ""
            };
            println!("  {}:{}: {}: {}{}", page, f.line, what, f.citation, valve);
        }
        if findings.iter().any(|(_, f)| matches!(f.arm, Arm::Unlinked)) {
            println!("  help: make the citation a link to its section: [<path> §<heading>](<target>#<anchor>).");
        }
        if findings.iter().any(|(_, f)| !matches!(f.arm, Arm::Unlinked)) {
            println!("  help: point the link at the section its text names.");
        }
        println!(
            "  help: a deliberate keep takes '<!-- {} <reason> -->' on the citation's line or the one above.",
            VALVE
        );
        return Ok(1);
    }
    println!(
        "CITATION-LINK: clean ({} page(s), {} linked citation(s), {} admitted later mention(s), {} valved; every section citation links its section)",
        pages.len(),
        tally.linked,
        tally.later,
        tally.valved
    );
    Ok(0)
}

fn read_page(page: &str) -> Result<String, String> {
    spec::read_text(Path::new(page)).map_err(|e| format!("cannot read declared page {}: {}", page, e))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Arm {
    Unlinked,
    NoAnchor,
    WrongAnchor,
}

#[derive(Debug)]
struct Finding {
    line: usize,
    arm: Arm,
    citation: String,
    empty_valve: bool,
}

#[derive(Default)]
struct Tally {
    linked: usize,
    later: usize,
    valved: usize,
}

struct Scan {
    findings: Vec<Finding>,
    tally: Tally,
}

// spec: canon-kit/SPEC.md §check-citation-link — the reader's paragraphs over prose only: the
// front-matter block, HTML comments and generated regions, markers included, are blanked in place,
// so every line keeps its number; §check-pendency-contradiction reads its corpus through it too
pub(crate) fn prose_only(text: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let raw: Vec<&str> = text.lines().collect();
    let body = spec::front_matter_span(&raw);
    let mut fence = spec::Fence::default();
    let mut comment = false;
    let mut gen = false;
    for (i, line) in raw.iter().enumerate() {
        if i < body {
            out.push(String::new());
            continue;
        }
        if gen {
            gen = !spec::is_gen_marker(line, ":end");
            out.push(String::new());
            continue;
        }
        if !comment && !fence.is_open() && spec::is_gen_marker(line, ":begin") {
            gen = true;
            out.push(String::new());
            continue;
        }
        if !comment && fence.delimits(line) {
            out.push(line.to_string());
            continue;
        }
        if fence.is_open() {
            out.push(line.to_string());
            continue;
        }
        let mut kept = String::new();
        let mut rest: &str = line;
        loop {
            if comment {
                match rest.find("-->") {
                    Some(e) => {
                        rest = &rest[e + 3..];
                        comment = false;
                        kept.push(' ');
                    }
                    None => break,
                }
            } else {
                match rest.find("<!--") {
                    Some(s) => {
                        kept.push_str(&rest[..s]);
                        rest = &rest[s + 4..];
                        comment = true;
                    }
                    None => {
                        kept.push_str(rest);
                        break;
                    }
                }
            }
        }
        out.push(kept);
    }
    out.join("\n")
}

// spec: canon-kit/SPEC.md §check-citation-link — an inline link's text span and its target, read
// outside code spans so a bracket quoted in code opens nothing
pub(crate) fn links(joined: &str) -> Vec<(usize, usize, String)> {
    let b = joined.as_bytes();
    let mut out: Vec<(usize, usize, String)> = Vec::new();
    let mut opens: Vec<usize> = Vec::new();
    let mut in_span = false;
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b'`' => in_span = !in_span,
            b'[' if !in_span => opens.push(i),
            b']' if !in_span => {
                if let Some(open) = opens.pop() {
                    if b.get(i + 1) == Some(&b'(') {
                        if let Some(off) = b[i + 2..].iter().position(|&c| c == b')') {
                            let inner = String::from_utf8_lossy(&b[i + 2..i + 2 + off]);
                            let tgt = inner.split(' ').next().unwrap_or("").to_string();
                            out.push((open + 1, i, tgt));
                            opens.clear();
                            i = i + 2 + off + 1;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

// spec: canon-kit/SPEC.md §check-citation-link — backticks dropped and whitespace folded, the
// later-mention comparison's normal form
fn fold(s: &str) -> String {
    s.replace('`', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

// spec: canon-kit/SPEC.md §check-citation-link — the text at a citation's start begins with an
// earlier citation link's text, ending at a boundary so a longer heading is not admitted
fn admitted(earlier: &[String], at_start: &str) -> bool {
    let tail = fold(at_start);
    earlier.iter().any(|t| {
        !t.is_empty()
            && tail.starts_with(t.as_str())
            && !tail[t.len()..]
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric())
    })
}

// spec: canon-kit/SPEC.md §check-citation-link — arm B: the anchor is the heading slug of the text
// after the §, or opens with that slug and a `-`
fn anchor_names(anchor: &str, heading: &str) -> bool {
    let slug = spec::anchor_slug(heading.trim());
    !slug.is_empty() && (anchor == slug || anchor.starts_with(&format!("{}-", slug)))
}

fn valve_reason(line: &str) -> Option<String> {
    let at = line.find(VALVE)?;
    let rest = &line[at + VALVE.len()..];
    let rest = rest.split("-->").next().unwrap_or(rest);
    Some(rest.trim().to_string())
}

fn scan(text: &str) -> Scan {
    let raw: Vec<&str> = text.lines().collect();
    let prose = prose_only(text);
    let mut out = Scan {
        findings: Vec::new(),
        tally: Tally::default(),
    };
    let mut earlier: Vec<String> = Vec::new();
    for para in paragraphs(&prose) {
        let j = &para.joined;
        let spans = links(j);
        let cites: Vec<_> = sites(j)
            .into_iter()
            .filter(|s| (s.path.is_some() || s.link.is_some() || s.bare) && opens_heading(&s.frag))
            .collect();
        let hosts = |at: usize| spans.iter().find(|(s, e, _)| *s <= at && at < *e);
        let mut cited_links: Vec<(usize, String)> = Vec::new();
        for site in &cites {
            if let Some((s, e, _)) = hosts(site.at) {
                if !cited_links.iter().any(|(end, _)| end == e) {
                    cited_links.push((*e, fold(&j[*s..*e])));
                }
            }
        }
        for site in cites {
            let start = site
                .path
                .as_ref()
                .map(|(s, _)| *s)
                .or(site.link.as_ref().map(|(s, _)| *s))
                .unwrap_or(site.at);
            let before: Vec<String> = earlier
                .iter()
                .cloned()
                .chain(cited_links.iter().filter(|(e, _)| *e <= start).map(|(_, t)| t.clone()))
                .collect();
            let line = para.line_of(start);
            let valve = [Some(line - 1), (line - 1).checked_sub(1)]
                .iter()
                .flatten()
                .find_map(|&i| raw.get(i).and_then(|l| valve_reason(l)));
            let exempt = valve.as_deref().is_some_and(|r| !r.is_empty());
            let empty_valve = valve.as_deref() == Some("");
            let shown: String = j[start..].chars().take(60).collect();
            let host = hosts(site.at);
            let arm = match host {
                Some((_, e, target)) => {
                    let text_after = &j[site.at + "§".len()..*e];
                    let heading = text_after.split('§').next().unwrap_or("");
                    match target.split_once('#') {
                        None => Some(Arm::NoAnchor),
                        Some((_, a)) if !anchor_names(a, heading) => Some(Arm::WrongAnchor),
                        _ => None,
                    }
                }
                None if admitted(&before, &j[start..]) => {
                    out.tally.later += 1;
                    None
                }
                None => Some(Arm::Unlinked),
            };
            match arm {
                None if host.is_some() => out.tally.linked += 1,
                None => {}
                Some(_) if exempt => out.tally.valved += 1,
                Some(arm) => out.findings.push(Finding {
                    line,
                    arm,
                    citation: shown,
                    empty_valve,
                }),
            }
        }
        earlier.extend(cited_links.into_iter().map(|(_, t)| t));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arms(page: &str) -> Vec<(usize, Arm)> {
        scan(page).findings.iter().map(|f| (f.line, f.arm)).collect()
    }

    #[test]
    fn a_later_mention_of_a_linked_section_is_admitted_and_a_longer_heading_is_not() {
        let page = "See [`a/SPEC.md` §Rules](a/SPEC.md#rules), then a/SPEC.md §Rules.\n\nAgain `a/SPEC.md` §Rules, then a/SPEC.md §Rulesets.\n";
        let s = scan(page);
        assert_eq!(s.tally.linked, 1);
        assert_eq!(s.tally.later, 2);
        assert_eq!(arms(page), vec![(3, Arm::Unlinked)]);
    }

    #[test]
    fn a_mention_before_its_link_is_not_admitted() {
        let page = "First a/SPEC.md §Rules.\n\nThen [a/SPEC.md §Rules](a/SPEC.md#rules).\n";
        assert_eq!(arms(page), vec![(1, Arm::Unlinked)]);
    }

    #[test]
    fn the_anchor_agrees_with_the_heading_whole_or_by_its_lead_clause() {
        assert!(anchor_names("rules", "Rules"));
        assert!(anchor_names("the-state-machine--stamps", "The state machine"));
        assert!(anchor_names("the-reference-link-grammar", "The reference-link grammar`"));
        assert!(!anchor_names("rulesets", "Rules"));
        assert!(!anchor_names("other", "Rules"));
        let page = "[a.md §Rules](a.md) and [b.md §Rules](b.md#other) and [c.md §Rules](c.md#rules-of-use).\n";
        assert_eq!(arms(page), vec![(1, Arm::NoAnchor), (1, Arm::WrongAnchor)]);
    }

    #[test]
    fn a_link_then_a_section_mark_is_unlinked_and_a_valve_needs_its_reason() {
        let page = "[SPEC.md](SPEC.md) §Rules.\n\n<!-- citation-link-exempt: quoted grammar -->\na.md §Rules\n\n<!-- citation-link-exempt: -->\nb.md §Rules\n";
        let s = scan(page);
        let got: Vec<(usize, bool)> = s.findings.iter().map(|f| (f.line, f.empty_valve)).collect();
        assert_eq!(got, vec![(1, false), (7, true)]);
        assert_eq!(s.tally.valved, 1);
    }

    #[test]
    fn front_matter_comments_fences_placeholders_and_code_spans_are_not_citations() {
        let page = "---\ntitle: a.md §Rules\n---\n<!-- a.md §Rules -->\n```\na.md §Rules\n```\nA §<heading> and `CANON §X` and § alone.\n";
        assert!(scan(page).findings.is_empty());
    }

    #[test]
    fn a_generated_region_is_held_out_and_its_numbering_kept() {
        let page = "<!-- roster:begin -->\na.md §Rules\n<!-- roster:end -->\nb.md §Rules\n";
        assert_eq!(arms(page), vec![(4, Arm::Unlinked)]);
    }

    // spec: canon-kit/SPEC.md §check-citation-link — the exit-2 path, which a `bad/` fixture held
    // to exit 1 cannot express
    #[test]
    fn an_unreadable_page_is_a_refusal() {
        assert!(read_page("no/such/citation-link.md")
            .unwrap_err()
            .contains("cannot read declared page"));
    }
}
