// spec: installer/SPEC.md §The front door's verbs — the verb table is the binary's roster (A), and
// every route-advertised verb is in the pinned release's table, or pending its release (B)
use super::pinned_release::{self, read, Disposition};
use std::collections::BTreeSet;

const NAME: &str = "check-front-door-verbs";
const README: &str = "installer/README.md";
const PAGES: &[&str] = &["README.md", "docs/index.md", "docs/install.md", "installer/README.md"];
const ROUTES: &[&str] = &["sh -s --", "install.ps1)))", "npx checkwright"];
const SPAN_ROUTE: &str = "checkwright";
// spec: installer/SPEC.md §The front door's verbs — the one flag a route may lead with
const HELP_FLAGS: &[&str] = &["--help", "-h"];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

fn binary_verbs() -> BTreeSet<String> {
    crate::installer::VERBS
        .iter()
        .map(|(v, _)| v.trim_start_matches("--").to_string())
        .collect()
}

fn cells(line: &str) -> Vec<&str> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    t.split('|').map(str::trim).collect()
}

// spec: installer/SPEC.md §The front door's verbs — the first-column code spans of the table whose
// header row's first cell is `verb`; one reader for the HEAD file and the tag's blob alike
fn verb_table(text: &str) -> Option<Vec<String>> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim_start().starts_with('|') && cells(l).first() == Some(&"verb"))?;
    let mut out = Vec::new();
    for l in lines[start + 1..].iter().take_while(|l| l.trim_start().starts_with('|')) {
        let first = cells(l).first().copied().unwrap_or("");
        if let Some(v) = first.strip_prefix('`').and_then(|s| s.strip_suffix('`')) {
            out.push(v.to_string());
        }
    }
    Some(out)
}

// spec: installer/SPEC.md §The front door's verbs — a page's code: each inline code span, and each
// line of a fenced block, with whether the text opens a code span
fn code_texts(text: &str) -> Vec<(usize, &str, bool)> {
    let mut out = Vec::new();
    let mut fenced = false;
    for (i, line) in text.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            out.push((i + 1, line, false));
            continue;
        }
        for (k, seg) in line.split('`').enumerate() {
            if k % 2 == 1 {
                out.push((i + 1, seg, true));
            }
        }
    }
    out
}

#[derive(Debug, PartialEq)]
enum Tok {
    Verb(String),
    Flag(String),
    Placeholder,
}

fn is_verb(t: &str) -> bool {
    let b = t.as_bytes();
    !b.is_empty()
        && b[0].is_ascii_lowercase()
        && b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}

fn token(after: &str) -> Option<Tok> {
    if !after.starts_with(char::is_whitespace) {
        return None;
    }
    let t = after.split_whitespace().next()?;
    Some(if HELP_FLAGS.contains(&t) {
        Tok::Placeholder
    } else if t.starts_with('-') {
        Tok::Flag(t.to_string())
    } else if is_verb(t) {
        Tok::Verb(t.to_string())
    } else {
        Tok::Placeholder
    })
}

// spec: installer/SPEC.md §The front door's verbs — the token after each route in one code text
fn advertised(code: &str, span: bool) -> Vec<Tok> {
    let mut out = Vec::new();
    for route in ROUTES {
        let mut from = 0;
        while let Some(i) = code[from..].find(route) {
            let end = from + i + route.len();
            out.extend(token(&code[end..]));
            from = end;
        }
    }
    if span {
        if let Some(after) = code.trim_start().strip_prefix(SPAN_ROUTE) {
            out.extend(token(after));
        }
    }
    out
}

