// spec: installer/SPEC.md §Versioning — the derivable bump floor: a release note carrying a bullet
// in any declaration-bearing section, or inheriting an outstanding deferred release's floor, may
// not ride a patch-only bump over its predecessor
use crate::declaration;
use crate::release_sections;
use crate::spec;
use crate::{proc, programs};
use crate::walk;
use std::path::Path;

const GRAMMAR: &str = "<major>.<minor>.<patch>, each a run of ASCII digits";

// spec: gate-sdk/SPEC.md §The declaration cohort — ordering is defined over a stated grammar
// rather than reproduced from `sort -V`, whose prerelease order contradicts the semver line this
// gate's own subject is; a token outside the grammar is a refusal, never a guessed order.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Version(u64, u64, u64);

// spec: gate-sdk/SPEC.md §The declaration cohort — the refusal names the token, the file or
// disposition line it came from, and the grammar: a refusal whose text does not name where the
// token came from sends its reader to the wrong file.
pub(crate) fn parse_version(token: &str, source: &str) -> Result<Version, String> {
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
        "version token '{}' from {} is outside the grammar this gate orders ({}) — the ordering could not be derived; treating as failure (not clean).\n  help: a prerelease or build-metadata suffix has no ruled order here (installer/SPEC.md §Versioning names where that ruling is owed); re-key the token to the triple, or land the ordering ruling first.",
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

// spec: installer/SPEC.md §Versioning — history ∪ live, the reader every truncated evidence file
// needs. The `git log` arm silences its own failure and yields no historical disposition, which
// the shell holder does too; the branch is unreachable in a tree that has a repository.
fn collect_dispositions(file: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Ok(c) = proc::run(
        &programs::GIT,
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

// spec: lifecycle-kit/SPEC.md §templates/stages/ — the release-disposition file's one derivation,
// read by this gate and check-front-door-verbs
pub(crate) fn disposition_file() -> Result<String, String> {
    Ok(format!("{}/release-disposition.txt", walk::knob_scalar("GATE_SDK_WORKFLOW_DIR")?))
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
    let posts = crate::fresh::positional_or_knob(args, 0, "GATE_LOCAL_RELEASE_POSTS_DIR")?;
    let posts = posts.as_str();
    let disposition = match args.get(1).filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => disposition_file()?,
    };
    let disposition = disposition.as_str();
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

    // spec: installer/SPEC.md §The upgrade contract — roster presence binds a note under
    // composition, under each section's own heading; a tagged note is history and is not
    // retro-fitted
    let tag = format!("refs/tags/v{}", newest.raw);
    let under_composition = proc::run(&programs::GIT, &["rev-parse", "-q", "--verify", &tag])?.code() != Some(0);
    let text = read_text(&newest.file)?;
    let roster = release_sections::roster()?;
    let roster_state = if under_composition {
        for s in &roster {
            if declaration::section_bullets(&text, &[s.heading.as_str()]).is_none() {
                return Err(format!("newest note {} is under composition (v{} carries no tag) and has no '{}' section — every roster section is fixed on a note under composition, not optional (installer/SPEC.md §The upgrade contract owns the note grammar)", newest.file, newest.raw, s.heading));
            }
        }
        "asserted"
    } else {
        "dormant"
    };

    // spec: installer/SPEC.md §The upgrade contract — the declaration-bearing sections derive the
    // floor, where non-empty = at least one bullet, a section found by an alias counts and one
    // absent from a tagged note counts zero
    let counts: Vec<(&release_sections::Section, usize)> = roster
        .iter()
        .filter(|s| s.role.declaration_bearing())
        .map(|s| (s, declaration::section_bullets(&text, &s.names()).map(|b| b.len()).unwrap_or(0)))
        .collect();

    if under_composition {
        let findings = table_findings(&text, &roster, &counts);
        if !findings.is_empty() {
            println!("check-release-bump: newest note {} (v{}, under composition) carries no summary table that agrees with its sections:", newest.file, newest.raw);
            for f in findings {
                println!("  {}", f);
            }
            println!("  help: the brief section opens with this table, one row per declaration-bearing section in roster order, each Entries cell that section's bullet count (installer/SPEC.md §The upgrade contract):");
            for line in expected_table(&counts) {
                println!("    {}", line);
            }
            return Ok(1);
        }
    }

    let patch_only = newest.version.0 == prev.version.0 && newest.version.1 == prev.version.1;
    if patch_only && (counts.iter().any(|(_, n)| *n > 0) || floor.is_some()) {
        println!("check-release-bump: v{} is a patch-only bump over v{}, but its note carries phase-B work (installer/SPEC.md §Versioning — the floor is minor):", newest.raw, prev.raw);
        for (s, n) in counts.iter().filter(|(_, n)| *n > 0) {
            println!("  {}: {} bullet(s) under '{}'", newest.file, n, s.heading);
        }
        if let Some(f) = floor {
            println!("  {}: an outstanding deferred release (v{}) whose unconsumed criteria this note inherits", disposition, f.1);
        }
        println!("  help: bump the minor instead (re-key the note's 'release:' and re-tag the plan), or move the declared work out of this release's note.");
        return Ok(1);
    }

    // spec: installer/SPEC.md §Versioning — the floor's second input binds the next qualifying note
    // numerically, gated on under_composition (the roster assertion's own "not retro-fitted
    // against history" rule).
    if under_composition {
        if let Some(f) = floor {
            if f.0 > newest.version {
                println!("check-release-bump: v{} falls below an outstanding deferred release (v{}) recorded in {} — installer/SPEC.md §Versioning: a later note may not fall below that version:", newest.raw, f.1, disposition);
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
        "RELEASE-BUMP: clean (newest note v{} holds the derivable floor over v{}{}; {} note(s); section roster and summary table {})",
        newest.raw,
        prev.raw,
        inheriting,
        rows.len(),
        roster_state
    );
    Ok(0)
}

const TABLE_HEADER: [&str; 3] = ["Section", "Entries", "Who acts"];

fn cells(line: &str) -> Vec<String> {
    let body = line.trim();
    let body = body.strip_prefix('|').unwrap_or(body);
    let body = body.strip_suffix('|').unwrap_or(body);
    body.split('|').map(|c| c.trim().to_string()).collect()
}

fn link(s: &release_sections::Section) -> String {
    format!("[{}](#{})", s.heading, spec::anchor_slug(&s.heading))
}

fn expected_table(counts: &[(&release_sections::Section, usize)]) -> Vec<String> {
    let mut out = vec![format!("| {} |", TABLE_HEADER.join(" | ")), "|---|---|---|".to_string()];
    out.extend(counts.iter().map(|(s, n)| format!("| {} | {} | <who acts> |", link(s), n)));
    out
}

// spec: installer/SPEC.md §The upgrade contract — the summary table under the brief section: its
// header, its row set equal to the declaration-bearing roles in roster order, each link naming its
// section's anchor and each count that section's bullets; the audience cell is not read
fn table_findings(
    text: &str,
    roster: &[release_sections::Section],
    counts: &[(&release_sections::Section, usize)],
) -> Vec<String> {
    let Some(brief) = release_sections::find(roster, release_sections::Role::Brief) else {
        return Vec::new();
    };
    let lines = declaration::section_lines(text, &[brief.heading.as_str()]).unwrap_or_default();
    let rows: Vec<Vec<String>> = lines.iter().filter(|l| l.trim_start().starts_with('|')).map(|l| cells(l)).collect();
    if rows.len() < 2 || rows[0] != TABLE_HEADER {
        return vec![format!("'{}' carries no table under the header | {} |", brief.heading, TABLE_HEADER.join(" | "))];
    }
    let body = &rows[2..];
    let mut out = Vec::new();
    if body.len() != counts.len() {
        out.push(format!("the table holds {} row(s) for {} declaration-bearing section(s)", body.len(), counts.len()));
    }
    for (i, (s, n)) in counts.iter().enumerate() {
        let Some(row) = body.get(i) else { break };
        let want = link(s);
        if row.first().map(String::as_str) != Some(want.as_str()) {
            out.push(format!("row {}: Section cell '{}' is not {}", i + 1, row.first().map(String::as_str).unwrap_or(""), want));
        }
        let got = row.get(1).map(String::as_str).unwrap_or("");
        if got.parse::<usize>().ok() != Some(*n) {
            out.push(format!("row {}: Entries cell '{}' is not the {} bullet(s) '{}' holds", i + 1, got, n, s.heading));
        }
    }
    out
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
