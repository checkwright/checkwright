// spec: site-kit/SPEC.md §check-docs-since — the governed pages' `since:` key, the untagged note it
// is held against, and the rewrite the release note's commit runs; one reader the gate calls in process
use crate::gates::release_bump::untagged_notes;
use crate::spec;
use crate::walk;
use std::path::Path;

pub const PAGES_KNOB: &str = "SITE_KIT_SINCE_PAGES";
pub const NOTES_KNOB: &str = "SITE_KIT_RELEASE_NOTES_DIR";
const UNRELEASED: &str = "unreleased";

#[derive(Debug, PartialEq)]
pub enum Since {
    Missing,
    Malformed(usize, String),
    Unreleased(usize),
    Version(usize, String),
    Duplicate(usize, usize),
}

fn is_version(v: &str) -> bool {
    let Some(rest) = v.strip_prefix('v') else {
        return false;
    };
    let parts: Vec<&str> = rest.split('.').collect();
    parts.len() == 3 && parts.iter().all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

// spec: site-kit/SPEC.md §check-docs-since — the key is read inside the front-matter span alone; an
// opening rule no later rule closes is a malformed span, an error rather than a missing key; a
// second `since:` line is a duplicate whatever either value says
pub fn read(text: &str) -> Result<Since, String> {
    let lines: Vec<&str> = text.lines().collect();
    let span = spec::front_matter_span(&lines);
    if span == 0 && lines.first().is_some_and(|l| l.trim_end() == "---") {
        return Err("front matter opens with --- and no later --- or ... closes it".to_string());
    }
    let mut keys = lines[..span].iter().enumerate().filter_map(|(i, l)| Some((i + 1, l.strip_prefix("since:")?)));
    let Some((line, rest)) = keys.next() else {
        return Ok(Since::Missing);
    };
    if let Some((second, _)) = keys.next() {
        return Ok(Since::Duplicate(line, second));
    }
    let v = rest.trim_matches([' ', '\t', '\r']).to_string();
    Ok(if v == UNRELEASED {
        Since::Unreleased(line)
    } else if is_version(&v) {
        Since::Version(line, v)
    } else {
        Since::Malformed(line, v)
    })
}

// spec: site-kit/SPEC.md §check-docs-since — the governed set: the files SITE_KIT_SINCE_PAGES's
// globs match from the work-tree top, pruned as every corpus glob is
pub fn pages() -> Result<Vec<String>, String> {
    let globs = spec::knob_array_pub(PAGES_KNOB)?;
    if globs.is_empty() {
        return Ok(Vec::new());
    }
    let mut out: Vec<String> = walk::glob_corpus(Path::new("."), &globs)?
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();
    out.sort();
    out.dedup();
    Ok(out)
}

pub enum Note {
    Off,
    None(String),
    Untagged(String, String),
}

// spec: site-kit/SPEC.md §check-docs-since — the note the rule reads: off while the notes knob is
// empty, and two untagged notes are a state the release choreography does not admit
pub fn note() -> Result<Note, String> {
    let dirs = walk::knob_words(NOTES_KNOB)?;
    let Some(dir) = dirs.first() else {
        return Ok(Note::Off);
    };
    let dir = dir.trim_end_matches('/').to_string();
    if !Path::new(&dir).is_dir() {
        return Err(format!("release notes dir not found: {}", dir));
    }
    let mut untagged = untagged_notes(Path::new(&dir))?;
    match untagged.len() {
        0 => Ok(Note::None(dir)),
        1 => {
            let (v, path) = untagged.remove(0);
            Ok(Note::Untagged(v, path))
        }
        _ => Err(format!(
            "{} carries more than one untagged release note, a state the release choreography does not admit: {} — tag the released one or remove the stray note",
            dir,
            untagged.iter().map(|(v, p)| format!("{} ({})", p, v)).collect::<Vec<_>>().join(", ")
        )),
    }
}

// spec: site-kit/SPEC.md §check-docs-since — the rewrite replaces the one `since:` line and keeps
// every other byte, line endings included
pub fn rewrite(text: &str, line: usize, version: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for (i, l) in text.split_inclusive('\n').enumerate() {
        if i + 1 == line {
            let ending = &l[l.trim_end_matches(['\n', '\r']).len()..];
            out.push_str(&format!("since: {}{}", version, ending));
        } else {
            out.push_str(l);
        }
    }
    out
}

// spec: site-kit/SPEC.md §check-docs-since — the bare arm prints each page it would rewrite;
// `--write` rewrites them; with no untagged note there is no version to write, so it refuses
pub fn emit(args: &[String]) -> Result<String, String> {
    let write = args.iter().any(|a| a == "--write");
    let pages = pages()?;
    if pages.is_empty() {
        return Ok(format!("docs-since: {} matches no page — no page is governed\n", PAGES_KNOB));
    }
    apply(&pages, note()?, write)
}

fn apply(pages: &[String], note: Note, write: bool) -> Result<String, String> {
    let version = match note {
        Note::Off => return Ok(format!("docs-since: {} is empty — the rewrite is off\n", NOTES_KNOB)),
        Note::None(dir) => {
            return Err(format!(
                "no untagged release note under {}, so there is no version to write — author the release note first",
                dir
            ))
        }
        Note::Untagged(v, _) => v,
    };
    let mut open = Vec::new();
    for page in pages {
        let text = std::fs::read_to_string(page).map_err(|e| format!("cannot read {}: {}", page, e))?;
        match read(&text).map_err(|e| format!("{}: {}", page, e))? {
            Since::Unreleased(line) => open.push((page, text, line)),
            Since::Duplicate(first, second) => {
                return Err(format!(
                    "{}:{}: a second since: key beside line {}, so no one line says the page's release — keep one",
                    page, second, first
                ))
            }
            _ => {}
        }
    }
    let mut out = String::new();
    for (page, text, line) in open {
        if write {
            std::fs::write(page, rewrite(&text, line, &version)).map_err(|e| format!("cannot write {}: {}", page, e))?;
            out.push_str(&format!("docs-since: {}:{}: since: {} → {}\n", page, line, UNRELEASED, version));
        } else {
            out.push_str(&format!("{}:{}: since: {} → {}\n", page, line, UNRELEASED, version));
        }
    }
    if out.is_empty() {
        out = format!("docs-since: no governed page says since: {}\n", UNRELEASED);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_key_reads_inside_the_span_and_classifies_its_value() {
        assert_eq!(read("---\ntitle: x\nsince: unreleased\n---\n# x\n"), Ok(Since::Unreleased(3)));
        assert_eq!(read("---\nsince: v1.20.3\n---\n"), Ok(Since::Version(2, "v1.20.3".into())));
        assert_eq!(read("---\nsince: v1.2\n---\n"), Ok(Since::Malformed(2, "v1.2".into())));
        assert_eq!(read("---\nsince: \"unreleased\"\n---\n"), Ok(Since::Malformed(2, "\"unreleased\"".into())));
        assert_eq!(read("---\nsince: 1.2.3\n---\n"), Ok(Since::Malformed(2, "1.2.3".into())));
        assert_eq!(read("---\ntitle: x\n---\nsince: unreleased\n"), Ok(Since::Missing));
        assert_eq!(read("# no front matter\n"), Ok(Since::Missing));
        assert!(read("---\nsince: unreleased\n").is_err());
        assert_eq!(read("---\nsince: v0.1.0\ntitle: x\nsince: unreleased\n---\n"), Ok(Since::Duplicate(2, 4)));
        assert_eq!(read("---\nsince: v0.1.0\nsince: v0.1.0\n---\n"), Ok(Since::Duplicate(2, 3)));
        assert_eq!(read("---\nsince: v0.1.0\n---\nsince: unreleased\n"), Ok(Since::Version(2, "v0.1.0".into())));
    }

    #[test]
    fn the_rewrite_replaces_one_line_and_keeps_every_other_byte() {
        let text = "---\r\ntitle: x\r\nsince: unreleased\r\n---\r\nbody since: unreleased\r\n";
        assert_eq!(
            rewrite(text, 3, "v0.40.0"),
            "---\r\ntitle: x\r\nsince: v0.40.0\r\n---\r\nbody since: unreleased\r\n"
        );
        assert_eq!(rewrite("---\nsince: unreleased", 2, "v1.0.0"), "---\nsince: v1.0.0");
    }

    // spec: site-kit/SPEC.md §check-docs-since — the arm writes the untagged note's version over
    // every `unreleased` page, leaves a versioned page alone, and refuses with no note to read
    #[test]
    fn the_arm_writes_the_note_version_and_refuses_with_no_untagged_note() {
        let d = std::env::temp_dir().join(format!("checkwright-docs-since.{}", std::process::id()));
        std::fs::create_dir_all(&d).expect("scratch");
        let open = d.join("open.md").display().to_string();
        let done = d.join("done.md").display().to_string();
        std::fs::write(&open, "---\ntitle: a\nsince: unreleased\n---\n# a\n").expect("write");
        std::fs::write(&done, "---\nsince: v0.1.0\n---\n# b\n").expect("write");
        let pages = vec![done.clone(), open.clone()];

        let e = apply(&pages, Note::None("posts".into()), true).expect_err("wrote with no untagged note");
        assert!(e.contains("no untagged release note under posts"), "{}", e);
        assert_eq!(std::fs::read_to_string(&open).unwrap(), "---\ntitle: a\nsince: unreleased\n---\n# a\n");

        let dry = apply(&pages, Note::Untagged("v0.2.0".into(), "posts/n.md".into()), false).expect("dry");
        assert_eq!(dry, format!("{}:3: since: unreleased → v0.2.0\n", open));
        assert!(std::fs::read_to_string(&open).unwrap().contains("since: unreleased"));

        apply(&pages, Note::Untagged("v0.2.0".into(), "posts/n.md".into()), true).expect("write");
        assert_eq!(std::fs::read_to_string(&open).unwrap(), "---\ntitle: a\nsince: v0.2.0\n---\n# a\n");
        assert_eq!(std::fs::read_to_string(&done).unwrap(), "---\nsince: v0.1.0\n---\n# b\n");
        let again = apply(&pages, Note::Untagged("v0.2.0".into(), "posts/n.md".into()), true).expect("rerun");
        assert_eq!(again, "docs-since: no governed page says since: unreleased\n");

        let twice = d.join("twice.md").display().to_string();
        std::fs::write(&open, "---\nsince: unreleased\n---\n").expect("write");
        std::fs::write(&twice, "---\nsince: v0.1.0\nsince: unreleased\n---\n").expect("write");
        let e = apply(&[open.clone(), twice.clone()], Note::Untagged("v0.2.0".into(), "posts/n.md".into()), true)
            .expect_err("wrote beside a duplicated key");
        assert!(e.contains(&format!("{}:3: a second since: key beside line 2", twice)), "{}", e);
        assert_eq!(std::fs::read_to_string(&open).unwrap(), "---\nsince: unreleased\n---\n", "nothing is written");
        let _ = std::fs::remove_dir_all(&d);
    }
}
