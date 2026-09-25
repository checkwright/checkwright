// spec: queue-kit/SPEC.md §The queue-migrate arm — `--emit queue-migrate [--write] <file>`: the
// retired bullet grammar rewritten into the heading grammar, refused whole when the rewrite would
// lose a slug, a tag or a done line
use crate::queue::{self, Sections};

pub const KNOBS: &[&str] = &[
    "QUEUE_KIT_ACTIVE_SECTIONS",
    "QUEUE_KIT_DEFERRED_SECTION",
    "QUEUE_KIT_ICEBOX_SECTION",
    "QUEUE_KIT_DONE_SECTION",
];

pub const USAGE: &str = "usage: --emit queue-migrate [--write] <file>";

// spec: queue-kit/SPEC.md §The queue-migrate arm — the three body declarations that become tags
const DECLARATIONS: [&str; 3] = ["recurrence:", "roadmap-summary:", "not-icebox-eligible:"];

pub fn emit(args: &[String]) -> Result<String, String> {
    let write = args.first().is_some_and(|a| a == "--write");
    let files: Vec<&String> = args.iter().skip(usize::from(write)).collect();
    if files.len() != 1 || files[0].starts_with("--") {
        return Err(USAGE.to_string());
    }
    let f = files[0];
    let sec = Sections::with_done()?;
    let text = std::fs::read_to_string(f).map_err(|e| format!("queue-migrate: cannot read {}: {}", f, e))?;
    let out = convert(&text, &sec).map_err(|e| format!("queue-migrate: {}: {}", f, e))?;
    if !write {
        return Ok(out);
    }
    if out != text {
        std::fs::write(f, &out).map_err(|e| format!("queue-migrate: cannot write {}: {}", f, e))?;
        if std::fs::read_to_string(f).ok().as_deref() != Some(out.as_str()) {
            return Err(format!("queue-migrate: {}: the written text does not read back", f));
        }
    }
    Ok(String::new())
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — a bracketed tag as the lead line wrote it: a
// lowercase name, then `]` or `:`, closed before any nested bracket and not opening a link
fn tag_spans(line: &str) -> Vec<(usize, usize)> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] != b'[' {
            i += 1;
            continue;
        }
        let name_end = (i + 1..b.len())
            .find(|&j| !(b[j].is_ascii_lowercase() || b[j].is_ascii_digit() || b[j] == b'-'))
            .unwrap_or(b.len());
        let named = name_end > i + 1 && b[i + 1].is_ascii_lowercase();
        let close = line[i + 1..].find([']', '[']).map(|r| i + 1 + r);
        match close {
            Some(c) if named && b[c] == b']' && (b[name_end] == b':' || name_end == c) && b.get(c + 1) != Some(&b'(') => {
                out.push((i, c + 1));
                i = c + 1;
            }
            _ => i += 1,
        }
    }
    out
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — a declaration line's tag, the self-naming slug
// dropped and the recurrence dates joined into the array; a bracket in the value cannot be a tag
fn declaration_tag(trimmed: &str, slug: &str, ln: usize) -> Result<Option<String>, String> {
    let Some(tok) = DECLARATIONS.iter().find(|t| trimmed.starts_with(*t)) else { return Ok(None) };
    let rest = trimmed[tok.len()..].trim();
    if rest.contains('[') || rest.contains(']') {
        return Err(format!("line {}: the {} value holds a bracket, which a tag value cannot carry", ln, tok));
    }
    let name = &tok[..tok.len() - 1];
    let mut f: Vec<&str> = rest.split_whitespace().collect();
    if name != "roadmap-summary" && f.first() == Some(&slug) {
        f.remove(0);
    }
    let value = if name == "recurrence" { f.join(", ") } else { f.join(" ") };
    Ok(Some(format!("[{}: {}]", name, value)))
}

struct Block {
    level: usize,
    slug: String,
    tags: Vec<String>,
    paras: Vec<String>,
}

