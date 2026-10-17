// spec: docs/agent-index.md §The arm and its gate — the agent
// index: `llms.txt` and `llms-full.txt` projected from the nav model, one rendering the gate calls in
// process
use crate::emit::product_statement as ps;
use crate::fresh;
use crate::nav::{self, Model};

pub const ROOT_KNOB: &str = "SITE_KIT_DOCS_DIR";
pub const INDEX_FILE: &str = "llms.txt";
pub const FULL_FILE: &str = "llms-full.txt";
const CONFIG_FILE: &str = "_config.yml";
const HOST_FILE: &str = "CNAME";
const HOME_PAGE: &str = "index.md";
// spec: docs/agent-index.md §llms-full.txt — the emitter declares the doors of a wholly generated
// file, whose every door is copied from a page the door gate sweeps at its source
const DOOR_DECLARATION: &str =
    "<!-- door-contributor: every door below is copied from a menu page the door gate sweeps at its source -->";

pub struct Rendered {
    pub index: String,
    pub full: String,
}

struct Item {
    label: String,
    url: String,
    note: Option<String>,
}

fn read(path: &str) -> Result<String, String> {
    fresh::read_captured(path).map_err(|e| format!("{}: {}", path, e))
}

// spec: docs/agent-index.md §Inputs — the site
// title is the first top-level `title:` line of the config
fn site_title(config: &str, text: &str) -> Result<String, String> {
    text.lines()
        .find_map(|l| l.strip_prefix("title:"))
        .map(|v| v.trim())
        .map(|v| {
            for q in ['"', '\''] {
                if v.len() >= 2 && v.starts_with(q) && v.ends_with(q) {
                    return v[1..v.len() - 1].to_string();
                }
            }
            v.to_string()
        })
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("{}: no title: line", config))
}

// spec: docs/agent-index.md §Inputs — a page's URL
// as the Pages build serves it: `index.md` as its directory, any other page `.html` for `.md`
fn served(rel: &str) -> String {
    if rel == HOME_PAGE {
        return String::new();
    }
    if let Some(dir) = rel.strip_suffix(&format!("/{}", HOME_PAGE)) {
        // path-dialect-exempt: a served URL path, its directory form ending in a slash
        return format!("{}/", dir);
    }
    match rel.strip_suffix(".md") {
        Some(stem) => format!("{}.html", stem),
        None => rel.to_string(),
    }
}

struct Site<'a> {
    model: &'a Model,
    host: String,
}

