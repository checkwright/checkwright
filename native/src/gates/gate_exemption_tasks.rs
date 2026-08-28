// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — every temporary-disposition annotation a
// gate declaration carries names a live task: an exception-list element's until:, or the
// declaration's own port-until: header field
use crate::proc;
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-gate-exemption-tasks: {}", e);
            2
        }
    }
}

fn lstrip(s: &str) -> &str {
    s.trim_start_matches([' ', '\t'])
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the section set and the lead-line shape are
// literals and never knobs, and the crate's own knob-taking queue module is deliberately not
// reached: a consumer free to redefine the shape could redefine it back into the fail-open
const LIVE_OPEN: &[&str] = &["## New Features", "## Technical Debt", "## Deferred"];
const LIVE_CLOSE: &[&str] = &["## Done", "## Lessons Learned"];

fn is_slug_byte(c: u8) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-'
}

fn slug_at(b: &[u8], at: usize) -> Option<usize> {
    if at >= b.len() || !(b[at].is_ascii_lowercase() || b[at].is_ascii_digit()) {
        return None;
    }
    let mut j = at + 1;
    while j < b.len() && is_slug_byte(b[j]) {
        j += 1;
    }
    Some(j)
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the live slug is the bold lead-in of a
// bullet lead line, one per entry; reading every bold token on every line is the fail-open
fn lead_line_slug(line: &str) -> Option<String> {
    let t = lstrip(line).as_bytes();
    if t.is_empty() || t[0] != b'-' {
        return None;
    }
    let mut i = 1usize;
    let start = i;
    while i < t.len() && (t[i] == b' ' || t[i] == b'\t') {
        i += 1;
    }
    if i == start {
        return None;
    }
    if i + 2 >= t.len() || t[i] != b'*' || t[i + 1] != b'*' {
        return None;
    }
    let end = slug_at(t, i + 2)?;
    if end + 1 >= t.len() || t[end] != b'*' || t[end + 1] != b'*' {
        return None;
    }
    Some(String::from_utf8_lossy(&t[i + 2..end]).into_owned())
}

fn live_slugs(queue: &Path) -> Vec<String> {
    let text = match std::fs::read(queue) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => return Vec::new(),
    };
    let mut live = false;
    let mut out: Vec<String> = Vec::new();
    for line in text.lines() {
        if LIVE_OPEN.iter().any(|h| line.starts_with(h)) {
            live = true;
            continue;
        }
        if LIVE_CLOSE.iter().any(|h| line.starts_with(h)) {
            live = false;
        }
        if !live {
            continue;
        }
        if let Some(s) = lead_line_slug(line) {
            if !out.contains(&s) {
                out.push(s);
            }
        }
    }
    out
}

