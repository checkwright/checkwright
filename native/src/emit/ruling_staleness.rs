// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the reporting arm over a consumer's
// ruling record: it dispatches each declared oracle, reports every conditioned ruling left
// undeclared, quotes the citing sites, and proposes no edit.
use crate::proc;
use crate::walk;
use std::path::Path;

const WS: [char; 5] = [' ', '\t', '\x0b', '\x0c', '\r'];

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the declared roster the bridge resolves;
// all three default empty or inert kit-side, so an adopter keeping no ruling record reaches a
// configured-off report rather than a broken arm.
pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_RULING_RECORD",
    "LIFECYCLE_KIT_RULING_CITERS",
    "LIFECYCLE_KIT_RULING_ORACLE_TIMEOUT",
];

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the deliberately weak, FP-bearing prose
// match that finds a conditioned ruling nobody declared. It is generic forward-looking English, so
// it is kit mechanism rather than any consumer's vocabulary, and it reports rather than reds.
const FORWARD_PHRASES: &[&str] = &[
    "discharge event",
    "only once",
    "at which point",
    "unblocks on",
    "unblocks when",
    "stays gated",
    "reopens",
    "until that",
    "until the",
    "waits on",
    "once that",
];

pub struct Declared {
    pub name: String,
    pub oracle: String,
    pub line: usize,
}

pub struct Undeclared {
    pub line: usize,
    pub excerpt: String,
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the oracle contract's three bands. The
// third exists so a broken oracle can never read as a fired condition, which is why a dispatch
// failure is its own variant rather than a `NotFired` with a note.
pub enum Band {
    Fired(String),
    NotFired,
    Manual(String),
    DispatchFailure(String),
}

impl Band {
    pub fn label(&self) -> &'static str {
        match self {
            Band::Fired(_) => "fired",
            Band::NotFired => "not-fired",
            Band::Manual(_) => "manual",
            Band::DispatchFailure(_) => "dispatch-failure",
        }
    }

