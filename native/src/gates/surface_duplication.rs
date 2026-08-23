// spec: canon-kit/SPEC.md §check-surface-duplication — a non-glossary surface may not carry a
// glossary term's bold-lead-in definition. The two awk programs the shell form drove become two
// in-process scans; every program it spawned was on the floor, so the compiled form spawns none.
use crate::spec;

const NAME: &str = "check-surface-duplication";

// spec: canon-kit/SPEC.md §check-surface-duplication — the valve keyword a surface's class
// licenses: a canonical spec introduces a term as a spec, every other configured surface as a
// vision. The keyword is also the scan's own parameter, so the two cannot drift apart.
#[derive(Clone, Copy, PartialEq)]
enum Valve {
    Spec,
    Vision,
}

impl Valve {
    fn keyword(self) -> &'static str {
        match self {
            Valve::Spec => "spec-introduces",
            Valve::Vision => "vision-introduces",
        }
    }
}

fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

fn trim_posix(s: &str) -> &str {
    s.trim_matches(is_space)
}

// spec: canon-kit/SPEC.md §check-surface-duplication — awk's `gsub(/\([^)]*\)/, "", cell)`: every
// shortest parenthesised span dropped, an unclosed `(` left alone
fn strip_parens(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(open) = rest.find('(') {
        match rest[open..].find(')') {
            Some(close) => {
                out.push_str(&rest[..open]);
                rest = &rest[open + close + 1..];
            }
            None => break,
        }
    }
    out.push_str(rest);
    out
}

// spec: canon-kit/SPEC.md §check-surface-duplication — the bold lead-in's head, the one derivation
// the glossary scan and the surface scan share: strip the opening `**`, truncate at the next
// `**`, drop backticks, then one trailing `.` and any trailing whitespace, in that order
fn lead_in_head(after_stars: &str) -> String {
    let body = match after_stars.find("**") {
        Some(i) => &after_stars[..i],
        None => after_stars,
    };
    let mut h: String = body.chars().filter(|c| *c != '`').collect();
    if h.ends_with('.') {
        h.pop();
    }
    while h.ends_with(is_space) {
        h.pop();
    }
    h.to_lowercase()
}

// spec: canon-kit/SPEC.md §check-surface-duplication — `^\*\*[^*]+\*\*`, the glossary body's own
// lead-in shape: the run between the star pairs carries no star, so the first `*` after the
// opening pair is where it ends
fn lead_in_body(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("**")?;
    let star = rest.find('*')?;
    if star < 1 || !rest[star..].starts_with("**") {
        return None;
    }
    Some(rest)
}

// spec: canon-kit/SPEC.md §check-surface-duplication — the glossary's term set: the Quick
// reference table's first column, slash-separated alternates split out, plus every bold lead-in in
// the body. `sort -u`, so the count is of distinct terms.
fn glossary_terms(text: &str) -> Vec<String> {
    let mut terms: Vec<String> = Vec::new();
    let mut in_qr = false;
    for line in text.lines() {
        if line.starts_with("## Quick reference") {
            in_qr = true;
        }
        if in_qr && line.starts_with("---") {
            in_qr = false;
        }
        if in_qr && line.starts_with('|') && !is_header_row(line) && !is_rule_row(line) {
            let cell = line.split('|').nth(1).unwrap_or("");
            let cell = strip_parens(&cell.chars().filter(|c| *c != '`').collect::<String>());
            for part in cell.split('/') {
                let t = trim_posix(part);
                if !t.is_empty() {
                    terms.push(t.to_lowercase());
                }
            }
            continue;
        }
        if let Some(rest) = lead_in_body(line) {
            let h = lead_in_head(rest);
            if !h.is_empty() {
                terms.push(h);
            }
        }
    }
    terms.sort();
    terms.dedup();
    terms
}

// spec: canon-kit/SPEC.md §check-surface-duplication — `^\| *Canonical`, the table's header row
fn is_header_row(line: &str) -> bool {
    line[1..].trim_start_matches(' ').starts_with("Canonical")
}

// spec: canon-kit/SPEC.md §check-surface-duplication — `^\|[-| ]*$`, the table's rule row
fn is_rule_row(line: &str) -> bool {
    line[1..].chars().all(|c| matches!(c, '-' | '|' | ' '))
}

