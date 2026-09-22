// spec: queue-kit/SPEC.md §The queue-index arm — the compact queue surface for task selection.
// Three modes on one arm, selected from the arm's own argv tail rather than from three arms:
// the emitter type is defined over an argv slice precisely so a mode rides as a flag.
use crate::gates::queue_entry_budget;
use crate::queue::{self, is_top_level_bullet, Sections};

const TITLE_CAP: usize = 64;
const CAUSE_CAP: usize = 48;

// spec: queue-kit/SPEC.md §The tag algebra — the board tags, in the order a deferred row re-echoes them
const BOARD_TAGS: [&str; 2] = ["cost", "surface"];

// spec: queue-kit/SPEC.md §The icebox tier — the low class, the icebox's partition of the cost set
const LOW_CLASS: [&str; 2] = ["event/low", "once/low"];

const USAGE: &str = "\
usage: --emit queue-index [--collapse-deferred] [--extent <slug>] [--icebox-candidates] [queue-file]
  default: header + active (• ready / ✗ blocked) + deferred titles and board tags + icebox tally;
  --collapse-deferred: deferred tally; --extent <slug>: \"<start> <end>\";
  --icebox-candidates: eviction worklist (• eligible / ✗ excluded, cause in place of the class)
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

// spec: queue-kit/SPEC.md §The queue-index arm — a markdown link renders as its text, so a summary
// citing a live entry reads as prose rather than as link syntax
fn plain(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        let link = after.find("](").and_then(|m| after[m + 2..].find(')').map(|c| (m, m + 2 + c)));
        match link {
            Some((m, c)) if !after[..m].contains(['[', ']']) => {
                out.push_str(&rest[..open]);
                out.push_str(&after[..m]);
                rest = &after[c + 1..];
            }
            _ => {
                out.push_str(&rest[..open + 1]);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

// spec: queue-kit/SPEC.md §The queue-index arm — the title is the entry's summary, its first body
// paragraph before any sub-task, and an entry with none renders as the bare slug
fn title(lines: &[&str], e: &queue::Entry) -> String {
    let summary = e
        .body_lines()
        .map(|i| lines[i])
        .take_while(|l| queue::heading_level(l).is_none())
        .find(|l| !l.trim().is_empty())
        .unwrap_or("");
    truncate_chars(plain(summary).trim_matches([' ', '\t']), TITLE_CAP)
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

// spec: queue-kit/SPEC.md §The queue-index arm — the lead line's `[blocked-by:]` set, read
// through the shared adapter; the presence of any one is what flips the row's ready mark.
fn blockers(line: &str) -> String {
    queue::blocked_by(line).join(", ")
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

    let lines: Vec<&str> = text.lines().collect();
    let mut active: Vec<(char, String)> = Vec::new();
    let mut deferred: Vec<String> = Vec::new();
    let mut icebox_n = 0usize;
    for e in queue::entries(&lines, &sec_cfg).iter().filter(|e| e.level == 3) {
        let tags = e.tags(&lines);
        if sec_cfg.active.contains(&e.section) {
            let bl = blockers(tags);
            let mark = if bl.is_empty() { '•' } else { '✗' };
            let mut row = joined(&e.slug, &title(&lines, e));
            if !bl.is_empty() {
                row.push_str(&format!("   [blocked-by: {}]", bl));
            }
            if let Some(d) = drain_exempt(tags) {
                row.push_str(&format!("   [drain-exempt: {}]", d));
            }
            active.push((mark, row));
        } else if sec_cfg.is_deferred(&e.section) {
            let mut row = joined(&e.slug, &title(&lines, e));
            for name in BOARD_TAGS {
                if let Some(t) = queue::field_tags(tags, name).first() {
                    row.push_str(&format!("   [{}: {}]", name, t.raw.trim()));
                }
            }
            deferred.push(row);
        } else {
            icebox_n += 1;
        }
    }

    let mut attend_n = 0usize;
    let mut attend: Vec<String> = Vec::new();
    let mut in_lessons = false;
    for line in &lines {
        if queue::is_section_line(line) {
            in_lessons = queue::is_lessons_line(line);
            continue;
        }
        if in_lessons && is_top_level_bullet(line) && line.contains("[attend]") {
            attend_n += 1;
            if attend_n <= cap {
                attend.push(line.trim_end_matches([' ', '\t']).to_string());
            }
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
        if deferred.is_empty() {
            out.push_str("  (none)\n");
        } else {
            out.push_str(&format!("  (top): {}\n", deferred.len()));
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
    let lines: Vec<&str> = text.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if let Some((level, s)) = queue::entry_heading(line) {
            if s == slug {
                return Ok(format!("{} {}\n", i + 1, queue::extent_end(&lines, i, level)));
            }
        }
    }
    Err(format!("slug not found: {}", slug))
}

// spec: queue-kit/SPEC.md §The queue-index arm — the cutoff is civil-day arithmetic on the
// operator's today, and an unreadable today refuses rather than guessing a zone
fn age_cutoff() -> Result<String, String> {
    let days = queue::knob_scalar("QUEUE_KIT_ICEBOX_AGE_DAYS")?;
    let n: i64 = days.trim().parse().map_err(|_| {
        format!(
            "cannot compute the age cutoff (QUEUE_KIT_ICEBOX_AGE_DAYS='{}' is not a day count)",
            days.trim()
        )
    })?;
    super::kpi::days_ago(n)
        .ok_or_else(|| "cannot compute the age cutoff (the local civil date is unreadable)".to_string())
}

// spec: queue-kit/SPEC.md §The queue-index arm — the class is the lead line's first `[cost:]` value
// that parses, through the shared adapter; a malformed value is no class and lists as unclassed.
fn lead_class(line: &str) -> String {
    queue::field_tags(line, "cost")
        .iter()
        .find_map(|t| t.value.filter(|v| queue::cost_class_valid(v)))
        .unwrap_or("")
        .to_string()
}

fn is_slug_byte(c: u8) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-'
}

// spec: queue-kit/SPEC.md §The queue-index arm — a slug is *named* only where it stands as a
// whole token, the neighbouring bytes falling outside the slug alphabet.
fn names_slug(hay: &str, slug: &str) -> bool {
    let b = hay.as_bytes();
    let mut from = 0usize;
    while let Some(rel) = hay[from..].find(slug) {
        let at = from + rel;
        let before_ok = at == 0 || !is_slug_byte(b[at - 1]);
        let end = at + slug.len();
        let after_ok = end >= b.len() || !is_slug_byte(b[end]);
        if before_ok && after_ok {
            return true;
        }
        from = at + 1;
    }
    false
}

fn date_at(b: &[u8], i: usize) -> bool {
    (0..10).all(|k| match k {
        4 | 7 => b[i + k] == b'-',
        _ => b[i + k].is_ascii_digit(),
    })
}

fn has_date(line: &str) -> bool {
    last_date(line).is_some()
}

// spec: queue-kit/SPEC.md §The queue-index arm — dates are appended in order, so the newest is the last
fn last_date(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    (0..b.len().saturating_sub(9)).rev().find(|&i| date_at(b, i)).map(|i| &line[i..i + 10])
}

// spec: queue-kit/SPEC.md §The icebox tier — the categorical half of the eligibility rule; §The
// queue-index arm owns the 2026-08-24 reopening that moved it here.
// spec: queue-kit/SPEC.md §The queue-index arm — a cause is its fixed class prefix, which prints
// whole, and its variable tail, which is the only part a caller may cap.
fn ineligibility(e: &Pending, cutoff: &str, live: &[String]) -> Option<(String, String)> {
    if e.lead.contains("[roadmap:") {
        return Some(("[roadmap] tag — not icebox-eligible".to_string(), String::new()));
    }
    // spec: queue-kit/SPEC.md §The queue-index arm — a written standing cause outranks an inferred trigger
    if let Some(t) = queue::field_tags(&e.lead, "not-icebox-eligible").first() {
        return Some(standing(t.raw));
    }
    // spec: queue-kit/SPEC.md §The icebox tier — a recurrence ages on the entry's own window, read
    // off the array's last element
    if let Some(d) = queue::recurrence_dates(&e.lead).last().filter(|d| **d >= cutoff) {
        return Some((format!("[recurrence] re-filed {} — live trigger", d), String::new()));
    }
    for s in live {
        if *s == e.slug {
            continue;
        }
        if names_slug(&e.body, s) {
            return Some(("[trigger] names live slug ".to_string(), s.clone()));
        }
    }
    None
}

fn standing(rest: &str) -> (String, String) {
    let fields: Vec<&str> = rest.split_whitespace().collect();
    let (date, grounds) = match fields.split_first() {
        Some((d, g)) if d.len() == 10 && has_date(d) => (*d, g),
        _ => ("(undated)", &fields[..]),
    };
    let grounds = if grounds.is_empty() { "(ungrounded)".to_string() } else { grounds.join(" ") };
    (format!("[standing] {} — ", date), grounds)
}

struct Pending {
    slug: String,
    start: usize,
    marks: queue::DeferMarks,
    cost: String,
    lead: String,
    body: String,
}

// spec: queue-kit/SPEC.md §The queue-index arm — each deferred entry's size in the cap's unit, as
// the budget gate's own walk measures it, keyed by lead line
struct Sizes {
    unit: queue::Unit,
    by_start: std::collections::HashMap<usize, usize>,
}

fn flush(p: &mut Option<Pending>, sizes: &Sizes, cutoff: &str, live: &[String], out: &mut String) {
    let Some(e) = p.take() else { return };
    let d = e.marks.defer_date().unwrap_or("");
    let dated_in = d.is_empty() || d < cutoff;
    let classed_in = e.cost.is_empty() || LOW_CLASS.contains(&e.cost.as_str());
    if dated_in && classed_in {
        let shown_date = if d.is_empty() { "(undated)" } else { d };
        // spec: queue-kit/SPEC.md §The queue-index arm — an ineligible row keeps its line and
        // trades its class for the reason: the class is inclusion evidence, and it decides
        // nothing once a categorical exclusion has already settled the row.
        let (mark, tail) = match ineligibility(&e, cutoff, live) {
            Some((prefix, tail)) => ('✗', format!("{}{}", prefix, cap_chars(&tail))),
            None => {
                let c = if e.cost.is_empty() { "(unclassed)" } else { e.cost.as_str() };
                ('•', c.to_string())
            }
        };
        out.push_str(&format!(
            "{} {:<46} {:>9}  {:<11} {}\n",
            mark,
            e.slug,
            format!("{}{}", sizes.by_start.get(&e.start).copied().unwrap_or(0), sizes.unit.suffix()),
            shown_date,
            tail
        ));
    }
}

fn cap_chars(t: &str) -> String {
    if t.chars().count() > CAUSE_CAP {
        let head: String = t.chars().take(CAUSE_CAP - 1).collect();
        return format!("{}…", head);
    }
    t.to_string()
}

fn candidates(text: &str) -> Result<String, String> {
    let sec_cfg = Sections::active_and_deferred()?;
    let cutoff = age_cutoff()?;
    let live = queue::live_slugs(text, &sec_cfg);
    let unit = queue_entry_budget::display_unit(queue_entry_budget::cap()?);
    let sizes = Sizes {
        unit,
        by_start: queue_entry_budget::walk(text, &sec_cfg)
            .entries
            .iter()
            .map(|e| (e.start, e.size(unit)))
            .collect(),
    };
    let mut out = String::new();
    let lines: Vec<&str> = text.lines().collect();
    for e in queue::entries(&lines, &sec_cfg) {
        if e.level != 3 || !sec_cfg.is_deferred(&e.section) {
            continue;
        }
        let tags = e.tags(&lines);
        let mut p = Pending {
            slug: e.slug.clone(),
            start: e.start + 1,
            marks: queue::DeferMarks::default(),
            cost: lead_class(tags),
            lead: tags.to_string(),
            body: String::new(),
        };
        for i in e.body_lines() {
            p.body.push_str(lines[i]);
            p.body.push('\n');
            p.marks.observe(lines[i]);
        }
        flush(&mut Some(p), &sizes, &cutoff, &live, &mut out);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: queue-kit/SPEC.md §The queue-index arm — the rendering cases: the title is the summary
    // with a link rendered as its text, the multi-tag tag line, the drain-exempt echo, the empty
    // title, and the attend block's cap and overflow across both index renderings.
    const Q: &str = "\
# TASK-QUEUE.md

## Iteration: demo

## New Features

### feat-tagged

[spec: SPEC-x.md]

a tagged active entry.

### feat-two-tags

[spec: SPEC-x.md] [roadmap: later/thing]

two tags here, citing [feat-tagged](#feat-tagged).

### feat-alltag

[spec: SPEC-x.md]

### feat-exempt

[drain-exempt: half pending]

exempt and titled.

### feat-blocked

[blocked-by: feat-tagged]

waiting on one.

## Technical Debt

## Deferred

### def-tagged

[cost: event/low] [surface: queue-kit]

a tagged deferred entry.

### def-alltag

[spec: some-kit/SPEC.md §A Long Pointer Section]

## Done

## Lessons Learned

- **l1** [attend] — first attention point
- **l2** [attend] — second attention point
- **l3** [attend] — third attention point
- **l4** — not an attention point
";

    fn render(collapse: bool, cap: usize) -> String {
        let knobs = crate::knobenv::lock();
        knobs.set("QUEUE_KIT_ATTEND_CAP", &cap.to_string());
        let out = index(Q, collapse).expect("index render failed");
        knobs.remove("QUEUE_KIT_ATTEND_CAP");
        out
    }

    #[test]
    fn a_tag_comes_off_without_leaving_the_separator_it_sat_next_to() {
        let out = render(false, 3);
        assert!(out.contains("• feat-tagged — a tagged active entry."), "{}", out);
        assert!(out.contains("• feat-two-tags — two tags here, citing feat-tagged."), "{}", out);
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

    // spec: queue-kit/SPEC.md §The queue-index arm — a deferred row re-echoes its board tags after
    // the title, cost first, and a row carrying neither echoes nothing.
    #[test]
    fn a_deferred_row_re_echoes_its_board_tags_after_the_title() {
        let out = render(false, 3);
        assert!(
            out.contains("  def-tagged — a tagged deferred entry.   [cost: event/low]   [surface: queue-kit]\n"),
            "{}",
            out
        );
        assert!(out.contains("  def-alltag\n"), "{}", out);
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
        let out = {
            let knobs = crate::knobenv::lock();
            knobs.remove("QUEUE_KIT_ATTEND_CAP");
            index(
                "## Iteration: demo\n\n## New Features\n\n## Deferred\n\n## Lessons Learned\n\n- **only** — an untagged lesson\n",
                false,
            )
            .expect("index render failed")
        };
        assert!(!out.contains("Attention (Lessons"), "{}", out);
        assert!(out.contains("  (none — active queue empty)"), "{}", out);
        assert!(out.contains("  (none)"), "{}", out);
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — extent is the range an eviction deletes, so it
    // ends at the line before the next same-or-shallower heading or separator.
    #[test]
    fn extent_ends_at_the_line_before_the_next_entry() {
        let r = extent(Q, "feat-tagged").expect("extent failed");
        assert_eq!(r, "7 12\n", "{}", r);
        let r = extent(Q, "def-alltag").expect("extent failed");
        assert_eq!(r, "45 48\n", "{}", r);
        assert!(extent(Q, "no-such-slug").is_err());
    }

    // spec: queue-kit/SPEC.md §The icebox tier — a slug is named only as a whole token, so a
    // longer slug that merely contains a shorter one holds no row against it.
    #[test]
    fn a_slug_is_named_only_where_it_stands_as_a_whole_token() {
        assert!(names_slug("held by `some-slug` today", "some-slug"));
        assert!(names_slug("held by [some-slug](#some-slug) today", "some-slug"));
        assert!(names_slug("[blocked-by: some-slug]", "some-slug"));
        assert!(!names_slug("names some-slug-extended instead", "some-slug"));
        assert!(!names_slug("names a-some-slug instead", "some-slug"));
        assert!(names_slug("some-slug", "some-slug"));
    }

    const CUT: &str = "2026-07-01";

    // spec: queue-kit/SPEC.md §The icebox tier — a recurrence is live only while the array's last
    // date is inside the age window, and the cause prints that date.
    #[test]
    fn a_recurrence_is_live_only_while_its_newest_date_is_inside_the_window() {
        let live: Vec<String> = vec![];
        let r = ineligibility(&pend("[recurrence: 2026-07-01]", "t.\n"), CUT, &live);
        assert_eq!(full(r).unwrap(), "[recurrence] re-filed 2026-07-01 — live trigger");
        let r = ineligibility(&pend("[recurrence: 2026-05-01, 2026-06-30]", "t.\n"), CUT, &live);
        assert!(r.is_none(), "an aged recurrence is no live trigger: {:?}", r);
        let r = ineligibility(&pend("[recurrence: 2026-05-01, 2026-08-02]", "t.\n"), CUT, &live);
        assert_eq!(full(r).unwrap(), "[recurrence] re-filed 2026-08-02 — live trigger");
        let live = vec!["other".to_string()];
        let r = ineligibility(&pend("[recurrence: 2026-05-01]", "waits on [other](#other).\n"), CUT, &live);
        assert_eq!(full(r).unwrap(), "[trigger] names live slug other");
    }

    fn pend(lead: &str, body: &str) -> Pending {
        Pending {
            slug: "subject".to_string(),
            start: 1,
            marks: queue::DeferMarks::default(),
            cost: "event/low".to_string(),
            lead: lead.to_string(),
            body: body.to_string(),
        }
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — a cause's fixed prefix and variable tail,
    // joined uncapped, is exactly the row a reader saw before the cap ever applies.
    fn full(r: Option<(String, String)>) -> Option<String> {
        r.map(|(prefix, tail)| format!("{}{}", prefix, tail))
    }

    // spec: queue-kit/SPEC.md §The icebox tier — all three categorical triggers, plus the two
    // near-misses: self-naming, and a recurrence tag carrying no date, neither a live trigger.
    #[test]
    fn every_categorical_trigger_is_decided_and_self_naming_is_not_one() {
        let live = vec!["other".to_string(), "subject".to_string()];
        let r = ineligibility(&pend("[roadmap: now/x]", ""), CUT, &live);
        assert!(full(r).unwrap().starts_with("[roadmap]"));
        let r = ineligibility(&pend("[recurrence: 2026-08-01]", ""), CUT, &live);
        assert!(full(r).unwrap().starts_with("[recurrence]"));
        let r = ineligibility(&pend("", "waits on [other](#other) landing.\n"), CUT, &live);
        assert_eq!(full(r).unwrap(), "[trigger] names live slug other");
        let r = ineligibility(&pend("", "subject is the whole of it.\n"), CUT, &live);
        assert!(r.is_none(), "self-naming is narration, not a trigger: {:?}", r);
        let r = ineligibility(&pend("[recurrence: soon]", ""), CUT, &live);
        assert!(r.is_none(), "an undated recurrence tag is no re-filing: {:?}", r);
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — the standing cause's place in the order and its
    // printed form, an absent date or grounds appearing rather than vanishing, and the tag's own
    // name dropped from the printed prefix.
    #[test]
    fn a_standing_tag_is_decided_after_the_roadmap_tag_and_before_the_recurrence() {
        let live = vec!["other".to_string()];
        let decl = "[not-icebox-eligible: 2026-08-17 eviction spends the clause]";
        let r = ineligibility(&pend(decl, ""), CUT, &live);
        assert_eq!(full(r).unwrap(), "[standing] 2026-08-17 — eviction spends the clause");
        let r = ineligibility(&pend(&format!("[roadmap: now/x] {}", decl), ""), CUT, &live);
        assert!(full(r).unwrap().starts_with("[roadmap]"));
        let lead = format!("[recurrence: 2026-08-01] {}", decl);
        let r = ineligibility(&pend(&lead, "waits on [other](#other).\n"), CUT, &live);
        assert!(full(r).as_deref().unwrap_or("").starts_with("[standing]"));
        let r = ineligibility(&pend("[not-icebox-eligible: grounds only]", ""), CUT, &live);
        assert_eq!(full(r).unwrap(), "[standing] (undated) — grounds only");
        let r = ineligibility(&pend("[not-icebox-eligible: 2026-08-17]", ""), CUT, &live);
        assert_eq!(full(r).unwrap(), "[standing] 2026-08-17 — (ungrounded)");
        let r = ineligibility(&pend("", "prose naming not-icebox-eligible: mid-line.\n"), CUT, &live);
        assert!(r.is_none(), "only the tag declares: {:?}", r);
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — the cap binds the variable tail and never the
    // fixed class prefix, however wide the prefix runs against the cap on its own.
    #[test]
    fn the_cap_binds_the_variable_tail_and_never_the_class_prefix() {
        let live: Vec<String> = vec![];
        let long = "a very long declared reason that runs well past the forty eight character mark on its own";
        let decl = format!("[not-icebox-eligible: 2026-08-17 {}]", long);
        let (prefix, tail) = ineligibility(&pend(&decl, ""), CUT, &live).unwrap();
        assert_eq!(prefix, "[standing] 2026-08-17 — ");
        assert!(!prefix.contains("not-icebox-eligible"), "{}", prefix);
        assert!(tail.chars().count() > CAUSE_CAP, "fixture must exceed the cap: {}", tail);
        let capped = cap_chars(&tail);
        assert_eq!(capped.chars().count(), CAUSE_CAP);
        assert!(capped.ends_with('…'), "{}", capped);
    }

    // spec: queue-kit/SPEC.md §The queue-migrate arm — the worklist cases are written in the retired
    // bullet grammar and read through the migration arm, which makes each one a case of that arm too
    fn worklist(legacy: &str) -> String {
        let sec = Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
            done: "Done".into(),
        };
        let q = crate::emit::queue_migrate::convert(legacy, &sec).expect("migration failed");
        let knobs = crate::knobenv::lock();
        knobs.set("QUEUE_KIT_ICEBOX_AGE_DAYS", "7");
        let out = candidates(&q).expect("candidates failed");
        knobs.remove("QUEUE_KIT_ICEBOX_AGE_DAYS");
        out
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — the census is preserved: an excluded row
    // keeps its line and trades the class for the cause.
    #[test]
    fn an_excluded_row_is_marked_and_carries_its_cause_instead_of_the_class() {
        let out = worklist(
            "\
## Iteration: demo

## New Features

- **live-one** — an unbuilt active entry.

## Deferred

- **keeper** [cost: event/low] [surface: queue-kit] — nothing holds it.
  **Cost while deferred:** paid when the surface is touched.
  Filed 2020-01-01 by close.
- **tagged** [cost: event/low] [surface: queue-kit] [roadmap: now/x] — published.
  **Cost while deferred:** paid when the surface is touched.
  Filed 2020-01-01 by close.
- **held** [cost: event/low] [surface: queue-kit] — waits.
  **Cost while deferred:** paid when the surface is touched.
  Filed 2020-01-01 by close, and it waits on `live-one`.

## Lessons Learned
",
        );
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 3, "every row is still listed: {}", out);
        assert!(lines[0].starts_with("• keeper"), "{}", out);
        assert!(lines[0].ends_with("event/low"), "{}", out);
        assert!(lines[1].starts_with("✗ tagged"), "{}", out);
        assert!(lines[1].contains("[roadmap]"), "{}", out);
        assert!(!lines[1].contains("event/low"), "{}", out);
        assert!(lines[2].starts_with("✗ held"), "{}", out);
        assert!(lines[2].ends_with("[trigger] names live slug live-one"), "{}", out);
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — the class comes off the lead line and never the
    // cost field's prose; only the low class is listed, and a missing or malformed class appears.
    #[test]
    fn only_event_low_and_once_low_are_listed_and_an_absent_class_appears_unclassed() {
        let out = worklist(
            "\
## Iteration: demo

## New Features

## Deferred

- **ev-low** [cost: event/low] [surface: queue-kit] — t.
  **Cost while deferred:** every iteration re-reads it.
  Filed 2020-01-01 by close.
- **once-low** [cost: once/low] [surface: queue-kit] — t.
  **Cost while deferred:** fixed.
  Filed 2020-01-01 by close.
- **it-low** [cost: iteration/low] [surface: queue-kit] — t.
  **Cost while deferred:** low and quiet.
  Filed 2020-01-01 by close.
- **ev-high** [cost: event/high] [surface: queue-kit] — t.
  **Cost while deferred:** low and quiet.
  Filed 2020-01-01 by close.
- **untagged** — t.
  **Cost while deferred:** low and quiet.
  Filed 2020-01-01 by close.
- **malformed** [cost: event/lo] [surface: queue-kit] — t.
  **Cost while deferred:** low and quiet.
  Filed 2020-01-01 by close.

## Lessons Learned
",
        );
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 4, "{}", out);
        assert!(lines[0].starts_with("• ev-low") && lines[0].ends_with("event/low"), "{}", out);
        assert!(lines[1].starts_with("• once-low") && lines[1].ends_with("once/low"), "{}", out);
        assert!(lines[2].starts_with("• untagged") && lines[2].ends_with("(unclassed)"), "{}", out);
        assert!(lines[3].starts_with("• malformed") && lines[3].ends_with("(unclassed)"), "{}", out);
    }

    // spec: queue-kit/SPEC.md §The queue-index arm — the rendered worklist row shows the cap's worth
    // of grounds behind the shortened prefix, not three characters of a cause string capped whole.
    #[test]
    fn a_rendered_standing_row_shows_capped_grounds_behind_the_shortened_prefix() {
        let out = worklist(
            "\
## Iteration: demo

## New Features

## Deferred

- **long-grounds** [cost: event/low] [surface: queue-kit] — t.
  not-icebox-eligible: long-grounds 2026-08-17 a very long declared reason that runs well past the forty eight character mark on its own
  Filed 2020-01-01 by close.

## Lessons Learned
",
        );
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 1, "{}", out);
        assert!(lines[0].starts_with("✗ long-grounds"), "{}", out);
        assert!(lines[0].contains("[standing] 2026-08-17 — "), "{}", out);
        assert!(!lines[0].contains("not-icebox-eligible"), "{}", out);
        assert!(lines[0].ends_with('…'), "{}", out);
    }
}
