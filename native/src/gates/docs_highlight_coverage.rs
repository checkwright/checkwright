// spec: site-kit/SPEC.md §check-docs-highlight-coverage — the layout overrides every token class the
// tracked snapshot declares the theme colours
use crate::fresh;
use crate::walk;
use crate::{proc, programs};
use std::path::Path;

const TOOL: &str = "check-docs-highlight-coverage";

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}: {}", TOOL, msg);
            eprintln!("DOCS-HIGHLIGHT-COVERAGE: {}", fresh::fail_closed("highlight-coverage", Some(2)));
            2
        }
    }
}

// spec: site-kit/SPEC.md §check-docs-highlight-coverage — a file carrying a `<style` element is read
// for its style bodies only; any other file is CSS throughout
fn css_text(raw: &str) -> String {
    let lower = raw.to_ascii_lowercase();
    if !lower.contains("<style") {
        return raw.to_string();
    }
    let mut out = String::new();
    let mut at = 0usize;
    while let Some(open) = lower[at..].find("<style") {
        let tag = at + open;
        let Some(gt) = lower[tag..].find('>') else { break };
        let body = tag + gt + 1;
        let end = lower[body..].find("</style").map(|e| body + e).unwrap_or(raw.len());
        out.push_str(&raw[body..end]);
        out.push('\n');
        at = end;
    }
    out
}

fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(open) = rest.find("/*") {
        out.push_str(&rest[..open]);
        match rest[open + 2..].find("*/") {
            Some(close) => rest = &rest[open + 2 + close + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

// spec: site-kit/SPEC.md §check-docs-highlight-coverage — every rule's selector list, split on commas
// with whitespace collapsed; the prelude of a rule is the text since the last `{`, `}` or `;`
fn rules(css: &str) -> Vec<Vec<String>> {
    let text = strip_comments(css);
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut start = 0usize;
    for (i, ch) in text.char_indices() {
        match ch {
            '{' => {
                let sels: Vec<String> = text[start..i]
                    .split(',')
                    .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
                    .filter(|s| !s.is_empty())
                    .collect();
                if !sels.is_empty() {
                    out.push(sels);
                }
                start = i + 1;
            }
            '}' | ';' => start = i + 1,
            _ => {}
        }
    }
    out
}

// spec: site-kit/SPEC.md §check-docs-highlight-coverage — a selector covers a class when it ends in
// `<scope> .<class>`, any ancestor prefix admitted before the scope
fn covers(selector: &str, scope: &str, class: &str) -> bool {
    let target = format!("{} {}", scope, class);
    match selector.strip_suffix(&target) {
        Some("") => true,
        Some(prefix) => prefix.ends_with(' ') || prefix.ends_with('>'),
        None => false,
    }
}

fn inner(_args: &[String]) -> Result<i32, String> {
    let snapshot = walk::knob_scalar("SITE_KIT_HIGHLIGHT_TOKENS")?;
    // spec: site-kit/SPEC.md §check-docs-highlight-coverage — the arming knob: empty asserts nothing and
    // says so, a different sentence from a snapshot found covered
    if snapshot.is_empty() {
        println!(
            "DOCS-HIGHLIGHT-COVERAGE: clean (no snapshot configured; SITE_KIT_HIGHLIGHT_TOKENS is empty, so nothing was asserted)"
        );
        return Ok(0);
    }
    let globs = walk::knob_array("SITE_KIT_HIGHLIGHT_OVERRIDES")?;
    let scope = walk::knob_scalar("SITE_KIT_HIGHLIGHT_SCOPE")?;
    if !Path::new(&snapshot).is_file() {
        return Err(format!("the snapshot {} (SITE_KIT_HIGHLIGHT_TOKENS) does not exist", snapshot));
    }
    let text = std::fs::read(&snapshot)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read the snapshot {}: {}", snapshot, e))?;

    let mut findings: Vec<String> = Vec::new();
    let mut classes: Vec<(usize, String)> = Vec::new();
    for (i, line) in fresh::file_lines(&text).iter().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if t.len() < 2 || !t.starts_with('.') || t.contains(char::is_whitespace) {
            findings.push(format!("{}:{}: '{}' is not one class written as .<class>", snapshot, i + 1, t));
            continue;
        }
        classes.push((i + 1, t.to_string()));
    }
    if classes.is_empty() && findings.is_empty() {
        findings.push(format!(
            "{}: the armed snapshot holds zero classes — a capture that extracted nothing is the extraction's failure, not an empty theme",
            snapshot
        ));
    }

    let mut files: Vec<String> = Vec::new();
    for g in &globs {
        let ls = proc::run(&programs::GIT, &["ls-files", "--", g])?;
        let listing = match ls.stdout() {
            Some(o) => String::from_utf8_lossy(o).into_owned(),
            None => return Err(fresh::fail_closed("git-ls-files", ls.code())),
        };
        let hits: Vec<&str> = listing.lines().filter(|p| !p.is_empty() && Path::new(p).is_file()).collect();
        if hits.is_empty() {
            findings.push(format!(
                "override glob '{}' (SITE_KIT_HIGHLIGHT_OVERRIDES) matches no tracked file, so nothing overrides the theme",
                g
            ));
        }
        for h in hits {
            if !files.iter().any(|f| f == h) {
                files.push(h.to_string());
            }
        }
    }

    let mut all_rules: Vec<Vec<String>> = Vec::new();
    for f in &files {
        let raw = std::fs::read(f)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read the override file {}: {}", f, e))?;
        all_rules.extend(rules(&css_text(&raw)));
    }
    let mut covering = vec![false; all_rules.len()];
    for (n, class) in &classes {
        let mut hit = false;
        for (r, sels) in all_rules.iter().enumerate() {
            if sels.iter().any(|s| covers(s, &scope, class)) {
                covering[r] = true;
                hit = true;
            }
        }
        if !hit {
            findings.push(format!(
                "{}:{}: class '{}' has no '{} {}' override in {}",
                snapshot,
                n,
                class,
                scope,
                class,
                globs.join(" ")
            ));
        }
    }

    if !findings.is_empty() {
        println!("check-docs-highlight-coverage: {} coverage finding(s):", findings.len());
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: add a rule whose selector ends in '{} .<class>' for each uncovered class,", scope);
        println!("        taking its token family's colour; the snapshot is the theme's coloured class");
        println!("        list, refreshed from the site-health monitor's printed list");
        println!("        (site-kit/SPEC.md §check-docs-highlight-coverage).");
        return Ok(1);
    }
    println!(
        "DOCS-HIGHLIGHT-COVERAGE: clean ({} snapshot class(es), {} override file(s), {} covering rule(s); every class overridden under '{}')",
        classes.len(),
        files.len(),
        covering.iter().filter(|c| **c).count(),
        scope
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: site-kit/SPEC.md §check-docs-highlight-coverage — the comma split is what reaches a grouped
    // rule's later members, the ones a first-selector reading misses
    #[test]
    fn a_grouped_rule_covers_every_member_not_only_the_first() {
        let r = rules(".highlight .c,\n  .highlight   .cd { color: #999 }\n.x .highlight .o{color:red}");
        assert_eq!(r.len(), 2);
        assert!(r[0].iter().any(|s| covers(s, ".highlight", ".cd")));
        assert!(r[1].iter().any(|s| covers(s, ".highlight", ".o")));
    }

    // spec: site-kit/SPEC.md §check-docs-highlight-coverage — the class must end the selector, and the
    // scope must start a compound: `.go` is not `.o`, `x.highlight` is not the scope
    #[test]
    fn the_suffix_is_bounded_on_both_sides() {
        assert!(covers(".highlight .o", ".highlight", ".o"));
        assert!(covers(".a .b .highlight .o", ".highlight", ".o"));
        assert!(covers(".a>.highlight .o", ".highlight", ".o"));
        assert!(!covers(".highlight .go", ".highlight", ".o"));
        assert!(!covers("pre.highlight .o", ".highlight", ".o"));
        assert!(!covers(".highlight .o:hover", ".highlight", ".o"));
    }

    // spec: site-kit/SPEC.md §check-docs-highlight-coverage — an HTML file is read for its style bodies,
    // so a brace elsewhere in the page opens no rule; a nested at-rule's inner rule is still a rule
    #[test]
    fn an_html_file_is_read_for_its_style_bodies_only() {
        let page = "<p>{{ x }} .highlight .k {</p><STYLE media=\"all\">@media (x) { .highlight .k { color: red; } }</STYLE>";
        let r = rules(&css_text(page));
        assert_eq!(r, vec![vec!["@media (x)".to_string()], vec![".highlight .k".to_string()]]);
        assert_eq!(rules("/* .highlight .o { */ .a{b:c}"), vec![vec![".a".to_string()]]);
    }
}
