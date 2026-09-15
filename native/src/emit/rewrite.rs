// spec: guard-kit/SPEC.md §rewrite — the fixed-text replacement arm: its effect is its command line,
// it spawns no program, and it prints every span it changed.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and `Arm::Run`: it resolves
// `GATE_SDK_WORKFLOW_DIR`, and its contract is the 0/1/2 split an emitting arm collapses.
use crate::ere::Ere;
use std::io::Write;
use std::path::Path;

pub const KNOBS: &[&str] = &["GATE_SDK_WORKFLOW_DIR"];

const NAME: &str = "rewrite";
const USAGE: &str = "usage: --rewrite [--regex] [--expect <n>] [--] <find> <replace> <file>…";

struct Opts {
    regex: bool,
    expect: Option<usize>,
    find: String,
    replace: String,
    files: Vec<String>,
}

// spec: guard-kit/SPEC.md §rewrite — options may sit anywhere before `--`, an unrecognized
// `-`-prefixed argument there is a refusal, and `--` keeps a `<find>` beginning with `-` reachable.
fn parse(args: &[String]) -> Result<Opts, String> {
    let mut regex = false;
    let mut expect: Option<usize> = None;
    let mut positional: Vec<String> = Vec::new();
    let mut options = true;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if options && a.starts_with('-') {
            match a.as_str() {
                "--" => options = false,
                "--regex" => regex = true,
                "--expect" => {
                    let n = it
                        .next()
                        .and_then(|v| v.parse::<usize>().ok())
                        .ok_or_else(|| format!("--expect takes a non-negative count\n{}", USAGE))?;
                    expect = Some(n);
                }
                _ => return Err(format!("unrecognized option: {}\n{}", a, USAGE)),
            }
            continue;
        }
        positional.push(a.clone());
    }
    if positional.len() < 3 {
        return Err(USAGE.to_string());
    }
    let files = positional.split_off(2);
    let replace = positional.pop().unwrap_or_default();
    let find = positional.pop().unwrap_or_default();
    Ok(Opts {
        regex,
        expect,
        find,
        replace,
        files,
    })
}

// spec: guard-kit/SPEC.md §rewrite — exactly three escapes, read left to right; any other
// backslash is literal.
pub fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            _ => {
                out.push('\\');
                continue;
            }
        }
        chars.next();
    }
    out
}

enum Matcher {
    Literal(Vec<u8>),
    Regex(Ere),
}

impl Matcher {
    fn spans(&self, content: &str) -> Vec<(usize, usize)> {
        match self {
            Matcher::Literal(needle) => literal_spans(content.as_bytes(), needle),
            Matcher::Regex(ere) => regex_spans(content, ere),
        }
    }
}

fn literal_spans(hay: &[u8], needle: &[u8]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos + needle.len() <= hay.len() {
        match hay[pos..].windows(needle.len()).position(|w| w == needle) {
            Some(k) => {
                out.push((pos + k, pos + k + needle.len()));
                pos += k + needle.len();
            }
            None => break,
        }
    }
    out
}

// spec: guard-kit/SPEC.md §rewrite — line-scoped: a trailing newline ends the last line and opens
// none, and an empty match advances one byte.
fn regex_spans(content: &str, ere: &Ere) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let bytes = content.as_bytes();
    let mut line_start = 0;
    while line_start < bytes.len() {
        let line_end = bytes[line_start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|k| line_start + k)
            .unwrap_or(bytes.len());
        let line = &content[line_start..line_end];
        let mut pos = 0;
        while pos <= line.len() {
            let Some((s, e)) = ere.find_from(line, pos) else {
                break;
            };
            out.push((line_start + s, line_start + e));
            pos = if e > s { e } else { e + 1 };
        }
        line_start = line_end + 1;
    }
    out
}

struct Change {
    line: usize,
    before: Vec<String>,
    after: Vec<String>,
}

