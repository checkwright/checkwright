// spec: gate-sdk/SPEC.md §The deny-term list — the one reader of a literal-term list and of the
// `# rule:` ids an ERE pattern file shares with it; each reading gate owns its own matcher
use crate::exit::Fault;
use crate::walk;

pub const KNOB: &str = "GATE_SDK_DENY_TERMS";

const RULE_LEAD: &str = "# rule:";

// spec: gate-sdk/SPEC.md §The deny-term list — a live entry, the `# rule:` id in force above it
// and its 1-based position among the live entries of the lists in the order they were read
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub text: String,
    pub id: Option<String>,
    pub position: usize,
}

impl Entry {
    pub fn id_or<'a>(&'a self, default: &'a str) -> &'a str {
        self.id.as_deref().unwrap_or(default)
    }
}

fn blank(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r'
}

// spec: gate-sdk/SPEC.md §The deny-term list — one list's live entries, trimmed, each with the id
// in force above it; `Err` is the 1-based line of a `# rule:` line whose id fails the class grammar
pub fn parse(text: &str) -> Result<Vec<(String, Option<String>)>, usize> {
    let mut id: Option<String> = None;
    let mut out = Vec::new();
    for (i, line) in crate::fresh::file_lines(text).into_iter().enumerate() {
        let body = line.trim_matches(blank);
        if let Some(rest) = body.strip_prefix(RULE_LEAD) {
            let token = rest.trim_matches(blank);
            if !crate::sarif::is_class(token) {
                return Err(i + 1);
            }
            id = Some(token.to_string());
            continue;
        }
        if body.is_empty() || body.starts_with('#') {
            continue;
        }
        out.push((body.to_string(), id.clone()));
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §The deny-term list — a malformed `# rule:` line names its file and line,
// never the id or its neighbours
pub fn malformed(file: &str, line: usize) -> Fault {
    Fault::refused(format!(
        "{}:{}: a '# rule:' line whose id is not [A-Z][A-Z0-9_-]* (gate-sdk/SPEC.md §The deny-term list)",
        file, line
    ))
}

// spec: gate-sdk/SPEC.md §The deny-term list — the files the knob names, each optional
pub fn files() -> Result<Vec<String>, Fault> {
    walk::knob_words(KNOB).map_err(Fault::refused)
}

// spec: gate-sdk/SPEC.md §The deny-term list — every live term of the lists, in knob order
pub fn read(files: &[String]) -> Result<Vec<Entry>, Fault> {
    read_lists(files, "deny-term list")
}

// spec: gate-sdk/SPEC.md §check-binary-strings — a shape list shares the deny-term list's
// grammar, positions and optional-file split, its entries being EREs
pub fn read_lists(files: &[String], noun: &str) -> Result<Vec<Entry>, Fault> {
    let mut out: Vec<Entry> = Vec::new();
    for f in files {
        let Some(text) = crate::exit::read_optional(f, noun)? else {
            continue;
        };
        for (text, id) in parse(&text).map_err(|line| malformed(f, line))? {
            out.push(Entry { text, id, position: out.len() + 1 });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rule_line_names_every_term_after_it_and_a_term_before_the_first_takes_none() {
        let got = parse("first\n\n# a comment\n# rule: CODE-1\n  second \t\n# rule: OTHER_ID\nthird\r\n").unwrap();
        assert_eq!(
            got,
            vec![
                ("first".to_string(), None),
                ("second".to_string(), Some("CODE-1".to_string())),
                ("third".to_string(), Some("OTHER_ID".to_string())),
            ]
        );
    }

    #[test]
    fn a_rule_line_whose_id_fails_the_class_grammar_is_named_by_its_line() {
        assert_eq!(parse("a\n# rule: lower\nb\n"), Err(2));
        assert_eq!(parse("# rule:\n"), Err(1));
        assert_eq!(parse("# rule: TWO WORDS\n"), Err(1));
    }

    #[test]
    fn positions_run_across_the_lists_in_order_and_an_absent_list_contributes_nothing() {
        let dir = std::env::temp_dir().join(format!("checkwright-deny-terms.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        let a = dir.join("a.list");
        let b = dir.join("b.list");
        std::fs::write(&a, "one\n# rule: X\ntwo\n").expect("write");
        std::fs::write(&b, "three\n").expect("write");
        let names: Vec<String> = [a, dir.join("absent.list"), b].iter().map(|p| p.display().to_string()).collect();
        let got = read(&names).expect("read");
        let pos: Vec<(usize, &str, &str)> = got.iter().map(|e| (e.position, e.text.as_str(), e.id_or("D"))).collect();
        assert_eq!(pos, vec![(1, "one", "D"), (2, "two", "X"), (3, "three", "D")]);
        let fault = read(&[dir.display().to_string()]).expect_err("a directory at the path is input");
        assert_eq!(fault.status(), crate::exit::INPUT);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
