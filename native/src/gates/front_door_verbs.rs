// spec: installer/SPEC.md §The front door's verbs — the verb table is the binary's roster (A), and
// every route-advertised verb is in the pinned release's table, or pending its release (B)
use super::install_pin::{is_triple, pin_of};
use super::release_bump::disposition_file;
use crate::{fresh, proc, programs, queue, stages};
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-front-door-verbs";
const README: &str = "installer/README.md";
const INSTALL_SH: &str = "docs/install.sh";
const PAGES: &[&str] = &["README.md", "docs/index.md", "docs/install.md", "installer/README.md"];
const ROUTES: &[&str] = &["sh -s --", "install.ps1)))", "npx checkwright"];
const SPAN_ROUTE: &str = "checkwright";
// spec: installer/SPEC.md §The dependency boundary — the verb the one-line install runs when no
// verb is named, which a flag-led route therefore advertises
const DEFAULT_VERB: &str = "init";

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
    Some(if t.starts_with('-') {
        Tok::Verb(DEFAULT_VERB.to_string())
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

#[derive(Debug, PartialEq)]
enum Disposition {
    Absent,
    Release,
    Withheld(String),
}

// spec: installer/SPEC.md §The front door's verbs — only the line keyed by the queue's iteration,
// and only its field, in one of three forms
fn disposition(path: &str, text: &str, iteration: &str) -> Result<Disposition, String> {
    let mut field: Option<&str> = None;
    for line in text.lines() {
        let mut f = line.split_whitespace();
        if f.next() == Some(iteration) && f.next() == Some("release") {
            field = Some(f.next().unwrap_or(""));
        }
    }
    let Some(field) = field else {
        return Ok(Disposition::Absent);
    };
    if field == "none" || field.strip_prefix("deferred:v").is_some_and(is_triple) {
        return Ok(Disposition::Withheld(field.to_string()));
    }
    if field.strip_prefix('v').is_some_and(is_triple) {
        return Ok(Disposition::Release);
    }
    Err(format!(
        "{}: iteration {}'s disposition field '{}' is none of vX.Y.Z, none or deferred:vX.Y.Z",
        path, iteration, field
    ))
}

fn git_ok(args: &[&str]) -> Result<Option<String>, String> {
    let c = proc::run(&programs::GIT, args)?;
    if c.code() != Some(0) {
        return Ok(None);
    }
    Ok(c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
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
        let pin = pin_of(INSTALL_SH, &fresh::read_captured(INSTALL_SH)?, "pin")?;
        let tag = format!("v{}", pin);
        match git_ok(&["rev-parse", "--verify", "--quiet", &format!("refs/tags/{}", tag)])? {
            None => None,
            Some(_) => match git_ok(&["show", &format!("{}:{}", tag, README)])? {
                Some(text) => Some((tag, text)),
                None => return Err(format!("the tag {} exists and carries no {}", tag, README)),
            },
        }
    };
    let disposition_path = if positional { args[2].clone() } else { disposition_file()? };
    let queue_path = if positional {
        args[3].clone()
    } else {
        queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?
    };
    let pages: Vec<String> = if positional {
        args[4..].to_vec()
    } else {
        PAGES.iter().map(|p| p.to_string()).collect()
    };

    let queue_text = read(&queue_path)?;
    let iteration = stages::header(&queue_text)
        .map(stages::header_iter)
        .ok_or_else(|| format!("{} carries no '## Iteration:' header", queue_path))?;
    let disp = if Path::new(&disposition_path).is_file() {
        disposition(&disposition_path, &read(&disposition_path)?, &iteration)?
    } else {
        Disposition::Absent
    };

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
    let mut pending: BTreeSet<String> = BTreeSet::new();
    let mut sites = 0usize;
    for page in &pages {
        let text = read(page)?;
        for (n, code, span) in code_texts(&text) {
            for tok in advertised(code, span) {
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

    if !a_findings.is_empty() || !b_findings.is_empty() {
        println!("{}: the front door advertises a verb its pinned release does not carry, or the verb table is not the binary's roster (installer/SPEC.md §The front door's verbs):", NAME);
        for f in b_findings.iter().chain(&a_findings) {
            println!("{}", f);
        }
        if !b_findings.is_empty() {
            println!("  help: release the verb so the pin carries it (RELEASING.md), or withdraw the advertisement from the page.");
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

    // spec: installer/SPEC.md §The front door's verbs — each route, a flag, a placeholder, and a
    // token outside any code span
    #[test]
    fn the_route_tokenizer_reads_each_route_a_flag_and_a_placeholder() {
        assert_eq!(verbs("curl -fsSL x | sh -s -- demo   # comment", false), vec![v("demo")]);
        assert_eq!(verbs("& ([scriptblock]::Create((irm x/install.ps1))) doctor", false), vec![v("doctor")]);
        assert_eq!(verbs("npx checkwright init", true), vec![v("init")]);
        assert_eq!(verbs("checkwright uninstall", true), vec![v("uninstall")]);
        assert_eq!(verbs("checkwright uninstall", false), vec![]);
        assert_eq!(verbs("sh -s -- --profile full", false), vec![v("init")]);
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

    // spec: installer/SPEC.md §The front door's verbs — the three disposition forms, the absent
    // line, and a field of none of them
    #[test]
    fn the_disposition_reads_three_forms_and_the_absent_line() {
        let d = |t: &str| disposition("d", t, "it");
        assert_eq!(d("other release none — b\n"), Ok(Disposition::Absent));
        assert_eq!(d("it release v1.2.3 — b\n"), Ok(Disposition::Release));
        assert_eq!(d("it release none — b\n"), Ok(Disposition::Withheld("none".to_string())));
        assert_eq!(
            d("it release deferred:v1.2.3 — b\n"),
            Ok(Disposition::Withheld("deferred:v1.2.3".to_string()))
        );
        assert!(d("it release soon — b\n").is_err());
        assert!(d("it release deferred:1.2 — b\n").is_err());
    }

    // spec: installer/SPEC.md §The front door's verbs — the binary's roster is read in-process
    #[test]
    fn the_binary_roster_carries_the_default_verb() {
        assert!(binary_verbs().contains(DEFAULT_VERB));
    }
}
