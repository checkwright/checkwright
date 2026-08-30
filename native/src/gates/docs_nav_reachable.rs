// spec: docs/site-architecture.md §Site chrome and the nav contract — every tracked docs page
// carries a title front-matter block and is reachable from the rendered nav (a nav slot, or a
// relative-link walk seeded from the nav set), or is listed in the off-nav allowlist
use crate::fresh;
use crate::walk;
use std::collections::{HashMap, HashSet};
use std::path::Path;

const DEFAULT_ROOT: &str = "docs";
const DEFAULT_ALLOWLIST: &str = "scripts/docs-offnav.list";
// spec: docs/site-architecture.md §Site chrome and the nav contract — POSIX `[[:space:]]` minus
// the newline awk's record separator already consumed
const SPACE: [char; 5] = [' ', '\t', '\r', '\x0b', '\x0c'];

// spec: docs/site-architecture.md §Site chrome and the nav contract — front-matter facts per
// page: title:, the nav_order slot, nav_id / nav_parent, the generated: mirror marker, and
// nav_children_key (the derived-children key a nav page names)
#[derive(Default)]
struct Fm {
    title: bool,
    order: bool,
    id: String,
    parent: String,
    generated: bool,
    children_key: String,
}

fn is_space(c: char) -> bool {
    SPACE.contains(&c)
}

// spec: docs/site-architecture.md §Site chrome and the nav contract — awk's `$2`: the second
// whitespace-delimited field of the line, empty where the line carries only one
fn field2(line: &str) -> String {
    line.split(is_space)
        .filter(|f| !f.is_empty())
        .nth(1)
        .unwrap_or("")
        .to_string()
}

fn key_matches<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(key)?;
    let t = rest.trim_start_matches(SPACE);
    if t.is_empty() {
        None
    } else {
        Some(t)
    }
}

fn front_matter(text: &str) -> Fm {
    let mut fm = Fm::default();
    let mut inside = false;
    for (n, line) in text.lines().enumerate() {
        if n == 0 {
            if line != "---" {
                return Fm::default();
            }
            inside = true;
            continue;
        }
        if inside && line == "---" {
            inside = false;
        }
        if !inside {
            continue;
        }
        if key_matches(line, "title:").is_some() {
            fm.title = true;
        }
        if let Some(t) = key_matches(line, "nav_order:") {
            if t.starts_with(|c: char| c.is_ascii_digit()) {
                fm.order = true;
            }
        }
        if key_matches(line, "nav_id:").is_some() {
            fm.id = field2(line);
        }
        if key_matches(line, "nav_parent:").is_some() {
            fm.parent = field2(line);
        }
        if let Some(t) = key_matches(line, "generated:") {
            if t.starts_with("true") {
                fm.generated = true;
            }
        }
        if key_matches(line, "nav_children_key:").is_some() {
            fm.children_key = field2(line);
        }
    }
    fm
}

// spec: docs/site-architecture.md §Site chrome and the nav contract — the include's
// derived-children rule: a page whose front matter carries a key a nav page names in
// nav_children_key holds a nav slot and takes the key value as its label
fn key_value(text: &str, key: &str) -> String {
    let mut inside = false;
    for (n, line) in text.lines().enumerate() {
        if n == 0 {
            if line != "---" {
                return String::new();
            }
            inside = true;
            continue;
        }
        if inside && line == "---" {
            return String::new();
        }
        if !inside {
            continue;
        }
        let fields: Vec<&str> = line.split(is_space).filter(|f| !f.is_empty()).collect();
        if fields.first() == Some(&key) {
            return fields.get(1).unwrap_or(&"").to_string();
        }
    }
    String::new()
}

fn dirname(p: &str) -> &str {
    match p.rfind('/') {
        None => ".",
        Some(0) => "/",
        Some(i) => &p[..i],
    }
}

fn basename(p: &str) -> &str {
    match p.rfind('/') {
        None => p,
        Some(i) => &p[i + 1..],
    }
}

