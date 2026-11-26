// spec: gate-sdk/SPEC.md §check-script-preflight — the rules-file block grammar, shared by every
// gate reading a consumer rules file of `rule <ID> <kind>` blocks
use crate::ere::Ere;
use crate::exit::Fault;

// spec: gate-sdk/SPEC.md §check-script-preflight — a kind's name, its required keys, then its
// optional ones
pub type Kind = (&'static str, &'static [&'static str], &'static [&'static str]);

pub struct Grammar {
    pub kinds: &'static [Kind],
    pub once: &'static [&'static str],
    pub tops: &'static [&'static str],
}

pub struct Block {
    pub id: String,
    pub kind: &'static str,
    pub line: usize,
    keys: Vec<(&'static str, String, usize)>,
}

pub struct Top {
    pub word: &'static str,
    pub values: Vec<String>,
    pub line: usize,
}

pub fn refusal(file: &str, line: usize, what: impl std::fmt::Display) -> Fault {
    Fault::refused(format!("{}:{}: {}", file, line, what))
}

impl Block {
    fn values<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a (&'static str, String, usize)> + 'a {
        self.keys.iter().filter(move |k| k.0 == key)
    }

    pub fn one(&self, key: &str) -> String {
        self.values(key).next().map(|k| k.1.clone()).unwrap_or_default()
    }

    pub fn all(&self, key: &str) -> Vec<String> {
        self.values(key).map(|k| k.1.clone()).collect()
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — a value split on blanks, for a key whose value is a list
    pub fn words(&self, key: &str) -> Vec<String> {
        self.values(key).flat_map(|k| k.1.split_whitespace().map(str::to_string)).collect()
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — an ERE the engine refuses names its key's line
    pub fn eres(&self, file: &str, key: &str) -> Result<Vec<Ere>, Fault> {
        self.values(key)
            .map(|(_, v, n)| Ere::compile(v).map_err(|e| refusal(file, *n, format!("{}: {}", key, e))))
            .collect()
    }
}

pub fn parse(file: &str, text: &str, g: &Grammar) -> Result<(Vec<Top>, Vec<Block>), Fault> {
    let (mut tops, mut blocks) = (Vec::<Top>::new(), Vec::<Block>::new());
    let kind_of = |name: &str| g.kinds.iter().find(|k| k.0 == name);
    for (i, raw) in text.lines().enumerate() {
        let n = i + 1;
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if line.starts_with([' ', '\t']) {
            let Some(b) = blocks.last_mut() else {
                return Err(refusal(file, n, "a key line before any 'rule' line"));
            };
            let (key, value) = trimmed.split_once([' ', '\t']).map_or((trimmed, ""), |(k, v)| (k, v.trim()));
            let (_, req, opt) = kind_of(b.kind).expect("a block's kind is known");
            let Some(key) = req.iter().chain(opt.iter()).find(|k| **k == key).copied() else {
                return Err(refusal(file, n, format!("unknown key '{}' for a {} rule", key, b.kind)));
            };
            if value.is_empty() {
                return Err(refusal(file, n, format!("key '{}' takes a value", key)));
            }
            if g.once.contains(&key) && b.keys.iter().any(|k| k.0 == key) {
                return Err(refusal(file, n, format!("key '{}' given twice in one rule", key)));
            }
            b.keys.push((key, value.to_string(), n));
            continue;
        }
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words[0] == "rule" {
            if words.len() != 3 {
                return Err(refusal(file, n, "a rule line is 'rule <ID> <kind>'"));
            }
            if !crate::sarif::is_class(words[1]) {
                return Err(refusal(file, n, format!("rule id '{}' is not [A-Z][A-Z0-9_-]*", words[1])));
            }
            let Some(kind) = kind_of(words[2]).map(|k| k.0) else {
                return Err(refusal(file, n, format!("unknown kind '{}'", words[2])));
            };
            blocks.push(Block { id: words[1].to_string(), kind, line: n, keys: Vec::new() });
            continue;
        }
        match g.tops.iter().find(|t| **t == words[0]) {
            Some(word) if blocks.is_empty() && words.len() > 1 => {
                tops.push(Top { word, values: words[1..].iter().map(|w| w.to_string()).collect(), line: n })
            }
            Some(word) if blocks.is_empty() => return Err(refusal(file, n, format!("'{}' takes a value", word))),
            _ => {
                let mut named = vec!["'rule'".to_string()];
                named.extend(g.tops.iter().map(|t| format!("'{}'", t)));
                let before = if g.tops.is_empty() { "" } else { ", every other one before the first rule" };
                return Err(refusal(
                    file,
                    n,
                    format!("unknown line '{}' (a top-level line is {}{})", words[0], named.join(" or "), before),
                ));
            }
        }
    }
    for b in &blocks {
        let (_, req, _) = kind_of(b.kind).expect("a block's kind is known");
        if let Some(missing) = req.iter().find(|r| b.values(r).next().is_none()) {
            return Err(refusal(file, b.line, format!("rule {} lacks its '{}' key", b.id, missing)));
        }
    }
    Ok((tops, blocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    const G: Grammar = Grammar {
        kinds: &[("pair", &["a"], &["b"])],
        once: &["a"],
        tops: &["floor"],
    };

    // spec: gate-sdk/SPEC.md §check-script-preflight — blocks, their keys and the top-level lines
    // before the first rule
    #[test]
    fn blocks_and_tops_parse() {
        let (tops, blocks) = parse("f", "# c\nfloor 3\n\nrule X-1 pair\n  a x y\n\tb one\n  b two\n", &G).expect("parses");
        assert_eq!((tops[0].word, tops[0].values.clone(), tops[0].line), ("floor", vec!["3".to_string()], 2));
        assert_eq!((blocks[0].id.as_str(), blocks[0].line, blocks[0].one("a")), ("X-1", 4, "x y".to_string()));
        assert_eq!(blocks[0].all("b"), ["one", "two"]);
        assert_eq!(blocks[0].words("a"), ["x", "y"]);
    }

    // spec: gate-sdk/SPEC.md §check-script-preflight — every malformation refuses naming its line
    #[test]
    fn malformations_refuse_at_their_line() {
        for (text, at) in [
            ("  a x\n", "f:1: a key line"),
            ("rule X pair\n  c x\n", "f:2: unknown key"),
            ("rule X pair\n  a\n", "f:2: key 'a' takes"),
            ("rule X pair\n  a x\n  a y\n", "f:3: key 'a' given twice"),
            ("rule X\n", "f:1: a rule line"),
            ("rule x pair\n", "f:1: rule id"),
            ("rule X other\n", "f:1: unknown kind"),
            ("rule X pair\n  b y\n", "f:1: rule X lacks its 'a' key"),
            ("rule X pair\n  a x\nfloor 2\n", "f:3: unknown line 'floor'"),
            ("floor\n", "f:1: 'floor' takes"),
            ("other 1\n", "f:1: unknown line 'other'"),
        ] {
            let fault = parse("f", text, &G).err().unwrap_or_else(|| panic!("{:?} parsed", text));
            assert_eq!(fault.status(), crate::exit::REFUSED, "{:?}", text);
            assert!(fault.message.starts_with(at), "{:?}: {}", text, fault.message);
        }
        let (_, blocks) = parse("f", "rule X pair\n  a (\n", &G).expect("parses");
        assert!(blocks[0].eres("f", "a").err().is_some_and(|e| e.message.starts_with("f:2: a:")));
    }
}