impl Site<'_> {
    fn rel<'p>(&self, page: &'p str) -> &'p str {
        crate::walk::rel_under(&self.model.root, page).unwrap_or(page)
    }

    fn url(&self, page: &str) -> String {
        format!("https://{}/{}", self.host, served(self.rel(page)))
    }

    fn title(&self, page: &str) -> Result<String, String> {
        self.model.fms[page]
            .title
            .clone()
            .ok_or_else(|| format!("{}: a nav page with no title:", page))
    }

    fn item(&self, page: &str) -> Result<Item, String> {
        Ok(Item {
            label: self.title(page)?,
            url: self.url(page),
            note: self.model.fms[page].description.clone(),
        })
    }

    // spec: docs/agent-index.md §llms-full.txt — a
    // relative target resolves against the page's directory, lexically, never past the docs root
    fn resolve(&self, page: &str, target: &str) -> Result<String, String> {
        let (path, anchor) = match target.split_once('#') {
            Some((p, a)) => (p, Some(a)),
            None => (target, None),
        };
        // path-dialect-exempt: a link target is a URL path the site serves, never a filesystem location
        let rooted = path.starts_with('/');
        let mut parts: Vec<&str> = Vec::new();
        if !rooted {
            parts = self.rel(page).split('/').collect();
            parts.pop();
        }
        for seg in path.split('/') {
            match seg {
                "" | "." => {}
                ".." => {
                    if parts.pop().is_none() {
                        return Err(format!("{}: link target {} resolves outside the docs root", page, target));
                    }
                }
                s => parts.push(s),
            }
        }
        let joined = parts.join("/");
        let joined = if path.ends_with('/') && !joined.is_empty() {
            // path-dialect-exempt: a served URL path keeps the target's trailing slash
            format!("{}/", joined)
        } else {
            joined
        };
        let mut url = if joined.ends_with(".md") {
            self.url(&format!("{}/{}", self.model.root, joined))
        } else {
            format!("https://{}/{}", self.host, joined)
        };
        if let Some(a) = anchor {
            url.push('#');
            url.push_str(a);
        }
        Ok(url)
    }

    fn rewrite_target(&self, page: &str, target: &str) -> Result<String, String> {
        let scheme = target
            .split_once(':')
            .map(|(s, _)| {
                s.starts_with(|c: char| c.is_ascii_alphabetic())
                    && s.chars().all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c))
            })
            .unwrap_or(false);
        if scheme {
            return Ok(target.to_string());
        }
        if target.starts_with('#') {
            return Ok(format!("{}{}", self.url(page), target));
        }
        self.resolve(page, target)
    }

    // spec: docs/agent-index.md §llms-full.txt — every
    // inline `](target)` outside a code span takes its absolute URL, a title after the target kept
    fn rewrite_links(&self, page: &str, line: &str) -> Result<String, String> {
        let b = line.as_bytes();
        let mut out = String::new();
        let mut i = 0usize;
        let mut copied = 0usize;
        while i < b.len() {
            if b[i] == b'`' {
                let run = b[i..].iter().take_while(|&&c| c == b'`').count();
                let close = (i + run..b.len()).find(|&j| {
                    b[j..].iter().take_while(|&&c| c == b'`').count() == run && (j == 0 || b[j - 1] != b'`')
                });
                i = match close {
                    Some(j) => j + run,
                    None => i + run,
                };
                continue;
            }
            if b[i] == b']' && i + 1 < b.len() && b[i + 1] == b'(' {
                let start = i + 2;
                if let Some(len) = line[start..].find(')') {
                    let raw = &line[start..start + len];
                    let (tgt, rest) = match raw.find(' ') {
                        Some(k) => (&raw[..k], &raw[k..]),
                        None => (raw, ""),
                    };
                    if !tgt.is_empty() && !tgt.starts_with('<') {
                        out.push_str(&line[copied..start]);
                        out.push_str(&self.rewrite_target(page, tgt)?);
                        out.push_str(rest);
                        copied = start + len;
                    }
                    i = start + len;
                    continue;
                }
            }
            i += 1;
        }
        out.push_str(&line[copied..]);
        Ok(out)
    }

    // spec: docs/agent-index.md §llms-full.txt — a body:
    // front matter dropped, every comment outside a fence dropped, every relative link absolute
    fn body(&self, page: &str) -> Result<String, String> {
        let text = &self.model.texts[page];
        let lines: Vec<&str> = fresh::file_lines(text);
        let span = crate::spec::front_matter_span(&lines);
        let mut fence = crate::spec::Fence::with_tilde();
        let mut in_comment = false;
        let mut swallow = false;
        let mut out: Vec<String> = Vec::new();
        for &line in &lines[span..] {
            if !in_comment && (fence.delimits(line) || fence.is_open()) {
                swallow = false;
                out.push(line.to_string());
                continue;
            }
            if std::mem::take(&mut swallow) && line.trim().is_empty() {
                continue;
            }
            let mut kept = String::new();
            let mut rest = line;
            let mut cut = in_comment;
            loop {
                if in_comment {
                    match rest.find("-->") {
                        Some(k) => {
                            rest = &rest[k + 3..];
                            in_comment = false;
                        }
                        None => break,
                    }
                } else {
                    match rest.find("<!--") {
                        Some(k) => {
                            kept.push_str(&rest[..k]);
                            rest = &rest[k + 4..];
                            in_comment = true;
                            cut = true;
                        }
                        None => {
                            kept.push_str(rest);
                            break;
                        }
                    }
                }
            }
            // spec: docs/agent-index.md §llms-full.txt — a
            // comment filling its lines drops them, and one blank line after them where one precedes
            if cut && kept.trim().is_empty() {
                swallow = !in_comment && out.last().map_or(true, |l| l.trim().is_empty());
                continue;
            }
            out.push(self.rewrite_links(page, &kept)?);
        }
        let body = out.join("\n");
        let body = body.trim_matches('\n').to_string();
        if body.contains("{{") || body.contains("{%") {
            return Err(format!(
                "{}: carries a Liquid tag, whose raw source would publish template syntax",
                page
            ));
        }
        Ok(body)
    }
}