fn table(label: &str, text: &str) -> Result<Vec<String>, String> {
    match verb_table(text) {
        Some(v) if !v.is_empty() => Ok(v),
        _ => Err(format!(
            "{} carries no verb table (a table whose header row's first cell is 'verb', listing code-spanned verbs)",
            label
        )),
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let positional = !args.is_empty();
    if positional && args.len() < 5 {
        return Err("usage: check-front-door-verbs [readme pinned-readme disposition queue page...]".to_string());
    }
    let readme = if positional { args[0].clone() } else { README.to_string() };
    // spec: installer/SPEC.md §The front door's verbs — the pinned set is the table at the tag the
    // hosted pin names, or the positional file; an unresolvable tag leaves B dormant
    let pinned: Option<(String, String)> = if positional {
        Some((args[1].clone(), read(&args[1])?))
    } else {
        match pinned_release::pinned_tag()? {
            None => None,
            Some(tag) => {
                let text = pinned_release::show_at(&tag, README)?;
                Some((tag, text))
            }
        }
    };
    let (disposition_path, queue_path) = if positional {
        (args[2].clone(), args[3].clone())
    } else {
        pinned_release::live_paths()?
    };
    let pages: Vec<String> = if positional {
        args[4..].to_vec()
    } else {
        PAGES.iter().map(|p| p.to_string()).collect()
    };

    let (iteration, disp) = pinned_release::iteration_disposition(&disposition_path, &queue_path)?;

    let binary = binary_verbs();
    let head: BTreeSet<String> = table(&readme, &read(&readme)?)?.into_iter().collect();
    let pinned_set: Option<(String, BTreeSet<String>)> = match &pinned {
        Some((label, text)) => Some((label.clone(), table(label, text)?.into_iter().collect())),
        None => None,
    };

    let mut a_findings: Vec<String> = Vec::new();
    for v in binary.difference(&head) {
        a_findings.push(format!("  {}: the verb table lacks `{}`, which the binary's VERBS carries", readme, v));
    }
    for v in head.difference(&binary) {
        a_findings.push(format!("  {}: the verb table lists `{}`, which the binary's VERBS does not carry", readme, v));
    }

    let mut b_findings: Vec<String> = Vec::new();
    let mut flag_findings: Vec<String> = Vec::new();
    let mut pending: BTreeSet<String> = BTreeSet::new();
    let mut sites = 0usize;
    for page in &pages {
        let text = read(page)?;
        for (n, code, span) in code_texts(&text) {
            for tok in advertised(code, span) {
                if let Tok::Flag(f) = &tok {
                    flag_findings.push(format!(
                        "  {}:{}: a route leads with `{}`, which reaches the binary as its first argument rather than a verb's",
                        page, n, f
                    ));
                    continue;
                }
                let Tok::Verb(v) = tok else { continue };
                sites += 1;
                let Some((label, set)) = &pinned_set else { continue };
                if set.contains(&v) {
                    continue;
                }
                if !binary.contains(&v) {
                    b_findings.push(format!(
                        "  {}:{}: `{}` is advertised, and neither the pinned release {} nor the binary's VERBS carries it",
                        page, n, v, label
                    ));
                    continue;
                }
                match &disp {
                    Disposition::Absent | Disposition::Release => {
                        pending.insert(v);
                    }
                    Disposition::Withheld(field) => b_findings.push(format!(
                        "  {}:{}: `{}` is advertised, and the pinned release {} lacks it while iteration {}'s disposition is {}",
                        page, n, v, label, iteration, field
                    )),
                }
            }
        }
    }

    if !a_findings.is_empty() || !b_findings.is_empty() || !flag_findings.is_empty() {
        println!("{}: the front door advertises a verb its pinned release does not carry, leads a route with a flag, or the verb table is not the binary's roster (installer/SPEC.md §The front door's verbs):", NAME);
        for f in b_findings.iter().chain(&flag_findings).chain(&a_findings) {
            println!("{}", f);
        }
        if !b_findings.is_empty() {
            println!("  help: release the verb so the pin carries it (RELEASING.md), or withdraw the advertisement from the page.");
        }
        if !flag_findings.is_empty() {
            println!("  help: name the verb before its flags, as `sh -s -- init --profile prose`: the one-line install runs `init` only on an empty argument list, and the bootstrap forwards a leading flag unchanged.");
        }
        if !a_findings.is_empty() {
            println!("  help: make {}'s verb table list exactly the verbs native/src/installer/mod.rs's VERBS carries.", readme);
        }
        return Ok(1);
    }
    let pinned_state = match &pinned_set {
        Some((label, _)) => format!("pinned set {}", label),
        None => "B dormant — the pinned tag does not resolve here".to_string(),
    };
    let pending_state = if pending.is_empty() {
        String::new()
    } else {
        format!(", pending release: {}", pending.into_iter().collect::<Vec<_>>().join(" "))
    };
    println!(
        "FRONT-DOOR-VERBS: clean ({} page(s), {} advertised site(s), {}, verb table equal to VERBS{})",
        pages.len(),
        sites,
        pinned_state,
        pending_state
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verbs(code: &str, span: bool) -> Vec<Tok> {
        advertised(code, span)
    }

    fn v(s: &str) -> Tok {
        Tok::Verb(s.to_string())
    }

    // spec: installer/SPEC.md §The front door's verbs — each route, a flag, the help flag, a
    // placeholder, and a token outside any code span
    #[test]
    fn the_route_tokenizer_reads_each_route_a_flag_and_a_placeholder() {
        assert_eq!(verbs("curl -fsSL x | sh -s -- demo   # comment", false), vec![v("demo")]);
        assert_eq!(verbs("& ([scriptblock]::Create((irm x/install.ps1))) doctor", false), vec![v("doctor")]);
        assert_eq!(verbs("npx checkwright init", true), vec![v("init")]);
        assert_eq!(verbs("checkwright uninstall", true), vec![v("uninstall")]);
        assert_eq!(verbs("checkwright uninstall", false), vec![]);
        assert_eq!(verbs("sh -s -- --profile full", false), vec![Tok::Flag("--profile".to_string())]);
        assert_eq!(verbs("sh -s -- init --profile full", false), vec![v("init")]);
        assert_eq!(verbs("checkwright --help", true), vec![Tok::Placeholder]);
        assert_eq!(verbs("checkwright <verb>", true), vec![Tok::Placeholder]);
        assert_eq!(verbs("checkwright", true), vec![]);
        assert_eq!(verbs("checkwright.lock", true), vec![]);
        assert_eq!(verbs("npx checkwright-pwsh init", true), vec![]);
        let page = "run npx checkwright demo here, or `npx checkwright init`\n```sh\nsh -s -- diff\n```\n";
        let found: Vec<(usize, Tok)> = code_texts(page)
            .into_iter()
            .flat_map(|(n, c, s)| advertised(c, s).into_iter().map(move |t| (n, t)))
            .collect();
        assert_eq!(found, vec![(1, v("init")), (3, v("diff"))]);
    }

    // spec: installer/SPEC.md §The front door's verbs — the table whose header's first cell is
    // `verb`, its first-column code spans, and nothing past the table
    #[test]
    fn the_table_reader_takes_the_verb_tables_first_column() {
        let t = "# x\n\n| verb | asks |\n| --- | --- |\n| `init` | a |\n| `demo` | b |\n\n| `other` | c |\n";
        assert_eq!(verb_table(t), Some(vec!["init".to_string(), "demo".to_string()]));
        assert_eq!(verb_table("| name | x |\n| --- | --- |\n| `a` | b |\n"), None);
    }
}
