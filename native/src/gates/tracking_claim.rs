// spec: canon-kit/SPEC.md §check-tracking-claim — every fixed-vocabulary tracking claim on a
// governed manifest surface agrees with git
use crate::proc;
use crate::spec;
use std::path::Path;

const PREDICATES: &[&str] = &["committed", "tracked", "gitignored", "local-only", "two-tier"];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-tracking-claim: {}", e);
            2
        }
    }
}

struct Claim {
    file: String,
    lineno: usize,
    path: String,
    pred: String,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let probe = proc::run("git", &["-C", root, "rev-parse", "--git-dir"])?;
    if probe.stdout().is_none() {
        return Err(format!(
            "{} is not a git repository — a tracking claim is unverifiable",
            root
        ));
    }

    let manifest_files: Vec<String> = spec::manifest_files(root)?
        .into_iter()
        .map(|p| p.display().to_string())
        .collect();
    if manifest_files.is_empty() {
        println!("TRACKING-CLAIM: clean (0 governed manifest surface(s))");
        return Ok(0);
    }

    let claims = extract(&manifest_files)?;
    let mut errors: Vec<String> = Vec::new();
    for c in &claims {
        let rel = spec::strip_dot_slash(
            c.file
                .strip_prefix(&format!("{}/", root))
                .unwrap_or(&c.file),
        );

        let ls = proc::run("git", &["-C", root, "ls-files", "--", &c.path])?;
        let tracked = match ls.stdout() {
            Some(o) => !o.is_empty(),
            None => {
                return Err(format!("git ls-files failed for '{}'", c.path));
            }
        };
        let ntracked = usize::from(tracked);

        // spec: canon-kit/SPEC.md §check-tracking-claim — the ignored side is rule-based
        // (check-ignore --no-index), not presence-based, so it resolves in a fresh checkout
        let ci = proc::run(
            "git",
            &["-C", root, "check-ignore", "-q", "--no-index", "--", &c.path],
        )?;
        let nignored = usize::from(ci.code() == Some(0));

        if ntracked == 0 && nignored == 0 && !Path::new(&format!("{}/{}", root, c.path)).exists() {
            errors.push(format!(
                "{}:{}: '{} is {}' — the path is in neither the index nor the ignore rules nor the working tree, so the claim is unverifiable",
                rel, c.lineno, c.path, c.pred
            ));
            continue;
        }

        let ok = match c.pred.as_str() {
            "committed" | "tracked" => ntracked == 1 && nignored == 0,
            "gitignored" | "local-only" => ntracked == 0 && nignored == 1,
            "two-tier" => ntracked == 1 && nignored == 1,
            _ => true,
        };
        if !ok {
            errors.push(format!(
                "{}:{}: '{} is {}' is false — tracked members: {}, ignored members: {}",
                rel, c.lineno, c.path, c.pred, ntracked, nignored
            ));
        }
    }

