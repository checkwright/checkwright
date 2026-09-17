// spec: canon-kit/SPEC.md §check-docs-cmd — every fenced invoked repo-relative .sh path, every
// backticked/fenced kit-prefixed env knob and every inline-span path citation in the governed doc
// set resolves against the tree or names no path the tree has retired
use super::manifest_temporal::{LineKind, TemporalValve};
use crate::proc;
use crate::spec;
use crate::walk;
use std::collections::HashSet;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-docs-cmd: {}", e);
            2
        }
    }
}

enum Token {
    Path(usize, String),
    Knob(usize, String),
    Cited(usize, String),
    Amendment(usize, String),
}

fn rule(args: &[String]) -> Result<i32, String> {
    let top = walk::toplevel()
        .map_err(|_| "not a git repository — cannot verify tracked paths/knobs".to_string())?;

    let exclude = spec::knob_array_pub("CANON_KIT_MDREF_EXCLUDE")?;
    let files: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        spec::manifest_files(".")?
            .into_iter()
            .map(|p| p.display().to_string())
            .filter(|f| {
                let rel = spec::strip_dot_slash(f);
                !exclude.iter().any(|g| walk::pattern_match(g, &rel))
            })
            .collect()
    };

    // spec: canon-kit/SPEC.md §check-docs-cmd — the kit-prefix roster, derived from the kit
    // roots: a caps name carrying one is a namespaced knob to verify
    let mut roots: Vec<String> = Vec::new();
    let mut prefixes: Vec<String> = Vec::new();
    for root in walk::kit_roots_rel()? {
        let root = root.trim_end_matches('/');
        if root.is_empty() {
            continue;
        }
        roots.push(root.to_string());
        let base = root.rsplit('/').next().unwrap_or(root);
        prefixes.push(format!("{}_", base.to_ascii_uppercase().replace('-', "_")));
    }

    let mut defined = defined_knobs(&top, &roots, &prefixes)?;
    // spec: canon-kit/SPEC.md §check-docs-cmd — a static kit's knobs left kit-root source with its
    // library, so the set is unioned with the names the static reader reads
    defined.extend(crate::knobs::static_names());
    let amend_glob = spec::knob_pub("CANON_KIT_AMENDMENT_GLOB")?;
    let valve = TemporalValve::load()?;
    let tree = Tree::read(&top)?;
    let cwd = walk::cwd()?;

    let mut bad: Vec<String> = Vec::new();
    let mut npath = 0usize;
    let mut nknob = 0usize;
    let mut ncited = 0usize;
    for f in &files {
        if !Path::new(f).is_file() {
            continue;
        }
        let docdir = dirname(f);
        let text = spec::read_text(Path::new(f))?;
        let cite = !valve.path_exempt(&spec::strip_dot_slash(f));
        for tok in scan(&text, &prefixes, &amend_glob, &valve, cite) {
            match tok {
                Token::Path(ln, t) => {
                    npath += 1;
                    if !path_ok(&docdir, &t)? {
                        bad.push(format!("{}:{}: invoked script '{}' is not a tracked file", f, ln, t));
                    }
                }
                Token::Knob(ln, t) => {
                    nknob += 1;
                    if !knob_ok(&defined, &t) {
                        bad.push(format!(
                            "{}:{}: env knob '{}' occurs in no tracked kit source",
                            f, ln, t
                        ));
                    }
                }
                Token::Cited(ln, t) => {
                    ncited += 1;
                    let cands = resolutions(&top, &cwd, &docdir, &t, &roots);
                    if tree.retired_unresolved(&cands) {
                        bad.push(format!(
                            "{}:{}: cited path '{}' was retired (no tracked file under any resolution)",
                            f, ln, t
                        ));
                    }
                }
                Token::Amendment(ln, t) => {
                    ncited += 1;
                    if tree.retired_basename(&t) {
                        bad.push(format!(
                            "{}:{}: cited amendment '{}' was retired (no tracked file carries the basename)",
                            f, ln, t
                        ));
                    }
                }
            }
        }
    }

    if !bad.is_empty() {
        println!("check-docs-cmd: unresolvable command path(s), env knob(s) or retired cited path(s) in the governed doc set:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: fix the path (relative to the doc, or repo-relative) and track the script, or");
        println!("        correct the knob name. A hypothetical invocation goes outside a fence, or the doc");
        println!("        joins CANON_KIT_MDREF_EXCLUDE.");
        println!("        A retired cited path: re-point it at the capability's current holder, or mark the");
        println!("        line as history with '<!-- manifest-temporal-exempt: <reason> -->' on it or the line");
        println!("        above, CANON_KIT_TEMPORAL_EXEMPT_SECTIONS or CANON_KIT_TEMPORAL_EXEMPT_PATHS.");
        return Ok(1);
    }
    let shallow = if tree.shallow {
        " (shallow clone: retirements limited to fetched history)"
    } else {
        ""
    };
    println!(
        "DOCS-CMD: clean ({} doc(s); {} invoked path(s) + {} kit-prefixed knob(s) resolve; {} cited path(s) name no retired path){}",
        files.len(),
        npath,
        nknob,
        ncited,
        shallow
    );
    Ok(0)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — `git grep` grades its own outcome by exit
