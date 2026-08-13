// spec: canon-kit/SPEC.md §check-todo-task-liveness — every TODO(task: <slug>) marker on a
// governed source resolves to a live queue task, stale-flagged on a done slug
use crate::spec;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-todo-task-liveness: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let queue = match args.get(1) {
        Some(q) => q.clone(),
        None => spec::knob_pub("CANON_KIT_QUEUE_FILE")?,
    };
    if !Path::new(&queue).is_file() {
        return Err(format!("queue file not found: {}", queue));
    }

    let slugs = spec::queue_slugs(Path::new(&queue))?;
    let surface = spec::comment_surface(root, false)?;

    let mut stale: Vec<String> = Vec::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    for f in &surface {
        let rel = spec::strip_dot_slash(f.strip_prefix(&format!("{}/", root)).unwrap_or(f));
        scanned += 1;
        let text = spec::read_text(Path::new(f))?;
        for (ln, raw) in text.lines().enumerate() {
            for slug in markers(raw) {
                if slugs.live.contains(&slug) {
                    continue;
                }
                if slugs.done.contains(&slug) {
                    stale.push(format!(
                        "{}:{}: TODO(task: {}) — task '{}' is done; drop the completed TODO",
                        rel,
                        ln + 1,
                        slug,
                        slug
                    ));
                } else {
                    unresolved.push(format!(
                        "{}:{}: TODO(task: {}) — no live task '{}' in {}",
                        rel,
                        ln + 1,
                        slug,
                        slug,
                        queue
                    ));
                }
            }
        }
    }

    if !stale.is_empty() || !unresolved.is_empty() {
        println!(
            "TODO-TASK-LIVENESS: {} violation(s):",
            stale.len() + unresolved.len()
        );
        for e in stale.iter().chain(unresolved.iter()) {
            println!("  {}", e);
        }
        println!("  help: a TODO(task: <slug>) binds a code site to a live queue task — point it at an active or deferred slug, or resolve the code and delete the marker once the task is done (a done or absent slug leaves the marker referencing nothing).");
        return Ok(1);
    }
    println!(
        "TODO-TASK-LIVENESS: clean ({} governed source(s); every TODO(task: <slug>) marker resolves to a live queue task)",
        scanned
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-todo-task-liveness — the marker needs a resolvable slug after
// the colon, so the bare roster literal a tool carries never self-matches. Every occurrence on
// the line is taken, the scan resuming past each match as awk's own loop does.
fn markers(line: &str) -> Vec<String> {
    const LEAD: &[u8] = b"TODO(task:";
    let b = line.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i + LEAD.len() <= b.len() {
        if &b[i..i + LEAD.len()] != LEAD {
            i += 1;
            continue;
        }
        let mut s = i + LEAD.len();
        while s < b.len() && (b[s] == b' ' || b[s] == b'\t') {
            s += 1;
        }
        if s >= b.len() || !(b[s].is_ascii_lowercase() || b[s].is_ascii_digit()) {
            i += 1;
            continue;
        }
        let mut e = s + 1;
        while e < b.len() && (b[e].is_ascii_lowercase() || b[e].is_ascii_digit() || b[e] == b'-') {
            e += 1;
        }
        out.push(String::from_utf8_lossy(&b[s..e]).into_owned());
        i = e;
    }
    out
}
