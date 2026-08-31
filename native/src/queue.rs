// spec: queue-kit/SPEC.md §lib/queue.sh — the Rust counterpart of queue-kit/lib/queue.sh's
// shared surface: the derived section matchers and the slug adapters. The shell library is
// not retired, so this module sits beside it rather than replacing it
// spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged read has exactly one implementation in
// the crate, `walk`'s; these two are the queue-kit-facing spelling of it, so a second copy
// of the unset-is-an-error rule cannot drift from the first
pub fn knob_array(name: &str) -> Result<Vec<String>, String> {
    crate::walk::knob_array(name)
}

pub fn knob_scalar(name: &str) -> Result<String, String> {
    crate::walk::knob_scalar(name)
}

// spec: queue-kit/SPEC.md §lib/queue.sh — the section vocabulary every derived matcher below
// is computed from, resolved once per invocation from the bridged knobs
pub struct Sections {
    pub active: Vec<String>,
    pub deferred: String,
    pub icebox: String,
    pub done: String,
}

impl Sections {
    pub fn active_and_deferred() -> Result<Self, String> {
        Ok(Sections {
            active: knob_array("QUEUE_KIT_ACTIVE_SECTIONS")?,
            deferred: knob_scalar("QUEUE_KIT_DEFERRED_SECTION")?,
            icebox: knob_scalar("QUEUE_KIT_ICEBOX_SECTION")?,
            done: String::new(),
        })
    }

    pub fn with_done() -> Result<Self, String> {
        let mut s = Sections::active_and_deferred()?;
        s.done = knob_scalar("QUEUE_KIT_DONE_SECTION")?;
        Ok(s)
    }

    // spec: queue-kit/SPEC.md §The icebox tier — the icebox is a *live* task section, so it
    // joins the shared task set by derivation and carries every task-scoped rule with it
    pub fn task_sections(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.active.iter().map(String::as_str).collect();
        v.push(self.deferred.as_str());
        if !self.icebox.is_empty() {
            v.push(self.icebox.as_str());
        }
        v
    }

    pub fn is_task(&self, line: &str) -> bool {
        match heading_name(line) {
            Some(n) => self.task_sections().contains(&n),
            None => false,
        }
    }

    pub fn is_deferred(&self, line: &str) -> bool {
        heading_name(line).map(|n| n == self.deferred).unwrap_or(false)
    }

    // spec: queue-kit/SPEC.md §The icebox tier — an unset knob leaves a matcher nothing can
    // match, so every icebox reader degrades to "no icebox" rather than to "every section"
    pub fn is_icebox(&self, line: &str) -> bool {
        if self.icebox.is_empty() {
            return false;
        }
        heading_name(line).map(|n| n == self.icebox).unwrap_or(false)
    }

    pub fn is_done(&self, line: &str) -> bool {
        heading_name(line).map(|n| n == self.done).unwrap_or(false)
    }
}

// spec: queue-kit/SPEC.md §lib/queue.sh — QUEUE_SECTION_RE is `^## ` and nothing more: a
// prefix test, so a heading with trailing content still closes the section it ends
pub fn is_section_line(line: &str) -> bool {
    line.starts_with("## ")
}

// spec: queue-kit/SPEC.md §lib/queue.sh — the `^## <name>[[:space:]]*$` shape the derived
// section matchers share; trailing whitespace is the only slack the regex allows
pub fn heading_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("## ")?;
    Some(rest.trim_end_matches([' ', '\t']))
}

// spec: queue-kit/SPEC.md §The tag algebra — the Lessons heading is fixed spelling, read by
// queue-index and check-tag-lead-line; no knob, and the port reproduces the asymmetry as-is
pub fn is_lessons_line(line: &str) -> bool {
    heading_name(line).map(|n| n == "Lessons Learned").unwrap_or(false)
}

fn is_slug_byte(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'
}

fn is_slug_head(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit()
}

