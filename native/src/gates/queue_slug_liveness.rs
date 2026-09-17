// spec: queue-kit/SPEC.md §check-queue-slug-liveness — every slug-shaped bold-code token in a
// configured prose surface resolves against the queue's live slug set, no configured citation
// surface cites a retired slug, and every status parenthetical in the queue matches its slug
use crate::queue;
use crate::walk;
use std::path::{Path, PathBuf};

const VALVE: &str = "retired-citation-exempt:";

// spec: queue-kit/SPEC.md §check-queue-slug-liveness — ``**`slug`**``: the bold-code form that
// claims queue membership, scanned for every occurrence on the line
fn bold_code_tokens(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 6 < b.len() {
        if !(b[i] == b'*' && b[i + 1] == b'*' && b[i + 2] == b'`') {
            i += 1;
            continue;
        }
        let start = i + 3;
        if start >= b.len() || !(b[start].is_ascii_lowercase() || b[start].is_ascii_digit()) {
            i += 1;
            continue;
        }
        let mut j = start + 1;
        while j < b.len()
            && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-')
        {
            j += 1;
        }
        if j + 2 < b.len() && b[j] == b'`' && b[j + 1] == b'*' && b[j + 2] == b'*' {
            out.push(&line[start..j]);
            i = j + 3;
            continue;
        }
        i += 1;
    }
    out
}

// spec: queue-kit/SPEC.md §check-queue-slug-liveness — assertion B's token is single-backtick: a
// double-backtick span is a quoted literal, not a citation
fn single_backtick_slugs(line: &str) -> Vec<&str> {
    let b = line.as_bytes();
    queue::backtick_slugs(line)
        .into_iter()
        .filter(|&(s, e)| !(s >= 2 && b[s - 2] == b'`') && !(e + 1 < b.len() && b[e + 1] == b'`'))
        .map(|(s, e)| &line[s..e])
        .collect()
}

// spec: queue-kit/SPEC.md §check-queue-slug-liveness — the valve rides the line or the one above,
// with a mandatory reason
fn valved(lines: &[&str], i: usize) -> bool {
    let has = |l: &str| {
        l.find(VALVE)
            .map(|at| {
                let rest = l[at + VALVE.len()..].trim_start();
                let reason = rest.split("-->").next().unwrap_or("").trim();
                !reason.is_empty()
            })
            .unwrap_or(false)
    };
    has(lines[i]) || (i > 0 && has(lines[i - 1]))
}

// spec: queue-kit/SPEC.md §check-queue-slug-liveness — assertion C's vocabulary: the configured
// single-word section names lowercased, plus `retired`
fn status_vocabulary(sec: &queue::Sections) -> Vec<(String, Option<String>)> {
    let mut out: Vec<(String, Option<String>)> = Vec::new();
    for name in [&sec.deferred, &sec.icebox, &sec.done] {
        if !name.is_empty() && !name.contains(char::is_whitespace) {
            out.push((name.to_ascii_lowercase(), Some(name.clone())));
        }
    }
    out.push(("retired".to_string(), None));
    out
}

// spec: queue-kit/SPEC.md §check-queue-slug-liveness — `(<status>)` right after the closing
// backtick, one space allowed between, exactly one vocabulary word inside
fn status_after(line: &str, end: usize) -> Option<&str> {
    let rest = &line[end + 1..];
    let rest = rest.strip_prefix(' ').unwrap_or(rest);
    let inner = rest.strip_prefix('(')?;
    let close = inner.find(')')?;
    let word = &inner[..close];
    if word.is_empty() || !word.bytes().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    Some(word)
}

// spec: queue-kit/SPEC.md §check-queue-slug-liveness — the section each slug heads an entry in: a
// lead-line bullet anywhere, or a bare bullet in the done section
fn slug_sections(text: &str, sec: &queue::Sections) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut cur: Option<&str> = None;
    for line in text.lines() {
        if queue::is_section_line(line) {
            cur = queue::heading_name(line);
            continue;
        }
        let Some(name) = cur else { continue };
        let slug = match queue::bullet_slug(line) {
            Some(s) => Some(s),
            None if !sec.done.is_empty() && name == sec.done => queue::bare_bullet_slug(line),
            None => None,
        };
        if let Some(s) = slug {
            out.push((s.to_string(), name.to_string()));
        }
    }
    out
}

