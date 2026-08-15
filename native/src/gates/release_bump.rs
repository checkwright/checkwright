// spec: docs/install.md §Versioning — the derivable bump floor: a release note declaring
// tightened gates or renamed knobs, or inheriting an outstanding deferred release's floor, may
// not ride a patch-only bump over its predecessor
use crate::declaration;
use crate::proc;
use crate::walk;
use std::path::Path;

const DEFAULT_POSTS: &str = "docs/posts";
const DEFAULT_DISPOSITION: &str = ".workflow/release-disposition.txt";
const GRAMMAR: &str = "<major>.<minor>.<patch>, each a run of ASCII digits";

// spec: gate-sdk/SPEC.md §The declaration cohort — ordering is defined over a stated grammar
// rather than reproduced from `sort -V`, whose prerelease order contradicts the semver line this
// gate's own subject is; a token outside the grammar is a refusal, never a guessed order.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version(u64, u64, u64);

// spec: gate-sdk/SPEC.md §The declaration cohort — the refusal names the token, the file or
// disposition line it came from, and the grammar: a refusal whose text does not name where the
// token came from sends its reader to the wrong file.
fn parse_version(token: &str, source: &str) -> Result<Version, String> {
    let fields: Vec<&str> = token.split('.').collect();
    if fields.len() == 3 {
        let mut n = [0u64; 3];
        let mut ok = true;
        for (i, f) in fields.iter().enumerate() {
            match f.parse::<u64>() {
                Ok(v) if !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()) => n[i] = v,
                _ => ok = false,
            }
        }
        if ok {
            return Ok(Version(n[0], n[1], n[2]));
        }
    }
    Err(format!(
        "version token '{}' from {} is outside the grammar this gate orders ({}) — the ordering could not be derived; treating as failure (not clean).\n  help: a prerelease or build-metadata suffix has no ruled order here (docs/install.md §Versioning names where that ruling is owed); re-key the token to the triple, or land the ordering ruling first.",
        token, source, GRAMMAR
    ))
}

