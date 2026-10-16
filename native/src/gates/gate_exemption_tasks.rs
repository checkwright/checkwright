// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — every temporary-disposition annotation a
// gate declaration carries names a live task: an exception-list element's until:, a descriptor's
// `# exception:` element's, or the declaration's own port-until: header field
use crate::{registry, walk};
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
const LIVE_OPEN: &[&str] = &["## New Features", "## Technical Debt", "## Deferred"]; // consumer-value-exempt: literals by the fail-open ground above
const LIVE_CLOSE: &[&str] = &["## Done", "## Lessons Learned"]; // consumer-value-exempt: literals by the fail-open ground above

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the live slug is an entry heading's whole
// text, `### <slug>` or `#### <slug>` for a sub-task; reading every slug-shaped token on every line
// is the fail-open
fn lead_line_slug(line: &str) -> Option<String> {
    let rest = line.strip_prefix("#### ").or_else(|| line.strip_prefix("### "))?;
    let t = rest.trim_end_matches([' ', '\t']).as_bytes();
    match registry::slug_at(t, 0) {
        Some(end) if end == t.len() => Some(String::from_utf8_lossy(t).into_owned()),
        _ => None,
    }
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

// spec: gate-sdk/SPEC.md §check-reads-couples — a one-level directory glob is pathname
// expansion rather than a recursive walk, which is why this member declares an empty read-root
// set: it resolves the corpus and never descends into one
fn declarations_in(dir: &str) -> Vec<String> {
    let mut out = walk::glob_entries(&format!("{}/*.sh", dir));
    out.extend(walk::glob_entries(&format!("{}/*.gate", dir)));
    out
}

fn header_hits(text: &str, opener: &str) -> Vec<(usize, String)> {
    if opener == "exception-list:" {
        let lines: Vec<&str> = text.lines().collect();
        return registry::exception_arrays(text).into_iter().map(|a| (a.header, lines[a.header - 1].to_string())).collect();
    }
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let t = lstrip(line);
        if !t.starts_with('#') {
            continue;
        }
        let rest = lstrip(&t[1..]);
        if rest.starts_with(opener) {
            out.push((idx + 1, line.to_string()));
        }
    }
    out
}

fn canon(p: &Path) -> Option<String> {
    walk::canonicalize(p)
}

// spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the scope rule, on walk's shared predicate
// rather than a second spelling: a temporary disposition is asserted against the queue of the tree
// that authored the declaration, so a kit-shipped one is out of scope in a vendoring tree
fn authoring_tree() -> Result<bool, String> {
    Ok(walk::authoring_tree(&walk::knob_scalar("GATE_SDK_NATIVE_CRATE")?))
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
            || (f.ends_with(".gate") && !registry::exemptions(&text).is_empty())
        {
            skipped += 1;
        }
    }

    let mut errors: Vec<String> = Vec::new();
    let mut arrays = 0usize;
    let mut described = 0usize;
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
    // halves of the declaration walk, in scope and out, canonicalized because the two producers
    // spell one file differently and a raw match would therefore never fire
    let seen: Vec<String> = scan_files
        .iter()
        .chain(oos_files.iter())
        .filter_map(|p| canon(Path::new(p)))
        .collect();
    let mut tree_texts: Vec<(String, String)> = Vec::new();
    for f in &tree_scoped {
        if canon(Path::new(f)).is_some_and(|c| seen.contains(&c)) {
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
            for el in registry::exception_arrays(text).into_iter().find(|a| a.header == lineno).unwrap().elements {
                if el.opening {
                    errors.push(format!("{}:{} — exemption element(s) on the array's opening '=(' line cannot carry a per-element disposition; put each element on its own line with a # until:/# permanent: comment", f, el.line));
                    continue;
                }
                match registry::disposition(&el.raw) {
                    registry::Disposition::Permanent => {}
                    registry::Disposition::Until(slug) => {
                        if !live.contains(&slug) {
                            errors.push(format!(
                                "{}:{} — # until: {} does not resolve to a live task (moved to Done, or missing from {})",
                                f, el.line, slug, queue
                            ));
                        }
                    }
                    registry::Disposition::Missing => errors.push(format!(
                        "{}:{} — exemption element carries neither # until: <slug> nor # permanent: <reason>",
                        f, el.line
                    )),
                }
            }
        }
    }

    // spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — the header-field arm and the descriptor
    // arm enter the walk independently of the '# exception-list:' marker, because a declaration
    // carrying only a header field or only `# exception:` lines is skipped by the trigger the array
    // arm opens on
    for (f, text) in texts.iter().filter(|(f, _)| f.ends_with(".gate")) {
        for ex in registry::exemptions(text) {
            described += 1;
            match ex.disposition {
                registry::Disposition::Permanent => {}
                registry::Disposition::Until(slug) => {
                    if !live.contains(&slug) {
                        errors.push(format!(
                            "{}:{} — # until: {} does not resolve to a live task (moved to Done, or missing from {})",
                            f, ex.line, slug, queue
                        ));
                    }
                }
                registry::Disposition::Missing => errors.push(format!(
                    "{}:{} — exemption element carries neither # until: <slug> nor # permanent: <reason>",
                    f, ex.line
                )),
            }
        }
    }

    for (f, text) in &texts {
        for (lineno, line) in header_hits(text, "port-until:") {
            let tail = match line.split_once("port-until:") {
                Some((_, t)) => t,
                None => continue,
            };
            let t = lstrip(tail).as_bytes();
            let mut n = 0usize;
            while n < t.len() && registry::is_slug_byte(t[n]) {
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
        "GATE-EXEMPTION-TASKS: clean ({} exemption array(s), {} descriptor exemption(s), {} '# port-until:' header field(s), {} kit-shipped declaration(s) out of scope, {} live task slug(s); every in-scope element declares until-with-live-task or permanent-with-reason and every in-scope held declaration names a live entry)",
        arrays, described, headers, skipped, live.len()
    );
    Ok(0)
}
