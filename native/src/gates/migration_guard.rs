// spec: gate-sdk/SPEC.md §check-migration-guard — a branch only appends migrations, in order, and
// none it adds is destructive unless declared
use crate::ere::Ere;
use crate::exit::{self, Fault};
use crate::since_base;
use crate::{listing, proc, programs, walk};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const NAME: &str = "check-migration-guard";
const DIR_KNOB: &str = "GATE_SDK_MIGRATION_DIR";
const LIST_KNOB: &str = "GATE_SDK_MIGRATION_DESTRUCTIVE";
const REFS_KNOB: &str = "GATE_SDK_MIGRATION_REFS";
const USAGE: &str = "usage: check-migration-guard [--base <ref> | --base-dir <dir>]";
const WAIVER: &str = "migration-ok:";

type Files = BTreeMap<String, u64>;

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            let fault = fault.prefixed(NAME);
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

// spec: gate-sdk/SPEC.md §check-migration-guard — `--base-dir` replaces the merge base, so the two
// never ride together
enum Against {
    Base(Option<String>),
    Dir(String),
}

fn parse_args(args: &[String]) -> Result<Against, Fault> {
    let (mut base, mut dir): (Option<String>, Option<String>) = (None, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let slot = match a.as_str() {
            "--base" => &mut base,
            "--base-dir" => &mut dir,
            _ => return Err(Fault::usage(format!("unknown argument '{}' — {}", a, USAGE))),
        };
        let value = it.next().filter(|v| !v.is_empty()).ok_or_else(|| Fault::usage(format!("{} takes a value — {}", a, USAGE)))?;
        if slot.replace(value.clone()).is_some() {
            return Err(Fault::usage(format!("{} given twice — {}", a, USAGE)));
        }
    }
    match (base, dir) {
        (Some(_), Some(_)) => Err(Fault::usage(format!("--base-dir stands in for the merge base, so it takes no --base — {}", USAGE))),
        (base, None) => Ok(Against::Base(base)),
        (None, Some(dir)) => Ok(Against::Dir(dir)),
    }
}

// spec: gate-sdk/SPEC.md §check-migration-guard — `<digits>_<name>.sql`, its number the decimal
// value of `<digits>`, refused past 64 bits
fn number(name: &str) -> Option<u64> {
    let digits = spelled(name);
    let stem = name[digits.len()..].strip_prefix('_')?.strip_suffix(".sql")?;
    if digits.is_empty() || stem.is_empty() || stem.contains('/') {
        return None;
    }
    digits.parse().ok()
}

fn spelled(name: &str) -> &str {
    &name[..name.bytes().take_while(u8::is_ascii_digit).count()]
}

