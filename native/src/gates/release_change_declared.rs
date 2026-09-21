// spec: gate-sdk/SPEC.md §check-release-change-declared — every kit `bin/` path removed and every
// template `init` claims changed since the nearest reachable `v*` tag is named on the release
// declaration surface's Behavior-changes section
use crate::declaration;
use crate::installer::{recipe, GATES_DIR};
use crate::walk;
use crate::{proc, programs};
use std::path::Path;

const NAME: &str = "check-release-change-declared";
const DEFAULT_DECL: &str = ".workflow/release-declarations.md";
const SECTION: &str = "Behavior changes";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Class {
    Removed,
    Template,
}

// spec: gate-sdk/SPEC.md §check-release-change-declared — the two readings of the change set: the
// live one reads git, and argument mode reads a canned dump so a fixture case can hold a base
trait Source {
    fn base(&self) -> Result<Option<String>, String>;
    fn changes(&self, base: &str) -> Result<Vec<(String, String)>, String>;
    fn at_base(&self, base: &str, path: &str) -> Result<Option<String>, String>;
    fn in_index(&self, path: &str) -> Result<Option<String>, String>;
}

struct Git;

fn git_out(args: &[&str]) -> Result<Vec<u8>, String> {
    let done = proc::run(&programs::GIT, args)?;
    match done.stdout() {
        Some(b) => Ok(b.to_vec()),
        None => Err(format!(
            "git {} failed ({}), so the change set could not be read; treating as failure (not clean)",
            args.join(" "),
            done.failure_report().unwrap_or_default()
        )),
    }
}

// spec: gate-sdk/SPEC.md §check-release-change-declared — a blob that may be absent is probed by
// `cat-file -e` status first, so a missing path is a verdict and a failed read stays a refusal
fn git_blob(spec: &str) -> Result<Option<String>, String> {
    if proc::run(&programs::GIT, &["cat-file", "-e", spec])?.code() != Some(0) {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&git_out(&["cat-file", "blob", spec])?).into_owned()))
}

impl Source for Git {
    // spec: gate-sdk/SPEC.md §check-release-change-declared — no commit and no reachable `v*` tag
    // are both the dormant verdict, decided by exit status and a ref listing rather than by
    // parsing describe's localized message
    fn base(&self) -> Result<Option<String>, String> {
        match proc::run(&programs::GIT, &["rev-parse", "-q", "--verify", "HEAD"])?.code() {
            Some(0) => {}
            Some(1) => return Ok(None),
            _ => return Err("git rev-parse HEAD failed, so no base can be resolved; treating as failure (not clean)".to_string()),
        }
        let tags = git_out(&["for-each-ref", "--merged", "HEAD", "--count=1", "--format=%(refname)", "refs/tags/v*"])?;
        if String::from_utf8_lossy(&tags).trim().is_empty() {
            return Ok(None);
        }
        let tag = git_out(&["describe", "--tags", "--abbrev=0", "--match", "v*"])?;
        Ok(Some(String::from_utf8_lossy(&tag).trim().to_string()))
    }

    fn changes(&self, base: &str) -> Result<Vec<(String, String)>, String> {
        let raw = git_out(&["diff", "--cached", "--no-renames", "--name-status", "-z", base])?;
        let text = String::from_utf8_lossy(&raw).into_owned();
        let fields: Vec<&str> = text.split('\0').filter(|f| !f.is_empty()).collect();
        if fields.len() % 2 != 0 {
            return Err("git diff --name-status -z returned an odd field count, so the change set is unreadable".to_string());
        }
        Ok(fields.chunks(2).map(|c| (c[0].to_string(), c[1].to_string())).collect())
    }

    fn at_base(&self, base: &str, path: &str) -> Result<Option<String>, String> {
        git_blob(&format!("{}:{}", base, path))
    }

    fn in_index(&self, path: &str) -> Result<Option<String>, String> {
        git_blob(&format!(":{}", path))
    }
}

// spec: gate-sdk/SPEC.md §check-release-change-declared — argument mode: the dump's `base` line
// names the tag or `-` for none, each other line is one `--name-status` record, the base tree is
// a directory and the index is the working directory
struct Canned {
    dump: String,
    base_dir: String,
}

