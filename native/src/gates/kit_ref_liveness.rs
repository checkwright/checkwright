// spec: canon-kit/SPEC.md §Layout and configuration — every kit-name reference in a tracked file
// resolves to a live kit root: a slash/line-anchored <name>-kit or gate-sdk path segment names a
// gate_kit_roots dir, and a live-prefix kit knob resolves to a tracked kit knob
use crate::fresh;
use crate::proc;
use crate::walk;
use std::collections::HashSet;
use std::path::Path;

// spec: gate-sdk/SPEC.md §Fail-closed contract — the refusals carry their own prefix, because
// this gate's own is not one string: its own name on the three it words itself, and
// `fail_closed`'s output-contract token on the ones it delegates
pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}", e);
            2
        }
    }
}

// spec: canon-kit/SPEC.md §Layout and configuration — awk's `/[A-Z][A-Z0-9_]+/`: a run of at
// least two, opening on a letter. Hand-compiled, because the pattern is the gate's own
// (gate-sdk/SPEC.md §The POSIX ERE matcher's boundary).
fn knob_runs(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if !b[i].is_ascii_uppercase() {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < b.len() && (b[j].is_ascii_uppercase() || b[j].is_ascii_digit() || b[j] == b'_') {
            j += 1;
        }
        if j - i >= 2 {
            out.push(&line[i..j]);
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

// spec: canon-kit/SPEC.md §Layout and configuration — the path-segment pattern, hand-compiled
// leftmost-longest: hyphen-joined lower-alnum segments closing on a `kit` segment, or the
// literal SDK root name; where both match at one offset the longer wins, as POSIX rules.
fn kit_token_at(b: &[u8], at: usize) -> Option<usize> {
    let seg = |mut i: usize| -> usize {
        while i < b.len() && (b[i].is_ascii_lowercase() || b[i].is_ascii_digit()) {
            i += 1;
        }
        i
    };
    let mut best: Option<usize> = None;
    let mut i = seg(at);
    if i > at {
        loop {
            if i >= b.len() || b[i] != b'-' {
                break;
            }
            let next = seg(i + 1);
            if next == i + 1 {
                break;
            }
            if &b[i + 1..next] == b"kit" {
                best = Some(next);
            }
            i = next;
        }
    }
    if b.len() >= at + 8 && &b[at..at + 8] == b"gate-sdk" && best.map(|e| e < at + 8).unwrap_or(true)
    {
        best = Some(at + 8);
    }
    best
}

struct PathHit {
    token: String,
    before: u8,
    after: u8,
}

// spec: canon-kit/SPEC.md §Layout and configuration — the shell scanner re-anchors on the
// remainder after each match, so a token opening exactly where the last one closed sees a line
// boundary rather than the previous token's last byte. Reproduced rather than tidied.
fn kit_tokens(line: &str) -> Vec<PathHit> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let (mut i, mut cut) = (0usize, 0usize);
    while i < b.len() {
        if let Some(end) = kit_token_at(b, i) {
            out.push(PathHit {
                token: line[i..end].to_string(),
                before: if i > cut { b[i - 1] } else { b'/' },
                after: if end < b.len() { b[end] } else { b'/' },
            });
            i = end;
            cut = end;
        } else {
            i += 1;
        }
    }
    out
}

// spec: canon-kit/SPEC.md §Layout and configuration — the *other* knob scan, a different rule
// from `knob_runs` above: building the defined set anchors on the prefix wherever it occurs, so
// a prefix embedded in a longer run is a hit here and is not one in the file scanner.
fn prefixed_knobs<'a>(line: &'a str, prefixes: &[String]) -> Vec<&'a str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        // spec: canon-kit/SPEC.md §Layout and configuration — the scan steps by byte, because a
        // prefix may occur anywhere in a line the rest of which is not ASCII
        let hit = prefixes
            .iter()
            .filter(|p| b.len() - i >= p.len() && &b[i..i + p.len()] == p.as_bytes())
            .map(|p| p.len())
            .max();
        match hit {
            None => i += 1,
            Some(len) => {
                let mut j = i + len;
                while j < b.len()
                    && (b[j].is_ascii_uppercase() || b[j].is_ascii_digit() || b[j] == b'_')
                {
                    j += 1;
                }
                out.push(&line[i..j]);
                i = j;
            }
        }
    }
    out
}

