// spec: queue-kit/SPEC.md §The queue-index arm — the compact queue surface for task selection.
// Three modes on one arm, selected from the arm's own argv tail rather than from three arms:
// the emitter type is defined over an argv slice precisely so a mode rides as a flag.
use crate::proc;
use crate::queue::{self, Sections};

const TITLE_CAP: usize = 64;
const OPENER_CAP: usize = 48;

const USAGE: &str = "\
usage: --emit queue-index [--collapse-deferred] [--extent <slug>] [--icebox-candidates] [queue-file]
  default: header + active (• ready / ✗ blocked) + deferred titles + icebox tally;
  --collapse-deferred: per-### tally; --extent <slug>: \"<start> <end>\"; --icebox-candidates: eviction worklist
";

enum Mode {
    Index,
    Extent,
    Candidates,
    Help,
}

struct Args {
    mode: Mode,
    collapse: bool,
    slug: String,
    file: String,
}

// spec: queue-kit/SPEC.md §The queue-index arm — the shell tool's own option loop: a mode and the
// collapse flag are set independently, so `--collapse-deferred` alongside a non-index mode is
// accepted and ignored exactly as it was.
fn parse(args: &[String]) -> Result<Args, String> {
    let mut a = Args {
        mode: Mode::Index,
        collapse: false,
        slug: String::new(),
        file: String::new(),
    };
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--collapse-deferred" => {
                a.collapse = true;
                i += 1;
            }
            "--extent" => {
                a.mode = Mode::Extent;
                a.slug = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--icebox-candidates" => {
                a.mode = Mode::Candidates;
                i += 1;
            }
            "-h" | "--help" => {
                a.mode = Mode::Help;
                i += 1;
            }
            other if other.starts_with('-') => {
                return Err(format!("unknown option: {}", other));
            }
            other => {
                a.file = other.to_string();
                i += 1;
            }
        }
    }
    Ok(a)
}

pub fn emit(args: &[String]) -> Result<String, String> {
    let a = parse(args)?;
    if let Mode::Help = a.mode {
        return Ok(USAGE.to_string());
    }
    let file = if a.file.is_empty() {
        queue::knob_scalar("QUEUE_KIT_QUEUE_FILE")?
    } else {
        a.file.clone()
    };
    let text = std::fs::read_to_string(&file).map_err(|e| format!("file not found: {}: {}", file, e))?;

    match a.mode {
        Mode::Help => unreachable!(),
        Mode::Extent => extent(&text, &a.slug),
        Mode::Candidates => candidates(&text),
        Mode::Index => index(&text, a.collapse),
    }
}

fn truncate_chars(s: &str, cap: usize) -> String {
    if s.chars().count() > cap {
        let head: String = s.chars().take(cap - 1).collect();
        format!("{}…", head)
    } else {
        s.to_string()
    }
}

// spec: queue-kit/SPEC.md §The queue-index arm — awk's `/^-[[:space:]]/`, the column-0 bullet the
// section scanners key on: an indented sub-task is deliberately outside it.
fn is_top_level_bullet(line: &str) -> bool {
    let b = line.as_bytes();
    matches!(b.first(), Some(&c) if c == b'-')
        && matches!(b.get(1), Some(&c) if c == b' ' || c == b'\t')
}

// spec: queue-kit/SPEC.md §The queue-index arm — awk's `gsub(/\[[^]]*\]/, "", t)`: every bracketed
// tag comes off before the slug-and-dash strip, so the dash is adjacent when that strip runs.
fn remove_bracketed(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(open) = rest.find('[') {
        match rest[open + 1..].find(']') {
            Some(rel) => {
                out.push_str(&rest[..open]);
                rest = &rest[open + 1 + rel + 1..];
            }
            None => break,
        }
    }
    out.push_str(rest);
    out
}