// spec: canon-kit/SPEC.md §check-surface-duplication — the valve's term, read off the tag: take
// everything after the *last* keyword occurrence, drop the whitespace behind it, then truncate at
// the comment close with its own leading whitespace
fn valve_term(line: &str, valve: Valve) -> String {
    let tag = format!("{}:", valve.keyword());
    let Some(i) = line.rfind(&tag) else {
        return String::new();
    };
    let mut t = &line[i + tag.len()..];
    t = t.trim_start_matches(is_space);
    if let Some(j) = t.find("-->") {
        t = &t[..j];
    }
    t.trim_end_matches(is_space).to_lowercase()
}

struct Hit {
    head: String,
    lineno: usize,
    valve: String,
}

// spec: canon-kit/SPEC.md §check-surface-duplication — the lead-in scan: a candidate opens a block
// (blank line, heading, or list bullet) and carries a definition punctuation. The probe is the line
// stripped of bullet and comments; the valve is read off the raw line, or the one above it.
fn scan_surface(text: &str, valve: Valve) -> Vec<Hit> {
    let mut hits = Vec::new();
    let mut prev = String::new();
    for (idx, line) in text.lines().enumerate() {
        let probe = probe_of(line);
        let is_block = prev.chars().all(is_space)
            || bullet_prefix(line).is_some()
            || prev.starts_with('#');
        if is_block {
            if let Some(rest) = definition_lead_in(&probe) {
                let head = lead_in_head(rest);
                let tag = format!("{}:", valve.keyword());
                let vt = if line.contains(&tag) {
                    valve_term(line, valve)
                } else if prev.contains(&tag) {
                    valve_term(&prev, valve)
                } else {
                    String::new()
                };
                hits.push(Hit {
                    head,
                    lineno: idx + 1,
                    valve: vt,
                });
            }
        }
        prev = line.to_string();
    }
    hits
}

// spec: canon-kit/SPEC.md §check-surface-duplication — `^[[:space:]]*[-*][[:space:]]+`, the list
// bullet the probe drops; the same shape, minus the `+`, is the block test's third arm
fn bullet_prefix(line: &str) -> Option<usize> {
    let indent = line.len() - line.trim_start_matches(is_space).len();
    let rest = &line[indent..];
    let marker = rest.chars().next()?;
    if marker != '-' && marker != '*' {
        return None;
    }
    if !rest[1..].starts_with(is_space) {
        return None;
    }
    Some(indent + 1 + (rest[1..].len() - rest[1..].trim_start_matches(is_space).len()))
}

// spec: canon-kit/SPEC.md §check-surface-duplication — awk's `gsub(/<!--[^>]*-->/, "", probe)`:
// the body is `[^>]*` and not `.*`, so the first `>` after the opener must be the one `-->` ends
// on. That section owns why reproducing the narrowness rather than widening it is the right port.
fn strip_html_comments(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        if b[i..].starts_with(b"<!--") {
            if let Some(g) = b[i + 4..].iter().position(|c| *c == b'>') {
                if g >= 2 && b[i + 4 + g - 1] == b'-' && b[i + 4 + g - 2] == b'-' {
                    i += 4 + g + 1;
                    continue;
                }
            }
        }
        let ch = s[i..].chars().next().expect("byte index is a char boundary");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn probe_of(line: &str) -> String {
    let start = bullet_prefix(line).unwrap_or(0);
    let stripped = strip_html_comments(&line[start..]);
    stripped.trim_start_matches(is_space).to_string()
}

// spec: canon-kit/SPEC.md §check-surface-duplication — the three definition punctuations:
// `**Term.**`, `**Term** —`, `**Term** -`. Narration that merely bolds a phrase is not one.
fn definition_lead_in(probe: &str) -> Option<&str> {
    let rest = probe.strip_prefix("**")?;
    let star = rest.find('*')?;
    if !rest[star..].starts_with("**") {
        return None;
    }
    let dotted = star >= 2 && rest.as_bytes()[star - 1] == b'.';
    let dashed = star >= 1 && (rest[star..].starts_with("** —") || rest[star..].starts_with("** -"));
    if dotted || dashed {
        Some(rest)
    } else {
        None
    }
}

struct Surface {
    path: String,
    valve: Valve,
}

pub fn run(args: &[String]) -> i32 {
    match check(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            2
        }
    }
}

