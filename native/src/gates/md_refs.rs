// spec: canon-kit/SPEC.md §check-md-refs — every internal markdown link in the governed doc
// set resolves (relative path to a tracked file/dir, #anchor to a heading slug)
use crate::proc;
use crate::spec;
use crate::walk;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-md-refs: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let probe = proc::run("git", &["rev-parse", "--git-dir"])?;
    if probe.stdout().is_none() {
        return Err("not a git repository — cannot verify tracked targets".to_string());
    }

    let exclude = spec::knob_array_pub("CANON_KIT_MDREF_EXCLUDE")?;
    let files: Vec<String> = if !args.is_empty() {
        args.to_vec()
    } else {
        spec::manifest_files(".")?
            .into_iter()
            .map(|p| p.display().to_string())
            .filter(|f| !excluded(&exclude, &spec::strip_dot_slash(f)))
            .collect()
    };

    // spec: canon-kit/SPEC.md §check-md-refs — the self-repo blob-link prefix, derived from
    // origin through the shared identity rule; an empty prefix skips the self-repo pass
    let self_repo_prefix = self_repo_prefix(&spec::knob_pub("CANON_KIT_DOCS_BLOB_REF")?)?;

    // spec: canon-kit/SPEC.md §check-md-refs — the tracked-file membership set, filled once
    // rather than by a per-link exec
    let listing = proc::run("git", &["ls-files", "-z"])?;
    let tracked: HashSet<String> = match listing.stdout() {
        Some(o) => String::from_utf8_lossy(o)
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect(),
        None => return Err("git ls-files failed — the tracked set is unknown".to_string()),
    };

    let mut anchors: HashMap<String, Vec<String>> = HashMap::new();
    let mut bad: Vec<String> = Vec::new();
    let mut links = 0usize;
    let mut selfrepo = 0usize;

    for f in &files {
        if !Path::new(f).is_file() {
            continue;
        }
        let base = dirname(f);
        let text = spec::read_text(Path::new(f))?;
        for tgt in link_targets(&text) {
            if !self_repo_prefix.is_empty() && tgt.starts_with(&self_repo_prefix) {
                let rest = &tgt[self_repo_prefix.len()..];
                links += 1;
                selfrepo += 1;
                let (path, anchor) = split_anchor(rest);
                if path.is_empty() {
                    bad.push(format!(
                        "{}: self-repo reference link '{}' names no path",
                        f, tgt
                    ));
                } else if !target_resolves(&tracked, &path)? {
                    bad.push(format!(
                        "{}: self-repo reference link '{}' → {} is not a git-tracked file",
                        f, tgt, path
                    ));
                } else if !anchor.is_empty()
                    && Path::new(&path).is_file()
                    && !anchor_ok(&mut anchors, &path, &anchor)?
                {
                    bad.push(format!(
                        "{}: [..]({}) — no heading in {} slugs to '{}'",
                        f, tgt, path, anchor
                    ));
                }
                continue;
            }
            if tgt.contains("://") || tgt.starts_with("mailto:") {
                continue;
            }
            links += 1;
            let (path, anchor) = split_anchor(&tgt);
            if path.is_empty() {
                if !anchor.is_empty() && !anchor_ok(&mut anchors, f, &anchor)? {
                    bad.push(format!(
                        "{}: [..](#{}) — no heading in this file slugs to '{}'",
                        f, anchor, anchor
                    ));
                }
                continue;
            }
            let p = spec::relative_to_cwd(&format!("{}/{}", base, path));
            if !target_resolves(&tracked, &p)? {
                bad.push(format!(
                    "{}: link target '{}' → {} is not a tracked file or directory",
                    f, tgt, p
                ));
                continue;
            }
            if !anchor.is_empty() && Path::new(&p).is_file() && !anchor_ok(&mut anchors, &p, &anchor)?
            {
                bad.push(format!(
                    "{}: [..]({}) — no heading in {} slugs to '{}'",
                    f, tgt, p, anchor
                ));
            }
        }
    }

    if !bad.is_empty() {
        println!("check-md-refs: unresolved internal markdown link(s) in the governed doc set:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: fix the path (relative to the linking file), track the target, or fix the");
        println!("        #anchor to a real heading slug. External URLs are out of scope.");
        return Ok(1);
    }
    println!(
        "MD-REFS: clean ({} doc(s), {} internal link(s) all resolve; {} self-repo reference link(s))",
        files.len(),
        links,
        selfrepo
    );
    Ok(0)
}

fn excluded(globs: &[String], rel: &str) -> bool {
    globs.iter().any(|g| walk::pattern_match(g, rel))
}

