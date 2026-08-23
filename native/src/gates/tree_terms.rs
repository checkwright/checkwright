// spec: gate-sdk/SPEC.md §check-tree-terms — no tracked file matches the banned-pattern set (the
// tracked-files half of the leak guard; the pattern files and their templates are self-exempt)
use crate::ere::Ere;
use crate::fresh;
use crate::gates::commit_msg::{is_pattern, resolve_files};
use crate::proc;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-tree-terms — the self-exemption is a *prefix* glob over the
// basename (`[[ "$base" == msg-patterns* ]]`), so a template and a `.local` sibling are exempt
// beside the file itself
const SELF_EXEMPT_PREFIX: &str = "msg-patterns";

fn self_exempt(path: &str) -> bool {
    path.rsplit('/')
        .next()
        .unwrap_or(path)
        .starts_with(SELF_EXEMPT_PREFIX)
}

// spec: gate-sdk/SPEC.md §check-tree-terms — a NUL byte is what makes a file binary, the same
// test `grep` applies before it substitutes a path-only verdict for the matching line
fn binary(bytes: &[u8]) -> bool {
    bytes.contains(&0)
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("{}", msg);
            2
        }
    }
}

fn inner(args: &[String]) -> Result<i32, String> {
    let scanroot = fresh::positional(args, 0, ".").to_string();

    // spec: gate-sdk/SPEC.md §check-tree-terms — the tracked set is the subject, so a non-repo
    // cwd is fail-closed before anything else: there is no listing to be clean about
    let probe = proc::run("git", &["rev-parse", "--git-dir"])
        .map_err(|e| format!("check-tree-terms: {}", e))?;
    if probe.stdout().is_none() {
        return Err(
            "check-tree-terms: not a git repository — cannot enumerate tracked files".into(),
        );
    }

    let files: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        match resolve_files() {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                return Err(format!(
                    "TREE-TERMS: {}",
                    fresh::fail_closed("pattern-files", Some(2))
                ));
            }
        }
    };

    let mut patterns: Vec<String> = Vec::new();
    for f in &files {
        let Ok(bytes) = std::fs::read(f) else {
            return Err(format!(
                "TREE-TERMS: {}",
                fresh::fail_closed("grep-patterns", Some(2))
            ));
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        patterns.extend(
            fresh::file_lines(&text)
                .into_iter()
                .filter(|l| is_pattern(l))
                .map(String::from),
        );
    }

    // spec: gate-sdk/SPEC.md §check-tree-terms — an empty set leaves the tree unchecked and
    // clean: the fail-closed obligation is on a missing pattern file, never on an empty one
    if patterns.is_empty() {
        println!("TREE-TERMS: clean (0 banned pattern(s) configured; tree unchecked)");
        return Ok(0);
    }

    // spec: gate-sdk/SPEC.md §check-tree-terms — the GNU escape set is refused by name at
    // compile, which is exit 2: a pattern the compiled engine cannot honour must not read as a
    // tree with nothing in it
    let mut compiled: Vec<Ere> = Vec::new();
    for p in &patterns {
        match Ere::compile(p) {
            Ok(e) => compiled.push(e),
            Err(e) => {
                eprintln!("check-tree-terms: {}: {}", p, e);
                return Err(format!("TREE-TERMS: {}", fresh::fail_closed("grep", Some(2))));
            }
        }
    }

    let prune = walk::prune_dirs().map_err(|e| format!("check-tree-terms: {}", e))?;
    let ls = proc::run("git", &["ls-files", "--", &scanroot])
        .map_err(|e| format!("check-tree-terms: {}", e))?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            return Err(format!(
                "check-tree-terms: {}",
                fresh::fail_closed("git-ls-files", ls.code())
            ))
        }
    };

    // spec: gate-sdk/SPEC.md §check-tree-terms — the cheap-filter-then-match split: the per-path
    // filter costs no process, and the pattern set is compiled once for the whole walk rather than
    // per file. A port recompiling per file is the regression the split exists to prevent.
    let mut paths: Vec<&str> = Vec::new();
    for path in listing.lines() {
        if path.is_empty() || walk::path_pruned(path, &prune) || self_exempt(path) {
            continue;
        }
        if Path::new(path).is_file() {
            paths.push(path);
        }
    }

    let mut hits: Vec<String> = Vec::new();
    for path in &paths {
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        let text = String::from_utf8_lossy(&bytes).into_owned();
        if binary(&bytes) {
            // spec: gate-sdk/SPEC.md §check-tree-terms — a binary match is a path-only record and
            // still reds
            if fresh::file_lines(&text)
                .iter()
                .any(|line| compiled.iter().any(|re| re.is_match(line)))
            {
                hits.push((*path).to_string());
            }
            continue;
        }
        // spec: gate-sdk/SPEC.md §check-tree-terms — `grep -EnHf`: one record per *matching line*,
        // filename-prefixed and line-numbered, however many patterns that line matches, and two
        // identical lines are two records
        for (i, line) in fresh::file_lines(&text).iter().enumerate() {
            if compiled.iter().any(|re| re.is_match(line)) {
                hits.push(format!("{}:{}:{}", path, i + 1, line));
            }
        }
    }

    if !hits.is_empty() {
        println!("check-tree-terms: tracked file(s) match a banned pattern (leaked local/private term):");
        for h in &hits {
            println!("{}", h);
        }
        println!("  help: remove the leaked term from the tracked file; private term lists");
        println!("        belong in the gitignored local pattern file, never in a tracked one.");
        return Ok(1);
    }

    println!(
        "TREE-TERMS: clean ({} tracked file(s) scanned under {}; none match the {} banned pattern(s))",
        paths.len(),
        scanroot,
        patterns.len()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-tree-terms — this module sits inside the corpus the gate
    // scans, so a banned shape is composed here and never spelled
    fn banned_home_path() -> String {
        format!("clone into /{}/bob/projects and run it", "home")
    }

    // spec: gate-sdk/SPEC.md §check-tree-terms — the test corpus is the gate's own fixture pair,
    // never the consumer's tracked pattern list
    fn fixture_patterns(case: &str) -> Vec<Ere> {
        let dir = walk::fixture_case_dirs("check-tree-terms")
            .into_iter()
            .find(|d| d.ends_with(case))
            .expect("the check-tree-terms fixture pair is missing a case dir");
        let text = std::fs::read_to_string(dir.join("patterns.list"))
            .expect("the fixture case has no patterns.list");
        fresh::file_lines(&text)
            .into_iter()
            .filter(|l| is_pattern(l))
            .map(|p| Ere::compile(p).expect("a fixture pattern failed to compile"))
            .collect()
    }

    #[test]
    fn the_fixture_patterns_compile_and_the_anchored_shape_stays_anchored() {
        let res = fixture_patterns("good");
        assert_eq!(res.len(), 4, "the good case's blank line is not a pattern");
        let hit = |s: &str| res.iter().any(|r| r.is_match(s));
        assert!(hit(&banned_home_path()));
        let uuid = "3f2504e0-4f89-41d3-9a0c-0305e82c3301";
        assert!(hit(&format!("Session-Id: {}", uuid)));
        assert!(!hit(&format!("  Session-Id: {}", uuid)));
        assert!(!hit(&format!("a-bare-uuid {} with no trailer key", uuid)));
    }

    // spec: gate-sdk/SPEC.md §check-tree-terms — a GNU escape in a consumer's pattern file is
    // refused by name at compile, which the caller turns into exit 2 rather than an empty set
    #[test]
    fn a_gnu_escape_is_refused_by_name_rather_than_silently_accepted() {
        let err = match Ere::compile("\\bparcel\\b") {
            Ok(_) => panic!("a GNU escape must not compile"),
            Err(e) => e,
        };
        assert!(format!("{}", err).contains("\\b"));
    }

    // spec: gate-sdk/SPEC.md §check-tree-terms — the self-exemption is a prefix glob over the
    // basename, so the `.local` sibling and the shipped template are exempt beside the file
    #[test]
    fn the_self_exemption_is_a_basename_prefix_and_not_an_exact_name() {
        assert!(self_exempt("scripts/msg-patterns.list"));
        assert!(self_exempt("scripts/msg-patterns.local.list"));
        assert!(self_exempt("msg-patterns.list"));
        assert!(!self_exempt("scripts/patterns.list"));
        assert!(!self_exempt("docs/notes-msg-patterns.list"));
    }

    #[test]
    fn a_nul_bearing_file_is_binary_and_a_text_one_is_not() {
        assert!(binary(b"PNG\x00\x01"));
        assert!(!binary(banned_home_path().as_bytes()));
    }
}