// spec: queue-kit/SPEC.md §lib/queue.sh — awk's leftmost-longest `\*\*[a-z0-9][a-z0-9-]*\*\*`,
// returning the slug between the delimiters
pub fn first_bold_slug(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i + 4 < b.len() {
        if b[i] != b'*' || b[i + 1] != b'*' {
            i += 1;
            continue;
        }
        let start = i + 2;
        if start >= b.len() || !is_slug_head(b[start]) {
            i += 1;
            continue;
        }
        let mut j = start + 1;
        while j < b.len() && is_slug_byte(b[j]) {
            j += 1;
        }
        if j + 1 < b.len() && b[j] == b'*' && b[j + 1] == b'*' {
            return Some(&line[start..j]);
        }
        i += 1;
    }
    None
}

// spec: queue-kit/SPEC.md §lib/queue.sh — the anchored guard the extraction runs behind:
// `^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*`
pub fn bullet_slug(line: &str) -> Option<&str> {
    let rest = strip_bullet_lead(line)?;
    let b = rest.as_bytes();
    if b.len() < 5 || b[0] != b'*' || b[1] != b'*' || !is_slug_head(b[2]) {
        return None;
    }
    let mut j = 3usize;
    while j < b.len() && is_slug_byte(b[j]) {
        j += 1;
    }
    if j + 1 < b.len() && b[j] == b'*' && b[j + 1] == b'*' {
        return Some(&rest[2..j]);
    }
    None
}

// spec: queue-kit/SPEC.md §The queue format — `^[[:space:]]*-[[:space:]]+`, returning what
// follows the bullet lead
pub fn strip_bullet_lead(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
    }
    if i >= b.len() || b[i] != b'-' {
        return None;
    }
    i += 1;
    let mut spaces = 0usize;
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
        spaces += 1;
    }
    if spaces == 0 {
        return None;
    }
    Some(&line[i..])
}

// spec: queue-kit/SPEC.md §The queue format — awk's `/^-[[:space:]]/`, the column-0 bullet the
// section scanners key on: an indented sub-task is deliberately outside it
// spec: queue-kit/SPEC.md §The queue-counts arm — shared, because the index and the counters
// report one queue's size and a second copy is what would let them disagree
pub fn is_top_level_bullet(line: &str) -> bool {
    let b = line.as_bytes();
    matches!(b.first(), Some(&c) if c == b'-')
        && matches!(b.get(1), Some(&c) if c == b' ' || c == b'\t')
}

// spec: queue-kit/SPEC.md §The tag algebra — every `[blocked-by: <slug>]` on a line, in order
// spec: queue-kit/SPEC.md §lib/queue.sh — a shared adapter because the grammar has two readers:
// the index arm marks a row blocked with it, the edges arm attributes an edge with it
pub fn blocked_by(line: &str) -> Vec<&str> {
    const TAG: &str = "[blocked-by:";
    let mut found: Vec<&str> = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find(TAG) {
        let after = &rest[open + TAG.len()..];
        let body = after.trim_start_matches([' ', '\t']);
        let consumed = after.len() - body.len();
        let b = body.as_bytes();
        let mut j = 0usize;
        if !b.is_empty() && is_slug_head(b[0]) {
            j = 1;
            while j < b.len() && is_slug_byte(b[j]) {
                j += 1;
            }
            found.push(&body[..j]);
        }
        rest = &after[consumed + j..];
    }
    found
}

// spec: queue-kit/SPEC.md §The queue format — `^[[:space:]]*-[[:space:]]` (one space, no
// `+`), the looser bullet test the section scanners use
pub fn is_bullet(line: &str) -> bool {
    let b = line.as_bytes();
    let mut i = 0usize;
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
    }
    if i >= b.len() || b[i] != b'-' {
        return false;
    }
    matches!(b.get(i + 1), Some(&c) if c == b' ' || c == b'\t')
}

// spec: queue-kit/SPEC.md §lib/queue.sh — awk's `match($0, /[^[:space:]]/) - 1`: the column
// of the first non-space character, and 0 for a line that is entirely space
pub fn indent(line: &str) -> usize {
    line.bytes()
        .position(|b| b != b' ' && b != b'\t')
        .unwrap_or(0)
}

