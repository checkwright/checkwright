// spec: lifecycle-kit/SPEC.md §check-survey-record — every survey block carries its whole
// witness: the five keys in order, a full-sha rev naming a real commit, a non-empty corpus
// and a non-empty oracle and edges
use crate::proc;
use crate::walk;
use std::path::Path;

const WANT: [&str; 5] = ["corpus", "oracle", "rev", "edges", "finding"];

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\u{b}' || c == '\u{c}'
}

fn is_full_sha(s: &str) -> bool {
    s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

// spec: lifecycle-kit/SPEC.md §check-survey-record — a git-object-shaped token in a field value:
// a whole word-bounded run of lowercase hex, 7 to 40 long, carrying at least one a-f so a bare
// number (a count, a compact date) is not mistaken for a citation
fn hex_tokens(s: &str) -> Vec<String> {
    s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|run| {
            (7..=40).contains(&run.len())
                && run.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
                && run.bytes().any(|b| b.is_ascii_lowercase())
        })
        .map(str::to_string)
        .collect()
}

// spec: lifecycle-kit/SPEC.md §check-survey-record — the block valve, reason mandatory: a
// hex-shaped token that names no object on purpose is exempted by an audit trail, never silently
fn exempt_reason(line: &str) -> Option<&str> {
    let inner = line.trim().strip_prefix("<!--")?.strip_suffix("-->")?;
    Some(inner.trim().strip_prefix("survey-token-exempt:")?.trim())
}

// spec: lifecycle-kit/SPEC.md §check-survey-record — the key line, `- <key>: <value>` with a
// lowercase key; a line inside a block that is not one is a stray
fn key_line(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix('-')?;
    let trimmed = rest.trim_start_matches(is_space);
    if trimmed.len() == rest.len() {
        return None;
    }
    let b = trimmed.as_bytes();
    let mut i = 0usize;
    while i < b.len() && b[i].is_ascii_lowercase() {
        i += 1;
    }
    if i == 0 || i >= b.len() || b[i] != b':' {
        return None;
    }
    let key = trimmed[..i].to_string();
    let val = trimmed[i + 1..]
        .trim_start_matches(is_space)
        .trim_end_matches(is_space)
        .to_string();
    Some((key, val))
}

struct Block {
    heading_line: usize,
    keys: Vec<(String, String, usize)>,
    tokens: Vec<(usize, String)>,
    exempt: bool,
}

fn finish(
    blk: &mut Option<Block>,
    findings: &mut Vec<(usize, String)>,
    tokens_out: &mut Vec<(usize, String)>,
) {
    let Some(b) = blk.take() else { return };
    if !b.exempt {
        tokens_out.extend(b.tokens.iter().cloned());
    }
    let k = b.keys.len();
    for (i, want) in WANT.iter().enumerate() {
        if i >= k {
            findings.push((
                b.heading_line,
                format!("block is missing its '- {}:' line", want),
            ));
            continue;
        }
        if b.keys[i].0 != *want {
            findings.push((
                b.keys[i].2,
                format!(
                    "expected the '- {}:' line here, found '- {}:'",
                    want, b.keys[i].0
                ),
            ));
        }
    }
    for e in b.keys.iter().skip(WANT.len()) {
        findings.push((
            e.2,
            format!(
                "block carries a sixth key '- {}:' — the grammar is exactly corpus/oracle/rev/edges/finding",
                e.0
            ),
        ));
    }
}

