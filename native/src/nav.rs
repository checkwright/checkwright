// spec: docs/site-architecture.md §Site chrome and the nav contract — the nav model the menu include
// renders, read once for `check-docs-nav-reachable` and for the agent index, so the two cannot
// disagree on which pages hold a slot
use crate::fresh;
use crate::walk;
use std::collections::{HashMap, HashSet};
use std::path::Path;

// spec: docs/site-architecture.md §Site chrome and the nav contract — POSIX `[[:space:]]` minus
// the newline awk's record separator already consumed
const SPACE: [char; 5] = [' ', '\t', '\r', '\x0b', '\x0c'];

// spec: docs/site-architecture.md §Site chrome and the nav contract — the front-matter facts the
// include reads per page, with the values the agent index orders and annotates by
#[derive(Default)]
pub struct Fm {
    pub title: Option<String>,
    pub order: Option<u64>,
    pub child_order: Option<u64>,
    pub id: String,
    pub parent: String,
    pub generated: bool,
    pub children_key: String,
    pub suffix: Vec<String>,
    pub description: Option<String>,
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

// spec: docs/agent-index.md §Inputs — a value
// with one pair of enclosing quotes dropped
fn unquoted(t: &str) -> String {
    let t = t.trim_end_matches(SPACE);
    for q in ['"', '\''] {
        if t.len() >= 2 && t.starts_with(q) && t.ends_with(q) {
            return t[1..t.len() - 1].to_string();
        }
    }
    t.to_string()
}

// spec: docs/site-architecture.md §Site chrome and the nav contract — a slot is a digit-led
// value, its order the leading digit run
fn slot(t: &str) -> Option<u64> {
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits.parse().unwrap_or(u64::MAX))
    }
}