// code: 1 is "no match" and ≥2 is a real error. Reading stdout through the success-only
// accessor would fold the two together and report an unsearchable tree as an empty knob set.
fn defined_knobs(
    top: &str,
    roots: &[String],
    prefixes: &[String],
) -> Result<HashSet<String>, String> {
    let pattern = format!("({})[A-Z0-9_]*", prefixes.join("|"));
    let mut argv: Vec<String> = vec![
        "-C".into(),
        top.into(),
        "grep".into(),
        "-h".into(),
        "-E".into(),
        pattern,
        "--".into(),
    ];
    argv.extend(roots.iter().cloned());
    argv.push(":!*.md".into());
    argv.push(":!*/gate-tests/*".into());
    let borrowed: Vec<&str> = argv.iter().map(String::as_str).collect();
    let out = proc::run("git", &borrowed)?;
    let text = match out.code() {
        Some(0) => String::from_utf8_lossy(out.stdout().unwrap_or(&[])).into_owned(),
        Some(1) => String::new(),
        Some(c) => {
            return Err(format!("git grep failed (exit {}) building the knob set", c));
        }
        None => {
            return Err("git grep failed (killed by a signal) building the knob set".to_string());
        }
    };
    let mut set = HashSet::new();
    for line in text.lines() {
        for run in caps_runs(line, 1) {
            if prefixes.iter().any(|p| run.starts_with(p.as_str())) {
                set.insert(run);
            }
        }
    }
    Ok(set)
}

// spec: canon-kit/SPEC.md §check-docs-cmd — assertion C's two repo-root-relative sets: what the
// index tracks (files and the directories holding them) and what the held history retired
struct Tree {
    tracked: HashSet<String>,
    retired: HashSet<String>,
    tracked_base: HashSet<String>,
    retired_base: HashSet<String>,
    shallow: bool,
}

impl Tree {
    fn read(top: &str) -> Result<Tree, String> {
        let mut tracked = HashSet::new();
        let mut tracked_base = HashSet::new();
        for f in git_names(top, &["ls-files", "-z"], "listing tracked files")? {
            tracked_base.insert(basename(&f).to_string());
            let mut i = 0usize;
            while let Some(off) = f[i..].find('/') {
                tracked.insert(f[..i + off].to_string());
                i += off + 1;
            }
            tracked.insert(f);
        }
        let shallow = git_text(top, &["rev-parse", "--is-shallow-repository"], "reading the shallow state")?
            .trim()
            == "true";
        let retired = retired_set(top)?;
        let retired_base = retired.iter().map(|r| basename(r).to_string()).collect();
        Ok(Tree {
            tracked,
            retired,
            tracked_base,
            retired_base,
            shallow,
        })
    }

    fn retired_unresolved(&self, cands: &[String]) -> bool {
        !cands.iter().any(|c| self.tracked.contains(c)) && cands.iter().any(|c| self.retired.contains(c))
    }

    // spec: canon-kit/SPEC.md §check-docs-cmd — an amendment is cited by bare name wherever its
    // component sits, so its basename resolves tree-wide rather than through the three roots
    fn retired_basename(&self, base: &str) -> bool {
        !self.tracked_base.contains(base) && self.retired_base.contains(base)
    }
}

fn basename(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}

// spec: canon-kit/SPEC.md §check-docs-cmd — every path deleted in held history plus every path
// the index deletes against HEAD, so the deleting commit reds at pre-commit; an unborn HEAD has
// retired nothing
fn retired_set(top: &str) -> Result<HashSet<String>, String> {
    let head = proc::run("git", &["-C", top, "rev-parse", "--verify", "-q", "HEAD"])?;
    match head.code() {
        Some(0) => {}
        Some(1) => return Ok(HashSet::new()),
        _ => return Err("git rev-parse failed resolving HEAD for the retired set".to_string()),
    }
    let mut set: HashSet<String> = HashSet::new();
    set.extend(git_names(
        top,
        &["log", "-z", "--no-renames", "--diff-filter=D", "--name-only", "--format="],
        "reading deletions from history",
    )?);
    set.extend(git_names(
        top,
        &["diff", "--cached", "-z", "--no-renames", "--diff-filter=D", "--name-only"],
        "reading deletions staged in the index",
    )?);
    Ok(set)
}