// spec: guard-kit/SPEC.md §rewrite — a span's report covers the whole lines it touched, before and
// as the file will read.
fn apply(orig: &[u8], spans: &[(usize, usize)], replace: &[u8]) -> (Vec<u8>, Vec<Change>) {
    let mut new = Vec::with_capacity(orig.len());
    let mut placed: Vec<(usize, usize)> = Vec::new();
    let mut last = 0;
    for &(s, e) in spans {
        new.extend_from_slice(&orig[last..s]);
        let ns = new.len();
        new.extend_from_slice(replace);
        placed.push((ns, new.len()));
        last = e;
    }
    new.extend_from_slice(&orig[last..]);
    let changes = spans
        .iter()
        .zip(placed.iter())
        .map(|(&(s, e), &(ns, ne))| {
            let orig_whole_end = e > s && orig[e - 1] == b'\n';
            // spec: guard-kit/SPEC.md §rewrite — a span that deleted whole lines leaves no `+` line,
            // and a replacement ending mid-line reports the line it now runs into
            let new_whole_end = orig_whole_end
                && match replace.last() {
                    None => s == line_start(orig, s),
                    Some(&b) => b == b'\n',
                };
            Change {
                line: 1 + orig[..s].iter().filter(|&&b| b == b'\n').count(),
                before: lines(&orig[line_start(orig, s)..region_end(orig, e, orig_whole_end)]),
                after: lines(&new[line_start(&new, ns)..region_end(&new, ne, new_whole_end)]),
            }
        })
        .collect();
    (new, changes)
}

fn line_start(buf: &[u8], at: usize) -> usize {
    buf[..at].iter().rposition(|&b| b == b'\n').map(|k| k + 1).unwrap_or(0)
}

fn region_end(buf: &[u8], at: usize, whole: bool) -> usize {
    if whole {
        return at;
    }
    buf[at..]
        .iter()
        .position(|&b| b == b'\n')
        .map(|k| at + k + 1)
        .unwrap_or(buf.len())
}

fn lines(region: &[u8]) -> Vec<String> {
    if region.is_empty() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(region);
    let body = text.strip_suffix('\n').unwrap_or(&text);
    body.split('\n').map(str::to_string).collect()
}

fn text_ok(bytes: &[u8]) -> bool {
    !bytes.contains(&0) && std::str::from_utf8(bytes).is_ok()
}

fn has_git_component(p: &Path) -> bool {
    p.components().any(|c| c.as_os_str() == ".git")
}

struct Planned {
    shown: String,
    canonical: String,
    new: Option<Vec<u8>>,
    changes: Vec<Change>,
}

// spec: guard-kit/SPEC.md §rewrite — the operand refusals, in argument order, every one before the
// first write.
fn check_operand(
    op: &str,
    root: &str,
    is_state: &dyn Fn(&str) -> Result<bool, String>,
    seen: &[Planned],
) -> Result<(String, Vec<u8>), String> {
    let path = Path::new(op);
    let meta = std::fs::symlink_metadata(path).map_err(|_| format!("no such file: {}", op))?;
    if meta.file_type().is_symlink() {
        return Err(format!("refusing {} — a symlink", op));
    }
    if !meta.is_file() {
        return Err(format!("refusing {} — not a regular file", op));
    }
    if has_git_component(path) {
        return Err(format!("refusing {} — a path through .git", op));
    }
    let canonical =
        crate::walk::canonicalize(path).ok_or_else(|| format!("cannot resolve {}", op))?;
    if !super::scratch_run::is_inside(root, &canonical) {
        return Err(format!("refusing {} — outside the working directory {}", op, root));
    }
    if has_git_component(Path::new(&canonical[root.len()..])) {
        return Err(format!("refusing {} — resolves through .git", op));
    }
    match is_state(op) {
        Ok(false) => {}
        Ok(true) => {
            return Err(format!(
                "refusing {} — the lifecycle state file, written only by the --enter-stage arm",
                op
            ))
        }
        Err(e) => {
            return Err(format!(
                "refusing {} — cannot tell whether it is the lifecycle state file: {}",
                op, e
            ))
        }
    }
    if seen.iter().any(|p| p.canonical == canonical) {
        return Err(format!("refusing {} — names a file already listed", op));
    }
    let content = std::fs::read(path).map_err(|e| format!("cannot read {}: {}", op, e))?;
    if !text_ok(&content) {
        return Err(format!("refusing {} — not UTF-8 text, or carries a NUL byte", op));
    }
    Ok((canonical, content))
}

