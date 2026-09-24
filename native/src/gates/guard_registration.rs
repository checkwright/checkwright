// spec: guard-kit/SPEC.md §check-guard-registration — the rule roster, the crate's rule table and
// every rule citation agree, fail-closed on a roster, table or corpus it cannot read
use crate::diff;
use crate::guard::engine::Shell;
use crate::guard::reader::View;
use crate::programs;
use crate::proc;
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-guard-registration";
const SECTION: &str = "The rule roster";
const CLAUSE: &str = "Declares `";
const SHELLS: &str = "Shells `";
const QUALIFIER: &str = "guard-kit ";

struct Item {
    line: usize,
    name: Option<String>,
    tokens: usize,
    body: String,
}

struct Row {
    name: String,
    views: Vec<View>,
    shells: Vec<Shell>,
}

struct Citation {
    line: usize,
    text: String,
    names: Vec<String>,
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
    let spec = match given(0) {
        Some(s) => s,
        None => format!("{}/SPEC.md", kit_root()?),
    };
    let table = match given(1) {
        Some(f) => table_file(&read(&f)?, &f)?,
        None => compiled(),
    };
    let items = roster(&read(&spec)?, &spec)?;
    let mut findings = 0usize;

    let mut grammar = false;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for it in &items {
        match &it.name {
            None => {
                println!(
                    "{}:{}: roster item carries no `(`<name>`)` token right after its bold title",
                    spec, it.line
                );
                grammar = true;
                findings += 1;
            }
            Some(n) => {
                if it.tokens != 1 {
                    println!(
                        "{}:{}: roster item `{}` carries {} `(`<name>`)` tokens where exactly one is owed",
                        spec, it.line, n, it.tokens
                    );
                    grammar = true;
                    findings += 1;
                }
                if !seen.insert(n) {
                    println!("{}:{}: roster name `{}` is already taken by an earlier item", spec, it.line, n);
                    grammar = true;
                    findings += 1;
                }
            }
        }
    }
    if grammar {
        println!("  help: open each item `- **<title>** (`<name>`) — ` with a distinct name (guard-kit/SPEC.md §The generic ruleset)");
    }

    let mut declaring = false;
    for it in &items {
        let label = it.name.as_deref().unwrap_or("?");
        let clauses = declarations(&it.body, &spec, it.line)?;
        if clauses.len() != 1 {
            println!(
                "{}:{}: roster item `{}` carries {} declaration clause(s) where exactly one is owed",
                spec,
                it.line,
                label,
                clauses.len()
            );
            declaring = true;
            findings += 1;
            continue;
        }
        let named = shells_clauses(&it.body, &spec, it.line)?;
        if named.len() > 1 {
            println!(
                "{}:{}: roster item `{}` carries {} shells clauses where at most one is owed",
                spec,
                it.line,
                label,
                named.len()
            );
            declaring = true;
            findings += 1;
            continue;
        }
        let Some(row) = table.iter().find(|r| Some(r.name.as_str()) == it.name.as_deref()) else {
            continue;
        };
        let applies: BTreeSet<&str> = match named.first() {
            Some(s) => s.iter().map(|s| s.spelling()).collect(),
            None => [Shell::Bash.spelling()].into_iter().collect(),
        };
        let runs: BTreeSet<&str> = row.shells.iter().map(|s| s.spelling()).collect();
        if applies != runs {
            println!(
                "{}:{}: rule `{}` names the shells {} where the table holds {}",
                spec,
                it.line,
                label,
                spelled(&applies),
                spelled(&runs)
            );
            declaring = true;
            findings += 1;
        }
        let declared: BTreeSet<&str> = clauses[0].iter().map(|v| v.spelling()).collect();
        let held: BTreeSet<&str> = row.views.iter().map(|v| v.spelling()).collect();
        if declared != held {
            println!(
                "{}:{}: rule `{}` declares {} where the table holds {}",
                spec,
                it.line,
                label,
                spelled(&declared),
                spelled(&held)
            );
            declaring = true;
            findings += 1;
        }
    }
    if declaring {
        println!("  help: carry one `Declares` clause per item naming exactly the views the rule's table row declares, and a `Shells` clause naming its shells where they are not `bash` alone");
    }