fn line_of(item: &Item) -> String {
    match &item.note {
        Some(n) => format!("- [{}]({}): {}", item.label, item.url, n),
        None => format!("- [{}]({})", item.label, item.url),
    }
}

// spec: docs/agent-index.md §llms.txt — a section
// per top-level page, then `## Optional`; every refusal comes before anything renders
pub fn render(root: &str, source: &str) -> Result<Rendered, String> {
    let root = fresh::strip_trailing_slash(root);
    let config = format!("{}/{}", root, CONFIG_FILE);
    let title = site_title(&config, &read(&config)?)?;
    let cname = format!("{}/{}", root, HOST_FILE);
    let host = read(&cname)?.lines().next().unwrap_or("").trim().to_string();
    if host.is_empty() {
        return Err(format!("{}: empty", cname));
    }
    let summary = ps::plain(&ps::parse(source, &read(source)?)?);
    let model = nav::load(root)?;
    let site = Site { model: &model, host };
    let home = format!("{}/{}", root, HOME_PAGE);
    if !model.texts.contains_key(&home) {
        return Err(format!("{}: unreadable or absent", home));
    }

    let header = format!("# {}\n\n> {}\n", title, summary);
    let mut index = header.clone();
    let mut full = format!("{}\n{}", DOOR_DECLARATION, header);
    let mut optional: Vec<Item> = Vec::new();
    let mut tail: Vec<Item> = Vec::new();
    let mut bodies: Vec<String> = vec![home.clone()];
    for top in model.top_level() {
        if *top == home {
            continue;
        }
        let fm = &model.fms[top];
        if !fm.children_key.is_empty() {
            tail.push(Item {
                note: None,
                ..site.item(top)?
            });
            continue;
        }
        let mut items: Vec<&String> = vec![top];
        items.extend(model.children_of(&fm.id));
        index.push_str(&format!("\n## {}\n\n", site.title(top)?));
        for page in items {
            let item = site.item(page)?;
            index.push_str(&line_of(&item));
            index.push('\n');
            bodies.push(page.clone());
            let sibs = if model.is_child(page) {
                model.generated_siblings(page)
            } else {
                Vec::new()
            };
            for sib in sibs {
                optional.push(Item {
                    label: format!("{} {}", item.label, site.title(sib)?),
                    url: site.url(sib),
                    note: None,
                });
            }
            for pair in &model.fms[page].suffix {
                let (label, target) = nav::suffix_target(root, pair)
                    .filter(|(_, t)| model.texts.contains_key(t))
                    .ok_or_else(|| format!("{}: nav_suffix pair {} names no docs page", page, pair))?;
                optional.push(Item {
                    label: format!("{} {}", item.label, label),
                    url: site.url(&target),
                    note: None,
                });
            }
        }
    }
    optional.extend(tail);
    if !optional.is_empty() {
        index.push_str("\n## Optional\n\n");
        for item in &optional {
            index.push_str(&line_of(item));
            index.push('\n');
        }
    }
    for page in &bodies {
        full.push_str(&format!("\nSource: {}\n\n{}\n", site.url(page), site.body(page)?));
    }
    Ok(Rendered { index, full })
}

