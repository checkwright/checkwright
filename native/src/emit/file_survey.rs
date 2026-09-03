// spec: lifecycle-kit/SPEC.md §The survey record — the capture affordance: one appended block per
// survey, the grammar stamped by the producer rather than by its author.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and not a hardcoded flag, because the
// arm resolves two consumer knobs and a hardcoded flag receives no override at all.
use crate::stages;
use crate::walk;

pub const KNOBS: &[&str] = &["LIFECYCLE_KIT_SURVEY_RECORD_FILE", "LIFECYCLE_KIT_STATE_FILE"];

const USAGE: &str = "usage: --emit file-survey [--] \"<question>\" \"<corpus>\" \"<oracle>\" \"<edges>\" \"<finding>\"\n  appends one dated block to the survey record; \"--\" files a field beginning with \"-\"";

// spec: lifecycle-kit/SPEC.md §The survey record — the never-named stage the queue header already
// uses, stamped when the cursor is absent.
const NO_CURSOR: &str = "—";

// spec: lifecycle-kit/SPEC.md §The survey record — the contract header seeded on a fresh
// consumer's first filing, byte-identical to the line the boundary truncation reduces back to.
const CONTRACT_HEADER: &str = "# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.\n";

// spec: gate-sdk/SPEC.md §The bin/-tool contract — the shape half of that contract outlives the
// member's port: free-text positionals validate shape, not only arity, and the scan covers every
// positional because five slots make arity no protection at all.
pub fn positionals<'a>(args: &'a [String], what: &str) -> Result<&'a [String], String> {
    if args.first().map(String::as_str) == Some("--") {
        return Ok(&args[1..]);
    }
    match args.iter().find(|a| a.starts_with('-')) {
        Some(bad) => Err(format!(
            "unrecognized option: {} — a {} beginning with \"-\" is passed after a \"--\" separator",
            bad, what
        )),
        None => Ok(args),
    }
}

// spec: lifecycle-kit/SPEC.md §The survey record — the repo-root anchor moves with the tool: a
// relative record path names the same file from any subdirectory, falling back to the working
// directory outside a repository.
// comment-tier-exempt: the configured spelling rides back beside the resolved path because the
// refusals print it, and an absolute path no knob carries is not what a reader can act on
pub fn anchored(knob: &str) -> Result<(String, String), String> {
    let root = match walk::toplevel_opt()? {
        Some(t) => t,
        None => walk::cwd()?,
    };
    let configured = walk::knob_scalar(knob)?;
    Ok((walk::abs_against(&root, &configured), configured))
}

// spec: lifecycle-kit/SPEC.md §The survey record — `rev` is machine-stamped because it is the field
// the whole re-use protocol turns on and the one an author gets wrong; a tree with no HEAD cannot
// ground a witness, so it is a refusal rather than a blank field.
fn head_rev() -> Result<String, String> {
    let refusal = || {
        "no HEAD commit to stamp as the survey rev — the witness would have nothing to diff \
         against; commit first."
            .to_string()
    };
    let out = crate::proc::run("git", &["rev-parse", "HEAD"]).map_err(|_| refusal())?;
    let rev = match out.stdout() {
        Some(o) => String::from_utf8_lossy(o).trim().to_string(),
        None => return Err(refusal()),
    };
    if rev.len() != 40 || !rev.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(refusal());
    }
    Ok(rev)
}

