// spec: gate-sdk/SPEC.md §check-path-dialect — every platform-native path producer in the two
// corpora converts at its own point of production, or records at the site why it does not
use crate::proc;
use crate::walk;
use std::path::Path;

const NAME: &str = "check-path-dialect";
const CITATION: &str = "The path-dialect contract";

// spec: gate-sdk/SPEC.md §The path-dialect contract — the scanner's own vocabulary is a recorded
// verdict rather than an uncrossed producer: a gate that reds on the forms it hunts is unwritable
const GIT_FLAGS: &[&str] = &["--show-toplevel", "--git-dir", "--git-common-dir"];
// spec: gate-sdk/SPEC.md §The path-dialect contract — the same verdict over the Rust half's forms
const RUST_FORMS: &[&str] = &["env::current_dir(", "fs::canonicalize(", r#"env!("CARGO_MANIFEST_DIR")"#];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: gate-sdk/SPEC.md §check-path-dialect — a line's code half and its comment half, kept apart
// once per line: a producer named in prose is not an occurrence, and a recorded verdict is read out
// of the same split rather than by a second pass
struct Line {
    code: String,
    comment: String,
}

fn cut(line: &str, at: Option<usize>) -> Line {
    match at {
        Some(i) => Line {
            code: line[..i].to_string(),
            comment: line[i..].to_string(),
        },
        None => Line {
            code: line.to_string(),
            comment: String::new(),
        },
    }
}

// spec: gate-sdk/SPEC.md §check-path-dialect — an unquoted '#' opening a word. A '#' inside quotes
// or mid-word is parameter expansion or content, never a comment, so neither ends the code half.
fn shell_split(line: &str) -> Line {
    let b = line.as_bytes();
    let (mut i, mut sq, mut dq) = (0usize, false, false);
    while i < b.len() {
        let c = b[i];
        if c == b'\\' && !sq {
            i += 2;
            continue;
        }
        if c == b'\'' && !dq {
            sq = !sq;
        } else if c == b'"' && !sq {
            dq = !dq;
        } else if c == b'#'
            && !sq
            && !dq
            && (i == 0 || matches!(b[i - 1], b' ' | b'\t' | b';' | b'(' | b'&' | b'|'))
        {
            return cut(line, Some(i));
        }
        i += 1;
    }
    cut(line, None)
}

// spec: gate-sdk/SPEC.md §check-path-dialect — '//' outside a string literal. A block comment is
// deliberately not recognized: leaving one unstripped errs toward red, which is the safe direction
// for a gate, and no source in this corpus writes a producer inside one.
fn rust_split(line: &str) -> Line {
    let b = line.as_bytes();
    let (mut i, mut dq) = (0usize, false);
    while i < b.len() {
        let c = b[i];
        if c == b'\\' && dq {
            i += 2;
            continue;
        }
        if c == b'"' {
            dq = !dq;
        } else if c == b'/' && !dq && i + 1 < b.len() && b[i + 1] == b'/' {
            return cut(line, Some(i));
        }
        i += 1;
    }
    cut(line, None)
}

fn split_file(text: &str, shell: bool) -> Vec<Line> {
    text.lines()
        .map(|l| if shell { shell_split(l) } else { rust_split(l) })
        .collect()
}

fn boundary(c: Option<u8>) -> bool {
    matches!(
        c,
        None | Some(b' ') | Some(b'\t') | Some(b'"') | Some(b'\'') | Some(b')') | Some(b';')
            | Some(b'|') | Some(b'&') | Some(b',')
    )
}

fn hits(code: &str, needle: &str, token: bool) -> Vec<usize> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(off) = code[from..].find(needle) {
        let at = from + off;
        let end = at + needle.len();
        if !token || boundary(code.as_bytes().get(end).copied()) {
            out.push(at);
        }
        from = end;
    }
    out
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the value-bound test on the shell side: a stdout
// redirected to the void binds nothing, so the occurrence produces no root. Scoped to the
// occurrence's own command substitution so a redirect belonging to a later command cannot clear it.
fn stdout_discarded(code: &str, at: usize) -> bool {
    let b = code.as_bytes();
    let mut depth = 0i32;
    let mut i = at;
    while i < b.len() {
        match b[i] {
            b'(' => depth += 1,
            b')' => {
                if depth == 0 {
                    return false;
                }
                depth -= 1;
            }
            _ => {}
        }
        if code[i..].starts_with("/dev/null") && redirects_stdout(code, i) {
            return true;
        }
        i += 1;
    }
    false
}

// spec: gate-sdk/SPEC.md §check-path-dialect — which descriptor the redirect names: a bare '>' and
// '1>' are stdout, '&>' is both, and '2>' is the stderr hedge every crossed site already carries
fn redirects_stdout(code: &str, at: usize) -> bool {
    let b = code.as_bytes();
    let mut j = at;
    while j > 0 && matches!(b[j - 1], b' ' | b'\t') {
        j -= 1;
    }
    if j == 0 || b[j - 1] != b'>' {
        return false;
    }
    while j > 0 && b[j - 1] == b'>' {
        j -= 1;
    }
    if j == 0 {
        return true;
    }
    let p = b[j - 1];
    if p == b'&' {
        return true;
    }
    if p.is_ascii_digit() {
        return p == b'1';
    }
    true
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the shell clearance: the substitution is the direct
// argument of a `cd`, which consumes the value as a chdir rather than as a tree-internal string
fn in_cd_position(code: &str, at: usize) -> bool {
    let prefix = &code[..at];
    let open = match prefix.rfind("$(") {
        Some(i) => i,
        None => return false,
    };
    let before = prefix[..open].trim_end_matches(['"', '\'', ' ', '\t']);
    if !before.ends_with("cd") {
        return false;
    }
    let b = before.as_bytes();
    b.len() == 2 || matches!(b[b.len() - 3], b' ' | b'\t' | b';' | b'&' | b'|' | b'{' | b'(')
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the Rust clearance: the occurrence is the direct
// argument of a Path constructor, so the value never becomes a string and std::path carries dialect
fn path_typed(code: &str, at: usize) -> bool {
    let prefix = &code[..at];
    prefix.ends_with("Path::new(") || prefix.ends_with("PathBuf::from(")
}

// spec: gate-sdk/SPEC.md §check-path-dialect — a recorded verdict is canon-kit's `spec:` one-line
// binding, read on the occurrence's own trailing comment or the contiguous comment run above it
fn recorded_verdict(lines: &[Line], idx: usize) -> bool {
    let cites = |c: &str| c.contains("spec:") && c.contains(CITATION);
    if cites(&lines[idx].comment) {
        return true;
    }
    let mut i = idx;
    while i > 0 {
        i -= 1;
        if !lines[i].code.trim().is_empty() || lines[i].comment.is_empty() {
            return false;
        }
        if cites(&lines[i].comment) {
            return true;
        }
    }
    false
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the read-back arm, anchored to an already-cleared
// occurrence: the first statement after the `cd`, comments and blank lines skipped
fn bare_pwd_readback(lines: &[Line], idx: usize) -> Option<usize> {
    let mut j = idx + 1;
    while j < lines.len() && lines[j].code.trim().is_empty() {
        j += 1;
    }
    if j >= lines.len() {
        return None;
    }
    let c = &lines[j].code;
    if c.contains("$(pwd)") || c.contains("`pwd`") {
        Some(j)
    } else {
        None
    }
}

struct Tally {
    cd: usize,
    typed: usize,
    crosser: usize,
    verdict: usize,
    probe: usize,
    total: usize,
}

fn scan_shell(path: &str, text: &str, t: &mut Tally, findings: &mut Vec<String>) {
    let lines = split_file(text, true);
    for (idx, line) in lines.iter().enumerate() {
        if !line.code.contains("rev-parse") {
            continue;
        }
        for flag in GIT_FLAGS {
            for at in hits(&line.code, flag, true) {
                t.total += 1;
                if stdout_discarded(&line.code, at) {
                    t.probe += 1;
                    continue;
                }
                if in_cd_position(&line.code, at) {
                    t.cd += 1;
                    if let Some(j) = bare_pwd_readback(&lines, idx) {
                        findings.push(format!(
                            "{}:{} — the crossed `cd` at line {} is read back with a logical `pwd`, which prints an absolute argument straight back and converts nothing",
                            path, j + 1, idx + 1
                        ));
                    }
                    continue;
                }
                if recorded_verdict(&lines, idx) {
                    t.verdict += 1;
                    continue;
                }
                findings.push(format!(
                    "{}:{} — `git rev-parse {}` binds a root that is never crossed: the substitution is not the direct argument of a `cd`, and the site records no verdict",
                    path, idx + 1, flag
                ));
            }
        }
    }
}

fn scan_rust(path: &str, text: &str, is_crosser: bool, t: &mut Tally, findings: &mut Vec<String>) {
    let lines = split_file(text, false);
    let mut forms: Vec<&str> = RUST_FORMS.to_vec();
    forms.push(GIT_FLAGS[0]);
    for (idx, line) in lines.iter().enumerate() {
        for form in &forms {
            for at in hits(&line.code, form, false) {
                t.total += 1;
                if is_crosser {
                    t.crosser += 1;
                    continue;
                }
                if path_typed(&line.code, at) {
                    t.typed += 1;
                    continue;
                }
                if recorded_verdict(&lines, idx) {
                    t.verdict += 1;
                    continue;
                }
                findings.push(format!(
                    "{}:{} — `{}` is a platform-native producer outside the crate's crosser: it is not `Path`-typed and the site records no verdict",
                    path, idx + 1, form
                ));
            }
        }
    }
}

fn read(p: &Path) -> Result<String, String> {
    std::fs::read(p)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", p.display(), e))
}

fn rule(_args: &[String]) -> Result<i32, String> {
    // spec: gate-sdk/SPEC.md §Fail-closed contract — the shell corpus degrades to empty outside a
    // work tree, so the repository is probed first and an absent one refuses rather than reporting
    // a clean scan over nothing
    let inside = proc::run("git", &["rev-parse", "--git-dir"])
        .map(|c| c.stdout().is_some())
        .unwrap_or(false);
    if !inside {
        return Err("not a git repository — the tracked shell corpus cannot be resolved".into());
    }

    let mut t = Tally {
        cd: 0,
        typed: 0,
        crosser: 0,
        verdict: 0,
        probe: 0,
        total: 0,
    };
    let mut findings: Vec<String> = Vec::new();

    let shell = walk::tracked_shell_tree()?;
    for f in &shell {
        let text = read(Path::new(f))?;
        scan_shell(f, &text, &mut t, &mut findings);
    }

    let src = walk::knob_scalar("GATE_SDK_NATIVE_SRC")?;
    let prune = walk::prune_dirs()?;
    let crosser = format!("{}/walk.rs", src.trim_end_matches('/'));
    let mut rust_files = 0usize;
    if Path::new(&src).is_dir() {
        for f in walk::find_with_prune(Path::new(&src), &|n| prune.iter().any(|d| d == n))? {
            let p = f.display().to_string();
            if !p.ends_with(".rs") {
                continue;
            }
            rust_files += 1;
            let text = read(&f)?;
            scan_rust(&p, &text, p == crosser, &mut t, &mut findings);
        }
    }

    if !findings.is_empty() {
        println!("PATH-DIALECT: {} violation(s):", findings.len());
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: cross at the producer — hand the substitution to `cd` directly, and bind a root");
        println!("        from it only where the site reads one, with `pwd -P` (gate-sdk/SPEC.md §The path-dialect contract)");
        println!("  help: in the crate, route the producer through native/src/walk.rs, or wrap the value in");
        println!("        Path::new(...) / PathBuf::from(...) so std::path carries the dialect");
        println!("  help: a site that deliberately does not cross records the verdict at the site — an adjacent");
        println!("        `spec:` comment citing gate-sdk/SPEC.md §The path-dialect contract");
        return Ok(1);
    }
    println!(
        "PATH-DIALECT: clean ({} shell file(s), {} Rust file(s) scanned; {} producer occurrence(s) — {} in `cd` position, {} `Path`-typed, {} inside the crate's crosser, {} by recorded verdict, {} presence probe(s) binding no value)",
        shell.len(), rust_files, t.total, t.cd, t.typed, t.crosser, t.verdict, t.probe
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-path-dialect — the comment split is the comment rule's whole
    // mechanism, and a '#' that is parameter expansion must not end the code half
    #[test]
    fn a_shell_comment_ends_the_code_half_and_an_expansion_does_not() {
        assert_eq!(shell_split("a # b").code, "a ");
        assert_eq!(shell_split("x=\"${V#pre}\"").comment, "");
        assert_eq!(shell_split("echo \"a # b\"").comment, "");
        assert_eq!(shell_split("# whole line").code, "");
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — a test line is composed from the scanner's own
    // vocabulary rather than re-spelling a producer: one literal cannot drift from the roster it is
    // meant to exercise, and the module keeps a single site for the forms it hunts
    fn producer(flag: usize) -> String {
        format!("git rev-parse {}", GIT_FLAGS[flag])
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — the descriptor the redirect names decides
    // whether a value was bound, and the stderr hedge every crossed site carries must not clear it
    #[test]
    fn only_a_stdout_redirect_to_the_void_discards() {
        for tail in [" >/dev/null 2>&1", " &>/dev/null", " 1>/dev/null"] {
            let line = format!("{}{}", producer(1), tail);
            assert!(stdout_discarded(&line, 0), "not discarded: {}", line);
        }
        let hedged = format!("{} 2>/dev/null", producer(0));
        assert!(!stdout_discarded(&hedged, 0));
        let later = format!("cd \"$({} 2>/dev/null)\" && x >/dev/null", producer(0));
        assert!(!stdout_discarded(
            &later,
            later.find(GIT_FLAGS[0]).expect("flag present")
        ));
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — both idiom forms clear and the pre-migration
    // shape does not, which is the whole distinction delta 4's sweep bought
    #[test]
    fn cd_position_separates_the_idiom_from_the_bound_root() {
        let at = |s: &str| s.find(GIT_FLAGS[0]).expect("flag present");
        let two = format!("cd \"$({} 2>/dev/null)\" || exit 1", producer(0));
        assert!(in_cd_position(&two, at(&two)));
        let sub = format!(
            "ROOT=\"$( {{ cd \"$({})\" && pwd -P; }} 2>/dev/null )\"",
            producer(0)
        );
        assert!(in_cd_position(&sub, at(&sub)));
        let bound = format!("REPO_ROOT=\"$({} 2>/dev/null || pwd)\"", producer(0));
        assert!(!in_cd_position(&bound, at(&bound)));
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — the read-back arm skips comments and blanks and
    // stops at the first statement, so a `|| pwd` hedge on the `cd`'s own line is not its subject
    #[test]
    fn the_read_back_arm_reads_the_first_statement_after_the_cd() {
        let ok = split_file("cd \"$(p)\" || exit\n# note\n\nR=\"$(pwd -P)\"\n", true);
        assert!(bare_pwd_readback(&ok, 0).is_none());
        let bad = split_file("cd \"$(p)\" || exit\n# note\nR=\"$(pwd)\"\n", true);
        assert_eq!(bare_pwd_readback(&bad, 0), Some(2));
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — a verdict is read off the contiguous comment run
    // only, so a citation elsewhere in the file clears nothing
    #[test]
    fn a_recorded_verdict_binds_to_its_own_site() {
        let near = split_file("// spec: x §The path-dialect contract\nlet a = 1;\n", false);
        assert!(recorded_verdict(&near, 1));
        let far = split_file("// spec: x §The path-dialect contract\n\nlet a = 1;\n", false);
        assert!(!recorded_verdict(&far, 2));
    }
}
