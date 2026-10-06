// spec: site-kit/SPEC.md §check-docs-collapsible — every collapsible region on a tracked docs page
// renders its body as markdown, names what it holds, and hides no heading
use super::docs_render_fidelity::jekyll_internal;
use crate::fresh;
use crate::walk;
use crate::{proc, programs};
use std::path::Path;

const NAME: &str = "check-docs-collapsible";

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}: {}", NAME, msg);
            2
        }
    }
}

fn inner(args: &[String]) -> Result<i32, String> {
    let docs_knob = walk::knob_scalar("SITE_KIT_DOCS_DIR")?;
    let docs = fresh::strip_trailing_slash(fresh::positional(args, 0, &docs_knob)).to_string();

    if walk::toplevel_opt()?.is_none() {
        return Err("not a git repository — cannot enumerate tracked pages".to_string());
    }
    if !fresh::is_dir(&docs) {
        return Err(format!("docs dir not found: {}", docs));
    }
    let ls = proc::run(&programs::GIT, &["ls-files", "--", &docs])?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => return Err(fresh::fail_closed("git-ls-files", ls.code())),
    };
    let prune = walk::prune_dirs()?;
    let mut pages: Vec<String> = listing
        .lines()
        .filter(|p| {
            !p.is_empty()
                && p.ends_with(".md")
                && !jekyll_internal(p)
                && !walk::path_pruned(p, &prune)
                && Path::new(p).is_file()
        })
        .map(str::to_string)
        .collect();
    pages.sort();

    let mut findings: Vec<(String, Finding)> = Vec::new();
    let mut regions = 0usize;
    for page in &pages {
        let text = std::fs::read(page)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read {}: {}", page, e))?;
        let scan = scan(&text);
        regions += scan.regions;
        findings.extend(scan.findings.into_iter().map(|f| (page.clone(), f)));
    }

    if !findings.is_empty() {
        println!(
            "{}: {} collapsible region finding(s) on the docs pages:",
            NAME,
            findings.len()
        );
        for (page, f) in &findings {
            println!("  {}:{}: {}", page, f.line, f.assertion.say());
        }
        println!("  help: a region opens '<details markdown=\"1\">', then a '<summary>' naming what opens on the next line; it holds no heading and closes with '</details>'.");
        return Ok(1);
    }
    println!(
        "DOCS-COLLAPSIBLE: clean ({} tracked markdown page(s) under {}, {} collapsible region(s), each marked for markdown, summarized, heading-free and closed)",
        pages.len(),
        docs,
        regions
    );
    Ok(0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Assertion {
    Unmarked,
    NoSummary,
    Heading,
    Unclosed,
}

impl Assertion {
    fn say(self) -> &'static str {
        match self {
            Assertion::Unmarked => "region opens without markdown=\"1\" — the Pages parser prints its body raw",
            Assertion::NoSummary => "region carries no <summary> naming what it holds",
            Assertion::Heading => "region hides a heading",
            Assertion::Unclosed => "region is never closed",
        }
    }
}

#[derive(Debug, PartialEq)]
struct Finding {
    line: usize,
    assertion: Assertion,
}

struct Scan {
    findings: Vec<Finding>,
    regions: usize,
}

struct Open {
    line: usize,
    awaiting_summary: bool,
    heading: bool,
}

// spec: site-kit/SPEC.md §check-docs-collapsible — the opening tag's markdown attribute, either
// quote, block or span-free `1`
fn marked(tag: &str) -> bool {
    ["markdown=\"1\"", "markdown='1'", "markdown=\"block\"", "markdown='block'"]
        .iter()
        .any(|a| tag.contains(a))
}

// spec: site-kit/SPEC.md §check-docs-collapsible — a `<summary>` element whose text, tags
// stripped, is not empty
fn summary(line: &str) -> Option<bool> {
    let at = line.find("<summary")?;
    let rest = &line[at..];
    let open_end = rest.find('>')? + 1;
    let body = &rest[open_end..];
    let body = body.split("</summary>").next().unwrap_or(body);
    let mut text = String::new();
    let mut in_tag = false;
    for c in body.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    Some(!text.trim().is_empty())
}

fn opens_region(line: &str) -> bool {
    let t = line.trim_start();
    t.strip_prefix("<details")
        .is_some_and(|r| r.is_empty() || r.starts_with([' ', '>', '\t']))
}

fn atx(line: &str) -> bool {
    let lead = line.len() - line.trim_start_matches(' ').len();
    if lead > 3 {
        return false;
    }
    let t = &line[lead..];
    let n = t.bytes().take_while(|&c| c == b'#').count();
    (1..=6).contains(&n) && t[n..].chars().next().map_or(true, |c| c == ' ' || c == '\t')
}

fn setext_underline(line: &str) -> bool {
    let lead = line.len() - line.trim_start_matches(' ').len();
    let t = line.trim();
    lead <= 3
        && ((!t.is_empty() && t.bytes().all(|c| c == b'='))
            || (t.len() >= 2 && t.bytes().all(|c| c == b'-')))
}

