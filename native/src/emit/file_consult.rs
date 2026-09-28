// spec: lifecycle-kit/SPEC.md §The consult inbox — the capture affordance: one dated bullet per
// item owed to the consult skill, the grammar stamped by the producer rather than by its filer.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and not a hardcoded flag, because the
// arm reads a consumer knob, which a hardcoded flag would hide from the knob-file derivation.

pub const KNOBS: &[&str] = &["LIFECYCLE_KIT_CONSULT_INBOX_FILE"];

pub const USAGE: &str = "usage: --emit file-consult [--] \"<item prose>\"\n  appends one dated bullet to the committed consult inbox; \"--\" files prose beginning with \"-\"";

// spec: lifecycle-kit/SPEC.md §The consult inbox — the contract header seeded on a fresh consumer's
// first filing; the drain removes bullets and keeps it.
const CONTRACT_HEADER: &str = "# contract: lifecycle-kit/SPEC.md §The consult inbox — append-only capture of items owed to the consult skill, consult-drained; one bullet per item below.\n";

fn bullet(today: &str, prose: &str) -> String {
    format!("- {} — {}", today, prose)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let fields =
        super::file_survey::positionals(args, "prose").map_err(|e| format!("{}\n{}", e, USAGE))?;
    if fields.len() != 1 || fields[0].is_empty() {
        return Err(USAGE.to_string());
    }
    let prose = &fields[0];
    super::file_survey::refuse_in_linked_worktree("LIFECYCLE_KIT_CONSULT_INBOX_FILE")?;

    let (inbox, spelled) = super::file_survey::anchored("LIFECYCLE_KIT_CONSULT_INBOX_FILE")?;
    let path = std::path::Path::new(&inbox);
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if !path.is_file() {
        std::fs::write(path, CONTRACT_HEADER)
            .map_err(|e| format!("cannot seed {}: {}", spelled, e))?;
    }

    let line = bullet(&super::kpi::today_iso(), prose);
    super::file_survey::append(path, &format!("{}\n", line))
        .map_err(|e| format!("cannot append to {}: {}", spelled, e))?;
    Ok(format!("file-consult: {}\n", line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::file_survey;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: lifecycle-kit/SPEC.md §The consult inbox — the gap inbox's one bullet shape, no slot
    // between the date and the prose
    #[test]
    fn every_filing_takes_the_one_plain_bullet_shape() {
        assert_eq!(bullet("2026-01-01", "an item"), "- 2026-01-01 — an item");
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — a leading dash is refused, `--` files it,
    // and a help flag is never taken as prose
    #[test]
    fn a_flag_is_refused_a_separator_files_it_and_help_is_not_a_capture() {
        let err = file_survey::positionals(&argv(&["--list"]), "prose")
            .expect_err("a leading-dash prose was captured");
        assert!(err.contains("--list"), "the refusal named no offender: {}", err);
        let sep = argv(&["--", "--list is captured at exit 0"]);
        assert_eq!(
            file_survey::positionals(&sep, "prose").expect("the separator did not end option processing"),
            &sep[1..]
        );
        for flag in ["-h", "--help"] {
            assert!(file_survey::positionals(&argv(&[flag]), "prose").is_err(), "{} was taken as prose", flag);
            assert!(emit(&argv(&[flag])).is_err(), "{} was filed", flag);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The consult inbox — one positional, required non-empty
    #[test]
    fn arity_misuse_is_a_refusal() {
        assert!(emit(&argv(&[])).is_err());
        assert!(emit(&argv(&[""])).is_err());
        assert!(emit(&argv(&["one", "two"])).is_err());
    }
}