    pub fn evidence(&self) -> &str {
        match self {
            Band::Fired(e) => e,
            Band::NotFired => "-",
            Band::Manual(p) => p,
            Band::DispatchFailure(e) => e,
        }
    }
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the two body-line declarations, read
// with the fence skip and the full-line lead token the sibling roster arm already established, so
// the grammar stays quotable in the surface that specifies it.
fn declarations<'a>(text: &'a str, lead: &str) -> Vec<(usize, &'a str)> {
    let mut out: Vec<(usize, &str)> = Vec::new();
    let mut fence = false;
    for (i, line) in text.lines().enumerate() {
        let t = line.trim_start_matches(WS);
        if t.starts_with("```") {
            fence = !fence;
            continue;
        }
        if fence {
            continue;
        }
        if let Some(rest) = t.strip_prefix(lead) {
            if rest.starts_with(WS) {
                out.push((i + 1, rest.trim_matches(WS)));
            }
        }
    }
    out
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — one or more names, appended and never
// rewritten: a single declared name is insufficient by construction, one measured site having
// survived its own repair by writing the ruling's noun a single word differently.
pub fn ruling_names(text: &str) -> Vec<Vec<String>> {
    declarations(text, "ruling:")
        .into_iter()
        .map(|(_, rest)| split_names(rest))
        .collect()
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — a `ruling:` line carries names, not one
// name per token: a proper noun with spaces in it is the common case, so the separator is a
// two-space run and a single space stays inside a name.
fn split_names(rest: &str) -> Vec<String> {
    rest.split("  ")
        .map(|n| n.trim_matches(WS).to_string())
        .filter(|n| !n.is_empty())
        .collect()
}

pub fn discharges(text: &str) -> Vec<Declared> {
    declarations(text, "discharge:")
        .into_iter()
        .filter_map(|(line, rest)| {
            let (name, oracle) = rest.split_once("  ").or_else(|| rest.split_once(' '))?;
            let name = name.trim_matches(WS).to_string();
            let oracle = oracle.trim_matches(WS).to_string();
            if name.is_empty() || oracle.is_empty() {
                return None;
            }
            Some(Declared { name, oracle, line })
        })
        .collect()
}

fn paragraphs(text: &str) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut start = 0usize;
    let mut buf: Vec<&str> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim_matches(WS).is_empty() {
            if !buf.is_empty() {
                out.push((start + 1, buf.join("\n")));
                buf.clear();
            }
            continue;
        }
        if buf.is_empty() {
            start = i;
        }
        buf.push(line);
    }
    if !buf.is_empty() {
        out.push((start + 1, buf.join("\n")));
    }
    out
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — an undeclared condition is reported,
// never inherited: without this the no-retrofit decision would be a silent hole, since a record
// with one declaration would report one condition and look complete.
pub fn undeclared(text: &str) -> Vec<Undeclared> {
    paragraphs(text)
        .into_iter()
        .filter(|(_, p)| {
            let low = p.to_lowercase();
            FORWARD_PHRASES.iter().any(|ph| low.contains(ph))
                && !p.lines().any(|l| l.trim_start_matches(WS).starts_with("discharge:"))
        })
        .map(|(line, p)| Undeclared {
            line,
            excerpt: excerpt(&p),
        })
        .collect()
}

fn excerpt(p: &str) -> String {
    let one = p
        .lines()
        .map(|l| l.trim_matches(WS))
        .collect::<Vec<_>>()
        .join(" ");
    let mut out = String::new();
    for c in one.chars() {
        if out.chars().count() >= 110 {
            out.push('…');
            break;
        }
        out.push(c);
    }
    out
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the kit interprets nothing: it
// dispatches the declared command and reports which band it landed in. The shell is what makes the
// declaration a command line rather than a program-plus-argv the record would have to quote.
fn dispatch(oracle: &str, timeout: u64) -> Band {
    if let Some(prose) = manual_prose(oracle) {
        return Band::Manual(prose);
    }
    match proc::run_bounded_capture("bash", &["-c", oracle], timeout) {
        Err(e) => Band::DispatchFailure(e),
        Ok(None) => Band::DispatchFailure(format!("exceeded the {}s bound", timeout)),
        Ok(Some((code, bytes))) => {
            let text = String::from_utf8_lossy(&bytes);
            let first = text
                .lines()
                .map(|l| l.trim_matches(WS))
                .find(|l| !l.is_empty())
                .unwrap_or("");
            if code == 0 && !first.is_empty() {
                Band::Fired(first.to_string())
            } else {
                Band::NotFired
            }
        }
    }
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the literal `manual` operand, which is
// the case that forced it rather than an escape hatch: a condition resolving only against a surface
// no tracked oracle reaches reports as owed to judgment instead of being silently skipped.
fn manual_prose(oracle: &str) -> Option<String> {
    if oracle == "manual" {
        return Some("-".to_string());
    }
    oracle
        .strip_prefix("manual")
        .filter(|r| r.starts_with(WS))
        .map(|r| r.trim_matches(WS).to_string())
}

fn cite_corpus(base: &str, record: &str) -> Result<Vec<String>, String> {
    let globs = walk::knob_array("LIFECYCLE_KIT_RULING_CITERS")?;
    let mut out: Vec<String> = Vec::new();
    for p in walk::glob_files(Path::new(base), &globs)? {
        let s = p.display().to_string();
        let rel = s
            .strip_prefix(&format!("{}/", base))
            .unwrap_or_else(|| s.trim_start_matches("./"))
            .to_string();
        if rel != record && !out.contains(&rel) {
            out.push(rel);
        }
    }
    out.sort();
    Ok(out)
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the citing line is quoted verbatim so a
// reader judges the claim rather than the match: past evidence and a live restatement have no
// syntactic tell, so the arm never says a citation is wrong.
fn citing_rows(base: &str, corpus: &[String], name: &str) -> Vec<String> {
    let needle = name.to_lowercase();
    let mut rows: Vec<String> = Vec::new();
    for rel in corpus {
        let text = match std::fs::read(format!("{}/{}", base, rel)) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(_) => continue,
        };
        for (i, line) in text.lines().enumerate() {
            if line.to_lowercase().contains(&needle) {
                rows.push(format!(
                    "{}\t{}:{}\t{}",
                    name,
                    rel,
                    i + 1,
                    line.trim_matches(WS)
                ));
            }
        }
    }
    rows
}

fn timeout_secs() -> Result<u64, String> {
    let raw = walk::knob_scalar("LIFECYCLE_KIT_RULING_ORACLE_TIMEOUT")?;
    raw.trim_matches(WS).parse::<u64>().map_err(|_| {
        format!(
            "LIFECYCLE_KIT_RULING_ORACLE_TIMEOUT is '{}' — a positive integer of seconds is the \
             only value that bounds a dispatch",
            raw
        )
    })
}

// spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the argv tail names rulings a session is
// about to retire, so the citing report answers "what would go stale" for a retirement the record
// does not yet carry. With no operand the report covers exactly the fired conditions.
pub fn emit(args: &[String]) -> Result<String, String> {
    let record = walk::knob_scalar("LIFECYCLE_KIT_RULING_RECORD")?;
    let record = record.trim_matches(WS).to_string();
    if record.is_empty() {
        return Ok("ruling-staleness: no ruling record configured \
                   (LIFECYCLE_KIT_RULING_RECORD is empty) — nothing to report\n"
            .to_string());
    }
    let base = match walk::toplevel_opt()? {
        Some(t) => t,
        None => walk::cwd()?,
    };
    let text = crate::emit::read_text(&walk::abs_against(&base, &record))?;
    let timeout = timeout_secs()?;

    let mut out = String::new();
    out.push_str(&format!("== discharge report ({}) ==\n", record));
    let declared = discharges(&text);
    let mut fired: Vec<String> = Vec::new();
    if declared.is_empty() {
        out.push_str("(no discharge: declaration on the record)\n");
    }
    for d in &declared {
        let band = dispatch(&d.oracle, timeout);
        if matches!(band, Band::Fired(_)) {
            fired.push(d.name.clone());
        }
        out.push_str(&format!(
            "{}\t{}\t{}:{}\t{}\n",
            d.name,
            band.label(),
            record,
            d.line,
            band.evidence()
        ));
    }

    out.push_str("\n== undeclared conditions ==\n");
    let holes = undeclared(&text);
    if holes.is_empty() {
        out.push_str("(none)\n");
    }
    for h in &holes {
        out.push_str(&format!("{}:{}\t{}\n", record, h.line, h.excerpt));
    }

    out.push_str("\n== citing sites ==\n");
    let mut subjects: Vec<String> = fired;
    for a in args {
        let a = a.trim_matches(WS).to_string();
        if !a.is_empty() && !subjects.contains(&a) {
            subjects.push(a);
        }
    }
    if subjects.is_empty() {
        out.push_str("(no fired condition and no named ruling — nothing to sweep)\n");
        return Ok(out);
    }
    let corpus = cite_corpus(&base, &record)?;
    let declared_names = ruling_names(&text);
    let mut rows: Vec<String> = Vec::new();
    for s in &subjects {
        let names = declared_names
            .iter()
            .find(|set| set.iter().any(|n| n.eq_ignore_ascii_case(s)))
            .cloned()
            .unwrap_or_else(|| vec![s.clone()]);
        for n in names {
            rows.extend(citing_rows(&base, &corpus, &n));
        }
    }
    if rows.is_empty() {
        out.push_str("(no citing site)\n");
    }
    for r in rows {
        out.push_str(&r);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the declaration grammar: a fenced
    // example is quotation, and a `ruling:` line carries names rather than tokens, so a proper noun
    // with a space in it survives the split that separates two names.
    #[test]
    fn names_split_on_a_two_space_run_and_a_fenced_declaration_is_quotation() {
        let text = "ruling: the seam sweep  seam-sweep-remainder\n\
                    ```\nruling: fenced\n```\n\
                    ruling:nospace\n";
        assert_eq!(
            ruling_names(text),
            vec![vec![
                "the seam sweep".to_string(),
                "seam-sweep-remainder".to_string()
            ]]
        );
    }

    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the oracle runs to end of line, so
    // the split takes the name and leaves the whole command; a declaration with no command at all
    // is not a condition and is dropped rather than dispatched as an empty one.
    #[test]
    fn a_discharge_takes_the_first_field_as_the_name_and_the_rest_as_the_command() {
        let d = discharges("discharge: a-ruling  git log --oneline -1 | grep x\n");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name, "a-ruling");
        assert_eq!(d[0].oracle, "git log --oneline -1 | grep x");
        assert!(discharges("discharge: lonely\n").is_empty());
    }

    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the `manual` operand reports as
    // owed to judgment and reaches no dispatcher, which is the difference between a probe that
    // knows what it cannot answer and one that answers wrongly.
    #[test]
    fn manual_is_recognised_bare_and_with_its_prose_and_never_as_a_command() {
        assert_eq!(manual_prose("manual"), Some("-".to_string()));
        assert_eq!(
            manual_prose("manual the local brief records it"),
            Some("the local brief records it".to_string())
        );
        assert_eq!(manual_prose("manual-ish thing"), None);
        assert!(matches!(dispatch("manual x", 1), Band::Manual(_)));
    }

    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the three bands, and the one that
    // matters: a clean run printing nothing is NOT fired, so an oracle whose grep found nothing
    // cannot read as a discharged ruling.
    #[test]
    fn only_a_clean_run_that_prints_is_a_fired_condition() {
        assert!(matches!(dispatch("echo hit", 5), Band::Fired(_)));
        assert!(matches!(dispatch("true", 5), Band::NotFired));
        assert!(matches!(dispatch("echo out; false", 5), Band::NotFired));
        assert!(matches!(dispatch("exit 3", 5), Band::NotFired));
    }

    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — a dispatch failure is joined to
    // neither other band, so a hung or broken oracle can never be read as a verdict.
    #[test]
    fn an_oracle_that_outruns_its_bound_is_a_dispatch_failure_and_not_a_band() {
        assert!(matches!(
            dispatch("sleep 5", 1),
            Band::DispatchFailure(_)
        ));
    }

    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — an undeclared condition is reported
    // rather than inherited, and the detection is deliberately weak: a paragraph carrying a
    // `discharge:` line is declared and drops out, whatever its prose says.
    #[test]
    fn a_conditioned_paragraph_reports_undeclared_until_it_carries_a_declaration() {
        let text = "A ruling. Discharge event: that unit lands.\n\n\
                    Another ruling, unconditioned.\n\n\
                    A third. Discharge event: the gate is green.\ndischarge: third  true\n";
        let holes = undeclared(text);
        assert_eq!(holes.len(), 1);
        assert_eq!(holes[0].line, 1);
    }

    // spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — escalation-only, proved rather than
    // asserted: a site citing a ruling as past evidence and a site restating it as a live rule
    // produce the SAME row, because the arm does not distinguish them and must not claim to.
    #[test]
    fn past_evidence_and_a_live_restatement_produce_the_same_row() {
        let dir = std::env::temp_dir().join(format!("cw-ruling-cite.{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("past.md"),
            "the port-only run was the rule until it was retired\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("live.md"),
            "the port-only run is the rule and binds this cut\n",
        )
        .unwrap();
        let base = dir.display().to_string();
        let corpus = vec!["past.md".to_string(), "live.md".to_string()];
        let rows = citing_rows(&base, &corpus, "port-only run");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|r| r.starts_with("port-only run\t")));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
