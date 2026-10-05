// spec: docs/generated-projections.md §Generated projections and their freshness gates — the product
// statement: one source, two renderings, five sites, one function the gate calls in process
use crate::fresh;

pub const SOURCE_KNOB: &str = "GATE_LOCAL_PRODUCT_STATEMENT_CONF";
pub const SITES_KNOB: &str = "GATE_LOCAL_PRODUCT_STATEMENT_SURFACES";
const BEGIN: &str = "<!-- product-statement:begin -->";
const END: &str = "<!-- product-statement:end -->";
const KEYS: &[&str] = &["category", "summary"];
const REFUSED: &[char] = &['`', '[', ']', '<', '>', '|'];

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub category: String,
    pub summary: String,
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — each key once,
// no other key, and no character a rendering would lose
pub fn parse(path: &str, text: &str) -> Result<Statement, String> {
    let mut found: Vec<(String, String)> = Vec::new();
    for (n, line) in fresh::file_lines(text).into_iter().enumerate() {
        if !fresh::live_line(line) {
            continue;
        }
        let line = line.trim();
        let (key, value) = match line.split_once(char::is_whitespace) {
            Some((k, v)) => (k, v.trim()),
            None => (line, ""),
        };
        if !KEYS.contains(&key) {
            return Err(format!("{}:{}: unknown key '{}' (admitted: {})", path, n + 1, key, KEYS.join(", ")));
        }
        if found.iter().any(|(k, _)| k == key) {
            return Err(format!("{}:{}: key '{}' repeated", path, n + 1, key));
        }
        if value.is_empty() {
            return Err(format!("{}:{}: key '{}' carries no value", path, n + 1, key));
        }
        if let Some(c) = value.chars().find(|c| REFUSED.contains(c) || c.is_control()) {
            return Err(format!(
                "{}:{}: '{}' carries the refused character {:?} — a value takes *…* emphasis and no other markdown",
                path,
                n + 1,
                key,
                c
            ));
        }
        found.push((key.to_string(), value.to_string()));
    }
    let get = |k: &str| -> Result<String, String> {
        found
            .iter()
            .find(|(key, _)| key == k)
            .map(|(_, v)| v.clone())
            .ok_or_else(|| format!("{}: key '{}' missing", path, k))
    };
    Ok(Statement {
        category: get("category")?,
        summary: get("summary")?,
    })
}

pub fn markdown(s: &Statement) -> String {
    format!("**{}** {}", s.category, s.summary)
}