fn check(args: &[String]) -> Result<i32, String> {
    let root = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).is_dir() {
        eprintln!("{}: not a directory: {}", NAME, root);
        return Ok(2);
    }
    let glossary_name = spec::knob_pub("CANON_KIT_GLOSSARY_FILE")?;
    let gloss = format!("{}/{}", root, glossary_name);
    if !std::path::Path::new(&gloss).is_file() {
        eprintln!(
            "{}: no {} at {} (register the gate only where the glossary topology exists)",
            NAME, glossary_name, gloss
        );
        return Ok(2);
    }

    let spec_name = spec::spec_name()?;
    let mut surfaces: Vec<Surface> = Vec::new();
    for s in spec::knob_array_pub("CANON_KIT_DUP_SURFACES")? {
        let path = format!("{}/{}", root, s);
        if !std::path::Path::new(&path).is_file() {
            continue;
        }
        let valve = if basename(&s) == spec_name {
            Valve::Spec
        } else {
            Valve::Vision
        };
        surfaces.push(Surface { path, valve });
    }

    // spec: canon-kit/SPEC.md §check-surface-duplication — a bold lead-in naming a *component* — a
    // directory owning a canonical spec — is never flagged: the component's definition lives in
    // its own spec, so there is nothing to restate
    let mut components: Vec<String> = Vec::new();
    for f in spec::canonical_specs_sorted(&root)? {
        let comp = match dirname(&f) {
            Some(d) if basename(&d) != "." => basename(&d).to_string(),
            _ => basename(&abs_root(&root)).to_string(),
        };
        components.push(comp.to_lowercase());
        surfaces.push(Surface {
            path: f,
            valve: Valve::Spec,
        });
    }

    if surfaces.is_empty() {
        println!(
            "SURFACE-DUPLICATION: clean (no configured surface present under {})",
            root
        );
        return Ok(0);
    }

    let terms = glossary_terms(&read(&gloss)?);
    let tcount = terms.len();

    let mut errors: Vec<String> = Vec::new();
    for surface in &surfaces {
        let text = read(&surface.path)?;
        let rel = surface
            .path
            .strip_prefix(&format!("{}/", root))
            .unwrap_or(&surface.path);
        for hit in scan_surface(&text, surface.valve) {
            if hit.head.is_empty() || !terms.contains(&hit.head) {
                continue;
            }
            if components.contains(&hit.head) {
                continue;
            }
            if hit.valve == hit.head {
                continue;
            }
            errors.push(format!(
                "{}:{}: bold-lead-in definition of glossary term '{}' — this surface may name it and explain *why*/*how-it-joins*, not carry the canonical definition (that lives in {}). Reword to narration + a pointer, or tag the line '<!-- {}: {} -->' if this surface legitimately introduces it",
                rel, hit.lineno, hit.head, glossary_name, surface.valve.keyword(), hit.head
            ));
        }
    }

    if !errors.is_empty() {
        println!("SURFACE-DUPLICATION: {} violation(s):", errors.len());
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: {} owns the definition; another surface owns the *why* and the local mechanism — reword a restated definition to narration + a pointer, or tag the line '<!-- vision-introduces: <term> -->' / '<!-- spec-introduces: <term> -->'", glossary_name);
        return Ok(1);
    }
    println!(
        "SURFACE-DUPLICATION: clean ({} surfaces, {} terms)",
        surfaces.len(),
        tcount
    );
    Ok(0)
}

// spec: gate-sdk/SPEC.md §Fail-closed contract — the shell form drove every surface through awk
// and refused on a non-zero exit; the compiled form has no child, so the same refusal is owed on
// the read that replaced it
fn read(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| {
        format!(
            "cannot read {}: {} — the check could not run; treating as failure (not clean)",
            path, e
        )
    })
}

fn basename(p: &str) -> &str {
    let trimmed = p.trim_end_matches('/');
    if trimmed.is_empty() {
        return "/";
    }
    match trimmed.rfind('/') {
        Some(i) => &trimmed[i + 1..],
        None => trimmed,
    }
}

fn dirname(p: &str) -> Option<String> {
    let trimmed = p.trim_end_matches('/');
    match trimmed.rfind('/') {
        Some(0) => Some("/".to_string()),
        Some(i) => Some(trimmed[..i].to_string()),
        None => Some(".".to_string()),
    }
}

