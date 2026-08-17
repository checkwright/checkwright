// spec: gate-sdk/SPEC.md §check-commit-subject — the subject line parses as
// <type>(<scope>)?!?: <summary> with <type> in the shared roster, or matches a git-generated
// carve-out (the parse guarantee under trajectory.sh's feat/debt column)
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §check-commit-subject — the `(\([a-z0-9./-]+\))?!?: ` tail of the
// conventional grammar, read off the remainder a roster type already consumed: an optional
// parenthesised scope token, an optional `!` break marker, then `: ` and a non-empty summary
fn tail_parses(rest: &str) -> bool {
    let rest = match rest.strip_prefix('(') {
        Some(inner) => match inner.find(')') {
            Some(i) => {
                let token = &inner[..i];
                if token.is_empty()
                    || !token
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"./-".contains(&b))
                {
                    return false;
                }
                &inner[i + 1..]
            }
            None => return false,
        },
        None => rest,
    };
    let rest = rest.strip_prefix('!').unwrap_or(rest);
    match rest.strip_prefix(": ") {
        Some(summary) => !summary.is_empty(),
        None => false,
    }
}

// spec: gate-sdk/SPEC.md §check-commit-subject — the roster alternation, split on the single
// space the shell's `tr ' ' '|'` splits on, so a roster spelling reaches the same alternatives
// here as it reaches the shell form's ERE
fn parses(subject: &str, roster: &str) -> bool {
    for t in roster.split(' ') {
        if let Some(rest) = subject.strip_prefix(t) {
            if tail_parses(rest) {
                return true;
            }
        }
    }
    // spec: gate-sdk/SPEC.md §check-commit-subject — the git-generated carve-outs: git authors
    // these subjects itself, so rejecting them would red on commits no author wrote
    ["Merge ", "Revert ", "fixup! ", "squash! "]
        .iter()
        .any(|c| subject.starts_with(c))
}

pub fn run(args: &[String]) -> i32 {
    // spec: gate-sdk/SPEC.md §check-commit-subject — no-arg is a clean skip: the message is not
    // a whole-tree surface, and the full battery runs the gate with no argument
    let Some(msg) = args.first() else {
        println!("COMMIT-SUBJECT: clean (no message file argument — the commit-msg hook surface is not a whole-tree target; skipped)");
        return 0;
    };
    if !Path::new(msg).is_file() {
        eprintln!("check-commit-subject: message file not found: {}", msg);
        return 2;
    }

    let roster = match walk::knob_scalar("GATE_SDK_COMMIT_TYPES") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("check-commit-subject: {}", e);
            return 2;
        }
    };

    let bytes = match std::fs::read(msg) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "check-commit-subject: cannot read {} ({}) — the check could not run; treating as failure (not clean)",
                msg, e
            );
            return 2;
        }
    };
    let text = String::from_utf8_lossy(&bytes).into_owned();
    // spec: gate-sdk/SPEC.md §check-commit-subject — the first line as `sed -n '1p'` yields it:
    // split on the newline alone, since a `lines()` that also strips a trailing CR would hand the
    // grammar a shorter summary than the shell form reads
    let subject = text.split('\n').next().unwrap_or("");

    if parses(subject, &roster) {
        println!(
            "COMMIT-SUBJECT: clean (subject parses against the {} roster or a git-generated carve-out)",
            roster.replace(' ', ", ")
        );
        return 0;
    }

    println!("check-commit-subject: subject line does not parse as <type>(<scope>)?!?: <summary>:");
    println!("  {}", subject);
    println!("  help: open the subject with a roster type followed by ': ' and a summary");
    println!("        (e.g. 'feat(scope): …'); the roster is GATE_SDK_COMMIT_TYPES");
    println!("        (default: {}). git's own Merge/Revert/fixup!/squash! forms are", roster);
    println!("        carve-outs — do not reword them.");
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROSTER: &str = "feat fix refactor perf docs test build ci chore style";

    #[test]
    fn the_conventional_shapes_parse_and_the_malformed_ones_do_not() {
        assert!(parses("docs: fix a typo", ROSTER));
        assert!(parses("feat(gate-sdk): add a gate", ROSTER));
        assert!(parses("refactor(a.b/c-d): rename", ROSTER));
        assert!(parses("fix!: breaking change", ROSTER));
        assert!(!parses("feat add a thing", ROSTER));
        assert!(!parses("feat: ", ROSTER));
        assert!(!parses("chore(scope) no colon", ROSTER));
        assert!(!parses("epic: land the milestone", ROSTER));
        assert!(!parses("feat(UPPER): shouted scope", ROSTER));
    }

    // spec: gate-sdk/SPEC.md §check-commit-subject — the carve-outs pass whatever the roster is,
    // which is what keeps a narrowed roster from rejecting a subject git wrote
    #[test]
    fn the_git_generated_carve_outs_pass_under_any_roster() {
        for s in [
            "Merge branch feature into master",
            "Revert \"feat: add a thing\"",
            "fixup! feat: add a thing",
            "squash! feat: add a thing",
        ] {
            assert!(parses(s, "fix"), "carve-out rejected: {}", s);
        }
    }

    // spec: gate-sdk/SPEC.md §check-commit-subject — the roster is the config surface, so a
    // widened one admits its new type and a narrowed one rejects a stock type
    #[test]
    fn the_roster_selects_which_types_parse() {
        assert!(parses("epic: land the milestone", "feat fix epic"));
        assert!(!parses("feat: add a thing", "fix"));
    }
}
