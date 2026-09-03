// spec: drift-kit/SPEC.md §The knowledge-friction loop — the capture affordance: one appended line
// per re-derivation, the grammar stamped by the producer rather than by its author.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and not a hardcoded flag, because the
// arm resolves a consumer knob and a hardcoded flag receives no override at all.

pub const KNOBS: &[&str] = &["DRIFT_KIT_KNOWLEDGE_LOG"];

const USAGE: &str = "usage: --emit kfric [--] \"<fact re-derived>\" \"<surface it was read from>\"\n  appends one dated line to the knowledge-friction log; \"--\" files a field beginning with \"-\"";

// spec: drift-kit/SPEC.md §The knowledge-friction loop — the line grammar is the log's contract, so
// it is stamped here byte for byte and the same string is returned as the arm's confirmation.
fn line(today: &str, fact: &str, surface: &str) -> String {
    format!("{} {} ← {}", today, fact, surface)
}

// spec: drift-kit/SPEC.md §The knowledge-friction loop — the log is created by its first line and
// never seeded: **absent** and **present and empty** are two of the three states the KPI reads
// apart, so a seeding write would report a capture loop that has captured nothing.
fn append_creating(path: &std::path::Path, body: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    f.write_all(body.as_bytes())
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let fields = super::file_survey::positionals(args, "field")?;
    if fields.len() != 2 || fields.iter().any(String::is_empty) {
        return Err(USAGE.to_string());
    }
    let (log, spelled) = super::file_survey::anchored("DRIFT_KIT_KNOWLEDGE_LOG")?;
    let path = std::path::Path::new(&log);
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let line = line(&super::kpi::today_iso(), &fields[0], &fields[1]);
    append_creating(path, &format!("{}\n", line))
        .map_err(|e| format!("cannot append to {}: {}", spelled, e))?;
    Ok(format!("kfric: {}\n", line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::file_survey;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the hazard belongs to the argument rather
    // than to the substrate, and two slots leave arity safe in neither, so the scan covers both.
    #[test]
    fn a_flag_in_either_slot_is_refused_and_a_separator_files_it() {
        for i in 0..2 {
            let mut a = argv(&["a fact", "a surface"]);
            a[i] = "--list".to_string();
            let err = file_survey::positionals(&a, "field")
                .err()
                .unwrap_or_else(|| panic!("a flag in slot {} was captured", i));
            assert!(
                err.contains("--list"),
                "the refusal named no offender: {}",
                err
            );
        }
        let sep = argv(&["--", "--list is captured at exit 0", "a surface"]);
        assert_eq!(
            file_survey::positionals(&sep, "field")
                .expect("the separator did not end option processing"),
            &sep[1..]
        );
    }

    // spec: drift-kit/SPEC.md §The knowledge-friction loop — `-h`/`--help` retires to the front-end,
    // and what replaces it is a refusal rather than a capture: usage text is not a re-derived fact.
    #[test]
    fn a_help_flag_is_a_refusal_rather_than_a_capture() {
        for flag in ["-h", "--help"] {
            assert!(
                file_survey::positionals(&argv(&[flag]), "field").is_err(),
                "{} was taken as a field",
                flag
            );
        }
    }

    // spec: drift-kit/SPEC.md §The knowledge-friction loop — both fields are required non-empty, so
    // arity misuse is a refusal rather than a half-blank line appended to the log.
    #[test]
    fn arity_misuse_is_a_refusal() {
        assert!(emit(&argv(&[])).is_err());
        assert!(emit(&argv(&["a fact"])).is_err());
        assert!(emit(&argv(&["a fact", ""])).is_err());
        assert!(emit(&argv(&["", "a surface"])).is_err());
        assert!(emit(&argv(&["a fact", "a surface", "a third"])).is_err());
    }

    // spec: drift-kit/SPEC.md §The knowledge-friction loop — the appended line's grammar, byte for
    // byte: the arrow separates the fact from the surface it was read from, in that fixed order.
    #[test]
    fn the_line_grammar_is_byte_preserved() {
        assert_eq!(
            line("2026-01-01", "a fact", "a surface"),
            "2026-01-01 a fact ← a surface"
        );
    }
}