pub fn plain(s: &Statement) -> String {
    format!("{} {}", s.category, s.summary).replace('*', "")
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — YAML's
// double-quoted scalar, JSON's string and TOML's basic string share the one escape the source admits
pub fn quoted(v: &str) -> String {
    format!("\"{}\"", v.replace('\\', "\\\\").replace('"', "\\\""))
}

#[derive(Clone, Copy)]
enum Kind {
    Block,
    Yaml,
    Json,
    Toml,
}

fn kind_of(site: &str) -> Result<Kind, String> {
    let ext = site.rsplit('.').next().unwrap_or("");
    match ext {
        "md" => Ok(Kind::Block),
        "yml" | "yaml" => Ok(Kind::Yaml),
        "json" => Ok(Kind::Json),
        "toml" => Ok(Kind::Toml),
        _ => Err(format!("{}: no rendering for a .{} site", site, ext)),
    }
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — the text a
// site carries: the markdown rendering in a block, the plain rendering as a quoted scalar elsewhere
pub fn expected(site: &str, s: &Statement) -> Result<String, String> {
    Ok(match kind_of(site)? {
        Kind::Block => markdown(s),
        _ => quoted(&plain(s)),
    })
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — a site's slot:
// the line span `--write` replaces and the text the gate compares; an absent or repeated slot is no
// place to write, so it refuses
pub struct Slot {
    pub line: usize,
    start: usize,
    end: usize,
    prefix: String,
    suffix: String,
    pub actual: String,
}

fn scalar_split(line: &str, kind: Kind) -> Option<(&str, &str)> {
    let (rest, sep) = match kind {
        Kind::Yaml => (line.strip_prefix("description")?, ':'),
        Kind::Toml => (line.strip_prefix("description")?, '='),
        Kind::Json => (line.trim_start().strip_prefix("\"description\"")?, ':'),
        Kind::Block => return None,
    };
    let after = rest.trim_start_matches([' ', '\t']).strip_prefix(sep)?;
    let value = after.trim_start_matches([' ', '\t']);
    Some((&line[..line.len() - value.len()], value))
}

fn slot(site: &str, text: &str) -> Result<Slot, String> {
    let lines = fresh::file_lines(text);
    let kind = kind_of(site)?;
    if let Kind::Block = kind {
        let at = |m: &str| -> Vec<usize> { lines.iter().enumerate().filter(|(_, l)| **l == m).map(|(i, _)| i).collect() };
        let (b, e) = (at(BEGIN), at(END));
        return match (b.as_slice(), e.as_slice()) {
            ([b], [e]) if b < e => Ok(Slot {
                line: b + 1,
                start: b + 1,
                end: *e,
                prefix: String::new(),
                suffix: String::new(),
                actual: lines[b + 1..*e].join("\n"),
            }),
            _ => Err(format!(
                "{}: needs exactly one {} … {} pair, in order ({} begin, {} end)",
                site,
                BEGIN,
                END,
                b.len(),
                e.len()
            )),
        };
    }
    let hits: Vec<(usize, &str, &str)> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, l)| scalar_split(l, kind).map(|(p, v)| (i, p, v)))
        .collect();
    match hits.as_slice() {
        [(i, prefix, value)] => {
            let v = value.trim_end();
            let (literal, suffix) = match (kind, v.strip_suffix(',')) {
                (Kind::Json, Some(lit)) => (lit.trim_end(), &value[lit.len()..]),
                _ => (v, &value[v.len()..]),
            };
            Ok(Slot {
                line: i + 1,
                start: *i,
                end: i + 1,
                prefix: prefix.to_string(),
                suffix: suffix.to_string(),
                actual: literal.to_string(),
            })
        }
        [] => Err(format!("{}: no description field", site)),
        many => Err(format!("{}: {} description fields; exactly one is admissible", site, many.len())),
    }
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — a block holds
// the rendering between two blank lines, so kramdown reads it as its own paragraph
pub fn check(site: &str, text: &str, s: &Statement) -> Result<(Slot, String), String> {
    let slot = slot(site, text)?;
    let want = match kind_of(site)? {
        Kind::Block => format!("\n{}\n", markdown(s)),
        _ => expected(site, s)?,
    };
    Ok((slot, want))
}

fn rewrite(site: &str, text: &str, s: &Statement) -> Result<String, String> {
    let (slot, want) = check(site, text, s)?;
    let lines = fresh::file_lines(text);
    let mut out: Vec<String> = lines[..slot.start].iter().map(|l| l.to_string()).collect();
    out.push(format!("{}{}{}", slot.prefix, want, slot.suffix));
    out.extend(lines[slot.end..].iter().map(|l| l.to_string()));
    let mut rendered = out.join("\n");
    if text.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}

// spec: gate-sdk/SPEC.md §The non-gate arm — the bare arm prints each site's expected text; `--write`
// rewrites every site's slot in place, and only after every site has placed its slot
pub fn emit(args: &[String]) -> Result<String, String> {
    let source = crate::walk::knob_scalar(SOURCE_KNOB)?;
    let st = parse(&source, &crate::emit::read_text(&source)?)?;
    let sites = crate::walk::knob_array(SITES_KNOB)?;
    if args.iter().any(|a| a == "--write") {
        let mut staged: Vec<(&str, String, bool)> = Vec::new();
        for site in &sites {
            let text = crate::emit::read_text(site)?;
            let new = rewrite(site, &text, &st)?;
            let changed = new != text;
            staged.push((site, new, changed));
        }
        let mut out = String::new();
        for (site, new, changed) in staged {
            if changed {
                std::fs::write(site, new).map_err(|e| format!("cannot write {}: {}", site, e))?;
                out.push_str(&format!("product-statement: rewrote {}\n", site));
            }
        }
        if out.is_empty() {
            out.push_str("product-statement: every site already fresh\n");
        }
        return Ok(out);
    }
    let mut out = String::new();
    for site in &sites {
        out.push_str(&format!("{}: {}\n", site, expected(site, &st)?));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn st() -> Statement {
        Statement {
            category: "Cat.".into(),
            summary: "A \"quoted\" *done* back\\slash.".into(),
        }
    }

    #[test]
    fn the_markdown_rendering_bolds_the_category_and_keeps_the_emphasis() {
        assert_eq!(markdown(&st()), "**Cat.** A \"quoted\" *done* back\\slash.");
    }

    #[test]
    fn the_plain_rendering_drops_every_asterisk() {
        assert_eq!(plain(&st()), "Cat. A \"quoted\" done back\\slash.");
    }

    // spec: docs/generated-projections.md §Generated projections and their freshness gates — the three
    // escapers are one: backslash first, then the quote
    #[test]
    fn each_scalar_escapes_the_quote_and_the_backslash() {
        let p = plain(&st());
        let want = "\"Cat. A \\\"quoted\\\" done back\\\\slash.\"";
        for site in ["x.yml", "x.json", "x.toml"] {
            assert_eq!(expected(site, &st()).unwrap(), want);
        }
        assert_eq!(quoted(&p), want);
    }

    #[test]
    fn a_missing_repeated_unknown_or_refused_source_line_fails_closed() {
        assert!(parse("s", "category A.\n").is_err());
        assert!(parse("s", "category A.\nsummary B.\nsummary C.\n").is_err());
        assert!(parse("s", "category A.\nsummary B.\ntagline C.\n").is_err());
        assert!(parse("s", "category A.\nsummary B `c`.\n").is_err());
        assert!(parse("s", "category A.\nsummary B\tc.\n").is_err());
        assert_eq!(
            parse("s", "# c\n\ncategory A.\nsummary B *c*.\n"),
            Ok(Statement {
                category: "A.".into(),
                summary: "B *c*.".into()
            })
        );
    }

    #[test]
    fn a_write_touches_the_slot_alone_and_keeps_a_json_comma() {
        let s = st();
        let json = "{\n  \"name\": \"x\",\n  \"description\": \"old\",\n  \"v\": 1\n}\n";
        let got = rewrite("p.json", json, &s).unwrap();
        assert_eq!(
            got,
            "{\n  \"name\": \"x\",\n  \"description\": \"Cat. A \\\"quoted\\\" done back\\\\slash.\",\n  \"v\": 1\n}\n"
        );
        let yaml = "title: X\ndescription: old words\ntheme: t\n";
        assert_eq!(
            rewrite("c.yml", yaml, &s).unwrap(),
            "title: X\ndescription: \"Cat. A \\\"quoted\\\" done back\\\\slash.\"\ntheme: t\n"
        );
        let md = format!("# T\n\n{}\nold\n{}\n\nrest\n", BEGIN, END);
        assert_eq!(
            rewrite("r.md", &md, &s).unwrap(),
            format!("# T\n\n{}\n\n{}\n\n{}\n\nrest\n", BEGIN, markdown(&s), END)
        );
        let fresh_md = rewrite("r.md", &md, &s).unwrap();
        assert_eq!(rewrite("r.md", &fresh_md, &s).unwrap(), fresh_md);
    }

    #[test]
    fn an_absent_or_repeated_slot_refuses() {
        assert!(slot("r.md", "no block\n").is_err());
        assert!(slot("r.md", &format!("{0}\n{1}\n{0}\n{1}\n", BEGIN, END)).is_err());
        assert!(slot("c.toml", "name = \"x\"\n").is_err());
        assert!(slot("c.toml", "description = \"a\"\ndescription = \"b\"\n").is_err());
        assert!(slot("c.toml", "descriptions = \"a\"\n").is_err());
    }
}