// spec: queue-kit/SPEC.md §lib/queue.sh — queue_live_slugs: every bold kebab slug leading a
// bullet in a task section, in file order
pub fn live_slugs(text: &str, sec: &Sections) -> Vec<String> {
    let mut out = Vec::new();
    let mut inq = false;
    for line in text.lines() {
        if sec.is_task(line) {
            inq = true;
            continue;
        }
        if is_section_line(line) {
            inq = false;
        }
        if !inq {
            continue;
        }
        if bullet_slug(line).is_some() {
            if let Some(s) = first_bold_slug(line) {
                out.push(s.to_string());
            }
        }
    }
    out
}

// spec: queue-kit/SPEC.md §check-task-conservation — a done entry is a bare `- <slug>` line and
// nothing else (awk's `^[[:space:]]*-[[:space:]]+[a-z0-9][a-z0-9-]*[[:space:]]*$`), so an entry
// carried into the done section with its live shape intact matches neither grammar
pub fn bare_bullet_slug(line: &str) -> Option<&str> {
    let rest = strip_bullet_lead(line)?;
    let trimmed = rest.trim_end_matches([' ', '\t']);
    let b = trimmed.as_bytes();
    if b.is_empty() || !is_slug_head(b[0]) {
        return None;
    }
    if !b.iter().all(|c| is_slug_byte(*c)) {
        return None;
    }
    Some(trimmed)
}

// spec: queue-kit/SPEC.md §check-task-conservation — every bare bullet slug in the done
// section, in file order; the shell library carries no counterpart, its dead one having been
// deleted rather than held in parity (§lib/queue.sh)
pub fn done_slugs(text: &str, sec: &Sections) -> Vec<String> {
    let mut out = Vec::new();
    let mut ind = false;
    for line in text.lines() {
        if sec.is_done(line) {
            ind = true;
            continue;
        }
        if is_section_line(line) {
            ind = false;
        }
        if !ind {
            continue;
        }
        if let Some(s) = bare_bullet_slug(line) {
            out.push(s.to_string());
        }
    }
    out
}

// spec: queue-kit/SPEC.md §The queue format — the parity subject between this module and
// queue-kit/lib/queue.sh: what each side *classifies* one queue file as, never the derived
// literals. Its one consumer is gate-tests/queue-lib-parity.test.sh, which reads every field
pub fn parity_report(text: &str, sec: &Sections) -> Vec<String> {
    let mut out = Vec::new();
    let mut head = String::from("task-sections");
    for s in sec.task_sections() {
        head.push('\t');
        head.push_str(s);
    }
    out.push(head);
    for (i, line) in text.lines().enumerate() {
        let mut v: Vec<&str> = Vec::new();
        if is_section_line(line) {
            v.push("section");
        }
        // spec: queue-kit/SPEC.md §lib/queue.sh — QUEUE_ACTIVE_RE's counterpart is composed from
        // the live matchers rather than added as an accessor the runner would be the only reader
        // of: the task set is the active sections plus the deferred one plus a configured icebox
        if sec.is_task(line) && !sec.is_deferred(line) && !sec.is_icebox(line) {
            v.push("active");
        }
        if sec.is_deferred(line) {
            v.push("deferred");
        }
        if sec.is_icebox(line) {
            v.push("icebox");
        }
        if sec.is_task(line) {
            v.push("task");
        }
        if is_lessons_line(line) {
            v.push("lessons");
        }
        // spec: queue-kit/SPEC.md §lib/queue.sh — queue_live_slugs' own two-step: the anchored
        // bullet grammar guards, and the leftmost bold slug is what it prints
        let bullet = match bullet_slug(line) {
            Some(_) => first_bold_slug(line).unwrap_or("-"),
            None => "-",
        };
        if v.is_empty() && bullet == "-" {
            continue;
        }
        let verdicts = if v.is_empty() {
            "-".to_string()
        } else {
            v.join(",")
        };
        out.push(format!("line\t{}\t{}\t{}", i + 1, verdicts, bullet));
    }
    for s in live_slugs(text, sec) {
        out.push(format!("live\t{}", s));
    }
    out
}

// spec: queue-kit/SPEC.md §lib/queue.sh — the one [roadmap:] + roadmap-summary: parse, shared by
// the roadmap arm and check-roadmap-fresh so the two never disagree on what an entry claims. The
// typed record is that section's, TSV line and defensive `-` column included.
pub struct RoadmapEntry {
    pub tags: usize,
    pub field: String,
    pub slug: String,
    pub declarations: usize,
    pub summary: String,
}

