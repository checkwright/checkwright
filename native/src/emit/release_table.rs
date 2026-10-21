// spec: installer/SPEC.md §The releases table — one row per release note, newest first; one
// function the gate calls in process
use crate::declaration;
use crate::emit::support_table as st;
use crate::gates::release_bump as rb;
use crate::release_sections::{self as rs, Role, Section};
use crate::walk;
use std::path::Path;

pub const POSTS_KNOB: &str = "GATE_LOCAL_RELEASE_POSTS_DIR";
pub const PAGE_KNOB: &str = "GATE_LOCAL_RELEASES_PAGE";
const BEGIN: &str = "<!-- release-table:begin -->";
const END: &str = "<!-- release-table:end -->";
const ABSENT: &str = "—";
const ARROW: char = '→';

struct Note {
    version: rb::Version,
    release: String,
    link: String,
    date: String,
    text: String,
}

fn date_of(path: &Path) -> Result<String, String> {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let d = name.get(..10).unwrap_or("");
    let shape = d.bytes().enumerate().all(|(i, b)| if i == 4 || i == 7 { b == b'-' } else { b.is_ascii_digit() });
    if d.len() == 10 && shape {
        return Ok(d.to_string());
    }
    Err(format!("{}: the filename does not open with its YYYY-MM-DD date", path.display()))
}

fn notes(posts: &str, page: &str) -> Result<Vec<Note>, String> {
    if !Path::new(posts).is_dir() {
        return Err(format!("posts dir not found: {}", posts));
    }
    let cwd = walk::cwd()?;
    let page_dir = Path::new(page).parent().map(|p| p.display().to_string()).filter(|p| !p.is_empty());
    let from = walk::abs_against(&cwd, page_dir.as_deref().unwrap_or("."));
    let mut out: Vec<Note> = Vec::new();
    for f in walk::glob_files(Path::new(posts), &["*.md".to_string()])? {
        let path = f.display().to_string();
        let text = rb::read_text(&path)?;
        let Some(release) = rb::front_matter_release(&text).filter(|v| !v.is_empty()) else {
            continue;
        };
        let version = rb::parse_version(release.strip_prefix('v').unwrap_or(&release), &path)?;
        if let Some(twin) = out.iter().find(|n| n.version == version) {
            return Err(format!("{}: release {} is also {}'s — one version, one note", path, release, twin.link));
        }
        out.push(Note {
            version,
            date: date_of(&f)?,
            link: walk::relative_to(&from, &walk::abs_against(&cwd, &path)),
            release,
            text,
        });
    }
    out.sort_by(|a, b| b.version.cmp(&a.version));
    Ok(out)
}

fn count(text: &str, s: &Section) -> Option<usize> {
    declaration::section_bullets(text, &s.names()).map(|b| b.len())
}

// spec: installer/SPEC.md §The releases table — a gates bullet, or a knob bullet carrying the
// arrow after its lead token: a rename or a removal, never an addition
fn needs_action(text: &str, roster: &[Section]) -> bool {
    let bullets = |role: Role| -> Vec<&str> {
        rs::find(roster, role)
            .and_then(|s| declaration::section_bullets(text, &s.names()))
            .unwrap_or_default()
    };
    !bullets(Role::Gates).is_empty()
        || bullets(Role::Knobs)
            .iter()
            .any(|b| declaration::after_lead_token(b).is_some_and(|rest| rest.contains(ARROW)))
}

fn table(notes: &[Note], roster: &[Section]) -> String {
    let counted: Vec<&Section> = roster.iter().filter(|s| s.role.declaration_bearing()).collect();
    let mut head = vec!["Release", "Date", "Bump", "Action on upgrade"];
    head.extend(counted.iter().map(|s| s.heading.as_str()));
    let mut out = vec![format!("| {} |", head.join(" | ")), format!("|{}", " --- |".repeat(head.len()))];
    for (i, n) in notes.iter().enumerate() {
        let bump = notes.get(i + 1).map(|older| n.version.bump_over(&older.version)).unwrap_or(ABSENT);
        let action = if needs_action(&n.text, roster) { "yes" } else { "no" };
        let mut cells = vec![format!("[{}]({})", n.release, n.link), n.date.clone(), bump.to_string(), action.to_string()];
        cells.extend(counted.iter().map(|s| count(&n.text, s).map(|c| c.to_string()).unwrap_or_else(|| ABSENT.to_string())));
        out.push(format!("| {} |", cells.join(" | ")));
    }
    format!("\n{}\n", out.join("\n"))
}

// spec: installer/SPEC.md §The releases table — the table between the markers' blank lines
pub fn render(posts: &str, page: &str) -> Result<String, String> {
    let roster = rs::roster()?;
    Ok(table(&notes(posts, page)?, &roster))
}

pub fn block(page: &str, text: &str) -> Result<st::Block, String> {
    st::marked_block(page, text, BEGIN, END)
}

