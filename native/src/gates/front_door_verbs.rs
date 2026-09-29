// spec: installer/SPEC.md §The front door's verbs — the verb and flag tables are the binary's
// rosters (A), and every route-advertised verb and each flag after it is in the pinned release's
// tables, or pending its release (B)
use super::pinned_release::{self, read, Disposition};
use std::collections::BTreeSet;

const NAME: &str = "check-front-door-verbs";
const README: &str = "installer/README.md";
const PAGES: &[&str] = &[
    "README.md",
    "docs/index.md",
    "docs/install.md",
    "installer/README.md",
    "plugin/skills/install/SKILL.md",
    "docs/speckit.md",
    "docs/openspec.md",
];
const ROUTES: &[&str] = &["sh -s --", "install.ps1)))", "npx checkwright"];
const CODE_ROUTE: &str = "checkwright";
// spec: installer/SPEC.md §The front door's verbs — the one flag a route may lead with
const HELP_FLAGS: &[&str] = &["--help", "-h"];
// spec: installer/SPEC.md §The front door's verbs — the tokens that end a verb's flag walk
const STOPS: &[&str] = &["|", ";", "&&", "||", ")"];

type Pair = (String, String);

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

fn binary_flags() -> BTreeSet<Pair> {
    crate::installer::FLAGS
        .iter()
        .flat_map(|(f, vs)| vs.iter().map(move |v| (v.to_string(), f.to_string())))
        .collect()
}

fn cells(line: &str) -> Vec<&str> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    t.split('|').map(str::trim).collect()
}

fn code_span(cell: &str) -> Option<&str> {
    cell.strip_prefix('`').and_then(|s| s.strip_suffix('`'))
}

// spec: installer/SPEC.md §The front door's verbs — the body rows of the table whose header row's
// first cell is `header`; one reader for the HEAD file and the tag's blob alike
fn table_rows<'a>(text: &'a str, header: &str) -> Option<Vec<Vec<&'a str>>> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim_start().starts_with('|') && cells(l).first() == Some(&header))?;
    Some(
        lines[start + 1..]
            .iter()
            .take_while(|l| l.trim_start().starts_with('|'))
            .map(|l| cells(l))
            .collect(),
    )
}

fn verb_table(text: &str) -> Option<Vec<String>> {
    let rows = table_rows(text, "verb")?;
    Some(
        rows.iter()
            .filter_map(|r| code_span(r.first().copied().unwrap_or("")))
            .map(str::to_string)
            .collect(),
    )
}

// spec: installer/SPEC.md §The front door's verbs — one pair per verb the second column lists
// beside the first column's flag
fn flag_table(text: &str) -> Option<Vec<Pair>> {
    let rows = table_rows(text, "flag")?;
    let mut out = Vec::new();
    for r in rows {
        let Some(flag) = code_span(r.first().copied().unwrap_or("")) else { continue };
        for v in r.get(1).copied().unwrap_or("").split(',') {
            if let Some(v) = code_span(v.trim()) {
                out.push((v.to_string(), flag.to_string()));
            }
        }
    }
    Some(out)
}

// spec: installer/SPEC.md §The front door's verbs — a page's code: each inline code span, and each
// line of a fenced block
fn code_texts(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut fenced = false;
    for (i, line) in text.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            out.push((i + 1, line));
            continue;
        }
        for (k, seg) in line.split('`').enumerate() {
            if k % 2 == 1 {
                out.push((i + 1, seg));
            }
        }
    }
    out
}

#[derive(Debug, PartialEq)]
enum Tok {
    Verb(String, Vec<String>),
    Flag(String),
    Placeholder,
}

fn is_verb(t: &str) -> bool {
    let b = t.as_bytes();
    !b.is_empty()
        && b[0].is_ascii_lowercase()
        && b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}

// spec: installer/SPEC.md §The front door's verbs — the flags after an advertised verb, read to the
// first command separator or comment, each cut at its first `=`
fn flags_after<'a>(words: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut out = Vec::new();
    for w in words {
        if STOPS.contains(&w) || w.starts_with('#') || w.starts_with('>') {
            break;
        }
        if w.starts_with('-') && !HELP_FLAGS.contains(&w) {
            out.push(w.split('=').next().unwrap_or(w).to_string());
        }
    }
    out
}

