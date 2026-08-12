// spec: canon-kit/SPEC.md §check-measured-claim — a measured count or extent claim that names
// an oracle agrees with it
use crate::spec;
use crate::walk;
use std::path::Path;

const CLOSE: &str = "-->";
const EXEMPT: &str = "measured-claim-exempt:";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-measured-claim: {}", e);
            2
        }
    }
}

struct Marker {
    file: String,
    line: usize,
    key: String,
    value: String,
    claim: String,
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    if spec::knob_pub("CANON_KIT_MEASURED_CLAIMS_CMD")?.is_empty() {
        println!("MEASURED-CLAIM: clean (CANON_KIT_MEASURED_CLAIMS_CMD unset — no oracle, so no marker has anything to disagree with)");
        return Ok(0);
    }
    let globs = spec::knob_array_pub("CANON_KIT_MEASURED_SURFACE_GLOBS")?;
    if globs.is_empty() {
        println!("MEASURED-CLAIM: clean (CANON_KIT_MEASURED_SURFACE_GLOBS empty — no scanned surface)");
        return Ok(0);
    }
    let keys = spec::knob_array_pub("CANON_KIT_MEASURED_KEYS")?;
    let values = spec::knob_array_pub("CANON_KIT_MEASURED_VALUES")?;
    if keys.len() != values.len() {
        return Err(format!(
            "the bridged measured-claim roster is not index-aligned: {} key(s) against {} \
             value(s) — the config bridge could not carry it; treating as failure (not clean)",
            keys.len(),
            values.len()
        ));
    }

    let mut files: Vec<String> = walk::glob_files(Path::new(root), &globs)?
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    files.sort();
    files.dedup();
    if files.is_empty() {
        println!("MEASURED-CLAIM: clean (0 file(s) on the measured surface)");
        return Ok(0);
    }

    let mut markers: Vec<Marker> = Vec::new();
    let mut errs: Vec<String> = Vec::new();
    for f in &files {
        collect(f, &spec::read_text(Path::new(f))?, &mut markers, &mut errs);
    }

    let mut out: Vec<String> = Vec::new();
    for m in &markers {
        // spec: canon-kit/SPEC.md §check-measured-claim — arm B, before either arm that reads
        // a value: an unknown key has no oracle value to compare against, so it fails closed
        // rather than reporting a disagreement it could not have measured
        let idx = match keys.iter().position(|k| *k == m.key) {
            Some(i) => i,
            None => {
                errs.push(format!(
                    "{}:{}  key '{}' is absent from the emitter's roster — a marker naming a key nobody emits is a claim with no oracle wearing the costume of one",
                    m.file, m.line, m.key
                ));
                continue;
            }
        };
        // spec: canon-kit/SPEC.md §check-measured-claim — arm A
        if values[idx] != m.value {
            out.push(format!(
                "{}:{}  key '{}' is marked '{}' but the oracle now reports '{}'",
                m.file, m.line, m.key, m.value, values[idx]
            ));
        }
        // spec: canon-kit/SPEC.md §check-measured-claim — arm C, which applies only to a
        // bare-cardinal value: an extent claim carries no cardinal and is covered by A and B
        let want = match cardinal_value(&m.value) {
            Some(w) => w,
            None => continue,
        };
        let found = distinct_cardinals(&m.claim);
        if found.len() > 1 {
            errs.push(format!(
                "{}:{}  the bound claim carries {} distinct cardinals ({}) — which one the marker holds is ambiguous",
                m.file,
                m.line,
                found.len(),
                found.join(", ")
            ));
        } else if !found.contains(&want) {
            out.push(format!(
                "{}:{}  the marker's cardinal '{}' appears nowhere in the claim it binds — the marker agrees with the tree while the sentence does not",
                m.file, m.line, m.value
            ));
        }
    }

    if !errs.is_empty() {
        return Err(format!(
            "a marker could not be resolved against the oracle; treating as failure (not clean):\n{}\n  help: emit the key from CANON_KIT_MEASURED_CLAIMS_CMD, or drop the marker; for an ambiguous claim, split the sentence or move the marker onto the clause that carries the measurement",
            errs.iter().map(|e| format!("  {}", e)).collect::<Vec<_>>().join("\n")
        ));
    }
    if !out.is_empty() {
        println!("check-measured-claim: a measured claim no longer agrees with the oracle it named — the number in the document has gone stale against the tree:");
        println!();
        for l in &out {
            println!("{}", l);
        }
        println!("  help: re-run the oracle (CANON_KIT_MEASURED_CLAIMS_CMD), then move the marker's <value> and the sentence it binds together — updating one and not the other is the drift this gate exists to catch; a claim that stopped being measurable drops its marker rather than keeping a value nothing checks");
        return Ok(1);
    }
    println!(
        "MEASURED-CLAIM: clean ({} file(s) on the measured surface; {} marker(s), each naming an emitted key and agreeing with it)",
        files.len(),
        markers.len()
    );
    Ok(0)
}

