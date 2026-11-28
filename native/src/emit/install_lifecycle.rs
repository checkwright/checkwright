// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the three-step resident install: the
// registration block into the always-loaded agent file, the merge attributes into
// `.gitattributes`, and the keep-ours driver into this clone's git config
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the contract is an action with
// an exit status, and a table member because it reads nine knobs a hardcoded flag would hide
// from the knob-file derivation
use crate::exit::Fault;
use crate::marker;
use crate::{proc, programs};
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
    "LIFECYCLE_KIT_CONSULT_INBOX_FILE",
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
            e.status()
        }
    }
}

// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the `[agent-file]` positional is the
// file the rule writes into rather than a selector for where configuration comes from, so it
// arrives as argv and overrides the knob's default (gate-sdk/SPEC.md §The non-gate arm)
fn agent_file(args: &[String]) -> Result<String, Fault> {
    match args.first().filter(|a| !a.is_empty()) {
        Some(a) if a.starts_with('-') => Err(Fault::usage(format!("unknown option: {}", a))),
        Some(a) => Ok(a.clone()),
        None => walk::knob_scalar("LIFECYCLE_KIT_AGENT_FILE").map_err(Fault::refused),
    }
}

fn install(args: &[String]) -> Result<(), Fault> {
    let agent = agent_file(args)?;
    install_blocks(&agent, ATTRS)?;
    register_driver();
    Ok(())
}

fn install_blocks(agent: &str, attrs: &str) -> Result<(), Fault> {
    // spec: gate-sdk/SPEC.md §lib/inject.sh — both targets resolve before the first write, so a
    // link that refuses leaves nothing written and the `.gitattributes` seed never follows one
    marker::target(agent).map_err(Fault::input)?;
    marker::target(attrs).map_err(Fault::input)?;
    // spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the installer edits an always-loaded
    // file, it does not mint one, so an absent agent file is a refusal and not a create
    if !Path::new(agent).is_file() {
        return Err(Fault::refused(format!(
            "agent file not found: {} — nothing to install into",
            agent
        )));
    }

    let block = stages::registration_block().map_err(Fault::refused)?;
    let action = marker::install_block(agent, BEGIN, END, &format!("{}\n", block)).map_err(Fault::input)?;
    println!(
        "install-lifecycle: {} the lifecycle registration block in {}",
        action, agent
    );

    // spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — `.gitattributes` is legitimately
    // minted when absent: it is not an always-loaded file the consumer authored. The opposite
    // disposition to the agent file above, one call apart, and neither is the other's default.
    if !Path::new(attrs).is_file() {
        std::fs::write(attrs, "").map_err(|e| Fault::tool(format!("cannot create {}: {}", attrs, e)))?;
    }
    let attrs_block = stages::merge_attrs_block().map_err(Fault::refused)?;
    let aaction = marker::install_block(attrs, ABEGIN, AEND, &attrs_block).map_err(Fault::input)?;
    println!(
        "install-lifecycle: {} the iteration-scoped merge attributes in {}",
        aaction, attrs
    );
    Ok(())
}

// spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the driver-config step is the
// `--install-hooks` per-clone opt-in class: no repository, or a refused one, is a skip naming its
// cause on stderr at exit 0, since the recorded honest limit depends on it failing soft
fn register_driver() {
    let cause = match crate::walk::toplevel_opt() {
        Ok(Some(_)) => None,
        Ok(None) => Some("not a git repository".to_string()),
        Err(e) => Some(e),
    };
    if let Some(cause) = cause {
        eprintln!("install-lifecycle: {} — skipped the merge.iteration-scoped driver (the .gitattributes attribute stays inert until 'git config merge.iteration-scoped.driver true' is run in a clone)", cause);
        return;
    }
    match proc::run(&programs::GIT, &["config", "merge.iteration-scoped.driver", "true"]) {
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
    use crate::exit;

    // spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the agent file is never minted, so
    // an absent one is exit 3 rather than a create; a flag in the positional slot is a refusal
    #[test]
    fn an_absent_agent_file_refuses_and_never_mints() {
        let _knobs = crate::knobenv::lock();
        let p = std::env::temp_dir()
            .join(format!("no-such-agent-{}.md", std::process::id()))
            .display()
            .to_string();
        let _ = std::fs::remove_file(&p);
        assert_eq!(run(std::slice::from_ref(&p)), exit::REFUSED);
        assert!(!Path::new(&p).exists(), "the refusal minted the agent file");
        assert_eq!(run(&["--nope".to_string()]), exit::USAGE);
    }

    // spec: gate-sdk/SPEC.md §lib/inject.sh — a `.gitattributes` link to an absent path outside
    // the repository refuses before the seed, so the external file is never created and the agent
    // file is untouched
    #[cfg(unix)]
    #[test]
    fn an_escaping_attributes_link_refuses_before_any_write() {
        let _knobs = crate::knobenv::lock();
        let dir = std::env::temp_dir().join(format!("cw-install-lifecycle-seed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let repo = dir.join("repo");
        std::fs::create_dir_all(repo.join(".git")).expect("cannot make the scratch repository");
        let agent = repo.join("AGENTS.md");
        std::fs::write(&agent, "# agent\n").expect("cannot seed the agent file");
        std::os::unix::fs::symlink("../outside.txt", repo.join(ATTRS)).expect("cannot link");
        let got = install_blocks(&agent.display().to_string(), &repo.join(ATTRS).display().to_string());
        let outside = dir.join("outside.txt").exists();
        let kept = std::fs::read_to_string(&agent).expect("read");
        let _ = std::fs::remove_dir_all(&dir);
        assert!(got.is_err_and(|e| e.to_string().contains("refusing to write through it")), "the escaping link did not refuse");
        assert!(!outside, "the seed created the file outside the repository");
        assert_eq!(kept, "# agent\n", "the refusal wrote the agent file");
    }
}
