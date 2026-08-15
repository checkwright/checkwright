// spec: docs/install.md §The upgrade contract — while a release note is under composition its
// Tightened-gates token set equals the declaration surface it was composed from, both directions
use crate::declaration;
use crate::declaration::SectionVerdict;
use crate::gates::release_bump::{front_matter_release, read_text};
use crate::proc;
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const DEFAULT_POSTS: &str = "docs/posts";
const DEFAULT_DECL: &str = ".workflow/tightened-gates.txt";
const SECTION: &str = "Tightened gates";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-tightened-gates-note-parity: {}", e);
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
    let decl_file = args
        .get(1)
        .filter(|a| !a.is_empty())
        .map(String::as_str)
        .unwrap_or(DEFAULT_DECL);
    if !Path::new(posts).is_dir() {
        return Err(format!("posts dir not found: {}", posts));
    }

    // spec: gate-sdk/SPEC.md §lib/declaration.sh — the note set is the posts dir filtered by the
    // `release:` key; the tag probe's non-zero status is the *no such tag* verdict rather than a
    // failure, so it is read through the exit code (§Fail-closed contract).
    let mut untagged: Vec<(String, String)> = Vec::new();
    for f in walk::glob_files(Path::new(posts), &["*.md".to_string()])? {
        let path = f.display().to_string();
        let text = read_text(&path)?;
        let v = match front_matter_release(&text) {
            Some(v) if !v.is_empty() => v,
            _ => continue,
        };
        let tag = format!("refs/tags/{}", v);
        if proc::run("git", &["rev-parse", "-q", "--verify", &tag])?.code() != Some(0) {
            untagged.push((v, path));
        }
    }

    if untagged.len() > 1 {
        eprintln!("check-tightened-gates-note-parity: {} carries more than one untagged release note, a state the release choreography does not admit:", posts);
        for (v, f) in &untagged {
            eprintln!("  {}\t{}", v, f);
        }
        eprintln!("  help: exactly one note is in flight at a time — tag the released one or remove the stray note.");
        return Ok(2);
    }
    if untagged.is_empty() {
        println!("TIGHTENED-GATES-NOTE-PARITY: dormant (every release note under {} is tagged, so the surface has been drained by contract and there is nothing to compare)", posts);
        return Ok(0);
    }
    let (note_v, note_f) = &untagged[0];

    // spec: gate-sdk/SPEC.md §upgrade-smoke — the surface's required header line. It is checked
    // before the record arm reads the file, so the arm's *missing file is the empty set* rule —
    // which stays at the caller — is unreachable from here rather than overridden.
    let decl_text = std::fs::read(decl_file)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    if !decl_text.lines().next().unwrap_or("").starts_with('#') {
        return Err(format!("{} is missing its required header line, so the declaration surface cannot be established (gate-sdk/SPEC.md §upgrade-smoke owns its contract)", decl_file));
    }

    let note_text = read_text(note_f)?;
    let note_tokens: Vec<String> = match declaration::section_tokens(&note_text, SECTION) {
        SectionVerdict::Absent => {
            return Err(format!("note {} has no '{}' section, so there is nothing to hold against the surface (docs/install.md §The upgrade contract owns the note grammar)", note_f, SECTION))
        }
        SectionVerdict::Unparsed(b) => {
            eprintln!("check-tightened-gates-note-parity: note {}'s '{}' section does not parse, so it would compare as a silently empty set:", note_f, SECTION);
            for line in &b {
                eprintln!("  {}", line);
            }
            return Ok(2);
        }
        SectionVerdict::ExplicitNone => Vec::new(),
        SectionVerdict::Tokens(t) => t,
    };

    let decl_tokens: Vec<String> = match declaration::record_tokens(&decl_text) {
        Ok(t) => t,
        Err(b) => {
            eprintln!("check-tightened-gates-note-parity: {} carries malformed data line(s), so the surface would compare as a silently wrong set:", decl_file);
            for line in &b {
                eprintln!("  {}", line);
            }
            return Ok(2);
        }
    };

    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the two-direction comparison
    // is the set difference the contract states, which is not locale-dependent where a `comm` over
    // two sorted streams is
    let surface: BTreeSet<&str> = decl_tokens.iter().map(String::as_str).collect();
    let note: BTreeSet<&str> = note_tokens.iter().map(String::as_str).collect();
    let only_surface: Vec<&&str> = surface.difference(&note).collect();
    let only_note: Vec<&&str> = note.difference(&surface).collect();

    if !only_surface.is_empty() || !only_note.is_empty() {
        println!("check-tightened-gates-note-parity: note {} (v{}, under composition) and {} declare different gate sets:", note_f, note_v.strip_prefix('v').unwrap_or(note_v), decl_file);
        if !only_surface.is_empty() {
            println!("  on the surface, missing from the note — tightened and shipping undeclared, which licenses a red the upgrade smoke would wave through:");
            for t in only_surface {
                println!("    {}", t);
            }
        }
        if !only_note.is_empty() {
            println!("  in the note, missing from the surface — declares a gate that never tightened, sending consumers hunting a reconcile that does not exist:");
            for t in only_note {
                println!("    {}", t);
            }
        }
        println!("  help: the note's Tightened-gates bullets are composed from the surface's data lines — bring the two into agreement before the drain-and-stamp commit.");
        return Ok(1);
    }

    println!(
        "TIGHTENED-GATES-NOTE-PARITY: clean (note {} is under composition and its Tightened-gates set equals {}, both directions; {} token(s))",
        note_f,
        decl_file,
        note_tokens.len()
    );
    Ok(0)
}