const TAG_OPEN: &str = "[roadmap:";
const DECLARATION: &str = "roadmap-summary:";

// spec: queue-kit/SPEC.md §lib/queue.sh — awk's non-overlapping `while (match(s, /\[roadmap:/))`
fn tag_count(line: &str) -> usize {
    line.matches(TAG_OPEN).count()
}

// spec: queue-kit/SPEC.md §lib/queue.sh — awk's `/\[roadmap:[^]]*\]/`: the leftmost `[roadmap:`
// bounded by the first `]` after it. No `]` anywhere to its right means no match at any later
// occurrence either, so the first one decides.
fn tag_field(line: &str) -> String {
    let open = match line.find(TAG_OPEN) {
        Some(i) => i + TAG_OPEN.len(),
        None => return String::new(),
    };
    let close = match line[open..].find(']') {
        Some(i) => open + i,
        None => return String::new(),
    };
    line[open..close]
        .trim_start_matches([' ', '\t'])
        .trim_end_matches([' ', '\t'])
        .to_string()
}

// spec: queue-kit/SPEC.md §The tag algebra — the declaration is body-scoped by design, so it is
// read off a *continuation* line: awk's `/^[[:space:]]+roadmap-summary:/` demands the indent, and
// a column-0 spelling is not a declaration.
fn is_declaration(line: &str) -> bool {
    let body = line.trim_start_matches([' ', '\t']);
    body.len() < line.len() && body.starts_with(DECLARATION)
}

// spec: queue-kit/SPEC.md §lib/queue.sh — awk's `declared()`: the marking stripped, every
// whitespace run collapsed to one space, and the leading and trailing space that collapse leaves
// removed. The declaration is a whitelist, so this is the only text that reaches the page.
fn declared(line: &str) -> String {
    let t = line.trim_start_matches([' ', '\t']);
    let t = t.strip_prefix(DECLARATION).unwrap_or(t);
    let mut out = String::new();
    let mut ws = false;
    for c in t.chars() {
        if c == ' ' || c == '\t' {
            ws = true;
            continue;
        }
        if ws && !out.is_empty() {
            out.push(' ');
        }
        ws = false;
        out.push(c);
    }
    out
}

