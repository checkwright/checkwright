// spec: lifecycle-kit/SPEC.md §The committed gap inbox — the capture affordance: one dated bullet
// per gap, the grammar stamped by the producer rather than by its filer.
// spec: gate-sdk/SPEC.md §The non-gate arm — a table member and not a hardcoded flag, because the
// arm resolves five consumer knobs and a hardcoded flag receives no override at all.
use crate::stages;

pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_GAP_INBOX_FILE",
    "LIFECYCLE_KIT_QUEUE_FILE",
    "LIFECYCLE_KIT_STATE_FILE",
    "LIFECYCLE_KIT_STAGES",
    "LIFECYCLE_KIT_FIRST_STAGE",
];

const USAGE: &str = "usage: --emit file-gap [--] \"<gap prose>\"\n  appends one dated bullet to the committed gap inbox; \"--\" files prose beginning with \"-\"";

// spec: lifecycle-kit/SPEC.md §The committed gap inbox — the contract header seeded on a fresh
// consumer's first filing, byte-identical to the line close's drain truncates back to.
const CONTRACT_HEADER: &str = "# contract: lifecycle-kit/SPEC.md §The committed gap inbox — append-only mid-iteration gap capture, close-drained; one bullet per gap below.\n";

// spec: lifecycle-kit/SPEC.md §The committed gap inbox — the fixed-spelling section the live-slug
// scan excludes by name: a lesson may legitimately be written in the entry shape, so a grammar-only
// scan would ask the filer about something that is not a queue entry.
const LESSONS_HEADING: &str = "## Lessons Learned";

// spec: lifecycle-kit/SPEC.md §The committed gap inbox — one bullet shape for every filing,
// matching or not: the tool records the observation and never interposes a verdict.
fn bullet(today: &str, prose: &str) -> String {
    format!("- {} — {}", today, prose)
}

// spec: lifecycle-kit/SPEC.md §The committed gap inbox — the word boundary is `[a-z0-9-]` in both
// directions, so a slug embedded in a longer hyphenated token raises nothing.
fn boundary(c: Option<char>) -> bool {
    match c {
        None => true,
        Some(c) => !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
    }
}

fn bounded(hay: &str, needle: &str) -> bool {
    let mut from = 0;
    while let Some(rel) = hay[from..].find(needle) {
        let at = from + rel;
        let before = hay[..at].chars().next_back();
        let after = hay[at + needle.len()..].chars().next();
        if boundary(before) && boundary(after) {
            return true;
        }
        from = at + 1;
    }
    false
}

// spec: lifecycle-kit/SPEC.md §The committed gap inbox — the live set is every column-0
// `- **<slug>** —` entry bullet outside the Lessons section; the done section falls out by grammar
// because a done entry is a bare-slug line, and an indented sub-task by the column-0 anchor.
fn entry_slug(line: &str) -> Option<&str> {
    let rest = line.strip_prefix('-')?;
    if !rest.starts_with([' ', '\t']) {
        return None;
    }
    let body = rest.trim_start_matches([' ', '\t']).strip_prefix("**")?;
    let end = body.find("**")?;
    let slug = &body[..end];
    let mut chars = slug.chars();
    let first = chars.next()?;
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return None;
    }
    if !chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return None;
    }
    match body[end + 2..].chars().next() {
        Some(' ') | Some('\t') => Some(slug),
        _ => None,
    }
}

