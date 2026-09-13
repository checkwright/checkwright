// spec: docs/install.md §The upgrade contract — while a release note is under composition each of
// its three declaration-bearing sections' token sets equals the release declaration surface it was
// composed from, both directions
use crate::declaration;
use crate::declaration::{SectionVerdict, TokenRule};
use crate::gates::release_bump::{front_matter_release, read_text};
use crate::proc;
use crate::walk;
use std::collections::BTreeSet;
use std::path::Path;

const NAME: &str = "check-release-declaration-parity";
const DEFAULT_POSTS: &str = "docs/posts";
const DEFAULT_DECL: &str = ".workflow/release-declarations.md";
const SECTIONS: [(&str, TokenRule); 3] = [
    ("Tightened gates", TokenRule::GateName),
    ("Renamed knobs", TokenRule::Backticked),
    ("Behavior changes", TokenRule::Bolded),
];

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: docs/install.md §The upgrade contract — the per-section cost of each direction, which the
// Tightened-gates pair states for the allowed-red set and the prose sections state for a reconstruction
fn grounds(section: &str) -> (&'static str, &'static str) {
    if section == SECTIONS[0].0 {
        (
            "tightened and shipping undeclared, which licenses a red the upgrade smoke would wave through",
            "declares a gate that never tightened, sending consumers hunting a reconcile that does not exist",
        )
    } else {
        (
            "a declared change the note dropped",
            "a change close reconstructed instead of the landing session declaring it — append it to the surface in the composing commit, then transcribe",
        )
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
        eprintln!("{}: {} carries more than one untagged release note, a state the release choreography does not admit:", NAME, posts);
        for (v, f) in &untagged {
            eprintln!("  {}\t{}", v, f);
        }
        eprintln!("  help: exactly one note is in flight at a time — tag the released one or remove the stray note.");
        return Ok(2);
    }
    if untagged.is_empty() {
        println!("RELEASE-DECLARATION-PARITY: dormant (every release note under {} is tagged, so the surface has been drained by contract and there is nothing to compare)", posts);
        return Ok(0);
    }
    let (note_v, note_f) = &untagged[0];

    // spec: gate-sdk/SPEC.md §upgrade-smoke — the surface's required header line, checked before any
    // section is read, so a missing surface refuses here rather than reading as the empty set; a
    // `## ` section heading on the first line is a surface that lost its header, not one carrying it
    let decl_text = std::fs::read(decl_file)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    if !decl_text.lines().next().unwrap_or("").starts_with("# ") {
        return Err(format!("{} is missing its required header line, so the declaration surface cannot be established (gate-sdk/SPEC.md §upgrade-smoke owns its contract)", decl_file));
    }

    let note_text = read_text(note_f)?;
    let mut sets: Vec<(&str, Vec<String>, Vec<String>)> = Vec::new();
    for (section, token_rule) in SECTIONS {
        let note_tokens = match declaration::section_tokens(&note_text, section, token_rule) {
            SectionVerdict::Absent => {
                return Err(format!("note {} has no '{}' section, so there is nothing to hold against the surface (docs/install.md §The upgrade contract owns the note grammar)", note_f, section))
            }
            SectionVerdict::Unparsed(b) => {
                eprintln!("{}: note {}'s '{}' section does not parse, so it would compare as a silently wrong set:", NAME, note_f, section);
                for line in &b {
                    eprintln!("  {}", line);
                }
                return Ok(2);
            }
            SectionVerdict::ExplicitNone => Vec::new(),
            SectionVerdict::Tokens(t) => t,
        };
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — the surface verdict: an absent, `None` or
        // bullet-less section is the empty set, and only an unreadable bullet refuses
        let decl_tokens = match declaration::section_tokens(&decl_text, section, token_rule) {
            SectionVerdict::Tokens(t) => t,
            SectionVerdict::Unparsed(b) if !b.is_empty() => {
                eprintln!("{}: {}'s '{}' section carries unreadable bullet(s), so the surface would compare as a silently wrong set:", NAME, decl_file, section);
                for line in &b {
                    eprintln!("  {}", line);
                }
                return Ok(2);
            }
            _ => Vec::new(),
        };
        sets.push((section, note_tokens, decl_tokens));
    }

    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the two-direction comparison
    // is the set difference the contract states, which is not locale-dependent where a `comm` over
    // two sorted streams is
    let mut report: Vec<String> = Vec::new();
    let mut total = 0usize;
    for (section, note_tokens, decl_tokens) in &sets {
        let surface: BTreeSet<&str> = decl_tokens.iter().map(String::as_str).collect();
        let note: BTreeSet<&str> = note_tokens.iter().map(String::as_str).collect();
        total += note.len();
        let only_surface: Vec<&&str> = surface.difference(&note).collect();
        let only_note: Vec<&&str> = note.difference(&surface).collect();
        let (dropped, added) = grounds(section);
        if !only_surface.is_empty() {
            report.push(format!("  {}: on the surface, missing from the note — {}:", section, dropped));
            report.extend(only_surface.iter().map(|t| format!("    {}", t)));
        }
        if !only_note.is_empty() {
            report.push(format!("  {}: in the note, missing from the surface — {}:", section, added));
            report.extend(only_note.iter().map(|t| format!("    {}", t)));
        }
    }

    if !report.is_empty() {
        println!("{}: note {} (v{}, under composition) and {} declare different sets:", NAME, note_f, note_v.strip_prefix('v').unwrap_or(note_v), decl_file);
        for line in report {
            println!("{}", line);
        }
        println!("  help: the note's declaration-bearing bullets are composed from the surface's bullets, lead tokens unchanged — bring the two into agreement before the drain-and-stamp commit.");
        return Ok(1);
    }

    println!(
        "RELEASE-DECLARATION-PARITY: clean (note {} is under composition and its three declaration-bearing sections equal {}, both directions; {} token(s))",
        note_f, decl_file, total
    );
    Ok(0)
}