fn write_replacing(canonical: &str, content: &[u8]) -> Result<(), String> {
    let target = Path::new(canonical);
    let dir = target.parent().unwrap_or(Path::new("."));
    let leaf = target.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let tmp = dir.join(format!(".{}.rewrite-{}.tmp", leaf, std::process::id()));
    let perms = std::fs::metadata(target)
        .map_err(|e| format!("cannot stat {}: {}", canonical, e))?
        .permissions();
    let result = (|| {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
            .map_err(|e| format!("cannot create {}: {}", tmp.display(), e))?;
        f.write_all(content)
            .and_then(|_| f.sync_all())
            .map_err(|e| format!("cannot write {}: {}", tmp.display(), e))?;
        std::fs::set_permissions(&tmp, perms)
            .map_err(|e| format!("cannot set the mode of {}: {}", tmp.display(), e))?;
        std::fs::rename(&tmp, target)
            .map_err(|e| format!("cannot replace {}: {}", canonical, e))
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

fn execute(
    args: &[String],
    root: &str,
    is_state: &dyn Fn(&str) -> Result<bool, String>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let mut refuse = |m: &str| {
        let _ = writeln!(err, "{}: {}", NAME, m);
        2
    };
    let opts = match parse(args) {
        Ok(o) => o,
        Err(e) => return refuse(&e),
    };
    let replace = unescape(&opts.replace);
    let matcher = if opts.regex {
        match Ere::compile(&opts.find) {
            Ok(e) => Matcher::Regex(e),
            Err(e) => return refuse(&format!("cannot compile /{}/: {}", opts.find, e)),
        }
    } else {
        let find = unescape(&opts.find);
        if find.is_empty() {
            return refuse(&format!("an empty <find> matches everywhere\n{}", USAGE));
        }
        Matcher::Literal(find.into_bytes())
    };
    let mut planned: Vec<Planned> = Vec::new();
    for op in &opts.files {
        let (canonical, content) = match check_operand(op, root, is_state, &planned) {
            Ok(v) => v,
            Err(e) => return refuse(&e),
        };
        let text = String::from_utf8_lossy(&content).into_owned();
        let spans = matcher.spans(&text);
        let (new, changes) = if spans.is_empty() {
            (None, Vec::new())
        } else {
            let (new, changes) = apply(&content, &spans, replace.as_bytes());
            if !text_ok(&new) {
                return refuse(&format!(
                    "refusing {} — the rewrite would leave it not UTF-8 (a match split a character)",
                    op
                ));
            }
            (Some(new), changes)
        };
        planned.push(Planned {
            shown: op.clone(),
            canonical,
            new,
            changes,
        });
    }
    let total: usize = planned.iter().map(|p| p.changes.len()).sum();
    let touched = planned.iter().filter(|p| p.new.is_some()).count();
    let files = planned.len();
    if let Some(n) = opts.expect {
        if n != total {
            let _ = writeln!(
                out,
                "{}: {} replacement(s) matched in {} of {} file(s), but --expect {} — nothing written",
                NAME, total, touched, files, n
            );
            return 1;
        }
    }
    if total == 0 {
        for p in &planned {
            let _ = writeln!(out, "{}: no match", p.shown);
        }
        let _ = writeln!(out, "{}: 0 replacement(s) in 0 of {} file(s) — nothing written", NAME, files);
        return 1;
    }
    let mut replaced: Vec<&str> = Vec::new();
    for p in &planned {
        let Some(new) = &p.new else {
            let _ = writeln!(out, "{}: no match", p.shown);
            continue;
        };
        if let Err(e) = write_replacing(&p.canonical, new) {
            let done = if replaced.is_empty() {
                "none".to_string()
            } else {
                replaced.join(", ")
            };
            return refuse(&format!("{} — files already replaced: {}", e, done));
        }
        replaced.push(&p.shown);
        for c in &p.changes {
            let _ = writeln!(out, "{}:{}:", p.shown, c.line);
            for l in &c.before {
                let _ = writeln!(out, "-{}", l);
            }
            for l in &c.after {
                let _ = writeln!(out, "+{}", l);
            }
        }
    }
    let _ = writeln!(out, "{}: {} replacement(s) in {} of {} file(s)", NAME, total, touched, files);
    0
}

pub fn run(args: &[String]) -> i32 {
    let root = match crate::walk::cwd().ok().and_then(crate::walk::canonicalize) {
        Some(r) => r,
        None => {
            eprintln!("{}: cannot resolve the working directory", NAME);
            return 2;
        }
    };
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    execute(
        args,
        &root,
        &crate::hook::workflow_state::is_state_file,
        &mut stdout.lock(),
        &mut stderr.lock(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Sandbox {
        root: String,
    }

    impl Sandbox {
        fn new(tag: &str) -> Sandbox {
            let dir = std::env::temp_dir().join(format!("cw-rewrite-{}-{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("cannot create the sandbox");
            Sandbox {
                root: crate::walk::canonicalize(&dir).expect("the sandbox must resolve"),
            }
        }
        fn file(&self, name: &str, body: &[u8]) -> String {
            let p = format!("{}/{}", self.root, name);
            std::fs::write(&p, body).expect("cannot seed a sandbox file");
            p
        }
        fn run(&self, args: &[&str]) -> (i32, String, String) {
            let argv: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            let (mut out, mut err) = (Vec::new(), Vec::new());
            let never = |_: &str| Ok(false);
            let rc = execute(&argv, &self.root, &never, &mut out, &mut err);
            (
                rc,
                String::from_utf8_lossy(&out).into_owned(),
                String::from_utf8_lossy(&err).into_owned(),
            )
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn read(p: &str) -> Vec<u8> {
        std::fs::read(p).expect("cannot read a sandbox file")
    }

    // spec: guard-kit/SPEC.md §rewrite — the `\n` escape carries a multi-line literal on one
    // command line, and the report covers both touched lines
    #[test]
    fn a_literal_spans_a_line_boundary_and_reports_both_lines() {
        let sb = Sandbox::new("literal");
        let f = sb.file("a.md", b"one\ntwo\nthree\n");
        let (rc, out, _) = sb.run(&["one\\ntwo", "uno\\ndos", &f]);
        assert_eq!(rc, 0);
        assert_eq!(read(&f), b"uno\ndos\nthree\n");
        assert!(
            out.contains(&format!("{}:1:\n-one\n-two\n+uno\n+dos\n", f)),
            "the two-line span's report was {:?}",
            out
        );
        assert!(out.ends_with("rewrite: 1 replacement(s) in 1 of 1 file(s)\n"));
    }

    // spec: guard-kit/SPEC.md §rewrite — `^` anchors per line, and an empty match advances rather
    // than looping
    #[test]
    fn a_regex_anchors_per_line_and_an_empty_match_advances() {
        let sb = Sandbox::new("regex");
        let f = sb.file("a.md", b"ab\nab\n");
        let (rc, _, _) = sb.run(&["--regex", "^a", "X", &f]);
        assert_eq!(rc, 0);
        assert_eq!(read(&f), b"Xb\nXb\n");
        let g = sb.file("b.md", b"bc\n");
        let (rc, _, err) = sb.run(&["--regex", "x*", "-", &g]);
        assert_eq!(rc, 2, "a `-`-prefixed operand before `--` is an option: {}", err);
        let (rc, out, _) = sb.run(&["--regex", "--", "x*", "-", &g]);
        assert_eq!(rc, 0);
        assert_eq!(read(&g), b"-b-c-\n");
        assert!(out.contains("rewrite: 3 replacement(s) in 1 of 1 file(s)"), "{}", out);
    }

    // spec: guard-kit/SPEC.md §rewrite — deleting whole lines prints them with no `+` line
    #[test]
    fn a_whole_line_deletion_reports_no_resulting_line() {
        let sb = Sandbox::new("delete");
        let f = sb.file("a.md", b"keep\ndrop\nkeep\n");
        let (rc, out, _) = sb.run(&["drop\\n", "", &f]);
        assert_eq!(rc, 0);
        assert_eq!(read(&f), b"keep\nkeep\n");
        assert!(out.starts_with(&format!("{}:2:\n-drop\nrewrite:", f)), "{}", out);
    }

    #[test]
    fn an_expect_mismatch_writes_nothing() {
        let sb = Sandbox::new("expect");
        let f = sb.file("a.md", b"x x\n");
        let (rc, out, _) = sb.run(&["--expect", "1", "x", "y", &f]);
        assert_eq!(rc, 1);
        assert_eq!(read(&f), b"x x\n");
        assert!(out.contains("--expect 1"), "{}", out);
    }

    #[test]
    fn no_match_exits_one_and_writes_nothing() {
        let sb = Sandbox::new("nomatch");
        let f = sb.file("a.md", b"abc\n");
        let (rc, out, _) = sb.run(&["zzz", "y", &f]);
        assert_eq!(rc, 1);
        assert_eq!(read(&f), b"abc\n");
        assert!(out.contains(&format!("{}: no match", f)), "{}", out);
    }

    // spec: guard-kit/SPEC.md §rewrite — a byte-wise match can split a multi-byte character, and
    // the content it would produce is refused before any write
    #[test]
    fn a_result_that_is_not_utf8_is_refused() {
        let sb = Sandbox::new("utf8");
        let f = sb.file("a.md", "é\n".as_bytes());
        let (rc, _, err) = sb.run(&["--regex", "^.", "x", &f]);
        assert_eq!(rc, 2);
        assert_eq!(read(&f), "é\n".as_bytes());
        assert!(err.contains("not UTF-8"), "{}", err);
    }

    #[test]
    fn a_duplicate_operand_is_refused_before_any_write() {
        let sb = Sandbox::new("dup");
        let f = sb.file("a.md", b"x\n");
        let dotted = format!("{}/./a.md", sb.root);
        let (rc, _, err) = sb.run(&["x", "y", &f, &dotted]);
        assert_eq!(rc, 2);
        assert_eq!(read(&f), b"x\n");
        assert!(err.contains("already listed"), "{}", err);
    }

    #[test]
    fn a_state_file_predicate_error_is_a_refusal() {
        let sb = Sandbox::new("state");
        let f = sb.file("a.md", b"x\n");
        let argv: Vec<String> = ["x", "y", f.as_str()].iter().map(|s| s.to_string()).collect();
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let broken = |_: &str| Err("no knob".to_string());
        assert_eq!(execute(&argv, &sb.root, &broken, &mut out, &mut err), 2);
        assert_eq!(read(&f), b"x\n");
    }

    #[test]
    fn the_three_escapes_and_a_literal_backslash() {
        assert_eq!(unescape("a\\nb\\tc\\\\d\\qe"), "a\nb\tc\\d\\qe");
        assert_eq!(unescape("\\\\n"), "\\n");
        assert_eq!(unescape("end\\"), "end\\");
    }
}