    let listed: Vec<&str> = items.iter().filter_map(|it| it.name.as_deref()).collect();
    let tabled: Vec<&str> = table.iter().map(|r| r.name.as_str()).collect();
    if listed != tabled {
        println!(
            "{}: the roster's names in order differ from the rule table's in dispatch order (< roster, > table):",
            spec
        );
        for l in diff::normal_diff(&listed, &tabled) {
            println!("  {}", l);
        }
        println!("  help: list one item per table row in dispatch order — reorder §{} or the table, and rename both sides together", SECTION);
        findings += 1;
    }

    let names: BTreeSet<&str> = listed.iter().copied().collect();
    let kit_dir = match Path::new(&spec).parent().map(|p| p.to_string_lossy().into_owned()) {
        Some(p) if !p.is_empty() => p,
        _ => ".".to_string(),
    };
    let mut inner = ls(&kit_dir)?;
    let guard_dir = format!("{}/guard", walk::knob_scalar("GATE_SDK_NATIVE_SRC")?.trim_end_matches('/'));
    if Path::new(&guard_dir).is_dir() {
        inner.extend(ls(&guard_dir)?);
    }
    let workflow = walk::knob_scalar("GATE_SDK_WORKFLOW_DIR")?;
    let skipped: BTreeSet<String> = if Path::new(&workflow).is_dir() {
        ls(&workflow)?.into_iter().collect()
    } else {
        BTreeSet::new()
    };
    let prune = walk::prune_dirs()?;
    let mut cited = 0usize;
    let mut citing = false;
    for file in ls(".")? {
        let own = inner.contains(&file);
        if skipped.contains(&file) || (!own && walk::path_pruned(&file, &prune)) {
            continue;
        }
        let Some(text) = corpus_file(&file)? else {
            continue;
        };
        for (n, line) in text.lines().enumerate() {
            if own {
                for num in numbered(line) {
                    println!("{}:{}: a rule cited by number (`{}`) — cite it by name", file, n + 1, num);
                    citing = true;
                    findings += 1;
                }
            }
            for c in citations(line, n + 1, own) {
                cited += 1;
                for name in &c.names {
                    if !names.contains(name.as_str()) {
                        println!(
                            "{}:{}: `{}` names `{}`, which no roster item carries",
                            file, c.line, c.text, name
                        );
                        citing = true;
                        findings += 1;
                    }
                }
            }
        }
    }
    if citing {
        println!("  help: cite a rule as rule `<name>`, qualified as guard-kit rule `<name>` outside guard-kit's tree and the guard module (guard-kit/SPEC.md §The generic ruleset)");
    }

    if findings > 0 {
        return Ok(1);
    }
    println!(
        "GUARD-REGISTRATION: clean ({} rule(s): roster, table order, declared views and {} citation(s) in lockstep)",
        items.len(),
        cited
    );
    Ok(0)
}