pub fn front_matter(text: &str) -> Fm {
    let mut fm = Fm::default();
    let lines: Vec<&str> = text.lines().collect();
    for &line in &lines[..crate::spec::front_matter_span(&lines)] {
        if let Some(t) = key_matches(line, "title:") {
            fm.title = Some(unquoted(t));
        }
        if let Some(t) = key_matches(line, "nav_order:") {
            if let Some(n) = slot(t) {
                fm.order = Some(n);
            }
        }
        if let Some(t) = key_matches(line, "nav_child_order:") {
            if let Some(n) = slot(t) {
                fm.child_order = Some(n);
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
        if let Some(t) = key_matches(line, "nav_suffix:") {
            fm.suffix = t.split(is_space).filter(|f| !f.is_empty()).map(String::from).collect();
        }
        if let Some(t) = key_matches(line, "description:") {
            fm.description = Some(unquoted(t));
        }
    }
    fm
}

// spec: docs/site-architecture.md §Site chrome and the nav contract — a nav_suffix pair is
// `<label>=<path>`, the path relative to the docs root; None where either side is empty
pub fn suffix_target(root: &str, pair: &str) -> Option<(String, String)> {
    let (label, path) = pair.split_once('=')?;
    if label.is_empty() || path.is_empty() {
        return None;
    }
    Some((label.to_string(), format!("{}/{}", root, path)))
}

// spec: docs/site-architecture.md §Site chrome and the nav contract — the include's
// derived-children rule: a page whose front matter carries a key a nav page names in
// nav_children_key holds a nav slot and takes the key value as its label
pub fn key_value(text: &str, key: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    for line in &lines[..crate::spec::front_matter_span(&lines)] {
        let fields: Vec<&str> = line.split(is_space).filter(|f| !f.is_empty()).collect();
        if fields.first() == Some(&key) {
            return fields.get(1).unwrap_or(&"").to_string();
        }
    }
    String::new()
}

pub fn dirname(p: &str) -> &str {
    match p.rfind('/') {
        None => ".",
        Some(0) => "/",
        Some(i) => &p[..i],
    }
}

pub fn basename(p: &str) -> &str {
    match p.rfind('/') {
        None => p,
        Some(i) => &p[i + 1..],
    }
}

pub struct Model {
    pub root: String,
    pub pages: Vec<String>,
    pub texts: HashMap<String, String>,
    pub fms: HashMap<String, Fm>,
    pub top_nav_ids: HashSet<String>,
    pub derived: HashSet<String>,
}

// spec: gate-sdk/SPEC.md §The consumer remainder cohort — the shell form's own prune, a bare
// `find … -type d -name '_*' -prune`, which is not `gate_find`'s prune set: neither reader of
// the model reads a prune knob or declares one.
pub fn load(root: &str) -> Result<Model, String> {
    let mut pages: Vec<String> = walk::find_with_prune(Path::new(root), &|n| n.starts_with('_'))?
        .into_iter()
        .map(|p| p.display().to_string())
        .filter(|p| p.ends_with(".md"))
        .collect();
    pages.sort();
    let mut texts: HashMap<String, String> = HashMap::new();
    let mut fms: HashMap<String, Fm> = HashMap::new();
    let mut top_nav_ids: HashSet<String> = HashSet::new();
    let mut named_keys: Vec<String> = Vec::new();
    for p in &pages {
        let text = fresh::read_captured(p)?;
        let fm = front_matter(&text);
        if fm.order.is_some() && !fm.id.is_empty() {
            top_nav_ids.insert(fm.id.clone());
        }
        // spec: docs/site-architecture.md §Site chrome and the nav contract — only a top-level
        // nav page names a derived-children key, matching the include's navpages-only iteration
        if fm.order.is_some() && !fm.children_key.is_empty() && !named_keys.contains(&fm.children_key) {
            named_keys.push(fm.children_key.clone());
        }
        texts.insert(p.clone(), text);
        fms.insert(p.clone(), fm);
    }
    let mut derived: HashSet<String> = HashSet::new();
    for p in &pages {
        if named_keys
            .iter()
            .any(|k| !key_value(&texts[p], &format!("{}:", k)).is_empty())
        {
            derived.insert(p.clone());
        }
    }
    Ok(Model {
        root: root.to_string(),
        pages,
        texts,
        fms,
        top_nav_ids,
        derived,
    })
}

impl Model {
    pub fn is_child(&self, p: &str) -> bool {
        let parent = &self.fms[p].parent;
        !parent.is_empty() && self.top_nav_ids.contains(parent)
    }

    // spec: docs/site-architecture.md §Site chrome and the nav contract — the include's top level:
    // every page holding a slot, by `nav_order`
    pub fn top_level(&self) -> Vec<&String> {
        let mut top: Vec<&String> = self.pages.iter().filter(|p| self.fms[*p].order.is_some()).collect();
        top.sort_by_key(|p| (self.fms[*p].order, p.as_str()));
        top
    }

    // spec: docs/site-architecture.md §Site chrome and the nav contract — a nav page's children by
    // `nav_child_order`, a child carrying none first as Jekyll's sort places a nil
    pub fn children_of(&self, id: &str) -> Vec<&String> {
        if id.is_empty() || !self.top_nav_ids.contains(id) {
            return Vec::new();
        }
        let mut kids: Vec<&String> = self.pages.iter().filter(|p| self.fms[*p].parent == id).collect();
        kids.sort_by_key(|p| (self.fms[*p].child_order, p.as_str()));
        kids
    }

    // spec: docs/site-architecture.md §Site chrome and the nav contract — the include's suffix-link
    // rule: the `generated:` pages of a nav child's directory, by title
    pub fn generated_siblings(&self, child: &str) -> Vec<&String> {
        let dir = dirname(child);
        let mut sibs: Vec<&String> = self
            .pages
            .iter()
            .filter(|s| s.as_str() != child && dirname(s) == dir && self.fms[*s].generated)
            .collect();
        sibs.sort_by_key(|s| (self.fms[*s].title.clone().unwrap_or_default(), s.as_str()));
        sibs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: docs/site-architecture.md §Site chrome and the nav contract — the front-matter
    // reader stops at the closing marker, and a page opening with anything else has none
    #[test]
    fn front_matter_is_read_from_the_first_block_only() {
        let fm = front_matter("---\ntitle: X\nnav_order: 2\nnav_id: kits\n---\ntitle: Y\n");
        assert!(fm.title.is_some() && fm.order.is_some());
        assert_eq!(fm.title.as_deref(), Some("X"));
        assert_eq!(fm.order, Some(2));
        assert_eq!(fm.id, "kits");
        let none = front_matter("# plain\ntitle: X\n");
        assert!(none.title.is_none());
        let closed = front_matter("---\n---\nnav_order: 2\n");
        assert!(closed.order.is_none());
        let dotted = front_matter("---\ntitle: X\n...\nnav_order: 2\n---\n");
        assert!(dotted.title.is_some() && dotted.order.is_none());
        assert!(front_matter("---\ntitle: X\n").title.is_none());
        assert_eq!(key_value("---\nversion: 1\n...\n", "version:"), "1");
        assert_eq!(key_value("---\na: 1\n...\nversion: 1\n---\n", "version:"), "");
    }

    // spec: docs/agent-index.md §Inputs — a
    // title and a description drop one pair of enclosing quotes; a non-digit order holds no slot
    #[test]
    fn the_values_drop_one_pair_of_quotes_and_an_order_reads_its_digit_run() {
        let fm = front_matter("---\ntitle: \"A: b\"\ndescription: 'c d'\nnav_order: x\nnav_child_order: 12\n---\n");
        assert_eq!(fm.title.as_deref(), Some("A: b"));
        assert_eq!(fm.description.as_deref(), Some("c d"));
        assert_eq!(fm.order, None);
        assert_eq!(fm.child_order, Some(12));
        assert_eq!(unquoted("\"x"), "\"x");
    }

    // spec: docs/site-architecture.md §Site chrome and the nav contract — a pair resolves
    // against the docs root, and one lacking its `=` or either side resolves to nothing
    #[test]
    fn a_nav_suffix_pair_takes_label_equals_path() {
        assert_eq!(
            suffix_target("docs", "spec=installer/SPEC.md"),
            Some(("spec".to_string(), "docs/installer/SPEC.md".to_string()))
        );
        assert_eq!(suffix_target("docs", "installer/SPEC.md"), None);
        assert_eq!(suffix_target("docs", "spec="), None);
        assert_eq!(suffix_target("docs", "=x.md"), None);
    }
}