fn token(after: &str) -> Option<Tok> {
    if !after.starts_with(char::is_whitespace) {
        return None;
    }
    let mut words = after.split_whitespace();
    let t = words.next()?;
    Some(if HELP_FLAGS.contains(&t) {
        Tok::Placeholder
    } else if t.starts_with('-') {
        Tok::Flag(t.to_string())
    } else if is_verb(t) {
        Tok::Verb(t.to_string(), flags_after(words))
    } else {
        Tok::Placeholder
    })
}

// spec: installer/SPEC.md §The front door's verbs — the token after each route in one code text,
// `checkwright` routing only where it opens the text
fn advertised(code: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    for route in ROUTES {
        let mut from = 0;
        while let Some(i) = code[from..].find(route) {
            let end = from + i + route.len();
            out.extend(token(&code[end..]));
            from = end;
        }
    }
    if let Some(after) = code.trim_start().strip_prefix(CODE_ROUTE) {
        out.extend(token(after));
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

fn head_flags(label: &str, text: &str) -> Result<BTreeSet<Pair>, String> {
    match flag_table(text) {
        Some(v) if !v.is_empty() => Ok(v.into_iter().collect()),
        _ => Err(format!(
            "{} carries no flag table (a table whose header row's first cell is 'flag', listing a code-spanned flag beside its code-spanned verbs)",
            label
        )),
    }
}

struct Pinned {
    label: String,
    verbs: BTreeSet<String>,
    flags: BTreeSet<Pair>,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let positional = !args.is_empty();
    if positional && args.len() < 5 {
        return Err("usage: check-front-door-verbs [readme pinned-readme disposition queue page...]".to_string());
    }
    let readme = if positional { args[0].clone() } else { README.to_string() };
    // spec: installer/SPEC.md §The front door's verbs — the pinned sets are the tables at the tag the
    // hosted pin names, or the positional file; an unresolvable tag leaves B dormant
    let pinned_text: Option<(String, String)> = if positional {
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
    let binary_flags = binary_flags();
    let readme_text = read(&readme)?;
    let head: BTreeSet<String> = table(&readme, &readme_text)?.into_iter().collect();
    let head_flags = head_flags(&readme, &readme_text)?;
    // spec: installer/SPEC.md §The front door's verbs — a tag carrying no flag table has the empty
    // pinned flag set, never a dormancy
    let pinned: Option<Pinned> = match &pinned_text {
        Some((label, text)) => Some(Pinned {
            label: label.clone(),
            verbs: table(label, text)?.into_iter().collect(),
            flags: flag_table(text).unwrap_or_default().into_iter().collect(),
        }),
        None => None,
    };

    let mut a_findings: Vec<String> = Vec::new();
    for v in binary.difference(&head) {
        a_findings.push(format!("  {}: the verb table lacks `{}`, which the binary's VERBS carries", readme, v));
    }
    for v in head.difference(&binary) {
        a_findings.push(format!("  {}: the verb table lists `{}`, which the binary's VERBS does not carry", readme, v));
    }
    for (v, f) in binary_flags.difference(&head_flags) {
        a_findings.push(format!("  {}: the flag table lacks `{}` for `{}`, which the binary's FLAGS carries", readme, f, v));
    }
    for (v, f) in head_flags.difference(&binary_flags) {
        a_findings.push(format!("  {}: the flag table lists `{}` for `{}`, which the binary's FLAGS does not carry", readme, f, v));
    }

    let mut b_findings: Vec<String> = Vec::new();
    let mut unreleased_flags: Vec<String> = Vec::new();
    let mut lead_findings: Vec<String> = Vec::new();
    let mut pending: BTreeSet<String> = BTreeSet::new();
    let mut pending_flags: BTreeSet<String> = BTreeSet::new();
    let (mut sites, mut flag_sites) = (0usize, 0usize);
    for page in &pages {
        let text = read(page)?;
        for (n, code) in code_texts(&text) {
            for tok in advertised(code) {
                if let Tok::Flag(f) = &tok {
                    lead_findings.push(format!(
                        "  {}:{}: a route leads with `{}`, which reaches the binary as its first argument rather than a verb's",
                        page, n, f
                    ));
                    continue;
                }
                let Tok::Verb(v, flags) = tok else { continue };
                sites += 1;
                flag_sites += flags.len();
                let Some(p) = &pinned else { continue };
                if !p.verbs.contains(&v) {
                    if !binary.contains(&v) {
                        b_findings.push(format!(
                            "  {}:{}: `{}` is advertised, and neither the pinned release {} nor the binary's VERBS carries it",
                            page, n, v, p.label
                        ));
                    } else {
                        match &disp {
                            Disposition::Absent | Disposition::Release => {
                                pending.insert(v.clone());
                            }
                            Disposition::Withheld(field) => b_findings.push(format!(
                                "  {}:{}: `{}` is advertised, and the pinned release {} lacks it while iteration {}'s disposition is {}",
                                page, n, v, p.label, iteration, field
                            )),
                        }
                    }
                }
                for f in flags {
                    let pair = (v.clone(), f.clone());
                    if p.flags.contains(&pair) {
                        continue;
                    }
                    if !binary_flags.contains(&pair) {
                        unreleased_flags.push(format!(
                            "  {}:{}: `{} {}` is advertised, and neither the pinned release {} nor the binary's FLAGS carries it",
                            page, n, v, f, p.label
                        ));
                        continue;
                    }
                    match &disp {
                        Disposition::Absent | Disposition::Release => {
                            pending_flags.insert(format!("{} {}", v, f));
                        }
                        Disposition::Withheld(field) => unreleased_flags.push(format!(
                            "  {}:{}: `{} {}` is advertised, and the pinned release {} lacks it while iteration {}'s disposition is {}",
                            page, n, v, f, p.label, iteration, field
                        )),
                    }
                }
            }
        }
    }

    if !a_findings.is_empty() || !b_findings.is_empty() || !unreleased_flags.is_empty() || !lead_findings.is_empty() {
        println!("{}: the front door advertises a verb or flag its pinned release does not carry, leads a route with a flag, or a table is not the binary's roster (installer/SPEC.md §The front door's verbs):", NAME);
        for f in b_findings.iter().chain(&unreleased_flags).chain(&lead_findings).chain(&a_findings) {
            println!("{}", f);
        }
        if !b_findings.is_empty() {
            println!("  help: release the verb so the pin carries it (RELEASING.md), or withdraw the advertisement from the page.");
        }
        if !unreleased_flags.is_empty() {
            println!("  help: release the flag so the pin carries it (RELEASING.md), or withdraw it from the page.");
        }
        if !lead_findings.is_empty() {
            println!("  help: name the verb before its flags, as `sh -s -- init --profile prose`: the one-line install runs `init` only on an empty argument list, and the bootstrap forwards a leading flag unchanged.");
        }
        if !a_findings.is_empty() {
            println!("  help: make {}'s verb table list exactly the verbs, and its flag table exactly the flag-verb pairs, that native/src/installer/mod.rs's VERBS and FLAGS carry.", readme);
        }
        return Ok(1);
    }
    let pinned_state = match &pinned {
        Some(p) => format!("pinned set {}", p.label),
        None => "B dormant — the pinned tag does not resolve here".to_string(),
    };
    let mut pending_state = String::new();
    if !pending.is_empty() {
        pending_state.push_str(&format!(", pending release: {}", pending.into_iter().collect::<Vec<_>>().join(" ")));
    }
    if !pending_flags.is_empty() {
        pending_state.push_str(&format!(", pending flags: {}", pending_flags.into_iter().collect::<Vec<_>>().join(", ")));
    }
    println!(
        "FRONT-DOOR-VERBS: clean ({} page(s), {} advertised site(s), {} advertised flag(s), {}, tables equal to VERBS and FLAGS{})",
        pages.len(),
        sites,
        flag_sites,
        pinned_state,
        pending_state
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Tok {
        Tok::Verb(s.to_string(), Vec::new())
    }

    fn vf(s: &str, flags: &[&str]) -> Tok {
        Tok::Verb(s.to_string(), flags.iter().map(|f| f.to_string()).collect())
    }

    // spec: installer/SPEC.md §The front door's verbs — each route, a leading flag, the help flag, a
    // placeholder, and a token outside any code text
    #[test]
    fn the_route_tokenizer_reads_each_route_a_flag_and_a_placeholder() {
        assert_eq!(advertised("curl -fsSL x | sh -s -- demo   # comment"), vec![v("demo")]);
        assert_eq!(advertised("& ([scriptblock]::Create((irm x/install.ps1))) doctor"), vec![v("doctor")]);
        assert_eq!(advertised("npx checkwright init"), vec![v("init")]);
        assert_eq!(advertised("checkwright uninstall"), vec![v("uninstall")]);
        assert_eq!(advertised("sh -s -- --profile full"), vec![Tok::Flag("--profile".to_string())]);
        assert_eq!(advertised("checkwright --help"), vec![Tok::Placeholder]);
        assert_eq!(advertised("checkwright <verb>"), vec![Tok::Placeholder]);
        assert_eq!(advertised("checkwright"), vec![]);
        assert_eq!(advertised("checkwright.lock"), vec![]);
        assert_eq!(advertised("npx checkwright-pwsh init"), vec![]);
        let page = "run npx checkwright demo here, or `npx checkwright init`\n```sh\nsh -s -- diff\ncheckwright doctor\n```\n";
        let found: Vec<(usize, Tok)> = code_texts(page)
            .into_iter()
            .flat_map(|(n, c)| advertised(c).into_iter().map(move |t| (n, t)))
            .collect();
        assert_eq!(found, vec![(1, v("init")), (3, v("diff")), (4, v("doctor"))]);
    }

    // spec: installer/SPEC.md §The front door's verbs — the flags after a verb, cut at `=`, the help
    // arm read as none, and the walk ending at a separator or a comment
    #[test]
    fn the_flag_walk_reads_to_the_first_separator() {
        assert_eq!(
            advertised("sh -s -- init --profile full --recipe=speckit"),
            vec![vf("init", &["--profile", "--recipe"])]
        );
        assert_eq!(advertised("checkwright init --help --dry-run"), vec![vf("init", &["--dry-run"])]);
        assert_eq!(advertised("sh -s -- init --force | tee log --no"), vec![vf("init", &["--force"])]);
        assert_eq!(advertised("checkwright init --force # --no-commit"), vec![vf("init", &["--force"])]);
        assert_eq!(advertised("checkwright init > out --x"), vec![v("init")]);
        assert_eq!(advertised("checkwright init --force && git log --oneline"), vec![vf("init", &["--force"])]);
        assert_eq!(advertised("checkwright <verb> --force"), vec![Tok::Placeholder]);
    }

    // spec: installer/SPEC.md §The front door's verbs — the table whose header's first cell is
    // `verb`, its first-column code spans, and nothing past the table; the flag table's pairs
    #[test]
    fn the_table_readers_take_each_tables_columns() {
        let t = "# x\n\n| verb | asks |\n| --- | --- |\n| `init` | a |\n| `demo` | b |\n\n| `other` | c |\n";
        assert_eq!(verb_table(t), Some(vec!["init".to_string(), "demo".to_string()]));
        assert_eq!(verb_table("| name | x |\n| --- | --- |\n| `a` | b |\n"), None);
        let f = "| flag | verbs | means |\n| --- | --- | --- |\n| `--force` | `init`, `uninstall` | x |\n";
        let pair = |v: &str, f: &str| (v.to_string(), f.to_string());
        assert_eq!(flag_table(f), Some(vec![pair("init", "--force"), pair("uninstall", "--force")]));
        assert_eq!(flag_table(t), None);
    }
}