struct Row {
    version: Version,
    raw: String,
    file: String,
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — the note set is the posts dir filtered by the
// `release:` front-matter key; the announcement post carries no front matter and is not a note
pub fn front_matter_release(text: &str) -> Option<String> {
    let mut fm = 0usize;
    for line in text.lines() {
        if line.starts_with("---")
            && line[3..]
                .bytes()
                .all(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\x0b' | b'\x0c'))
        {
            fm += 1;
            continue;
        }
        if fm == 1 {
            if let Some(rest) = line.strip_prefix("release:") {
                return Some(rest.trim_start_matches([' ', '\t']).to_string());
            }
        }
    }
    None
}

// spec: docs/install.md §Versioning — history ∪ live, the reader every truncated evidence file
// needs. The `git log` arm silences its own failure and yields no historical disposition, which
// the shell holder does too; the branch is unreachable in a tree that has a repository.
fn collect_dispositions(file: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Ok(c) = proc::run(
        "git",
        &["log", "--reverse", "--format=%H", "-p", "-U0", "--", file],
    ) {
        if let Some(bytes) = c.stdout() {
            for line in String::from_utf8_lossy(bytes).lines() {
                if let Some(rest) = line.strip_prefix('+') {
                    if is_disposition(rest) {
                        out.push(rest.to_string());
                    }
                }
            }
        }
    }
    if let Ok(text) = std::fs::read_to_string(file) {
        for line in text.lines() {
            if is_disposition(line) {
                out.push(line.to_string());
            }
        }
    }
    // spec: gate-sdk/SPEC.md §The declaration cohort — `sort -u` here is a byte sort with dedupe
    // over whole lines, not version ordering; it ports as one.
    out.sort();
    out.dedup();
    out
}

// spec: lifecycle-kit/SPEC.md §templates/stages/ — the disposition line grammar this gate reads:
// a bare iteration slug, the literal keyword, then the value
fn is_disposition(line: &str) -> bool {
    let b = line.as_bytes();
    if b.is_empty() || !(b[0].is_ascii_lowercase() || b[0].is_ascii_digit()) {
        return false;
    }
    let mut i = 0usize;
    while i < b.len() && (b[i].is_ascii_lowercase() || b[i].is_ascii_digit() || b[i] == b'-') {
        i += 1;
    }
    line[i..].starts_with(" release ")
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — an unreadable file is an error the caller
// surfaces, never a silently smaller corpus; bytes that are not UTF-8 are read as the shell
// holder's tools read them rather than refused
pub fn read_text(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-release-bump: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let posts = args
        .first()
        .filter(|a| !a.is_empty())
        .map(String::as_str)
        .unwrap_or(DEFAULT_POSTS);
    let disposition = args
        .get(1)
        .filter(|a| !a.is_empty())
        .map(String::as_str)
        .unwrap_or(DEFAULT_DISPOSITION);
    if !Path::new(posts).is_dir() {
        return Err(format!("posts dir not found: {}", posts));
    }

    let mut rows: Vec<Row> = Vec::new();
    for f in walk::glob_files(Path::new(posts), &["*.md".to_string()])? {
        let path = f.display().to_string();
        let text = read_text(&path)?;
        if let Some(v) = front_matter_release(&text) {
            if v.is_empty() {
                continue;
            }
            let raw = v.strip_prefix('v').unwrap_or(&v).to_string();
            rows.push(Row {
                version: parse_version(&raw, &path)?,
                raw,
                file: path,
            });
        }
    }

    // spec: lifecycle-kit/SPEC.md §templates/stages/ — a deferral is outstanding until a
    // disposition line releases at or above its version; nothing tracks discharge.
    let mut deferred: Vec<(Version, String)> = Vec::new();
    let mut released: Vec<Version> = Vec::new();
    for line in collect_dispositions(disposition) {
        let value = line.split_whitespace().nth(2).unwrap_or("");
        if let Some(v) = value.strip_prefix("deferred:v") {
            deferred.push((parse_version(v, &line)?, v.to_string()));
        } else if let Some(v) = value.strip_prefix('v') {
            released.push(parse_version(v, &line)?);
        }
    }
    let mut floor: Option<&(Version, String)> = None;
    for d in &deferred {
        if released.iter().any(|r| *r >= d.0) {
            continue;
        }
        if floor.map(|f| d.0 >= f.0).unwrap_or(true) {
            floor = Some(d);
        }
    }

    if rows.len() < 2 {
        if let Some(f) = floor {
            println!("check-release-bump: an outstanding deferred release (v{}, {}) floors the newest note, and a single-note tree cannot ride it out:", f.1, disposition);
            println!("  help: cut the note at v{} or above, or discharge the deferral with a disposition line releasing at or above it.", f.1);
            return Ok(1);
        }
        println!(
            "RELEASE-BUMP: clean ({} release note(s) under {} — no predecessor to derive a floor against)",
            rows.len(),
            posts
        );
        return Ok(0);
    }

    // spec: gate-sdk/SPEC.md §The declaration cohort — the row form ties on the path's byte order
    rows.sort_by(|a, b| a.version.cmp(&b.version).then_with(|| a.file.cmp(&b.file)));
    let newest = &rows[rows.len() - 1];
    let prev = &rows[rows.len() - 2];

    // spec: docs/install.md §The upgrade contract — the In brief presence assertion binds a note
    // under composition; a note published before the section existed is history and is not
    // retro-fitted
    let tag = format!("refs/tags/v{}", newest.raw);
    let under_composition = proc::run("git", &["rev-parse", "-q", "--verify", &tag])?.code() != Some(0);
    let text = read_text(&newest.file)?;
    let in_brief_state = if under_composition {
        if declaration::section_bullets(&text, "In brief").is_none() {
            return Err(format!("newest note {} is under composition (v{} carries no tag) and has no 'In brief' section — the 30-second human read is a fixed section, not optional (docs/install.md §The upgrade contract owns the note grammar)", newest.file, newest.raw));
        }
        "asserted"
    } else {
        "dormant"
    };

    // spec: docs/install.md §The upgrade contract — every fixed section must be present, and the
    // declaration-bearing ones derive the floor, where non-empty = at least one bullet
    let count = |section: &str| -> Result<usize, String> {
        declaration::section_bullets(&text, section)
            .map(|b| b.len())
            .ok_or_else(|| format!("newest note {} has no '{}' section — the floor cannot be derived (docs/install.md §The upgrade contract owns the note grammar)", newest.file, section))
    };
    let tg = count("Tightened gates")?;
    let rk = count("Renamed knobs")?;
    let bc = count("Behavior changes")?;

    let patch_only = newest.version.0 == prev.version.0 && newest.version.1 == prev.version.1;
    if patch_only && (tg > 0 || rk > 0 || bc > 0 || floor.is_some()) {
        println!("check-release-bump: v{} is a patch-only bump over v{}, but its note carries phase-B work (docs/install.md §Versioning — the floor is minor):", newest.raw, prev.raw);
        if tg > 0 {
            println!("  {}: {} tightened-gate bullet(s)", newest.file, tg);
        }
        if rk > 0 {
            println!("  {}: {} renamed-knob bullet(s)", newest.file, rk);
        }
        if bc > 0 {
            println!("  {}: {} behavior-change bullet(s)", newest.file, bc);
        }
        if let Some(f) = floor {
            println!("  {}: an outstanding deferred release (v{}) whose unconsumed criteria this note inherits", disposition, f.1);
        }
        println!("  help: bump the minor instead (re-key the note's 'release:' and re-tag the plan), or move the declared work out of this release's note.");
        return Ok(1);
    }

    // spec: docs/install.md §Versioning — the floor's second input binds the next qualifying note
    // numerically, gated on under_composition (the In brief assertion's own "not retro-fitted
    // against history" rule).
    if under_composition {
        if let Some(f) = floor {
            if f.0 > newest.version {
                println!("check-release-bump: v{} falls below an outstanding deferred release (v{}) recorded in {} — docs/install.md §Versioning: a later note may not fall below that version:", newest.raw, f.1, disposition);
                println!("  help: bump to v{} or above, or discharge the deferral with a disposition line releasing at or above it.", f.1);
                return Ok(1);
            }
        }
    }

    let inheriting = match floor {
        Some(f) => format!(", inheriting outstanding deferral v{}", f.1),
        None => String::new(),
    };
    println!(
        "RELEASE-BUMP: clean (newest note v{} holds the derivable floor over v{}{}; {} note(s); In brief presence {})",
        newest.raw,
        prev.raw,
        inheriting,
        rows.len(),
        in_brief_state
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grammar_is_three_digit_runs_and_anything_else_is_a_named_refusal() {
        assert!(parse_version("1.0.0", "x").is_ok());
        assert_eq!(parse_version("10.2.30", "x").ok(), Some(Version(10, 2, 30)));
        for bad in ["1.0.0-rc1", "1.0", "1.0.0.1", "1.0.x", "v1.0.0", "1..0", ""] {
            let e = parse_version(bad, "a-source").expect_err("admitted a token outside the grammar");
            assert!(e.contains(bad) || bad.is_empty(), "the refusal did not name the token: {}", e);
            assert!(e.contains("a-source"), "the refusal did not name its source: {}", e);
            assert!(e.contains(GRAMMAR), "the refusal did not name the grammar: {}", e);
        }
    }

    // spec: gate-sdk/SPEC.md §The declaration cohort — field-wise numeric, which is where the
    // ordering parts company with a lexical one
    #[test]
    fn ordering_is_field_wise_numeric() {
        assert!(parse_version("0.10.0", "x").unwrap() > parse_version("0.9.0", "x").unwrap());
        assert!(parse_version("1.0.0", "x").unwrap() > parse_version("0.99.99", "x").unwrap());
        assert!(parse_version("0.1.2", "x").unwrap() > parse_version("0.1.1", "x").unwrap());
    }

    #[test]
    fn the_front_matter_key_is_read_from_the_first_block_only() {
        assert_eq!(
            front_matter_release("---\nrelease: v1.2.3\n---\n\n# x\n").as_deref(),
            Some("v1.2.3")
        );
        assert_eq!(front_matter_release("# no front matter\nrelease: v1.2.3\n"), None);
        assert_eq!(
            front_matter_release("---\ntitle: x\n---\n\nrelease: v1.2.3\n"),
            None
        );
    }

    #[test]
    fn a_disposition_line_is_a_slug_the_keyword_and_a_value() {
        assert!(is_disposition("some-iter release v0.1.0"));
        assert!(!is_disposition("Some-iter release v0.1.0"));
        assert!(!is_disposition("some-iter released v0.1.0"));
        assert!(!is_disposition("# a comment line"));
    }
}
