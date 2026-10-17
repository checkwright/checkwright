// spec: docs/generated-projections.md §Generated projections and their freshness gates — the product
// statement: one source, its renderings per site kind, one function the gate calls in process
use crate::fresh;

pub const SOURCE_KNOB: &str = "GATE_LOCAL_PRODUCT_STATEMENT_CONF";
pub const SITES_KNOB: &str = "GATE_LOCAL_PRODUCT_STATEMENT_SURFACES";
const BEGIN: &str = "<!-- product-statement:begin -->";
const END: &str = "<!-- product-statement:end -->";
const KEYS: &[&str] = &["category", "summary", "keywords"];
const KEYWORDS_MAX: usize = 5;
const KEYWORD_LEN_MAX: usize = 20;
const REFUSED: &[char] = &['`', '[', ']', '<', '>', '|'];

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub category: String,
    pub summary: String,
    pub keywords: Vec<String>,
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — crates.io's
// keyword grammar, the narrower of the two registries': an ASCII letter, then letters, digits, `-`
// or `_`, twenty in all at most
fn keyword_ok(w: &str) -> bool {
    w.len() <= KEYWORD_LEN_MAX
        && w.starts_with(|c: char| c.is_ascii_alphabetic())
        && w.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn keywords(path: &str, n: usize, value: &str) -> Result<Vec<String>, String> {
    let words: Vec<String> = value.split_whitespace().map(String::from).collect();
    if words.len() > KEYWORDS_MAX {
        return Err(format!(
            "{}:{}: 'keywords' carries {} words; at most {} are admitted",
            path,
            n,
            words.len(),
            KEYWORDS_MAX
        ));
    }
    if let Some(w) = words.iter().find(|w| !keyword_ok(w)) {
        return Err(format!(
            "{}:{}: keyword '{}' is outside the grammar — an ASCII letter, then up to {} letters, digits, '-' or '_'",
            path,
            n,
            w,
            KEYWORD_LEN_MAX - 1
        ));
    }
    Ok(words)
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
        if key == "keywords" {
            keywords(path, n + 1, value)?;
        } else if let Some(c) = value.chars().find(|c| REFUSED.contains(c) || c.is_control()) {
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
        keywords: get("keywords")?.split_whitespace().map(String::from).collect(),
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

const BLOCK: &str = "block";
const DESCRIPTION: &str = "description";
const KEYWORDS: &str = "keywords";

// spec: docs/generated-projections.md §Generated projections and their freshness gates — a manifest
// site's slots: every one carries the description, a `.json` or `.toml` one the keywords too
fn fields_of(kind: Kind) -> &'static [&'static str] {
    match kind {
        Kind::Block => &[BLOCK],
        Kind::Yaml => &[DESCRIPTION],
        Kind::Json | Kind::Toml => &[DESCRIPTION, KEYWORDS],
    }
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — the keywords
// on one line, an array of the quoted words in source order
pub fn keyword_array(s: &Statement) -> String {
    let words: Vec<String> = s.keywords.iter().map(|w| quoted(w)).collect();
    format!("[{}]", words.join(", "))
}

fn rendering(field: &str, s: &Statement) -> String {
    match field {
        BLOCK => markdown(s),
        KEYWORDS => keyword_array(s),
        _ => quoted(&plain(s)),
    }
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — the text a
// site carries per slot: the markdown rendering in a block, the plain rendering as a quoted scalar and
// the keywords as an array elsewhere
pub fn expected(site: &str, s: &Statement) -> Result<Vec<String>, String> {
    Ok(fields_of(kind_of(site)?).iter().map(|f| rendering(f, s)).collect())
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — a site's slot:
// the line span `--write` replaces and the text the gate compares; an absent or repeated slot is no
// place to write, so it refuses
pub struct Slot {
    pub field: &'static str,
    pub line: usize,
    start: usize,
    end: usize,
    prefix: String,
    suffix: String,
    pub actual: String,
}

fn scalar_split<'a>(line: &'a str, kind: Kind, field: &str) -> Option<(&'a str, &'a str)> {
    let json_key = format!("\"{}\"", field);
    let (rest, sep) = match kind {
        Kind::Yaml => (line.strip_prefix(field)?, ':'),
        Kind::Toml => (line.strip_prefix(field)?, '='),
        Kind::Json => (line.trim_start().strip_prefix(json_key.as_str())?, ':'),
        Kind::Block => return None,
    };
    let after = rest.trim_start_matches([' ', '\t']).strip_prefix(sep)?;
    let value = after.trim_start_matches([' ', '\t']);
    Some((&line[..line.len() - value.len()], value))
}

fn slot(site: &str, text: &str, field: &'static str) -> Result<Slot, String> {
    let lines = fresh::file_lines(text);
    let kind = kind_of(site)?;
    if let Kind::Block = kind {
        let at = |m: &str| -> Vec<usize> { lines.iter().enumerate().filter(|(_, l)| **l == m).map(|(i, _)| i).collect() };
        let (b, e) = (at(BEGIN), at(END));
        return match (b.as_slice(), e.as_slice()) {
            ([b], [e]) if b < e => Ok(Slot {
                field,
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
        .filter_map(|(i, l)| scalar_split(l, kind, field).map(|(p, v)| (i, p, v)))
        .collect();
    match hits.as_slice() {
        [(i, prefix, value)] => {
            let v = value.trim_end();
            let (literal, suffix) = match (kind, v.strip_suffix(',')) {
                (Kind::Json, Some(lit)) => (lit.trim_end(), &value[lit.len()..]),
                _ => (v, &value[v.len()..]),
            };
            Ok(Slot {
                field,
                line: i + 1,
                start: *i,
                end: i + 1,
                prefix: prefix.to_string(),
                suffix: suffix.to_string(),
                actual: literal.to_string(),
            })
        }
        [] => Err(format!("{}: no {} field", site, field)),
        many => Err(format!("{}: {} {} fields; exactly one is admissible", site, many.len(), field)),
    }
}

// spec: docs/generated-projections.md §Generated projections and their freshness gates — a block holds
// the rendering between two blank lines, so kramdown reads it as its own paragraph
pub fn check(site: &str, text: &str, s: &Statement) -> Result<Vec<(Slot, String)>, String> {
    let kind = kind_of(site)?;
    let mut out = Vec::new();
    for field in fields_of(kind) {
        let slot = slot(site, text, field)?;
        let want = match kind {
            Kind::Block => format!("\n{}\n", markdown(s)),
            _ => rendering(field, s),
        };
        out.push((slot, want));
    }
    Ok(out)
}

fn rewrite(site: &str, text: &str, s: &Statement) -> Result<String, String> {
    let mut slots = check(site, text, s)?;
    slots.sort_by_key(|(slot, _)| std::cmp::Reverse(slot.start));
    let mut out: Vec<String> = fresh::file_lines(text).iter().map(|l| l.to_string()).collect();
    for (slot, want) in slots {
        out.splice(slot.start..slot.end, [format!("{}{}{}", slot.prefix, want, slot.suffix)]);
    }
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
        for text in expected(site, &st)? {
            out.push_str(&format!("{}: {}\n", site, text));
        }
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
            keywords: vec!["a-b".into(), "c_d".into()],
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
            assert_eq!(expected(site, &st()).unwrap()[0], want);
        }
        assert_eq!(quoted(&p), want);
    }

    #[test]
    fn a_missing_repeated_unknown_or_refused_source_line_fails_closed() {
        let k = "keywords x\n";
        assert!(parse("s", "category A.\nsummary B.\n").is_err());
        assert!(parse("s", &format!("category A.\nsummary B.\nsummary C.\n{}", k)).is_err());
        assert!(parse("s", &format!("category A.\nsummary B.\ntagline C.\n{}", k)).is_err());
        assert!(parse("s", &format!("category A.\nsummary B `c`.\n{}", k)).is_err());
        assert!(parse("s", &format!("category A.\nsummary B\tc.\n{}", k)).is_err());
        assert_eq!(
            parse("s", "# c\n\ncategory A.\nsummary B *c*.\nkeywords x y-z\n"),
            Ok(Statement {
                category: "A.".into(),
                summary: "B *c*.".into(),
                keywords: vec!["x".into(), "y-z".into()],
            })
        );
    }

    // spec: docs/generated-projections.md §Generated projections and their freshness gates — one to
    // five words, each in crates.io's grammar
    #[test]
    fn a_keyword_outside_the_grammar_or_a_sixth_word_fails_closed() {
        let src = |k: &str| format!("category A.\nsummary B.\nkeywords {}\n", k);
        assert!(parse("s", &src("a b c d e")).is_ok());
        assert!(parse("s", &src("a b c d e f")).is_err());
        assert!(parse("s", &src("1a")).is_err());
        assert!(parse("s", &src("a.b")).is_err());
        assert!(parse("s", &src("abcdefghijklmnopqrst")).is_ok());
        assert!(parse("s", &src("abcdefghijklmnopqrstu")).is_err());
        assert_eq!(keyword_array(&st()), "[\"a-b\", \"c_d\"]");
    }

    #[test]
    fn a_write_touches_the_slot_alone_and_keeps_a_json_comma() {
        let s = st();
        let json = "{\n  \"name\": \"x\",\n  \"description\": \"old\",\n  \"keywords\": [\"z\"],\n  \"v\": 1\n}\n";
        let got = rewrite("p.json", json, &s).unwrap();
        assert_eq!(
            got,
            "{\n  \"name\": \"x\",\n  \"description\": \"Cat. A \\\"quoted\\\" done back\\\\slash.\",\n  \"keywords\": [\"a-b\", \"c_d\"],\n  \"v\": 1\n}\n"
        );
        let toml = "[package]\ndescription = \"old\"\nkeywords = []\n";
        assert_eq!(
            rewrite("c.toml", toml, &s).unwrap(),
            "[package]\ndescription = \"Cat. A \\\"quoted\\\" done back\\\\slash.\"\nkeywords = [\"a-b\", \"c_d\"]\n"
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
        assert!(slot("r.md", "no block\n", BLOCK).is_err());
        assert!(slot("r.md", &format!("{0}\n{1}\n{0}\n{1}\n", BEGIN, END), BLOCK).is_err());
        assert!(slot("c.toml", "name = \"x\"\n", DESCRIPTION).is_err());
        assert!(slot("c.toml", "description = \"a\"\ndescription = \"b\"\n", DESCRIPTION).is_err());
        assert!(slot("c.toml", "descriptions = \"a\"\n", DESCRIPTION).is_err());
        assert!(check("c.toml", "description = \"a\"\n", &st()).is_err());
        assert!(check("c.yml", "description: a\n", &st()).is_ok());
    }
}