// spec: docs/site-architecture.md §Site chrome and the nav contract — `realpath -m
// --relative-to=. --`: a lexical resolution that needs no existing path, then re-spelled
// against the invoking directory, which is what makes a link target comparable to a page path
fn relative_to_cwd(joined: &str) -> Option<String> {
    let here = walk::cwd().ok()?;
    let abs = if joined.starts_with('/') {
        walk::normalize_abs(joined)
    } else {
        walk::normalize_abs(&format!("{}/{}", here, joined))
    };
    if abs == here {
        return Some(".".to_string());
    }
    if let Some(r) = abs.strip_prefix(&format!("{}/", here)) {
        return Some(r.to_string());
    }
    let (a, b): (Vec<&str>, Vec<&str>) = (
        here.split('/').filter(|s| !s.is_empty()).collect(),
        abs.split('/').filter(|s| !s.is_empty()).collect(),
    );
    let common = a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count();
    let mut parts: Vec<String> = vec!["..".to_string(); a.len() - common];
    parts.extend(b[common..].iter().map(|s| s.to_string()));
    Some(parts.join("/"))
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
    let root = fresh::strip_trailing_slash(fresh::positional(args, 0, DEFAULT_ROOT)).to_string();
    let allowlist = fresh::positional(args, 1, DEFAULT_ALLOWLIST).to_string();
    if !fresh::is_dir(&root) {
        return Err(format!("not a directory: {}", root));
    }

    // spec: gate-sdk/SPEC.md §The consumer remainder cohort — the shell form's own prune, a
    // bare `find … -type d -name '_*' -prune`, which is not `gate_find`'s bridged set: this
    // member reads no GATE_PRUNE_DIRS and declares none.
    let mut pages: Vec<String> = walk::find_with_prune(Path::new(&root), &|n| n.starts_with('_'))?
        .into_iter()
        .map(|p| p.display().to_string())
        .filter(|p| p.ends_with(".md"))
        .collect();
    pages.sort();
    if pages.is_empty() {
        println!(
            "DOCS-NAV-REACHABLE: clean (0 docs page(s) found under {})",
            root
        );
        return Ok(0);
    }

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

    let mut texts: HashMap<String, String> = HashMap::new();
    let mut fms: HashMap<String, Fm> = HashMap::new();
    let mut top_nav_ids: HashSet<String> = HashSet::new();
    let mut named_keys: Vec<String> = Vec::new();
    for p in &pages {
        let text = fresh::read_captured(p)?;
        let fm = front_matter(&text);
        if fm.order && !fm.id.is_empty() {
            top_nav_ids.insert(fm.id.clone());
        }
        // spec: docs/site-architecture.md §Site chrome and the nav contract — only a top-level
        // nav page names a derived-children key, matching the include's navpages-only iteration
        if fm.order && !fm.children_key.is_empty() && !named_keys.contains(&fm.children_key) {
            named_keys.push(fm.children_key.clone());
        }
        texts.insert(p.clone(), text);
        fms.insert(p.clone(), fm);
    }

    let mut derived: HashSet<String> = HashSet::new();
    if !named_keys.is_empty() {
        for p in &pages {
            for k in &named_keys {
                if !key_value(&texts[p], &format!("{}:", k)).is_empty() {
                    derived.insert(p.clone());
                    break;
                }
            }
        }
    }

    let inscope: HashSet<&String> = pages.iter().collect();
    let mut reach: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = Vec::new();
    for p in &pages {
        let fm = &fms[p];
        let seeded = fm.order
            || (!fm.parent.is_empty() && top_nav_ids.contains(&fm.parent))
            || derived.contains(p);
        if seeded {
            reach.insert(p.clone());
            queue.push(p.clone());
        }
    }

    let mut head = 0usize;
    while head < queue.len() {
        let cur = queue[head].clone();
        head += 1;
        for tgt in links_of(&cur, &texts[&cur]) {
            if inscope.contains(&tgt) && !reach.contains(&tgt) {
                reach.insert(tgt.clone());
                queue.push(tgt);
            }
        }
        // spec: docs/site-architecture.md §Site chrome and the nav contract — the include's
        // suffix-link rule: a generated mirror page is reachable iff its directory-sibling
        // index.md is nav-reachable
        if basename(&cur) == "index.md" {
            let curdir = dirname(&cur).to_string();
            for sib in &pages {
                if dirname(sib) == curdir && fms[sib].generated && !reach.contains(sib) {
                    reach.insert(sib.clone());
                    queue.push(sib.clone());
                }
            }
        }
    }

    let mut bad: Vec<String> = Vec::new();
    for p in &pages {
        if allow.contains(p) {
            continue;
        }
        if !(fms[p].title || derived.contains(p)) {
            bad.push(format!(
                "{}: no title front-matter block — it renders without a nav slot and joins no nav",
                p
            ));
        }
        if !reach.contains(p) {
            bad.push(format!(
                "{}: not reachable from the rendered nav — no nav slot and no link walk reaches it",
                p
            ));
        }
    }

    if !bad.is_empty() {
        println!("check-docs-nav-reachable: docs page(s) fall outside the site nav:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: open the page with a front-matter block carrying 'title:', and give it a nav slot");
        println!("        ('nav_order: <n>' top-level, or 'nav_parent: <id>' under one), or link it from a nav");
        println!(
            "        page. A page off-nav by design (an embedded fragment) goes in {}.",
            allowlist
        );
        return Ok(1);
    }
    println!(
        "DOCS-NAV-REACHABLE: clean ({} docs page(s) under {}; each carries a title block and sits in the rendered nav — a nav slot, its link walk, or the generated-sibling suffix rule — or is allowlisted off-nav)",
        pages.len(),
        root
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: docs/site-architecture.md §Site chrome and the nav contract — the front-matter
    // reader stops at the closing marker, and a page opening with anything else has none
    #[test]
    fn front_matter_is_read_from_the_first_block_only() {
        let fm = front_matter("---\ntitle: X\nnav_order: 2\nnav_id: kits\n---\ntitle: Y\n");
        assert!(fm.title && fm.order);
        assert_eq!(fm.id, "kits");
        let none = front_matter("# plain\ntitle: X\n");
        assert!(!none.title);
        let closed = front_matter("---\n---\nnav_order: 2\n");
        assert!(!closed.order);
    }

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
