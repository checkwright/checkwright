// spec: canon-kit/SPEC.md §check-amendment-retired-spelling — every amendment carries a
// `## Retired spellings` block, and every spelling it declares survives only at a path its
// `## Existing sections updated` roster names
use crate::proc;
use crate::spec;
use crate::walk;
use std::path::Path;

const EXEMPT: &str = "retired-spelling-exempt:";

// spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the three heading names are kit
// constants on §check-amendment-update-target's own ground, not config
const WHAT_CHANGES: &str = "## What changes";
const UPDATED: &str = "## Existing sections updated";
const RETIRED: &str = "## Retired spellings";

const EXCLUDE_KNOB: &str = "CANON_KIT_RETIRED_SPELLING_EXCLUDE";

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-amendment-retired-spelling: {}", e);
            2
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Sec {
    What,
    Updated,
    Retired,
    Other,
}

struct Bullet {
    line: usize,
    exempt: bool,
    flat: spec::FlatPara,
}

struct Scan {
    has_what: bool,
    has_retired: bool,
    deltas: Vec<usize>,
    roster_paths: Vec<String>,
    bullets: Vec<Bullet>,
}

// spec: canon-kit/SPEC.md §check-amendment-retired-spelling — one pass over the amendment
// collecting all three sections' data: the delta set arm C reconciles against, the roster paths
// arm B reconciles against, and the block's own bullets arm A judges
fn scan(text: &str) -> Scan {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Scan {
        has_what: false,
        has_retired: false,
        deltas: Vec::new(),
        roster_paths: Vec::new(),
        bullets: Vec::new(),
    };
    let mut fence = false;
    let mut sec = Sec::Other;
    let mut i = 0usize;
    while i < lines.len() {
        let raw = lines[i];
        // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — a fenced block is skipped
        // whole, on §check-amendment-update-target's ground: an embedded wire-contract delta is
        // grammar being shown rather than a bullet being declared
        if spec::is_fence_line(raw) {
            fence = !fence;
            i += 1;
            continue;
        }
        if fence {
            i += 1;
            continue;
        }
        if raw.starts_with("## ") {
            let name = raw.trim_end();
            sec = if name == WHAT_CHANGES {
                out.has_what = true;
                Sec::What
            } else if name == UPDATED {
                Sec::Updated
            } else if name == RETIRED {
                out.has_retired = true;
                Sec::Retired
            } else {
                Sec::Other
            };
            i += 1;
            continue;
        }
        match sec {
            Sec::What => {
                if raw.starts_with("### ") {
                    if let Some(n) = spec::delta_number(raw) {
                        out.deltas.push(n);
                    }
                }
                i += 1;
            }
            Sec::Updated => {
                if raw.starts_with("- ") {
                    if let Some(p) = spec::leading_backticked(raw) {
                        out.roster_paths.push(p);
                    }
                }
                i += 1;
            }
            Sec::Retired => {
                if !raw.starts_with("- ") {
                    i += 1;
                    continue;
                }
                // spec: canon-kit/SPEC.md §lib/spec.sh — the shared exempt window, the line or
                // the one above
                let exempt = raw.contains(EXEMPT) || (i > 0 && lines[i - 1].contains(EXEMPT));
                let mut para = spec::Para::default();
                para.add(i + 1, raw);
                let mut j = i + 1;
                // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the bullet window
                // is the bullet plus its indented continuation, the wrap-straddling boundary arm
                // B of §check-amendment-update-target crosses and for the same reason
                while j < lines.len() {
                    let l = lines[j];
                    if spec::is_blank(l) {
                        j += 1;
                        continue;
                    }
                    if spec::is_fence_line(l) || !(l.starts_with(' ') || l.starts_with('\t')) {
                        break;
                    }
                    para.add(j + 1, l);
                    j += 1;
                }
                out.bullets.push(Bullet {
                    line: i + 1,
                    exempt,
                    flat: spec::flatten_para(&para),
                });
                i = j;
            }
            Sec::Other => i += 1,
        }
    }
    out
}

// spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the negative form, recognised
// before its reason is judged so a missing reason is reported as itself
fn is_negative(flat: &str) -> bool {
    let body = flat.strip_prefix("- ").unwrap_or(flat);
    let first = body.split_whitespace().next().unwrap_or("");
    first.trim_end_matches(|c: char| !c.is_ascii_alphanumeric())
        .eq_ignore_ascii_case("none")
}

// spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the negative form's reason: an em
// dash and non-empty text after it. An empty reason is the form discharged without being
// answered, which is the one thing the negative form exists to make an author write.
fn negative_reason(flat: &str) -> bool {
    match flat.split_once('—') {
        Some((_, rest)) => !rest.trim().is_empty(),
        None => false,
    }
}

fn lead(text: &str) -> String {
    let mut s: String = text.chars().take(72).collect();
    if text.chars().count() > 72 {
        s.push('…');
    }
    s
}

struct Declared {
    file: String,
    line: usize,
    spelling: String,
}

// spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the reconciliation corpus:
// `git ls-files` minus the amendment set, minus the configured exclusion
fn corpus(root: &str, amendments: &[String]) -> Result<Vec<String>, String> {
    let probe = proc::run("git", &["rev-parse", "--git-dir"])?;
    if probe.stdout().is_none() {
        return Err("not a git repository — cannot enumerate the reconciliation corpus".into());
    }
    let ls = proc::run("git", &["ls-files", "--", root])?;
    let listing = match ls.stdout() {
        Some(o) => String::from_utf8_lossy(o).into_owned(),
        None => {
            return Err(format!(
                "git ls-files exited {} — the reconciliation corpus could not be enumerated",
                ls.code().unwrap_or(-1)
            ))
        }
    };
    let exclude = spec::knob_array_pub(EXCLUDE_KNOB)?;
    Ok(listing
        .lines()
        .filter(|p| !p.is_empty())
        .filter(|p| !amendments.iter().any(|a| a == p))
        .filter(|p| !exclude.iter().any(|g| walk::pattern_match(g, p)))
        .map(String::from)
        .collect())
}

fn rule(args: &[String]) -> Result<i32, String> {
    let root = args.first().map(String::as_str).unwrap_or(".");
    if !Path::new(root).is_dir() {
        return Err(format!("not a directory: {}", root));
    }
    // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the strict finder, following
    // §check-amendment-update-target and not §check-amendment-queue: an empty amendment set
    // hides every violation here, where the queue gate has a second direction contradicting it
    let files: Vec<String> = spec::amendments_strict(root)?
        .into_iter()
        .map(|p| spec::strip_dot_slash(&p.display().to_string()))
        .collect();

    let mut missing: Vec<String> = Vec::new();
    let mut malformed: Vec<String> = Vec::new();
    let mut dangling: Vec<String> = Vec::new();
    let mut declared: Vec<Declared> = Vec::new();
    let mut negatives = 0usize;

    for f in &files {
        let text = spec::read_text(Path::new(f))?;
        let s = scan(&text);
        if s.has_retired && !s.has_what {
            return Err(format!(
                "{}: carries '{}' but no '{}' — no bullet can cite a delta here and no arm could \
                 say which to blame",
                f, RETIRED, WHAT_CHANGES
            ));
        }
        if !s.has_what {
            continue;
        }
        if !s.has_retired {
            missing.push(format!("  {}: no '{}' section", f, RETIRED));
            continue;
        }
        if s.bullets.is_empty() {
            malformed.push(format!(
                "  {}: '{}' carries no bullet — the section is mandatory and so is its body",
                f, RETIRED
            ));
            continue;
        }
        let live: Vec<&Bullet> = s.bullets.iter().filter(|b| !b.exempt).collect();
        let negative: Vec<&&Bullet> = live.iter().filter(|b| is_negative(&b.flat.text)).collect();
        if !negative.is_empty() {
            if live.len() > 1 {
                malformed.push(format!(
                    "  {}:{}: the negative form is a single bullet, and this section carries {}",
                    f,
                    live[0].line,
                    live.len()
                ));
                continue;
            }
            if !negative_reason(&live[0].flat.text) {
                malformed.push(format!(
                    "  {}:{}: 'None' with no em-dashed reason: {}",
                    f,
                    live[0].line,
                    lead(&live[0].flat.text)
                ));
                continue;
            }
            negatives += 1;
            continue;
        }
        for b in &live {
            let spelling = spec::leading_backticked(&b.flat.text);
            let (nums, all) = spec::citations(&b.flat.text);
            if spelling.is_none() || (nums.is_empty() && all.is_none()) {
                malformed.push(format!(
                    "  {}:{}: {}",
                    f,
                    b.line,
                    lead(&b.flat.text)
                ));
                continue;
            }
            if all.is_some() && s.deltas.is_empty() {
                dangling.push(format!(
                    "  {}:{}: 'all deltas', but the amendment defines none",
                    f, b.line
                ));
            }
            let mut seen: Vec<usize> = Vec::new();
            for (n, _) in &nums {
                if s.deltas.contains(n) || seen.contains(n) {
                    continue;
                }
                seen.push(*n);
                dangling.push(format!(
                    "  {}:{}: cites delta {}, which no '### ({})' heading defines",
                    f, b.line, n, n
                ));
            }
            if let Some(spelling) = spelling {
                declared.push(Declared {
                    file: f.clone(),
                    line: b.line,
                    spelling,
                });
            }
        }
    }

    // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the roster is read per
    // amendment and the reconciliation is per declaration, so a spelling declared in one
    // amendment is never excused by a path another amendment's roster names
    let mut rosters: Vec<(String, Vec<String>)> = Vec::new();
    let mut survivors: Vec<String> = Vec::new();
    if !declared.is_empty() {
        for f in &files {
            let text = spec::read_text(Path::new(f))?;
            rosters.push((f.clone(), scan(&text).roster_paths));
        }
        let paths = corpus(root, &files)?;
        for path in &paths {
            let p = Path::new(path);
            if !p.is_file() {
                continue;
            }
            let text = spec::read_text(p)?;
            for d in &declared {
                let named = rosters
                    .iter()
                    .find(|(f, _)| f == &d.file)
                    .map(|(_, r)| r.iter().any(|n| n == path))
                    .unwrap_or(false);
                if named {
                    continue;
                }
                // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the finding's
                // subject is a surface, so one line per surface and not one per occurrence
                if let Some((n, _)) = text
                    .lines()
                    .enumerate()
                    .find(|(_, l)| l.contains(&d.spelling))
                {
                    survivors.push(format!(
                        "  {}:{}: {} survives at {}:{}, named by no roster bullet",
                        d.file,
                        d.line,
                        d.spelling,
                        path,
                        n + 1
                    ));
                }
            }
        }
    }

    let mut errors = String::new();
    let mut block = |head: &str, items: &[String]| {
        if items.is_empty() {
            return;
        }
        errors.push_str(head);
        errors.push('\n');
        for it in items {
            errors.push_str(it);
            errors.push('\n');
        }
    };
    block(
        "amendments carrying no retired-spelling block (arm A — the section is mandatory, which is what gives it the negative form the roster it repairs does not have):",
        &missing,
    );
    block(
        "block bodies matching neither form (arm A — one `None — <reason>` bullet, or bullets each carrying a backticked spelling and a delta citation):",
        &malformed,
    );
    block(
        "declared spellings surviving where no roster bullet names the path (arm B — the Definition of Done's own grep, re-executed):",
        &survivors,
    );
    block(
        "citations naming a delta the amendment does not define (arm C — renumbered out from under the bullet):",
        &dangling,
    );

    if !errors.is_empty() {
        println!("check-amendment-retired-spelling: an amendment declaring no retired spelling, or a retired spelling surviving at a surface its roster never named:");
        println!();
        print!("{}", errors);
        println!("  help: give every amendment a '{}' section — '- None — <reason>' where no delta retires a spelling, otherwise one bullet per spelling opening with the backticked spelling and citing the delta that retired it ('delta 3', 'deltas 2 and 4', 'all deltas'). For a survivor that is a deliberate mention rather than a missed site, either name its path in a '{}' bullet or tag '<!-- retired-spelling-exempt: <reason> -->' on the bullet's first line or the one above (a reason is mandatory); '{}' holds a consumer's history-bearing surfaces out of the corpus.", RETIRED, UPDATED, EXCLUDE_KNOB);
        return Ok(1);
    }
    println!(
        "AMENDMENT-RETIRED-SPELLING: clean ({} amendment(s), {} spelling(s) declared, {} taking the negative form)",
        files.len(),
        declared.len(),
        negatives
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the negative form is a first
    // word, so it survives the punctuation an author reaches for and does not fire on a bullet
    // that merely opens with a word beginning `none`
    #[test]
    fn the_negative_form_is_the_word_none_and_a_reason() {
        assert!(is_negative("- None — no delta retires a spelling."));
        assert!(is_negative("- none — lower case is the same word."));
        assert!(is_negative("- None, — trailing punctuation is not part of the word."));
        assert!(!is_negative("- `nonexistent` — a spelling that merely opens with it."));
        assert!(!is_negative("- Nothing — a synonym is not the form."));
        assert!(negative_reason("- None — a reason."));
        assert!(!negative_reason("- None —"));
        assert!(!negative_reason("- None, no em dash at all."));
    }

    // spec: canon-kit/SPEC.md §check-amendment-retired-spelling — the block's bullets, its
    // exempt window, and the roster paths and delta set the other two arms reconcile against,
    // all read in one pass with a fence skipped whole
    #[test]
    fn one_pass_reads_the_block_the_roster_and_the_delta_set() {
        let text = "## What changes\n\n### (1) one\n\n```\n### (9) fenced\n- `fenced/spelling` (delta 9)\n```\n\n## Existing sections updated\n\n- `a/named.md` — prose (delta 1).\n- **bold/unnamed.md** — names no path.\n\n## Retired spellings\n\n<!-- retired-spelling-exempt: r -->\n- `valved` — exempt (delta 1).\n- `live/spelling` — retired by\n  delta 1.\n\n## Definition of Done\n";
        let s = scan(text);
        assert!(s.has_what && s.has_retired);
        assert_eq!(s.deltas, vec![1]);
        assert_eq!(s.roster_paths, vec!["a/named.md".to_string()]);
        assert_eq!(s.bullets.len(), 2);
        assert!(s.bullets[0].exempt);
        assert!(!s.bullets[1].exempt);
        assert_eq!(
            spec::leading_backticked(&s.bullets[1].flat.text),
            Some("live/spelling".to_string())
        );
        assert_eq!(spec::citations(&s.bullets[1].flat.text).0.len(), 1);
    }
}