// spec: guard-kit/SPEC.md §Testing — the gate reaches guard-kit through the transported kit
// roots, the resolution --run-guard-tests uses
fn kit_root() -> Result<String, String> {
    walk::kit_roots_abs()?
        .into_iter()
        .find(|r| r.rsplit('/').next() == Some("guard-kit"))
        .ok_or_else(|| "the kit roots name no guard-kit root, so the rule roster cannot be found".to_string())
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

// spec: guard-kit/SPEC.md §check-guard-registration — a tracked path the working tree no longer
// holds, or a directory entry, carries no citation; any other failure to read is a refusal
fn corpus_file(path: &str) -> Result<Option<String>, String> {
    let p = Path::new(path);
    if p.is_dir() {
        return Ok(None);
    }
    match std::fs::read(p) {
        Ok(b) => Ok(Some(String::from_utf8_lossy(&b).into_owned())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("cannot read {}: {}", path, e)),
    }
}

fn ls(pathspec: &str) -> Result<Vec<String>, String> {
    let out = proc::run(&programs::GIT, &["ls-files", "--", pathspec])?;
    match out.stdout() {
        Some(o) => Ok(String::from_utf8_lossy(o).lines().filter(|l| !l.is_empty()).map(String::from).collect()),
        None => Err(format!(
            "git ls-files -- {} exited {} — the citation corpus could not be enumerated",
            pathspec,
            out.code().unwrap_or(-1)
        )),
    }
}

fn compiled() -> Vec<Row> {
    crate::guard::rule_table()
        .iter()
        .map(|r| Row { name: r.name.to_string(), views: r.views.to_vec(), shells: r.shells.to_vec() })
        .collect()
}

fn is_name(s: &str) -> bool {
    let mut c = s.chars();
    matches!(c.next(), Some('a'..='z')) && c.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

fn spelled(views: &BTreeSet<&str>) -> String {
    let v: Vec<String> = views.iter().map(|s| format!("`{}`", s)).collect();
    if v.is_empty() {
        "nothing".to_string()
    } else {
        v.join(", ")
    }
}

// spec: guard-kit/SPEC.md §check-guard-registration — `<name><TAB><view>[; <view>…][<TAB><shell>[,
// <shell>…]]`, an absent third field read as `bash`, a blank or `#` line skipped, any other line a
// refusal
fn table_file(text: &str, path: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let bad = || format!("{}:{}: a table line of no known shape ({}) — the gate reads `<name><TAB><view>[; <view>…][<TAB><shell>[, <shell>…]]`", path, n + 1, line);
        let (name, rest) = line.split_once('\t').ok_or_else(bad)?;
        let (views, shells) = match rest.split_once('\t') {
            Some((v, s)) => (v, Some(s)),
            None => (rest, None),
        };
        if !is_name(name) || shells.is_some_and(|s| s.contains('\t')) {
            return Err(bad());
        }
        let views = views
            .split("; ")
            .map(View::from_spelling)
            .collect::<Option<Vec<View>>>()
            .ok_or_else(bad)?;
        let shells = match shells {
            None => vec![Shell::Bash],
            Some(s) => s.split(", ").map(Shell::from_spelling).collect::<Option<Vec<Shell>>>().ok_or_else(bad)?,
        };
        rows.push(Row { name: name.to_string(), views, shells });
    }
    Ok(rows)
}

fn heading(line: &str) -> Option<&str> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    line[hashes..].strip_prefix(' ').map(str::trim)
}

// spec: guard-kit/SPEC.md §check-guard-registration — the `(`<name>`)` tokens an item carries
// right after a bold title, and the name its lead line opens with
fn name_tokens(body: &str) -> (Option<String>, usize) {
    let mut count = 0usize;
    let mut from = 0usize;
    while let Some(off) = body[from..].find("** (`") {
        let at = from + off + "** (`".len();
        if let Some(end) = body[at..].find("`)") {
            if is_name(&body[at..at + end]) {
                count += 1;
            }
        }
        from = at;
    }
    let lead = body.lines().next().unwrap_or("");
    let name = lead.strip_prefix("- **").and_then(|rest| {
        let close = rest.find("** (`")?;
        let after = &rest[close + "** (`".len()..];
        let end = after.find("`)")?;
        let n = &after[..end];
        (is_name(n) && after[end..].starts_with("`) — ")).then(|| n.to_string())
    });
    (name, count)
}

fn roster(text: &str, path: &str) -> Result<Vec<Item>, String> {
    let mut inside = false;
    let mut found = false;
    let mut fence = false;
    let mut items: Vec<Item> = Vec::new();
    let mut open: Option<(usize, String)> = None;
    let close = |(line, body): (usize, String)| {
        let (name, tokens) = name_tokens(&body);
        Item { line, name, tokens, body }
    };
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
        if line.starts_with("- ") {
            if let Some(o) = open.take() {
                items.push(close(o));
            }
            open = Some((n + 1, line.to_string()));
            continue;
        }
        let continues = line.trim().is_empty() || line.starts_with(|c: char| c.is_whitespace());
        match open.take() {
            Some((at, mut body)) if continues => {
                body.push('\n');
                body.push_str(line);
                open = Some((at, body));
            }
            Some(o) => items.push(close(o)),
            None => {}
        }
    }
    if let Some(o) = open.take() {
        items.push(close(o));
    }
    if !found {
        return Err(format!("{}: no \"{}\" section heading — the roster is unreadable", path, SECTION));
    }
    if items.is_empty() {
        return Err(format!("{}: §{} carries no `- ` item — the roster is unreadable", path, SECTION));
    }
    Ok(items)
}

