// spec: canon-kit/SPEC.md §check-manifest-temporal — no temporal-narration marker in
// governed manifest prose outside an exempt site; the marker vocabulary is a consumer ERE
// array, so it is the engine's first consumer rather than a hand-compiled kit literal
use crate::ere::Ere;
use crate::spec::{self, compile_pattern as compile};
use crate::walk;
use std::path::Path;

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-manifest-temporal: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    let all = spec::manifest_files_sorted_stripped(root)?;
    if all.is_empty() {
        println!("MANIFEST-TEMPORAL: clean (0 manifest file(s) found)");
        return Ok(0);
    }

    // spec: canon-kit/SPEC.md §check-manifest-temporal — path valve: a whole file whose
    // immutable dated narrative a heading name cannot address (dated posts)
    let paths = spec::knob_array_pub("CANON_KIT_TEMPORAL_EXEMPT_PATHS")?;
    let mut exempt_n = 0usize;
    let mut manifests: Vec<String> = Vec::new();
    for m in all {
        if paths.iter().any(|g| walk::pattern_match(g, &m)) {
            exempt_n += 1;
        } else {
            manifests.push(m);
        }
    }
    if manifests.is_empty() {
        println!(
            "MANIFEST-TEMPORAL: clean ({} path-exempt, no other manifest)",
            exempt_n
        );
        return Ok(0);
    }

    // spec: gate-sdk/SPEC.md §The POSIX ERE matcher — every consumer pattern compiles before
    // the first corpus line is read, so a pattern the substrate cannot honour exits 2 naming
    // the knob rather than scanning clean past what it meant
    let markers = spec::knob_array_pub("CANON_KIT_TEMPORAL_MARKERS")?;
    let mut compiled: Vec<(String, Ere)> = Vec::new();
    for m in &markers {
        if m.is_empty() {
            continue;
        }
        compiled.push((m.clone(), compile(m, "CANON_KIT_TEMPORAL_MARKERS")?));
    }

    let exempt_sections: Vec<String> = spec::knob_array_pub("CANON_KIT_TEMPORAL_EXEMPT_SECTIONS")?
        .iter()
        .map(|s| s.to_ascii_lowercase())
        .collect();

    let mut out: Vec<String> = Vec::new();
    for f in &manifests {
        let text = spec::read_text(Path::new(f))?;
        let mut in_fence = false;
        let mut exempt = false;
        let mut exempt_level = 0usize;
        let mut prev = String::new();
        for (idx, raw) in text.lines().enumerate() {
            let fnr = idx + 1;
            if spec::is_fence_line(raw) {
                in_fence = !in_fence;
                prev = raw.to_string();
                continue;
            }
            if in_fence {
                prev = raw.to_string();
                continue;
            }
            if let Some(lvl) = heading_level(raw) {
                if exempt && lvl <= exempt_level {
                    exempt = false;
                    exempt_level = 0;
                }
                let h = heading_text(raw).to_ascii_lowercase();
                if exempt_sections.contains(&h) {
                    exempt = true;
                    exempt_level = lvl;
                }
                prev = raw.to_string();
                continue;
            }
            if exempt {
                prev = raw.to_string();
                continue;
            }
            if raw.contains(EXEMPT_MARKER) || prev.contains(EXEMPT_MARKER) {
                prev = raw.to_string();
                continue;
            }
            // spec: canon-kit/SPEC.md §check-manifest-temporal — a marker named in inline code
            // is a meta-reference, not narration; the subject is folded and the pattern is not
            let scan = spec::strip_inline_code(raw.as_bytes());
            let low: String = String::from_utf8_lossy(&scan).to_ascii_lowercase();
            for (name, re) in &compiled {
                if re.is_match(&low) {
                    out.push(format!(
                        "  {}:{}  temporal-narration marker: {}",
                        f, fnr, name
                    ));
                    break;
                }
            }
            prev = raw.to_string();
        }
    }

    if !out.is_empty() {
        println!("check-manifest-temporal: temporal-narration marker(s) in manifest prose — a manifest states current behavior; history is derivable from git:");
        println!();
        for l in &out {
            println!("{}", l);
        }
        println!("  help: reword to state the current behavior only (drop the 'formerly…' framing); if the line is legitimately about the past, add a '<!-- manifest-temporal-exempt: <reason> -->' comment on it or the line directly above; a whole provenance section rides CANON_KIT_TEMPORAL_EXEMPT_SECTIONS");
        return Ok(1);
    }
    let suffix = if exempt_n > 0 {
        format!(", {} path-exempt", exempt_n)
    } else {
        String::new()
    };
    println!(
        "MANIFEST-TEMPORAL: clean ({} manifest file(s){}; no temporal-narration marker in governed prose outside an exempt site)",
        manifests.len(),
        suffix
    );
    Ok(0)
}

const EXEMPT_MARKER: &str = "manifest-temporal-exempt:";

// spec: canon-kit/SPEC.md §check-manifest-temporal — `^#{1,6}[[:space:]]` and its level
// count, a kit literal that hand-compiles
fn heading_level(raw: &str) -> Option<usize> {
    let b = raw.as_bytes();
    let mut n = 0usize;
    while n < b.len() && b[n] == b'#' {
        n += 1;
    }
    if n == 0 || n > 6 {
        return None;
    }
    match b.get(n) {
        Some(c) if is_space(*c) => Some(n),
        _ => None,
    }
}

// spec: canon-kit/SPEC.md §check-manifest-temporal — `sub(/^#{1,6}[[:space:]]+/, "", h)` and
// the trailing-space strip: substitutions over kit literals, written directly, which is why
// the engine owes no `replace`
fn heading_text(raw: &str) -> String {
    let b = raw.as_bytes();
    let mut i = 0usize;
    while i < b.len() && b[i] == b'#' {
        i += 1;
    }
    while i < b.len() && is_space(b[i]) {
        i += 1;
    }
    let mut e = b.len();
    while e > i && is_space(b[e - 1]) {
        e -= 1;
    }
    String::from_utf8_lossy(&b[i..e]).into_owned()
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}
