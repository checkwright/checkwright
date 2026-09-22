// spec: queue-kit/SPEC.md §The shared queue adapters — the sole holder of the queue's shared
// surface: the derived section matchers, the slug adapters and the roadmap parse
// spec: gate-sdk/SPEC.md §lib/gate.sh — the knob read has exactly one implementation in the
// crate, `walk`'s; these two are the queue-kit-facing spelling of it
use crate::programs;
pub fn knob_array(name: &str) -> Result<Vec<String>, String> {
    crate::walk::knob_array(name)
}

pub fn knob_scalar(name: &str) -> Result<String, String> {
    crate::walk::knob_scalar(name)
}

// spec: queue-kit/SPEC.md §The shared queue adapters — the required set composed in the reader:
// a configured icebox joins it unless already named, so a consumer setting the list keeps the append
pub fn required_sections() -> Result<Vec<String>, String> {
    let mut out = knob_array("QUEUE_KIT_REQUIRED_SECTIONS")?;
    let icebox = knob_scalar("QUEUE_KIT_ICEBOX_SECTION")?;
    if !icebox.is_empty() && !out.contains(&icebox) {
        out.push(icebox);
    }
    Ok(out)
}

// spec: queue-kit/SPEC.md §The shared queue adapters — the section vocabulary every derived
// matcher below is computed from, resolved once per invocation
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

    // spec: queue-kit/SPEC.md §The shared queue adapters — these two take a section *name*, the
    // `Entry::section` an entry walk carries, rather than a heading line
    pub fn is_deferred(&self, name: &str) -> bool {
        name == self.deferred
    }

    // spec: queue-kit/SPEC.md §The icebox tier — an unset knob leaves a matcher nothing can
    // match, so every icebox reader degrades to "no icebox" rather than to "every section"
    pub fn is_icebox(&self, name: &str) -> bool {
        !self.icebox.is_empty() && name == self.icebox
    }

    pub fn is_done(&self, line: &str) -> bool {
        heading_name(line).map(|n| n == self.done).unwrap_or(false)
    }
}

// spec: queue-kit/SPEC.md §The shared queue adapters — QUEUE_SECTION_RE is `^## ` and nothing more: a
// prefix test, so a heading with trailing content still closes the section it ends
pub fn is_section_line(line: &str) -> bool {
    line.starts_with("## ")
}

// spec: queue-kit/SPEC.md §The shared queue adapters — the `^## <name>[[:space:]]*$` shape the derived
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

// spec: queue-kit/SPEC.md §The queue format — `[a-z0-9][a-z0-9-]*`, the whole string
pub fn is_slug(s: &str) -> bool {
    let b = s.as_bytes();
    !b.is_empty() && is_slug_head(b[0]) && b.iter().all(|c| is_slug_byte(*c))
}

// spec: queue-kit/SPEC.md §The queue format — an ATX heading's level: one to six `#` then a space
pub fn heading_level(line: &str) -> Option<usize> {
    let n = line.bytes().take_while(|b| *b == b'#').count();
    if (1..=6).contains(&n) && matches!(line.as_bytes().get(n), Some(b' ') | Some(b'\t')) {
        Some(n)
    } else {
        None
    }
}

// spec: queue-kit/SPEC.md §The queue format — `### <slug>` opens an entry and `#### <slug>` a
// sub-task; the heading's whole text is the slug, trailing whitespace the only slack
pub fn entry_heading(line: &str) -> Option<(usize, &str)> {
    let level = heading_level(line).filter(|l| *l == 3 || *l == 4)?;
    let slug = line[level + 1..].trim_matches([' ', '\t']);
    if is_slug(slug) {
        Some((level, slug))
    } else {
        None
    }
}

// spec: queue-kit/SPEC.md §The queue format — the tag line: one or more bracketed tags separated by
// spaces and nothing else; a bracket followed by `(` is a link and never a tag
pub fn is_tag_line(line: &str) -> bool {
    let mut rest = line.trim_matches([' ', '\t']);
    if rest.is_empty() {
        return false;
    }
    while !rest.is_empty() {
        let Some(body) = rest.strip_prefix('[') else { return false };
        let Some(close) = body.find(']') else { return false };
        if body[..close].contains('[') || close == 0 || !is_slug_head(body.as_bytes()[0]) {
            return false;
        }
        let after = &body[close + 1..];
        let next = after.trim_start_matches([' ', '\t']);
        if !next.is_empty() && next.len() == after.len() {
            return false;
        }
        rest = next;
    }
    true
}

