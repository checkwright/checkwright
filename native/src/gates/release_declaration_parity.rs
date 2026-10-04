// spec: installer/SPEC.md §The upgrade contract — a note under composition equals its surface in
// each surface-declared section, and the platforms-table diff in Platforms, both directions
use crate::declaration;
use crate::declaration::{SectionVerdict, TokenRule};
use crate::gates::install_platforms::{declarations, Decl, State};
use crate::gates::release_bump::{front_matter_release, parse_version, read_text, Version};
use crate::release_sections::{self, Role, Section};
use crate::{proc, programs};
use crate::walk;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const NAME: &str = "check-release-declaration-parity";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

// spec: installer/SPEC.md §The upgrade contract — the per-section cost of each direction, which the
// gates pair states for the allowed-red set and the prose sections state for a reconstruction
fn grounds(role: Role) -> (&'static str, &'static str) {
    match role {
        Role::Gates => (
            "tightened and shipping undeclared, which licenses a red the upgrade smoke would wave through",
            "declares a gate that never tightened, sending consumers hunting a reconcile that does not exist",
        ),
        Role::Platforms => (
            "a changed platforms row the note dropped, so an adopter on that platform is never told",
            "a platform the table did not change, announcing support that moved nowhere",
        ),
        _ => (
            "a declared change the note dropped",
            "a change close reconstructed instead of the landing session declaring it — append it to the surface in the composing commit, then transcribe",
        ),
    }
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — the note verdict: an absent section refuses, an
// unparsed one refuses with its lines, and `None` is the empty set
fn note_tokens(text: &str, file: &str, s: &Section, rule: TokenRule) -> Result<Result<Vec<String>, i32>, String> {
    match declaration::section_tokens(text, &s.names(), rule) {
        SectionVerdict::Absent => Err(format!("note {} has no '{}' section, so there is nothing to hold it against (installer/SPEC.md §The upgrade contract owns the note grammar)", file, s.heading)),
        SectionVerdict::Unparsed(b) => {
            eprintln!("{}: note {}'s '{}' section does not parse, so it would compare as a silently wrong set:", NAME, file, s.heading);
            for line in &b {
                eprintln!("  {}", line);
            }
            Ok(Err(2))
        }
        SectionVerdict::ExplicitNone => Ok(Ok(Vec::new())),
        SectionVerdict::Tokens(t) => Ok(Ok(t)),
    }
}

fn status(s: &State) -> String {
    match s {
        State::Joined => "joined".to_string(),
        State::Held(c) => format!("held: {}", c),
        State::Unreadable(m) => format!("unreadable ({})", m),
    }
}

// spec: installer/SPEC.md §The upgrade contract — the previous release is the newest `v*` tag below
// the note's version, ordered as check-release-bump orders notes
fn previous_tag(note_v: &str) -> Result<Option<String>, String> {
    let raw = note_v.strip_prefix('v').unwrap_or(note_v);
    let mine = parse_version(raw, "the note under composition")?;
    let done = proc::run(&programs::GIT, &["tag", "--list", "v*"])?;
    let out = done.stdout().ok_or_else(|| {
        format!(
            "git tag --list failed ({}), so the previous release cannot be resolved; treating as failure (not clean)",
            done.failure_report().unwrap_or_default()
        )
    })?;
    let mut best: Option<(Version, String)> = None;
    for tag in String::from_utf8_lossy(out).lines().map(str::trim).filter(|t| !t.is_empty()) {
        let v = parse_version(tag.strip_prefix('v').unwrap_or(tag), &format!("tag {}", tag))?;
        if v < mine && best.as_ref().map(|(b, _)| v > *b).unwrap_or(true) {
            best = Some((v, tag.to_string()));
        }
    }
    Ok(best.map(|(_, t)| t))
}

// spec: installer/SPEC.md §The upgrade contract — the page at the previous release, a page or a tag
// absent there reading as the empty table
fn page_at(tag: &str, page: &str) -> Result<String, String> {
    let spec = format!("{}:{}", tag, page.trim_start_matches("./"));
    if proc::run(&programs::GIT, &["cat-file", "-e", &spec])?.code() != Some(0) {
        return Ok(String::new());
    }
    let done = proc::run(&programs::GIT, &["cat-file", "blob", &spec])?;
    match done.stdout() {
        Some(b) => Ok(String::from_utf8_lossy(b).into_owned()),
        None => Err(format!("git cat-file blob {} failed, so the previous platforms table is unreadable; treating as failure (not clean)", spec)),
    }
}

// spec: installer/SPEC.md §The upgrade contract — a triple is changed when it was added, removed,
// or its Minimum or Status cell moved; each change is printed as the bullet it derives
fn changed(before: &[Decl], after: &[Decl]) -> BTreeMap<String, String> {
    let index = |d: &[Decl]| -> BTreeMap<String, (String, String)> {
        d.iter().map(|x| (x.triple.clone(), (x.minimum.clone(), status(&x.state)))).collect()
    };
    let (b, a) = (index(before), index(after));
    let mut out = BTreeMap::new();
    for (t, (min, st)) in &a {
        match b.get(t) {
            None => {
                out.insert(t.clone(), format!("added: Minimum {}, Status {}", min, st));
            }
            Some((bmin, bst)) if bmin != min || bst != st => {
                let mut moved = Vec::new();
                if bmin != min {
                    moved.push(format!("Minimum {} → {}", bmin, min));
                }
                if bst != st {
                    moved.push(format!("Status {} → {}", bst, st));
                }
                out.insert(t.clone(), moved.join("; "));
            }
            _ => {}
        }
    }
    for (t, (min, st)) in &b {
        if !a.contains_key(t) {
            out.insert(t.clone(), format!("removed (was Minimum {}, Status {})", min, st));
        }
    }
    out
}

fn rule(args: &[String]) -> Result<i32, String> {
    let posts = crate::fresh::positional_or_knob(args, 0, "GATE_LOCAL_RELEASE_POSTS_DIR")?;
    let posts = posts.as_str();
    let decl_file = match args.get(1).filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => format!("{}/release-declarations.md", walk::knob_scalar("GATE_SDK_WORKFLOW_DIR")?),
    };
    let decl_file = decl_file.as_str();
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
        if proc::run(&programs::GIT, &["rev-parse", "-q", "--verify", &tag])?.code() != Some(0) {
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

    let roster = release_sections::roster()?;
    let note_text = read_text(note_f)?;
    let mut sets: Vec<(&Section, Vec<String>, Vec<String>)> = Vec::new();
    for s in roster.iter().filter(|s| s.role != Role::Platforms) {
        let Some(token_rule) = s.role.token_rule() else {
            continue;
        };
        let note = match note_tokens(&note_text, note_f, s, token_rule)? {
            Ok(t) => t,
            Err(rc) => return Ok(rc),
        };
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — the surface verdict: an absent, `None` or
        // bullet-less section is the empty set, and only an unreadable bullet refuses
        let decl = match declaration::section_tokens(&decl_text, &s.names(), token_rule) {
            SectionVerdict::Tokens(t) => t,
            SectionVerdict::Unparsed(b) if !b.is_empty() => {
                eprintln!("{}: {}'s '{}' section carries unreadable bullet(s), so the surface would compare as a silently wrong set:", NAME, decl_file, s.heading);
                for line in &b {
                    eprintln!("  {}", line);
                }
                return Ok(2);
            }
            _ => Vec::new(),
        };
        sets.push((s, note, decl));
    }

    // spec: installer/SPEC.md §The upgrade contract — arm P: Platforms is derived at composition
    // from the platforms table, never declared on the surface. The third positional is the page in
    // the tree and the fourth stands in for its previous release's copy, so a fixture holds a base.
    let mut derived: Option<(String, BTreeMap<String, String>)> = None;
    if let Some(s) = release_sections::find(&roster, Role::Platforms) {
        let page = crate::fresh::positional_or_knob(args, 2, "GATE_LOCAL_INSTALL_PAGE")?;
        let now = read_text(&page)?;
        let (base_name, base) = match args.get(3).filter(|a| !a.is_empty()) {
            Some(b) => (b.clone(), read_text(b)?),
            None => match previous_tag(note_v)? {
                Some(t) => (t.clone(), page_at(&t, &page)?),
                None => ("no previous release".to_string(), String::new()),
            },
        };
        let diff = changed(&declarations(&base), &declarations(&now));
        let note = match note_tokens(&note_text, note_f, s, TokenRule::Backticked)? {
            Ok(t) => t,
            Err(rc) => return Ok(rc),
        };
        sets.push((s, note, diff.keys().cloned().collect()));
        derived = Some((format!("{} against {}", page, base_name), diff));
    }

    // spec: gate-sdk/SPEC.md §The kit-roots `gate_kit_roots` cohort — the two-direction comparison
    // is the set difference the contract states, which is not locale-dependent where a `comm` over
    // two sorted streams is
    let mut report: Vec<String> = Vec::new();
    let mut total = 0usize;
    let mut platforms_red = false;
    for (s, note_tokens, decl_tokens) in &sets {
        let surface: BTreeSet<&str> = decl_tokens.iter().map(String::as_str).collect();
        let note: BTreeSet<&str> = note_tokens.iter().map(String::as_str).collect();
        total += note.len();
        let only_surface: Vec<&&str> = surface.difference(&note).collect();
        let only_note: Vec<&&str> = note.difference(&surface).collect();
        let (dropped, added) = grounds(s.role);
        let source = if s.role == Role::Platforms { "in the platforms-table diff" } else { "on the surface" };
        if !only_surface.is_empty() {
            report.push(format!("  {}: {}, missing from the note — {}:", s.heading, source, dropped));
            report.extend(only_surface.iter().map(|t| format!("    {}", t)));
        }
        if !only_note.is_empty() {
            let missing = if s.role == Role::Platforms { "missing from the platforms-table diff" } else { "missing from the surface" };
            report.push(format!("  {}: in the note, {} — {}:", s.heading, missing, added));
            report.extend(only_note.iter().map(|t| format!("    {}", t)));
        }
        platforms_red |= s.role == Role::Platforms && !(only_surface.is_empty() && only_note.is_empty());
    }

    if !report.is_empty() {
        println!("{}: note {} (v{}, under composition) and the sets it is composed from disagree:", NAME, note_f, note_v.strip_prefix('v').unwrap_or(note_v));
        for line in report {
            println!("{}", line);
        }
        if let (true, Some((from, diff))) = (platforms_red, &derived) {
            println!("  the platforms-table diff ({}) derives these bullets — transcribe them:", from);
            if diff.is_empty() {
                println!("    None.");
            }
            for (t, what) in diff {
                println!("    - `{}` — {}", t, what);
            }
        }
        println!("  help: the note's declaration-bearing bullets are composed from {}'s bullets, lead tokens unchanged, and its Platforms bullets from the printed diff — bring them into agreement before the drain-and-stamp commit.", decl_file);
        return Ok(1);
    }

    let platforms = match &derived {
        Some((from, _)) => format!("; Platforms equals the platforms-table diff of {}", from),
        None => String::new(),
    };
    println!(
        "RELEASE-DECLARATION-PARITY: clean (note {} is under composition and its declaration-bearing sections equal {}, both directions{}; {} token(s))",
        note_f, decl_file, platforms, total
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(rows: &str) -> String {
        format!("<!-- platforms:begin -->\n| System | Minimum | Binary | Status |\n|---|---|---|---|\n{}<!-- platforms:end -->\n", rows)
    }

    #[test]
    fn a_triple_changes_when_added_removed_or_a_cell_moves() {
        let before = page("| A | v1 | `a-1` | joined |\n| B | v1 | `b-1` | joined |\n| C | v1 | `c-1` | held: x |\n| D | v1 | `d-1` | joined |\n");
        let after = page("| A | v1 | `a-1` | joined |\n| B | v2 | `b-1` | joined |\n| C | v1 | `c-1` | joined |\n| E | v1 | `e-1` | joined |\n");
        let d = changed(&declarations(&before), &declarations(&after));
        assert_eq!(d.keys().map(String::as_str).collect::<Vec<_>>(), vec!["b-1", "c-1", "d-1", "e-1"]);
        assert_eq!(d["b-1"], "Minimum v1 → v2");
        assert_eq!(d["c-1"], "Status held: x → joined");
        assert!(d["d-1"].starts_with("removed"));
        assert!(d["e-1"].starts_with("added"));
        assert!(changed(&declarations(""), &declarations("")).is_empty());
    }
}