fn prefix_of(root: &str) -> String {
    let base = root.rsplit('/').next().unwrap_or(root);
    format!("{}_", base.to_ascii_uppercase().replace('-', "_"))
}

fn rule(args: &[String]) -> Result<i32, String> {
    let scanroot = fresh::positional(args, 0, ".");
    let top = fresh::toplevel().map_err(|_| {
        "check-kit-ref-liveness: not a git repository — cannot enumerate tracked paths/knobs"
            .to_string()
    })?;

    let mut live: HashSet<String> = HashSet::new();
    let mut roots: Vec<String> = Vec::new();
    let mut prefixes: Vec<String> = Vec::new();
    for root in walk::kit_roots_rel().map_err(|e| format!("check-kit-ref-liveness: {}", e))? {
        if root.is_empty() {
            continue;
        }
        let root = root.trim_end_matches('/').to_string();
        live.insert(root.rsplit('/').next().unwrap_or(&root).to_string());
        prefixes.push(prefix_of(&root));
        roots.push(root);
    }
    if live.is_empty() {
        return Err("check-kit-ref-liveness: gate_kit_roots enumerated no roots".to_string());
    }

    // spec: canon-kit/SPEC.md §Layout and configuration — the defined-knob set, out of the
    // tracked kit sources themselves: `git grep` over the kit roots, minus prose and fixtures
    let pattern = format!("({})[A-Z0-9_]*", prefixes.join("|"));
    let mut argv: Vec<&str> = vec!["-C", &top, "grep", "-h", "-E", &pattern, "--"];
    for r in &roots {
        argv.push(r);
    }
    argv.push(":!*.md");
    argv.push(":!*/gate-tests/*");
    let completed = proc::run("git", &argv).map_err(|e| format!("check-kit-ref-liveness: {}", e))?;
    let code = completed.code().unwrap_or(-1);
    if code > 1 {
        return Err(format!(
            "check-kit-ref-liveness: git grep failed (exit {}) building the knob set",
            code
        ));
    }
    let hits = completed
        .stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
        .unwrap_or_default();
    let mut defined: HashSet<String> = HashSet::new();
    for line in hits.lines() {
        for run in prefixed_knobs(line, &prefixes) {
            defined.insert(run.to_string());
        }
    }

    // spec: canon-kit/SPEC.md §Layout and configuration — the queue is design-ahead and names
    // future knobs and paths, so it is valved out by basename; the knob crosses the config
    // bridge rather than carrying a crate-side default (gate-sdk/SPEC.md §lib/gate.sh)
    let queue_file = walk::knob_scalar("GATE_SDK_QUEUE_FILE")
        .map_err(|e| format!("check-kit-ref-liveness: {}", e))?;
    let queue_base = queue_file.rsplit('/').next().unwrap_or(&queue_file).to_string();
    let prune = walk::prune_dirs().map_err(|e| format!("check-kit-ref-liveness: {}", e))?;

    let ls = proc::run("git", &["ls-files", "--", scanroot])
        .map_err(|e| format!("check-kit-ref-liveness: {}", e))?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            return Err(format!(
                "KIT-REF-LIVENESS: {}",
                fresh::fail_closed("git-ls-files", ls.code())
            ))
        }
    };

    let mut files: Vec<String> = Vec::new();
    for path in listing.lines() {
        if path.is_empty() {
            continue;
        }
        if walk::path_pruned(path, &prune) {
            continue;
        }
        if path.starts_with("docs/posts/") || path == "docs/evidence-data.md" {
            continue;
        }
        let base = path.rsplit('/').next().unwrap_or(path);
        if base.starts_with("SPEC-") && base.ends_with(".md") {
            continue;
        }
        if base == queue_base {
            continue;
        }
        if !Path::new(path).is_file() {
            continue;
        }
        files.push(path.to_string());
    }
    let scanned = files.len();

    if files.is_empty() {
        println!(
            "KIT-REF-LIVENESS: clean (0 tracked file(s) under {} after valves; nothing to resolve)",
            scanroot
        );
        return Ok(0);
    }

    let mut bad: Vec<String> = Vec::new();
    for path in &files {
        let text = fresh::read_captured(path)
            .map_err(|_| format!("KIT-REF-LIVENESS: {}", fresh::fail_closed("awk", Some(2))))?;
        for (n, line) in fresh::file_lines(&text).iter().enumerate() {
            let fnr = n + 1;
            for hit in kit_tokens(line) {
                // spec: canon-kit/SPEC.md §Layout and configuration — a path *segment*: the
                // characters bracketing the token are slashes, with a line boundary counting
                // as one
                let (before, after) = (hit.before, hit.after);
                if before == b'/' && after == b'/' && !live.contains(&hit.token) {
                    bad.push(format!(
                        "{}:{}: path segment <{}> names no live kit root",
                        path, fnr, hit.token
                    ));
                }
            }
            for run in knob_runs(line) {
                if !prefixes.iter().any(|p| run.starts_with(p.as_str())) {
                    continue;
                }
                if !knob_ok(run, &defined, &prefixes) {
                    bad.push(format!(
                        "{}:{}: kit knob <{}> resolves to no tracked kit knob",
                        path, fnr, run
                    ));
                }
            }
        }
    }

    if !bad.is_empty() {
        println!("check-kit-ref-liveness: tracked file(s) reference a kit that names no live root:");
        for b in &bad {
            println!("  {}", b);
        }
        println!("  help: a kit was renamed or retired and a reference dangles — update the path");
        println!("        segment or knob to a live kit (gate_kit_roots), or delete the reference.");
        return Ok(1);
    }

    println!(
        "KIT-REF-LIVENESS: clean ({} tracked file(s) scanned under {}; every kit path segment + live-prefix knob resolves)",
        scanned, scanroot
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §Layout and configuration — an exact occurrence, or either side of a
// family-stem match: a stem naming defined members, or a member of a defined stem. A bare kit
// prefix is not a stem, or every knob of that kit would resolve against the prefix alone.
fn knob_ok(t: &str, defined: &HashSet<String>, prefixes: &[String]) -> bool {
    if defined.contains(t) {
        return true;
    }
    for k in defined {
        if t.ends_with('_') && k.starts_with(t) {
            return true;
        }
        if k.ends_with('_') && t.starts_with(k.as_str()) && !prefixes.iter().any(|p| p == k) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §Layout and configuration — leftmost-longest over the two
    // alternatives, which is where a naive scan and POSIX part company
    #[test]
    fn a_kit_token_is_the_longest_hyphen_run_closing_on_kit() {
        let toks = |s: &str| -> Vec<String> { kit_tokens(s).into_iter().map(|h| h.token).collect() };
        assert_eq!(toks("/canon-kit/"), vec!["canon-kit"]);
        assert_eq!(toks("gate-sdk/lib"), vec!["gate-sdk"]);
        assert_eq!(toks("a-kit-b-kit"), vec!["a-kit-b-kit"]);
        assert_eq!(toks("lifecycle-kit-extra"), vec!["lifecycle-kit"]);
        assert!(toks("no tokens here").is_empty());
        assert!(toks("kit").is_empty());
    }

    // spec: canon-kit/SPEC.md §Layout and configuration — the file scanner's own run shape:
    // opens on a letter, at least two long, and unanchored to any word boundary
    // comment-tier-exempt: the fixture names are assembled from parts rather than spelled, so
    // this gate scanning its own source finds no knob token that resolves to nothing
    #[test]
    fn a_knob_run_opens_on_a_letter_and_is_at_least_two_long() {
        let pfx = "GATE_SDK_";
        let live = format!("{}X and A", pfx);
        assert_eq!(knob_runs(&live), vec![format!("{}X", pfx)]);
        assert_eq!(knob_runs("xGATE_SDKy"), vec!["GATE_SDK"]);
        assert!(knob_runs("lower case").is_empty());
    }

    // spec: canon-kit/SPEC.md §Layout and configuration — the stem arms, and the bare-prefix
    // exclusion that keeps a whole family from resolving against its prefix alone
    #[test]
    fn the_stem_arms_resolve_in_both_directions_but_never_off_a_bare_prefix() {
        let pfx = "GATE_SDK_".to_string();
        let known = format!("{}ROOT", pfx);
        let unknown = format!("{}NOSUCHNAME", pfx);
        let prefixes = vec![pfx.clone()];
        let defined: HashSet<String> = [known.clone()].into_iter().collect();
        assert!(knob_ok(&known, &defined, &prefixes));
        assert!(knob_ok(&pfx, &defined, &prefixes));
        assert!(!knob_ok(&unknown, &defined, &prefixes));
        let stem: HashSet<String> = [pfx.clone()].into_iter().collect();
        assert!(!knob_ok(&unknown, &stem, &prefixes));
    }
}