fn status_findings(
    text: &str,
    sec: &queue::Sections,
    retired: &dyn Fn() -> Vec<String>,
    qpath: &Path,
) -> Vec<String> {
    let vocab = status_vocabulary(sec);
    let mut cands: Vec<(usize, &str, &str)> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        for (s, e) in queue::backtick_slugs(line) {
            if let Some(word) = status_after(line, e) {
                if vocab.iter().any(|(w, _)| w == word) {
                    cands.push((i + 1, &line[s..e], word));
                }
            }
        }
    }
    if cands.is_empty() {
        return Vec::new();
    }
    let homes = slug_sections(text, sec);
    let retired = retired();
    let mut out = Vec::new();
    for (ln, slug, word) in cands {
        let home = homes.iter().find(|(s, _)| s == slug).map(|(_, n)| n.as_str());
        let is_retired = retired.iter().any(|r| r == slug);
        if home.is_none() && !is_retired {
            continue;
        }
        let agrees = match vocab.iter().find(|(w, _)| w == word) {
            Some((_, Some(section))) => home == Some(section.as_str()),
            _ => is_retired,
        };
        if !agrees {
            let actual = match home {
                Some(n) => n.to_string(),
                None => "retired".to_string(),
            };
            out.push(format!(
                "{}:{}:{} says ({}) but is {}",
                qpath.display(),
                ln,
                slug,
                word,
                actual
            ));
        }
    }
    out
}

fn surface_files(root: &Path, knob: &str) -> Result<Vec<PathBuf>, String> {
    let globs = queue::knob_array(knob)?;
    if globs.is_empty() {
        return Ok(Vec::new());
    }
    walk::glob_files(root, &globs).map_err(|e| {
        format!(
            "{} — the check could not run; treating as failure (not clean)",
            e
        )
    })
}