fn git_text(top: &str, args: &[&str], what: &str) -> Result<String, String> {
    let mut argv: Vec<&str> = vec!["-C", top];
    argv.extend_from_slice(args);
    let out = proc::run("git", &argv)?;
    match out.code() {
        Some(0) => Ok(String::from_utf8_lossy(out.stdout().unwrap_or(&[])).into_owned()),
        Some(c) => Err(format!("git {} failed (exit {}) {}", args[0], c, what)),
        None => Err(format!("git {} failed (killed by a signal) {}", args[0], what)),
    }
}

fn git_names(top: &str, args: &[&str], what: &str) -> Result<Vec<String>, String> {
    Ok(git_text(top, args, what)?
        .split(['\0', '\n'])
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect())
}

// spec: canon-kit/SPEC.md §check-docs-cmd — a cited token's resolutions, repo-root-relative: the
// doc's directory, the repo root, and each kit root
fn resolutions(top: &str, cwd: &str, docdir: &str, tok: &str, roots: &[String]) -> Vec<String> {
    let docabs = if walk::path_root(docdir).is_some() {
        docdir.to_string()
    } else {
        format!("{}/{}", cwd, docdir)
    };
    let mut out: Vec<String> = Vec::new();
    let mut bases: Vec<String> = vec![docabs, top.to_string()];
    bases.extend(roots.iter().map(|r| format!("{}/{}", top, r)));
    for base in bases {
        if let Some(rel) = under_top(top, &walk::normalize_abs(&format!("{}/{}", base, tok))) {
            if !out.contains(&rel) {
                out.push(rel);
            }
        }
    }
    out
}

fn under_top(top: &str, abs: &str) -> Option<String> {
    let lead = format!("{}/", top.trim_end_matches('/'));
    abs.strip_prefix(&lead).filter(|r| !r.is_empty()).map(str::to_string)
}

// spec: canon-kit/SPEC.md §check-docs-cmd — an exact code occurrence, or for a family stem
// ending '_' any name under it
fn knob_ok(defined: &HashSet<String>, t: &str) -> bool {
    if defined.contains(t) {
        return true;
    }
    t.ends_with('_') && defined.iter().any(|k| k.starts_with(t))
}