// spec: queue-kit/SPEC.md §The queue-index arm — awk's
// `^\*\*[a-z0-9][a-z0-9-]*\*\*[[:space:]]*(—[[:space:]]*)?`: the em-dash separator goes with the
// slug, so a lead line that is all tag renders as the bare slug and not an orphaned separator.
fn strip_slug_and_dash(s: &str) -> String {
    let b = s.as_bytes();
    if b.len() < 5 || b[0] != b'*' || b[1] != b'*' {
        return s.to_string();
    }
    let head = b[2];
    if !(head.is_ascii_lowercase() || head.is_ascii_digit()) {
        return s.to_string();
    }
    let mut j = 3usize;
    while j < b.len() && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-') {
        j += 1;
    }
    if !(j + 1 < b.len() && b[j] == b'*' && b[j + 1] == b'*') {
        return s.to_string();
    }
    let mut rest = &s[j + 2..];
    rest = rest.trim_start_matches([' ', '\t']);
    if let Some(r) = rest.strip_prefix('—') {
        rest = r.trim_start_matches([' ', '\t']);
    }
    rest.to_string()
}

fn title(line: &str) -> String {
    let t = queue::strip_bullet_lead(line).unwrap_or(line);
    let t = remove_bracketed(t);
    let t = strip_slug_and_dash(&t);
    truncate_chars(t.trim_matches([' ', '\t']), TITLE_CAP)
}

fn joined(slug: &str, t: &str) -> String {
    if t.is_empty() {
        slug.to_string()
    } else {
        format!("{} — {}", slug, t)
    }
}

// spec: queue-kit/SPEC.md §The queue-index arm — `\[drain-exempt:[[:space:]]*[^]]+\]`, re-echoed on
// the active line because the tag's reason is what a reader needs and the tag strip removed it.
fn drain_exempt(line: &str) -> Option<String> {
    let open = line.find("[drain-exempt:")?;
    let after = &line[open + "[drain-exempt:".len()..];
    let close = after.find(']')?;
    let body = after[..close].trim_start_matches([' ', '\t']);
    if body.is_empty() {
        return None;
    }
    Some(body.trim_end_matches([' ', '\t']).to_string())
}

