// spec: lifecycle-kit/SPEC.md §check-disposed-findings — every non-blank line below the record's
// contract header is a record: a calendar date, at least one backticked term with no empty span,
// and non-empty prose
use crate::walk;
use std::path::Path;

const SEP: &str = " — ";

pub struct Record<'a> {
    pub date: &'a str,
    pub terms: Vec<&'a str>,
    pub prose: &'a str,
}

// spec: lifecycle-kit/SPEC.md §The disposed-findings record — `- <YYYY-MM-DD> — <terms> — <prose>`,
// `<terms>` one or more backticked spans joined by `, `; the error names what the line lacks
pub fn record(line: &str) -> Result<Record<'_>, &'static str> {
    let rest = line
        .strip_prefix("- ")
        .ok_or("not a record — a record opens with '- '")?;
    let (date, rest) = rest
        .split_once(SEP)
        .ok_or("no ' — ' after the date")?;
    if !crate::queue::is_calendar_date(date) {
        return Err("the date is not a calendar YYYY-MM-DD");
    }
    let mut terms: Vec<&str> = Vec::new();
    let mut rest = rest;
    loop {
        let Some(open) = rest.strip_prefix('`') else {
            return Err(if terms.is_empty() {
                "no backticked term"
            } else {
                "the terms are not backticked spans joined by ', '"
            });
        };
        let close = open.find('`').ok_or("an unclosed backticked term")?;
        if close == 0 {
            return Err("an empty backticked term");
        }
        terms.push(&open[..close]);
        rest = &open[close + 1..];
        if let Some(next) = rest.strip_prefix(", ") {
            rest = next;
            continue;
        }
        break;
    }
    let prose = rest
        .strip_prefix(SEP)
        .or_else(|| rest.strip_prefix(" —").filter(|p| p.trim().is_empty()))
        .ok_or("no ' — ' between the terms and the prose")?;
    if prose.trim().is_empty() {
        return Err("no prose — the finding and its ground");
    }
    Ok(Record { date, terms, prose })
}

// spec: lifecycle-kit/SPEC.md §The disposed-findings record — the record lines: every non-blank line
// after a first-line `# ` header, numbered from one
pub fn lines(text: &str) -> impl Iterator<Item = (usize, &str)> {
    text.lines()
        .enumerate()
        .filter(|(i, l)| !(*i == 0 && l.starts_with("# ")) && !l.trim().is_empty())
        .map(|(i, l)| (i + 1, l))
}

// spec: lifecycle-kit/SPEC.md §check-disposed-findings — the verdict over one record; `None` is the
// absent record at the configured path, clean because a consumer that never discarded is legal
fn verdict(path: &str, text: Option<&str>) -> (i32, Vec<String>) {
    let Some(text) = text else {
        return (
            0,
            vec![format!(
                "DISPOSED-FINDINGS: clean (0 record(s); no record at {} — no finding discarded yet)",
                path
            )],
        );
    };
    let mut findings: Vec<String> = Vec::new();
    let mut n = 0usize;
    for (fnr, line) in lines(text) {
        n += 1;
        if let Err(lacks) = record(line) {
            findings.push(format!("  {}:{}: {}", path, fnr, lacks));
        }
    }
    if findings.is_empty() {
        return (0, vec![format!("DISPOSED-FINDINGS: clean ({} record(s) in {})", n, path)]);
    }
    let mut out = vec![format!(
        "check-disposed-findings: {} malformed record(s) in {}:",
        findings.len(),
        path
    )];
    out.extend(findings);
    out.push(
        "  help: every line below the '# contract:' header is one record, '- <YYYY-MM-DD> — `<term>`[, `<term>`]… — <finding and ground>': a calendar date, one or more non-empty backticked terms joined by ', ', and the finding with the ground it was discarded on (lifecycle-kit/SPEC.md §The disposed-findings record). The capture arm skips a malformed record without a word, so until the line is fixed a re-filing of its finding raises no advisory."
            .to_string(),
    );
    (1, out)
}

pub fn run(args: &[String]) -> i32 {
    let hermetic = args.first().filter(|a| !a.is_empty());
    let path = match hermetic {
        Some(p) => {
            if !Path::new(p.as_str()).is_file() {
                eprintln!("check-disposed-findings: record file not found: {}", p);
                return 2;
            }
            p.clone()
        }
        None => match walk::knob_scalar("LIFECYCLE_KIT_DISPOSED_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-disposed-findings: {}", e);
                return 2;
            }
        },
    };
    let text = if Path::new(&path).is_file() {
        match std::fs::read(&path) {
            Ok(b) => Some(String::from_utf8_lossy(&b).into_owned()),
            Err(_) => {
                eprintln!("check-disposed-findings: record file not readable: {}", path);
                return 2;
            }
        }
    } else if Path::new(&path).exists() {
        eprintln!("check-disposed-findings: record file not readable: {}", path);
        return 2;
    } else {
        None
    };
    let (rc, out) = verdict(&path, text.as_deref());
    for l in out {
        println!("{}", l);
    }
    rc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_parses_into_its_date_terms_and_prose() {
        let r = record("- 2026-09-27 — `ubuntu-latest`, `runner image` — the finding; its ground.")
            .expect("a well-formed record was refused");
        assert_eq!(r.date, "2026-09-27");
        assert_eq!(r.terms, vec!["ubuntu-latest", "runner image"]);
        assert_eq!(r.prose, "the finding; its ground.");
    }

    #[test]
    fn each_missing_field_is_named() {
        for (line, lacks) in [
            ("- 2026-02-30 — `x` — prose", "calendar"),
            ("- 2026-01-01 — no term here — prose", "no backticked term"),
            ("- 2026-01-01 — `` — prose", "empty backticked term"),
            ("- 2026-01-01 — `x`, `` — prose", "empty backticked term"),
            ("- 2026-01-01 — `x` —", "no prose"),
            ("- 2026-01-01 — `x` — ", "no prose"),
            ("- 2026-01-01 — `x`", "between the terms and the prose"),
            ("- 2026-01-01 — `x` `y` — prose", "between the terms and the prose"),
            ("* 2026-01-01 — `x` — prose", "opens with"),
        ] {
            let err = record(line).err().unwrap_or_else(|| panic!("accepted: {}", line));
            assert!(err.contains(lacks), "{} -> {}", line, err);
        }
    }

    // spec: lifecycle-kit/SPEC.md §check-disposed-findings — the absent-record clean line only the
    // bare form reaches, since a named missing file is exit 2
    #[test]
    fn an_absent_record_is_clean() {
        let (rc, out) = verdict(".workflow/disposed-findings.txt", None);
        assert_eq!(rc, 0);
        assert!(out[0].starts_with("DISPOSED-FINDINGS: clean (0 record(s)"), "{}", out[0]);
    }

    #[test]
    fn the_header_and_blank_lines_are_not_records() {
        let text = "# contract: x\n\n- 2026-01-01 — `a` — p\n\n";
        let (rc, out) = verdict("r", Some(text));
        assert_eq!(rc, 0);
        assert_eq!(out[0], "DISPOSED-FINDINGS: clean (1 record(s) in r)");
    }
}
