// spec: lifecycle-kit/SPEC.md §check-stamp-subject — a commit adding a stamp line carries the
// stamped stage as its subject scope, the last added stamp deciding
use crate::stages;
use crate::{proc, programs, walk};
use std::path::Path;

fn read_file(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {} ({})", path, e))
}

// spec: lifecycle-kit/SPEC.md §check-stamp-subject — a blob git cannot show is an absent side: no
// staged state file skips, and no `HEAD` one (a root commit) makes every staged line an addition.
fn blob(spec: &str) -> Option<String> {
    proc::run(&programs::GIT, &["show", spec])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).into_owned()))
}

// spec: lifecycle-kit/SPEC.md §check-stamp-subject — the commits a merge in progress joins, read
// off `MERGE_HEAD`; none when no merge is in progress
fn merge_parents() -> Vec<String> {
    let present = proc::run(&programs::GIT, &["rev-parse", "-q", "--verify", "MERGE_HEAD"])
        .map(|c| c.stdout().is_some())
        .unwrap_or(false);
    if !present {
        return Vec::new();
    }
    let Some(path) = proc::run(&programs::GIT, &["rev-parse", "--git-path", "MERGE_HEAD"])
        .ok()
        .and_then(|c| c.stdout().map(|o| String::from_utf8_lossy(o).trim().to_string()))
    else {
        return Vec::new();
    };
    std::fs::read_to_string(&path)
        .map(|t| t.split_whitespace().map(str::to_string).collect())
        .unwrap_or_default()
}

// spec: lifecycle-kit/SPEC.md §check-stamp-subject — the live added-stamp read, shared with
// check-dispatch-entry: the staged state file, and the `HEAD` and merge-parent versions before it
pub fn live_state_sides(state: &str) -> Option<(String, Vec<String>)> {
    let staged = blob(&format!(":{}", state))?;
    let mut priors = vec![blob(&format!("HEAD:{}", state)).unwrap_or_default()];
    for parent in merge_parents() {
        priors.push(blob(&format!("{}:{}", parent, state)).unwrap_or_default());
    }
    Some((staged, priors))
}

pub fn run(args: &[String]) -> i32 {
    // spec: lifecycle-kit/SPEC.md §check-stamp-subject — no-arg is a clean skip, on
    // check-commit-subject's ground: the message is not a whole-tree surface
    let Some(msg) = args.first() else {
        println!("STAMP-SUBJECT: clean (no message file argument — the commit-msg hook surface is not a whole-tree target; skipped)");
        return 0;
    };
    if args.len() == 2 {
        eprintln!("check-stamp-subject: usage: check-stamp-subject <message-file> [<staged-state> <head-state> [<merge-parent-state>...]]");
        return 2;
    }
    if !Path::new(msg).is_file() {
        eprintln!("check-stamp-subject: message file not found: {}", msg);
        return 2;
    }
    let roster = match stages::stages() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("check-stamp-subject: {}", e);
            return 2;
        }
    };
    let text = match read_file(msg) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("check-stamp-subject: {} — the check could not run; treating as failure (not clean)", e);
            return 2;
        }
    };
    // spec: lifecycle-kit/SPEC.md §check-stamp-subject — the fixture form hands every state-file
    // side in as a file, so the pair runs hermetically without a git index
    let (staged, priors) = if args.len() >= 3 {
        let mut sides = Vec::new();
        for a in &args[1..] {
            match read_file(a) {
                Ok(t) => sides.push(t),
                Err(e) => {
                    eprintln!("check-stamp-subject: {}", e);
                    return 2;
                }
            }
        }
        let staged = sides.remove(0);
        (staged, sides)
    } else {
        let state = match walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-stamp-subject: {}", e);
                return 2;
            }
        };
        match live_state_sides(&state) {
            Some(sides) => sides,
            None => {
                println!("STAMP-SUBJECT: clean (no staged state file {} — no stamp to bind)", state);
                return 0;
            }
        }
    };

    let prior: Vec<&str> = priors.iter().map(String::as_str).collect();
    let Some(line) = stages::last_added_stamp_over(&staged, &prior, &roster) else {
        println!("STAMP-SUBJECT: clean (the commit adds no stamp line)");
        return 0;
    };
    let stage = stages::stamp_stage(line);
    let subject = text.split('\n').next().unwrap_or("");
    if super::commit_subject::scope_token(subject) == Some(stage) {
        println!("STAMP-SUBJECT: clean (the subject scope is the stamped stage '{}')", stage);
        return 0;
    }
    println!("check-stamp-subject: a commit adding a stamp must carry the stamped stage as its subject scope:");
    println!("  subject: {}", subject);
    println!("  stamp:   {}", line);
    println!("  expected scope: ({})", stage);
    println!("  help: use the subject --enter-stage printed on its `subject:` line, e.g.");
    println!(
        "        '{}'; a stage session's other commits keep their component scope.",
        stages::entry_subject(stage)
    );
    1
}