fn read(p: &Path) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| {
        format!(
            "cannot read {} ({}) — the check could not run; treating as failure (not clean)",
            p.display(),
            e
        )
    })
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-queue-slug-liveness: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    let queue_knob = queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?;
    let sec = queue::Sections::with_done()?;

    let scanroot = args.first().map(String::as_str).unwrap_or(".");
    let root = Path::new(scanroot);
    if !root.is_dir() {
        return Err(format!("not a directory: {}", scanroot));
    }
    let prose = surface_files(root, "QUEUE_KIT_PROSE_SURFACE_GLOBS")?;
    let cites = surface_files(root, "QUEUE_KIT_CITATION_SURFACE_GLOBS")?;

    // spec: queue-kit/SPEC.md §check-queue-slug-liveness — the queue is looked for at the
    // configured path first and under the scan root second, so a case dir and a repo root
    // both resolve it
    let mut qpath = PathBuf::from(&queue_knob);
    if !qpath.is_file() {
        qpath = root.join(&queue_knob);
    }
    if !qpath.is_file() {
        if prose.is_empty() && cites.is_empty() {
            println!("QUEUE-SLUG-LIVENESS: clean (no queue file and no surface configured — nothing to resolve)");
            return Ok(0);
        }
        return Err(format!("queue file not found: {}", queue_knob));
    }
    let qtext = read(&qpath)?;
    let live = queue::live_slugs(&qtext, &sec);
    let qfile = qpath.display().to_string();
    let cell: std::cell::OnceCell<Vec<String>> = std::cell::OnceCell::new();
    let retired = || cell.get_or_init(|| queue::retired_set(&qfile, &live)).clone();

    let mut dead: Vec<String> = Vec::new();
    for f in &prose {
        let text = read(f)?;
        for (i, line) in text.lines().enumerate() {
            for tok in bold_code_tokens(line) {
                if !live.iter().any(|s| s == tok) {
                    dead.push(format!("{}:{}:{}", f.display(), i + 1, tok));
                }
            }
        }
    }

    let mut stale: Vec<String> = Vec::new();
    if !cites.is_empty() {
        let ret = retired();
        if !ret.is_empty() {
            let stems = queue::tracked_stems(&qfile);
            for f in &cites {
                let text = read(f)?;
                let lines: Vec<&str> = text.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    for tok in single_backtick_slugs(line) {
                        if ret.iter().any(|r| r == tok)
                            && queue::name_live_at(&stems, tok).is_empty()
                            && !valved(&lines, i)
                        {
                            stale.push(format!("{}:{}:{}", f.display(), i + 1, tok));
                        }
                    }
                }
            }
        }
    }

    let status = status_findings(&qtext, &sec, &retired, &qpath);

    if !dead.is_empty() {
        println!("check-queue-slug-liveness: bold-code token claims queue membership but names no live task:");
        for x in &dead {
            println!("  {}", x);
        }
        println!("  help: a **`slug`** token claims the slug is a live queue task. If the task");
        println!("        landed, drop the bold-code form and cite its owning SPEC; otherwise fix");
        println!("        the slug or restore the task to the queue.");
    }
    if !stale.is_empty() {
        println!("check-queue-slug-liveness: governed prose cites a retired queue slug as a live pointer:");
        for x in &stale {
            println!("  {}", x);
        }
        println!("  help: name the owning SPEC section or the defect itself in place of the slug; a");
        println!("        line that is history takes '<!-- retired-citation-exempt: <reason> -->' on it");
        println!("        or the line above.");
    }
    if !status.is_empty() {
        println!("check-queue-slug-liveness: status parenthetical disagrees with where the cited slug is:");
        for x in &status {
            println!("  {}", x);
        }
        println!("  help: correct the parenthetical to the slug's current section, or drop it.");
    }
    if !dead.is_empty() || !stale.is_empty() || !status.is_empty() {
        return Ok(1);
    }

    println!(
        "QUEUE-SLUG-LIVENESS: clean ({} prose surface(s), {} citation surface(s) scanned; every bold-code token resolves to a live task, no retired slug is cited and every status parenthetical agrees in {})",
        prose.len(),
        cites.len(),
        qpath.display()
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sec() -> queue::Sections {
        queue::Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: "Icebox".into(),
            done: "Done".into(),
        }
    }

    #[test]
    fn a_bold_code_token_is_lowercase_kebab_inside_backticks_and_bold() {
        assert_eq!(bold_code_tokens("see **`a-b`** here"), vec!["a-b"]);
        assert_eq!(bold_code_tokens("**`x`** and **`y`**"), vec!["x", "y"]);
        assert!(bold_code_tokens("**bare-bold**").is_empty());
        assert!(bold_code_tokens("`just-code`").is_empty());
        assert!(bold_code_tokens("**`Upper`**").is_empty());
    }

    #[test]
    fn a_retired_citation_token_is_single_backtick() {
        assert_eq!(single_backtick_slugs("see `a-b` and ``c-d``"), vec!["a-b"]);
        assert_eq!(single_backtick_slugs("**`x`**"), vec!["x"]);
    }

    #[test]
    fn the_valve_needs_a_reason_on_the_line_or_the_one_above() {
        let lines = vec!["<!-- retired-citation-exempt: history -->", "`x`", "`y`"];
        assert!(valved(&lines, 1));
        assert!(!valved(&lines, 2));
        let bare = vec!["`x` <!-- retired-citation-exempt: -->"];
        assert!(!valved(&bare, 0));
    }

    #[test]
    fn a_status_parenthetical_agrees_with_the_slug_section() {
        let q = "## New Features\n\n- **a-live** — x\n\n## Deferred\n\n- **b-def** — cites `c-ice` (icebox), `b-def` (deferred), `a-live` (deferred), `d-done` (retired), `gone` (retired), `x-none` (done), `d-done` (landed 2026)\n\n## Icebox\n\n- **c-ice** — y\n\n## Done\n\n- d-done\n";
        let ret = || vec!["d-done".to_string(), "gone".to_string()];
        let out = status_findings(q, &sec(), &ret, Path::new("Q.md"));
        assert_eq!(out, vec!["Q.md:7:a-live says (deferred) but is New Features".to_string()]);
        let q2 = "## Deferred\n\n- **b-def** — `d-done` (icebox) and `gone` (done)\n\n## Done\n\n- d-done\n";
        let out = status_findings(q2, &sec(), &ret, Path::new("Q.md"));
        assert_eq!(
            out,
            vec![
                "Q.md:3:d-done says (icebox) but is Done".to_string(),
                "Q.md:3:gone says (done) but is retired".to_string(),
            ]
        );
    }

    #[test]
    fn a_multi_word_section_is_outside_the_vocabulary() {
        let mut s = sec();
        s.icebox = "Cold Storage".into();
        let words: Vec<String> = status_vocabulary(&s).into_iter().map(|(w, _)| w).collect();
        assert_eq!(words, vec!["deferred", "done", "retired"]);
    }
}
