// spec: doctrine-kit/SPEC.md §stage-rules — the craft-rule router: for a given stage, the pointers
// to every rule whose `*Stages:*` trailer routes to it, one line each, read from the doctrine file
// at run time so no rule content and no stage vocabulary is baked here
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Emit` because the contract is a document and
// both failures are already exit 2, which is exactly what that variant collapses to
use crate::emit;
use crate::walk;

// spec: gate-sdk/SPEC.md §The non-gate arm — one declared knob: a hardcoded top-level flag would
// resolve a platform default and silently ignore every consumer override
pub const KNOBS: &[&str] = &["DOCTRINE_KIT_DOCTRINE_FILE"];

// spec: doctrine-kit/SPEC.md §stage-rules — kit mechanism (the kit ships `DOCTRINE.md`), never
// config: what crosses to a consumer is the rule content in that file, not this heading
const CRAFT_SECTION: &str = "## Engineering-craft rules";

// spec: doctrine-kit/SPEC.md §stage-rules — awk's `^#+[[:space:]]` heading level, so a line that
// is not a heading is level 0 and can never open or close the section
fn hlevel(line: &str) -> usize {
    let b = line.as_bytes();
    let n = b.iter().take_while(|c| **c == b'#').count();
    if n == 0 || !matches!(b.get(n), Some(b' ') | Some(b'\t')) {
        return 0;
    }
    n
}

// spec: doctrine-kit/SPEC.md §stage-rules — a numbered craft rule's number and bolded name, the
// shell form's four substitutions in order: they strip from the FIRST match, which is what keeps a
// name carrying a period or a second bold run from being truncated differently here
fn rule_head(line: &str) -> Option<(String, String)> {
    let b = line.as_bytes();
    let mut i = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 {
        return None;
    }
    let num = &line[..i];
    let after = line.get(i..)?.strip_prefix('.')?;
    let trimmed = after.trim_start_matches([' ', '\t']);
    if trimmed.len() == after.len() {
        return None;
    }
    let name = trimmed.strip_prefix("**")?;
    let name = match name.find("**") {
        Some(j) => &name[..j],
        None => name,
    };
    Some((
        num.to_string(),
        name.strip_suffix('.').unwrap_or(name).to_string(),
    ))
}

// spec: doctrine-kit/SPEC.md §stage-rules — the `*Stages:*` trailer's tokens: split on a comma,
// then strip every space from the token, so a routing is matched on the bare stage name
fn stages_trailer(line: &str) -> Option<Vec<String>> {
    let t = line.trim_start_matches([' ', '\t']);
    let val = t.strip_prefix("*Stages:*")?;
    Some(
        val.split(',')
            .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
            .collect(),
    )
}

// spec: doctrine-kit/SPEC.md §stage-rules — an unknown stage and an absent craft section each yield
// **empty output at exit 0**, graceful by design: a consumer with a renamed stage set gets no
// routing rather than wrong routing, and that honest limit is preserved rather than closed
pub fn render(text: &str, stage: &str, path: &str) -> String {
    let mut out = String::new();
    let mut insec = false;
    let mut start_lvl = 0usize;
    let mut cur_num = String::new();
    let mut cur_name = String::new();
    for line in text.lines() {
        let lvl = hlevel(line);
        if !insec {
            if lvl > 0 && line.starts_with(CRAFT_SECTION) {
                insec = true;
                start_lvl = lvl;
            }
            continue;
        }
        if lvl > 0 && lvl <= start_lvl {
            insec = false;
            continue;
        }
        if let Some((num, name)) = rule_head(line) {
            cur_num = num;
            cur_name = name;
        } else if let Some(toks) = stages_trailer(line) {
            if toks.iter().any(|t| t == stage) {
                out.push_str(&format!("  • {}. {} — {}\n", cur_num, cur_name, path));
            }
        }
    }
    out
}

// spec: doctrine-kit/SPEC.md §stage-rules — `<stage>` required, `[doctrine-file]` optional and
// overriding the knob, the shell precedence unchanged; the optional positional is kept because the
// gate and the installer it matches still take theirs (gate-sdk/SPEC.md §The non-gate arm)
pub fn emit(args: &[String]) -> Result<String, String> {
    let stage = match args.first().filter(|a| !a.is_empty()) {
        Some(s) => s.clone(),
        None => return Err("usage: --emit stage-rules <stage> [doctrine-file]".to_string()),
    };
    let path = match args.get(1).filter(|a| !a.is_empty()) {
        Some(p) => p.clone(),
        None => walk::knob_scalar("DOCTRINE_KIT_DOCTRINE_FILE")?,
    };
    let text = emit::read_text(&path)
        .map_err(|_| format!("doctrine file not found: {} — treating as failure", path))?;
    Ok(render(&text, &stage, &path))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = "\
# Doctrine

## Engineering-craft rules

1. **Alpha rule.** Body text.
   *Stages:* build, validate

2. **Beta rule** — a name with no trailing period.
   *Stages:* close

### A deeper heading does not close the section

3. **Gamma rule.**
   *Stages:* build

## A sibling heading does close it

4. **Delta rule.**
   *Stages:* build
";

    #[test]
    fn a_routed_stage_gets_one_pointer_line_per_rule_and_a_deeper_heading_does_not_close() {
        assert_eq!(
            render(DOC, "build", "DOCTRINE.md"),
            "  • 1. Alpha rule — DOCTRINE.md\n  • 3. Gamma rule — DOCTRINE.md\n"
        );
        assert_eq!(
            render(DOC, "close", "DOCTRINE.md"),
            "  • 2. Beta rule — DOCTRINE.md\n"
        );
    }

    // spec: doctrine-kit/SPEC.md §stage-rules — the two graceful limits, preserved verbatim because
    // an idiomatic rewrite would make either an error: both are empty output, never a diagnostic
    #[test]
    fn an_unknown_stage_and_an_absent_craft_section_are_both_empty_output() {
        assert_eq!(render(DOC, "no-such-stage", "DOCTRINE.md"), "");
        assert_eq!(render("# Doctrine\n\n## Other\n\n1. **X.**\n   *Stages:* build\n", "build", "d.md"), "");
        assert_eq!(render("", "build", "d.md"), "");
    }

    // spec: doctrine-kit/SPEC.md §stage-rules — the arm bakes no stage vocabulary: the token is
    // compared against the caller's argument, so a consumer's renamed set routes nothing
    #[test]
    fn the_trailer_is_matched_on_the_bare_token_after_whitespace_is_stripped() {
        assert_eq!(
            stages_trailer("   *Stages:* build , validate"),
            Some(vec!["build".to_string(), "validate".to_string()])
        );
        assert_eq!(stages_trailer("not a trailer"), None);
        assert_eq!(
            rule_head("12. **Named thing.** rest"),
            Some(("12".to_string(), "Named thing".to_string()))
        );
        assert_eq!(rule_head("- **not numbered**"), None);
        assert_eq!(rule_head("3.**no space**"), None);
    }

    // spec: doctrine-kit/SPEC.md §stage-rules — `<stage>` is required and its absence is exit 2
    // through `Arm::Emit`'s collapse, which is the shell form's usage status unchanged
    #[test]
    fn a_missing_stage_argument_refuses() {
        assert!(emit(&[]).is_err());
        assert!(emit(&["".to_string()]).is_err());
    }
}