// spec: guard-kit/SPEC.md §The generic ruleset — a backticked list after `at`: names or views joined
// by `, `, ` and ` or `, and `, as the raw spans, with the byte offset the list ends at
fn backticked_list(s: &str) -> (Vec<&str>, usize) {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(rest) = s[at..].strip_prefix('`') {
        let Some(end) = rest.find('`') else {
            break;
        };
        out.push(&rest[..end]);
        at += end + 2;
        match [", and `", ", `", " and `"].iter().find(|sep| s[at..].starts_with(**sep)) {
            Some(sep) => at += sep.len() - 1,
            None => break,
        }
    }
    (out, at)
}

// spec: guard-kit/SPEC.md §check-guard-registration — every declaration clause in an item, each
// parsed to its views; a clause the grammar cannot read is a refusal
fn declarations(body: &str, path: &str, line: usize) -> Result<Vec<Vec<View>>, String> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(off) = body[from..].find(CLAUSE) {
        let at = from + off + CLAUSE.len() - 1;
        let (spans, end) = backticked_list(&body[at..]);
        let unreadable = || {
            format!(
                "{}:{}: a declaration clause the grammar cannot read ({}) — the gate reads `Declares` and backticked views joined by `, ` or ` and `",
                path,
                line,
                body[at + 1 - CLAUSE.len()..(at + end.max(1)).min(body.len())].trim()
            )
        };
        if spans.is_empty() {
            return Err(unreadable());
        }
        let views = spans.iter().map(|s| View::from_spelling(s)).collect::<Option<Vec<View>>>().ok_or_else(unreadable)?;
        out.push(views);
        from = at + end.max(1);
    }
    Ok(out)
}

// spec: guard-kit/SPEC.md §The generic ruleset — every shells clause in an item, each parsed to its
// shells; a clause the grammar cannot read, or a shell the crate does not carry, is a refusal
fn shells_clauses(body: &str, path: &str, line: usize) -> Result<Vec<Vec<Shell>>, String> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(off) = body[from..].find(SHELLS) {
        let at = from + off + SHELLS.len() - 1;
        let (spans, end) = backticked_list(&body[at..]);
        let unreadable = || {
            format!(
                "{}:{}: a shells clause the grammar cannot read ({}) — the gate reads `Shells` and backticked shells (`bash`, `powershell`) joined by `, ` or ` and `",
                path,
                line,
                body[at + 1 - SHELLS.len()..(at + end.max(1)).min(body.len())].trim()
            )
        };
        if spans.is_empty() {
            return Err(unreadable());
        }
        out.push(spans.iter().map(|s| Shell::from_spelling(s)).collect::<Option<Vec<Shell>>>().ok_or_else(unreadable)?);
        from = at + end.max(1);
    }
    Ok(out)
}

// spec: guard-kit/SPEC.md §check-guard-registration — every `[Rr]ules?[ -][0-9]+` token on a line
fn numbered(line: &str) -> Vec<String> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(off) = line[i..].find("ule") {
        let at = i + off;
        i = at + 3;
        if at == 0 || !matches!(b[at - 1], b'r' | b'R') {
            continue;
        }
        let mut j = at + 3;
        if b.get(j) == Some(&b's') {
            j += 1;
        }
        if !matches!(b.get(j), Some(b' ' | b'-')) {
            continue;
        }
        let digits = b[j + 1..].iter().take_while(|c| c.is_ascii_digit()).count();
        if digits > 0 {
            out.push(line[at - 1..j + 1 + digits].to_string());
        }
    }
    out
}