// spec: canon-kit/SPEC.md §check-docs-cmd — an invoked token resolves kit-relative (against
// the doc's own directory) or repo-relative
fn path_ok(docdir: &str, tok: &str) -> Result<bool, String> {
    if tok.contains("..") {
        return Ok(false);
    }
    for base in [docdir, "."] {
        let cand = spec::relative_to_cwd(&format!("{}/{}", base, tok));
        if cand.is_empty() || cand.starts_with("../") {
            continue;
        }
        let ls = proc::run("git", &["ls-files", "--error-unmatch", "--", &cand])?;
        if ls.code() == Some(0) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn dirname(p: &str) -> String {
    match p.rfind('/') {
        Some(0) => "/".to_string(),
        Some(i) => p[..i].to_string(),
        None => ".".to_string(),
    }
}

// spec: canon-kit/SPEC.md §check-docs-cmd — the caps-run shape both scans share
fn caps_runs(s: &str, min_tail: usize) -> Vec<String> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if b[i].is_ascii_uppercase() {
            let mut j = i + 1;
            while j < b.len() && (b[j].is_ascii_uppercase() || b[j].is_ascii_digit() || b[j] == b'_')
            {
                j += 1;
            }
            if j - i > min_tail {
                out.push(String::from_utf8_lossy(&b[i..j]).into_owned());
            }
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

// spec: canon-kit/SPEC.md §check-docs-cmd — a history valve exempts a line's cited paths alone;
// its fenced invocations and knobs are still scanned
fn scan(
    text: &str,
    prefixes: &[String],
    amend_glob: &str,
    valve: &TemporalValve,
    cite: bool,
) -> Vec<Token> {
    let mut out = Vec::new();
    for line in valve.lines(text) {
        match line.kind {
            LineKind::Fence => {}
            LineKind::Fenced => {
                scan_a(line.raw, line.ln, &mut out);
                scan_b(line.raw, line.ln, prefixes, &mut out);
            }
            LineKind::Heading | LineKind::Prose => {
                for span in inline_code_spans(line.raw) {
                    scan_b(&span, line.ln, prefixes, &mut out);
                    if cite && !line.valved {
                        scan_c(&span, line.ln, amend_glob, &mut out);
                    }
                }
            }
        }
    }
    out
}

fn inline_code_spans(line: &str) -> Vec<String> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'`' {
            if let Some(off) = b[i + 1..].iter().position(|&c| c == b'`') {
                out.push(String::from_utf8_lossy(&b[i + 1..i + 1 + off]).into_owned());
                i = i + 1 + off + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn scan_b(text: &str, ln: usize, prefixes: &[String], out: &mut Vec<Token>) {
    for run in caps_runs(text, 1) {
        if prefixes.iter().any(|p| run.starts_with(p.as_str())) {
            out.push(Token::Knob(ln, run));
        }
    }
}

fn scan_c(span: &str, ln: usize, amend_glob: &str, out: &mut Vec<Token>) {
    for w in span.split_whitespace() {
        if let Some(t) = cited_path(w) {
            out.push(Token::Cited(ln, t));
        } else if let Some(t) = cited_amendment(w, amend_glob) {
            out.push(Token::Amendment(ln, t));
        }
    }
}

// spec: canon-kit/SPEC.md §check-docs-cmd — the single-segment shape: one segment matching the
// amendment glob, after the same trims
fn cited_amendment(w: &str, glob: &str) -> Option<String> {
    let e = trim_word(w);
    let rest = e.strip_prefix("./").unwrap_or(e);
    if glob.is_empty() || !seg_ok(rest) || !walk::pattern_match(glob, rest) {
        return None;
    }
    Some(rest.to_string())
}

// spec: canon-kit/SPEC.md §check-docs-cmd — the path shape: two or more segments, an extension on
// the last, no `..`, after (A)'s quote and punctuation trims
fn cited_path(w: &str) -> Option<String> {
    let e = trim_word(w);
    let rest = e.strip_prefix("./").unwrap_or(e);
    let parts: Vec<&str> = rest.split('/').collect();
    if parts.len() < 2 || !parts.iter().all(|p| seg_ok(p)) || parts.contains(&"..") {
        return None;
    }
    if !parts[parts.len() - 1].trim_matches('.').contains('.') {
        return None;
    }
    Some(rest.to_string())
}

fn trim_word(w: &str) -> &str {
    w.trim_start_matches(['`', '"', '\'', '('])
        .trim_end_matches(['`', '"', '\'', ')', ';', ':', ','])
}

fn seg_ok(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|c| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'_' | b'-'))
}

fn scan_a(line: &str, ln: usize, out: &mut Vec<Token>) {
    for seg in split_commands(line) {
        let cmd = seg.trim_start();
        let cmd = strip_prompt(cmd);
        let words: Vec<&str> = cmd.split_whitespace().collect();
        let mut exe = match words.first() {
            Some(w) => (*w).to_string(),
            None => continue,
        };
        if matches!(exe.as_str(), "bash" | "sh" | "source" | ".") {
            exe = String::new();
            for w in words.iter().skip(1) {
                if w.starts_with('-') {
                    continue;
                }
                exe = (*w).to_string();
                break;
            }
        }
        if exe.is_empty() {
            continue;
        }
        if let Some(e) = invoked_script(&exe) {
            out.push(Token::Path(ln, e));
        }
    }
}

// spec: canon-kit/SPEC.md §check-docs-cmd — one fenced line is many commands, split on the
// shell's own separators before the first word of each is read as an executable
fn split_commands(line: &str) -> Vec<String> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut cur: Vec<u8> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let two = i + 1 < b.len();
        if two && b[i] == b'&' && b[i + 1] == b'&' {
            out.push(String::from_utf8_lossy(&cur).into_owned());
            cur.clear();
            i += 2;
            continue;
        }
        if two && b[i] == b'|' && b[i + 1] == b'|' {
            out.push(String::from_utf8_lossy(&cur).into_owned());
            cur.clear();
            i += 2;
            continue;
        }
        if matches!(b[i], b';' | b'|' | b'&') {
            out.push(String::from_utf8_lossy(&cur).into_owned());
            cur.clear();
            i += 1;
            continue;
        }
        cur.push(b[i]);
        i += 1;
    }
    out.push(String::from_utf8_lossy(&cur).into_owned());
    out
}

// spec: canon-kit/SPEC.md §check-docs-cmd — a copied shell prompt is not the executable
fn strip_prompt(cmd: &str) -> &str {
    let b = cmd.as_bytes();
    if b.is_empty() || !matches!(b[0], b'$' | b'#') {
        return cmd;
    }
    let mut i = 1usize;
    while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\x0b' | b'\x0c' | b'\r') {
        i += 1;
    }
    if i == 1 {
        return cmd;
    }
    &cmd[i..]
}

