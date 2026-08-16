// spec: lifecycle-kit/SPEC.md §check-close-surfaces — the derived close-surface roster is complete
// and moded: no undeclared capture surface, every declaration carries a mode with a well-formed
// forced= citation, every capture-tier declaration names a reclaim command
use crate::emit::close_surfaces;
use crate::proc;

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

fn in_path_class(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'/' | b'-')
}

// spec: lifecycle-kit/SPEC.md §check-close-surfaces — `^forced=[A-Za-z0-9._/-]+\.md[[:space:]]+§
// [^[:space:]]`, a pattern baked into this member's own source rather than resolved from consumer
// config, so it is a byte scan here and reaches the ERE engine nowhere.
fn well_formed_forced(mode: &str) -> bool {
    let Some(rest) = mode.strip_prefix("forced=") else {
        return false;
    };
    let b = rest.as_bytes();
    let section = "\u{a7}".as_bytes();
    for dot in 1..b.len().saturating_sub(2) {
        if &b[dot..dot + 3] != b".md" {
            continue;
        }
        if !b[..dot].iter().all(|&c| in_path_class(c)) {
            continue;
        }
        let mut i = dot + 3;
        let ws_start = i;
        while i < b.len() && is_space(b[i]) {
            i += 1;
        }
        if i == ws_start {
            continue;
        }
        if i + section.len() >= b.len() || &b[i..i + section.len()] != section {
            continue;
        }
        if !is_space(b[i + section.len()]) {
            return true;
        }
    }
    false
}

pub fn run(args: &[String]) -> i32 {
    match rule(args) {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("check-close-surfaces: {}", e);
            2
        }
    }
}

fn rule(args: &[String]) -> Result<i32, String> {
    // spec: lifecycle-kit/SPEC.md §check-close-surfaces — the derivation is a function call, not a
    // spawn, so the roster the gate rules on and the roster close reads can never be two
    // computations that disagree
    let roster = close_surfaces::derive(args)?;
    let wf_prefix = format!("{}/", roster.workflow_dir);

    let mut errors: Vec<String> = Vec::new();
    let mut declarations = 0usize;
    let mut captures = 0usize;

    for row in &roster.rows {
        let mut f = row.splitn(4, '\t');
        let path = f.next().unwrap_or("");
        let mode = f.next().unwrap_or("");
        let reclaim = f.next().unwrap_or("");
        let owner = f.next().unwrap_or("");
        if path.is_empty() {
            continue;
        }

        // assertion A: no undeclared capture surface
        if mode == "(undeclared)" {
            captures += 1;
            errors.push(format!(
                "{}: capture-tier workflow member with no 'close-surface:' declaration — close would read it only by luck",
                path
            ));
            continue;
        }
        declarations += 1;

        // assertion B: every declaration carries a mode; a forced= citation is well-formed
        if mode.is_empty() {
            errors.push(format!(
                "{}: 'close-surface: {}' carries no mode — say 'advisory' or 'forced=<owner-path>.md §<section>'",
                owner, path
            ));
        } else if mode != "advisory" && !well_formed_forced(mode) {
            errors.push(format!(
                "{}: 'close-surface: {}' mode is neither 'advisory' nor a well-formed 'forced=<owner-path>.md §<section>': {}",
                owner, path, mode
            ));
        }

        // assertion C: a capture-tier declaration names its reclaim command
        if path.starts_with(&wf_prefix) {
            let ci = proc::run(
                "git",
                &["-C", &roster.base, "check-ignore", "-q", "--", path],
            )?;
            if ci.code() == Some(0) {
                captures += 1;
                if reclaim == "-" || reclaim.is_empty() {
                    errors.push(format!(
                        "{}: 'close-surface: {}' is capture-tier (gitignored) and names no reclaim= command",
                        owner, path
                    ));
                }
            }
        }
    }

    if !errors.is_empty() {
        println!(
            "check-close-surfaces: {} close-surface roster violation(s):",
            errors.len()
        );
        for e in &errors {
            println!("  {}", e);
        }
        println!("  help: declare the surface with a full-line 'close-surface: <path> <mode> [reclaim=<command>]' directive in the SPEC section that already owns it — never a central list. <mode> is 'advisory' (no forcing function; a skip is a visible judgment) or 'forced=<owner-path>.md §<section>' naming the structural forcing function. A gitignored capture surface names the drain that empties it as the trailing reclaim=<command>.");
        return Ok(1);
    }
    println!(
        "CLOSE-SURFACES: clean ({} declared surface(s), {} capture-tier; every capture member declared, every declaration moded, every capture-tier declaration reclaimed)",
        declarations, captures
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §check-close-surfaces — assertion B is shape-only: a `.md` owner
    // path, whitespace, a section sign, and something after it
    #[test]
    fn a_forced_citation_is_well_formed_only_with_an_md_owner_and_a_named_section() {
        assert!(well_formed_forced(
            "forced=lifecycle-kit/SPEC.md §The state machine"
        ));
        assert!(well_formed_forced("forced=a.b.md §x"));
        assert!(!well_formed_forced("forced=the entry refusal"));
        assert!(!well_formed_forced("forced=SPEC.md §"));
        assert!(!well_formed_forced("forced=SPEC.md § "));
        assert!(!well_formed_forced("forced=.md §x"));
        assert!(!well_formed_forced("forced=SPEC.mdx §x"));
        assert!(!well_formed_forced("advisory"));
    }
}
