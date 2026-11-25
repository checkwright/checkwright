// spec: gate-sdk/SPEC.md §check-tree-terms — no tracked file matches the banned-pattern set (the
// tracked-files half of the leak guard; the pattern files and their templates are self-exempt)
use crate::ere::Ere;
use crate::exit::{self, Fault};
use crate::fresh;
use crate::deny_terms::{self, Entry};
use crate::gates::commit_msg::resolve_files;
use crate::knobs::gate_sdk::{MSG_PATTERN_FILE, STRINGS_SHAPES_FILE};
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-tree-terms — a NUL byte is what makes a file binary, the same
// test `grep` applies before it substitutes a path-only verdict for the matching line
fn binary(bytes: &[u8]) -> bool {
    bytes.contains(&0)
}

pub fn run(args: &[String]) -> i32 {
    match inner(args) {
        Ok(code) => code,
        Err(fault) => {
            eprintln!("{}", fault);
            fault.status()
        }
    }
}

// spec: gate-sdk/SPEC.md §check-tree-terms — the id a hit takes when no `# rule:` line names one
const DEFAULT_ID: &str = "TEXT-DENIED";

struct Pattern {
    re: Ere,
    id: Option<String>,
}

// spec: gate-sdk/SPEC.md §The deny-term list — a term hits where it occurs with no ASCII letter or
// digit on either side, ASCII case folded
fn word_hit(line: &[u8], term: &[u8]) -> bool {
    if term.is_empty() || term.len() > line.len() {
        return false;
    }
    (0..=line.len() - term.len()).any(|at| {
        line[at..at + term.len()].eq_ignore_ascii_case(term)
            && (at == 0 || !line[at - 1].is_ascii_alphanumeric())
            && line.get(at + term.len()).map_or(true, |b| !b.is_ascii_alphanumeric())
    })
}

// spec: gate-sdk/SPEC.md §check-tree-terms — every distinct id hitting one line, ordered
fn line_ids<'a>(line: &str, patterns: &'a [Pattern], terms: &'a [Entry]) -> BTreeSet<&'a str> {
    let mut ids = BTreeSet::new();
    for p in patterns.iter().filter(|p| p.re.is_match(line)) {
        ids.insert(p.id.as_deref().unwrap_or(DEFAULT_ID));
    }
    for t in terms.iter().filter(|t| word_hit(line.as_bytes(), t.text.as_bytes())) {
        ids.insert(t.id_or(DEFAULT_ID));
    }
    ids
}

// spec: gate-sdk/SPEC.md §check-tree-terms — each resolved pattern file's live lines with the
// `# rule:` id in force above each, under §The deny-term list's grammar
fn read_patterns(files: &[String]) -> Result<Vec<(String, Option<String>)>, Fault> {
    let mut out = Vec::new();
    for f in files {
        let bytes = std::fs::read(f).map_err(|e| {
            let fault = Fault::reading(f, format!("check-tree-terms: cannot read the pattern file {}: {}", f, e));
            let closed = fresh::fail_closed("grep-patterns", Some(fault.status()));
            Fault { message: format!("{}\nTREE-TERMS: {}", fault.message, closed), ..fault }
        })?;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        out.extend(deny_terms::parse(&text).map_err(|line| deny_terms::malformed(f, line).prefixed("check-tree-terms"))?);
    }
    Ok(out)
}

