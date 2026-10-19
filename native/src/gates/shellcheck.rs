// spec: gate-sdk/SPEC.md §check-shellcheck — ShellCheck lint of the gate family at -S warning
// (the self-lint contract), as the wrapper a program-is-the-rule member ports to: the program
// stays a declared dependency this spawns and refuses at exit 2 without
use crate::{proc, programs};
use crate::walk;

const NAME: &str = "check-shellcheck";

// spec: gate-sdk/SPEC.md §check-shellcheck — each kit root contributes these six directories and
// no other, in this order: the target list's order is the order shellcheck reports in
const KIT_SUBDIRS: &[&str] = &["lib", "bin", "checks", "templates", "smoke", "gate-tests"];

// spec: gate-sdk/SPEC.md §Fail-closed contract — the refusal is this member's own text at the
// shell form's own point in the order: before the target glob, so a tree with nothing to lint and
// no linter reports the linter. *Cannot verify* and *verified clean* do not share an exit code.
fn refuse_absent_program() -> i32 {
    eprintln!("{}: {} not found on PATH — the gate cannot run.", NAME, programs::SHELLCHECK);
    eprintln!("  A gate that cannot run is not clean (fail-closed).");
    eprintln!("  help: install ShellCheck (e.g. 'apt install shellcheck' / 'brew install shellcheck').");
    2
}

// spec: gate-sdk/SPEC.md §check-shellcheck — the derived corpus both lint gates read: the
// consumer gates and tests dirs, each kit root's six directories, then the extra-dirs knob
// appended
pub fn lint_dirs_from(gates_dir: &str, tests_dir: &str, kit_roots: &[String], extra: &[String]) -> Vec<String> {
    let mut dirs = vec![gates_dir.to_string(), tests_dir.to_string()];
    for kit in kit_roots {
        for sub in KIT_SUBDIRS {
            dirs.push(format!("{}/{}", kit, sub));
        }
    }
    // spec: gate-sdk/SPEC.md §check-shellcheck — the consumer-added set is appended to the
    // derived default, never substituted for it, so a consumer that sets nothing keeps the
    // shipped coverage exactly and one that sets it can only widen
    dirs.extend(extra.iter().cloned());
    dirs
}

pub fn lint_dirs() -> Result<Vec<String>, String> {
    Ok(lint_dirs_from(
        &walk::knob_scalar("GATE_SDK_GATES_DIR")?,
        &walk::knob_scalar("GATE_SDK_TESTS_DIR")?,
        &walk::kit_roots_abs()?,
        &walk::knob_words("GATE_SDK_LINT_EXTRA_DIRS")?,
    ))
}

// spec: gate-sdk/SPEC.md §check-shellcheck — positional arguments remain a full scope override
// (the hermetic fixture affordance), never an addition
fn target_dirs(args: &[String]) -> Result<Vec<String>, String> {
    if !args.is_empty() {
        return Ok(args.to_vec());
    }
    lint_dirs()
}

