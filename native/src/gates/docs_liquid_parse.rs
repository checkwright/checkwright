// spec: site-kit/SPEC.md §check-docs-liquid-parse — every tracked docs file the Pages build runs Liquid over parses under the parser SITE_KIT_LIQUID_PARSER names
use super::docs_render_fidelity::{jekyll_internal, nul_records, spawn_filter};
use crate::fresh;
use crate::walk;
use crate::{proc, programs};
use std::io::Read;
use std::path::Path;

const NAME: &str = "check-docs-liquid-parse";

// spec: site-kit/SPEC.md §check-docs-liquid-parse — the three Jekyll directories whose files are Liquid templates or pages
const TEMPLATE_DIRS: &[&str] = &["_layouts", "_includes", "_posts"];

const CLIP: usize = 200;

const PROBE: &[u8] = b"{{ probe }}\0{{ probe\0";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Class {
    Page,
    FrontMatter,
    Template,
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

fn inner(_args: &[String]) -> Result<i32, String> {
    let docs_knob = walk::knob_scalar("SITE_KIT_DOCS_DIR").map_err(|e| format!("{}: {}", NAME, e))?;
    let docs = fresh::strip_trailing_slash(&docs_knob).to_string();

    let probe = proc::run(&programs::GIT, &["rev-parse", "--git-dir"]).map_err(|e| format!("{}: {}", NAME, e))?;
    if probe.stdout().is_none() {
        return Err(format!("{}: not a git repository — cannot enumerate tracked docs files", NAME));
    }
    if !fresh::is_dir(&docs) {
        return Err(format!("{}: docs dir not found: {}", NAME, docs));
    }

    let parser = walk::knob_array("SITE_KIT_LIQUID_PARSER").map_err(|e| format!("{}: {}", NAME, e))?;
    let probed = spawn_filter("SITE_KIT_LIQUID_PARSER", &parser, PROBE, proc::Stderr::Discard)
        .map(|(_, out)| nul_records(&out));
    if let Some(why) = probe_refusal(&probed) {
        return Err(format!(
            "{}: Liquid parser '{}' failed its probe ({})\n  help: SITE_KIT_LIQUID_PARSER must read NUL-terminated documents from stdin and write one\n        NUL-terminated verdict per input, in order: empty when the document parses, else\n        the parse error — the default needs ruby with the liquid gem",
            NAME,
            parser.join(" "),
            why
        ));
    }

    let ls = proc::run(&programs::GIT, &["ls-files", "--", &docs]).map_err(|e| format!("{}: {}", NAME, e))?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            return Err(format!(
                "DOCS-LIQUID-PARSE: {}",
                fresh::fail_closed("git-ls-files", ls.code())
            ))
        }
    };

    let prune = walk::prune_dirs().map_err(|e| format!("{}: {}", NAME, e))?;
    let mut corpus: Vec<(String, Class)> = Vec::new();
    for p in listing.lines() {
        if p.is_empty() || walk::path_pruned(p, &prune) || !Path::new(p).is_file() {
            continue;
        }
        let rel = walk::rel_under(&docs, p).unwrap_or(p);
        let fenced = || first_line_is_fence(p);
        if let Some(class) = classify(rel, fenced) {
            corpus.push((p.to_string(), class));
        }
    }

    let mut stream: Vec<u8> = Vec::new();
    for (p, _) in &corpus {
        let raw = std::fs::read(p)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("{}: cannot read {}: {}", NAME, p, e))?;
        stream.extend_from_slice(frame(&raw).as_bytes());
        stream.push(0);
    }

    let verdicts = if corpus.is_empty() {
        Vec::new()
    } else {
        let (_, out) = spawn_filter("SITE_KIT_LIQUID_PARSER", &parser, &stream, proc::Stderr::Inherit)?;
        nul_records(&out)
    };
    if verdicts.len() != corpus.len() {
        return Err(format!(
            "{}: Liquid parser returned {} verdict(s) for {} document(s)\n  help: SITE_KIT_LIQUID_PARSER must write exactly one NUL-terminated verdict per NUL-terminated\n        input document, in order; a short count is a parser that died mid-stream, truncated\n        its output, or framed it wrongly",
            NAME,
            verdicts.len(),
            corpus.len()
        ));
    }

    let findings: Vec<String> = corpus
        .iter()
        .zip(&verdicts)
        .filter(|(_, v)| !v.is_empty())
        .map(|((p, _), v)| format!("{}: {}", p, clip(&String::from_utf8_lossy(v))))
        .collect();
    if !findings.is_empty() {
        println!("{}: docs file(s) fail to parse under the Liquid parser the Pages build runs:", NAME);
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: wrap Liquid-significant text a page means literally in a {{% raw %}} … {{% endraw %}} block");
        println!("        (for a generated page, in its emitter), or remove the token");
        return Ok(1);
    }

    let count = |c: Class| corpus.iter().filter(|(_, k)| *k == c).count();
    println!(
        "DOCS-LIQUID-PARSE: clean ({} file(s) under {} parse under the Liquid parser: {} markdown page(s), {} front-matter file(s), {} template(s))",
        corpus.len(),
        docs,
        count(Class::Page),
        count(Class::FrontMatter),
        count(Class::Template)
    );
    Ok(0)
}

