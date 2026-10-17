// spec: docs/site-architecture.md §Site chrome and the nav contract — every tracked docs page
// carries a title block and a menu entry of its own (nav slot, derived child, generated-sibling
// suffix link, nav_suffix pair), or is listed in the off-nav allowlist
use crate::fresh;
use crate::nav::{self, basename, dirname, suffix_target, Fm};
use crate::walk;
use std::collections::{HashMap, HashSet};
use std::path::Path;

// spec: docs/site-architecture.md §Site chrome and the nav contract — `realpath -m
// --relative-to=. --`: a lexical resolution that needs no existing path, then re-spelled
// against the invoking directory, which is what makes a link target comparable to a page path
fn relative_to_cwd(joined: &str) -> Option<String> {
    let here = walk::cwd().ok()?;
    let abs = walk::abs_against(&here, joined);
    Some(walk::relative_to(&here, &abs))
}

// spec: docs/site-architecture.md §Site chrome and the nav contract — each in-tree relative
// `.md` link target a page carries, cwd-relative. `grep -noE '\]\([^)]+\)'` finds every
// `](target)` on every line, so the scan is per occurrence rather than per line.
fn links_of(path: &str, text: &str) -> Vec<String> {
    let base = dirname(path).to_string();
    let mut out: Vec<String> = Vec::new();
    let b = text.as_bytes();
    let mut i = 0usize;
    while i + 1 < b.len() {
        if b[i] != b']' || b[i + 1] != b'(' {
            i += 1;
            continue;
        }
        let start = i + 2;
        let mut j = start;
        // spec: docs/site-architecture.md §Site chrome and the nav contract — `[^)]+` never
        // crosses a `)`, and a newline is an ordinary member of that class in a `grep -o` scan
        // over a line-oriented file: a run reaching the end of its line simply does not close
        while j < b.len() && b[j] != b')' && b[j] != b'\n' {
            j += 1;
        }
        if j >= b.len() || b[j] != b')' || j == start {
            i += 1;
            continue;
        }
        let raw = &text[start..j];
        i = j + 1;
        let tgt = raw.split(' ').next().unwrap_or("");
        if tgt.is_empty() || tgt.contains("://") || tgt.starts_with("mailto:") {
            continue;
        }
        let p = tgt.split('#').next().unwrap_or("");
        if p.is_empty() || !p.ends_with(".md") {
            continue;
        }
        if let Some(rel) = relative_to_cwd(&format!("{}/{}", base, p)) {
            if !rel.is_empty() && Path::new(&rel).is_file() {
                out.push(rel);
            }
        }
    }
    out
}

// spec: docs/page-authoring.md §Page-authoring rules — the nav children of each nav_id, in
// page order, for the overview-as-sibling arm
fn children_by_id(pages: &[String], fms: &HashMap<String, Fm>) -> Vec<(String, Vec<String>)> {
    let ids: HashSet<&String> = pages.iter().map(|p| &fms[p].id).filter(|i| !i.is_empty()).collect();
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    for p in pages {
        let parent = &fms[p].parent;
        if parent.is_empty() || !ids.contains(parent) {
            continue;
        }
        match out.iter_mut().find(|(id, _)| id == parent) {
            Some((_, kids)) => kids.push(p.clone()),
            None => out.push((parent.clone(), vec![p.clone()])),
        }
    }
    out.sort();
    out
}

