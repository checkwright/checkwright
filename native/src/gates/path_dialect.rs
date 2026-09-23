// spec: gate-sdk/SPEC.md §check-path-dialect — every platform-native path producer in the two
// corpora converts at its own point of production, or records at the site why it does not
use crate::{proc, programs};
use crate::walk;
use std::path::Path;

const NAME: &str = "check-path-dialect";
const CITATION: &str = "The path-dialect contract";

// spec: gate-sdk/SPEC.md §The path-dialect contract — the scanner's own vocabulary is a recorded
// verdict rather than an uncrossed producer: a gate that reds on the forms it hunts is unwritable
const GIT_FLAGS: &[&str] = &["--show-toplevel", "--git-dir", "--git-common-dir"];
// spec: gate-sdk/SPEC.md §The path-dialect contract — the same verdict over the Rust half's forms
const RUST_FORMS: &[&str] = &["env::current_dir(", "fs::canonicalize(", r#"env!("CARGO_MANIFEST_DIR")"#];
// path-dialect-exempt: the locality scanner's own vocabulary — a roster of form literals, not a
// path comparison, and the `spec:` verdict above cannot carry it because a recorded verdict does
// not clear this arm (gate-sdk/SPEC.md §check-path-dialect)
// spec: gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure — testing absoluteness
// from a path's text: the leading-separator test the clause names wrong, and `std::path`'s own
// answer, which disagrees with `walk::path_root` on a separator-rooted path under Windows
const ABS_FORMS: &[&str] = &["starts_with('/')", r#"starts_with("/")"#, r"starts_with('\\')", r#"starts_with("\\")"#, ".is_absolute()"];
// path-dialect-exempt: the same roster for the containment primitive, exempt on the same ground
// spec: gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure — composing a prefix to
// test containment: the bare trailing-slash composition, whose only consumer is a prefix test, and
// a `format!` handed straight to one
const PREFIX_FORM: &str = r#"format!("{}/""#;
const PREFIX_TESTS: &[&str] = &["starts_with(&format!(", "strip_prefix(&format!("];
// path-dialect-exempt: the token this gate reads, named once here rather than spelled at each read
const EXEMPT: &str = "path-dialect-exempt:";
// spec: gate-sdk/SPEC.md §check-path-dialect — the cwd-anchor arm's sole clearance, matched as a
// line's whole code half
const ANCHOR: &str = r#"cd "$(pwd -P)""#;

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
pub(crate) struct Line {
    pub(crate) code: String,
    pub(crate) comment: String,
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
pub(crate) fn shell_split(line: &str) -> Line {
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

// spec: gate-sdk/SPEC.md §check-path-dialect — the declaration window both at-site tokens ride:
// the occurrence's own trailing comment, else the contiguous comment run above it
fn window<'a>(lines: &'a [Line], idx: usize, reads: &dyn Fn(&str) -> bool) -> Option<&'a str> {
    if reads(&lines[idx].comment) {
        return Some(&lines[idx].comment);
    }
    let mut i = idx;
    while i > 0 {
        i -= 1;
        if !lines[i].code.trim().is_empty() || lines[i].comment.is_empty() {
            return None;
        }
        if reads(&lines[i].comment) {
            return Some(&lines[i].comment);
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §check-path-dialect — a recorded verdict is canon-kit's `spec:` one-line
// binding citing this contract, which clears the producer arm and never the locality one
fn recorded_verdict(lines: &[Line], idx: usize) -> bool {
    window(lines, idx, &|c: &str| c.contains("spec:") && c.contains(CITATION)).is_some()
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the namespace declaration, whose reason is
// mandatory: `None` is no declaration in the window and `Some(false)` one whose reason is empty,
// which declares nothing and is malformed rather than clearing
fn namespace_declared(lines: &[Line], idx: usize) -> Option<bool> {
    window(lines, idx, &|c: &str| c.contains(EXEMPT))
        .map(|c| c.split_once(EXEMPT).map(|(_, r)| !r.trim().is_empty()).unwrap_or(false))
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
    prim: usize,
    local: usize,
    namespace: usize,
    anchored: usize,
    shell_abs: usize,
    shell_rooted: usize,
    shell_namespace: usize,
}

// spec: gate-sdk/SPEC.md §check-path-dialect — which containment spelling a line carries: the bare
// trailing-slash composition, else a `format!` handed straight to a prefix test whose template
// composes a separator. A `{}=` or `{} ` template splits a key or a word, never a path.
fn prefix_spelling(code: &str) -> Option<&'static str> {
    if code.contains(PREFIX_FORM) {
        return Some(PREFIX_FORM);
    }
    PREFIX_TESTS.iter().copied().find(|t| {
        hits(code, t, false)
            .iter()
            .any(|at| template(code, at + t.len()).is_some_and(|s| s.contains('/')))
    })
}

// spec: gate-sdk/SPEC.md §check-path-dialect — a `format!`'s own template, the first string literal
// after the macro's open paren
fn template(code: &str, from: usize) -> Option<&str> {
    let rest = code.get(from..)?;
    let open = rest.find('"')? + 1;
    let end = rest.get(open..)?.find('"')? + open;
    rest.get(open..end)
}

// spec: gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure — the locality arm: the
// three text-level primitives are the crate speller's, and a module outside it reaches each through
// a named `walk` helper or declares its value out of the filesystem namespace at the site
fn scan_locality(path: &str, text: &str, is_crosser: bool, t: &mut Tally, findings: &mut Vec<String>) {
    let lines = split_file(text, false);
    for (idx, line) in lines.iter().enumerate() {
        let owners = [
            (
                ABS_FORMS.iter().copied().find(|f| line.code.contains(f)),
                "walk::path_root — or walk::abs_against, where the site's other arm joins onto an anchor",
            ),
            (
                prefix_spelling(&line.code),
                "walk::under / walk::at_or_under / walk::rel_under",
            ),
        ];
        for (form, owner) in owners {
            let form = match form {
                Some(f) => f,
                None => continue,
            };
            t.prim += 1;
            if is_crosser {
                t.local += 1;
                continue;
            }
            match namespace_declared(&lines, idx) {
                Some(true) => t.namespace += 1,
                Some(false) => findings.push(format!(
                    "{}:{} — `{}` carries a `{}` declaration whose reason is empty, so nothing is declared",
                    path, idx + 1, form, EXEMPT
                )),
                None => findings.push(format!(
                    "{}:{} — `{}` spells a text-level path primitive outside the crate's speller: route it through {}",
                    path, idx + 1, form, owner
                )),
            }
        }
    }
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

// spec: gate-sdk/SPEC.md §check-path-dialect — a root binding on one line: a name assigned a
// substitution that `cd`s and captures `pwd`, from `BASH_SOURCE` or after a toplevel `cd`;
// `Some(true)` marks the `BASH_SOURCE` kind
fn root_binding(code: &str) -> Option<(String, bool)> {
    let mut s = code.trim_start();
    for p in ["export ", "local ", "readonly "] {
        if let Some(r) = s.strip_prefix(p) {
            s = r.trim_start();
        }
    }
    let eq = s.find('=')?;
    let name = &s[..eq];
    let first = name.chars().next()?;
    if !(first.is_ascii_alphabetic() || first == '_') || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    let value = s[eq + 1..].trim_start_matches('"');
    if !value.starts_with("$(") || !value.contains("cd ") || !value.contains("pwd") {
        return None;
    }
    let from_source = value.contains("BASH_SOURCE");
    if from_source || value.contains(GIT_FLAGS[0]) {
        Some((name.to_string(), from_source))
    } else {
        None
    }
}

// spec: gate-sdk/SPEC.md §check-path-dialect — string arithmetic on a root: a join inside a word,
// quoted or braced, or a suffix or prefix strip
fn composes(code: &str, name: &str) -> bool {
    [
        format!("${}/", name),
        format!("${{{}}}/", name),
        format!("\"${}\"/", name),
        format!("\"${{{}}}\"/", name),
        format!("${{{}%", name),
        format!("${{{}#", name),
    ]
    .iter()
    .any(|f| code.contains(f.as_str()))
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the cwd-anchor arm: a file binding two roots, one from
// `BASH_SOURCE`, that composes either by string arithmetic anchors its cwd before the first binding
fn scan_anchor(path: &str, text: &str, t: &mut Tally, findings: &mut Vec<String>) {
    let lines = split_file(text, true);
    let mut anchored = false;
    let mut first: Option<usize> = None;
    let mut roots: Vec<(String, bool)> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if first.is_none() && line.code.trim() == ANCHOR {
            anchored = true;
        }
        if let Some(r) = root_binding(&line.code) {
            first.get_or_insert(idx);
            if !roots.iter().any(|(n, _)| *n == r.0) {
                roots.push(r);
            }
        }
    }
    let first = match first {
        Some(f) if roots.len() >= 2 && roots.iter().any(|(_, s)| *s) => f,
        _ => return,
    };
    if !lines.iter().any(|l| roots.iter().any(|(n, _)| composes(&l.code, n))) {
        return;
    }
    if anchored {
        t.anchored += 1;
        return;
    }
    findings.push(format!(
        "{}:{} — the file binds {} roots, one from `BASH_SOURCE`, and composes them by string arithmetic with no cwd anchor: anchor the cwd with `{}` before deriving the first root",
        path, first + 1, roots.len(), ANCHOR
    ));
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the rooted glob a single-dialect test spells, bare or
// with its separator quoted
const SHELL_ROOTS: &[&str] = &["/*", "\"/\"*", "'/'*"];
// spec: gate-sdk/SPEC.md §check-path-dialect — the one function whose body is the predicate itself
const PREDICATE_HEAD: &str = "gate_path_rooted() {";

// spec: gate-sdk/SPEC.md §check-path-dialect — a `[[ ]]` glob test whose right operand is exactly a
// rooted glob, closed by a blank, `]`, `;` or the end of the line
fn shell_glob_test(code: &str) -> Option<&'static str> {
    if !code.contains("[[") {
        return None;
    }
    for op in ["==", "!=", " ="] {
        for at in hits(code, op, false) {
            if op == " =" && code.as_bytes().get(at + 2) == Some(&b'=') {
                continue;
            }
            let rest = code[at + op.len()..].trim_start_matches([' ', '\t']);
            for root in SHELL_ROOTS {
                if let Some(after) = rest.strip_prefix(root) {
                    if after.is_empty() || after.starts_with([' ', '\t', ']', ';']) {
                        return Some("[[ … == /* ]]");
                    }
                }
            }
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §check-path-dialect — a `case` alternative that is exactly a rooted glob:
// opened by the line's start, a blank, `(` or `|`, and closed by optional blanks then `)` or `|`
fn shell_case_arm(code: &str) -> Option<&'static str> {
    for root in SHELL_ROOTS {
        for at in hits(code, root, false) {
            let opened = at == 0 || matches!(code.as_bytes()[at - 1], b' ' | b'\t' | b'(' | b'|');
            let after = code[at + root.len()..].trim_start_matches([' ', '\t']);
            if opened && (after.starts_with(')') || after.starts_with('|')) {
                return Some("case … /*)");
            }
        }
    }
    None
}

// spec: gate-sdk/SPEC.md §check-path-dialect — the locality arm's shell half: an absoluteness test
// spelled in one dialect is a finding outside gate_path_rooted's own body, unless the site declares
// its value out of the filesystem namespace
fn scan_shell_locality(path: &str, text: &str, t: &mut Tally, findings: &mut Vec<String>) {
    let lines = split_file(text, true);
    let mut in_predicate = false;
    for (idx, line) in lines.iter().enumerate() {
        let code = line.code.trim();
        if code.starts_with(PREDICATE_HEAD) {
            in_predicate = true;
        }
        let form = shell_glob_test(&line.code).or_else(|| shell_case_arm(&line.code));
        if let Some(form) = form {
            t.shell_abs += 1;
            if in_predicate {
                t.shell_rooted += 1;
            } else {
                match namespace_declared(&lines, idx) {
                    Some(true) => t.shell_namespace += 1,
                    Some(false) => findings.push(format!(
                        "{}:{} — `{}` carries a `{}` declaration whose reason is empty, so nothing is declared",
                        path, idx + 1, form, EXEMPT
                    )),
                    None => findings.push(format!(
                        "{}:{} — `{}` tests absoluteness in one dialect, so a drive-rooted path reads as relative: route it through gate_path_rooted (gate-sdk/lib/gate.sh)",
                        path, idx + 1, form
                    )),
                }
            }
        }
        if in_predicate && code == "}" {
            in_predicate = false;
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
    let inside = proc::run(&programs::GIT, &["rev-parse", "--git-dir"])
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
        prim: 0,
        local: 0,
        namespace: 0,
        anchored: 0,
        shell_abs: 0,
        shell_rooted: 0,
        shell_namespace: 0,
    };
    let mut findings: Vec<String> = Vec::new();

    let shell = walk::tracked_shell_tree()?;
    for f in &shell {
        let text = read(Path::new(f))?;
        scan_shell(f, &text, &mut t, &mut findings);
        scan_anchor(f, &text, &mut t, &mut findings);
        scan_shell_locality(f, &text, &mut t, &mut findings);
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
            scan_locality(&p, &text, p == crosser, &mut t, &mut findings);
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
        println!("  help: a text-level path primitive — testing absoluteness, composing a prefix to test");
        println!("        containment — is native/src/walk.rs's; reach it through the named helper rather than");
        println!("        re-spelling it. A `/`-separated value naming something other than a filesystem");
        println!("        location is outside the contract and says so at its own line or the one above, with");
        println!("        `// path-dialect-exempt: <reason>`; a recorded verdict does not clear this arm");
        println!("  help: a shell file composing two roots anchors its cwd with `{}` before its first", ANCHOR);
        println!("        root binding; nothing else clears that arm");
        println!("  help: a shell absoluteness test asks gate_path_rooted (gate-sdk/lib/gate.sh), which reads");
        println!("        both dialects; a value outside the filesystem namespace says so with");
        println!("        `# path-dialect-exempt: <reason>`");
        return Ok(1);
    }
    println!(
        "PATH-DIALECT: clean ({} shell file(s), {} Rust file(s) scanned; {} producer occurrence(s) — {} in `cd` position, {} `Path`-typed, {} inside the crate's crosser, {} by recorded verdict, {} presence probe(s) binding no value; {} primitive spelling(s) — {} inside the crate's speller, {} declared out of the filesystem namespace; {} shell absoluteness test(s) — {} inside gate_path_rooted, {} declared out of the filesystem namespace; {} two-root file(s) anchored)",
        shell.len(), rust_files, t.total, t.cd, t.typed, t.crosser, t.verdict, t.probe, t.prim, t.local, t.namespace,
        t.shell_abs, t.shell_rooted, t.shell_namespace, t.anchored
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-path-dialect — the shell half reads the two single-dialect forms
    // and leaves every prefix or containment glob alone
    #[test]
    fn the_shell_half_reads_a_rooted_glob_and_no_prefix_glob() {
        for red in [
            r#"[[ "$x" == /* ]] || x="$r/$x""#,
            r#"[[ $x != "/"* ]]"#,
            r#"[[ $x = /* ]]"#,
            r#"[[ "$x" == /*]]"#,
        ] {
            assert!(shell_glob_test(red).is_some(), "{}", red);
        }
        for red in [r#"case "$x" in /*) ;; esac"#, "    /* | ./*) :", "(/*) :", "a | '/'* ) :"] {
            assert!(shell_case_arm(red).is_some(), "{}", red);
        }
        for green in [
            r#"[[ "$x" == /*/sub ]]"#,
            r#"[[ "$x" == *"/"* ]]"#,
            r#"x="/*""#,
            "    */*) :",
            r#"    "$d"/*) :"#,
            "    /dev/*) :",
            "    https://*|http://*) :",
            "    */../*) :",
            "for f in /*; do",
        ] {
            assert!(shell_glob_test(green).is_none() && shell_case_arm(green).is_none(), "{}", green);
        }
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — the predicate's own body clears by name, and only
    // through its closing brace
    #[test]
    fn only_the_predicate_body_clears_by_name() {
        let text = "gate_path_rooted() {  # x\n    case \"$1\" in\n        /* | \\\\*) return 0 ;;\n    esac\n}\ncase \"$y\" in /*) ;; esac\n";
        let mut t = Tally {
            cd: 0,
            typed: 0,
            crosser: 0,
            verdict: 0,
            probe: 0,
            total: 0,
            prim: 0,
            local: 0,
            namespace: 0,
            anchored: 0,
            shell_abs: 0,
            shell_rooted: 0,
            shell_namespace: 0,
        };
        let mut findings = Vec::new();
        scan_shell_locality("f.sh", text, &mut t, &mut findings);
        assert_eq!((t.shell_abs, t.shell_rooted), (2, 1));
        assert_eq!(findings.len(), 1, "{:?}", findings);
        assert!(findings[0].starts_with("f.sh:6 "), "{:?}", findings);
    }

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

    // spec: gate-sdk/SPEC.md §check-path-dialect — the containment vocabulary is composed from the
    // module's own constants: the bare prefix is the primitive wherever it sits, and a `format!`
    // handed to a prefix test is one only where its template composes a separator
    #[test]
    fn a_prefix_test_is_the_primitive_only_where_its_template_composes_a_separator() {
        assert_eq!(prefix_spelling(&format!("let p = {}, root);", PREFIX_FORM)), Some(PREFIX_FORM));
        let keyed = format!("k.{}\"{{}}=\", name)", PREFIX_TESTS[1]);
        assert_eq!(prefix_spelling(&keyed), None, "a `{{}}=` template splits a key, not a path");
        let joined = format!("p.{}\"{{}}/{{}}\", a, b))", PREFIX_TESTS[0]);
        assert_eq!(prefix_spelling(&joined), Some(PREFIX_TESTS[0]));
        assert_eq!(prefix_spelling("p.starts_with(&prefix)"), None);
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — the declaration's three states: absent, present
    // with a reason, and present with none, which is malformed rather than clearing
    #[test]
    fn a_namespace_declaration_without_a_reason_declares_nothing() {
        let with = split_file(&format!("// {} a queue tag's field\nlet a = 1;\n", EXEMPT), false);
        assert_eq!(namespace_declared(&with, 1), Some(true));
        let empty = split_file(&format!("// {}\nlet a = 1;\n", EXEMPT), false);
        assert_eq!(namespace_declared(&empty, 1), Some(false));
        let none = split_file("// nothing declared here\nlet a = 1;\n", false);
        assert_eq!(namespace_declared(&none, 1), None);
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

    // spec: gate-sdk/SPEC.md §check-path-dialect — a root is a one-line `cd … pwd` capture of either
    // kind, a prefix keyword binds the same name, and a capture from neither source is no root
    #[test]
    fn a_root_binding_is_read_from_its_own_line() {
        let src = r#"local KIT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)""#;
        assert_eq!(root_binding(src), Some(("KIT".to_string(), true)));
        let top = format!(r#"REPO="$(cd "$(git rev-parse {})" && pwd -P)""#, GIT_FLAGS[0]);
        assert_eq!(root_binding(&top), Some(("REPO".to_string(), false)));
        assert_eq!(root_binding(r#"FE="$(cd "$DIR/../bin" && pwd)""#), None);
        assert_eq!(root_binding(r#"out="$(cd "$SANDBOX" && run)""#), None);
    }

    // spec: gate-sdk/SPEC.md §check-path-dialect — arithmetic is a join or a strip on the name
    // itself, never on a longer name it prefixes
    #[test]
    fn string_arithmetic_is_a_join_or_a_strip_on_the_root() {
        assert!(composes(r#"echo "$R/x""#, "R"));
        assert!(composes(r#"for k in "$R"/*-kit"#, "R"));
        assert!(composes(r#"x="${R%/lib}""#, "R"));
        assert!(composes(r#"x="${R#pre}""#, "R"));
        assert!(!composes(r#"echo "$RX/x""#, "R"));
        assert!(!composes(r#"cd "$R""#, "R"));
    }
}