#[cfg(test)]
mod tests {
    use super::super::commit_subject::scope_token;
    use crate::stages::{last_added_stamp, last_added_stamp_over};

    fn roster() -> Vec<String> {
        ["scope", "spec", "align", "build", "validate", "close"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    // spec: lifecycle-kit/SPEC.md §check-stamp-subject — the last added stamp decides; a waiver
    // line is no stamp, and a boundary reset's truncation leaves its one new line as the addition
    #[test]
    fn the_last_added_roster_stamp_decides_and_a_waiver_is_skipped() {
        let hdr = "# h\n---\n\n";
        let head = format!("{}it scope s1 2026-01-01 aaaaaaa\n", hdr);
        let entry = format!("{}it align s2 2026-01-01 bbbbbbb\nit align-waived s2 2026-01-01 bbbbbbb\n", head);
        assert_eq!(
            last_added_stamp(&entry, &head, &roster()),
            Some("it align s2 2026-01-01 bbbbbbb")
        );
        let reset = format!("{}next scope s9 2026-01-02 ccccccc\n", hdr);
        assert_eq!(
            last_added_stamp(&reset, &entry, &roster()),
            Some("next scope s9 2026-01-02 ccccccc")
        );
        assert_eq!(last_added_stamp(&head, &head, &roster()), None);
    }

    // spec: lifecycle-kit/SPEC.md §check-stamp-subject — the scope is read by check-commit-subject's
    // grammar, so an unscoped or malformed subject has none
    #[test]
    fn the_scope_token_is_read_by_the_commit_subject_grammar() {
        assert_eq!(scope_token("chore(align): stamp the align stage entry"), Some("align"));
        assert_eq!(scope_token("feat(lifecycle-kit)!: x"), Some("lifecycle-kit"));
        assert_eq!(scope_token("chore: stamp"), None);
        assert_eq!(scope_token("chore(align) no colon"), None);
        assert_eq!(scope_token("Merge (align): x"), None);
    }

    // spec: lifecycle-kit/SPEC.md §check-stamp-subject — a line only a merge parent carries is
    // inherited, not added; a line no parent carries is added; no parents is the `HEAD` read
    #[test]
    fn a_stamp_a_merge_parent_carries_is_not_an_addition() {
        let hdr = "# h\n---\n\n";
        let head = format!("{}it scope s1 2026-01-01 aaaaaaa\n", hdr);
        let side = format!("{}it build s2 2026-01-01 bbbbbbb\n", head);
        assert_eq!(last_added_stamp_over(&side, &[&head, &side], &roster()), None);
        let resolved = format!("{}it build s3 2026-01-02 ccccccc\n", side);
        assert_eq!(
            last_added_stamp_over(&resolved, &[&head, &side], &roster()),
            Some("it build s3 2026-01-02 ccccccc")
        );
        assert_eq!(
            last_added_stamp_over(&side, &[&head], &roster()),
            last_added_stamp(&side, &head, &roster())
        );
    }
}
