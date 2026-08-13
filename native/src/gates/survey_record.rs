// spec: lifecycle-kit/SPEC.md §check-survey-record — every survey block carries its whole
// witness: the four keys in order, a full-sha rev naming a real commit, a non-empty corpus
// and a non-empty oracle
use crate::proc;
use crate::walk;
use std::path::Path;

const WANT: [&str; 4] = ["corpus", "oracle", "rev", "finding"];

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\u{b}' || c == '\u{c}'
}

fn is_full_sha(s: &str) -> bool {
    s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
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
}

fn finish(blk: &mut Option<Block>, findings: &mut Vec<(usize, String)>) {
    let Some(b) = blk.take() else { return };
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
                "block carries a fifth key '- {}:' — the grammar is exactly corpus/oracle/rev/finding",
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
    let mut blocks = 0usize;
    let mut blk: Option<Block> = None;
    for (idx, line) in text.lines().enumerate() {
        let fnr = idx + 1;
        if line.starts_with("##") && line[2..].starts_with(is_space) {
            finish(&mut blk, &mut raw);
            blk = Some(Block {
                heading_line: fnr,
                keys: Vec::new(),
            });
            blocks += 1;
            continue;
        }
        let Some(b) = blk.as_mut() else { continue };
        if line.chars().all(is_space) {
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
        if key == "rev" {
            if is_full_sha(&val) {
                revs.push((fnr, val));
            } else {
                raw.push((fnr, format!("rev is not a full 40-hex sha: '{}'", val)));
            }
        }
    }
    finish(&mut blk, &mut raw);

    let mut findings: Vec<String> = raw
        .into_iter()
        .map(|(l, w)| format!("{}:{}: {}", record, l, w))
        .collect();

    // spec: lifecycle-kit/SPEC.md §check-survey-record — the existence probe catches the
    // wrong-rev case the 40-hex shape cannot: a sha the tree does not carry makes
    // 'git diff <rev>..HEAD' fail rather than witness anything
    let mut probed = 0usize;
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
        println!("  help: each '## <date> <stage> — <question>' block carries exactly four lines — '- corpus:', '- oracle:', '- rev:', '- finding:' — in that order, with a non-empty corpus, a non-empty oracle (the literal 'none' is the honest form for a survey no oracle grounds), and a full 40-hex rev naming a real commit. File blocks with 'bash lifecycle-kit/bin/file-survey.sh \"<question>\" \"<corpus>\" \"<oracle>\" \"<finding>\"', which stamps the rev itself.");
        return 1;
    }

    if probe_rev {
        println!(
            "SURVEY-RECORD: clean ({} block(s) in {}; grammar holds and {} rev(s) name a real commit)",
            blocks, record, probed
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
}
