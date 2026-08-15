// spec: docs/install.md §The upgrade contract — every release note's Tightened-gates section
// resolves to an explicit `None` or to a non-empty set of backticked bare gate names; a
// non-`none` section yielding no tokens is the silently-empty declaration the smoke cannot see
use crate::declaration;
use crate::declaration::SectionVerdict;
use crate::gates::release_bump::read_text;
use crate::walk;
use std::path::Path;

const DEFAULT_POSTS: &str = "docs/posts";
const SECTION: &str = "Tightened gates";

// spec: gate-sdk/SPEC.md §lib/declaration.sh — `^release:[[:space:]]+v`, the gate's own literal
// shape, so it is matched directly rather than compiled
fn is_note(text: &str) -> bool {
    text.lines().any(|l| {
        l.strip_prefix("release:")
            .map(|r| {
                let t = r.trim_start_matches([' ', '\t', '\r', '\x0b', '\x0c']);
                t.len() < r.len() && t.starts_with('v')
            })
            .unwrap_or(false)
    })
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-tightened-gates-grammar: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let posts = args
        .first()
        .filter(|a| !a.is_empty())
        .map(String::as_str)
        .unwrap_or(DEFAULT_POSTS);
    if !Path::new(posts).is_dir() {
        return Err(format!("posts dir not found: {}", posts));
    }

    let mut errors: Vec<String> = Vec::new();
    let (mut notes, mut tokens, mut none) = (0usize, 0usize, 0usize);
    for f in walk::glob_files(Path::new(posts), &["*.md".to_string()])? {
        let path = f.display().to_string();
        let text = read_text(&path)?;
        if !is_note(&text) {
            continue;
        }
        notes += 1;
        match declaration::section_tokens(&text, SECTION) {
            SectionVerdict::ExplicitNone => none += 1,
            SectionVerdict::Tokens(t) => tokens += t.len(),
            SectionVerdict::Unparsed(b) if b.is_empty() => errors.push(format!(
                "{}: '{}' is not `None` and yields no lead token at all — the parse resolves to an empty allowed-red set the note contradicts",
                path, SECTION
            )),
            SectionVerdict::Unparsed(b) => {
                for line in b {
                    errors.push(format!(
                        "{}: '{}' bullet's lead token is unreadable: {}",
                        path,
                        SECTION,
                        line.chars().take(72).collect::<String>()
                    ));
                }
            }
            SectionVerdict::Absent => errors.push(format!(
                "{}: no '{}' section — every release note carries the fixed sections its note grammar rosters",
                path, SECTION
            )),
        }
    }

    if !errors.is_empty() {
        println!(
            "check-tightened-gates-grammar: {} unreadable tightened-gates declaration(s):",
            errors.len()
        );
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: a Tightened-gates bullet's lead token is a backticked, unbolded bare gate name directly after the bullet marker (- `check-foo` — …); strip any bold emphasis and add the backticks. A release that tightened nothing states a bare \"None.\" body instead. docs/install.md §The upgrade contract owns the grammar; a mechanical consumer reads these tokens as the release's allowed-red set, so a section that parses to nothing disarms it silently.");
        return Ok(1);
    }
    println!(
        "TIGHTENED-GATES-GRAMMAR: clean ({} release note(s) under {}; {} declare `None`, the rest resolve {} lead token(s))",
        notes, posts, none, tokens
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_note_is_a_file_whose_release_key_carries_a_v_prefixed_value() {
        assert!(is_note("---\nrelease: v1.2.3\n---\n"));
        assert!(!is_note("---\nrelease:v1.2.3\n---\n"));
        assert!(!is_note("---\nrelease: 1.2.3\n---\n"));
        assert!(!is_note("# an announcement post with no front matter\n"));
    }
}