fn is_rule(line: &str) -> bool {
    line.trim_end_matches([' ', '\t']) == "---"
}

// spec: queue-kit/SPEC.md §The queue format — one entry as the heading grammar reads it: 0-based
// line indices, `end` exclusive, the extent running to the next heading of the same or a shallower
// level, a `---` rule, or the end of the file
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub level: usize,
    pub slug: String,
    pub section: String,
    pub start: usize,
    pub end: usize,
    pub tag_line: Option<usize>,
}

impl Entry {
    // spec: queue-kit/SPEC.md §The queue format — the tag line's text, empty for an untagged entry
    pub fn tags<'a>(&self, lines: &[&'a str]) -> &'a str {
        self.tag_line.map(|i| lines[i]).unwrap_or("")
    }

    // spec: queue-kit/SPEC.md §The queue format — the body: every line of the extent after the
    // heading and the tag line, sub-task lines included
    pub fn body_lines(&self) -> impl Iterator<Item = usize> + '_ {
        (self.start + 1..self.end).filter(move |i| Some(*i) != self.tag_line)
    }
}

// spec: queue-kit/SPEC.md §The shared queue adapters — the extent of the heading at `start`
pub fn extent_end(lines: &[&str], start: usize, level: usize) -> usize {
    (start + 1..lines.len())
        .find(|&i| heading_level(lines[i]).is_some_and(|l| l <= level) || is_rule(lines[i]))
        .unwrap_or(lines.len())
}

// spec: queue-kit/SPEC.md §The shared queue adapters — every entry and sub-task heading in the task
// sections, in file order, each with its extent and tag line
pub fn entries(lines: &[&str], sec: &Sections) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut section: Option<String> = None;
    for (i, line) in lines.iter().enumerate() {
        if is_section_line(line) {
            section = if sec.is_task(line) { heading_name(line).map(str::to_string) } else { None };
            continue;
        }
        let Some(name) = section.as_ref() else { continue };
        let Some((level, slug)) = entry_heading(line) else { continue };
        let end = extent_end(lines, i, level);
        let tag_line = (i + 1..end)
            .find(|&j| !lines[j].trim().is_empty())
            .filter(|&j| heading_level(lines[j]).is_none() && is_tag_line(lines[j]));
        out.push(Entry {
            level,
            slug: slug.to_string(),
            section: name.clone(),
            start: i,
            end,
            tag_line,
        });
    }
    out
}

// spec: queue-kit/SPEC.md §The tag algebra — the `[recurrence:]` array's dates, in order; its
// count is the number of re-filings and its last element the newest
pub fn recurrence_dates(tag_line: &str) -> Vec<&str> {
    field_tags(tag_line, "recurrence")
        .first()
        .map(|t| t.raw.split(',').map(str::trim).filter(|d| is_iso_date(d)).collect())
        .unwrap_or_default()
}

pub fn is_iso_date(tok: &str) -> bool {
    let b = tok.as_bytes();
    b.len() == 10
        && b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b'-',
            _ => c.is_ascii_digit(),
        })
}

// spec: queue-kit/SPEC.md §The shared queue adapters — one markdown link on a line: its target's
// path half, its fragment half, and the byte offsets of the fragment and of the closing paren
pub struct Link<'a> {
    pub path: &'a str,
    pub frag: &'a str,
    pub frag_start: usize,
    pub close: usize,
}

// spec: queue-kit/SPEC.md §The shared queue adapters — every `](<target>)` on a line, the target
// split at its first `#`; a target holding whitespace is no link
pub fn md_links(line: &str) -> Vec<Link<'_>> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = line[from..].find("](") {
        let t = from + rel + 2;
        let Some(len) = line[t..].find(')') else { break };
        let target = &line[t..t + len];
        from = t + len + 1;
        if target.contains(char::is_whitespace) {
            continue;
        }
        let (path, frag, frag_start) = match target.find('#') {
            Some(h) => (&target[..h], &target[h + 1..], t + h + 1),
            None => (target, "", t + len),
        };
        out.push(Link { path, frag, frag_start, close: t + len });
    }
    out
}