// spec: queue-kit/SPEC.md §lib/queue.sh — one record per live entry carrying a tag or a
// declaration, in queue order. An entry carrying neither is not a roadmap entry and is dropped
// here rather than at each caller, which is what keeps the two callers' universe identical.
pub fn roadmap_entries(text: &str, sec: &Sections) -> Vec<RoadmapEntry> {
    let mut out: Vec<RoadmapEntry> = Vec::new();
    let mut cur: Option<RoadmapEntry> = None;
    let mut inq = false;

    fn flush(cur: &mut Option<RoadmapEntry>, out: &mut Vec<RoadmapEntry>) {
        if let Some(e) = cur.take() {
            if !e.slug.is_empty() && (e.tags > 0 || e.declarations > 0) {
                out.push(e);
            }
        }
    }

    for line in text.lines() {
        if sec.is_task(line) {
            flush(&mut cur, &mut out);
            inq = true;
            continue;
        }
        if is_section_line(line) {
            flush(&mut cur, &mut out);
            inq = false;
            continue;
        }
        if !inq {
            continue;
        }
        if bullet_slug(line).is_some() {
            flush(&mut cur, &mut out);
            cur = Some(RoadmapEntry {
                tags: tag_count(line),
                field: tag_field(line),
                slug: first_bold_slug(line).unwrap_or_default().to_string(),
                declarations: 0,
                summary: String::new(),
            });
            continue;
        }
        if let Some(e) = cur.as_mut() {
            if !e.slug.is_empty() && is_declaration(line) {
                e.declarations += 1;
                if e.declarations == 1 {
                    e.summary = declared(line);
                }
            }
        }
    }
    flush(&mut cur, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_heading_matches_only_with_trailing_space_slack() {
        assert_eq!(heading_name("## Deferred"), Some("Deferred"));
        assert_eq!(heading_name("## Deferred   "), Some("Deferred"));
        assert_eq!(heading_name("## Deferred x"), Some("Deferred x"));
        assert_eq!(heading_name("### Deferred"), None);
        assert_eq!(heading_name("#Deferred"), None);
    }

    #[test]
    fn a_bold_slug_is_lowercase_kebab_and_leftmost() {
        assert_eq!(bullet_slug("- **the-slug** — prose"), Some("the-slug"));
        assert_eq!(bullet_slug("  - **a1**"), Some("a1"));
        assert_eq!(bullet_slug("- **Bad** — prose"), None);
        assert_eq!(bullet_slug("- plain"), None);
        assert_eq!(bullet_slug("-**no-space**"), None);
        assert_eq!(first_bold_slug("x **one** y **two**"), Some("one"));
    }

    #[test]
    fn the_icebox_matcher_is_inert_when_the_knob_is_unset() {
        let mut s = Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
            done: "Done".into(),
        };
        assert!(!s.is_icebox("## Icebox"));
        assert_eq!(s.task_sections(), vec!["New Features", "Deferred"]);
        s.icebox = "Icebox".into();
        assert!(s.is_icebox("## Icebox"));
        assert_eq!(
            s.task_sections(),
            vec!["New Features", "Deferred", "Icebox"]
        );
    }

    #[test]
    fn live_slugs_are_scoped_to_the_task_sections() {
        let sec = Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
            done: "Done".into(),
        };
        let text = "## New Features\n- **a** x\n## Done\n- b\n## Deferred\n- **c** y\n";
        assert_eq!(live_slugs(text, &sec), vec!["a".to_string(), "c".to_string()]);
    }

    // spec: queue-kit/SPEC.md §check-task-conservation — the done grammar is bare-slug-only, so
    // an entry relocated with its live shape intact lands in neither set and reds as a loss
    #[test]
    fn a_done_slug_is_a_bare_bullet_and_nothing_else() {
        assert_eq!(bare_bullet_slug("- the-slug"), Some("the-slug"));
        assert_eq!(bare_bullet_slug("  -   a1  "), Some("a1"));
        assert_eq!(bare_bullet_slug("- **the-slug**"), None);
        assert_eq!(bare_bullet_slug("- the-slug — prose"), None);
        assert_eq!(bare_bullet_slug("- the-slug [tag]"), None);
        assert_eq!(bare_bullet_slug("- -leading-dash"), None);
        assert_eq!(bare_bullet_slug("-nospace"), None);
        assert_eq!(bare_bullet_slug("- "), None);
    }

    // spec: queue-kit/SPEC.md §The tag algebra — every tag on the line, in order, and a tag with
    // no slug after the colon contributes nothing rather than swallowing the next one
    #[test]
    fn every_blocked_by_tag_on_a_line_is_read_in_order() {
        assert_eq!(blocked_by("- **a** [blocked-by: b] — x"), vec!["b"]);
        assert_eq!(
            blocked_by("- **a** [blocked-by: b] [blocked-by:c2] — x"),
            vec!["b", "c2"]
        );
        assert_eq!(blocked_by("- **a** — no tag"), Vec::<&str>::new());
        assert_eq!(blocked_by("[blocked-by: ] [blocked-by: d]"), vec!["d"]);
    }

    // spec: queue-kit/SPEC.md §The queue format — the counted unit is the column-0 bullet, so an
    // indented sub-task bullet is body to every reader of this adapter
    #[test]
    fn a_top_level_bullet_is_the_column_zero_one() {
        assert!(is_top_level_bullet("- **a** — x"));
        assert!(is_top_level_bullet("-\tx"));
        assert!(!is_top_level_bullet("  - **a** — x"));
        assert!(!is_top_level_bullet("-nospace"));
        assert!(!is_top_level_bullet(""));
    }

    #[test]
    fn done_slugs_are_scoped_to_the_done_section() {
        let sec = Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
            done: "Done".into(),
        };
        let text = "## Done\n- a\n- b\n## New Features\n- c\n- **d** x\n";
        assert_eq!(done_slugs(text, &sec), vec!["a".to_string(), "b".to_string()]);
    }
}