// spec: lifecycle-kit/SPEC.md §The survey record — the stage is derived from the cursor rather than
// asked for, which is why the state-file knob is on this arm's roster and on no other's.
fn stage() -> Result<String, String> {
    let (state, _) = anchored("LIFECYCLE_KIT_STATE_FILE")?;
    let s = std::fs::read(state)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    let cursor = stages::current_stage(&s);
    Ok(if cursor.is_empty() {
        NO_CURSOR.to_string()
    } else {
        cursor
    })
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let fields = positionals(args, "field")?;
    if fields.len() != 5 || fields.iter().any(String::is_empty) {
        return Err(USAGE.to_string());
    }
    let (question, corpus, oracle, edges, finding) = (
        &fields[0], &fields[1], &fields[2], &fields[3], &fields[4],
    );
    let rev = head_rev()?;
    let stage = stage()?;
    let today = super::kpi::today_iso();
    let (record, spelled) = anchored("LIFECYCLE_KIT_SURVEY_RECORD_FILE")?;
    let path = std::path::Path::new(&record);
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if !path.is_file() {
        std::fs::write(path, CONTRACT_HEADER)
            .map_err(|e| format!("cannot seed {}: {}", spelled, e))?;
    }
    let block = format!(
        "\n## {} {} — {}\n- corpus: {}\n- oracle: {}\n- rev: {}\n- edges: {}\n- finding: {}\n",
        today, stage, question, corpus, oracle, rev, edges, finding
    );
    append(path, &block).map_err(|e| format!("cannot append to {}: {}", spelled, e))?;

    // spec: lifecycle-kit/SPEC.md §The survey record — the honest limit and the witness hint, the
    // two advisories written straight to stderr because the arm's returned string is the record's
    // own confirmation and a reader pasting it must not receive either of these inside it.
    if oracle == "none" {
        eprintln!(
            "file-survey: oracle \"none\" — this block is a note, not a re-usable survey: a later \
             stage may read it for orientation and must re-derive before relying on it."
        );
    } else {
        eprintln!(
            "file-survey: the witness a later stage runs — git diff --quiet {}..HEAD -- {}, then \
             re-run: {}",
            rev, corpus, oracle
        );
    }
    Ok(format!(
        "file-survey: ## {} {} — {} (rev {})\n",
        today, stage, question, rev
    ))
}

// spec: lifecycle-kit/SPEC.md §The survey record — append-only within the iteration, never edited
// in place, so a filing cannot lose a block a concurrent one landed. Shared with `--emit-file-gap`,
// whose surface carries the same rule as a merge property (§The committed gap inbox).
pub(super) fn append(path: &std::path::Path, body: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().append(true).open(path)?;
    f.write_all(body.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the hazard belongs to the argument rather
    // than to the substrate: a flag captured into a committed surface at exit 0 is the attested
    // failure, and five slots leave arity no protection, so the scan covers every positional.
    #[test]
    fn a_flag_in_any_slot_is_refused_and_a_separator_files_it() {
        for i in 0..5 {
            let mut a = argv(&["q", "c", "o", "e", "f"]);
            a[i] = "--finding".to_string();
            let err = positionals(&a, "field")
                .err()
                .unwrap_or_else(|| panic!("a flag in slot {} was captured", i));
            assert!(err.contains("--finding"), "the refusal named no offender: {}", err);
        }
        let sep = argv(&["--", "-q", "c", "o", "e", "f"]);
        assert_eq!(
            positionals(&sep, "field").expect("the separator did not end option processing"),
            &sep[1..]
        );
    }

    // spec: lifecycle-kit/SPEC.md §The survey record — the edges slot takes no default, so an
    // omitted fifth argument is arity misuse the arm refuses rather than a silently blank field
    #[test]
    fn four_fields_or_an_empty_one_is_arity_misuse() {
        assert!(emit(&argv(&["q", "c", "o", "e"])).is_err());
        assert!(emit(&argv(&["q", "c", "o", "", "f"])).is_err());
        assert!(emit(&argv(&[])).is_err());
    }

    // spec: lifecycle-kit/SPEC.md §The survey record — `-h`/`--help` retires to the front-end, and
    // what replaces it is a refusal rather than a capture: help text is not a survey
    #[test]
    fn a_help_flag_is_a_refusal_rather_than_a_capture() {
        for flag in ["-h", "--help"] {
            assert!(
                positionals(&argv(&[flag]), "field").is_err(),
                "{} was taken as a field",
                flag
            );
        }
    }
}