// spec: guard-kit/SPEC.md §check-guard-registration — the citations a line carries: the word `rule`
// or `rules` at a word boundary, then a backticked name list; outside guard-kit's own tree only the
// qualified form is read
fn citations(line: &str, n: usize, own: bool) -> Vec<Citation> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(off) = line[i..].find("ule") {
        let at = i + off;
        i = at + 3;
        if at == 0 || !matches!(b[at - 1], b'r' | b'R') {
            continue;
        }
        let start = at - 1;
        if start > 0 && (b[start - 1].is_ascii_alphanumeric() || b[start - 1] == b'_' || b[start - 1] == b'-') {
            continue;
        }
        let qualified = line[..start].ends_with(QUALIFIER);
        if !own && !qualified {
            continue;
        }
        let mut j = at + 3;
        if b.get(j) == Some(&b's') {
            j += 1;
        }
        let Some(rest) = line[j..].strip_prefix(' ') else {
            continue;
        };
        let (spans, end) = backticked_list(rest);
        let names: Vec<String> = spans.iter().take_while(|s| is_name(s)).map(|s| s.to_string()).collect();
        if names.is_empty() {
            continue;
        }
        let from = if qualified { start - QUALIFIER.len() } else { start };
        out.push(Citation { line: n, text: line[from..j + 1 + end].to_string(), names });
        i = j + 1 + end;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_item_ends_at_the_first_unindented_non_blank_line() {
        let spec = "## The rule roster\n\n- **a** (`alpha`) — Declares `raw`. x\n  more\n\n  - **(a) arm**: y\n\nProse (`beta`).\n- **b** (`beta`) — Declares `sq`.\n## Next\n- **c** (`gamma`) — z\n";
        let items = roster(spec, "s").unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name.as_deref(), Some("alpha"));
        assert_eq!(items[0].tokens, 1);
        assert_eq!(items[1].name.as_deref(), Some("beta"));
    }

    #[test]
    fn a_lead_needs_its_bold_title_its_name_and_the_dash() {
        assert_eq!(name_tokens("- **t** (`cd_compound`) — x").0.as_deref(), Some("cd_compound"));
        assert_eq!(name_tokens("- **t** (`Cd`) — x").0, None);
        assert_eq!(name_tokens("- **t** — (`cd`) x").0, None);
        assert_eq!(name_tokens("- **t** (`a`) — x **u** (`b`)").1, 2);
    }

    #[test]
    fn a_clause_reads_views_joined_by_comma_and_and() {
        let d = declarations("x Declares `raw`, `sq dq hd` and `dequoted`. y", "s", 1).unwrap();
        assert_eq!(d, vec![vec![View::Raw, View::SqDqHd, View::Dequoted]]);
        assert_eq!(declarations("no clause, and it declares `raw`", "s", 1).unwrap().len(), 0);
        assert!(declarations("Declares `dq`.", "s", 1).is_err());
        assert!(declarations("Declares `raw", "s", 1).is_err());
    }

    #[test]
    fn a_citation_is_bounded_and_outside_the_kit_qualified() {
        let c = citations("rules `a`, `b` and `c`'s test, and rule `x` (see prules `z`)", 1, true);
        assert_eq!(c.iter().map(|c| c.names.clone()).collect::<Vec<_>>(), vec![vec!["a", "b", "c"], vec!["x"]]);
        assert!(citations("a rule `Bash(x)` and rule `sq dq`", 1, true).is_empty());
        assert!(citations("rule `cd_compound`", 1, false).is_empty());
        let q = citations("guard-kit rule `bounded_wait`'s arm (B)", 1, false);
        assert_eq!(q[0].text, "guard-kit rule `bounded_wait`");
    }

    #[test]
    fn a_number_is_read_after_either_case_and_number() {
        assert_eq!(numbered("Rule 12 and rules 3, 4; ruleset 2; rule x; rule-22"), vec!["Rule 12", "rules 3", "rule-22"]);
    }

    #[test]
    fn a_table_line_is_a_name_a_tab_and_known_views() {
        let t = table_file("# c\n\nalpha\traw; sq dq hd\n", "t").unwrap();
        assert_eq!(t[0].views, vec![View::Raw, View::SqDqHd]);
        assert!(table_file("alpha raw\n", "t").is_err());
        assert!(table_file("alpha\tdq\n", "t").is_err());
        assert!(table_file("Alpha\traw\n", "t").is_err());
        assert_eq!(t[0].shells, vec![Shell::Bash]);
        let s = table_file("alpha\traw\tbash, powershell\n", "t").unwrap();
        assert_eq!(s[0].shells, vec![Shell::Bash, Shell::PowerShell]);
        assert!(table_file("alpha\traw\tzsh\n", "t").is_err());
        assert!(table_file("alpha\traw\tbash\tx\n", "t").is_err());
    }

    #[test]
    fn a_shells_clause_reads_the_crate_s_shells_or_refuses() {
        let s = shells_clauses("Declares `raw`. Shells `bash` and `powershell`. y", "s", 1).unwrap();
        assert_eq!(s, vec![vec![Shell::Bash, Shell::PowerShell]]);
        assert!(shells_clauses("Declares `raw`.", "s", 1).unwrap().is_empty());
        assert!(shells_clauses("Shells `zsh`.", "s", 1).is_err());
        assert!(shells_clauses("Shells `bash", "s", 1).is_err());
    }
}
