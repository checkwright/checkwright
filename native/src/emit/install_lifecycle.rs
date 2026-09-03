// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the three-step resident install: the
// registration block into the always-loaded agent file, the merge attributes into
// `.gitattributes`, and the keep-ours driver into this clone's git config
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the contract is an action with
// an exit status, and a table member because it resolves eight knobs a hardcoded flag would ignore
use crate::marker;
use crate::proc;
use crate::stages;
use crate::walk;
use std::path::Path;

// spec: gate-sdk/SPEC.md §The non-gate arm — the declared roster is the union of what the two
// renderers read, which is what `check-lifecycle-registration` and `check-merge-attrs` already
// declare between them: the arm writes the surfaces they assert, so it reads their knobs
pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_AGENT_FILE",
    "LIFECYCLE_KIT_STAGES",
    "LIFECYCLE_KIT_QUEUE_FILE",
    "LIFECYCLE_KIT_STATE_FILE",
    "LIFECYCLE_KIT_LESSON_EVIDENCE_FILE",
    "LIFECYCLE_KIT_SURVEY_RECORD_FILE",
    "LIFECYCLE_KIT_BOUNDARY_TRUNCATE",
    "LIFECYCLE_KIT_GAP_INBOX_FILE",
];

const BEGIN: &str = "<!-- lifecycle-kit:begin -->";
const END: &str = "<!-- lifecycle-kit:end -->";
const ATTRS: &str = ".gitattributes";
const ABEGIN: &str = "# lifecycle-kit:merge:begin";
const AEND: &str = "# lifecycle-kit:merge:end";

pub fn run(args: &[String]) -> i32 {
    match install(args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("install-lifecycle: {}", e);
            2
        }
    }
}

// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the `[agent-file]` positional is the
// file the rule writes into rather than a selector for where configuration comes from, so it
// arrives as argv and overrides the bridged default (gate-sdk/SPEC.md §The non-gate arm)
fn agent_file(args: &[String]) -> Result<String, String> {
    match args.first().filter(|a| !a.is_empty()) {
        Some(a) if a.starts_with('-') => Err(format!("unknown option: {}", a)),
        Some(a) => Ok(a.clone()),
        None => walk::knob_scalar("LIFECYCLE_KIT_AGENT_FILE"),
    }
}

fn install(args: &[String]) -> Result<(), String> {
    let agent = agent_file(args)?;
    // spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the installer edits an always-loaded
    // file, it does not mint one, so an absent agent file is a refusal and not a create
    if !Path::new(&agent).is_file() {
        return Err(format!(
            "agent file not found: {} — nothing to install into",
            agent
        ));
    }

    let block = stages::registration_block()?;
    let action = marker::install_block(&agent, BEGIN, END, &format!("{}\n", block))?;
    println!(
        "install-lifecycle: {} the lifecycle registration block in {}",
        action, agent
    );

    // spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — `.gitattributes` is legitimately
    // minted when absent: it is not an always-loaded file the consumer authored. The opposite
    // disposition to the agent file above, one call apart, and neither is the other's default.
    if !Path::new(ATTRS).is_file() {
        std::fs::write(ATTRS, "").map_err(|e| format!("cannot create {}: {}", ATTRS, e))?;
    }
    let aaction = marker::install_block(ATTRS, ABEGIN, AEND, &stages::merge_attrs_block()?)?;
    println!(
        "install-lifecycle: {} the iteration-scoped merge attributes in {}",
        aaction, ATTRS
    );

    register_driver();
    Ok(())
}

// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the driver-config step is the
// `install-hooks.sh` per-clone opt-in class: a non-repo cwd degrades to a printed skip on stderr
// at exit 0, never a hard failure, because the recorded honest limit depends on it failing soft
fn register_driver() {
    let inside = proc::run("git", &["rev-parse", "--git-dir"])
        .map(|c| c.stdout().is_some())
        .unwrap_or(false);
    if !inside {
        eprintln!("install-lifecycle: not a git repository — skipped the merge.iteration-scoped driver (the .gitattributes attribute stays inert until 'git config merge.iteration-scoped.driver true' is run in a clone)");
        return;
    }
    match proc::run("git", &["config", "merge.iteration-scoped.driver", "true"]) {
        Ok(c) if c.stdout().is_some() => {
            println!("install-lifecycle: registered the keep-ours merge.iteration-scoped driver (per-clone git config)");
        }
        Ok(c) => eprintln!(
            "install-lifecycle: could not register the merge.iteration-scoped driver ({})",
            c.failure_report().unwrap_or_default()
        ),
        Err(e) => eprintln!("install-lifecycle: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the agent file is never minted, so
    // an absent one is exit 2 rather than a create; a flag in the positional slot is a refusal
    #[test]
    fn an_absent_agent_file_refuses_and_never_mints() {
        let p = std::env::temp_dir()
            .join(format!("no-such-agent-{}.md", std::process::id()))
            .display()
            .to_string();
        let _ = std::fs::remove_file(&p);
        assert_eq!(run(std::slice::from_ref(&p)), 2);
        assert!(!Path::new(&p).exists(), "the refusal minted the agent file");
        assert_eq!(run(&["--nope".to_string()]), 2);
    }
}
