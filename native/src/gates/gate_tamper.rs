// spec: delegation-kit/SPEC.md §Verify after every agent commit — a gate-weakening commit is
// blocked by shape (A gate edits stay meta-isolated; B a new path-exemption can't excuse a
// co-staged file)
use crate::{proc, programs, registry};
use crate::walk;
use std::path::Path;

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\u{b}' || c == '\u{c}'
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — a meta-layer path is one of
// the configured prefixes, or a root-level `*.md`
// spec: gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure — a configured member
// names a directory whether or not its value spells the trailing separator the knob's default
// carries, so containment is asked by component rather than by text
fn is_meta(p: &str, meta: &[String]) -> bool {
    if meta.iter().any(|pre| walk::at_or_under(pre.trim_end_matches('/'), p)) {
        return true;
    }
    p.ends_with(".md") && !p.contains('/')
}

// spec: delegation-kit/SPEC.md §Layout and configuration — each element is a bash `[[ == ]]`
// pattern, whose `*` crosses `/`; `walk::pattern_match` is that rule
fn is_gate_file(p: &str, globs: &[String]) -> bool {
    globs.iter().any(|g| walk::pattern_match(g, p))
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — assertion B judges only a
// path-shaped exemption: one carrying `/`, `*`, `?` or `[`
fn is_pathlike(s: &str) -> bool {
    s.contains('/') || s.contains('*') || s.contains('?') || s.contains('[')
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — the comment-and-blank filter
// the fixture lists are read through
fn list_lines(text: &str) -> Vec<String> {
    text.lines()
        .filter(|l| {
            let t = l.trim_start_matches(is_space);
            !t.is_empty() && !t.starts_with('#')
        })
        .map(String::from)
        .collect()
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — shell-array values use the shared reader
fn extract_exemptions(text: &str) -> Vec<String> {
    registry::exception_arrays(text).into_iter().flat_map(|a| a.elements.into_iter().map(|e| e.value)).collect()
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — a descriptor's `# exception:`
// elements join its shell-array read, through the one parser of that field
fn declared_exemptions(path: &str, text: &str) -> Vec<String> {
    let mut out = extract_exemptions(text);
    if path.ends_with(".gate") {
        out.extend(registry::exemptions(text).into_iter().map(|e| e.element));
    }
    out
}

fn added_between(path: &str, old: &str, new: &str) -> Vec<String> {
    let old_set = declared_exemptions(path, old);
    declared_exemptions(path, new).into_iter().filter(|v| !old_set.contains(v)).collect()
}

fn git_blob(rev: &str) -> Result<String, String> {
    let c = proc::run(&programs::GIT, &["show", rev])?;
    Ok(c.stdout()
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_default())
}

fn collect_live(globs: &[String]) -> Result<(Vec<String>, Vec<String>), String> {
    let staged: Vec<String> = match crate::listing::names(None, &["diff", "--cached", "-z", "--name-only"]) {
        Ok(names) => names,
        Err(crate::listing::Refusal::Failed(_)) => Vec::new(),
        Err(e) => return Err(e.text()),
    };

    let mut added: Vec<String> = Vec::new();
    for f in &staged {
        if !is_gate_file(f, globs) {
            continue;
        }
        let new = git_blob(&format!(":{}", f))?;
        let old = git_blob(&format!("HEAD:{}", f))?;
        for v in added_between(f, &old, &new) {
            if !added.contains(&v) {
                added.push(v);
            }
        }
    }
    // spec: delegation-kit/SPEC.md §Verify after every agent commit — the shell held this as a
    // bash associative array, whose key order is an unreproducible hash order; the compiled
    // form emits the same *set* in a byte order, on the kit-roots cohort's own ruling
    added.sort();
    Ok((staged, added))
}

fn collect_fixture(dir: &str) -> Result<(Vec<String>, Vec<String>), String> {
    if !Path::new(dir).is_dir() {
        return Err(format!("fixture dir not found: {}", dir));
    }
    let read = |name: &str| -> Vec<String> {
        let p = format!("{}/{}", dir, name);
        if !Path::new(&p).is_file() {
            return Vec::new();
        }
        std::fs::read(&p)
            .map(|b| list_lines(&String::from_utf8_lossy(&b)))
            .unwrap_or_default()
    };
    let mut added = read("added-exemptions");
    let path = format!("{}/exception-arrays.sh", dir);
    if let Ok(text) = std::fs::read_to_string(path) { added.extend(extract_exemptions(&text)); }
    Ok((read("staged-files"), added))
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-gate-tamper: {}", e);
            2
        }
    }
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — the gate files the reader derives:
// every registered member's resolved declaration and the gate-sdk library, each spelled against
// the toplevel and dropped where it lies outside it
fn derived_gate_files(
    top: &str,
    gates_dir: &str,
    kit_roots: &[String],
    sdk_root: &str,
    exe_suffix: &str,
) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    let mut add = |abs: &str| {
        if let Some(rel) = walk::rel_under(top, abs) {
            if !rel.is_empty() && !out.iter().any(|o| o == rel) {
                out.push(rel.to_string());
            }
        }
    };
    let list = registry::list_path(gates_dir);
    if Path::new(&list).exists() {
        let text = std::fs::read(&list)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read {}: {}", list, e))?;
        let dirs = registry::resolve_dirs(gates_dir, kit_roots);
        for m in registry::members(&text) {
            if let Some(p) = registry::resolve(m.trim(), &dirs) {
                add(&p);
                // spec: delegation-kit/SPEC.md §Verify after every agent commit — an exec member's
                // target is its implementation, held to meta-layer isolation as its descriptor is,
                // under the path dispatch starts
                if let registry::Substrate::Exec(v) = registry::substrate(&p)? {
                    add(&crate::substrate::started_path(&registry::exec_target(&p, &v), exe_suffix));
                }
            }
        }
    }
    add(&format!("{}/lib/gate.sh", sdk_root.trim_end_matches('/')));
    Ok(out)
}

fn rule(args: &[String]) -> Result<i32, String> {
    let top = walk::toplevel()?;
    let here = walk::cwd()?;
    let kit_roots = walk::kit_roots_under(&top)?;
    let mut globs = walk::knob_array("DELEGATION_KIT_GATE_FILES")?;
    let kit_roots_abs: Vec<String> = kit_roots.iter().map(|r| walk::abs_against(&top, r)).collect();
    let derived = derived_gate_files(
        &top,
        &walk::abs_against(&here, &walk::knob_scalar("GATE_SDK_GATES_DIR")?),
        &kit_roots_abs,
        &walk::abs_against(&here, &walk::sdk_root()),
        std::env::consts::EXE_SUFFIX,
    )?;
    let declared = globs.join(" ");
    for d in &derived {
        if !globs.contains(d) {
            globs.push(d.clone());
        }
    }
    let mut meta = walk::knob_array("DELEGATION_KIT_META_PATHS")?;
    // spec: delegation-kit/SPEC.md §Layout and configuration — a vendored kit's edits are meta-layer by
    // definition: every kit root joins as a `<root>/` prefix the resolved value does not already hold
    // spec: gate-sdk/SPEC.md §Layout and configuration — the prefix is matched against the names
    // `git diff --cached` prints, which are spelled against the toplevel whatever the cwd is, so
    // the root is spelled against it too
    for root in kit_roots {
        let dir = root.trim_end_matches('/').to_string();
        if !dir.is_empty() && !meta.contains(&dir) {
            meta.push(dir);
        }
    }
    // spec: delegation-kit/SPEC.md §Verify after every agent commit — the install manifest joins at
    // the toplevel, since every update and profile move co-stages it with the gate files it rewrites
    let manifest = crate::installer::lock::FILE.to_string();
    if !meta.contains(&manifest) {
        meta.push(manifest);
    }

    let mut corpus = Corpus::Index;
    let mut i = 0usize;
    while i < args.len() {
        let next = match args[i].as_str() {
            // spec: delegation-kit/SPEC.md §Verify after every agent commit — a trailing
            // `--fixture` names the empty directory and refuses rather than hanging
            "--fixture" => Corpus::Fixture(args.get(i + 1).cloned().unwrap_or_default()),
            "--range" => Corpus::Range(args.get(i + 1).cloned().unwrap_or_default()),
            other => return Err(format!("unknown argument: {}", other)),
        };
        if !matches!(corpus, Corpus::Index) {
            return Err("--range and --fixture each select the corpus: give one, once".to_string());
        }
        corpus = next;
        i += 2;
    }

    let (findings, read) = match corpus {
        Corpus::Index => {
            let (staged, added) = collect_live(&globs)?;
            (vec![judge(&staged, &added, &globs, &meta, None)], format!("{} staged path(s)", staged.len()))
        }
        Corpus::Fixture(d) => {
            let (staged, added) = collect_fixture(&d)?;
            (vec![judge(&staged, &added, &globs, &meta, None)], format!("{} staged path(s)", staged.len()))
        }
        Corpus::Range(r) => {
            let (base, head) = parse_range(&r)?;
            let span = format!("{}..{}", commit_id(base)?, commit_id(head)?);
            let commits = rev_list(&["rev-list", "--reverse", "--no-merges", &span])?;
            let merges = rev_list(&["rev-list", "--merges", &span])?.len();
            let mut findings: Vec<Finding> = Vec::new();
            for c in &commits {
                let (changed, added) = collect_commit(c, &globs)?;
                findings.push(judge(&changed, &added, &globs, &meta, Some(c)));
            }
            (findings, format!("{} commit(s) in {}; {} merge(s) not judged", commits.len(), r, merges))
        }
    };
    Ok(verdict(&findings, &declared, &meta, &read, derived.len()))
}

enum Corpus {
    Index,
    Range(String),
    Fixture(String),
}

struct Finding {
    commit: Option<String>,
    viol_a: Vec<String>,
    viol_b: Vec<String>,
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — the range spelling is
// `<base>..<head>`, both sides non-empty; a bare revision and a symmetric `...` are refused
fn parse_range(r: &str) -> Result<(&str, &str), String> {
    let refuse = || format!("--range takes <base>..<head>, got '{}'", r);
    let (base, head) = r.split_once("..").ok_or_else(refuse)?;
    if base.is_empty() || head.is_empty() || head.starts_with('.') || head.contains("..") {
        return Err(refuse());
    }
    Ok((base, head))
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — each side resolves to a commit
// id before any listing reads it, so no caller spelling reaches git as an option
fn commit_id(rev: &str) -> Result<String, String> {
    let refuse = || format!("--range: '{}' names no commit", rev);
    if rev.starts_with('-') {
        return Err(refuse());
    }
    let c = proc::run(&programs::GIT, &["rev-parse", "--verify", "--quiet", &format!("{}^{{commit}}", rev)])?;
    let id = c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()).unwrap_or_default();
    if id.is_empty() {
        return Err(refuse());
    }
    Ok(id)
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — a listing git cannot produce
// refuses rather than degrading to an empty range
fn rev_list(args: &[&str]) -> Result<Vec<String>, String> {
    let c = proc::run(&programs::GIT, args)?;
    match c.stdout() {
        Some(out) => Ok(String::from_utf8_lossy(out).split_whitespace().map(String::from).collect()),
        None => Err(crate::fresh::fail_closed("git-rev-list", c.code())),
    }
}

fn abbreviated(c: &str) -> String {
    proc::run(&programs::GIT, &["rev-parse", "--short", c])
        .ok()
        .and_then(|r| r.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| c.to_string())
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — a commit is judged against its
// first parent and a root commit against the empty tree; a deleted gate file reads as an empty set
fn collect_commit(c: &str, globs: &[String]) -> Result<(Vec<String>, Vec<String>), String> {
    let changed = crate::listing::names(None, &["diff-tree", "-r", "-z", "--no-commit-id", "--name-only", "--root", c])
        .map_err(|e| e.text_or(|code| crate::fresh::fail_closed("git-diff-tree", code)))?;
    let mut added: Vec<String> = Vec::new();
    for f in &changed {
        if !is_gate_file(f, globs) {
            continue;
        }
        let new = git_blob(&format!("{}:{}", c, f))?;
        let old = git_blob(&format!("{}^:{}", c, f))?;
        for v in added_between(f, &old, &new) {
            if !added.contains(&v) {
                added.push(v);
            }
        }
    }
    added.sort();
    Ok((changed, added))
}

fn judge(staged: &[String], added: &[String], globs: &[String], meta: &[String], commit: Option<&String>) -> Finding {
    let mut viol_a: Vec<String> = Vec::new();
    if staged.iter().any(|f| is_gate_file(f, globs)) {
        for f in staged {
            if !is_meta(f, meta) {
                viol_a.push(f.clone());
            }
        }
    }

    let mut viol_b: Vec<String> = Vec::new();
    for e in added {
        if !is_pathlike(e) {
            continue;
        }
        for f in staged {
            if walk::pattern_match(e, f) {
                viol_b.push(format!("{} -> {}", e, f));
            }
        }
    }
    Finding { commit: commit.cloned(), viol_a, viol_b }
}

fn verdict(findings: &[Finding], declared: &str, meta: &[String], read: &str, covered: usize) -> i32 {
    let mut red = false;
    for f in findings {
        let at = f.commit.as_deref().map(|c| format!(" in commit {}", abbreviated(c))).unwrap_or_default();
        if !f.viol_a.is_empty() {
            red = true;
            println!("check-gate-tamper: gate edit not isolated{} — a commit touching a gate file ({} plus every registered gate and the gate library) may touch only meta-layer paths; these co-staged paths are not:", at, declared);
            for v in &f.viol_a {
                println!("  {}", v);
            }
            println!("  help: split the gate change into its own commit (meta-layer paths only: {} and root *.md); land the product change in a separate commit", meta.join(" "));
        }
        if !f.viol_b.is_empty() {
            red = true;
            println!("check-gate-tamper: self-serving exemption{} — a newly added path/glob exemption matches a file staged in the same commit:", at);
            for v in &f.viol_b {
                println!("  {}", v);
            }
            println!("  help: an exemption must not excuse the very change it lands with; add the exemption in a separate commit, or drop the matched file from this commit");
        }
    }
    if red {
        return 1;
    }
    println!("GATE-TAMPER: clean ({}; {} registered gate file(s) covered; gate edits meta-isolated, no self-serving path-exemption)", read, covered);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_exemption_reader_takes_only_a_tagged_array_and_only_its_first_token() {
        let text = "\
# exception-list: the tag
FOO=(
    'a/b.sh'   # trailing prose is not a value
    \"c/*.rs\" d/e
)
BAR=(
    not-tagged
)
";
        assert_eq!(extract_exemptions(text), vec!["a/b.sh", "c/*.rs"]);
    }

    #[test]
    fn a_descriptor_element_is_read_as_an_addition_against_the_head_blob() {
        let head = "# graph: dir=one\n# exception: kept/*.md # permanent: old\n";
        let staged = format!("{}# exception: src/new.rs # until: some-task\n", head);
        assert_eq!(added_between("scripts/check-x.gate", head, &staged), vec!["src/new.rs"]);
        assert!(added_between("scripts/check-x.sh", head, &staged).is_empty());
    }

    #[test]
    fn a_single_line_array_yields_its_first_value() {
        assert_eq!(extract_exemptions("# exception-list:\nFOO=(a b c)\nd\n"), vec!["a"]);
    }

    #[test]
    fn the_derived_gate_files_union_the_registry_and_an_in_tree_library() {
        let scratch = std::env::temp_dir().join(format!("gate-tamper-union-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&scratch);
        let top = walk::normalize_abs(&scratch.join("repo").display().to_string());
        let outside = walk::normalize_abs(&scratch.join("beside").display().to_string());
        let gates = format!("{}/scripts", top);
        let kit = format!("{}/vendor/extra", top);
        let sdk = format!("{}/vendor/gate-sdk", top);
        for d in [&gates, &format!("{}/checks", kit), &format!("{}/lib", sdk), &format!("{}/lib", outside)] {
            std::fs::create_dir_all(d).unwrap();
        }
        std::fs::write(format!("{}/check-local.sh", gates), "").unwrap();
        std::fs::write(format!("{}/checks/check-kit.gate", kit), "").unwrap();
        std::fs::write(format!("{}/checks/check-local.gate", kit), "").unwrap();
        std::fs::write(format!("{}/check-ps.gate", gates), "# exec: check-ps.ps1\n").unwrap();
        std::fs::write(format!("{}/check-ps.ps1", gates), "").unwrap();
        std::fs::write(format!("{}/check-bin.gate", gates), "# exec: check-bin\n").unwrap();

        assert_eq!(
            derived_gate_files(&top, &gates, std::slice::from_ref(&kit), &sdk, "").unwrap(),
            vec!["vendor/gate-sdk/lib/gate.sh"]
        );

        std::fs::write(
            format!("{}/gates.list", gates),
            "# comment\ncheck-local\ncheck-kit\ncheck-nowhere\ncheck-ps\ncheck-bin\n",
        )
        .unwrap();
        assert_eq!(
            derived_gate_files(&top, &gates, std::slice::from_ref(&kit), &sdk, "").unwrap(),
            vec![
                "scripts/check-local.sh",
                "vendor/extra/checks/check-kit.gate",
                "scripts/check-ps.gate",
                "scripts/check-ps.ps1",
                "scripts/check-bin.gate",
                "scripts/check-bin",
                "vendor/gate-sdk/lib/gate.sh"
            ]
        );

        // spec: delegation-kit/SPEC.md §Verify after every agent commit — on a suffixed host the set
        // holds the executable dispatch starts, and a PowerShell target as named
        let suffixed = derived_gate_files(&top, &gates, std::slice::from_ref(&kit), &sdk, ".exe").unwrap();
        assert!(suffixed.iter().any(|f| f == "scripts/check-bin.exe") && !suffixed.iter().any(|f| f == "scripts/check-bin"), "{:?}", suffixed);
        assert!(suffixed.iter().any(|f| f == "scripts/check-ps.ps1"), "{:?}", suffixed);

        assert_eq!(
            derived_gate_files(&top, &gates, std::slice::from_ref(&kit), &outside, "").unwrap(),
            vec![
                "scripts/check-local.sh",
                "vendor/extra/checks/check-kit.gate",
                "scripts/check-ps.gate",
                "scripts/check-ps.ps1",
                "scripts/check-bin.gate",
                "scripts/check-bin"
            ]
        );
        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn a_range_is_two_non_empty_sides_joined_by_two_dots() {
        assert_eq!(parse_range("main..topic").unwrap(), ("main", "topic"));
        assert_eq!(parse_range("HEAD~3..HEAD").unwrap(), ("HEAD~3", "HEAD"));
        for bad in ["", "HEAD", "..HEAD", "HEAD..", "a...b", "a..b..c", ".."] {
            assert!(parse_range(bad).is_err(), "{:?} was accepted", bad);
        }
    }

    #[test]
    fn a_side_spelled_as_an_option_names_no_commit() {
        assert!(commit_id("--output=x").unwrap_err().contains("names no commit"));
    }

    #[test]
    fn the_meta_and_pathlike_predicates_match_the_shell_globs() {
        let meta = vec!["scripts/".to_string(), ".workflow/".to_string()];
        assert!(is_meta("scripts/x.sh", &meta));
        assert!(is_meta("README.md", &meta));
        assert!(!is_meta("docs/README.md", &meta));
        assert!(!is_meta("src/lib.rs", &meta));
        assert!(is_pathlike("a/b"));
        assert!(is_pathlike("*.sh"));
        assert!(is_pathlike("x?y"));
        assert!(is_pathlike("[abc]"));
        assert!(!is_pathlike("plain-token"));
    }
}