// spec: queue-kit/SPEC.md §The tag algebra — the same-file link to an entry, `](#<slug>)`: the byte
// span of each slug-shaped fragment, beside `backtick_slugs`
pub fn link_slugs(line: &str) -> Vec<(usize, usize)> {
    md_links(line)
        .into_iter()
        .filter(|l| l.path.is_empty() && is_slug(l.frag))
        .map(|l| (l.frag_start, l.frag_start + l.frag.len()))
        .collect()
}

// spec: queue-kit/SPEC.md §The shared queue adapters — the retired bullet grammar's lead,
// `^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*`, read by the history walk and the migration
// arm alone
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

// spec: queue-kit/SPEC.md §The tag algebra — awk's `/^-[[:space:]]/`, the column-0 bullet a lesson is
pub fn is_top_level_bullet(line: &str) -> bool {
    let b = line.as_bytes();
    matches!(b.first(), Some(&c) if c == b'-')
        && matches!(b.get(1), Some(&c) if c == b' ' || c == b'\t')
}

// spec: queue-kit/SPEC.md §The tag algebra — every `[blocked-by: <slug>]` on a line, in order
// spec: queue-kit/SPEC.md §The shared queue adapters — a shared adapter because the grammar has two readers:
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

// spec: queue-kit/SPEC.md §The tag algebra — the cost tag's closed value set, recurrence before
// magnitude; kit mechanism, because both partitions its readers take are kit rules
pub const COST_RECURRENCES: &[&str] = &["session", "iteration", "event", "once"];
pub const COST_MAGNITUDES: &[&str] = &["high", "low"];

pub fn cost_class_valid(value: &str) -> bool {
    match value.split_once('/') {
        Some((r, m)) => COST_RECURRENCES.contains(&r) && COST_MAGNITUDES.contains(&m),
        None => false,
    }
}

// spec: queue-kit/SPEC.md §The tag algebra — a surface value names one top-level root entry, so a
// path separator or a self-reference is never one
pub fn surface_value_shaped(value: &str) -> bool {
    !value.is_empty() && value != "." && value != ".." && !value.contains('/')
}

// spec: queue-kit/SPEC.md §The tag algebra — `value` is present only for the fixed spelling, one
// space after the colon and no whitespace inside, which is what bounds the tag's width
pub struct FieldTag<'a> {
    pub start: usize,
    pub end: usize,
    pub raw: &'a str,
    pub value: Option<&'a str>,
}

// spec: queue-kit/SPEC.md §The shared queue adapters — every `[<name>:…]` token on a line, in order; shared,
// because the board-tags gate asserts on the tags and the wrap gate discounts them, and a second
// parse is what would let the two disagree
pub fn field_tags<'a>(line: &'a str, name: &str) -> Vec<FieldTag<'a>> {
    let open = format!("[{}:", name);
    let mut out: Vec<FieldTag<'a>> = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = line[from..].find(&open) {
        let start = from + rel;
        let body = start + open.len();
        let close = match line[body..].find(']') {
            Some(c) => c,
            None => break,
        };
        let end = body + close + 1;
        let raw = &line[body..end - 1];
        let value = raw
            .strip_prefix(' ')
            .filter(|v| !v.is_empty() && !v.contains(char::is_whitespace) && !v.contains('['));
        out.push(FieldTag { start, end, raw, value });
        from = end;
    }
    out
}

// spec: queue-kit/SPEC.md §Layout and configuration — the unit a record size is measured in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Cp,
    Lines,
}

impl Unit {
    pub fn suffix(self) -> &'static str {
        match self {
            Unit::Cp => "cp",
            Unit::Lines => "lines",
        }
    }
}

// spec: queue-kit/SPEC.md §Layout and configuration — `<n>cp`, `<n>lines`, or `off` (`None`), with
// `<n>` a positive integer; anything else is malformed config. Shared by the knob validator and
// every reader, so the grammar the table admits is the grammar the readers parse.
pub fn parse_size(v: &str) -> Result<Option<(usize, Unit)>, String> {
    if v == "off" {
        return Ok(None);
    }
    let (digits, unit) = if let Some(d) = v.strip_suffix("cp") {
        (d, Unit::Cp)
    } else if let Some(d) = v.strip_suffix("lines") {
        (d, Unit::Lines)
    } else {
        return Err(format!("'{}' is not <n>cp, <n>lines or off", v));
    };
    match digits.parse::<usize>() {
        Ok(n) if n > 0 && digits.bytes().all(|b| b.is_ascii_digit()) => Ok(Some((n, unit))),
        _ => Err(format!("'{}' is not <n>cp, <n>lines or off", v)),
    }
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

// spec: queue-kit/SPEC.md §The shared queue adapters — queue_live_slugs: every entry and sub-task
// heading in a task section, in file order
pub fn live_slugs(text: &str, sec: &Sections) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    entries(&lines, sec).into_iter().map(|e| e.slug).collect()
}

