// spec: gate-sdk/SPEC-queue-cohort.md — the Rust counterpart of queue-kit/lib/queue.sh's
// shared surface: the derived section matchers and the slug adapters. The shell library is
// not retired, so this module sits beside it rather than replacing it
// spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged value, tab-split. The crate holds no
// default for a bridged knob, so an absent variable is an error rather than a fallback;
// an empty one is a resolved-empty value, which is why the two part company here.
pub fn knob_array(name: &str) -> Result<Vec<String>, String> {
    let raw = std::env::var(format!("GATE_SDK_KNOB_{}", name)).map_err(|_| {
        format!(
            "GATE_SDK_KNOB_{} is unset — the gate was invoked without the config bridge \
             gate_command emits, so {} could not be resolved",
            name, name
        )
    })?;
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    Ok(raw.split('\t').map(String::from).collect())
}

pub fn knob_scalar(name: &str) -> Result<String, String> {
    let raw = std::env::var(format!("GATE_SDK_KNOB_{}", name)).map_err(|_| {
        format!(
            "GATE_SDK_KNOB_{} is unset — the gate was invoked without the config bridge \
             gate_command emits, so {} could not be resolved",
            name, name
        )
    })?;
    Ok(raw)
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
    Some(rest.trim_end_matches(|c: char| c == ' ' || c == '\t'))
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
}
