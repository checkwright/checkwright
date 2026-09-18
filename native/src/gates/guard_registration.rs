// spec: guard-kit/SPEC.md §check-guard-registration — the generic ruleset's numbered roster, its
// guard_rule_* definitions and guard_generic_rules' dispatch order agree one-for-one and in order,
// fail-closed on an unreadable section or dispatcher
use crate::diff;
use crate::walk;
use std::collections::BTreeSet;

const NAME: &str = "check-guard-registration";
const SECTION: &str = "The generic ruleset";
const DISPATCHER: &str = "guard_generic_rules() {";
const PREFIX: &str = "guard_rule_";

struct Item {
    number: String,
    line: usize,
    tokens: Vec<String>,
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let given = |at: usize| args.get(at).filter(|a| !a.is_empty()).cloned();
    let (spec, lib) = match (given(0), given(1)) {
        (Some(s), Some(l)) => (s, l),
        (s, l) => {
            let root = kit_root()?;
            (
                s.unwrap_or_else(|| format!("{}/SPEC.md", root)),
                l.unwrap_or_else(|| format!("{}/lib/guard.sh", root)),
            )
        }
    };
    let spec_text = read(&spec)?;
    let lib_text = read(&lib)?;

    let items = roster(&spec_text, &spec)?;
    let calls = dispatch(&lib_text, &lib)?;
    let defined = definitions(&lib_text);

    let mut findings = 0usize;

    let mut numbering = false;
    for (i, it) in items.iter().enumerate() {
        if it.number != (i + 1).to_string() {
            println!(
                "{}:{}: roster item numbered {} where {} is next — the items are numbered from 1 with no gap or repeat",
                spec,
                it.line,
                it.number,
                i + 1
            );
            numbering = true;
            findings += 1;
        }
    }
    if numbering {
        println!("  help: renumber §{} so its items run 1..n, and repoint every \"rule N\" citation the renumbering moves", SECTION);
    }

    let mut tokening = false;
    for it in &items {
        if it.tokens.len() != 1 {
            let named = if it.tokens.is_empty() {
                "no guard_rule_* function".to_string()
            } else {
                format!("{} guard_rule_* functions ({})", it.tokens.len(), it.tokens.join(", "))
            };
            println!(
                "{}:{}: roster item {} names {} where exactly one is owed",
                spec, it.line, it.number, named
            );
            tokening = true;
            findings += 1;
        }
    }
    if tokening {
        println!("  help: carry the rule's own `guard_rule_<name>` token right after the item's bold title, and cite another rule by number (guard-kit/SPEC.md §The generic ruleset)");
    }

    let listed: Vec<&str> = items
        .iter()
        .filter_map(|it| it.tokens.first().map(String::as_str))
        .collect();
    let called: Vec<&str> = calls.iter().map(String::as_str).collect();
    if listed != called {
        println!(
            "{}: the roster's tokens in order differ from guard_generic_rules' calls in order (< roster, > dispatch):",
            spec
        );
        for l in diff::normal_diff(&listed, &called) {
            println!("  {}", l);
        }
        println!("  help: dispatch one call per roster item in roster order — reorder the calls in {} or the items in §{}", lib, SECTION);
        findings += 1;
    }

    let dispatched: BTreeSet<&str> = called.iter().copied().collect();
    let defined_set: BTreeSet<&str> = defined.iter().map(String::as_str).collect();
    let mut setting = false;
    for f in defined_set.difference(&dispatched) {
        println!("{}: {} is defined but never dispatched by guard_generic_rules", lib, f);
        setting = true;
        findings += 1;
    }
    for f in dispatched.difference(&defined_set) {
        println!("{}: {} is dispatched by guard_generic_rules but never defined", lib, f);
        setting = true;
        findings += 1;
    }
    if setting {
        println!("  help: define every dispatched rule and dispatch every defined one — a rule defined and never called never runs");
    }

    if findings > 0 {
        return Ok(1);
    }
    println!(
        "GUARD-REGISTRATION: clean ({} rule(s): roster, definitions and dispatch order in lockstep)",
        items.len()
    );
    Ok(0)
}

// spec: guard-kit/SPEC.md §Testing — the gate reaches guard-kit through the transported kit
// roots, the resolution --run-guard-tests uses
fn kit_root() -> Result<String, String> {
    walk::kit_roots_abs()?
        .into_iter()
        .find(|r| r.rsplit('/').next() == Some("guard-kit"))
        .ok_or_else(|| {
            "the kit roots name no guard-kit root, so neither the ruleset section nor the library can be found".to_string()
        })
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

fn heading(line: &str) -> Option<&str> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    line[hashes..].strip_prefix(' ').map(str::trim)
}