fn read_opt(path: &str) -> Result<Option<String>, String> {
    let p = Path::new(path);
    if !p.exists() {
        return Ok(None);
    }
    std::fs::read(p)
        .map(|b| Some(String::from_utf8_lossy(&b).into_owned()))
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

impl Canned {
    fn records(&self) -> impl Iterator<Item = &str> {
        self.dump.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
    }
}

impl Source for Canned {
    fn base(&self) -> Result<Option<String>, String> {
        let line = self.records().next().unwrap_or("");
        match line.strip_prefix("base ").map(str::trim) {
            Some("-") => Ok(None),
            Some(t) if !t.is_empty() => Ok(Some(t.to_string())),
            _ => Err("the canned change dump does not open with a `base <tag>` or `base -` line".to_string()),
        }
    }

    fn changes(&self, _base: &str) -> Result<Vec<(String, String)>, String> {
        let mut out = Vec::new();
        for line in self.records().skip(1) {
            match line.split_once('\t') {
                Some((s, p)) => out.push((s.to_string(), p.to_string())),
                None => return Err(format!("the canned change dump carries a record with no tab: {}", line)),
            }
        }
        Ok(out)
    }

    fn at_base(&self, _base: &str, path: &str) -> Result<Option<String>, String> {
        read_opt(&format!("{}/{}", self.base_dir, path))
    }

    fn in_index(&self, path: &str) -> Result<Option<String>, String> {
        read_opt(path)
    }
}

fn clean_rel(p: &str) -> String {
    p.trim_start_matches("./").trim_end_matches('/').to_string()
}

// spec: gate-sdk/SPEC.md §check-release-change-declared — the claimed set is the installer
// recipe's own derivation, the config seam plus every planned seed source, never a list
fn claimed(roots: &[String]) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for root in roots {
        let payload = Path::new(root);
        for (src, _) in recipe::config_seam_plan(payload, GATES_DIR) {
            out.push(clean_rel(&src));
        }
        let kit = payload
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        for s in recipe::seed(&kit, payload, Path::new("."), true)? {
            if let recipe::Seeded::Plan(src, _) = s {
                out.push(clean_rel(&src));
            }
        }
    }
    Ok(out)
}

// spec: gate-sdk/SPEC.md §check-release-change-declared — a data change is a change to the
// multiset of non-comment, non-blank lines, so a comment-only edit or a reorder is none
fn data_lines(text: &str) -> Vec<String> {
    let mut v: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect();
    v.sort();
    v
}

// spec: gate-sdk/SPEC.md §check-release-change-declared — a path is named verbatim, or by its
// directory with a trailing `/` together with its basename in one top-level bullet
fn declared(bullets: &[&str], path: &str) -> bool {
    let (dir, base) = match path.rsplit_once('/') {
        // path-dialect-exempt: a substring matched against bullet prose, never a filesystem location
        Some((d, b)) => (format!("{}/", d), b),
        None => (String::new(), path),
    };
    bullets
        .iter()
        .any(|b| b.contains(path) || (!dir.is_empty() && b.contains(&dir) && b.contains(base)))
}