// spec: lifecycle-kit/SPEC.md §The committed gap inbox — lifecycle-kit's own predicate, reproduced
// rather than collapsed onto `queue::live_slugs`: grammar-scoped against knob-scoped is a different
// corpus, so the collapse would be a verdict change on a real consumer.
// spec: lifecycle-kit/SPEC.md §The committed gap inbox — longest match wins.
pub fn live_slug(queue_text: &str, prose: &str) -> Option<String> {
    let mut live: Vec<&str> = Vec::new();
    let mut in_lessons = false;
    for line in queue_text.lines() {
        if line.trim_end_matches([' ', '\t']) == LESSONS_HEADING {
            in_lessons = true;
            continue;
        }
        if line.starts_with("## ") {
            in_lessons = false;
        }
        if in_lessons {
            continue;
        }
        if let Some(s) = entry_slug(line) {
            live.push(s);
        }
    }
    let hay = prose.to_lowercase();
    let mut best: Option<&str> = None;
    for s in live {
        if s.len() > best.map_or(0, str::len) && bounded(&hay, s) {
            best = Some(s);
        }
    }
    best.map(str::to_string)
}

// spec: lifecycle-kit/SPEC.md §lib/stages.sh — the closing-stage predicate, composed here from the
// cursor and the last configured stage; the hoisting's by-construction guarantee holds inside each
// substrate and not across them until `--enter-stage` ports.
fn cursor() -> Result<(String, bool), String> {
    let (state, _) = super::file_survey::anchored("LIFECYCLE_KIT_STATE_FILE")?;
    let text = std::fs::read(state)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    let stage = stages::current_stage(&text);
    let last = stages::stages()?.last().cloned().unwrap_or_default();
    let closing = !last.is_empty() && stage == last;
    Ok((stage, closing))
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let fields = super::file_survey::positionals(args, "prose")?;
    if fields.len() != 1 || fields[0].is_empty() {
        return Err(USAGE.to_string());
    }
    let prose = &fields[0];

    let (inbox, spelled) = super::file_survey::anchored("LIFECYCLE_KIT_GAP_INBOX_FILE")?;
    let path = std::path::Path::new(&inbox);
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if !path.is_file() {
        std::fs::write(path, CONTRACT_HEADER)
            .map_err(|e| format!("cannot seed {}: {}", spelled, e))?;
    }

    let (queue, _) = super::file_survey::anchored("LIFECYCLE_KIT_QUEUE_FILE")?;
    let slug = std::fs::read(queue)
        .ok()
        .and_then(|b| live_slug(&String::from_utf8_lossy(&b), prose));

    let (stage, closing) = cursor()?;
    let first = crate::walk::knob_scalar("LIFECYCLE_KIT_FIRST_STAGE")?;

    let line = bullet(&super::kpi::today_iso(), prose);
    super::file_survey::append(path, &format!("{}\n", line))
        .map_err(|e| format!("cannot append to {}: {}", spelled, e))?;

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — the advisory asks rather than asserts,
    // and rides stderr because the returned string is the filed bullet.
    if let Some(s) = slug {
        eprintln!(
            "file-gap: this prose names live entry `{}`. If this bullet RE-FILES that finding, say \
             so in the prose and say why; if it merely cites, corrects, or argues against that \
             entry, say it is DISTINCT and why. The closing stage's drain judges the recurrence and \
             reads what you wrote — it has nothing else to go on.",
            s
        );
    }
    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — warn at the point of capture, while
    // the filer can still act: after the iteration's last stage stamps there is no drainer left in
    // the machine.
    if closing {
        eprintln!(
            "file-gap: WARNING — the cursor is at {}, the last stage of the iteration. Disposition \
             this bullet before the iteration ends: once that stage has finished, none is left to \
             drain it, and the next {} entry carries it into that session's own intake instead.",
            stage, first
        );
    } else {
        eprintln!(
            "file-gap: this bullet blocks the next {} entry until close drains it.",
            first
        );
    }
    Ok(format!("file-gap: {}\n", line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::file_survey;

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — the sandboxed queue the re-homed
    // end-to-end suite drives the arm against, held here so the grammar cases and the
    // no-queue-write invariant read one corpus rather than two that could drift.
    const QUEUE: &str = r#"# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

- **fork-dispatch** — an active entry whose slug is a hyphen-prefix of a deferred one.

## Technical Debt

## Deferred

- **fork-dispatch-prohibition** [design-pending] — the deferred entry.
  Cost while deferred: recovery is re-paid each time.
  - **nested-subtask** — a sub-task, indented, deliberately out of the entry scan.

## Icebox

- **iced-entry** — one line, still live work.

## Done

- landed-and-fixed

## Lessons Learned

- **lesson-shaped-slug** [attend] — a lesson written in the entry shape.
"#;

    fn argv(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — longest match wins, and an icebox
    // entry is live: both are the reproduced predicate's own behaviour and neither is
    // `queue::live_slugs`'.
    #[test]
    fn a_live_slug_resolves_and_the_longest_match_wins() {
        assert_eq!(
            live_slug(
                QUEUE,
                "the fork-dispatch-prohibition failure mode fired again inside scope"
            )
            .as_deref(),
            Some("fork-dispatch-prohibition")
        );
        assert_eq!(
            live_slug(QUEUE, "iced-entry regressed").as_deref(),
            Some("iced-entry")
        );
    }

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — the four exclusions: done by grammar,
    // Lessons by name, a sub-task by the column-0 anchor, and a hyphen-embedded near-miss by the
    // `[a-z0-9-]` word boundary.
    #[test]
    fn done_lessons_subtask_and_a_hyphen_embedded_near_miss_all_stay_silent() {
        for prose in [
            "landed-and-fixed broke again after its fix landed",
            "lesson-shaped-slug came up once more",
            "nested-subtask surfaced again",
            "the fork-dispatching helper is slow",
        ] {
            assert_eq!(
                live_slug(QUEUE, prose),
                None,
                "the matcher spoke on: {}",
                prose
            );
        }
    }

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — asking is not asserting: a prose that
    // denies the recurrence in words is still asked about, because no syntactic tell separates
    // "this recurred" from "this is about that".
    #[test]
    fn a_denying_prose_is_still_asked_about() {
        assert_eq!(
            live_slug(
                QUEUE,
                "the harness grows unbounded and nothing prunes it. NOT a recurrence of \
                 `fork-dispatch-prohibition` — the filer ruled this must be a separate entry."
            )
            .as_deref(),
            Some("fork-dispatch-prohibition")
        );
    }

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — one bullet shape for every filing,
    // with no structured slot between the date and the prose and no verdict interposed.
    #[test]
    fn every_filing_takes_the_one_plain_bullet_shape() {
        assert_eq!(bullet("2026-01-01", "a gap"), "- 2026-01-01 — a gap");
        assert!(!bullet("2026-01-01", "a gap").contains("recurrence of"));
    }

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the hazard belongs to the argument rather
    // than to the substrate, and one slot makes arity no protection at all: `--help` is accepted
    // *as* the positional by an arity check, which is the attested capture this refusal exists for.
    #[test]
    fn a_flag_is_refused_a_separator_files_it_and_help_is_not_a_capture() {
        let err = file_survey::positionals(&argv(&["--list"]), "prose")
            .expect_err("a leading-dash prose was captured");
        assert!(
            err.contains("--list"),
            "the refusal named no offender: {}",
            err
        );
        let sep = argv(&["--", "--list is captured at exit 0"]);
        assert_eq!(
            file_survey::positionals(&sep, "prose")
                .expect("the separator did not end option processing"),
            &sep[1..]
        );
        for flag in ["-h", "--help"] {
            assert!(
                file_survey::positionals(&argv(&[flag]), "prose").is_err(),
                "{} was taken as prose",
                flag
            );
        }
    }

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — one positional, required non-empty;
    // arity misuse is a refusal rather than an empty bullet appended to a committed surface.
    #[test]
    fn arity_misuse_is_a_refusal() {
        assert!(emit(&argv(&[])).is_err());
        assert!(emit(&argv(&[""])).is_err());
        assert!(emit(&argv(&["one", "two"])).is_err());
    }
}