fn strip_comment(s: &str) -> &str {
    match s.find('#') {
        Some(i) => &s[..i],
        None => s,
    }
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the balance scan ignores a parenthesis
// inside a comment or a quoted string, so an element carrying one does not close the array early
fn strip_for_balance(s: &str) -> String {
    drop_pairs(&drop_pairs(strip_comment(s), b'"'), b'\'')
}

fn drop_pairs(s: &str, q: u8) -> String {
    let b = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == q {
            if let Some(off) = b[i + 1..].iter().position(|&c| c == q) {
                i = i + 1 + off + 1;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

enum Kind {
    OpenLine,
    Until(String),
    Permanent,
    None,
}

struct Element {
    lno: usize,
    kind: Kind,
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the `# until:` payload, taken off the
// first `#` that opens the annotation rather than off the whole line
fn until_slug(line: &str) -> Option<String> {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] != b'#' {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
            j += 1;
        }
        if line[j..].starts_with("until:") {
            let mut k = j + "until:".len();
            while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                k += 1;
            }
            if let Some(end) = slug_at(b, k) {
                return Some(String::from_utf8_lossy(&b[k..end]).into_owned());
            }
        }
        i += 1;
    }
    None
}

fn has_permanent(line: &str) -> bool {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'#' {
            let mut j = i + 1;
            while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
                j += 1;
            }
            if line[j..].starts_with("permanent:") {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn assignment_opens(line: &str) -> bool {
    let t = lstrip(line).as_bytes();
    if t.is_empty() || !(t[0].is_ascii_alphabetic() || t[0] == b'_') {
        return false;
    }
    let mut i = 1usize;
    while i < t.len() && (t[i].is_ascii_alphanumeric() || t[i] == b'_') {
        i += 1;
    }
    i + 1 < t.len() && t[i] == b'=' && t[i + 1] == b'('
}

fn parse_elements(text: &str, header_line: usize) -> Vec<Element> {
    let mut out: Vec<Element> = Vec::new();
    let mut asgn = 0usize;
    let mut started = false;
    let mut bal: i64 = 0;
    for (idx, line) in text.lines().enumerate() {
        let nr = idx + 1;
        if nr <= header_line {
            continue;
        }
        if asgn == 0 {
            if !assignment_opens(line) {
                continue;
            }
            asgn = nr;
            let after = match line.find('(') {
                Some(i) => &line[i + 1..],
                None => "",
            };
            let rest: String = strip_comment(after)
                .chars()
                .filter(|c| *c != ')' && !c.is_ascii_whitespace())
                .collect();
            if !rest.is_empty() {
                out.push(Element {
                    lno: nr,
                    kind: Kind::OpenLine,
                });
            }
        }
        if nr > asgn {
            let noc = strip_comment(line);
            if noc
                .chars()
                .any(|c| !c.is_ascii_whitespace() && c != '(' && c != ')')
            {
                let kind = match until_slug(line) {
                    Some(s) => Kind::Until(s),
                    None if has_permanent(line) => Kind::Permanent,
                    None => Kind::None,
                };
                out.push(Element { lno: nr, kind });
            }
        }
        let s2 = strip_for_balance(line);
        let o = s2.matches('(').count() as i64;
        let c = s2.matches(')').count() as i64;
        if o > 0 {
            started = true;
        }
        bal += o - c;
        if started && bal <= 0 {
            break;
        }
    }
    out
}

// spec: gate-sdk/SPEC.md §check-reads-couples — a one-level directory glob is pathname
// expansion rather than a recursive walk, which is why this member declares an empty read-root
// set: it resolves the corpus and never descends into one
fn declarations_in(dir: &str) -> Vec<String> {
    let mut out = walk::glob_entries(&format!("{}/*.sh", dir));
    out.extend(walk::glob_entries(&format!("{}/*.gate", dir)));
    out
}

fn header_hits(text: &str, opener: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let t = lstrip(line);
        if !t.starts_with('#') {
            continue;
        }
        let rest = lstrip(&t[1..]);
        if opener == "exception-list:" {
            if t.starts_with("# exception-list:") {
                out.push((idx + 1, line.to_string()));
            }
        } else if rest.starts_with(opener) {
            out.push((idx + 1, line.to_string()));
        }
    }
    out
}

fn canon(p: &Path) -> Option<String> {
    std::fs::canonicalize(p)
        .ok()
        .map(|c| c.display().to_string())
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the scope rule: a temporary disposition is
// asserted against the queue of the tree that authored the declaration, so a kit-shipped one is
// out of scope wherever this tree vendored that kit rather than authoring it
fn authoring_tree() -> Result<bool, String> {
    let crate_dir = walk::knob_scalar("GATE_SDK_NATIVE_CRATE")?;
    if !Path::new(&crate_dir).is_dir() {
        return Ok(false);
    }
    let out = match proc::run("git", &["-C", &crate_dir, "ls-files"]) {
        Ok(c) => c,
        Err(_) => return Ok(false),
    };
    Ok(out.stdout().map(|o| !o.is_empty()).unwrap_or(false))
}

fn rule(args: &[String]) -> Result<i32, String> {
    let queue = match args.first() {
        Some(q) => q.clone(),
        None => walk::knob_scalar("GATE_SDK_QUEUE_FILE")?,
    };
    let gates_dir = walk::knob_scalar("GATE_SDK_GATES_DIR")?;
    let dirs: Vec<String> = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        let mut d = vec![gates_dir.clone()];
        d.extend(walk::kit_roots_abs()?.into_iter().map(|k| format!("{}/checks", k)));
        d
    };

    let live = live_slugs(Path::new(&queue));
    let authoring = authoring_tree()?;
    let gates_dir_abs = canon(Path::new(&gates_dir));

    let mut scan_files: Vec<String> = Vec::new();
    let mut oos_files: Vec<String> = Vec::new();
    for d in &dirs {
        let in_scope = authoring
            || match (canon(Path::new(d)), gates_dir_abs.as_ref()) {
                (Some(a), Some(g)) => a == *g,
                _ => false,
            };
        if in_scope {
            scan_files.extend(declarations_in(d));
        } else {
            oos_files.extend(declarations_in(d));
        }
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the skipped set is counted rather than
    // dropped silently: a scope rule that quietly stopped asserting is indistinguishable from a
    // corpus with nothing to assert
    let mut skipped = 0usize;
    for f in &oos_files {
        let text = match std::fs::read(Path::new(f)) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(_) => continue,
        };
        if !header_hits(&text, "exception-list:").is_empty()
            || !header_hits(&text, "port-until:").is_empty()
        {
            skipped += 1;
        }
    }

    let mut errors: Vec<String> = Vec::new();
    let mut arrays = 0usize;
    let mut headers = 0usize;

    let mut texts: Vec<(String, String)> = Vec::new();
    for f in &scan_files {
        if let Ok(b) = std::fs::read(Path::new(f)) {
            texts.push((f.clone(), String::from_utf8_lossy(&b).into_owned()));
        }
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the header-field arm's corpus is the
    // tracked shell tree beside the declaration set and never instead of it, with the scope rule
    // lifted from a directory to a file so a vendoring adopter is held to no kit author's slug
    let tree_scoped: Vec<String> = walk::tracked_shell_tree()?;
    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the union de-duplicates against BOTH
    // halves of the declaration walk, in scope and out: a declaration already counted as skipped
    // would otherwise be counted a second time through the tree corpus that also reaches it
    let seen: Vec<String> = scan_files.iter().chain(oos_files.iter()).cloned().collect();
    let mut tree_texts: Vec<(String, String)> = Vec::new();
    for f in &tree_scoped {
        if seen.contains(f) {
            continue;
        }
        let in_scope = authoring || Path::new(f).starts_with(&gates_dir);
        let text = match std::fs::read(Path::new(f)) {
            Ok(b) => walk::header_block(&String::from_utf8_lossy(&b)),
            Err(_) => continue,
        };
        if in_scope {
            tree_texts.push((f.clone(), text));
        } else if !header_hits(&text, "port-until:").is_empty() {
            skipped += 1;
        }
    }

    for (f, text) in &texts {
        for (lineno, _) in header_hits(text, "exception-list:") {
            arrays += 1;
            for el in parse_elements(text, lineno) {
                match el.kind {
                    Kind::Permanent => {}
                    Kind::Until(slug) => {
                        if !live.contains(&slug) {
                            errors.push(format!(
                                "{}:{} — # until: {} does not resolve to a live task (moved to Done, or missing from {})",
                                f, el.lno, slug, queue
                            ));
                        }
                    }
                    Kind::OpenLine => errors.push(format!(
                        "{}:{} — exemption element(s) on the array's opening '=(' line cannot carry a per-element disposition; put each element on its own line with a # until:/# permanent: comment",
                        f, el.lno
                    )),
                    Kind::None => errors.push(format!(
                        "{}:{} — exemption element carries neither # until: <slug> nor # permanent: <reason>",
                        f, el.lno
                    )),
                }
            }
        }
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the header-field arm enters the walk
    // independently of the '# exception-list:' marker, because a declaration carrying only the
    // field is skipped by the trigger the array arm opens on
    for (f, text) in &texts {
        for (lineno, line) in header_hits(text, "port-until:") {
            let tail = match line.split_once("port-until:") {
                Some((_, t)) => t,
                None => continue,
            };
            let t = lstrip(tail).as_bytes();
            let mut n = 0usize;
            while n < t.len() && is_slug_byte(t[n]) {
                n += 1;
            }
            // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — a bare field is assertion G's
            // shape clause, not this gate's: there is no slug to resolve
            if n == 0 {
                continue;
            }
            let slug = String::from_utf8_lossy(&t[..n]).into_owned();
            headers += 1;
            if !live.contains(&slug) {
                errors.push(format!(
                    "{}:{} — # port-until: {} does not resolve to a live task (moved to Done, or missing from {})",
                    f, lineno, slug, queue
                ));
            }
        }
    }

    // spec: gate-sdk/SPEC.md §port-blockers — the tree half reads the shared disposition triple
    // rather than a narrower scan of its own, so the `--tree` arm and this gate cannot disagree
    // about which files declared a hold at all.
    for (f, text) in &tree_texts {
        let slug = match walk::disposition(text) {
            walk::Disposition::PortUntil(s) => s,
            _ => continue,
        };
        headers += 1;
        if !live.contains(&slug) {
            let lineno = header_hits(text, "port-until:")
                .first()
                .map(|(n, _)| *n)
                .unwrap_or(0);
            errors.push(format!(
                "{}:{} — # port-until: {} does not resolve to a live task (moved to Done, or missing from {})",
                f, lineno, slug, queue
            ));
        }
    }

    if !errors.is_empty() {
        println!("GATE-EXEMPTION-TASKS: {} violation(s):", errors.len());
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: annotate each exemption element '# until: <live-slug>' or '# permanent: <reason>', and");
        println!("        point each '# port-until: <slug>' header field at a live queue entry — when the blocker");
        println!("        lands and the entry moves to Done, the declaration is dropped rather than left behind");
        return Ok(1);
    }
    println!(
        "GATE-EXEMPTION-TASKS: clean ({} exemption array(s), {} '# port-until:' header field(s), {} kit-shipped declaration(s) out of scope, {} live task slug(s); every in-scope element declares until-with-live-task or permanent-with-reason and every in-scope held declaration names a live entry)",
        arrays, headers, skipped, live.len()
    );
    Ok(0)
}