fn rule(args: &[String]) -> Result<i32, String> {
    let decl_file = args
        .first()
        .filter(|a| !a.is_empty())
        .map(String::as_str)
        .unwrap_or(DEFAULT_DECL);
    let canned = match (args.get(1), args.get(2)) {
        (Some(d), Some(b)) => Some(Canned {
            dump: read_opt(d)?.ok_or_else(|| format!("canned change dump not found: {}", d))?,
            base_dir: b.clone(),
        }),
        (Some(_), None) => return Err("argument mode takes a change dump and a base-tree dir together".to_string()),
        _ => None,
    };
    let source: &dyn Source = match &canned {
        Some(c) => c,
        None => &Git,
    };

    let roots: Vec<String> = walk::kit_roots()?.iter().map(|r| clean_rel(r)).collect();
    if roots.is_empty() {
        return Err("the kit-root set resolved empty, so no kit `bin/` path or claimed template can be recognised".to_string());
    }

    // spec: gate-sdk/SPEC.md §check-release-change-declared — the surface is read where the commit
    // will carry it, the index, and a surface without its header refuses rather than reading empty
    let surface = match &canned {
        Some(_) => read_opt(decl_file)?,
        None => source.in_index(decl_file)?,
    };
    let surface = surface.ok_or_else(|| format!("{} is absent, so the declaration surface cannot be read (gate-sdk/SPEC.md §upgrade-smoke owns its contract)", decl_file))?;
    if !surface.lines().next().unwrap_or("").starts_with("# contract:") {
        return Err(format!("{} lacks its `# contract:` header line, so the declaration surface cannot be established (gate-sdk/SPEC.md §upgrade-smoke owns its contract)", decl_file));
    }

    let Some(base) = source.base()? else {
        println!("RELEASE-CHANGE-DECLARED: clean (dormant: no v* tag is reachable from HEAD, so no released base exists to diff against and nothing was checked)");
        return Ok(0);
    };

    let claimed = claimed(&roots)?;
    let mut subjects: Vec<(String, Class)> = Vec::new();
    for (status, path) in source.changes(&base)? {
        let removed_tool = status == "D" && roots.iter().any(|r| walk::under(&format!("{}/bin", r), &path));
        if removed_tool {
            subjects.push((path, Class::Removed));
            continue;
        }
        if !claimed.iter().any(|c| c == &path) {
            continue;
        }
        let (Some(old), Some(new)) = (source.at_base(&base, &path)?, source.in_index(&path)?) else {
            continue;
        };
        if data_lines(&old) != data_lines(&new) {
            subjects.push((path, Class::Template));
        }
    }

    let bullets: Vec<&str> = declaration::section_bullets(&surface, SECTION)
        .unwrap_or_default()
        .into_iter()
        .filter(|l| l.starts_with("- ") || l.starts_with("* "))
        .collect();
    let missing: Vec<&(String, Class)> = subjects.iter().filter(|(p, _)| !declared(&bullets, p)).collect();

    if !missing.is_empty() {
        println!(
            "{}: {} kit-shipped change(s) since {} are not named in {}'s '{}' section:",
            NAME,
            missing.len(),
            base,
            decl_file,
            SECTION
        );
        for (p, class) in &missing {
            match class {
                Class::Removed => println!("  {} — class R, a removed kit tool: add a Behavior-changes bullet naming this path and what replaced it", p),
                Class::Template => println!("  {} — class T, a changed template `init` claims: add a Behavior-changes bullet naming this path, and a Tightened-gates bullet for each gate the change can red", p),
            }
        }
        println!("  help: append the bullet(s) to {} in this commit — the session that finds an undeclared kit-shipped change declares it (gate-sdk/SPEC.md §upgrade-smoke)", decl_file);
        return Ok(1);
    }

    let removed = subjects.iter().filter(|(_, c)| *c == Class::Removed).count();
    println!(
        "RELEASE-CHANGE-DECLARED: clean ({} removed kit tool path(s) and {} changed claimed template(s) since {}, each named in {}'s '{}' section)",
        removed,
        subjects.len() - removed,
        base,
        decl_file,
        SECTION
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-release-change-declared — the dir-and-basename form needs both
    // halves in one bullet, and a verbatim path needs nothing else
    #[test]
    fn a_path_is_declared_verbatim_or_by_directory_and_basename_in_one_bullet() {
        let bullets = ["- **`a-kit/bin/`** — deleted (`one.sh`, `two.sh`)", "- **`b-kit/bin/x.sh`** — gone"];
        assert!(declared(&bullets, "a-kit/bin/one.sh"));
        assert!(declared(&bullets, "b-kit/bin/x.sh"));
        assert!(!declared(&bullets, "a-kit/bin/three.sh"));
        assert!(!declared(&["- `one.sh` gone", "- `a-kit/bin/` gone"], "a-kit/bin/one.sh"));
    }

    // spec: gate-sdk/SPEC.md §check-release-change-declared — comments, blanks and order carry no data
    #[test]
    fn a_comment_or_order_edit_is_no_data_change() {
        assert_eq!(data_lines("# a\nx\ny\n"), data_lines("y\n\n# b\nx\n"));
        assert_ne!(data_lines("x\n"), data_lines("x\nx\n"));
    }
}