fn files(dir: &str, names: impl IntoIterator<Item = String>) -> Result<Files, Fault> {
    names
        .into_iter()
        .map(|n| match number(&n) {
            Some(k) => Ok((n, k)),
            None => Err(Fault::input(format!(
                "{}/{}: not a migration file — every file there is named <digits>_<name>.sql, its number under 64 bits",
                dir, n
            ))),
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §check-migration-guard — every file at the merge base and in the working
// tree shares the width of the first by name, so name order is number order
fn shared_width(sets: &[(&str, &Files)]) -> Result<(), Fault> {
    let mut all: Vec<(&str, &str)> = sets.iter().flat_map(|(d, f)| f.keys().map(move |n| (n.as_str(), *d))).collect();
    all.sort();
    let Some((first, _)) = all.first().copied() else {
        return Ok(());
    };
    let width = spelled(first).len();
    match all.iter().find(|(n, _)| spelled(n).len() != width) {
        Some((n, d)) => Err(Fault::input(format!(
            "{}/{}: its number is {} digit(s) wide where the first file by name, {}, is {} — every file there shares one width",
            d,
            n,
            spelled(n).len(),
            first,
            width
        ))),
        None => Ok(()),
    }
}

// spec: gate-sdk/SPEC.md §check-migration-guard — a tree this commit does not make: its names that
// are no migration name are skipped, and they join no width check
fn readable(names: impl IntoIterator<Item = String>) -> Files {
    names.into_iter().filter_map(|n| number(&n).map(|k| (n, k))).collect()
}

// spec: gate-sdk/SPEC.md §check-migration-guard — the branch's additions: new since the merge base,
// less any the base ref's own commit already holds by name
fn added(base: &Files, work: &Files, tip: &Files) -> Files {
    work.iter().filter(|(n, _)| !base.contains_key(*n) && !tip.contains_key(*n)).map(|(n, k)| (n.clone(), *k)).collect()
}

// spec: gate-sdk/SPEC.md §check-migration-guard — a merge-base file gone or changed is edited; each
// addition carries the first of ORDER against both base commits, DUP, then CLASH with each ref
fn judge(dir: &str, base: &Files, work: &Files, edited: &BTreeSet<String>, tip: &Files, refs: &[(String, Files)]) -> Vec<String> {
    let mut out: Vec<String> = base
        .keys()
        .filter(|n| !work.contains_key(*n) || edited.contains(*n))
        .map(|n| format!("{}/{}: [MIGRATION-EDITED]", dir, n))
        .collect();
    let highest = base.iter().chain(tip).max_by_key(|(_, k)| **k);
    let mut first: BTreeMap<u64, String> = BTreeMap::new();
    for (n, k) in &added(base, work, tip) {
        if let Some((top, _)) = highest.filter(|(_, h)| k <= *h) {
            out.push(format!("{}/{}: [MIGRATION-ORDER] {} <= {}", dir, n, spelled(n), spelled(top)));
        } else if let Some(f) = first.get(k) {
            out.push(format!("{}/{}: [MIGRATION-DUP] {} also {}", dir, n, spelled(n), f));
        } else {
            first.insert(*k, n.clone());
            for (r, theirs) in refs {
                if let Some((t, _)) = theirs.iter().find(|(t, j)| *j == k && *t != n) {
                    out.push(format!("{}/{}: [MIGRATION-CLASH] {} {} {}", dir, n, spelled(n), r, t));
                }
            }
        }
    }
    out
}

fn git(top: &str, args: &[&str]) -> Result<proc::Completed, Fault> {
    let mut argv = vec!["-C", top];
    argv.extend(args);
    proc::run(&programs::GIT, &argv).map_err(Fault::tool)
}

fn git_failed(c: &proc::Completed, what: &str) -> Fault {
    Fault::tool(c.failure_report().map_or_else(|| format!("{} failed", what), |r| format!("{} failed: {}", what, r)))
}

// spec: gate-sdk/SPEC.md §check-migration-guard — the ref set: each ref the globs match that the base
// ref's commit lacks, less `HEAD`'s own, an orphan or a shallow cut skipped; a ref's additions are
// its directory's files its merge base with the base ref lacks
fn ref_additions(b: &since_base::Base, dir_top: &str, globs: &[String]) -> Result<Vec<(String, Files)>, Fault> {
    let head = git(&b.top, &["symbolic-ref", "-q", "HEAD"])?;
    let own = match (head.stdout(), head.code()) {
        (Some(o), _) => Some(String::from_utf8_lossy(o).trim().to_string()),
        (None, Some(1)) => None,
        (None, _) => return Err(git_failed(&head, "git symbolic-ref HEAD")),
    };
    let mut args = vec!["for-each-ref", "--no-merged", &b.tip, "--format=%(refname)"];
    args.extend(globs.iter().map(String::as_str));
    let listed = git(&b.top, &args)?;
    let refs = listed.stdout().ok_or_else(|| git_failed(&listed, "git for-each-ref"))?;
    let mut out = Vec::new();
    for r in String::from_utf8_lossy(refs).lines().filter(|r| !r.is_empty() && Some(*r) != own.as_deref()) {
        let mb = git(&b.top, &["merge-base", r, &b.tip])?;
        let mb = match (mb.stdout(), mb.code()) {
            (Some(o), _) => String::from_utf8_lossy(o).trim().to_string(),
            (None, Some(1)) => continue,
            (None, _) => return Err(git_failed(&mb, &format!("git merge-base {}", r))),
        };
        let names = listing::names(Some(&b.top), &["diff", "--name-only", "-z", "--no-renames", "--diff-filter=A", &mb, r, "--", dir_top])
            .map_err(|e| Fault::tool(e.text()))?;
        out.push((r.to_string(), readable(under(names, dir_top))));
    }
    Ok(out)
}

#[derive(Debug, PartialEq)]
struct Statement {
    first: usize,
    last: usize,
    text: String,
}

// spec: gate-sdk/SPEC.md §check-migration-guard — statements cut at `;`, comments and string
// literals read as a space, a quoted identifier as its name, whitespace runs as one, upper-cased;
// each `-- migration-ok:` reason with its line
fn statements(sql: &str) -> (Vec<Statement>, Vec<(usize, String)>) {
    let (mut out, mut waivers) = (Vec::new(), Vec::new());
    let (mut text, mut first, mut line) = (String::new(), None, 1);
    let mut chars = sql.chars().peekable();
    let space = |text: &mut String| {
        if !text.is_empty() && !text.ends_with(' ') {
            text.push(' ');
        }
    };
    while let Some(c) = chars.next() {
        match c {
            '-' if chars.peek() == Some(&'-') => {
                chars.next();
                let comment: String = std::iter::from_fn(|| chars.next_if(|&c| c != '\n')).collect();
                if let Some(reason) = comment.trim_start().strip_prefix(WAIVER) {
                    waivers.push((line, reason.trim().to_string()));
                }
                space(&mut text);
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut prev = ' ';
                for c in chars.by_ref() {
                    line += usize::from(c == '\n');
                    if prev == '*' && c == '/' {
                        break;
                    }
                    prev = c;
                }
                space(&mut text);
            }
            '\'' => {
                first.get_or_insert(line);
                while let Some(q) = chars.next() {
                    line += usize::from(q == '\n');
                    if q == c && chars.next_if_eq(&c).is_none() {
                        break;
                    }
                }
                space(&mut text);
            }
            '"' => {
                first.get_or_insert(line);
                while let Some(q) = chars.next() {
                    if q == '"' && chars.next_if_eq(&'"').is_none() {
                        break;
                    }
                    if q.is_whitespace() {
                        line += usize::from(q == '\n');
                        space(&mut text);
                    } else {
                        text.push(q);
                    }
                }
            }
            ';' => {
                if let Some(f) = first.take() {
                    out.push(Statement { first: f, last: line, text: text.trim_end().to_ascii_uppercase() });
                }
                text.clear();
            }
            c if c.is_whitespace() => {
                line += usize::from(c == '\n');
                space(&mut text);
            }
            c => {
                first.get_or_insert(line);
                text.push(c);
            }
        }
    }
    if let Some(f) = first {
        out.push(Statement { first: f, last: line, text: text.trim_end().to_ascii_uppercase() });
    }
    (out, waivers)
}

// spec: gate-sdk/SPEC.md §check-migration-guard — a statement matching a live ERE reds at its first
// line unless a reasoned declaration sits on one of its lines
fn destructive(path: &str, sql: &str, list: &[(String, Ere)]) -> Vec<String> {
    let (stmts, waivers) = statements(sql);
    stmts
        .iter()
        .filter_map(|s| {
            let (pattern, _) = list.iter().find(|(_, ere)| ere.is_match(&s.text))?;
            let reasons: Vec<&String> = waivers.iter().filter(|(l, _)| *l >= s.first && *l <= s.last).map(|(_, r)| r).collect();
            if reasons.iter().any(|r| !r.is_empty()) {
                return None;
            }
            let tail = if reasons.is_empty() { "" } else { ", its waiver reason empty" };
            Some(format!("{}:{}: [MIGRATION-DESTRUCTIVE] matches '{}'{}", path, s.first, pattern, tail))
        })
        .collect()
}

fn read_list(path: &str) -> Result<Option<Vec<(String, Ere)>>, Fault> {
    let Some(text) = exit::read_optional(path, "destructive list")? else {
        return Ok(None);
    };
    let mut out = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let pattern = raw.trim();
        if pattern.is_empty() || pattern.starts_with('#') {
            continue;
        }
        let ere = Ere::compile(pattern).map_err(|e| Fault::refused(format!("{}:{}: '{}' does not compile: {}", path, i + 1, pattern, e)))?;
        out.push((pattern.to_string(), ere));
    }
    Ok(Some(out))
}

fn prefix() -> Result<String, Fault> {
    let c = proc::run(&programs::GIT, &["rev-parse", "--show-prefix"]).map_err(Fault::tool)?;
    let out = c.stdout().ok_or_else(|| Fault::tool(c.failure_report().unwrap_or_else(|| "git rev-parse --show-prefix failed".to_string())))?;
    Ok(String::from_utf8_lossy(out).trim().to_string())
}

fn under(paths: Vec<String>, dir_top: &str) -> Vec<String> {
    paths.into_iter().map(|p| walk::rel_under(dir_top, &p).map_or_else(|| p.clone(), str::to_string)).collect()
}

// spec: gate-sdk/SPEC.md §check-migration-guard — the working tree's files: tracked ones still on
// disk and untracked ones git does not ignore
fn working(top: &str, dir_top: &str) -> Result<Vec<String>, Fault> {
    let listed = listing::names(Some(top), &["ls-files", "-z", "-c", "-o", "--exclude-standard", "--", dir_top])
        .map_err(|e| Fault::tool(e.text()))?;
    let on_disk: BTreeSet<String> = listed.into_iter().filter(|p| Path::new(top).join(p).is_file()).collect();
    Ok(under(on_disk.into_iter().collect(), dir_top))
}

fn read(path: &Path) -> Result<Vec<u8>, Fault> {
    std::fs::read(path).map_err(|e| Fault::input(format!("cannot read {}: {}", path.display(), e)))
}

// spec: gate-sdk/SPEC.md §check-migration-guard — the pair's stand-in: its files compared byte for byte
fn from_dir(base_dir: &str, dir: &str, work: &Files) -> Result<(Files, BTreeSet<String>), Fault> {
    if !Path::new(base_dir).is_dir() {
        return Err(Fault::refused(format!("--base-dir {} is not a directory", base_dir)));
    }
    let names = walk::list_dir(Path::new(base_dir)).map_err(Fault::input)?;
    let base = files(base_dir, names.into_iter().filter(|(_, is_dir)| !is_dir).map(|(n, _)| n))?;
    shared_width(&[(base_dir, &base), (dir, work)])?;
    let mut edited = BTreeSet::new();
    for n in base.keys().filter(|n| work.contains_key(*n)) {
        if read(&Path::new(base_dir).join(n))? != read(&Path::new(dir).join(n))? {
            edited.insert(n.clone());
        }
    }
    Ok((base, edited))
}

// spec: gate-sdk/SPEC.md §check-migration-guard — every attribute git's checkout conversion reads
const CONVERSION: &[&str] = &["text", "crlf", "eol", "ident", "filter", "working-tree-encoding"];

type Attrs = BTreeMap<String, Vec<(String, String)>>;

// spec: gate-sdk/SPEC.md §check-migration-guard — each path's conversion attributes, from the working
// tree's attribute files or, with `index` set, from that index's alone
fn attrs(top: &str, paths: &[String], index: Option<&Path>) -> Result<Attrs, Fault> {
    let mut out: Attrs = BTreeMap::new();
    for chunk in paths.chunks(128) {
        let mut argv = vec!["-C", top, "check-attr", "-z"];
        if index.is_some() {
            argv.push("--cached");
        }
        argv.extend(CONVERSION);
        argv.push("--");
        argv.extend(chunk.iter().map(String::as_str));
        let env: Vec<(String, String)> = index.map(|i| ("GIT_INDEX_FILE".to_string(), i.display().to_string())).into_iter().collect();
        let c = proc::run_with_env(&programs::GIT, &argv, &env).map_err(Fault::tool)?;
        let raw = c.stdout().ok_or_else(|| git_failed(&c, "git check-attr"))?;
        let fields: Vec<String> = raw.split(|&b| b == 0).map(|f| String::from_utf8_lossy(f).into_owned()).collect();
        for r in fields.chunks_exact(3) {
            out.entry(r[0].clone()).or_default().push((r[1].clone(), r[2].clone()));
        }
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §check-migration-guard — the driver names some `filter.<name>.smudge`,
// `.clean` or `.process` configures
fn drivers(top: &str) -> Result<BTreeSet<String>, Fault> {
    let c = git(top, &["config", "-z", "--get-regexp", r"^filter\..*\.(smudge|clean|process)$"])?;
    let raw = match (c.stdout(), c.code()) {
        (Some(o), _) => o.to_vec(),
        (None, Some(1)) => Vec::new(),
        (None, _) => return Err(git_failed(&c, "git config --get-regexp")),
    };
    Ok(driver_names(&raw))
}

// spec: gate-sdk/SPEC.md §check-migration-guard — `git config -z` records, `<key>\n<value>`, read as
// the `<name>` of each `filter.<name>.<var>` key
fn driver_names(raw: &[u8]) -> BTreeSet<String> {
    raw.split(|&b| b == 0)
        .filter_map(|rec| {
            let rec = String::from_utf8_lossy(rec);
            let key = rec.split('\n').next()?.to_string();
            let (name, _) = key.strip_prefix("filter.")?.rsplit_once('.')?;
            Some(name.to_string())
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §check-migration-guard — the merge base's conversion attributes, read off a
// throwaway index of its tree in this process's private temp dir, removed before return
fn attrs_at_base(b: &since_base::Base, paths: &[String]) -> Result<Attrs, Fault> {
    let mut index = proc::temp_base().map_err(Fault::tool)?.join(format!("migration-base-index.{}", std::process::id()));
    if index.is_relative() {
        index = Path::new(&walk::cwd().map_err(Fault::tool)?).join(index);
    }
    let _ = std::fs::remove_file(&index);
    let env = [("GIT_INDEX_FILE".to_string(), index.display().to_string())];
    let read = proc::run_with_env(&programs::GIT, &["-C", &b.top, "read-tree", &b.commit], &env).map_err(Fault::tool);
    let got = read.and_then(|c| match c.stdout() {
        Some(_) => attrs(&b.top, paths, Some(&index)),
        None => Err(git_failed(&c, "git read-tree of the merge base")),
    });
    let _ = std::fs::remove_file(&index);
    got
}

// spec: gate-sdk/SPEC.md §check-migration-guard — the git arm's edited set, a merge-base file whose
// bytes differ from its blob's checkout form, and its attribute drift; a missing driver is exit 3
fn checkout_edits(b: &since_base::Base, dir: &str, dir_top: &str, base: &Files, work: &Files) -> Result<(BTreeSet<String>, Vec<String>), Fault> {
    let kept: Vec<&String> = base.keys().filter(|n| work.contains_key(*n)).collect();
    if kept.is_empty() {
        return Ok((BTreeSet::new(), Vec::new()));
    }
    let paths: Vec<String> = kept.iter().map(|n| format!("{}/{}", dir_top, n)).collect();
    let now = attrs(&b.top, &paths, None)?;
    let configured = drivers(&b.top)?;
    for (n, p) in kept.iter().zip(&paths) {
        let named = now.get(p).into_iter().flatten().find(|(a, v)| a == "filter" && !matches!(v.as_str(), "unspecified" | "unset" | "set"));
        if let Some((_, driver)) = named.filter(|(_, v)| !configured.contains(v)) {
            return Err(Fault::tool(format!(
                "{}/{}: its filter attribute names driver '{}', which no filter.{}.smudge, .clean or .process configures, so its checkout form cannot be read — install the driver",
                dir, n, driver, driver
            )));
        }
    }
    let mut edited = BTreeSet::new();
    for (n, p) in kept.iter().zip(&paths) {
        let c = git(&b.top, &["cat-file", "--filters", &format!("{}:{}", b.commit, p)])?;
        let form = c.stdout().ok_or_else(|| git_failed(&c, &format!("git cat-file --filters of {}", p)))?;
        if read(&Path::new(&b.top).join(p))? != form {
            edited.insert((*n).clone());
        }
    }
    let then = attrs_at_base(b, &paths)?;
    let mut drift = Vec::new();
    for (n, p) in kept.iter().zip(&paths) {
        let (was, is) = (then.get(p).cloned().unwrap_or_default(), now.get(p).cloned().unwrap_or_default());
        for (attr, value) in &is {
            let before = was.iter().find(|(a, _)| a == attr).map_or("unspecified", |(_, v)| v.as_str());
            if before != value {
                drift.push(format!("{}/{}: [MIGRATION-ATTR] {} {} -> {}", dir, n, attr, before, value));
            }
        }
    }
    Ok((edited, drift))
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    let against = parse_args(args)?;
    let dir = walk::knob_scalar(DIR_KNOB).map_err(Fault::refused)?;
    let dir = dir.trim_start_matches("./").trim_end_matches('/').to_string();
    if dir.is_empty() {
        println!("MIGRATION-GUARD: clean (not applicable: no migration directory configured)");
        return Ok(0);
    }
    let list_path = walk::knob_scalar(LIST_KNOB).map_err(Fault::refused)?;
    let list = read_list(&list_path)?;
    let top = walk::toplevel_opt()
        .map_err(Fault::tool)?
        .ok_or_else(|| Fault::refused("not a git repository — there is no working tree to read migrations from"))?;
    let dir_top = format!("{}{}", prefix()?, dir);
    let work = files(&dir, working(&top, &dir_top)?)?;

    let (base, edited, drift, tip, refs, what, clash) = match against {
        Against::Base(flag) => {
            let b = since_base::resolve(flag.as_deref())?;
            let base = files(&dir, under(b.files_at_base(&dir_top)?, &dir_top))?;
            shared_width(&[(&dir, &base), (&dir, &work)])?;
            let (edited, drift) = checkout_edits(&b, &dir, &dir_top, &base, &work)?;
            let tip = readable(under(b.files_at_tip(&dir_top)?, &dir_top));
            let globs = walk::knob_words(REFS_KNOB).map_err(Fault::refused)?;
            let (refs, clash) = if globs == ["none"] {
                (Vec::new(), format!("; clash check off: {} is none", REFS_KNOB))
            } else {
                let refs = ref_additions(&b, &dir_top, &globs)?;
                let read = format!(", {} other ref(s) read", refs.len());
                (refs, read)
            };
            (base, edited, drift, tip, refs, format!("merge base {}", b.short()), clash)
        }
        Against::Dir(d) => {
            let (base, edited) = from_dir(&d, &dir, &work)?;
            (base, edited, Vec::new(), Files::new(), Vec::new(), d, "; clash check off: no base ref under --base-dir".to_string())
        }
    };

    let mut out = judge(&dir, &base, &work, &edited, &tip, &refs);
    out.extend(drift);
    let added = added(&base, &work, &tip);
    if let Some(list) = &list {
        for n in added.keys() {
            let path = format!("{}/{}", dir, n);
            let sql = std::fs::read_to_string(&path).map_err(|e| Fault::input(format!("cannot read {}: {}", path, e)))?;
            out.extend(destructive(&path, &sql, list));
        }
    }
    out.sort();

    if out.is_empty() {
        let off = if list.is_none() { format!("; destructive check off: no list at {}", list_path) } else { String::new() };
        println!("MIGRATION-GUARD: clean ({} migration(s), {} added, against {}{}{})", work.len(), added.len(), what, clash, off);
        return Ok(0);
    }
    println!(
        "{}: migration(s) under {} edited, out of order, duplicated, clashing, destructive or re-attributed against {}:",
        NAME, dir, what
    );
    for r in &out {
        println!("{}", r);
    }
    println!("  help: a merged migration is history: restore an edited or deleted one and put the change in a");
    println!("        new file numbered above the highest; renumber a duplicate or a clash above every other");
    println!("        branch's; declare a deliberate destructive statement with '-- {} <reason>' on one of", WAIVER);
    println!("        its lines; restore a merged migration's conversion attributes (.gitattributes) to the");
    println!("        merge base's, since they change the bytes a checkout writes.");
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-migration-guard — digits of any width, an underscore, a name and
    // `.sql`, the number fitting 64 bits
    #[test]
    fn a_migration_name_carries_its_number() {
        assert_eq!(number("0042_add_users.sql"), Some(42));
        assert_eq!(number("42_add.sql"), Some(42));
        assert_eq!(number("20261126120000_x.sql"), Some(20261126120000));
        for stray in ["0042add.sql", "0042_.sql", "0042_add.txt", "00a2_add.sql", "_add.sql", "README.md", "0001_x/y.sql", "99999999999999999999_x.sql"] {
            assert_eq!(number(stray), None, "{}", stray);
        }
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — one width across both sets, the first file off
    // it named
    #[test]
    fn every_file_shares_the_first_file_s_width() {
        let base: Files = [("0001_a.sql".to_string(), 1), ("0002_b.sql".to_string(), 2)].into();
        let work: Files = [("0001_a.sql".to_string(), 1), ("0003_c.sql".to_string(), 3)].into();
        assert!(shared_width(&[("m", &base), ("m", &work)]).is_ok());
        assert!(shared_width(&[("m", &Files::new()), ("m", &Files::new())]).is_ok());
        let wide: Files = [("0001_a.sql".to_string(), 1), ("10003_c.sql".to_string(), 3)].into();
        let fault = shared_width(&[("m", &base), ("w", &wide)]).expect_err("a five-digit name in a four-digit directory");
        assert!(fault.to_string().contains("w/10003_c.sql: its number is 5 digit(s) wide where the first file by name, 0001_a.sql, is 4"), "{}", fault);
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — gone or changed is edited, an addition at or
    // under the highest is out of order
    #[test]
    fn history_is_append_only_and_ordered() {
        let base: Files = [("0001_a.sql".to_string(), 1), ("0002_b.sql".to_string(), 2), ("0003_c.sql".to_string(), 3)].into();
        let work: Files = [("0001_a.sql".to_string(), 1), ("0003_c.sql".to_string(), 3), ("0002_late.sql".to_string(), 2), ("0004_d.sql".to_string(), 4)].into();
        let edited: BTreeSet<String> = ["0001_a.sql".to_string()].into();
        let mut got = judge("m", &base, &work, &edited, &Files::new(), &[]);
        got.sort();
        assert_eq!(got, ["m/0001_a.sql: [MIGRATION-EDITED]", "m/0002_b.sql: [MIGRATION-EDITED]", "m/0002_late.sql: [MIGRATION-ORDER] 0002 <= 0003"]);
        assert!(judge("m", &Files::new(), &work, &BTreeSet::new(), &Files::new(), &[]).is_empty());
    }

    fn set(names: &[&str]) -> Files {
        readable(names.iter().map(|n| n.to_string()))
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — ORDER reads the base ref's commit too, a shared
    // number reds every addition but the first by name, a clash names each ref's first such file, and
    // an addition carries the first of the three that applies
    #[test]
    fn each_addition_carries_the_first_of_order_dup_and_clash() {
        let base = set(&["0001_a.sql"]);
        let tip = set(&["0001_a.sql", "0002_theirs.sql", "README.md"]);
        let work = set(&["0001_a.sql", "0002_mine.sql", "0003_x.sql", "0003_y.sql", "0004_z.sql", "0005_w.sql"]);
        let refs = vec![
            ("refs/heads/one".to_string(), set(&["0003_q.sql", "0004_r.sql", "0004_s.sql", "0005_w.sql"])),
            ("refs/heads/two".to_string(), set(&["0004_t.sql"])),
        ];
        let mut got = judge("m", &base, &work, &BTreeSet::new(), &tip, &refs);
        got.sort();
        assert_eq!(
            got,
            [
                "m/0002_mine.sql: [MIGRATION-ORDER] 0002 <= 0002",
                "m/0003_x.sql: [MIGRATION-CLASH] 0003 refs/heads/one 0003_q.sql",
                "m/0003_y.sql: [MIGRATION-DUP] 0003 also 0003_x.sql",
                "m/0004_z.sql: [MIGRATION-CLASH] 0004 refs/heads/one 0004_r.sql",
                "m/0004_z.sql: [MIGRATION-CLASH] 0004 refs/heads/two 0004_t.sql",
            ]
        );
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — an addition the base ref's commit holds by name
    // is the base's, so no class reads it, the destructive check included
    #[test]
    fn an_addition_the_base_already_holds_by_name_is_not_the_branch_s() {
        let base = set(&["0001_a.sql"]);
        let tip = set(&["0001_a.sql", "0002_picked.sql"]);
        let work = set(&["0001_a.sql", "0002_picked.sql", "0003_own.sql"]);
        assert!(judge("m", &base, &work, &BTreeSet::new(), &tip, &[]).is_empty());
        assert_eq!(added(&base, &work, &tip).keys().collect::<Vec<_>>(), ["0003_own.sql"]);
        let list = vec![("DROP TABLE".to_string(), Ere::compile("DROP TABLE").expect("compiles"))];
        let reds: Vec<String> = added(&base, &work, &tip).keys().flat_map(|n| destructive(n, "DROP TABLE t;", &list)).collect();
        assert_eq!(reds, ["0003_own.sql:1: [MIGRATION-DESTRUCTIVE] matches 'DROP TABLE'"]);
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — comments and strings read as a space, a
    // statement's lines run from its first code line to its `;`
    #[test]
    fn statements_strip_comments_and_strings() {
        let sql = "-- head\ncreate table t (x int); /* drop table t */\ninsert into t values ('drop table t;');\n\nalter table t\n  drop   column x; -- migration-ok: legacy\ndrop table u";
        let (stmts, waivers) = statements(sql);
        let texts: Vec<(usize, usize, &str)> = stmts.iter().map(|s| (s.first, s.last, s.text.as_str())).collect();
        assert_eq!(
            texts,
            [(2, 2, "CREATE TABLE T (X INT)"), (3, 3, "INSERT INTO T VALUES ( )"), (5, 6, "ALTER TABLE T DROP COLUMN X"), (7, 7, "DROP TABLE U")]
        );
        assert_eq!(waivers, [(6, "legacy".to_string())]);
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — a reasoned declaration clears, an empty one
    // clears nothing and says so
    #[test]
    fn a_destructive_statement_needs_a_reasoned_declaration() {
        let list: Vec<(String, Ere)> = ["DROP (TABLE|INDEX|VIEW|TRIGGER)", "ALTER TABLE .* DROP"]
            .iter()
            .map(|p| (p.to_string(), Ere::compile(p).expect("compiles")))
            .collect();
        let sql = "DROP TABLE a; -- migration-ok: retired\nDROP INDEX b; -- migration-ok:\nalter table c\n  drop column d;\nALTER TABLE \"users\" DROP COLUMN \"email\";\n";
        assert_eq!(
            destructive("m/0002_x.sql", sql, &list),
            [
                "m/0002_x.sql:2: [MIGRATION-DESTRUCTIVE] matches 'DROP (TABLE|INDEX|VIEW|TRIGGER)', its waiver reason empty",
                "m/0002_x.sql:3: [MIGRATION-DESTRUCTIVE] matches 'ALTER TABLE .* DROP'",
                "m/0002_x.sql:5: [MIGRATION-DESTRUCTIVE] matches 'ALTER TABLE .* DROP'",
            ]
        );
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — a double-quoted identifier reads as its name,
    // its doubled quote as one
    #[test]
    fn a_quoted_identifier_reads_as_its_name() {
        let (stmts, _) = statements("ALTER TABLE \"users\" DROP COLUMN \"e\"\"mail\";\nselect 'a; b' from \"my t\"");
        let texts: Vec<&str> = stmts.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(texts, ["ALTER TABLE USERS DROP COLUMN E\"MAIL", "SELECT FROM MY T"]);
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — a driver is the name between `filter.` and the
    // last dot, a dotted name kept whole
    #[test]
    fn a_configured_driver_is_its_key_s_middle() {
        let got = driver_names(b"filter.lfs.smudge\ngit-lfs smudge -- %f\0filter.my.tool.clean\ntr -d x\0filter.lfs.process\ngit-lfs filter-process\0");
        assert_eq!(got.into_iter().collect::<Vec<_>>(), ["lfs", "my.tool"]);
        assert!(driver_names(b"").is_empty());
    }
}
