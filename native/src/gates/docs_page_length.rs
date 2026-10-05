// spec: canon-kit/SPEC.md §check-docs-page-length — no declared page runs past the configured
// word bound
use crate::spec;
use crate::walk;
use std::path::Path;

const PAGES_KNOB: &str = "CANON_KIT_PAGE_LENGTH_PAGES";
const EXCLUDE_KNOB: &str = "CANON_KIT_PAGE_LENGTH_EXCLUDE";
const MAX_KNOB: &str = "CANON_KIT_PAGE_LENGTH_MAX_WORDS";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-docs-page-length: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let bound = parse_bound(&spec::knob_pub(MAX_KNOB)?)?;
    let excluded = expand(root, &spec::knob_array_pub(EXCLUDE_KNOB)?)?;
    let pages: Vec<String> = expand(root, &spec::knob_array_pub(PAGES_KNOB)?)?
        .into_iter()
        .filter(|p| !excluded.contains(p))
        .collect();

    let Some(bound) = bound else {
        println!(
            "DOCS-PAGE-LENGTH: clean ({} page(s), nothing asserted: {} is off)",
            pages.len(),
            MAX_KNOB
        );
        return Ok(0);
    };

    let mut counts: Vec<(String, usize)> = Vec::new();
    for page in pages {
        let n = page_words(&read_page(&page)?);
        counts.push((page, n));
    }
    let over: Vec<&(String, usize)> = counts.iter().filter(|(_, n)| *n > bound).collect();
    if !over.is_empty() {
        println!(
            "check-docs-page-length: {} page(s) past {} word(s):",
            over.len(),
            bound
        );
        for (page, n) in &over {
            println!("  {}: {} word(s), the bound is {}", page, n, bound);
        }
        println!("  help: shorten the page, or split it into a parent and sub-pages.");
        return Ok(1);
    }
    let longest = match counts.iter().max_by_key(|(_, n)| *n) {
        Some((page, n)) => format!(", the longest {} at {}", page, n),
        None => String::new(),
    };
    println!(
        "DOCS-PAGE-LENGTH: clean ({} page(s), none past {} word(s){})",
        counts.len(),
        bound,
        longest
    );
    Ok(0)
}

fn expand(root: &str, globs: &[String]) -> Result<Vec<String>, String> {
    if globs.is_empty() {
        return Ok(Vec::new());
    }
    let mut out: Vec<String> = walk::glob_corpus(Path::new(root), globs)?
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    out.sort();
    out.dedup();
    Ok(out)
}

// spec: canon-kit/SPEC.md §check-docs-page-length — the bound is a positive integer or `off`
fn parse_bound(raw: &str) -> Result<Option<usize>, String> {
    let t = raw.trim();
    if t == "off" {
        return Ok(None);
    }
    match t.parse::<usize>() {
        Ok(n) if n > 0 => Ok(Some(n)),
        _ => Err(format!("{} is '{}', neither a positive integer nor off", MAX_KNOB, raw)),
    }
}

fn read_page(page: &str) -> Result<String, String> {
    spec::read_text(Path::new(page)).map_err(|e| format!("cannot read declared page {}: {}", page, e))
}

// spec: canon-kit/SPEC.md §check-docs-page-length — the measure: whitespace-separated tokens after
// the front-matter block, HTML comments skipped outside a fence, table rows and fenced blocks counted
fn page_words(text: &str) -> usize {
    let raw: Vec<&str> = text.lines().collect();
    let mut front = raw.first().map(|l| l.trim_end() == "---").unwrap_or(false);
    let mut fence = false;
    let mut comment = false;
    let mut words = 0usize;
    for (i, line) in raw.iter().enumerate() {
        if front {
            if i > 0 && line.trim_end() == "---" {
                front = false;
            }
            continue;
        }
        if !comment && spec::is_fence_line(line) {
            fence = !fence;
            words += line.split_whitespace().count();
            continue;
        }
        if fence {
            words += line.split_whitespace().count();
            continue;
        }
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
                        words += rest[..s].split_whitespace().count();
                        rest = &rest[s + 4..];
                        comment = true;
                    }
                    None => {
                        words += rest.split_whitespace().count();
                        break;
                    }
                }
            }
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front_matter_and_comments_are_skipped_and_tables_and_fences_count() {
        let page = "---\ntitle: not counted here\n---\n# One two\n\nthree <!-- skipped\nstill skipped --> four\n\n| five | six |\n\n```sh\nseven <!-- eight -->\n```\n";
        assert_eq!(page_words(page), 16);
    }

    #[test]
    fn a_page_with_no_front_matter_counts_from_its_first_line() {
        assert_eq!(page_words("one two\n---\nthree\n"), 4);
    }

    // spec: canon-kit/SPEC.md §check-docs-page-length — both exit-2 paths, which a `bad/` fixture
    // held to exit 1 cannot express
    #[test]
    fn an_unreadable_page_and_a_bad_bound_are_refusals() {
        assert!(read_page("no/such/page-length.md")
            .unwrap_err()
            .contains("cannot read declared page"));
        for bad in ["0", "-1", "many", "", "OFF"] {
            assert!(parse_bound(bad).unwrap_err().contains("neither a positive integer nor off"));
        }
        assert_eq!(parse_bound(" 1500 "), Ok(Some(1500)));
        assert_eq!(parse_bound("off"), Ok(None));
    }
}