// spec: queue-kit/SPEC.md §The queue-index arm — every `[blocked-by: <slug>]` on the lead line, in
// order; the presence of any one is what flips the row's ready mark.
fn blockers(line: &str) -> String {
    let mut found: Vec<&str> = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find("[blocked-by:") {
        let after = &rest[open + "[blocked-by:".len()..];
        let body = after.trim_start_matches([' ', '\t']);
        let consumed = after.len() - body.len();
        let b = body.as_bytes();
        let mut j = 0usize;
        if !b.is_empty() && (b[0].is_ascii_lowercase() || b[0].is_ascii_digit()) {
            j = 1;
            while j < b.len() && (b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-')
            {
                j += 1;
            }
            found.push(&body[..j]);
        }
        rest = &after[consumed + j..];
    }
    found.join(", ")
}

#[derive(PartialEq)]
enum Sec {
    None,
    Active,
    Deferred,
    Icebox,
    Lessons,
    Other,
}

fn index(text: &str, collapse: bool) -> Result<String, String> {
    let sec_cfg = Sections::active_and_deferred()?;
    let cap: usize = queue::knob_scalar("QUEUE_KIT_ATTEND_CAP")?
        .trim()
        .parse()
        .map_err(|_| "QUEUE_KIT_ATTEND_CAP is not a positive integer".to_string())?;

    let mut out = String::new();
    if let Some(h) = text.lines().find(|l| l.starts_with("## Iteration:")) {
        out.push_str(h);
        out.push('\n');
        out.push('\n');
    }

    let mut sec = Sec::None;
    let mut cur_sub = String::new();
    let mut active: Vec<(char, String)> = Vec::new();
    let mut deferred: Vec<String> = Vec::new();
    let mut tally_order: Vec<String> = Vec::new();
    let mut tally: Vec<(String, usize)> = Vec::new();
    let mut icebox_n = 0usize;
    let mut attend_n = 0usize;
    let mut attend: Vec<String> = Vec::new();

    for line in text.lines() {
        if sec_cfg.is_deferred(line) {
            sec = Sec::Deferred;
            continue;
        }
        if let Some(name) = queue::heading_name(line) {
            if sec_cfg.active.iter().any(|a| a == name) {
                sec = Sec::Active;
                continue;
            }
        }
        if sec_cfg.is_icebox(line) {
            sec = Sec::Icebox;
            continue;
        }
        if queue::is_lessons_line(line) {
            sec = Sec::Lessons;
            continue;
        }
        if queue::is_section_line(line) {
            sec = Sec::Other;
            continue;
        }

        match sec {
            Sec::Icebox => {
                if is_top_level_bullet(line) && queue::first_bold_slug(line).is_some() {
                    icebox_n += 1;
                }
            }
            Sec::Lessons => {
                if is_top_level_bullet(line) && line.contains("[attend]") {
                    attend_n += 1;
                    if attend_n <= cap {
                        attend.push(line.trim_end_matches([' ', '\t']).to_string());
                    }
                }
            }
            Sec::Active => {
                if is_top_level_bullet(line) {
                    if let Some(slug) = queue::first_bold_slug(line) {
                        let bl = blockers(line);
                        let de = drain_exempt(line);
                        let mark = if bl.is_empty() { '•' } else { '✗' };
                        let mut row = joined(slug, &title(line));
                        if !bl.is_empty() {
                            row.push_str(&format!("   [blocked-by: {}]", bl));
                        }
                        if let Some(d) = de {
                            row.push_str(&format!("   [drain-exempt: {}]", d));
                        }
                        active.push((mark, row));
                    }
                }
            }
            Sec::Deferred => {
                if let Some(rest) = line.strip_prefix("### ") {
                    cur_sub = rest.trim_end_matches([' ', '\t']).to_string();
                } else if is_top_level_bullet(line) {
                    if let Some(slug) = queue::first_bold_slug(line) {
                        let key = if cur_sub.is_empty() { "(top)".to_string() } else { cur_sub.clone() };
                        match tally.iter_mut().find(|(k, _)| *k == key) {
                            Some((_, n)) => *n += 1,
                            None => {
                                tally_order.push(key.clone());
                                tally.push((key.clone(), 1));
                            }
                        }
                        deferred.push(joined(slug, &title(line)));
                    }
                }
            }
            _ => {}
        }
    }

    out.push_str("Active (pick the first •):\n");
    if active.is_empty() {
        out.push_str("  (none — active queue empty)\n");
    }
    for (mark, row) in &active {
        out.push_str(&format!("  {} {}\n", mark, row));
    }
    out.push('\n');
    if collapse {
        out.push_str("Deferred (tally):\n");
        if tally_order.is_empty() {
            out.push_str("  (none)\n");
        }
        for key in &tally_order {
            let n = tally.iter().find(|(k, _)| k == key).map(|(_, n)| *n).unwrap_or(0);
            out.push_str(&format!("  {}: {}\n", key, n));
        }
    } else {
        out.push_str("Deferred:\n");
        if deferred.is_empty() {
            out.push_str("  (none)\n");
        }
        for row in &deferred {
            out.push_str(&format!("  {}\n", row));
        }
    }
    if !sec_cfg.icebox.is_empty() {
        out.push_str(&format!("{}: {} entries\n", sec_cfg.icebox, icebox_n));
    }
    if attend_n > 0 {
        out.push('\n');
        out.push_str("Attention (Lessons [attend], this iteration):\n");
        for row in &attend {
            out.push_str(&format!("  {}\n", row));
        }
        if attend_n > cap {
            out.push_str(&format!("  (+{} more [attend])\n", attend_n - cap));
        }
    }
    Ok(out)
}

fn extent(text: &str, slug: &str) -> Result<String, String> {
    if slug.is_empty() {
        return Err("--extent needs a <slug>".to_string());
    }
    let mut start = 0usize;
    let mut ind = 0usize;
    let mut found = false;
    let mut n = 0usize;
    for (i, line) in text.lines().enumerate() {
        n = i + 1;
        if !found {
            if let Some(s) = queue::bullet_slug(line) {
                if s == slug {
                    found = true;
                    start = n;
                    ind = queue::indent(line);
                }
            }
            continue;
        }
        if line.starts_with('#') || line.trim_end_matches([' ', '\t']) == "---" {
            return Ok(format!("{} {}\n", start, n - 1));
        }
        if queue::is_bullet(line) && queue::indent(line) <= ind {
            return Ok(format!("{} {}\n", start, n - 1));
        }
    }
    if found {
        return Ok(format!("{} {}\n", start, n));
    }
    Err(format!("slug not found: {}", slug))
}

// spec: queue-kit/SPEC.md §The queue-index arm — the cutoff keeps its `date -d` derivation rather
// than an in-crate civil-date one, which would resolve UTC where this resolves the operator's zone
fn age_cutoff() -> Result<String, String> {
    let days = queue::knob_scalar("QUEUE_KIT_ICEBOX_AGE_DAYS")?;
    let spec = format!("{} days ago", days.trim());
    let c = proc::run("date", &["-d", &spec, "+%F"])?;
    let out = c
        .stdout()
        .ok_or_else(|| "cannot compute the age cutoff (date -d unavailable)".to_string())?;
    let s = String::from_utf8_lossy(out).trim().to_string();
    if s.is_empty() {
        return Err("cannot compute the age cutoff (date -d unavailable)".to_string());
    }
    Ok(s)
}

// spec: queue-kit/SPEC.md §The queue-index arm — awk's `^.*\*\*Cost while deferred:?\*\*:?[ ]*`
// with a greedy lead, so the rightmost lead-in wins; the truncation is the advisory's own ceiling.
fn opener(line: &str) -> String {
    const NEEDLE: &str = "**Cost while deferred";
    let mut cut: Option<usize> = None;
    let mut from = 0usize;
    while let Some(rel) = line[from..].find(NEEDLE) {
        let at = from + rel;
        let mut p = at + NEEDLE.len();
        let b = line.as_bytes();
        if b.get(p) == Some(&b':') {
            p += 1;
        }
        if line[p..].starts_with("**") {
            p += 2;
            if line.as_bytes().get(p) == Some(&b':') {
                p += 1;
            }
            while matches!(line.as_bytes().get(p), Some(&c) if c == b' ' || c == b'\t') {
                p += 1;
            }
            cut = Some(p);
        }
        from = at + 1;
    }
    let t = match cut {
        Some(p) => &line[p..],
        None => line,
    };
    let t = t.trim_end_matches([' ', '\t']);
    if t.is_empty() {
        return "(unstated)".to_string();
    }
    if t.chars().count() > OPENER_CAP {
        let head: String = t.chars().take(OPENER_CAP - 1).collect();
        return format!("{}…", head);
    }
    t.to_string()
}

// spec: queue-kit/SPEC.md §The queue-index arm — the low cost class is matched on the opener as
// prose, an unacceptable heuristic in a gate and the right ceiling in an advisory worklist.
fn low_class(t: &str) -> bool {
    ["low", "zero", "bounded", "cosmetic"].iter().any(|p| t.starts_with(p))
}

fn find_dated(line: &str, label: &str) -> Option<String> {
    let mut from = 0usize;
    while let Some(rel) = line[from..].find(label) {
        let at = from + rel;
        let p = at + label.len();
        let b = line.as_bytes();
        if p + 10 <= line.len() {
            let d = &line[p..p + 10];
            let ok = d.as_bytes().iter().enumerate().all(|(i, c)| match i {
                4 | 7 => *c == b'-',
                _ => c.is_ascii_digit(),
            });
            if ok {
                return Some(d.to_string());
            }
        }
        let _ = b;
        from = at + 1;
    }
    None
}

struct Pending {
    slug: String,
    start: usize,
    surfaced: String,
    filed: String,
    cost: String,
}

fn flush(p: &mut Option<Pending>, at: usize, cutoff: &str, out: &mut String) {
    let Some(e) = p.take() else { return };
    let d = if !e.surfaced.is_empty() { e.surfaced.as_str() } else { e.filed.as_str() };
    let dated_in = d.is_empty() || d < cutoff;
    let costed_in = e.cost.is_empty() || low_class(&e.cost);
    if dated_in && costed_in {
        let shown_date = if d.is_empty() { "(undated)" } else { d };
        let shown_cost = if e.cost.is_empty() { "(uncosted)" } else { e.cost.as_str() };
        out.push_str(&format!(
            "{:<46} {:>4}l  {:<11} {}\n",
            e.slug,
            at - e.start,
            shown_date,
            shown_cost
        ));
    }
}

fn candidates(text: &str) -> Result<String, String> {
    let sec_cfg = Sections::active_and_deferred()?;
    let cutoff = age_cutoff()?;
    let mut out = String::new();
    let mut in_deferred = false;
    let mut pending: Option<Pending> = None;
    let mut n = 0usize;

    for (i, line) in text.lines().enumerate() {
        n = i + 1;
        if queue::is_section_line(line) {
            flush(&mut pending, n, &cutoff, &mut out);
            in_deferred = sec_cfg.is_deferred(line);
            continue;
        }
        if !in_deferred {
            continue;
        }
        if let Some(slug) = queue::bullet_slug(line) {
            flush(&mut pending, n, &cutoff, &mut out);
            pending = Some(Pending {
                slug: slug.to_string(),
                start: n,
                surfaced: String::new(),
                filed: String::new(),
                cost: String::new(),
            });
            continue;
        }
        let Some(e) = pending.as_mut() else { continue };
        if e.surfaced.is_empty() {
            if let Some(d) = find_dated(line, "Surfaced ") {
                e.surfaced = d;
            }
        }
        if e.filed.is_empty() {
            if let Some(d) = find_dated(line, "Filed ") {
                e.filed = d;
            }
        }
        if e.cost.is_empty() && line.contains("**Cost while deferred") {
            e.cost = opener(line);
        }
    }
    flush(&mut pending, n + 1, &cutoff, &mut out);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: queue-kit/SPEC.md §The queue-index arm — the rendering cases the shell fixture used to
    // pin, moved here with the tool: tag residue, the multi-tag lead line, the drain-exempt echo,
    // the empty title, and the attend block's cap and overflow across both index renderings.
    const Q: &str = "\
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-tagged** [design-pending] — a tagged active entry.
- **feat-two-tags** [design-pending] [roadmap: later/thing] — two tags here.
- **feat-alltag** [design-pending]
- **feat-exempt** [drain-exempt: half pending] — exempt and titled.
- **feat-blocked** [blocked-by: feat-tagged] — waiting on one.

## Technical Debt

## Deferred

- **def-tagged** [design-pending] — a tagged deferred entry.
- **def-alltag** [spec: some-kit/SPEC.md §A Long Pointer Section]

## Done

## Lessons Learned

- **l1** [attend] — first attention point
- **l2** [attend] — second attention point
- **l3** [attend] — third attention point
- **l4** — not an attention point
";

    fn render(collapse: bool, cap: usize) -> String {
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_ACTIVE_SECTIONS", "New Features\tTechnical Debt");
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_DEFERRED_SECTION", "Deferred");
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_ICEBOX_SECTION", "");
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_ATTEND_CAP", cap.to_string());
        index(Q, collapse).expect("index render failed")
    }

    #[test]
    fn a_tag_comes_off_without_leaving_the_separator_it_sat_next_to() {
        let out = render(false, 3);
        assert!(out.contains("• feat-tagged — a tagged active entry."), "{}", out);
        assert!(out.contains("• feat-two-tags — two tags here."), "{}", out);
        assert!(out.contains("def-tagged — a tagged deferred entry."), "{}", out);
    }

    #[test]
    fn a_lead_line_that_is_all_tag_renders_as_the_bare_slug() {
        let out = render(false, 3);
        assert!(out.contains("• feat-alltag"), "{}", out);
        assert!(!out.contains("feat-alltag —"), "{}", out);
        assert!(!out.contains("def-alltag —"), "{}", out);
    }

    #[test]
    fn drain_exempt_is_re_echoed_and_blocked_by_flips_the_mark() {
        let out = render(false, 3);
        assert!(
            out.contains("• feat-exempt — exempt and titled.   [drain-exempt: half pending]"),
            "{}",
            out
        );
        assert!(
            out.contains("✗ feat-blocked — waiting on one.   [blocked-by: feat-tagged]"),
            "{}",
            out
        );
    }

    #[test]
    fn the_attend_block_caps_and_reports_its_overflow() {
        let out = render(false, 2);
        assert!(out.contains("Attention (Lessons [attend], this iteration):"), "{}", out);
        assert!(out.contains("first attention point"), "{}", out);
        assert!(out.contains("second attention point"), "{}", out);
        assert!(!out.contains("third attention point"), "{}", out);
        assert!(out.contains("(+1 more [attend])"), "{}", out);
        assert!(!out.contains("not an attention point"), "{}", out);
    }

    #[test]
    fn the_default_cap_shows_every_lead_line_with_no_overflow_note() {
        let out = render(false, 3);
        assert!(out.contains("third attention point"), "{}", out);
        assert!(!out.contains("more [attend])"), "{}", out);
    }

    #[test]
    fn collapse_deferred_tallies_and_still_appends_the_attend_block() {
        let out = render(true, 2);
        assert!(out.contains("Deferred (tally):"), "{}", out);
        assert!(out.contains("  (top): 2"), "{}", out);
        assert!(out.contains("Attention (Lessons [attend], this iteration):"), "{}", out);
        assert!(out.contains("first attention point"), "{}", out);
    }

    #[test]
    fn an_absent_attend_tag_produces_no_block_at_all() {
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_ACTIVE_SECTIONS", "New Features");
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_DEFERRED_SECTION", "Deferred");
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_ICEBOX_SECTION", "");
        std::env::set_var("GATE_SDK_KNOB_QUEUE_KIT_ATTEND_CAP", "3");
        let out = index(
            "## Iteration: demo\n\n## New Features\n\n## Deferred\n\n## Lessons Learned\n\n- **only** — an untagged lesson\n",
            false,
        )
        .expect("index render failed");
        assert!(!out.contains("Attention (Lessons"), "{}", out);
        assert!(out.contains("  (none — active queue empty)"), "{}", out);
        assert!(out.contains("  (none)"), "{}", out);
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — extent is the range an eviction deletes, so it
    // ends at the line before the next same-or-shallower bullet, heading or separator.
    #[test]
    fn extent_ends_at_the_line_before_the_next_entry() {
        let r = extent(Q, "feat-tagged").expect("extent failed");
        assert_eq!(r, "7 7\n", "{}", r);
        let r = extent(Q, "def-alltag").expect("extent failed");
        assert_eq!(r, "18 19\n", "{}", r);
        assert!(extent(Q, "no-such-slug").is_err());
    }

    #[test]
    fn the_cost_opener_takes_the_rightmost_lead_in_and_reports_an_empty_one() {
        assert_eq!(opener("  **Cost while deferred:** low and non-rotting."), "low and non-rotting.");
        assert_eq!(opener("  **Cost while deferred**: zero today."), "zero today.");
        assert_eq!(opener("  **Cost while deferred:**"), "(unstated)");
        assert!(low_class("low and non-rotting."));
        assert!(!low_class("high, paid every iteration."));
    }
}