// spec: canon-kit/SPEC.md §check-measured-claim — the marker is a full-line HTML comment on
// the line immediately above its claim; a fenced block is grammar being shown rather than a
// claim being made, and the per-site exempt window is the line or the one above
fn collect(file: &str, text: &str, markers: &mut Vec<Marker>, errs: &mut Vec<String>) {
    let lines: Vec<&str> = text.lines().collect();
    let mut fence = false;
    for (i, raw) in lines.iter().enumerate() {
        if spec::is_fence_line(raw) {
            fence = !fence;
            continue;
        }
        if fence {
            continue;
        }
        let t = raw.trim();
        if !t.starts_with(spec::MEASURED_MARKER) {
            continue;
        }
        if raw.contains(EXEMPT) || (i > 0 && lines[i - 1].contains(EXEMPT)) {
            continue;
        }
        match parse(t) {
            Some((key, value)) => markers.push(Marker {
                file: file.to_string(),
                line: i + 1,
                key,
                value,
                claim: bound_claim(&lines, i),
            }),
            None => errs.push(format!(
                "{}:{}  marker does not parse: {} — the grammar is `<!-- measured: <key>=<value> -->`",
                file,
                i + 1,
                t
            )),
        }
    }
}

fn parse(t: &str) -> Option<(String, String)> {
    let inner = t
        .strip_prefix(spec::MEASURED_MARKER)?
        .strip_suffix(CLOSE)?
        .trim();
    let (k, v) = inner.split_once('=')?;
    let (k, v) = (k.trim(), v.trim());
    if k.is_empty() || v.is_empty() {
        return None;
    }
    Some((k.to_string(), v.to_string()))
}

// spec: canon-kit/SPEC.md §check-measured-claim — the bound claim is the paragraph the marker
// sits above: the run of lines below it up to a blank line, a fence, a second marker or the
// end of the file
fn bound_claim(lines: &[&str], i: usize) -> String {
    let mut out: Vec<&str> = Vec::new();
    for l in lines.iter().skip(i + 1) {
        if spec::is_blank(l) || spec::is_fence_line(l) || l.trim().starts_with(spec::MEASURED_MARKER)
        {
            break;
        }
        out.push(l);
    }
    out.join(" ")
}

// spec: canon-kit/SPEC.md §check-measured-claim — a bare cardinal is a digit run or one of the
// count grammar's cardinal words, normalized to digits so a marker's `12` and a sentence's
// "twelve" are one cardinal rather than two
fn cardinal_value(s: &str) -> Option<String> {
    if !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit()) {
        return Some(s.to_string());
    }
    spec::cardinal_word_value(&s.to_ascii_lowercase())
}

fn distinct_cardinals(text: &str) -> Vec<String> {
    let b: Vec<u8> = text.bytes().map(|c| c.to_ascii_lowercase()).collect();
    let mut out: Vec<String> = Vec::new();
    let mut push = |v: String| {
        if !out.contains(&v) {
            out.push(v);
        }
    };
    let edge = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = 0usize;
    while i < b.len() {
        if b[i].is_ascii_digit() && (i == 0 || !edge(b[i - 1])) {
            let mut j = i;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            if j == b.len() || !edge(b[j]) {
                push(String::from_utf8_lossy(&b[i..j]).into_owned());
            }
            i = j;
            continue;
        }
        if b[i].is_ascii_lowercase() && (i == 0 || !edge(b[i - 1])) {
            let mut j = i;
            while j < b.len() && b[j].is_ascii_lowercase() {
                j += 1;
            }
            if j == b.len() || !edge(b[j]) {
                if let Some(v) = spec::cardinal_word_value(&String::from_utf8_lossy(&b[i..j])) {
                    push(v);
                }
            }
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §check-measured-claim — arm C's grammar, held to the shapes the
    // authoring contract prices: a word and its digits are one cardinal, a token inside a
    // longer word is none, and two distinct ones are what the ambiguity fail-close counts
    #[test]
    fn a_cardinal_word_and_its_digits_are_one_cardinal() {
        assert_eq!(distinct_cardinals("twelve instances"), vec!["12"]);
        assert_eq!(distinct_cardinals("12 instances and twelve more"), vec!["12"]);
    }

    #[test]
    fn a_cardinal_inside_a_longer_token_is_not_one() {
        assert!(distinct_cardinals("v12beta and twelvefold").is_empty());
        assert_eq!(distinct_cardinals("the 17 gates"), vec!["17"]);
    }

    #[test]
    fn two_distinct_cardinals_are_what_the_ambiguity_close_counts() {
        assert_eq!(
            distinct_cardinals("thirteen instances across five iterations"),
            vec!["5"]
        );
        assert_eq!(distinct_cardinals("17 of 102 gates"), vec!["17", "102"]);
    }

    #[test]
    fn the_marker_grammar_takes_a_key_and_a_value_and_nothing_else() {
        assert_eq!(
            parse("<!-- measured: ported-gate-members=17 -->"),
            Some(("ported-gate-members".into(), "17".into()))
        );
        assert_eq!(parse("<!-- measured: no-value= -->"), None);
        assert_eq!(parse("<!-- measured: no-separator -->"), None);
    }

    // spec: canon-kit/SPEC.md §check-measured-claim — an extent value carries no cardinal, so
    // arm C does not apply to it and arms A and B do the whole job
    #[test]
    fn an_extent_value_is_not_a_bare_cardinal() {
        assert_eq!(cardinal_value("17"), Some("17".into()));
        assert_eq!(cardinal_value("twelve"), Some("12".into()));
        assert_eq!(cardinal_value("canon-kit,gate-sdk,queue-kit"), None);
        assert_eq!(cardinal_value("17 gates"), None);
    }
}
