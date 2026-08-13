// spec: lifecycle-kit/SPEC.md §check-lifecycle-registration — the always-loaded agent file
// carries a lifecycle-kit marker block whose content byte-matches the block regenerated from
// the live stage machine, fail-closed when the target or a marker is missing
use crate::diff;
use crate::stages;
use crate::walk;
use std::path::Path;

const BEGIN: &str = "<!-- lifecycle-kit:begin -->";
const END: &str = "<!-- lifecycle-kit:end -->";

pub fn run(args: &[String]) -> i32 {
    let agent = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("LIFECYCLE_KIT_AGENT_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("check-lifecycle-registration: {}", e);
                return 2;
            }
        },
    };
    if !Path::new(&agent).is_file() {
        eprintln!(
            "check-lifecycle-registration: agent file not found: {}",
            agent
        );
        return 2;
    }
    let text = match std::fs::read(&agent) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            eprintln!("check-lifecycle-registration: cannot read {}: {}", agent, e);
            return 2;
        }
    };

    if !text.contains(BEGIN) {
        println!(
            "check-lifecycle-registration: no lifecycle-kit registration block in {}",
            agent
        );
        println!("  help: install the resident registration block into the always-loaded agent file —");
        println!("        bash lifecycle-kit/bin/install-lifecycle.sh — so a session that loads it");
        println!("        is pointed at the stage machine. Override the path with LIFECYCLE_KIT_AGENT_FILE.");
        return 1;
    }
    if !text.contains(END) {
        eprintln!(
            "check-lifecycle-registration: begin marker present but end marker missing in {} — the block bounds are unreadable",
            agent
        );
        return 2;
    }

    let mut present: Vec<&str> = Vec::new();
    let mut inb = false;
    for line in text.lines() {
        if line == BEGIN {
            inb = true;
            continue;
        }
        if line == END {
            inb = false;
            continue;
        }
        if inb {
            present.push(line);
        }
    }

    let block = match stages::registration_block() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("check-lifecycle-registration: {}", e);
            return 2;
        }
    };
    let expected: Vec<&str> = block.lines().collect();
    let stage_count = match stages::stages() {
        Ok(v) => v.len(),
        Err(e) => {
            eprintln!("check-lifecycle-registration: {}", e);
            return 2;
        }
    };

    if present != expected {
        println!(
            "check-lifecycle-registration: the registration block in {} is stale — it does not match the block derived from the live stage machine:",
            agent
        );
        for l in diff::normal_diff(&expected, &present) {
            println!("  {}", l);
        }
        println!("  help: a reshaped stage machine (LIFECYCLE_KIT_STAGES / LIFECYCLE_KIT_QUEUE_FILE) or a");
        println!("        hand-edited block staled the registration — regenerate it in place:");
        println!("        bash lifecycle-kit/bin/install-lifecycle.sh");
        return 1;
    }

    println!(
        "LIFECYCLE-REGISTRATION: clean ({} carries the lifecycle-kit registration block in byte-lockstep with the derived stage machine; {} stage(s))",
        agent, stage_count
    );
    0
}