// spec: installer/SPEC.md §The releases table — the bare arm prints the block; `--write`
// rewrites it in place and touches nothing else
pub fn emit(args: &[String]) -> Result<String, String> {
    let posts = walk::knob_scalar(POSTS_KNOB)?;
    let page = walk::knob_scalar(PAGE_KNOB)?;
    let table = render(&posts, &page)?;
    if args.iter().any(|a| a == "--write") {
        return st::write_marked("release-table", &page, BEGIN, END, &table);
    }
    Ok(format!("{}\n", table.trim_matches('\n')))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn roster() -> Vec<Section> {
        let v = |e: &[&str]| e.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        rs::parse(
            &v(&["brief: In brief", "gates: Gates", "knobs: Knobs", "behavior: Behavior"]),
            &v(&["gates: Old gates"]),
        )
        .expect("roster refused")
    }

    fn note(name: &str, release: &str, body: &str) -> (String, String) {
        (name.to_string(), format!("---\nrelease: {}\n---\n\n# x\n\n{}", release, body))
    }

    fn tree(name: &str, files: &[(String, String)]) -> String {
        let root = crate::proc::temp_base().join(format!("release-table-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("posts")).unwrap();
        for (f, body) in files {
            fs::write(root.join("posts").join(f), body).unwrap();
        }
        root.display().to_string()
    }

    fn rows(name: &str, files: &[(String, String)]) -> Vec<Vec<String>> {
        let root = tree(name, files);
        let posts = format!("{}/posts", root);
        let page = format!("{}/releases.md", root);
        let got = table(&notes(&posts, &page).unwrap(), &roster());
        let _ = fs::remove_dir_all(&root);
        got.trim()
            .lines()
            .skip(2)
            .map(|l| l.trim_matches('|').split('|').map(|c| c.trim().to_string()).collect())
            .collect()
    }

    fn render_err(name: &str, files: &[(String, String)]) -> String {
        let root = tree(name, files);
        let got = notes(&format!("{}/posts", root), &format!("{}/releases.md", root)).err();
        let _ = fs::remove_dir_all(&root);
        got.expect("refusal owed")
    }

    const NONE: &str = "## Gates\n\nNone.\n\n## Knobs\n\nNone.\n\n## Behavior\n\nNone.\n";

    #[test]
    fn each_bump_class_and_a_dash_on_the_oldest_row() {
        let r = rows(
            "bump",
            &[
                note("2026-01-01-a.md", "v0.1.0", NONE),
                note("2026-01-02-b.md", "v0.1.1", NONE),
                note("2026-01-03-c.md", "v0.2.0", NONE),
                note("2026-01-04-d.md", "v1.0.0", NONE),
                note("2026-01-05-post.md", "", NONE),
                ("2026-01-06-announce.md".to_string(), "# no front matter\n".to_string()),
            ],
        );
        let bumps: Vec<&str> = r.iter().map(|c| c[2].as_str()).collect();
        assert_eq!(bumps, vec!["major", "minor", "patch", "—"]);
        assert_eq!(r[0][0], "[v1.0.0](posts/2026-01-04-d.md)");
        assert_eq!(r[0][1], "2026-01-04");
    }

    #[test]
    fn a_gate_a_rename_or_a_removal_needs_action_and_an_addition_alone_does_not() {
        let body = |gates: &str, knobs: &str| format!("## Gates\n\n{}\n\n## Knobs\n\n{}\n\n## Behavior\n\nNone.\n", gates, knobs);
        let r = rows(
            "action",
            &[
                note("2026-01-01-a.md", "v0.1.0", &body("- `check-x` — new.", "None.")),
                note("2026-01-02-b.md", "v0.2.0", &body("None.", "- `OLD` → `NEW` — renamed.")),
                note("2026-01-03-c.md", "v0.3.0", &body("None.", "- `GONE`, `ALSO_GONE` → ∅")),
                note("2026-01-04-d.md", "v0.4.0", &body("None.", "- `NEW_KNOB` — new, default empty.")),
            ],
        );
        let action: Vec<&str> = r.iter().map(|c| c[3].as_str()).collect();
        assert_eq!(action, vec!["no", "yes", "yes", "yes"]);
    }

    #[test]
    fn a_none_body_counts_zero_and_an_absent_section_a_dash_and_an_alias_counts_under_its_role() {
        let r = rows(
            "count",
            &[note(
                "2026-01-01-a.md",
                "v0.1.0",
                "## Old gates\n\n- `check-x` — new.\n- `check-y` — tightened.\n\n## Knobs\n\nNone.\n",
            )],
        );
        assert_eq!(r[0][4..], ["2", "0", "—"]);
    }

    #[test]
    fn every_refusal_is_an_error() {
        let e = render_err("grammar", &[note("2026-01-01-a.md", "v1.0", NONE)]);
        assert!(e.contains("outside the grammar"), "{}", e);
        let e = render_err("twin", &[note("2026-01-01-a.md", "v0.1.0", NONE), note("2026-01-02-b.md", "0.1.0", NONE)]);
        assert!(e.contains("one version, one note"), "{}", e);
        let e = render_err("date", &[note("checkwright-v0-1-0.md", "v0.1.0", NONE)]);
        assert!(e.contains("YYYY-MM-DD"), "{}", e);
        assert!(notes("/nonexistent-release-posts", "releases.md").is_err());
        assert!(rs::parse(&["gates: G".to_string(), "knobs: G".to_string()], &[]).is_err());
        let page = format!("# R\n\n{}\n\n{}\n", BEGIN, END);
        assert!(block("p.md", &page).is_ok());
        assert!(block("p.md", "# R\n").is_err());
        assert!(block("p.md", &format!("{0}\n{1}\n{0}\n{1}\n", BEGIN, END)).is_err());
    }
}