// spec: queue-kit/SPEC.md §The shared queue adapters — the retired set, derived from the queue
// file's own history; every degradation §The queue-edges arm declares yields the empty set
pub fn retired_set(file: &str, live: &[String]) -> Vec<String> {
    if !crate::proc::on_path(&programs::GIT) {
        return Vec::new();
    }
    let path = std::path::Path::new(file);
    let dir = match path.parent().map(|p| p.to_string_lossy().into_owned()) {
        Some(d) if !d.is_empty() => d,
        _ => ".".to_string(),
    };
    let Some(base) = path.file_name().map(|b| b.to_string_lossy().into_owned()) else {
        return Vec::new();
    };
    match crate::proc::run(&programs::GIT, &["-C", &dir, "rev-parse", "--is-inside-work-tree"]) {
        Ok(c) if c.stdout().is_some() => {}
        _ => return Vec::new(),
    }
    let log = match crate::proc::run(&programs::GIT, &["-C", &dir, "log", "-p", "--format=", "--", &base]) {
        Ok(c) => match c.stdout() {
            Some(o) => String::from_utf8_lossy(o).into_owned(),
            None => return Vec::new(),
        },
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<String> = Vec::new();
    for line in log.lines() {
        // spec: queue-kit/SPEC.md §The queue-edges arm — one strip takes the diff column off
        // added, removed and context lines alike, so an entry lead counts wherever the walk meets
        // it; history holds both entry grammars, the heading and the retired bullet.
        let mut chars = line.chars();
        chars.next();
        let s = chars.as_str();
        if let Some(g) = entry_heading(s).map(|(_, g)| g).or_else(|| bullet_slug(s)) {
            if !live.iter().any(|l| l == g) && !out.iter().any(|r| r == g) {
                out.push(g.to_string());
            }
        }
    }
    out
}

// spec: queue-kit/SPEC.md §The shared queue adapters — the body-position citation token: a
// backticked kebab run, returned as the byte span of the slug between its backticks; resolution
// decides the rest
pub fn backtick_slugs(line: &str) -> Vec<(usize, usize)> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] != b'`' {
            i += 1;
            continue;
        }
        let start = i + 1;
        if start >= b.len() || !is_slug_head(b[start]) {
            i += 1;
            continue;
        }
        let mut j = start + 1;
        while j < b.len() && is_slug_byte(b[j]) {
            j += 1;
        }
        if j < b.len() && b[j] == b'`' {
            out.push((start, j));
            i = j + 1;
        } else {
            i += 1;
        }
    }
    out
}

// spec: queue-kit/SPEC.md §The queue-edges arm — the name-live test's input is the tracked tree
// listing and not a curated roster; an absent `git` or a queue file outside a work tree marks
// nothing, the same direction the retired set's own degradations take
pub fn tracked_stems(file: &str) -> Vec<(String, String)> {
    if !crate::proc::on_path(&programs::GIT) {
        return Vec::new();
    }
    let path = std::path::Path::new(file);
    let dir = match path.parent().map(|p| p.to_string_lossy().into_owned()) {
        Some(d) if !d.is_empty() => d,
        _ => ".".to_string(),
    };
    let listing =
        match crate::proc::run(&programs::GIT, &["-C", &dir, "ls-files", "--full-name", "--", ":/"]) {
            Ok(c) => match c.stdout() {
                Some(o) => String::from_utf8_lossy(o).into_owned(),
                None => return Vec::new(),
            },
            Err(_) => return Vec::new(),
        };
    listing
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            std::path::Path::new(l)
                .file_stem()
                .map(|s| (s.to_string_lossy().into_owned(), l.to_string()))
        })
        .collect()
}

// spec: queue-kit/SPEC.md §The queue-edges arm — every tracked path whose own stem is the slug, in
// tracked order
pub fn name_live_at<'a>(stems: &'a [(String, String)], slug: &str) -> Vec<&'a str> {
    stems
        .iter()
        .filter(|(stem, _)| stem == slug)
        .map(|(_, path)| path.as_str())
        .collect()
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
// section, in file order
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

