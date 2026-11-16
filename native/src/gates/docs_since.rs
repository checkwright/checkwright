// spec: site-kit/SPEC.md §check-docs-since — every governed page carries a well-formed `since:`,
// and none says `unreleased` while a release note is untagged
use crate::emit::docs_since::{self, Note, Since};

const NAME: &str = "check-docs-since";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

#[derive(Debug, PartialEq)]
enum Finding {
    Missing(String),
    Malformed(String, usize, String),
    Unreleased(String, usize),
}

fn findings(pages: &[(String, String)], note: &Note) -> Result<Vec<Finding>, String> {
    let mut out = Vec::new();
    for (page, text) in pages {
        match docs_since::read(text).map_err(|e| format!("{}: {}", page, e))? {
            Since::Missing => out.push(Finding::Missing(page.clone())),
            Since::Malformed(line, v) => out.push(Finding::Malformed(page.clone(), line, v)),
            Since::Unreleased(line) if matches!(note, Note::Untagged(..)) => out.push(Finding::Unreleased(page.clone(), line)),
            _ => {}
        }
    }
    Ok(out)
}

fn rule(_args: &[String]) -> Result<i32, String> {
    let pages = docs_since::pages()?;
    if pages.is_empty() {
        println!("DOCS-SINCE: clean (not applicable: {} matches no page — no page is governed, 0 checked)", docs_since::PAGES_KNOB);
        return Ok(0);
    }
    let mut texts = Vec::new();
    for page in &pages {
        let text = std::fs::read(page)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("cannot read {}: {}", page, e))?;
        texts.push((page.clone(), text));
    }
    let note = docs_since::note()?;
    let found = findings(&texts, &note)?;
    if found.is_empty() {
        let rule = match &note {
            Note::Off => format!("the note rule is off while {} is empty", docs_since::NOTES_KNOB),
            Note::None(dir) => format!("no untagged release note under {}", dir),
            Note::Untagged(v, _) => format!("none says unreleased beside the untagged {} note", v),
        };
        println!("DOCS-SINCE: clean ({} page(s) carry since:; {})", pages.len(), rule);
        return Ok(0);
    }
    let mut key = false;
    for f in &found {
        match f {
            Finding::Missing(page) => {
                println!("{}: {}: no since: key in its front matter", NAME, page);
                key = true;
            }
            Finding::Malformed(page, line, v) => {
                println!("{}: {}:{}: since: '{}' is neither unreleased nor vX.Y.Z", NAME, page, line, v);
                key = true;
            }
            Finding::Unreleased(page, line) => {
                if let Note::Untagged(v, path) = &note {
                    println!("{}: {}:{}: says since: unreleased while the {} note {} is untagged", NAME, page, line, v, path);
                }
            }
        }
    }
    if key {
        println!("  help: give the page since: unreleased, or since: vX.Y.Z naming the first release that carries what it says");
    }
    if found.iter().any(|f| matches!(f, Finding::Unreleased(..))) {
        println!("  help: rewrite in the note-authoring commit — {}", super::door_command("--emit docs-since --write")?);
    }
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(name: &str, text: &str) -> (String, String) {
        (name.to_string(), text.to_string())
    }

    // spec: site-kit/SPEC.md §check-docs-since — the note rule fires only beside an untagged note;
    // a versioned page passes beside one, and the presence rule runs with the note rule off
    #[test]
    fn the_note_rule_reds_unreleased_beside_an_untagged_note_alone() {
        let pages = vec![
            page("open.md", "---\nsince: unreleased\n---\n"),
            page("done.md", "---\nsince: v0.3.0\n---\n"),
        ];
        let untagged = Note::Untagged("v0.4.0".into(), "posts/n.md".into());
        assert_eq!(findings(&pages, &untagged), Ok(vec![Finding::Unreleased("open.md".into(), 2)]));
        assert_eq!(findings(&pages[1..], &untagged), Ok(vec![]));
        assert_eq!(findings(&pages, &Note::None("posts".into())), Ok(vec![]));
        assert_eq!(findings(&pages, &Note::Off), Ok(vec![]));
        let bare = vec![page("bare.md", "# x\n")];
        assert_eq!(findings(&bare, &Note::Off), Ok(vec![Finding::Missing("bare.md".into())]));
        assert!(findings(&[page("open.md", "---\nsince: unreleased\n")], &Note::Off).is_err());
    }
}
