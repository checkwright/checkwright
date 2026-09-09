// spec: installer/README.md §update — init with one added precondition and its argv forwarded
// verbatim: `checkwright.lock` must already exist, so a verb named update can manage an install but
// never perform the first one. Every init flag stays valid, `--dry-run` included.
use super::{lock, refuse, Refusal};

const USAGE: &[&str] = &[
    "usage: checkwright update [--profile <name>] [--dry-run] [--force] [--no-commit]",
    "",
    "Runs checkwright init with the same arguments, refusing when there is no",
    "existing install for it to update. Every init flag is valid here — see",
    "'checkwright init --help' for what each one does.",
];

pub fn run(args: &[String]) -> i32 {
    // spec: installer/README.md §The verbs — `-h`/`--help` is intercepted first and answers on its
    // own, outside any repository precondition, exactly as every other verb's `--help` does.
    if args.iter().any(|a| a == "-h" || a == "--help") {
        for line in USAGE {
            println!("{}", line);
        }
        return 0;
    }
    if let Err(r) = precondition() {
        return super::finish("update", Err(r));
    }
    // spec: installer/README.md §init — everything else, including --dry-run, --force and
    // --no-commit, is init's own contract, unrepeated here.
    super::init::run(args)
}

// spec: installer/README.md §update — the one added precondition, and the whole behavioral
// difference from init. It checks existence only: every other precondition is init's own, one call
// away, and repeating one here would be a second copy that could drift.
// spec: installer/README.md §update — an unresolvable root is not this verb's to own either, so it
// falls through to init's own refusal, which already names the accurate remedy.
fn precondition() -> Result<(), Refusal> {
    let Some(root) = super::repo_root() else {
        return Ok(());
    };
    if lock::path(&root).is_file() {
        return Ok(());
    }
    Err(refuse(
        format!("no {} at {}", lock::FILE, root.display()),
        "update manages an install init already made; there isn't one yet. Run 'checkwright init' first.",
        2,
    ))
}

#[cfg(test)]
mod tests {
    // spec: installer/README.md §The verbs — `--help` answers on its own wherever it sits in argv,
    // which is what keeps it outside the added precondition.
    #[test]
    fn help_answers_from_any_position_and_before_the_precondition() {
        assert_eq!(super::run(&["--profile".to_string(), "starter".to_string(), "--help".to_string()]), 0);
    }
}