// spec: queue-kit/SPEC.md §The shared queue adapters — the one [roadmap:] + [roadmap-summary:] parse, shared
// by the roadmap arm and check-roadmap-fresh so the two never disagree on what an entry claims
pub struct RoadmapEntry {
    pub tags: usize,
    pub field: String,
    pub slug: String,
    pub summaries: usize,
    pub summary: String,
}

const TAG_OPEN: &str = "[roadmap:";

// spec: queue-kit/SPEC.md §The shared queue adapters — awk's non-overlapping `while (match(s, /\[roadmap:/))`
fn tag_count(line: &str) -> usize {
    line.matches(TAG_OPEN).count()
}

// spec: queue-kit/SPEC.md §The shared queue adapters — awk's `/\[roadmap:[^]]*\]/`: the leftmost `[roadmap:`
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

// spec: queue-kit/SPEC.md §The shared queue adapters — every whitespace run collapsed to one space and
// the ends trimmed. The summary is a whitelist, so this is the only text that reaches the page.
fn collapsed(t: &str) -> String {
    t.split_whitespace().collect::<Vec<_>>().join(" ")
}

// spec: queue-kit/SPEC.md §The shared queue adapters — one record per live entry carrying a
// `[roadmap:]` or a `[roadmap-summary:]` tag, in queue order, both read off the tag line. An entry
// carrying neither is dropped here rather than at each caller, keeping the callers' universe one.
pub fn roadmap_entries(text: &str, sec: &Sections) -> Vec<RoadmapEntry> {
    let lines: Vec<&str> = text.lines().collect();
    entries(&lines, sec)
        .into_iter()
        .filter_map(|e| {
            let t = e.tags(&lines);
            let sums = field_tags(t, "roadmap-summary");
            let r = RoadmapEntry {
                tags: tag_count(t),
                field: tag_field(t),
                slug: e.slug,
                summaries: sums.len(),
                summary: sums.first().map(|s| collapsed(s.raw)).unwrap_or_default(),
            };
            (r.tags > 0 || r.summaries > 0).then_some(r)
        })
        .collect()
}

// spec: queue-kit/SPEC.md §The queue format — the first `<label><iso-day>` on a line; the label is
// case-sensitive and the date must follow it directly, so a reflow or a word between the two reads
// as no mark
fn marked_date(line: &str, label: &str) -> Option<String> {
    let mut from = 0usize;
    while let Some(rel) = line[from..].find(label) {
        let at = from + rel;
        let p = at + label.len();
        if p + 10 <= line.len() {
            let d = &line[p..p + 10];
            if d.bytes().enumerate().all(|(i, c)| match i {
                4 | 7 => c == b'-',
                _ => c.is_ascii_digit(),
            }) {
                return Some(d.to_string());
            }
        }
        from = at + 1;
    }
    None
}

// spec: queue-kit/SPEC.md §The queue format — the defer date, one parse shared by the icebox
// candidates arm and check-queue-entry-budget's assertion (E): fed an entry's body lines, it
// resolves the first `Surfaced` mark, else the first `Filed` one
#[derive(Default)]
pub struct DeferMarks {
    surfaced: Option<String>,
    filed: Option<String>,
}

impl DeferMarks {
    pub fn observe(&mut self, line: &str) {
        if self.surfaced.is_none() {
            self.surfaced = marked_date(line, "Surfaced ");
        }
        if self.filed.is_none() {
            self.filed = marked_date(line, "Filed ");
        }
    }

    pub fn defer_date(&self) -> Option<&str> {
        self.surfaced.as_deref().or(self.filed.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: queue-kit/SPEC.md §The queue format — Surfaced outranks Filed wherever each sits, and the
    // spellings the definition does not name (lowercase, a word between, a wrapped date) resolve none
    #[test]
    fn the_defer_date_is_surfaced_else_filed_and_nothing_else() {
        let mut m = DeferMarks::default();
        m.observe("  Filed 2026-07-07 by close.");
        assert_eq!(m.defer_date(), Some("2026-07-07"));
        m.observe("  Surfaced 2026-08-01 at build.");
        assert_eq!(m.defer_date(), Some("2026-08-01"));
        for miss in ["  filed 2026-08-18 by close", "  Filed at build 2026-08-18", "  body ending Filed", "  2026-08-24 by close."] {
            let mut m = DeferMarks::default();
            m.observe(miss);
            assert_eq!(m.defer_date(), None, "{}", miss);
        }
    }

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
    }

    #[test]
    fn the_icebox_matcher_is_inert_when_the_knob_is_unset() {
        let mut s = Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
            done: "Done".into(),
        };
        assert!(!s.is_icebox("Icebox") && !s.is_icebox(""));
        assert_eq!(s.task_sections(), vec!["New Features", "Deferred"]);
        s.icebox = "Icebox".into();
        assert!(s.is_icebox("Icebox"));
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
        let text = "## New Features\n### a\nx\n#### a-sub\n## Done\n- b\n### d\n## Deferred\n### c\n";
        assert_eq!(live_slugs(text, &sec), vec!["a".to_string(), "a-sub".to_string(), "c".to_string()]);
    }

