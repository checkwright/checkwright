// spec: canon-kit/SPEC.md §check-deprecation-task — every deprecation marker on a governed
// source binds task: <slug> to a live queue task
use crate::spec;
use std::path::Path;

const MARKERS: &str = "CANON_KIT_DEPRECATION_MARKERS";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-deprecation-task: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let markers = spec::knob_array_pub(MARKERS)?;
    if markers.is_empty() {
        println!("DEPRECATION-TASK: clean (no CANON_KIT_DEPRECATION_MARKERS configured; the deprecation-marker vocabulary is consumer config — nothing to resolve)");
        return Ok(0);
    }
    let queue = match args.get(1) {
        Some(q) => q.clone(),
        None => spec::knob_pub("CANON_KIT_QUEUE_FILE")?,
    };
    if !Path::new(&queue).is_file() {
        return Err(format!("queue file not found: {}", queue));
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — the consumer vocabulary is joined into
    // one alternation and *interpreted*, which is what puts this member on the engine
    let marker_re = spec::compile_pattern(&markers.join("|"), MARKERS)?;

    let slugs = spec::queue_slugs(Path::new(&queue))?;
    let surface = spec::comment_surface(root, false)?;

    let mut unbound: Vec<String> = Vec::new();
    let mut stale: Vec<String> = Vec::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut scanned = 0usize;
    for f in &surface {
        let rel = spec::strip_dot_slash(f.strip_prefix(&format!("{}/", root)).unwrap_or(f));
        scanned += 1;
        let text = spec::read_text(Path::new(f))?;
        for (idx, raw) in text.lines().enumerate() {
            let (ms, me) = match marker_re.find(raw) {
                Some(v) => v,
                None => continue,
            };
            let ln = idx + 1;
            let marker = &raw[ms..me];
            match binding(raw) {
                None => unbound.push(format!(
                    "{}:{}: deprecation marker '{}' — no 'task: <slug>' binding on the line",
                    rel, ln, marker
                )),
                Some(slug) => {
                    if slugs.live.contains(&slug) {
                        continue;
                    }
                    if slugs.done.contains(&slug) {
                        stale.push(format!(
                            "{}:{}: deprecation marker '{}' → task: {} is done; decommission the surface or bind a live task",
                            rel, ln, marker, slug
                        ));
                    } else {
                        unresolved.push(format!(
                            "{}:{}: deprecation marker '{}' → no live task '{}' in {}",
                            rel, ln, marker, slug, queue
                        ));
                    }
                }
            }
        }
    }

    let total = unbound.len() + stale.len() + unresolved.len();
    if total > 0 {
        println!("DEPRECATION-TASK: {} violation(s):", total);
        for e in unbound.iter().chain(stale.iter()).chain(unresolved.iter()) {
            println!("  {}", e);
        }
        println!("  help: a deprecation marker binds its surface to a live decommission task — add 'task: <slug>' on the marker line pointing at an active or deferred queue task, or decommission the surface and drop the marker once that task is done (an unbound marker, a done slug, or an absent slug all leave the deprecation tracking nothing).");
        return Ok(1);
    }
    println!(
        "DEPRECATION-TASK: clean ({} governed source(s); every deprecation marker binds task: <slug> to a live queue task)",
        scanned
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-deprecation-task — `task:[[:space:]]*[a-z0-9][a-z0-9-]*`, the
// first occurrence on the line, which is where awk's leftmost match lands
fn binding(line: &str) -> Option<String> {
    const LEAD: &[u8] = b"task:";
    let b = line.as_bytes();
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
        if s < b.len() && (b[s].is_ascii_lowercase() || b[s].is_ascii_digit()) {
            let mut e = s + 1;
            while e < b.len() && (b[e].is_ascii_lowercase() || b[e].is_ascii_digit() || b[e] == b'-')
            {
                e += 1;
            }
            return Some(String::from_utf8_lossy(&b[s..e]).into_owned());
        }
        i += 1;
    }
    None
}