fn inner(args: &[String]) -> Result<i32, Fault> {
    let scanroot = fresh::positional(args, 0, ".").to_string();

    // spec: gate-sdk/SPEC.md §check-tree-terms — the tracked set is the subject, so a non-repo
    // cwd is fail-closed before anything else: there is no listing to be clean about
    if crate::walk::toplevel_opt().map_err(|e| Fault::tool(format!("check-tree-terms: {}", e)))?.is_none() {
        return Err(Fault::refused(
            "check-tree-terms: not a git repository — cannot enumerate tracked files",
        ));
    }

    let files: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        match resolve_files() {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e.message);
                let status = e.status();
                return Err(Fault {
                    message: format!("TREE-TERMS: {}", fresh::fail_closed("pattern-files", Some(status))),
                    ..e
                });
            }
        }
    };

    let patterns = read_patterns(&files)?;
    let term_files = deny_terms::files().map_err(|e| e.prefixed("check-tree-terms"))?;
    let terms = deny_terms::read(&term_files).map_err(|e| e.prefixed("check-tree-terms"))?;

    // spec: gate-sdk/SPEC.md §check-tree-terms — an empty set leaves the tree unchecked and
    // clean: the fail-closed obligation is on a missing pattern file, never on an empty one
    if patterns.is_empty() && terms.is_empty() {
        println!("TREE-TERMS: clean (not applicable: 0 banned pattern(s) and 0 deny term(s) configured; tree unchecked)");
        return Ok(0);
    }

    // spec: gate-sdk/SPEC.md §check-tree-terms — the GNU escape set is refused by name at
    // compile, which is exit 2: a pattern the compiled engine cannot honour must not read as a
    // tree with nothing in it
    let mut compiled: Vec<Pattern> = Vec::new();
    for (p, id) in &patterns {
        match Ere::compile(p) {
            Ok(re) => compiled.push(Pattern { re, id: id.clone() }),
            Err(e) => {
                eprintln!("check-tree-terms: {}: {}", p, e);
                return Err(Fault::refused(format!(
                    "TREE-TERMS: {}",
                    fresh::fail_closed("grep", Some(exit::REFUSED))
                )));
            }
        }
    }

    // spec: gate-sdk/SPEC.md §check-tree-terms — the pattern files the two knobs name, or the
    // positional list, the deny-term lists, and each kit's starter rosters
    let mut named: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        let mut n = walk::knob_words("GATE_SDK_MSG_PATTERN_FILES").map_err(Fault::refused)?;
        n.extend(walk::knob_words("GATE_SDK_MSG_PATTERN_FILES_LOCAL").map_err(Fault::refused)?);
        n
    };
    named.extend(term_files);
    let exempt = walk::self_exempt_paths(&named, &[MSG_PATTERN_FILE, STRINGS_SHAPES_FILE])
        .map_err(|e| Fault::refused(format!("check-tree-terms: {}", e)))?;

    let prune = walk::prune_dirs().map_err(|e| Fault::refused(format!("check-tree-terms: {}", e)))?;
    let listing = crate::listing::tracked(None, &[&scanroot])
        .map_err(|e| Fault::tool(format!("check-tree-terms: {}", e.text())))?;

    // spec: gate-sdk/SPEC.md §check-tree-terms — the cheap-filter-then-match split: the per-path
    // filter costs no process, and the pattern set is compiled once for the whole walk rather than
    // per file. A port recompiling per file is the regression the split exists to prevent.
    let mut paths: Vec<&str> = Vec::new();
    for path in listing.iter().map(String::as_str) {
        if walk::path_pruned(path, &prune) || exempt.iter().any(|e| e == path) {
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
            // spec: gate-sdk/SPEC.md §check-tree-terms — a binary hit is a path-only record per
            // distinct id, and still reds
            let mut ids = BTreeSet::new();
            for line in fresh::file_lines(&text) {
                ids.extend(line_ids(line, &compiled, &terms));
            }
            hits.extend(ids.into_iter().map(|id| format!("{}: [{}]", path, id)));
            continue;
        }
        // spec: gate-sdk/SPEC.md §check-tree-terms — one record per matching line per distinct id,
        // never the line itself, and two identical lines are two records
        for (i, line) in fresh::file_lines(&text).iter().enumerate() {
            for id in line_ids(line, &compiled, &terms) {
                hits.push(format!("{}:{}: [{}]", path, i + 1, id));
            }
        }
    }

    if !hits.is_empty() {
        println!("check-tree-terms: tracked file(s) match a banned pattern or deny term (leaked local/private term):");
        for h in &hits {
            println!("{}", h);
        }
        println!("  help: remove the leaked term from the line each record names; the record carries");
        println!("        the rule id, never the term. Private term lists belong in a gitignored");
        println!("        local file, never in a tracked one.");
        return Ok(1);
    }

    println!(
        "TREE-TERMS: clean ({} tracked file(s) scanned under {}; none match the {} banned pattern(s) or {} deny term(s))",
        paths.len(),
        scanroot,
        patterns.len(),
        terms.len()
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
            .filter(|l| fresh::live_line(l))
            .map(|p| Ere::compile(p).expect("a fixture pattern failed to compile"))
            .collect()
    }

    #[test]
    fn the_fixture_patterns_compile_and_the_anchored_shape_stays_anchored() {
        let res = fixture_patterns("good");
        assert_eq!(res.len(), 5, "the good case's blank lines and comments are not patterns");
        let hit = |s: &str| res.iter().any(|r| r.is_match(s));
        assert!(hit(&banned_home_path()));
        let uuid = "3f2504e0-4f89-41d3-9a0c-0305e82c3301";
        assert!(hit(&format!("Session-Id: {}", uuid)));
        assert!(!hit(&format!("  Session-Id: {}", uuid)));
        assert!(!hit(&format!("a-bare-uuid {} with no trailer key", uuid)));
        let noun = "account";
        assert!(hit(&format!("the forge {}s were `acct-0` and `acct-1`", noun)));
        assert!(!hit("the resolver handles a `..` traversal segment"));
        assert!(!hit("the wrapper handles `gh` before it pushes the tag"));
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

    // spec: gate-sdk/SPEC.md §The deny-term list — a term hits as a whole word, case folded, and
    // never inside a longer word
    #[test]
    fn a_term_hits_as_a_whole_word_and_never_inside_a_longer_one() {
        assert!(word_hit(b"ships the Term today", b"term"));
        assert!(word_hit(b"term", b"term"));
        assert!(word_hit(b"(term-x)", b"term"));
        assert!(!word_hit(b"the terms2 table", b"term"));
        assert!(!word_hit(b"determine", b"term"));
        assert!(!word_hit(b"ter", b"term"));
    }

    #[test]
    fn a_nul_bearing_file_is_binary_and_a_text_one_is_not() {
        assert!(binary(b"PNG\x00\x01"));
        assert!(!binary(banned_home_path().as_bytes()));
    }
}