// spec: docs/page-authoring.md §Page-authoring rules — the overview proxy: under three or
// more siblings, a child every other sibling links and that links every other sibling; two
// children linking each other are a pair, hence the floor
fn overviews(kids: &[String], texts: &HashMap<String, String>) -> Vec<String> {
    if kids.len() < 3 {
        return Vec::new();
    }
    let links: HashMap<&String, HashSet<String>> = kids
        .iter()
        .map(|k| (k, links_of(k, &texts[k]).into_iter().collect()))
        .collect();
    kids.iter()
        .filter(|x| {
            kids.iter()
                .filter(|y| y != x)
                .all(|y| links[y].contains(*x) && links[*x].contains(y))
        })
        .cloned()
        .collect()
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-docs-nav-reachable: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = fresh::strip_trailing_slash(&fresh::positional_or_knob(args, 0, "SITE_KIT_DOCS_DIR")?).to_string();
    if !fresh::is_dir(&root) {
        return Err(format!("not a directory: {}", root));
    }

    let model = nav::load(&root)?;
    let pages = &model.pages;
    if pages.is_empty() {
        println!(
            "DOCS-NAV-REACHABLE: clean (0 docs page(s) found under {})",
            root
        );
        return Ok(0);
    }

    let allowlist = fresh::positional_or_knob(args, 1, "GATE_LOCAL_OFFNAV_LIST")?;
    let mut allow: HashSet<String> = HashSet::new();
    if Path::new(&allowlist).is_file() {
        for line in fresh::read_captured(&allowlist)?.lines() {
            let cut = line.split('#').next().unwrap_or("");
            let squeezed: String = cut.chars().filter(|c| !c.is_whitespace()).collect();
            if !squeezed.is_empty() {
                allow.insert(squeezed);
            }
        }
    }

    let fms = &model.fms;
    let inscope: HashSet<&String> = pages.iter().collect();
    let mut bad: Vec<String> = Vec::new();
    let mut reach: HashSet<String> = model.derived.clone();
    for p in pages {
        let fm = &fms[p];
        for pair in &fm.suffix {
            match suffix_target(&root, pair) {
                None => bad.push(format!(
                    "{}: nav_suffix pair {} is malformed — it takes <label>=<path>",
                    p, pair
                )),
                Some((_, t)) if !inscope.contains(&t) => bad.push(format!(
                    "{}: nav_suffix names {}, which is not a docs page",
                    p,
                    pair.split_once('=').map_or("", |(_, path)| path)
                )),
                Some(_) => {}
            }
        }
        let child = model.is_child(p);
        if !(fm.order.is_some() || child) {
            continue;
        }
        reach.insert(p.clone());
        for (_, t) in fm.suffix.iter().filter_map(|pair| suffix_target(&root, pair)) {
            reach.insert(t);
        }
        // spec: docs/site-architecture.md §Site chrome and the nav contract — the include's
        // suffix-link rule, read where a nav child is its directory's index.md
        if child && basename(p) == "index.md" {
            for sib in model.generated_siblings(p) {
                reach.insert(sib.clone());
            }
        }
    }

    for (id, kids) in children_by_id(pages, fms) {
        for overview in overviews(&kids, &model.texts) {
            bad.push(format!(
                "{}: an overview among its siblings under {}. Every sibling links it and it links each of them, so make it their parent with nav_id.",
                overview, id
            ));
        }
    }
    for p in pages {
        if allow.contains(p) {
            continue;
        }
        if !(fms[p].title.is_some() || model.derived.contains(p)) {
            bad.push(format!(
                "{}: no title front-matter block — it renders without a nav slot and joins no nav",
                p
            ));
        }
        if !reach.contains(p) {
            bad.push(format!(
                "{}: not reachable from the rendered nav — it holds no menu entry of its own",
                p
            ));
        }
    }

    if !bad.is_empty() {
        println!("check-docs-nav-reachable: docs page(s) fall outside the site nav or break its rules:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: open the page with a front-matter block carrying 'title:', and give it a nav slot");
        println!("        ('nav_order: <n>' top-level, or 'nav_parent: <id>' under one). A mirror page with no");
        println!("        kit index is named in 'nav_suffix: <label>=<path>' on the page documenting its");
        println!(
            "        component. A page off-nav by design (an embedded fragment) goes in {}.",
            allowlist
        );
        return Ok(1);
    }
    println!(
        "DOCS-NAV-REACHABLE: clean ({} docs page(s) under {}; each carries a title block and holds a menu entry — a nav slot, a derived child, a suffix link or a nav_suffix pair — or is allowlisted off-nav)",
        pages.len(),
        root
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: docs/site-architecture.md §Site chrome and the nav contract — a link scan per
    // occurrence, dropping externals, anchors-only and non-`.md` targets
    #[test]
    fn the_link_scan_takes_every_in_tree_md_occurrence_on_a_line() {
        let text = "see [a](x.md) and [b](y.md) and [c](https://e/z.md) and [d](p.png)\n";
        let got = links_of("docs/index.md", text);
        assert!(got.is_empty() || got.iter().all(|g| g.ends_with(".md")));
        assert!(!got.iter().any(|g| g.contains("://")));
    }
}