    if !errors.is_empty() {
        println!(
            "check-tracking-claim: {} prose claim(s) git disagrees with:",
            errors.len()
        );
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: git owns a path's tracking status, so prose states the rule and this gate verifies it. Reword the sentence to the predicate git actually supports — 'is committed'/'is tracked' (every member tracked, none ignored), 'is gitignored'/'is local-only' (no member tracked), or 'is two-tier' (both classes non-empty) — or fix the tracking. There is no per-site valve: a claim that cannot be made true is a claim that must be reworded.");
        return Ok(1);
    }
    println!(
        "TRACKING-CLAIM: clean ({} tracking claim(s) across {} manifest surface(s); every predicate agrees with git)",
        claims.len(),
        manifest_files.len()
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-tracking-claim — shape-only extraction over the blank-line
// paragraph join: a predicate outside inline code, bound by adjacency to the backticked path
// it follows. Verification is the caller's.
fn extract(files: &[String]) -> Result<Vec<Claim>, String> {
    let mut out: Vec<Claim> = Vec::new();
    for f in files {
        let text = spec::read_text(Path::new(f))?;
        let mut para: Vec<(usize, String)> = Vec::new();
        let mut fence = false;
        for (idx, raw) in text.lines().enumerate() {
            if spec::is_fence_line(raw) {
                flush(f, &mut para, &mut out);
                fence = !fence;
                continue;
            }
            if fence || spec::is_blank(raw) {
                flush(f, &mut para, &mut out);
                continue;
            }
            para.push((idx + 1, raw.to_string()));
        }
        flush(f, &mut para, &mut out);
    }
    Ok(out)
}

fn flush(file: &str, para: &mut Vec<(usize, String)>, out: &mut Vec<Claim>) {
    if para.is_empty() {
        return;
    }
    let mut joined = String::new();
    let mut lstart: Vec<usize> = Vec::new();
    for (i, (_, text)) in para.iter().enumerate() {
        if i > 0 {
            joined.push(' ');
        }
        lstart.push(joined.len());
        joined.push_str(text);
    }
    let jb = joined.as_bytes();
    let scan = blank_code(jb);
    let mut off = 0usize;
    while off < scan.len() {
        let m = (off..scan.len()).find_map(|i| match_pred(&scan, i).map(|(e, w)| (i, e, w)));
        let (ms, me, word) = match m {
            Some(v) => v,
            None => break,
        };
        let prefix = &jb[..ms];
        if let Some(path) = bound_path(prefix) {
            let mut li = 0usize;
            for (i, s) in lstart.iter().enumerate() {
                if *s <= ms {
                    li = i;
                }
            }
            out.push(Claim {
                file: file.to_string(),
                lineno: para[li].0,
                path,
                pred: word.to_string(),
            });
        }
        off = me;
    }
    para.clear();
}

// spec: canon-kit/SPEC.md §check-tracking-claim — the fixed predicate vocabulary, matched
// leftmost-longest and word-bounded on both sides
fn match_pred(b: &[u8], i: usize) -> Option<(usize, &'static str)> {
    let mut best: Option<(usize, &'static str)> = None;
    let mut starts: Vec<usize> = Vec::new();
    if i == 0 {
        starts.push(0);
    }
    if i < b.len() && !b[i].is_ascii_alphabetic() {
        starts.push(1);
    }
    for lead in starts {
        let p = i + lead;
        if !b[p..].starts_with(b"is") {
            continue;
        }
        let mut sp = p + 2;
        while sp < b.len() && is_space(b[sp]) {
            sp += 1;
        }
        if sp == p + 2 {
            continue;
        }
        for w in PREDICATES {
            if !b[sp..].starts_with(w.as_bytes()) {
                continue;
            }
            let after = sp + w.len();
            if after == b.len() {
                if best.map(|(e, _)| after > e).unwrap_or(true) {
                    best = Some((after, w));
                }
            } else if !b[after].is_ascii_alphabetic()
                && best.map(|(e, _)| after + 1 > e).unwrap_or(true)
            {
                best = Some((after + 1, w));
            }
        }
    }
    best
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\x0b' | b'\x0c' | b'\r' | b'\n')
}

// spec: canon-kit/SPEC.md §check-tracking-claim — the binding is adjacency: only whitespace
// may sit between the path's closing backtick and the predicate
fn bound_path(prefix: &[u8]) -> Option<String> {
    let mut e = prefix.len();
    while e > 0 && is_space(prefix[e - 1]) {
        e -= 1;
    }
    if e == 0 || prefix[e - 1] != b'`' {
        return None;
    }
    let p = &prefix[..e - 1];
    let open = p.iter().rposition(|&c| c == b'`')?;
    let tok = String::from_utf8_lossy(&p[open + 1..]).into_owned();
    if is_pathish(&tok) {
        Some(tok)
    } else {
        None
    }
}

// spec: canon-kit/SPEC.md §check-tracking-claim — the shape a backticked token must have to
// be read as a path at all
fn is_pathish(t: &str) -> bool {
    let ok = |c: u8| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'_' | b'~' | b'-');
    let b = t.as_bytes();
    if b.is_empty() {
        return false;
    }
    let body = if b[b.len() - 1] == b'/' {
        &b[..b.len() - 1]
    } else {
        b
    };
    if body.is_empty() {
        return false;
    }
    for seg in body.split(|&c| c == b'/') {
        if seg.is_empty() || !seg.iter().all(|&c| ok(c)) {
            return false;
        }
    }
    t.contains('/') || t.contains('.')
}

// spec: canon-kit/SPEC.md §check-tracking-claim — a predicate inside inline code is not a
// claim; blanking rather than deleting keeps the paragraph's offsets intact
fn blank_code(b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(b.len());
    let mut incode = false;
    for &c in b {
        if c == b'`' {
            incode = !incode;
            out.push(b' ');
            continue;
        }
        out.push(if incode { b' ' } else { c });
    }
    out
}
