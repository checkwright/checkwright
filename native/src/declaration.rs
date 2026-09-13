// spec: gate-sdk/SPEC.md §lib/declaration.sh — the declaration grammar's compiled holder, pure
// over text so the caller keeps the *missing file is the empty set* rule and this module reaches
// no filesystem
// spec: gate-sdk/SPEC.md §lib/declaration.sh — the trichotomy is a type here rather than the
// shell's status code, so a caller cannot reach a token list without having matched the
// resolved-empty arm; nothing else about the grammar changes
// spec: gate-sdk/SPEC.md §lib/declaration.sh — the grammar's only holder since the 2026-09-03 cut
// deleted the shell form: every "shell holder" below names the implementation this one was ported
// from, never a live second side to hold equal

// spec: gate-sdk/SPEC.md §lib/declaration.sh — awk's `[[:space:]]` in the C locale, as a byte
// class: a Unicode predicate would classify bytes the shell holder never treats as blank
fn is_blank(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\x0b' | b'\x0c')
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — awk's record split, which keeps a `\r` inside the
// record and processes a final unterminated line; `str::lines` strips the `\r` and would let a
// CRLF token pass here while the shell holder refused it
fn split_lines(text: &str) -> Vec<&str> {
    let mut v: Vec<&str> = text.split('\n').collect();
    if v.last() == Some(&"") {
        v.pop();
    }
    v
}

fn has_content(line: &str) -> bool {
    line.bytes().any(|b| !is_blank(b))
}

fn skip_blanks(line: &str) -> &str {
    let b = line.as_bytes();
    let mut i = 0;
    while i < b.len() && is_blank(b[i]) {
        i += 1;
    }
    &line[i..]
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — `^##[[:space:]]`: a container heading, which a
// deeper heading is not, so a subsection does not close the section being read
fn is_container_heading(line: &str) -> bool {
    let b = line.as_bytes();
    b.len() >= 3 && b[0] == b'#' && b[1] == b'#' && is_blank(b[2])
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — `^##[[:space:]]+<name>[[:space:]]*$`, with the
// name matched literally where the shell holder interpolates it into an ERE: the two agree for
// every name without an ERE metacharacter, which a fixed heading in a governed grammar is.
fn heading_names(line: &str, section: &str) -> bool {
    let rest = &line[2..];
    let named = skip_blanks(rest);
    if named.len() == rest.len() {
        return false;
    }
    match named.strip_prefix(section) {
        Some(tail) => !has_content(tail),
        None => false,
    }
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — `^[[:space:]]*[-*][[:space:]]+`, the container's
// whole predicate: what follows the marker is the token predicate's business, never this one's
fn bullet_body(line: &str) -> Option<&str> {
    let b = line.as_bytes();
    let mut i = 0;
    while i < b.len() && is_blank(b[i]) {
        i += 1;
    }
    if i >= b.len() || (b[i] != b'-' && b[i] != b'*') {
        return None;
    }
    i += 1;
    let marker_end = i;
    while i < b.len() && is_blank(b[i]) {
        i += 1;
    }
    if i == marker_end {
        return None;
    }
    line.get(i..)
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — a backticked lead is the span *directly* after the
// marker: anything else yields no token rather than a stripped one, which is what keeps the bolded
// spellings visible instead of silent
fn lead_token(line: &str) -> Option<&str> {
    let body = bullet_body(line)?;
    let inner = body.strip_prefix('`')?;
    let end = inner.find('`')?;
    inner.get(..end)
}

fn bold_lead(line: &str) -> Option<&str> {
    let body = bullet_body(line)?;
    let inner = body.strip_prefix("**")?;
    let end = inner.find("**")?;
    let b = inner.as_bytes();
    let (mut s, mut e) = (0, end);
    while s < e && is_blank(b[s]) {
        s += 1;
    }
    while e > s && is_blank(b[e - 1]) {
        e -= 1;
    }
    inner.get(s..e)
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — each declaration-bearing section's lead-token rule;
// the caller names the rule with the section, so the holder still carries no section name
#[derive(Clone, Copy)]
pub enum TokenRule {
    GateName,
    Backticked,
    Bolded,
}

fn rule_token(line: &str, rule: TokenRule) -> Option<&str> {
    let t = match rule {
        TokenRule::GateName => return lead_token(line).filter(|t| is_token(t)),
        TokenRule::Backticked => lead_token(line)?,
        TokenRule::Bolded => bold_lead(line)?,
    };
    if has_content(t) {
        Some(t)
    } else {
        None
    }
}

fn is_token(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_alphabetic() {
        return false;
    }
    b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'-')
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — an explicit `None` body, read off the first line
// carrying content inside the named container
fn opens_with_none(line: &str) -> bool {
    let t = skip_blanks(line);
    let rest = match t.strip_prefix("None").or_else(|| t.strip_prefix("none")) {
        Some(r) => r,
        None => return false,
    };
    match rest.as_bytes().first() {
        None => true,
        Some(b) => !b.is_ascii_alphanumeric(),
    }
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — the container arm alone, for the caller that needs
// bullets without the token predicate. `None` is *the section is absent*, which the shell holder
// reports as a status rather than as a value.
pub fn section_bullets<'a>(text: &'a str, section: &str) -> Option<Vec<&'a str>> {
    let mut found = false;
    let mut inside = false;
    let mut out: Vec<&str> = Vec::new();
    for line in split_lines(text) {
        if is_container_heading(line) {
            inside = heading_names(line, section);
            found |= inside;
            continue;
        }
        if inside && bullet_body(line).is_some() {
            out.push(line);
        }
    }
    if found {
        Some(out)
    } else {
        None
    }
}

fn section_is_none(text: &str, section: &str) -> bool {
    let mut found = false;
    let mut inside = false;
    let mut seen = false;
    let mut none = false;
    for line in split_lines(text) {
        if is_container_heading(line) {
            inside = heading_names(line, section);
            found |= inside;
            continue;
        }
        if inside && !seen && has_content(line) {
            seen = true;
            none = opens_with_none(line);
        }
    }
    found && none
}

// spec: gate-sdk/SPEC.md §lib/declaration.sh — the markdown arm's verdict; `Tokens` is non-empty
// by construction and `Unparsed` is the caller's finding list, empty where the container held no
// bullet at all.
// spec: gate-sdk/SPEC.md §lib/declaration.sh — `Unparsed` leads with any token the shell holder
// had already printed before it met the first offender: that holder emits tokens as it walks and
// appends the offenders, and a port reproduces the rules it ports rather than fixing them.
pub enum SectionVerdict {
    Absent,
    ExplicitNone,
    Tokens(Vec<String>),
    Unparsed(Vec<String>),
}

pub fn section_tokens(text: &str, section: &str, rule: TokenRule) -> SectionVerdict {
    let bullets = match section_bullets(text, section) {
        None => return SectionVerdict::Absent,
        Some(b) => b,
    };
    if section_is_none(text, section) {
        return SectionVerdict::ExplicitNone;
    }
    let mut tokens: Vec<String> = Vec::new();
    let mut bad: Vec<String> = Vec::new();
    for line in bullets {
        if !has_content(line) {
            continue;
        }
        match rule_token(line, rule) {
            Some(t) => tokens.push(t.to_string()),
            None => bad.push(line.to_string()),
        }
    }
    if bad.is_empty() && !tokens.is_empty() {
        return SectionVerdict::Tokens(tokens);
    }
    tokens.extend(bad);
    SectionVerdict::Unparsed(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CORPUS: &str = "\
## Alpha

- `alpha-one` — resolved.
- `alpha-two` — resolved.

### Deeper

- `alpha-three` — a deeper heading does not close the container.

## Beta

None, and the trailing clause rides the explicit empty set.

## Gamma

## Delta

- **delta-one** — bolded, so no token is read here.
- **`delta-two`** — bold-and-backticked, the same silence by a second spelling.

## Epsilon

- `epsilon-one` — readable.
- see `epsilon-two` — a lead token not directly after the marker.

## Zeta

Prose only: not `None`, and no bullet at all, so the parse resolves to an empty
set the section contradicts.

## Eta

- `eta-one` — resolved.

None of the above reaches a vendored tree that shadows it.
";

    fn gate_tokens(section: &str) -> SectionVerdict {
        section_tokens(CORPUS, section, TokenRule::GateName)
    }

    #[test]
    fn the_container_arm_reports_absence_as_a_value_and_collects_across_subheadings() {
        assert!(section_bullets(CORPUS, "Missing").is_none());
        assert_eq!(section_bullets(CORPUS, "Alpha").expect("absent").len(), 3);
        assert_eq!(section_bullets(CORPUS, "Gamma").expect("absent").len(), 0);
    }

    #[test]
    fn every_arm_of_the_trichotomy_is_reachable_and_distinct() {
        assert!(matches!(
            gate_tokens("Missing"),
            SectionVerdict::Absent
        ));
        assert!(matches!(
            gate_tokens("Beta"),
            SectionVerdict::ExplicitNone
        ));
        match gate_tokens("Alpha") {
            SectionVerdict::Tokens(t) => assert_eq!(t, vec!["alpha-one", "alpha-two", "alpha-three"]),
            _ => panic!("a resolving container did not report tokens"),
        }
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — both spellings the corpus actually carried,
        // bolded and bold-and-backticked, refused in one container rather than one at a time.
        match gate_tokens("Delta") {
            SectionVerdict::Unparsed(b) => {
                assert!(b[0].starts_with("- **delta-one"));
                assert!(b[1].starts_with("- **`delta-two`"));
                assert_eq!(b.len(), 2);
            }
            _ => panic!("an unreadable lead token did not report as unparsed"),
        }
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — the preserved conflation: a bare token
        // first, a whole bullet line second.
        match gate_tokens("Epsilon") {
            SectionVerdict::Unparsed(b) => {
                assert_eq!(b[0], "epsilon-one");
                assert!(b[1].starts_with("- see "));
                assert_eq!(b.len(), 2);
            }
            _ => panic!("a container mixing a readable and an unreadable bullet resolved"),
        }
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — the silently-empty declaration the
        // grammar refuses, carrying no offending line.
        match gate_tokens("Gamma") {
            SectionVerdict::Unparsed(b) => assert!(b.is_empty()),
            _ => panic!("an empty container resolved instead of refusing"),
        }
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — a prose-only container is the same refusal
        // by a different route: content, but no bullet and no `None` body, so the empty offender
        // list is what the container's own text contradicts.
        match gate_tokens("Zeta") {
            SectionVerdict::Unparsed(b) => assert!(b.is_empty()),
            _ => panic!("a prose-only container resolved instead of refusing"),
        }
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — `None` is read off the container's first
        // line carrying content, so a later line opening with the word leaves a real declaration
        // resolving rather than emptying it.
        match gate_tokens("Eta") {
            SectionVerdict::Tokens(t) => assert_eq!(t, vec!["eta-one"]),
            _ => panic!("a later `None` line emptied a resolving container"),
        }
    }

    #[test]
    fn a_lead_token_is_read_only_directly_after_the_marker() {
        assert_eq!(lead_token("- `alpha-one` — x"), Some("alpha-one"));
        assert_eq!(lead_token("  * `alpha-one`"), Some("alpha-one"));
        assert_eq!(lead_token("- **`alpha-one`** — x"), None);
        assert_eq!(lead_token("- see `alpha-one`"), None);
        assert_eq!(lead_token("-`alpha-one`"), None);
        assert_eq!(lead_token("- `alpha one`"), Some("alpha one"));
        assert!(!is_token("alpha one"));
        assert!(!is_token("1alpha"));
        assert!(!is_token(""));
    }

    // spec: gate-sdk/SPEC.md §lib/declaration.sh — a drained surface is its header line alone, so
    // every section reads `Absent`, which the surface's readers take as the empty set
    #[test]
    fn a_header_only_surface_holds_no_section() {
        let drained = "# contract: x.md §y\n";
        for rule in [TokenRule::GateName, TokenRule::Backticked, TokenRule::Bolded] {
            assert!(matches!(
                section_tokens(drained, "Alpha", rule),
                SectionVerdict::Absent
            ));
        }
    }

    const PROSE: &str = "\
## Renamed knobs

- `KIT_OLD_KNOB` → `KIT_NEW_KNOB` — renamed.
- `[tag:]` → ∅ — removed.
- **`KIT_BOLD`** → `KIT_X` — bolded, which this section refuses.

## Behavior changes

- **`kit/bin/tool.sh`** refuses a bare pair — prose after the bold.
- ** a surface phrase ** — trimmed.
- `kit/bin/other.sh` — backticked, which this section refuses.
- **** — an empty bold.
";

    #[test]
    fn the_prose_sections_take_their_own_lead_rules() {
        match section_tokens(PROSE, "Renamed knobs", TokenRule::Backticked) {
            SectionVerdict::Unparsed(b) => {
                assert_eq!(b[0], "KIT_OLD_KNOB");
                assert_eq!(b[1], "[tag:]");
                assert!(b[2].starts_with("- **`KIT_BOLD`**"));
                assert_eq!(b.len(), 3);
            }
            _ => panic!("a bolded knob lead resolved"),
        }
        match section_tokens(PROSE, "Behavior changes", TokenRule::Bolded) {
            SectionVerdict::Unparsed(b) => {
                assert_eq!(b[0], "`kit/bin/tool.sh`");
                assert_eq!(b[1], "a surface phrase");
                assert!(b[2].starts_with("- `kit/bin/other.sh`"));
                assert!(b[3].starts_with("- ****"));
                assert_eq!(b.len(), 4);
            }
            _ => panic!("an unbolded or empty behavior lead resolved"),
        }
        // spec: gate-sdk/SPEC.md §lib/declaration.sh — the gate-name predicate refuses the
        // underscore and bracket a knob lead legitimately carries, which is why the rule is per section
        assert!(matches!(
            section_tokens(PROSE, "Renamed knobs", TokenRule::GateName),
            SectionVerdict::Unparsed(_)
        ));
        let clean = "## Renamed knobs\n\n- `KIT_OLD` → `KIT_NEW`\n";
        match section_tokens(clean, "Renamed knobs", TokenRule::Backticked) {
            SectionVerdict::Tokens(t) => assert_eq!(t, vec!["KIT_OLD"]),
            _ => panic!("a canonical knob lead did not resolve"),
        }
    }
}
