// spec: lifecycle-kit/SPEC.md §check-close-surfaces — the derived close-surface roster is complete
// and moded: no undeclared capture surface, every declaration carries a mode with a well-formed
// forced= citation, every capture-tier declaration names a reclaim command that rotates
use crate::emit::close_surfaces;

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

// spec: lifecycle-kit/SPEC.md §check-close-surfaces — assertion D's positive form: the reclaim's
// last tokens are `capture-drain`, an optional `--`, and the row's own path
fn rotates(reclaim: &str, path: &str) -> bool {
    let mut t = reclaim.split_ascii_whitespace().rev();
    if t.next() != Some(path) {
        return false;
    }
    match t.next() {
        Some("capture-drain") => true,
        Some("--") => t.next() == Some("capture-drain"),
        _ => false,
    }
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

    let mut errors: Vec<String> = Vec::new();
    let mut declarations = 0usize;
    let mut captures = 0usize;

    for row in &roster.rows {
        // spec: lifecycle-kit/SPEC.md §check-close-surfaces — the fifth field, the row's state, is
        // split off and never read, and the sixth is `<tracking>`: a four-way split would fold both
        // into the owner
        let mut f = row.splitn(6, '\t');
        let path = f.next().unwrap_or("");
        let mode = f.next().unwrap_or("");
        let reclaim = f.next().unwrap_or("");
        let owner = f.next().unwrap_or("");
        let tracking = f.nth(1).unwrap_or("");
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
        if tracking == "ignored" {
            captures += 1;
            if reclaim == "-" || reclaim.is_empty() {
                errors.push(format!(
                    "{}: 'close-surface: {}' is capture-tier (gitignored) and names no reclaim= command",
                    owner, path
                ));
            // assertion D: that reclaim rotates the row's own path aside
            } else if !rotates(reclaim, path) {
                errors.push(format!(
                    "{}: 'close-surface: {}' reclaim does not rotate the log — close reads it before the reclaim, so a truncation erases every line appended in between; name '--emit capture-drain {}': {}",
                    owner, path, path, reclaim
                ));
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
        println!("  help: declare the surface with a full-line 'close-surface: <path> <mode> [reclaim=<command>]' directive in the section that owns the surface — never a central list; for an undeclared kit-owned log in a vendored tree, run `update`. <mode> is 'advisory' (no forcing function; a skip is a visible judgment) or 'forced=<owner-path>.md §<section>' naming the structural forcing function. A gitignored capture surface names the drain that empties it as the trailing reclaim=<command>, a rotation: '<gate binary> --emit capture-drain <path>'.");
        return Ok(1);
    }
    println!(
        "CLOSE-SURFACES: clean ({} declared surface(s), {} capture-tier; every capture member declared, every declaration moded, every capture-tier declaration reclaimed by rotation)",
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

    #[test]
    fn a_reclaim_rotates_only_as_capture_drain_on_the_rows_own_path() {
        let p = ".workflow/x.log";
        assert!(rotates("bash gate-sdk/bin/run-gates.sh --emit capture-drain .workflow/x.log", p));
        assert!(rotates("checkwright --emit capture-drain -- .workflow/x.log", p));
        assert!(!rotates(": > .workflow/x.log", p));
        assert!(!rotates("truncate -s0 .workflow/x.log", p));
        assert!(!rotates("run-gates.sh --emit capture-drain --done .workflow/x.log", p));
        assert!(!rotates("run-gates.sh --emit capture-drain .workflow/other.log", p));
        assert!(!rotates("run-gates.sh --emit capture-drain .workflow/x.log extra", p));
    }
}