// spec: canon-kit/SPEC.md §check-docs-cmd — only an invoked repo-relative `.sh` path counts,
// after the quote and paren trims
fn invoked_script(w: &str) -> Option<String> {
    let e = trim_word(w);
    for lead in ["./", ".", "/", ""] {
        let rest = match e.strip_prefix(lead) {
            Some(r) => r,
            None => continue,
        };
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() < 2 || !parts.iter().all(|p| seg_ok(p)) {
            continue;
        }
        let last = parts[parts.len() - 1];
        if last.ends_with(".sh") && last.len() > 3 {
            return Some(e.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(dir: &str, args: &[&str]) {
        let mut argv: Vec<&str> = vec!["-C", dir, "-c", "user.name=t", "-c", "user.email=t@t"];
        argv.extend_from_slice(args);
        let out = proc::run("git", &argv).expect("git runs");
        assert_eq!(out.code(), Some(0), "git {:?}", args);
    }

    // spec: canon-kit/SPEC.md §check-docs-cmd — the retired set's staged half: a deletion the
    // index holds against HEAD is retired before any commit records it
    #[test]
    fn the_retired_set_holds_history_and_staged_deletions() {
        let dir = std::env::temp_dir().join(format!("cw-docs-cmd-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("bin")).unwrap();
        let d = walk::normalize_abs(&dir.display().to_string());
        git(&d, &["init", "-q"]);
        assert!(retired_set(&d).unwrap().is_empty());
        for f in ["bin/old.sh", "bin/staged.sh", "bin/kept.sh"] {
            std::fs::write(dir.join(f), "x\n").unwrap();
        }
        git(&d, &["add", "."]);
        git(&d, &["commit", "-q", "-m", "a"]);
        git(&d, &["rm", "-q", "bin/old.sh"]);
        git(&d, &["commit", "-q", "-m", "b"]);
        git(&d, &["rm", "-q", "--cached", "bin/staged.sh"]);
        let set = retired_set(&d).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(set.contains("bin/old.sh"));
        assert!(set.contains("bin/staged.sh"));
        assert!(!set.contains("bin/kept.sh"));
    }

    #[test]
    fn a_cited_path_needs_two_segments_and_an_extension() {
        assert_eq!(cited_path("`bin/x.sh`,"), Some("bin/x.sh".to_string()));
        assert_eq!(cited_path("./lib/gate.sh"), Some("lib/gate.sh".to_string()));
        assert_eq!(cited_path("x.sh"), None);
        assert_eq!(cited_path("bin/tool"), None);
        assert_eq!(cited_path("a/.gitignore"), None);
        assert_eq!(cited_path("../a/b.sh"), None);
        assert_eq!(cited_path("*/SPEC.md"), None);
        assert_eq!(cited_path("/abs/b.sh"), None);
    }

    #[test]
    fn an_amendment_basename_is_path_shaped_in_one_segment() {
        let g = "SPEC-*.md";
        assert_eq!(cited_amendment("`SPEC-sqlite.md`,", g), Some("SPEC-sqlite.md".to_string()));
        assert_eq!(cited_amendment("./SPEC-x.md", g), Some("SPEC-x.md".to_string()));
        assert_eq!(cited_amendment("SPEC.md", g), None);
        assert_eq!(cited_amendment("README.md", g), None);
        assert_eq!(cited_amendment("SPEC-<name>.md", g), None);
        assert_eq!(cited_amendment("SPEC-x.md", ""), None);
    }

    #[test]
    fn a_valved_line_exempts_citations_and_not_knobs() {
        let valve = TemporalValve::new(vec!["History".into()], vec![]);
        let text = "<!-- manifest-temporal-exempt: port record -->\n`bin/a.sh` `GATE_SDK_NATIVE_BIN`\n## History\n`bin/b.sh`\n## Now\n`bin/c.sh`\n";
        let toks = scan(text, &["GATE_SDK_".to_string()], "SPEC-*.md", &valve, true);
        let cited: Vec<String> = toks
            .iter()
            .filter_map(|t| match t {
                Token::Cited(_, s) => Some(s.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(cited, vec!["bin/c.sh".to_string()]);
        assert!(toks.iter().any(|t| matches!(t, Token::Knob(2, _))));
    }
}
