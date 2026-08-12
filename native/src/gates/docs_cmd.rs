// spec: canon-kit/SPEC.md §check-docs-cmd — every fenced invoked repo-relative .sh path and
// every backticked/fenced kit-prefixed env knob in the governed doc set resolves against the
// tree
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
}

fn rule(args: &[String]) -> Result<i32, String> {
    let top_out = proc::run("git", &["rev-parse", "--show-toplevel"])?;
    let top = match top_out.stdout() {
        Some(o) => String::from_utf8_lossy(o).trim().to_string(),
        None => {
            return Err("not a git repository — cannot verify tracked paths/knobs".to_string());
        }
    };

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

    let defined = defined_knobs(&top, &roots, &prefixes)?;

    let mut bad: Vec<String> = Vec::new();
    let mut npath = 0usize;
    let mut nknob = 0usize;
    for f in &files {
        if !Path::new(f).is_file() {
            continue;
        }
        let docdir = dirname(f);
        let text = spec::read_text(Path::new(f))?;
        for tok in scan(&text, &prefixes) {
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
            }
        }
    }

    if !bad.is_empty() {
        println!("check-docs-cmd: unresolvable command path(s) or env knob(s) in the governed doc set:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: fix the path (relative to the doc, or repo-relative) and track the script, or");
        println!("        correct the knob name. A hypothetical example goes outside a fence, or the doc");
        println!("        joins CANON_KIT_MDREF_EXCLUDE. Only invoked .sh paths and kit-prefixed knobs count.");
        return Ok(1);
    }
    println!(
        "DOCS-CMD: clean ({} doc(s); {} invoked path(s) + {} kit-prefixed knob(s) resolve)",
        files.len(),
        npath,
        nknob
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

fn scan(text: &str, prefixes: &[String]) -> Vec<Token> {
    let mut out = Vec::new();
    let mut infence = false;
    for (idx, line) in text.lines().enumerate() {
        let ln = idx + 1;
        if spec::is_fence_line(line) {
            infence = !infence;
            continue;
        }
        if infence {
            scan_a(line, ln, &mut out);
            scan_b(line, ln, prefixes, &mut out);
        } else {
            for span in inline_code_spans(line) {
                scan_b(&span, ln, prefixes, &mut out);
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
    let e = w
        .trim_start_matches(['`', '"', '\'', '('])
        .trim_end_matches(['`', '"', '\'', ')', ';', ':', ',']);
    let seg_ok =
        |s: &str| !s.is_empty() && s.bytes().all(|c| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'_' | b'-'));
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