    // spec: queue-kit/SPEC.md §The queue format — the heading is the bare slug, three or four hashes
    #[test]
    fn an_entry_heading_is_a_bare_slug_at_level_three_or_four() {
        assert_eq!(entry_heading("### the-slug"), Some((3, "the-slug")));
        assert_eq!(entry_heading("#### sub-1  "), Some((4, "sub-1")));
        assert_eq!(entry_heading("### Someday"), None);
        assert_eq!(entry_heading("### a [tag]"), None);
        assert_eq!(entry_heading("## a"), None);
        assert_eq!(entry_heading("##### a"), None);
        assert_eq!(heading_level("###x"), None);
    }

    // spec: queue-kit/SPEC.md §The queue format — a tag line holds bracketed tags and nothing else
    #[test]
    fn a_tag_line_is_only_bracketed_tags() {
        assert!(is_tag_line("[cost: event/low] [surface: queue-kit]"));
        assert!(is_tag_line("[attend]"));
        assert!(is_tag_line("[roadmap-summary: One sentence, with commas.]"));
        assert!(!is_tag_line("[link](#x) prose"));
        assert!(!is_tag_line("[cost: event/low] and prose"));
        assert!(!is_tag_line("[a][b]"));
        assert!(!is_tag_line(""));
        assert!(!is_tag_line("[Upper: x]"));
    }

    // spec: queue-kit/SPEC.md §The queue format — the extent runs to the next heading of the same or
    // a shallower level, and the tag line is the first non-blank line under the heading
    #[test]
    fn an_entry_carries_its_extent_and_tag_line() {
        let sec = Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: String::new(),
            done: "Done".into(),
        };
        let text = "## New Features\n\n### a\n\n[spec: x.md]\n\nbody\n\n#### a-sub\n\nsub body\n\n### b\n\nno tags\n## Done\n";
        let lines: Vec<&str> = text.lines().collect();
        let es = entries(&lines, &sec);
        assert_eq!(es.len(), 3);
        assert_eq!((es[0].start, es[0].end, es[0].tag_line), (2, 12, Some(4)));
        assert_eq!(es[0].tags(&lines), "[spec: x.md]");
        assert_eq!((es[1].level, es[1].start, es[1].end, es[1].tag_line), (4, 8, 12, None));
        assert_eq!((es[2].start, es[2].end, es[2].tag_line), (12, 15, None));
    }

    // spec: queue-kit/SPEC.md §The tag algebra — the recurrence array, count and last element
    #[test]
    fn the_recurrence_array_reads_every_date_in_order() {
        assert_eq!(recurrence_dates("[cost: once/low] [recurrence: 2026-09-03, 2026-09-10]"), vec!["2026-09-03", "2026-09-10"]);
        assert!(recurrence_dates("[recurrence: soon]").is_empty());
        assert!(recurrence_dates("[cost: once/low]").is_empty());
    }

    // spec: queue-kit/SPEC.md §The tag algebra — a same-file link's slug-shaped fragment
    #[test]
    fn a_link_slug_is_a_same_file_slug_fragment() {
        let l = "see [a-b](#a-b), [x](other.md#c), [y](#Not-Slug) and [z](#d)";
        let got: Vec<&str> = link_slugs(l).into_iter().map(|(s, e)| &l[s..e]).collect();
        assert_eq!(got, vec!["a-b", "d"]);
        let m = md_links("[q](../TASK-QUEUE.md#e-f) (deferred)");
        assert_eq!((m[0].path, m[0].frag), ("../TASK-QUEUE.md", "e-f"));
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