// spec: site-kit/SPEC.md §check-docs-liquid-parse — two verdicts back, the first empty and the second not; anything else, an unrunnable command included, is a refusal
fn probe_refusal(probed: &Result<Vec<Vec<u8>>, String>) -> Option<String> {
    match probed {
        Err(e) => Some(e.clone()),
        Ok(v) if v.len() != 2 => Some(format!("2 documents in, {} verdict(s) back", v.len())),
        Ok(v) if !v[0].is_empty() => Some("a parsing document drew a non-empty verdict".to_string()),
        Ok(v) if v[1].is_empty() => Some("an unterminated '{{' drew an empty verdict".to_string()),
        Ok(_) => None,
    }
}

// spec: site-kit/SPEC.md §check-docs-liquid-parse — the corpus's three classes, read on the path relative to the docs dir
fn classify(rel: &str, fenced: impl FnOnce() -> bool) -> Option<Class> {
    let mut segs = rel.split('/');
    let top = segs.next().unwrap_or("");
    if segs.next().is_some() && TEMPLATE_DIRS.contains(&top) {
        return Some(Class::Template);
    }
    if jekyll_internal(rel) {
        return None;
    }
    if rel.ends_with(".md") {
        return Some(Class::Page);
    }
    if fenced() {
        return Some(Class::FrontMatter);
    }
    None
}

fn is_fence(line: &str) -> bool {
    line.trim_end_matches([' ', '\t', '\r']) == "---"
}

fn first_line_is_fence(p: &str) -> bool {
    let mut head = Vec::new();
    let read = std::fs::File::open(p).and_then(|f| f.take(64).read_to_end(&mut head));
    if read.is_err() {
        return false;
    }
    let text = String::from_utf8_lossy(&head);
    text.contains('\n') && is_fence(text.split('\n').next().unwrap_or(""))
}

// spec: site-kit/SPEC.md §check-docs-liquid-parse — NULs dropped before framing, and each front-matter line blanked so a reported line number is the file's own
fn frame(raw: &str) -> String {
    let s: String = raw.chars().filter(|c| *c != '\0').collect();
    let lines: Vec<&str> = s.split('\n').collect();
    if lines.first().is_some_and(|l| is_fence(l)) {
        if let Some(close) = lines.iter().skip(1).position(|l| is_fence(l)) {
            let close = close + 1;
            return format!("{}{}", "\n".repeat(close + 1), lines[close + 1..].join("\n"));
        }
    }
    s
}

// spec: site-kit/SPEC.md §check-docs-liquid-parse — the verdict's first line, cut to 200 characters with a trailing ellipsis
fn clip(verdict: &str) -> String {
    let first = verdict.split('\n').next().unwrap_or("").trim_end_matches('\r');
    if first.chars().count() > CLIP {
        format!("{}…", first.chars().take(CLIP).collect::<String>())
    } else {
        first.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_corpus_holds_three_classes_and_nothing_else() {
        assert_eq!(classify("install.md", || false), Some(Class::Page));
        assert_eq!(classify("kit/SPEC.md", || false), Some(Class::Page));
        assert_eq!(classify("search.json", || true), Some(Class::FrontMatter));
        assert_eq!(classify("raw.html", || false), None);
        assert_eq!(classify("_layouts/default.html", || false), Some(Class::Template));
        assert_eq!(classify("_includes/nav.html", || false), Some(Class::Template));
        assert_eq!(classify("_posts/2026-01-01-a.md", || false), Some(Class::Template));
        assert_eq!(classify("_data/x.md", || true), None);
        assert_eq!(classify("_draft.md", || true), None);
        assert_eq!(classify("a/_layouts/x.html", || true), None);
        assert_eq!(classify("_layouts", || true), None);
    }

    #[test]
    fn front_matter_is_blanked_line_for_line() {
        assert_eq!(frame("---\ntitle: x\n---\n{{ a\n"), "\n\n\n{{ a\n");
        assert_eq!(frame("--- \r\nt: {{\r\n---\r\nbody"), "\n\n\nbody");
        assert_eq!(frame("body\n---\n{{ a\n---\n"), "body\n---\n{{ a\n---\n");
        assert_eq!(frame("---\nnever closed {{\n"), "---\nnever closed {{\n");
        assert_eq!(frame("a\0b"), "ab");
    }

    #[test]
    fn a_verdict_is_clipped_to_its_first_line_and_two_hundred_characters() {
        assert_eq!(clip("Liquid syntax error (line 3): x\nmore"), "Liquid syntax error (line 3): x");
        let long = "é".repeat(250);
        let cut = clip(&long);
        assert_eq!(cut.chars().count(), CLIP + 1);
        assert!(cut.ends_with('…'));
        assert_eq!(clip(&"a".repeat(CLIP)), "a".repeat(CLIP));
    }

    #[test]
    fn the_probe_admits_only_a_detecting_parser() {
        let ok = |v: Vec<&[u8]>| Ok(v.into_iter().map(<[u8]>::to_vec).collect::<Vec<_>>());
        assert!(probe_refusal(&ok(vec![b"", b"Liquid syntax error"])).is_none());
        assert!(probe_refusal(&ok(vec![b"", b""])).is_some());
        assert!(probe_refusal(&ok(vec![b"x", b"y"])).is_some());
        assert!(probe_refusal(&ok(vec![b""])).is_some());
        assert!(probe_refusal(&Err("cannot run".to_string())).is_some());
    }
}