fn abs_root(root: &str) -> String {
    std::fs::canonicalize(root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| root.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: canon-kit/SPEC.md §check-surface-duplication — the three definition punctuations, and
    // the narration that merely bolds a phrase. A fixture pair asserts a finding's presence and
    // never its absence, so the negative half of the rule lives here.
    #[test]
    fn only_the_three_definition_punctuations_are_lead_ins() {
        for yes in [
            "**Widget.** A unit of work.",
            "**Widget** — a unit of work.",
            "**Widget** - a unit of work.",
        ] {
            assert!(
                definition_lead_in(yes).is_some(),
                "a definition punctuation stopped reading as a lead-in: {:?}",
                yes
            );
        }
        for no in [
            "A **Widget** mid-sentence.",
            "**Widget** carries no punctuation.",
            "****",
            "**Widget**",
        ] {
            assert!(
                definition_lead_in(no).is_none(),
                "narration read as a definition lead-in, so the gate would flag prose: {:?}",
                no
            );
        }
    }

    // spec: canon-kit/SPEC.md §check-surface-duplication — awk's `[^>]*` comment body, the one
    // place the scan is deliberately narrower than a reader expects: a comment carrying a `>` is
    // not a comment to this rule, and widening it would silence a valve tag the shell form reads
    #[test]
    fn an_html_comment_carrying_an_angle_bracket_is_not_stripped() {
        assert_eq!(strip_html_comments("<!-- plain -->**W.** x"), "**W.** x");
        assert_eq!(strip_html_comments("<!---->keep"), "keep");
        assert_eq!(
            strip_html_comments("<!-- a > b -->**W.** x"),
            "<!-- a > b -->**W.** x"
        );
        assert_eq!(strip_html_comments("<!-->unclosed"), "<!-->unclosed");
    }

    // spec: canon-kit/SPEC.md §check-surface-duplication — the valve reads the *last* keyword on
    // the line and stops at the comment close with its own leading whitespace, so a line naming
    // the tag twice tags the term it ends with
    #[test]
    fn the_valve_term_is_the_last_tags_value_up_to_the_comment_close() {
        assert_eq!(
            valve_term("<!-- vision-introduces: Widget -->", Valve::Vision),
            "widget"
        );
        assert_eq!(
            valve_term("x <!-- vision-introduces: a --> <!-- vision-introduces: b -->", Valve::Vision),
            "b"
        );
        assert_eq!(valve_term("nothing here", Valve::Vision), "");
        assert_eq!(
            valve_term("<!-- spec-introduces: term -->", Valve::Spec),
            "term"
        );
    }

    // spec: canon-kit/SPEC.md §check-surface-duplication — a lead-in counts only where a block
    // opens: after a blank line, after a heading, or as a list bullet. Two lead-ins in a row are
    // one definition and one continuation, and the second is not flagged.
    #[test]
    fn a_lead_in_counts_only_where_a_block_opens() {
        let text = "**A.** first, after nothing at all.\n\
                    **B.** second, with a lead-in above it.\n\
                    \n\
                    **C.** third, after a blank line.\n\
                    # Heading\n\
                    **D.** fourth, after a heading.\n\
                    - **E.** fifth, as a bullet.\n";
        let heads: Vec<String> = scan_surface(text, Valve::Vision)
            .into_iter()
            .map(|h| h.head)
            .collect();
        assert_eq!(
            heads,
            vec!["a", "c", "d", "e"],
            "the block rule admitted or dropped a lead-in, so the gate flags a continuation \
             line or misses a definition"
        );
    }

    // spec: canon-kit/SPEC.md §check-surface-duplication — the Quick reference table's first
    // column: slash-separated alternates each become a term, parenthesised asides and backticks
    // are dropped, and the header and rule rows are not rows
    #[test]
    fn the_quick_reference_column_yields_every_alternate_and_no_furniture() {
        let gloss = "# G\n\
                     \n\
                     ## Quick reference\n\
                     \n\
                     | Canonical | Meaning |\n\
                     |-----------|---------|\n\
                     | Widget / Gizmo | a unit |\n\
                     | `Sprocket` (deprecated) | a thing |\n\
                     \n\
                     ---\n\
                     \n\
                     | Below | the rule row, so not a term |\n\
                     \n\
                     **Cog.** A toothed wheel.\n";
        assert_eq!(
            glossary_terms(gloss),
            vec!["cog", "gizmo", "sprocket", "widget"],
            "the term set gained furniture or lost an alternate, so the gate flags the wrong \
             heads"
        );
    }
}