fn paragraph_text(line: &str) -> bool {
    let t = line.trim_start();
    !t.is_empty() && !t.starts_with('<') && !t.starts_with('-') && !t.starts_with('*') && !atx(line)
}

fn scan(text: &str) -> Scan {
    let raw: Vec<&str> = text.lines().collect();
    let mut out = Scan {
        findings: Vec::new(),
        regions: 0,
    };
    let mut stack: Vec<Open> = Vec::new();
    let mut front = raw.first().map(|l| l.trim_end() == "---").unwrap_or(false);
    let mut fence = crate::spec::Fence::default();
    let mut comment = false;
    let mut prev = "";
    for (i, line) in raw.iter().enumerate() {
        if front {
            if i > 0 && line.trim_end() == "---" {
                front = false;
            }
            continue;
        }
        if !comment && fence.delimits(line) {
            prev = "";
            continue;
        }
        if fence.is_open() {
            continue;
        }
        // spec: site-kit/SPEC.md §check-docs-collapsible — an HTML comment is not markup
        let mut kept = String::new();
        let mut rest: &str = line;
        loop {
            if comment {
                match rest.find("-->") {
                    Some(e) => {
                        rest = &rest[e + 3..];
                        comment = false;
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
        let line = kept.as_str();
        if line.trim().is_empty() {
            prev = "";
            continue;
        }
        let ln = i + 1;
        if let Some(top) = stack.last_mut() {
            if top.awaiting_summary {
                top.awaiting_summary = false;
                if summary(line) != Some(true) {
                    out.findings.push(Finding {
                        line: top.line,
                        assertion: Assertion::NoSummary,
                    });
                }
            }
        }
        if opens_region(line) {
            out.regions += 1;
            let tag_end = line.find('>').map(|e| e + 1).unwrap_or(line.len());
            if !marked(&line[..tag_end]) {
                out.findings.push(Finding {
                    line: ln,
                    assertion: Assertion::Unmarked,
                });
            }
            let inline = summary(&line[tag_end..]);
            if inline == Some(false) {
                out.findings.push(Finding {
                    line: ln,
                    assertion: Assertion::NoSummary,
                });
            }
            stack.push(Open {
                line: ln,
                awaiting_summary: inline.is_none(),
                heading: false,
            });
        } else if let Some(top) = stack.last_mut() {
            let heading = atx(line) || (setext_underline(line) && paragraph_text(prev));
            if heading && !top.heading {
                top.heading = true;
                out.findings.push(Finding {
                    line: top.line,
                    assertion: Assertion::Heading,
                });
            }
        }
        for _ in 0..line.matches("</details>").count() {
            stack.pop();
        }
        prev = raw[i];
    }
    for open in stack {
        out.findings.push(Finding {
            line: open.line,
            assertion: Assertion::Unclosed,
        });
    }
    out.findings.sort_by_key(|f| f.line);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn got(page: &str) -> Vec<(usize, Assertion)> {
        scan(page).findings.iter().map(|f| (f.line, f.assertion)).collect()
    }

    #[test]
    fn a_marked_summarized_region_with_a_fence_and_a_nested_region_is_clean() {
        let page = "<details markdown=\"1\">\n<summary>Steps</summary>\n\n```sh\n# not a heading\n```\n\n<details markdown=\"block\"><summary>Inner</summary>\n\nText\n</details>\n</details>\n";
        let s = scan(page);
        assert!(s.findings.is_empty(), "{:?}", s.findings);
        assert_eq!(s.regions, 2);
    }

    #[test]
    fn each_assertion_reds_at_the_opening_line() {
        assert_eq!(got("<details>\n<summary>A</summary>\n</details>\n"), vec![(1, Assertion::Unmarked)]);
        assert_eq!(got("<details markdown=\"1\">\n\nBody\n</details>\n"), vec![(1, Assertion::NoSummary)]);
        assert_eq!(got("<details markdown=\"1\">\n<summary> </summary>\n</details>\n"), vec![(1, Assertion::NoSummary)]);
        assert_eq!(got("<details markdown=\"1\">\n<summary>A</summary>\n\n## Hidden\n</details>\n"), vec![(1, Assertion::Heading)]);
        assert_eq!(got("<details markdown=\"1\">\n<summary>A</summary>\n\nHidden\n---\n</details>\n"), vec![(1, Assertion::Heading)]);
        assert_eq!(got("<details markdown=\"1\">\n<summary>A</summary>\n\nBody\n"), vec![(1, Assertion::Unclosed)]);
    }

    #[test]
    fn a_fenced_or_commented_region_and_front_matter_are_not_markup() {
        let page = "---\ntitle: x\n---\n```html\n<details>\n```\n<!-- <details> -->\n# Top heading\n";
        let s = scan(page);
        assert!(s.findings.is_empty());
        assert_eq!(s.regions, 0);
    }
}
