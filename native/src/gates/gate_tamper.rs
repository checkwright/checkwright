// spec: delegation-kit/SPEC.md §Verify after every agent commit — a gate-weakening commit is
// blocked by shape (A gate edits stay meta-isolated; B a new path-exemption can't excuse a
// co-staged file)
use crate::proc;
use crate::walk;
use std::path::Path;

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\u{b}' || c == '\u{c}'
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — a meta-layer path is one of
// the configured prefixes, or a root-level `*.md`
fn is_meta(p: &str, meta: &[String]) -> bool {
    if meta.iter().any(|pre| p.starts_with(pre.as_str())) {
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

fn strip_comment(line: &str) -> &str {
    match line.find('#') {
        Some(i) => &line[..i],
        None => line,
    }
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — the exemption reader: a
// `# exception-list:` tag arms the next `NAME=(` line, and each element inside the array's
// parenthesis balance yields its first whitespace-delimited token, unquoted
fn extract_exemptions(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut tag = false;
    let mut inarr = false;
    let mut bal: i64 = 0;
    for raw in text.lines() {
        let t = raw.trim_start_matches(is_space);
        if let Some(rest) = t.strip_prefix('#') {
            if rest.trim_start_matches(is_space).starts_with("exception-list:") {
                tag = true;
                continue;
            }
        }
        if tag && opens_array(t) {
            tag = false;
            inarr = true;
            let (ob, cb, _) = paren_split(strip_comment(raw));
            bal = ob - cb;
            if bal <= 0 {
                inarr = false;
            }
            continue;
        }
        if !inarr {
            continue;
        }
        let (ob, cb, stripped) = paren_split(strip_comment(raw));
        let v = stripped.trim_start_matches(is_space);
        let v = match v.find(is_space) {
            Some(i) => &v[..i],
            None => v,
        };
        let v: String = v.chars().filter(|c| *c != '\'' && *c != '"').collect();
        if !v.is_empty() {
            out.push(v);
        }
        bal += ob - cb;
        if bal <= 0 {
            inarr = false;
        }
    }
    out
}

// spec: delegation-kit/SPEC.md §Verify after every agent commit — awk's `gsub` both counts and
// removes, so the value a line yields is already parenthesis-free before its first token is taken
fn paren_split(line: &str) -> (i64, i64, String) {
    let ob = line.matches('(').count() as i64;
    let cb = line.matches(')').count() as i64;
    (
        ob,
        cb,
        line.chars().filter(|c| *c != '(' && *c != ')').collect(),
    )
}

fn opens_array(t: &str) -> bool {
    let b = t.as_bytes();
    if b.is_empty() || !(b[0].is_ascii_alphabetic() || b[0] == b'_') {
        return false;
    }
    let mut i = 1usize;
    while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
        i += 1;
    }
    b[i..].starts_with(b"=(")
}

fn git_blob(rev: &str) -> Result<String, String> {
    let c = proc::run("git", &["show", rev])?;
    Ok(c.stdout()
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_default())
}

fn collect_live(globs: &[String]) -> Result<(Vec<String>, Vec<String>), String> {
    let c = proc::run("git", &["diff", "--cached", "--name-only"])?;
    let staged: Vec<String> = c
        .stdout()
        .map(|b| {
            String::from_utf8_lossy(b)
                .lines()
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let mut added: Vec<String> = Vec::new();
    for f in &staged {
        if !is_gate_file(f, globs) {
            continue;
        }
        let new = git_blob(&format!(":{}", f))?;
        let old = git_blob(&format!("HEAD:{}", f))?;
        let old_set = extract_exemptions(&old);
        for v in extract_exemptions(&new) {
            if !old_set.contains(&v) && !added.contains(&v) {
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
    Ok((read("staged-files"), read("added-exemptions")))
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

fn rule(args: &[String]) -> Result<i32, String> {
    let globs = walk::knob_array("DELEGATION_KIT_GATE_FILES")?;
    let meta = walk::knob_array("DELEGATION_KIT_META_PATHS")?;

    let mut fixture: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == "--fixture" {
            // spec: delegation-kit/SPEC.md §Verify after every agent commit — a trailing
            // `--fixture` names the empty directory and refuses rather than hanging
            fixture = Some(args.get(i + 1).cloned().unwrap_or_default());
            i += 2;
            continue;
        }
        return Err(format!("unknown argument: {}", args[i]));
    }

    let (staged, added) = match &fixture {
        Some(d) => collect_fixture(d)?,
        None => collect_live(&globs)?,
    };

    let mut viol_a: Vec<String> = Vec::new();
    if staged.iter().any(|f| is_gate_file(f, &globs)) {
        for f in &staged {
            if !is_meta(f, &meta) {
                viol_a.push(f.clone());
            }
        }
    }

    let mut viol_b: Vec<String> = Vec::new();
    for e in &added {
        if !is_pathlike(e) {
            continue;
        }
        for f in &staged {
            if walk::pattern_match(e, f) {
                viol_b.push(format!("{} -> {}", e, f));
            }
        }
    }

    if !viol_a.is_empty() || !viol_b.is_empty() {
        if !viol_a.is_empty() {
            println!("check-gate-tamper: gate edit not isolated — a commit touching a gate file ({}) may touch only meta-layer paths; these co-staged paths are not:", globs.join(" "));
            for v in &viol_a {
                println!("  {}", v);
            }
            println!("  help: split the gate change into its own commit (meta-layer paths only: {} and root *.md); land the product change in a separate commit", meta.join(" "));
        }
        if !viol_b.is_empty() {
            println!("check-gate-tamper: self-serving exemption — a newly added path/glob exemption matches a file staged in the same commit:");
            for v in &viol_b {
                println!("  {}", v);
            }
            println!("  help: an exemption must not excuse the very change it lands with; add the exemption in a separate commit, or drop the matched file from this commit");
        }
        return Ok(1);
    }

    println!("GATE-TAMPER: clean ({} staged path(s); gate edits meta-isolated, no self-serving path-exemption)", staged.len());
    Ok(0)
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

    // spec: delegation-kit/SPEC.md §Verify after every agent commit — a single-line array
    // closes its own balance, so the tag arms nothing and no value is read
    #[test]
    fn a_single_line_array_yields_nothing() {
        assert!(extract_exemptions("# exception-list:\nFOO=(a b c)\nd\n").is_empty());
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
