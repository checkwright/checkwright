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
const USAGE: &str = "usage: check-migration-guard [--base <ref> | --base-dir <dir>]";
const WAIVER: &str = "migration-ok:";
const DIGITS: usize = 4;

type Files = BTreeMap<String, u32>;

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

// spec: gate-sdk/SPEC.md §check-migration-guard — `NNNN_<name>.sql`, four digits and a name
fn number(name: &str) -> Option<u32> {
    let stem = name.get(DIGITS..)?.strip_prefix('_')?.strip_suffix(".sql")?;
    let digits = name.get(..DIGITS)?;
    if stem.is_empty() || stem.contains('/') || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

fn files(dir: &str, names: impl IntoIterator<Item = String>) -> Result<Files, Fault> {
    names
        .into_iter()
        .map(|n| match number(&n) {
            Some(k) => Ok((n, k)),
            None => Err(Fault::input(format!("{}/{}: not a migration file — every file there is named NNNN_<name>.sql", dir, n))),
        })
        .collect()
}

fn padded(n: u32) -> String {
    format!("{:0width$}", n, width = DIGITS)
}

// spec: gate-sdk/SPEC.md §check-migration-guard — a merge-base file gone or changed is edited; an
// added one numbered at or under the merge base's highest is out of order
fn judge(dir: &str, base: &Files, work: &Files, edited: &BTreeSet<String>) -> Vec<String> {
    let mut out: Vec<String> = base
        .keys()
        .filter(|n| !work.contains_key(*n) || edited.contains(*n))
        .map(|n| format!("{}/{}: [MIGRATION-EDITED]", dir, n))
        .collect();
    if let Some(highest) = base.values().max() {
        out.extend(
            work.iter()
                .filter(|(n, k)| !base.contains_key(*n) && *k <= highest)
                .map(|(n, k)| format!("{}/{}: [MIGRATION-ORDER] {} <= {}", dir, n, padded(*k), padded(*highest))),
        );
    }
    out
}

#[derive(Debug, PartialEq)]
struct Statement {
    first: usize,
    last: usize,
    text: String,
}

// spec: gate-sdk/SPEC.md §check-migration-guard — statements cut at `;`, comments and quoted strings
// read as a space, whitespace runs as one, upper-cased; each `-- migration-ok:` reason with its line
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
            '\'' | '"' => {
                first.get_or_insert(line);
                while let Some(q) = chars.next() {
                    line += usize::from(q == '\n');
                    if q == c && chars.next_if_eq(&c).is_none() {
                        break;
                    }
                }
                space(&mut text);
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
    let mut edited = BTreeSet::new();
    for n in base.keys().filter(|n| work.contains_key(*n)) {
        if read(&Path::new(base_dir).join(n))? != read(&Path::new(dir).join(n))? {
            edited.insert(n.clone());
        }
    }
    Ok((base, edited))
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

    let (base, edited, what) = match against {
        Against::Base(flag) => {
            let b = since_base::resolve(flag.as_deref())?;
            let base = files(&dir, under(b.files_at_base(&dir_top)?, &dir_top))?;
            let edited: BTreeSet<String> = under(b.changed(&[&dir_top])?, &dir_top).into_iter().collect();
            (base, edited, format!("merge base {}", b.short()))
        }
        Against::Dir(d) => {
            let (base, edited) = from_dir(&d, &dir, &work)?;
            (base, edited, d)
        }
    };

    let mut out = judge(&dir, &base, &work, &edited);
    let added: Vec<&String> = work.keys().filter(|n| !base.contains_key(*n)).collect();
    if let Some(list) = &list {
        for n in &added {
            let path = format!("{}/{}", dir, n);
            let sql = std::fs::read_to_string(&path).map_err(|e| Fault::input(format!("cannot read {}: {}", path, e)))?;
            out.extend(destructive(&path, &sql, list));
        }
    }
    out.sort();

    if out.is_empty() {
        let off = if list.is_none() { format!("; destructive check off: no list at {}", list_path) } else { String::new() };
        println!("MIGRATION-GUARD: clean ({} migration(s), {} added, against {}{})", work.len(), added.len(), what, off);
        return Ok(0);
    }
    println!("{}: migration(s) under {} edited, out of order or destructive against {}:", NAME, dir, what);
    for r in &out {
        println!("{}", r);
    }
    println!("  help: a merged migration is history: restore an edited or deleted one and put the change in a");
    println!("        new file numbered above the highest; declare a deliberate destructive statement with");
    println!("        '-- {} <reason>' on one of its lines.", WAIVER);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-migration-guard — four digits, an underscore, a name and `.sql`
    #[test]
    fn a_migration_name_carries_its_number() {
        assert_eq!(number("0042_add_users.sql"), Some(42));
        for stray in ["42_add.sql", "0042add.sql", "0042_.sql", "0042_add.txt", "00a2_add.sql", "README.md", "0001_x/y.sql"] {
            assert_eq!(number(stray), None, "{}", stray);
        }
    }

    // spec: gate-sdk/SPEC.md §check-migration-guard — gone or changed is edited, an addition at or
    // under the highest is out of order
    #[test]
    fn history_is_append_only_and_ordered() {
        let base: Files = [("0001_a.sql".to_string(), 1), ("0002_b.sql".to_string(), 2), ("0003_c.sql".to_string(), 3)].into();
        let work: Files = [("0001_a.sql".to_string(), 1), ("0003_c.sql".to_string(), 3), ("0002_late.sql".to_string(), 2), ("0004_d.sql".to_string(), 4)].into();
        let edited: BTreeSet<String> = ["0001_a.sql".to_string()].into();
        let mut got = judge("m", &base, &work, &edited);
        got.sort();
        assert_eq!(got, ["m/0001_a.sql: [MIGRATION-EDITED]", "m/0002_b.sql: [MIGRATION-EDITED]", "m/0002_late.sql: [MIGRATION-ORDER] 0002 <= 0003"]);
        assert!(judge("m", &Files::new(), &work, &BTreeSet::new()).is_empty());
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
        let sql = "DROP TABLE a; -- migration-ok: retired\nDROP INDEX b; -- migration-ok:\nalter table c\n  drop column d;\n";
        assert_eq!(
            destructive("m/0002_x.sql", sql, &list),
            [
                "m/0002_x.sql:2: [MIGRATION-DESTRUCTIVE] matches 'DROP (TABLE|INDEX|VIEW|TRIGGER)', its waiver reason empty",
                "m/0002_x.sql:3: [MIGRATION-DESTRUCTIVE] matches 'ALTER TABLE .* DROP'",
            ]
        );
    }
}