fn lead_number(line: &str) -> Option<&str> {
    let digits = line.len() - line.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if digits == 0 || !line[digits..].starts_with(". ") {
        return None;
    }
    Some(&line[..digits])
}

fn tokens(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let bytes = text.as_bytes();
    let ident = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_';
    let mut from = 0usize;
    while let Some(off) = text[from..].find(PREFIX) {
        let at = from + off;
        let mut end = at + PREFIX.len();
        while end < bytes.len() && ident(bytes[end]) {
            end += 1;
        }
        let bounded = at == 0 || !(ident(bytes[at - 1]) || bytes[at - 1].is_ascii_uppercase());
        if bounded && end > at + PREFIX.len() {
            let t = text[at..end].to_string();
            if !out.contains(&t) {
                out.push(t);
            }
        }
        from = end;
    }
    out
}

fn roster(text: &str, path: &str) -> Result<Vec<Item>, String> {
    let mut inside = false;
    let mut found = false;
    let mut fence = false;
    let mut items: Vec<Item> = Vec::new();
    let mut open: Option<(Item, String)> = None;
    for (n, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            fence = !fence;
        }
        if !fence {
            if let Some(h) = heading(line) {
                if inside {
                    break;
                }
                if h == SECTION {
                    inside = true;
                    found = true;
                }
                continue;
            }
        }
        if !inside {
            continue;
        }
        if let Some(num) = lead_number(line) {
            if let Some((it, body)) = open.take() {
                items.push(close(it, &body));
            }
            let item = Item {
                number: num.to_string(),
                line: n + 1,
                tokens: Vec::new(),
            };
            open = Some((item, line.to_string()));
            continue;
        }
        let continues = line.trim().is_empty() || line.starts_with(|c: char| c.is_whitespace());
        match open.take() {
            Some((it, mut body)) if continues => {
                body.push('\n');
                body.push_str(line);
                open = Some((it, body));
            }
            Some((it, body)) => items.push(close(it, &body)),
            None => {}
        }
    }
    if let Some((it, body)) = open.take() {
        items.push(close(it, &body));
    }
    if !found {
        return Err(format!("{}: no \"{}\" section heading — the roster is unreadable", path, SECTION));
    }
    if items.is_empty() {
        return Err(format!("{}: §{} carries no numbered item — the roster is unreadable", path, SECTION));
    }
    Ok(items)
}

fn close(mut it: Item, body: &str) -> Item {
    it.tokens = tokens(body);
    it
}

fn dispatch(text: &str, path: &str) -> Result<Vec<String>, String> {
    let mut lines = text.lines().enumerate().skip_while(|(_, l)| *l != DISPATCHER);
    if lines.next().is_none() {
        return Err(format!("{}: no `{}` definition — the dispatch order is unreadable", path, DISPATCHER));
    }
    let mut calls: Vec<String> = Vec::new();
    for (n, line) in lines {
        if line == "}" {
            return Ok(calls);
        }
        let t = line.trim();
        if t.is_empty() || t == "local cmd=\"$1\"" {
            continue;
        }
        match t.strip_suffix(" \"$cmd\"") {
            Some(name)
                if name.starts_with(PREFIX)
                    && name.len() > PREFIX.len()
                    && tokens(name) == [name.to_string()] =>
            {
                calls.push(name.to_string())
            }
            _ => {
                return Err(format!(
                    "{}:{}: a guard_generic_rules line of no known shape ({}) — the gate reads only `guard_rule_<name> \"$cmd\"` calls, the `local cmd=\"$1\"` binding and blanks",
                    path,
                    n + 1,
                    t
                ))
            }
        }
    }
    Err(format!("{}: `{}` is never closed by a column-0 `}}` — the dispatch order is unreadable", path, DISPATCHER))
}

fn definitions(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| l.strip_suffix("() {"))
        .filter(|name| name.starts_with(PREFIX) && tokens(name) == [name.to_string()])
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_needs_a_name_and_a_word_boundary() {
        assert_eq!(tokens("`guard_rule_*` and `guard_rule_cd` then _guard_rule_x"), vec!["guard_rule_cd"]);
    }

    #[test]
    fn an_item_ends_at_the_first_unindented_non_blank_line() {
        let spec = "## The generic ruleset\n\n1. **a** (`guard_rule_a`) — x\n   more\n\n   `guard_rule_a` again\n\nProse `guard_rule_b`.\n2. **b** (`guard_rule_b`)\n## Next\n3. **c**\n";
        let items = roster(spec, "s").unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].tokens, vec!["guard_rule_a"]);
        assert_eq!(items[1].tokens, vec!["guard_rule_b"]);
    }
}