pub fn run(args: &[String]) -> i32 {
    let dirs = match target_dirs(args) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };

    if !proc::on_path(&programs::SHELLCHECK) {
        return refuse_absent_program();
    }

    // spec: gate-sdk/SPEC.md §check-shellcheck — `shopt -s nullglob; targets+=("$d"/*.sh)`: a
    // directory that does not exist contributes nothing rather than its unexpanded pattern, and
    // the dirs are visited in their own order with each expansion sorted within itself
    let mut targets: Vec<String> = Vec::new();
    for d in &dirs {
        targets.extend(walk::glob_entries(&format!("{}/*.sh", d)));
    }

    if targets.is_empty() {
        eprintln!(
            "{}: no *.sh found under: {} — nothing to lint.",
            NAME,
            dirs.join(" ")
        );
        return 2;
    }

    let mut argv: Vec<&str> = vec!["-S", "warning"];
    argv.extend(targets.iter().map(String::as_str));
    let completed = match proc::run_merged(&programs::SHELLCHECK, &argv) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            return 2;
        }
    };

    if completed.succeeded() {
        println!("SHELLCHECK: clean ({} scripts)", targets.len());
        return 0;
    }

    // spec: gate-sdk/SPEC.md §check-shellcheck — `printf '%s\n' "$output"` on a command
    // substitution's value: every trailing newline is stripped by the capture and exactly one is
    // printed back, so a linter's report never gains or loses a blank line at its tail
    let report = String::from_utf8_lossy(completed.output()).into_owned();
    println!("{}", report.trim_end_matches('\n'));
    println!();
    println!("help: ShellCheck flagged the script(s) above (-S warning). Fix each finding,");
    println!("      or silence a genuine false positive inline with '# shellcheck");
    println!("      disable=SCxxxx' PLUS a justifying comment (no blanket .shellcheckrc).");
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-shellcheck — positional arguments override the derived set
    // whole, which is what lets the fixture pair run hermetically on one directory
    #[test]
    fn positionals_replace_the_derived_set_rather_than_extending_it() {
        let args = vec!["alpha".to_string(), "beta".to_string()];
        let dirs = target_dirs(&args).expect("positional dirs need no knob");
        assert_eq!(
            dirs,
            vec!["alpha".to_string(), "beta".to_string()],
            "a positional scope override picked up a derived directory, so the hermetic \
             fixture affordance is no longer hermetic"
        );
    }

    // spec: gate-sdk/SPEC.md §check-shellcheck — the suites are derived, never named: each kit
    // root's `gate-tests/` and the tests dir join the corpus, in the order shellcheck reports in
    #[test]
    fn the_derived_dirs_take_each_kit_root_s_suites_and_the_tests_dir() {
        let roots = vec!["/r/alpha-kit".to_string(), "/r/beta-kit".to_string()];
        let dirs = lint_dirs_from("scripts", "scripts/gate-tests", &roots, &["docs".to_string()]);
        let mut want = vec!["scripts".to_string(), "scripts/gate-tests".to_string()];
        for root in &roots {
            for sub in ["lib", "bin", "checks", "templates", "smoke", "gate-tests"] {
                want.push(format!("{}/{}", root, sub));
            }
        }
        want.push("docs".to_string());
        assert_eq!(
            dirs, want,
            "the derived corpus lost a kit root's suites or the tests dir, so a new kit's \
             suites would lint nothing until a knob named them"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the wrapper contract's refusal predicate,
    // exercised directly: a fixture pair cannot remove a program from PATH, so the presence
    // probe is the half a committed case can never reach
    #[test]
    fn the_presence_probe_answers_for_a_program_that_is_not_installed() {
        assert!(
            !proc::on_path(&programs::Program::consumer(
                "test",
                "checkwright-no-such-program-exists"
            )),
            "the presence probe found a program that does not exist, so the wrapper would \
             spawn instead of printing its own refusal"
        );
        assert!(
            proc::on_path(&programs::BASH),
            "the presence probe missed bash, so every wrapper would refuse at exit 2 with \
             its program installed"
        );
    }

    // spec: gate-sdk/SPEC.md §Fail-closed contract — the merged capture carries a child's report
    // whatever its status, and the clean-line predicate reads the status rather than the capture
    #[test]
    fn a_merged_capture_carries_stderr_and_grades_by_status_not_emptiness() {
        let m = proc::run_merged(&programs::BASH, &["-c", "printf out; printf err >&2; exit 3"])
            .expect("cannot run bash");
        assert!(!m.succeeded(), "a child that exited 3 reported success");
        let text = String::from_utf8_lossy(m.output()).into_owned();
        assert!(
            text.contains("out") && text.contains("err"),
            "the merged capture dropped one of the two streams, so a wrapper printing it \
             would lose half its program's report: {:?}",
            text
        );
    }
}