fn dirname(p: &str) -> String {
    match p.rfind('/') {
        Some(0) => "/".to_string(),
        Some(i) => p[..i].to_string(),
        None => ".".to_string(),
    }
}

fn split_anchor(t: &str) -> (String, String) {
    match t.find('#') {
        Some(i) => (t[..i].to_string(), t[i + 1..].to_string()),
        None => (t.to_string(), String::new()),
    }
}

// spec: canon-kit/SPEC.md §check-md-refs — a link token is matched **within one line**, so
// a `](` whose `)` sits on a later line is not one; that is how a prose sentence quoting
// the token itself stays out of the corpus
fn link_targets(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        line_link_targets(line, &mut out);
    }
    out
}

fn line_link_targets(line: &str, out: &mut Vec<String>) {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i + 1 < b.len() {
        if b[i] == b']' && b[i + 1] == b'(' {
            if let Some(off) = b[i + 2..].iter().position(|&c| c == b')') {
                if off > 0 {
                    let inner = String::from_utf8_lossy(&b[i + 2..i + 2 + off]).into_owned();
                    let tgt = inner.split(' ').next().unwrap_or("").to_string();
                    if !tgt.is_empty() {
                        out.push(tgt);
                    }
                    i = i + 2 + off + 1;
                    continue;
                }
            }
        }
        i += 1;
    }
}

fn target_resolves(tracked: &HashSet<String>, p: &str) -> Result<bool, String> {
    if p.starts_with("..") {
        return Ok(false);
    }
    if Path::new(p).is_file() {
        if tracked.contains(p) {
            return Ok(true);
        }
        let ci = proc::run("git", &["check-ignore", "-q", "--", p])?;
        return Ok(ci.code() == Some(0));
    }
    if !Path::new(p).is_dir() {
        return Ok(false);
    }
    let ls = proc::run("git", &["ls-files", "--", p])?;
    Ok(ls.stdout().map(|o| !o.is_empty()).unwrap_or(false))
}

// spec: canon-kit/SPEC.md §check-md-refs — a target file's slug set is memoized on first use
fn anchor_ok(
    cache: &mut HashMap<String, Vec<String>>,
    file: &str,
    anchor: &str,
) -> Result<bool, String> {
    if !cache.contains_key(file) {
        let text = spec::read_text(Path::new(file))?;
        cache.insert(file.to_string(), heading_slugs(&text));
    }
    Ok(cache[file].iter().any(|s| s == anchor))
}

// spec: canon-kit/SPEC.md §check-md-refs — the heading-to-anchor slug rule an `#anchor`
// link is resolved against
fn heading_slugs(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let b = line.as_bytes();
        let mut h = 0usize;
        while h < b.len() && b[h] == b'#' {
            h += 1;
        }
        if h == 0 || h > 6 || h >= b.len() {
            continue;
        }
        let mut s = h;
        while s < b.len() && is_space(b[s]) {
            s += 1;
        }
        if s == h {
            continue;
        }
        let mut e = b.len();
        while e > s && is_space(b[e - 1]) {
            e -= 1;
        }
        if e <= s {
            continue;
        }
        let head = String::from_utf8_lossy(&b[s..e]).to_lowercase();
        let kept: String = head
            .chars()
            .filter(|c| {
                c.is_ascii_lowercase() || c.is_ascii_digit() || *c == ' ' || *c == '_' || *c == '-'
            })
            .collect();
        out.push(collapse_spaces(&kept));
    }
    out
}

fn collapse_spaces(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_run = false;
    for c in s.chars() {
        if c == ' ' {
            if !in_run {
                out.push('-');
                in_run = true;
            }
            continue;
        }
        in_run = false;
        out.push(c);
    }
    out
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\x0b' | b'\x0c' | b'\r' | b'\n')
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — `gate_self_repo_prefix`: the git@ and https remote
// forms normalize to one https identity, so no kit ships a repo name
fn self_repo_prefix(git_ref: &str) -> Result<String, String> {
    let out = proc::run("git", &["remote", "get-url", "origin"])?;
    let origin = match out.stdout() {
        Some(o) => String::from_utf8_lossy(o).trim().to_string(),
        None => return Ok(String::new()),
    };
    if origin.is_empty() {
        return Ok(String::new());
    }
    let mut id = origin.strip_suffix(".git").unwrap_or(&origin).to_string();
    id = id.trim_end_matches('/').to_string();
    if let Some(rest) = id.strip_prefix("git@") {
        match rest.find(':') {
            Some(i) => id = format!("https://{}/{}", &rest[..i], &rest[i + 1..]),
            None => return Ok(String::new()),
        }
    } else if !id.starts_with("https://") && !id.starts_with("http://") {
        return Ok(String::new());
    }
    Ok(format!("{}/blob/{}/", id, git_ref))
}