fn bullet_depth(line: &str) -> usize {
    if line.starts_with(['-']) {
        3
    } else {
        4
    }
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — a line opening with a bold span starts a
// paragraph only as a lead-in: its own span ends in `.` or `:`, or the open paragraph ends a sentence
fn bold_lead_in(t: &str, open: &str) -> bool {
    let Some(body) = t.strip_prefix("**") else { return false };
    if body.find("**").is_some_and(|e| body[..e].ends_with(['.', ':'])) {
        return true;
    }
    let tail = open.trim_end().trim_end_matches(['*', '`', ')', '"', '\'']);
    open.trim().is_empty() || tail.ends_with(['.', ':', '?', '!'])
}

fn is_continuation(line: &str) -> bool {
    line.starts_with([' ', '\t']) && !line.trim().is_empty() && queue::bullet_slug(line).is_none()
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — a single-backticked live slug other than the
// entry's own becomes the same-file link, the remedy check-task-names assertion R names
fn link_citations(p: &str, own: &str, live: &[String]) -> String {
    let b = p.as_bytes();
    let mut out = String::new();
    let mut last = 0usize;
    for (s, e) in queue::backtick_slugs(p) {
        let doubled = (s >= 2 && b[s - 2] == b'`') || b.get(e + 1) == Some(&b'`');
        let tok = &p[s..e];
        if doubled || tok == own || !live.iter().any(|l| l == tok) {
            continue;
        }
        out.push_str(&p[last..s - 1]);
        out.push_str(&format!("[{}](#{})", tok, tok));
        last = e + 1;
    }
    out.push_str(&p[last..]);
    out
}

fn render(b: &Block, live: &[String], out: &mut Vec<String>) {
    if out.last().is_some_and(|l| !l.is_empty()) {
        out.push(String::new());
    }
    out.push(format!("{} {}", "#".repeat(b.level), b.slug));
    out.push(String::new());
    if !b.tags.is_empty() {
        out.push(b.tags.join(" "));
        out.push(String::new());
    }
    for p in &b.paras {
        out.push(link_citations(p, &b.slug, live));
        out.push(String::new());
    }
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — the legacy live set, read by the retired grammar
fn legacy_live(lines: &[&str], sec: &Sections) -> Vec<String> {
    let mut out = Vec::new();
    let mut inq = false;
    for l in lines {
        if queue::is_section_line(l) {
            inq = sec.is_task(l);
            continue;
        }
        if inq {
            if let Some(s) = queue::bullet_slug(l) {
                out.push(s.to_string());
            }
        }
    }
    out
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — a revision written before the migration, read in
// the heading grammar: a text holding bullet entries and no entry heading is converted, so a reader
// comparing across the converting commit compares like with like; anything else reads as it stands
pub fn heading_form<'a>(text: &'a str, sec: &Sections) -> std::borrow::Cow<'a, str> {
    let lines: Vec<&str> = text.lines().collect();
    if queue::entries(&lines, sec).is_empty() && !legacy_live(&lines, sec).is_empty() {
        if let Ok(t) = convert(text, sec) {
            return std::borrow::Cow::Owned(t);
        }
    }
    std::borrow::Cow::Borrowed(text)
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — `sec` carries the done section, which the
// postcondition reads; a file holding no bullet entry is returned as it stands
pub fn convert(text: &str, sec: &Sections) -> Result<String, String> {
    let lines: Vec<&str> = text.lines().collect();
    let live = legacy_live(&lines, sec);
    if live.is_empty() {
        return Ok(text.to_string());
    }
    let mut out: Vec<String> = Vec::new();
    let mut expected: Vec<(String, Vec<String>)> = Vec::new();
    let mut in_task = false;
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        if queue::is_section_line(line) {
            in_task = sec.is_task(line);
            out.push(line.to_string());
            i += 1;
            continue;
        }
        let slug = match queue::bullet_slug(line) {
            Some(s) if in_task => s.to_string(),
            _ => {
                if !(in_task && line.trim().is_empty() && out.last().is_some_and(|l| l.is_empty())) {
                    out.push(line.to_string());
                }
                i += 1;
                continue;
            }
        };
        let lead = queue::strip_bullet_lead(line).unwrap_or(line);
        let lead = &lead[slug.len() + 4..];
        let mut tags: Vec<String> = Vec::new();
        let mut prose = String::new();
        let mut from = 0usize;
        for (s, e) in tag_spans(lead) {
            tags.push(lead[s..e].to_string());
            prose.push_str(&lead[from..s]);
            prose.push(' ');
            from = e;
        }
        prose.push_str(&lead[from..]);
        let mut paras: Vec<String> = vec![prose.split_whitespace().collect::<Vec<_>>().join(" ")];
        i += 1;
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty() {
                let next = (i + 1..lines.len()).find(|&j| !lines[j].trim().is_empty());
                if next.is_some_and(|j| is_continuation(lines[j])) {
                    paras.push(String::new());
                    i += 1;
                    continue;
                }
                break;
            }
            if !is_continuation(l) {
                break;
            }
            let t = l.trim();
            if let Some(tag) = declaration_tag(t, &slug, i + 1)? {
                tags.push(tag);
            } else if t.starts_with("- ") || t.starts_with("* ") || bold_lead_in(t, paras.last().map_or("", |p| p)) {
                paras.push(t.to_string());
            } else {
                let cur = paras.last_mut().expect("a paragraph is always open");
                if !cur.is_empty() {
                    cur.push(' ');
                }
                cur.push_str(t);
            }
            i += 1;
        }
        let paras: Vec<String> = paras
            .into_iter()
            .enumerate()
            .map(|(k, p)| if k == 0 { p.trim_start_matches(['—', ' ']).to_string() } else { p })
            .filter(|p| !p.is_empty())
            .collect();
        expected.push((slug.clone(), tags.clone()));
        render(&Block { level: bullet_depth(line), slug, tags, paras }, &live, &mut out);
    }
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }
    let mut result = out.join("\n");
    result.push('\n');
    postcondition(text, &result, &expected, sec)?;
    Ok(result)
}