pub fn paths(root: &str) -> (String, String) {
    let root = fresh::strip_trailing_slash(root);
    (format!("{}/{}", root, INDEX_FILE), format!("{}/{}", root, FULL_FILE))
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the bare arm prints `llms.txt`; `--write` writes both
// files, and neither until both render
pub fn emit(args: &[String]) -> Result<String, String> {
    let root = crate::walk::knob_scalar(ROOT_KNOB)?;
    let source = crate::walk::knob_scalar(ps::SOURCE_KNOB)?;
    let r = render(&root, &source)?;
    if !args.iter().any(|a| a == "--write") {
        return Ok(r.index);
    }
    let (index, full) = paths(&root);
    let mut out = String::new();
    for (path, text) in [(&index, &r.index), (&full, &r.full)] {
        if std::fs::read_to_string(path).ok().as_deref() != Some(text.as_str()) {
            std::fs::write(path, text).map_err(|e| format!("cannot write {}: {}", path, e))?;
            out.push_str(&format!("llms: wrote {}\n", path));
        }
    }
    if out.is_empty() {
        out.push_str("llms: both files already fresh\n");
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn model(pages: &[(&str, &str)]) -> Model {
        let mut texts = HashMap::new();
        let mut fms = HashMap::new();
        for (p, t) in pages {
            texts.insert(p.to_string(), t.to_string());
            fms.insert(p.to_string(), nav::front_matter(t));
        }
        Model {
            root: "d".into(),
            pages: pages.iter().map(|(p, _)| p.to_string()).collect(),
            texts,
            fms,
            top_nav_ids: HashSet::new(),
            derived: HashSet::new(),
        }
    }

    #[test]
    fn a_page_url_is_its_directory_for_an_index_and_html_otherwise() {
        assert_eq!(served("index.md"), "");
        assert_eq!(served("kit/index.md"), "kit/");
        assert_eq!(served("kit/SPEC.md"), "kit/SPEC.html");
        assert_eq!(served("a.png"), "a.png");
    }

    // spec: docs/agent-index.md §llms-full.txt — a
    // scheme stays, an anchor takes the page's URL, a relative path resolves and keeps its anchor
    #[test]
    fn a_link_target_takes_its_absolute_url_and_refuses_past_the_root() {
        let m = model(&[("d/kit/index.md", "")]);
        let s = Site { model: &m, host: "h.dev".into() };
        let page = "d/kit/index.md";
        assert_eq!(s.rewrite_target(page, "https://x/y.md").unwrap(), "https://x/y.md");
        assert_eq!(s.rewrite_target(page, "mailto:a@b").unwrap(), "mailto:a@b");
        assert_eq!(s.rewrite_target(page, "#sec").unwrap(), "https://h.dev/kit/#sec");
        assert_eq!(s.rewrite_target(page, "SPEC.md#a-b").unwrap(), "https://h.dev/kit/SPEC.html#a-b");
        assert_eq!(s.rewrite_target(page, "../index.md").unwrap(), "https://h.dev/");
        assert_eq!(s.rewrite_target(page, "../graph.html").unwrap(), "https://h.dev/graph.html");
        assert!(s.rewrite_target(page, "../../x.md").is_err());
    }

    // spec: docs/agent-index.md §llms-full.txt — a body
    // drops front matter and comments outside a fence, and rewrites no link inside a fence or code
    #[test]
    fn a_body_drops_front_matter_and_comments_and_leaves_code_alone() {
        let text = "---\ntitle: A\n---\n<!-- whole -->\n# A\n\nsee [b](b.md) `[c](c.md)` x<!-- tail -->\n<!-- open\nstill -->\n```\n[d](d.md) <!-- kept -->\n```\n";
        let m = model(&[("d/a.md", text)]);
        let s = Site { model: &m, host: "h".into() };
        assert_eq!(
            s.body("d/a.md").unwrap(),
            "# A\n\nsee [b](https://h/b.html) `[c](c.md)` x\n```\n[d](d.md) <!-- kept -->\n```"
        );
        let blanks = model(&[("d/b.md", "a\n\n<!-- c -->\n\nb\n")]);
        let s = Site { model: &blanks, host: "h".into() };
        assert_eq!(s.body("d/b.md").unwrap(), "a\n\nb");
        let liquid = model(&[("d/l.md", "{{ site.x }}\n")]);
        let s = Site { model: &liquid, host: "h".into() };
        assert!(s.body("d/l.md").is_err());
    }

    #[test]
    fn the_site_title_drops_its_quotes_and_an_absent_one_refuses() {
        assert_eq!(site_title("c", "a: b\ntitle: \"T\"\n").unwrap(), "T");
        assert!(site_title("c", "  title: T\n").is_err());
        assert!(site_title("c", "title:\n").is_err());
    }
}
