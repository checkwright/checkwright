// spec: canon-kit/SPEC.md §check-docs-link-convention — docs pages cite downward: no
// directory-target relative link, a kit page's back-link to its own README/SPEC carries a
// #section anchor, no relative link to a target resolving outside the docs root
use crate::fresh;
use crate::walk;
use std::path::Path;

// spec: canon-kit/SPEC.md §check-docs-link-convention — `grep -oE '\]\([^)]+\)'`: every inline
// link target on the line, the run between `](` and the first `)`
fn link_targets(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 2 < b.len() {
        if b[i] == b']' && b[i + 1] == b'(' {
            if let Some(off) = line[i + 2..].find(')') {
                if off > 0 {
                    out.push(&line[i + 2..i + 2 + off]);
                    i = i + 2 + off + 1;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
}

// spec: canon-kit/SPEC.md §check-docs-link-convention — `${tgt%% *}`: a `"title"` suffix inside
// the parentheses is not part of the target
fn target_head(raw: &str) -> &str {
    raw.split(' ').next().unwrap_or("")
}

// spec: gate-sdk/SPEC.md §The fourth budget batch — `realpath -m --relative-to=.`: lexical
// normalization that keeps a leading `..`, since the target need not exist
fn normalize_rel(p: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for seg in p.split('/') {
        match seg {
            "" | "." => {}
            ".." => match stack.last() {
                Some(last) if *last != ".." => {
                    stack.pop();
                }
                _ => stack.push(".."),
            },
            s => stack.push(s),
        }
    }
    if stack.is_empty() {
        ".".to_string()
    } else {
        stack.join("/")
    }
}

// spec: gate-sdk/SPEC.md §The fourth budget batch — the same call for an absolute join: normalize,
// then express the result against the invoking directory, which is what `--relative-to=.` does
fn relative_to(abs: &str, here: &str) -> String {
    let a: Vec<&str> = abs.split('/').filter(|s| !s.is_empty()).collect();
    let h: Vec<&str> = here.split('/').filter(|s| !s.is_empty()).collect();
    let mut i = 0usize;
    while i < a.len() && i < h.len() && a[i] == h[i] {
        i += 1;
    }
    let mut out: Vec<&str> = vec![".."; h.len() - i];
    out.extend_from_slice(&a[i..]);
    if out.is_empty() {
        ".".to_string()
    } else {
        out.join("/")
    }
}

fn resolve(base: &str, path: &str) -> Result<String, String> {
    let joined = format!("{}/{}", base, path);
    if !joined.starts_with('/') {
        return Ok(normalize_rel(&joined));
    }
    let here = std::env::current_dir()
        .map_err(|e| format!("cannot read the current directory: {}", e))?
        .display()
        .to_string();
    Ok(relative_to(&walk::normalize_abs(&joined), &here))
}

fn dir_of(p: &str) -> &str {
    match p.rfind('/') {
        Some(0) => "/",
        Some(i) => &p[..i],
        None => ".",
    }
}

fn base_of(p: &str) -> &str {
    match p.rfind('/') {
        Some(i) => &p[i + 1..],
        None => p,
    }
}

// spec: canon-kit/SPEC.md §check-docs-link-convention — a `docs-link-exempt:` marker on the hit
// line or the one above it
fn exempt(lines: &[&str], lineno: usize) -> bool {
    let lo = if lineno > 1 { lineno - 1 } else { 1 };
    lines[lo - 1..lineno]
        .iter()
        .any(|l| l.contains("docs-link-exempt:"))
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

fn inner(args: &[String]) -> Result<i32, String> {
    let knob = walk::knob_scalar("CANON_KIT_LINK_ROOT")
        .map_err(|e| format!("check-docs-link-convention: {}", e))?;
    let root = fresh::strip_trailing_slash(fresh::positional(args, 0, &knob)).to_string();
    if !Path::new(&root).is_dir() {
        return Err(format!(
            "check-docs-link-convention: not a directory: {}",
            root
        ));
    }

    // spec: canon-kit/SPEC.md §check-docs-link-convention — the shell form reaches for a bare
    // `find`, so no prune set applies and the walk is the whole docs tree
    let pages = walk::find_with_prune(Path::new(&root), &|_| false)
        .map_err(|e| format!("check-docs-link-convention: {}", e))?;
    let pages: Vec<String> = pages
        .into_iter()
        .map(|p| p.display().to_string())
        .filter(|p| p.ends_with(".md"))
        .collect();
    if pages.is_empty() {
        println!("DOCS-LINK-CONVENTION: clean (0 docs page(s) found)");
        return Ok(0);
    }

    let mut bad: Vec<String> = Vec::new();
    let mut links = 0usize;
    for f in &pages {
        let base = dir_of(f).to_string();
        // spec: canon-kit/SPEC.md §check-docs-link-convention — a kit page is <root>/<kit>/index.md
        let kit = if base_of(f) == "index.md" && base != root && dir_of(&base) == root {
            base_of(&base).to_string()
        } else {
            String::new()
        };
        let text = std::fs::read(f)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("check-docs-link-convention: cannot read {}: {}", f, e))?;
        let lines = fresh::file_lines(&text);
        for (idx, line) in lines.iter().enumerate() {
            let lno = idx + 1;
            for raw in link_targets(line) {
                let tgt = target_head(raw);
                if tgt.is_empty() || tgt.contains("://") || tgt.starts_with("mailto:") {
                    continue;
                }
                links += 1;
                let (path, anchor) = match tgt.split_once('#') {
                    Some((p, a)) => (p, a),
                    None => (tgt, ""),
                };
                if path.is_empty() {
                    continue;
                }
                if path.ends_with('/') {
                    if !exempt(&lines, lno) {
                        bad.push(format!(
                            "{}:{}: directory-target link '{}' — name the file (e.g. {}index.md), not the directory",
                            f, lno, tgt, path
                        ));
                    }
                    continue;
                }
                let p = resolve(&base, path)?;
                if p.is_empty() {
                    continue;
                }
                if Path::new(&p).is_dir() {
                    if !exempt(&lines, lno) {
                        bad.push(format!(
                            "{}:{}: directory-target link '{}' → {}/ — name the file (e.g. {}/index.md), not the directory",
                            f, lno, tgt, p, tgt
                        ));
                    }
                    continue;
                }
                if Path::new(&p).exists() && !p.starts_with(&format!("{}/", root)) {
                    if !exempt(&lines, lno) {
                        bad.push(format!(
                            "{}:{}: off-root relative link '{}' → {} — resolves outside {}/, so it 404s on a site served from {}/ alone; cite it in the absolute self-repo blob form",
                            f, lno, tgt, p, root, root
                        ));
                    }
                    continue;
                }
                if !kit.is_empty() && anchor.is_empty() {
                    let b = base_of(&p);
                    if (b == "README.md" || b == "SPEC.md")
                        && base_of(dir_of(&p)) == kit
                        && !exempt(&lines, lno)
                    {
                        bad.push(format!(
                            "{}:{}: anchorless back-link '{}' to this kit's {} — a docs page cites downward, name the #section rather than the whole spec",
                            f, lno, tgt, b
                        ));
                    }
                }
            }
        }
    }

    if !bad.is_empty() {
        println!("check-docs-link-convention: docs page link(s) break a shape convention (resolution is check-md-refs' job; this gate owns shape):");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: name the file a directory link points at (kit/index.md, not kit/); give a kit page's");
        println!("        back-link to its own README/SPEC a #section anchor; cite a target outside the docs");
        println!("        root with the absolute self-repo blob form (canon-kit/SPEC.md §The reference-link");
        println!("        grammar) rather than relatively. Per-site valve: a 'docs-link-exempt: <reason>'");
        println!("        HTML comment on the link line or the one above.");
        return Ok(1);
    }
    println!(
        "DOCS-LINK-CONVENTION: clean ({} docs page(s), {} relative link(s); no directory target, kit back-links anchored, no off-root relative target)",
        pages.len(),
        links
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_inline_target_on_a_line_is_taken_and_a_title_suffix_is_not_part_of_it() {
        assert_eq!(
            link_targets("see [a](one.md) and [b](two.md#x) here"),
            vec!["one.md", "two.md#x"]
        );
        assert_eq!(link_targets("an empty [a]() target"), Vec::<&str>::new());
        assert_eq!(link_targets("no links at all"), Vec::<&str>::new());
        assert_eq!(target_head("p.md \"a title\""), "p.md");
    }

    // spec: gate-sdk/SPEC.md §The fourth budget batch — the `..` join keeps climbing past the
    // start, which is what makes an off-root target expressible at all
    #[test]
    fn the_normalizer_keeps_a_leading_climb_and_collapses_the_rest() {
        assert_eq!(normalize_rel("docs/kit/../index.md"), "docs/index.md");
        assert_eq!(normalize_rel("docs/../../OUTSIDE.md"), "../OUTSIDE.md");
        assert_eq!(normalize_rel("docs/./a//b.md"), "docs/a/b.md");
        assert_eq!(normalize_rel("docs/.."), ".");
        assert_eq!(normalize_rel("../../a"), "../../a");
    }

    #[test]
    fn an_absolute_join_is_expressed_against_the_invoking_directory() {
        assert_eq!(relative_to("/a/b/c/d.md", "/a/b"), "c/d.md");
        assert_eq!(relative_to("/a/x.md", "/a/b/c"), "../../x.md");
        assert_eq!(relative_to("/a/b", "/a/b"), ".");
    }

    #[test]
    fn the_exempt_marker_covers_its_own_line_and_the_one_above() {
        let lines = vec!["<!-- docs-link-exempt: why -->", "[a](kit/)", "[b](kit/)"];
        assert!(exempt(&lines, 2));
        assert!(exempt(&lines, 1));
        assert!(!exempt(&lines, 3));
    }

    #[test]
    fn the_path_splitters_answer_the_root_and_the_bare_name_cases() {
        assert_eq!(dir_of("docs/kit/index.md"), "docs/kit");
        assert_eq!(dir_of("index.md"), ".");
        assert_eq!(dir_of("/index.md"), "/");
        assert_eq!(base_of("docs/kit/index.md"), "index.md");
        assert_eq!(base_of("index.md"), "index.md");
    }
}