pub fn run(args: &[String]) -> i32 {
    let hermetic = args.first().filter(|a| !a.is_empty());
    let (record, probe_rev) = match hermetic {
        Some(p) => {
            if !Path::new(p.as_str()).is_file() {
                eprintln!("check-survey-record: record file not found: {}", p);
                return 2;
            }
            (p.clone(), false)
        }
        None => {
            let p = match walk::knob_scalar("LIFECYCLE_KIT_SURVEY_RECORD_FILE") {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("check-survey-record: {}", e);
                    return 2;
                }
            };
            // spec: lifecycle-kit/SPEC.md §check-survey-record — an absent record is clean and
            // counted inert: the surface is optional
            if !Path::new(&p).is_file() {
                println!(
                    "SURVEY-RECORD: clean (no record at {} — no survey filed this iteration)",
                    p
                );
                return 0;
            }
            let in_repo = proc::run("git", &["rev-parse", "--git-dir"])
                .map(|c| c.stdout().is_some())
                .unwrap_or(false);
            (p, in_repo)
        }
    };
    let text = match std::fs::read(&record) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => {
            eprintln!("check-survey-record: record file not readable: {}", record);
            return 2;
        }
    };

    let mut raw: Vec<(usize, String)> = Vec::new();
    let mut revs: Vec<(usize, String)> = Vec::new();
    let mut tokens: Vec<(usize, String)> = Vec::new();
    let mut blocks = 0usize;
    let mut blk: Option<Block> = None;
    for (idx, line) in text.lines().enumerate() {
        let fnr = idx + 1;
        if line.starts_with("##") && line[2..].starts_with(is_space) {
            finish(&mut blk, &mut raw, &mut tokens);
            blk = Some(Block {
                heading_line: fnr,
                keys: Vec::new(),
                tokens: Vec::new(),
                exempt: false,
            });
            blocks += 1;
            continue;
        }
        let Some(b) = blk.as_mut() else { continue };
        if line.chars().all(is_space) {
            continue;
        }
        if let Some(reason) = exempt_reason(line) {
            if reason.is_empty() {
                raw.push((fnr, "survey-token-exempt valve carries no reason — the reason is mandatory, and a valve without one does not exempt".to_string()));
            } else {
                b.exempt = true;
            }
            continue;
        }
        let Some((key, val)) = key_line(line) else {
            raw.push((
                fnr,
                "stray line inside a survey block (the grammar is one '- <key>: <value>' line per key)".to_string(),
            ));
            continue;
        };
        b.keys.push((key.clone(), val.clone(), fnr));
        if key == "corpus" && val.is_empty() {
            raw.push((fnr, "empty corpus — the witness has no pathspec to diff".to_string()));
        }
        if key == "oracle" && val.is_empty() {
            raw.push((fnr, "empty oracle — write the grounding command, or the literal 'none' (which marks the block a note, not a re-usable survey)".to_string()));
        }
        // spec: lifecycle-kit/SPEC.md §check-survey-record — edges is asserted on oracle's two
        // footings: an absent key and an empty value are both the silent form of 'no sum was taken'
        if key == "edges" && val.is_empty() {
            raw.push((fnr, "empty edges — write the per-candidate inbound sum, or the literal 'none' (which says this survey ranked no candidates)".to_string()));
        }
        if key == "rev" {
            if is_full_sha(&val) {
                revs.push((fnr, val));
            } else {
                raw.push((fnr, format!("rev is not a full 40-hex sha: '{}'", val)));
            }
            continue;
        }
        // spec: lifecycle-kit/SPEC.md §check-survey-record — the widened corpus is every field but
        // rev; rev has its own stricter arm above and reporting it twice would say one thing in
        // two voices
        b.tokens
            .extend(hex_tokens(&val).into_iter().map(|t| (fnr, t)));
    }
    finish(&mut blk, &mut raw, &mut tokens);

    let mut findings: Vec<String> = raw
        .into_iter()
        .map(|(l, w)| format!("{}:{}: {}", record, l, w))
        .collect();

    // spec: lifecycle-kit/SPEC.md §check-survey-record — the existence probe catches the
    // wrong-rev case the 40-hex shape cannot: a sha the tree does not carry makes
    // 'git diff <rev>..HEAD' fail rather than witness anything
    let mut probed = 0usize;
    let mut tokens_probed = 0usize;
    if probe_rev {
        for (line, rev) in &revs {
            let spec = format!("{}^{{commit}}", rev);
            let ok = proc::run("git", &["cat-file", "-e", &spec])
                .map(|c| c.stdout().is_some())
                .unwrap_or(false);
            if ok {
                probed += 1;
            } else {
                findings.push(format!(
                    "{}:{}: rev names no commit in this repository: {}",
                    record, line, rev
                ));
            }
        }
        // spec: lifecycle-kit/SPEC.md §check-survey-record — the same probe over a wider corpus:
        // any object type resolves, because a sha naming a blob or a tree is a real citation
        for (line, tok) in &tokens {
            let ok = proc::run("git", &["cat-file", "-e", tok])
                .map(|c| c.stdout().is_some())
                .unwrap_or(false);
            if ok {
                tokens_probed += 1;
            } else {
                findings.push(format!(
                    "{}:{}: git-object-shaped token names no object in this repository: {}",
                    record, line, tok
                ));
            }
        }
    }

    if !findings.is_empty() {
        println!(
            "check-survey-record: {} malformed survey block(s) in {}:",
            findings.len(),
            record
        );
        for f in &findings {
            println!("  {}", f);
        }
        println!("  help: each '## <date> <stage> — <question>' block carries exactly five lines — '- corpus:', '- oracle:', '- rev:', '- edges:', '- finding:' — in that order, with a non-empty corpus, a non-empty oracle (the literal 'none' is the honest form for a survey no oracle grounds), a full 40-hex rev naming a real commit, and a non-empty edges carrying the per-candidate inbound-citation sum this survey ranked on (the literal 'none' when it ranked none). Every git-object-shaped token in the other four fields must name a real object too — an identifier you did not read is not a citation — and one that names none on purpose takes a '<!-- survey-token-exempt: <reason> -->' line on its block, reason mandatory. File blocks with 'bash gate-sdk/bin/run-gates.sh --emit file-survey \"<question>\" \"<corpus>\" \"<oracle>\" \"<edges>\" \"<finding>\"', which stamps the rev itself.");
        return 1;
    }

    if probe_rev {
        println!(
            "SURVEY-RECORD: clean ({} block(s) in {}; grammar holds, {} rev(s) name a real commit and {} cited token(s) name a real object)",
            blocks, record, probed, tokens_probed
        );
    } else {
        println!(
            "SURVEY-RECORD: clean ({} block(s) in {}; grammar holds — hermetic file argument, so no rev-existence probe)",
            blocks, record
        );
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_key_line_needs_a_dash_a_space_a_lowercase_key_and_a_colon() {
        assert_eq!(
            key_line("-  corpus:  a b  "),
            Some(("corpus".to_string(), "a b".to_string()))
        );
        assert_eq!(key_line("- rev:"), Some(("rev".to_string(), String::new())));
        assert_eq!(key_line("-corpus: x"), None);
        assert_eq!(key_line("- Corpus: x"), None);
        assert_eq!(key_line("prose"), None);
    }

    #[test]
    fn the_rev_shape_is_forty_lowercase_hex_digits() {
        assert!(is_full_sha(&"a1b2c3d4e5".repeat(4)));
        assert!(!is_full_sha(&"A1B2C3D4E5".repeat(4)));
        assert!(!is_full_sha("0123456"));
    }

    #[test]
    fn a_token_is_a_whole_word_of_seven_to_forty_lowercase_hex_carrying_at_least_one_letter() {
        assert_eq!(hex_tokens("taken at 4c0d3bb4 exactly"), vec!["4c0d3bb4"]);
        assert_eq!(hex_tokens("path/to-9b01495a.txt"), vec!["9b01495a"]);
        assert_eq!(hex_tokens(&"a".repeat(40)), vec!["a".repeat(40)]);
        assert!(hex_tokens("deadbeefzz").is_empty());
        assert!(hex_tokens("xdeadbeef").is_empty());
        assert!(hex_tokens("abc123").is_empty());
        assert!(hex_tokens(&"a".repeat(41)).is_empty());
        assert!(hex_tokens("ABCDEF1").is_empty());
        assert!(hex_tokens("20260824").is_empty());
        assert!(hex_tokens("1234567 files").is_empty());
    }

    #[test]
    fn the_valve_needs_its_reason_and_nothing_else_is_a_valve() {
        assert_eq!(
            exempt_reason("  <!-- survey-token-exempt: an illustrative sha -->  "),
            Some("an illustrative sha")
        );
        assert_eq!(exempt_reason("<!-- survey-token-exempt: -->"), Some(""));
        assert_eq!(exempt_reason("<!-- comment-tier-exempt: x -->"), None);
        assert_eq!(exempt_reason("- corpus: x"), None);
    }
}
