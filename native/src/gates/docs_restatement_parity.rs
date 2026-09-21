// spec: canon-kit/SPEC.md §check-docs-restatement-parity — every arm and kit-knob token a
// declared restating page carries in its code still occurs in the `README.md` beside it
use super::docs_cmd::{inline_code_spans, kit_knob_prefixes, kit_knob_runs};
use crate::spec;
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-docs-restatement-parity: {}", e);
            2
        }
    }
}

const SOURCE: &str = "README.md";

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let globs = spec::knob_array_pub("CANON_KIT_RESTATEMENT_PAGES")?;
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

    let pairs = pair(&pages)?;
    let prefixes = kit_knob_prefixes()?;
    let mut bad: Vec<String> = Vec::new();
    let mut ntok = 0usize;
    for (page, source) in &pairs {
        let src = spec::read_text(Path::new(source))?;
        let held = source_tokens(&src, &prefixes);
        let text = spec::read_text(Path::new(page))?;
        for (ln, tok) in page_tokens(&text, &prefixes) {
            ntok += 1;
            if !held.contains(&tok) {
                bad.push(format!("{}:{}: '{}' occurs nowhere in {}", page, ln, tok, source));
            }
        }
    }

    if !bad.is_empty() {
        println!("check-docs-restatement-parity: a restating page names a token its source no longer carries:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: the source README is the owner — re-spell the page's code to the arm or knob the README now names, or drop the token from the page; the page may carry a subset, never a token its source lacks.");
        return Ok(1);
    }
    println!(
        "DOCS-RESTATEMENT-PARITY: clean ({} page(s), {} token(s) held to their sibling {})",
        pairs.len(),
        ntok,
        SOURCE
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-docs-restatement-parity — a declared page with no source
// beside it is a configuration error, never a pass, so every page is paired before any compare
fn pair(pages: &[String]) -> Result<Vec<(String, String)>, String> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    for page in pages {
        let source = sibling(page);
        if !Path::new(&source).is_file() {
            return Err(format!(
                "{} is a declared restating page (CANON_KIT_RESTATEMENT_PAGES) with no {} beside it to hold it to",
                page, SOURCE
            ));
        }
        pairs.push((page.clone(), source));
    }
    Ok(pairs)
}

fn sibling(page: &str) -> String {
    match page.rfind('/') {
        Some(i) => format!("{}/{}", &page[..i], SOURCE),
        None => SOURCE.to_string(),
    }
}

// spec: canon-kit/SPEC.md §check-docs-restatement-parity — the page's tokens are read from its
// code alone, inline spans and fenced lines; prose outside code is a narrative mention
fn page_tokens(text: &str, prefixes: &[String]) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut fenced = false;
    for (i, raw) in text.lines().enumerate() {
        if spec::is_fence_line(raw) {
            fenced = !fenced;
            continue;
        }
        let spans: Vec<String> = if fenced {
            vec![raw.to_string()]
        } else {
            inline_code_spans(raw)
        };
        for span in spans {
            for tok in tokens(&span, prefixes) {
                out.push((i + 1, tok));
            }
        }
    }
    out
}

// spec: canon-kit/SPEC.md §check-docs-restatement-parity — the source is read whole, prose
// included, and a token is held by a whole-token occurrence, so `--run` is not found inside
// `--run-gate-tests`
fn source_tokens(text: &str, prefixes: &[String]) -> BTreeSet<String> {
    tokens(text, prefixes).into_iter().collect()
}

fn tokens(text: &str, prefixes: &[String]) -> Vec<String> {
    let mut out = arm_tokens(text);
    out.extend(kit_knob_runs(text, prefixes));
    out
}

// spec: canon-kit/SPEC.md §check-docs-restatement-parity — an arm token is `--`, a lowercase
// letter, then `[a-z0-9-]`, not glued to a preceding word character
fn arm_tokens(text: &str) -> Vec<String> {
    let b = text.as_bytes();
    let tail = |c: u8| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-';
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 2 < b.len() {
        let glued = i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'-' || b[i - 1] == b'_');
        if b[i] == b'-' && b[i + 1] == b'-' && b[i + 2].is_ascii_lowercase() && !glued {
            let mut j = i + 3;
            while j < b.len() && tail(b[j]) {
                j += 1;
            }
            out.push(String::from_utf8_lossy(&b[i..j]).into_owned());
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_arm_token_is_whole_and_unglued() {
        assert_eq!(
            arm_tokens("x --run-gate-tests a --emit md-section a--b ---c --9 --run"),
            vec!["--run-gate-tests", "--emit", "--run"]
        );
    }

    #[test]
    fn a_page_is_read_in_its_code_and_nowhere_else() {
        let prefixes = vec!["WIDGET_KIT_".to_string()];
        let page = "Prose names --prose-only and WIDGET_KIT_PROSE.\n\nRun `--emit graph`.\n\n```bash\ngates --run-gate-tests WIDGET_KIT_X\n```\n";
        assert_eq!(
            page_tokens(page, &prefixes),
            vec![
                (3, "--emit".to_string()),
                (6, "--run-gate-tests".to_string()),
                (6, "WIDGET_KIT_X".to_string()),
            ]
        );
    }

    #[test]
    fn a_source_holds_a_token_whole_and_never_as_a_prefix() {
        let held = source_tokens("run --run-gate-tests and WIDGET_KIT_XY", &["WIDGET_KIT_".to_string()]);
        assert!(held.contains("--run-gate-tests"));
        assert!(!held.contains("--run"));
        assert!(!held.contains("WIDGET_KIT_X"));
    }

    #[test]
    fn the_source_is_the_readme_in_the_pages_own_directory() {
        assert_eq!(sibling("docs/canon-kit/index.md"), "docs/canon-kit/README.md");
        assert_eq!(sibling("index.md"), "README.md");
    }

    // spec: canon-kit/SPEC.md §check-docs-restatement-parity — the no-source arm is exit 2, which
    // a `bad/` fixture (held to exit 1) cannot express, so it is asserted here
    #[test]
    fn a_declared_page_with_no_source_is_a_configuration_error() {
        let dir = std::env::temp_dir().join(format!("ck-restate-{}", std::process::id()));
        let kit = dir.join("kit");
        std::fs::create_dir_all(&kit).expect("cannot create the scratch page directory");
        std::fs::write(kit.join("index.md"), "`--emit graph`\n").expect("cannot write the scratch page");
        let pages = vec![format!("{}/kit/index.md", dir.display())];
        let err = pair(&pages).expect_err("a page with no sibling README was paired rather than refused");
        assert!(err.contains("no README.md beside it"), "{}", err);
        std::fs::write(kit.join(SOURCE), "`--emit graph`\n").expect("cannot write the scratch source");
        let paired = pair(&pages).expect("a page with its sibling README was refused");
        assert_eq!(paired[0].1, format!("{}/kit/README.md", dir.display()));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