// spec: queue-kit/SPEC.md §The queue-migrate arm — the shared adapters read the same live slugs, the
// same tags and the same done set after the rewrite as the retired grammar read before it
fn postcondition(before: &str, after: &str, expected: &[(String, Vec<String>)], sec: &Sections) -> Result<(), String> {
    let lines: Vec<&str> = after.lines().collect();
    let got: Vec<(String, Vec<String>)> = queue::entries(&lines, sec)
        .into_iter()
        .map(|e| {
            let t = e.tags(&lines);
            let tags = tag_spans(t).into_iter().map(|(s, x)| t[s..x].to_string()).collect();
            (e.slug, tags)
        })
        .collect();
    if got != expected {
        return Err("the rewrite would change the live slugs or their tags; nothing written".to_string());
    }
    if queue::done_slugs(before, sec) != queue::done_slugs(after, sec) {
        return Err("the rewrite would change the done set; nothing written".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sec() -> Sections {
        Sections {
            active: vec!["New Features".into()],
            deferred: "Deferred".into(),
            icebox: "Icebox".into(),
            done: "Done".into(),
        }
    }

    const LEGACY: &str = "\
# Q

## Iteration: demo

  A preamble line.

## New Features

- **alpha** [spec: SPEC-a.md] [blocked-by: beta] — build it, after `beta` and
  past `gone-slug`.
  **Deliverable:** the thing.
  - **alpha-sub** — a sub-task.

- **only-tags** [spec: SPEC-b.md]
  — the lead held only tags.

- **wrapped** — a wrap that leaves a
  **bold phrase** mid-sentence and *ends.*
  **After a sentence** still opens one. A line with no sentence end, then
  **Label:** opens one too.

## Deferred

- **beta** [cost: once/low] [surface: queue-kit] — deferred work.
  recurrence: beta 2026-01-01 2026-02-02
  roadmap-summary: One public sentence.
  not-icebox-eligible: beta 2026-03-03 ruled a defect
  **Cost while deferred:** low.
  Filed 2026-01-01 by close.

## Icebox

- **iced** — dormant one-liner.

## Done

- gone-slug

## Lessons Learned

- **lesson** [attend] — stays a bullet.
";

    const HEADINGS: &str = "\
# Q

## Iteration: demo

  A preamble line.

## New Features

### alpha

[spec: SPEC-a.md] [blocked-by: beta]

build it, after [beta](#beta) and past `gone-slug`.

**Deliverable:** the thing.

#### alpha-sub

a sub-task.

### only-tags

[spec: SPEC-b.md]

the lead held only tags.

### wrapped

a wrap that leaves a **bold phrase** mid-sentence and *ends.*

**After a sentence** still opens one. A line with no sentence end, then

**Label:** opens one too.

## Deferred

### beta

[cost: once/low] [surface: queue-kit] [recurrence: 2026-01-01, 2026-02-02] [roadmap-summary: One public sentence.] [not-icebox-eligible: 2026-03-03 ruled a defect]

deferred work.

**Cost while deferred:** low. Filed 2026-01-01 by close.

## Icebox

### iced

dormant one-liner.

## Done

- gone-slug

## Lessons Learned

- **lesson** [attend] — stays a bullet.
";

    // spec: queue-kit/SPEC.md §The queue-migrate arm — every rule the arm names, in one queue: the
    // heading and sub-task, the declarations on the tag line, a bold lead-in against a wrapped bold
    // phrase, the live citation linked, and done, lessons and preambles passing through
    #[test]
    fn a_bullet_queue_converts_to_the_heading_grammar() {
        assert_eq!(convert(LEGACY, &sec()).expect("convert"), HEADINGS);
    }

    // spec: queue-kit/SPEC.md §The queue-migrate arm — a converted queue is returned as it stands,
    // and heading_form reads a bullet revision as the conversion would write it
    #[test]
    fn the_conversion_is_idempotent_and_heading_form_applies_it() {
        assert_eq!(convert(HEADINGS, &sec()).expect("convert"), HEADINGS);
        assert_eq!(heading_form(LEGACY, &sec()), HEADINGS);
        assert_eq!(heading_form(HEADINGS, &sec()), HEADINGS);
    }

    // spec: queue-kit/SPEC.md §The queue-migrate arm — a declaration value holding a bracket cannot
    // become a tag, so the file is refused whole and the line named
    #[test]
    fn a_bracket_in_a_declaration_value_refuses_the_file() {
        let q = "## Deferred\n\n- **a** — x\n  roadmap-summary: a [bracketed] sentence\n";
        let err = convert(q, &sec()).expect_err("a bracket was carried into a tag");
        assert!(err.contains("line 4"), "{}", err);
    }
}
