// spec: installer/README.md §What init seeds — the crate's owner of the per-kit install recipe:
// the starting gate roster a kit registers in a fresh consumer, and the seam surfaces those gates
// need. Two output channels, and the difference between them is who writes the file.
use super::{AGENT_FILE, GATES_DIR};
use std::path::Path;

// spec: installer/README.md §What init seeds — seed what is absent, plan what must be claimed: a
// surface init creates once is written here only when absent, and one it rewrites every run is
// planned instead, because only the claim can compare the adopter's content before the overwrite.
pub enum Seeded {
    Path(String),
    Plan(String, String),
}

// spec: gate-sdk/SPEC.md §check-reads-couples — a single-level listing through the crate's one
// traversal owner, so no module outside it reaches the filesystem's own walk API.
fn entries(dir: &Path) -> Vec<std::path::PathBuf> {
    crate::walk::list_dir(dir)
        .unwrap_or_default()
        .into_iter()
        .map(|(name, _)| dir.join(name))
        .collect()
}

// spec: installer/README.md §What init seeds — the config seam is derived, never listed: a kit's
// consumer config is whatever `templates/*-config.sh` it ships. It plans and writes nothing — a
// copy landing before the claim hashes the tree destroys the evidence the refusal is computed from.
pub fn config_seam_plan(kit_payload: &Path, gates_dir: &str) -> Vec<(String, String)> {
    entries(&kit_payload.join("templates"))
        .into_iter()
        .filter_map(|t| {
            let base = t.file_name()?.to_string_lossy().into_owned();
            if !base.ends_with("-config.sh") {
                return None;
            }
            Some((
                t.to_string_lossy().into_owned(),
                format!("{}/{}", gates_dir, base),
            ))
        })
        .collect()
}

// spec: gate-sdk/SPEC.md §The install disposition — the gate is the one place that knows whether it
// can run on a tree init just made, so this reads the declaration off each shipped gate instead of
// carrying a second copy of it. A kit that adds a zero-config gate is picked up with no edit here.
pub fn install_disposition(gate_file: &Path) -> String {
    let Ok(text) = std::fs::read_to_string(gate_file) else {
        return String::new();
    };
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("# install:") else {
            continue;
        };
        if !rest.starts_with(|c: char| c.is_whitespace()) {
            continue;
        }
        return rest.split_whitespace().next().unwrap_or("").to_string();
    }
    String::new()
}

// spec: installer/README.md §What init seeds — the starting roster is the subset a fresh consumer
// begins with: a gate whose subject the adopter has not authored yet would exit 2 on their tree, so
// it is registered when that surface exists rather than at install.
// spec: installer/README.md §Profiles — the roster is keyed by profile as well as by kit, so one
// that varies by profile becomes a change to one arm rather than to this signature. Nothing varies
// on it today; the parameter is the seam.
// spec: gate-sdk/SPEC.md §Consumer payload — both declaration spellings are scanned, because a
// ported gate is still a gate a kit ships and its descriptor rides the payload on the same terms.
pub fn gates(kit_payload: &Path, _profile: &str) -> Vec<String> {
    let checks = kit_payload.join("checks");
    if !checks.is_dir() {
        return Vec::new();
    }
    let mut out: Vec<String> = Vec::new();
    for f in entries(&checks) {
        let name = f.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let is_member = name.starts_with("check-") && (name.ends_with(".sh") || name.ends_with(".gate"));
        if !is_member || install_disposition(&f) != "zero-config" {
            continue;
        }
        let stem = match name.rfind('.') {
            Some(i) => name[..i].to_string(),
            None => name,
        };
        out.push(stem);
    }
    out.sort();
    out.dedup();
    out
}

pub fn needs_queue(kit: &str) -> bool {
    matches!(kit, "canon-kit" | "lifecycle-kit" | "queue-kit")
}

// spec: installer/README.md §What init seeds — the queue source is resolved once over the whole kit
// set, never inside a per-kit call blind to the others: a kit declares itself the format's owner by
// shipping `templates/TASK-QUEUE.md`, and `-` is the inline fallback.
pub fn queue_source(payload: &Path, kits: &[String]) -> Option<String> {
    let mut tpl = String::new();
    let mut needs = false;
    for kit in kits {
        if needs_queue(kit) {
            needs = true;
        }
        let candidate = payload.join(kit).join("templates/TASK-QUEUE.md");
        if tpl.is_empty() && candidate.is_file() {
            tpl = candidate.to_string_lossy().into_owned();
        }
    }
    if !needs {
        return None;
    }
    Some(if tpl.is_empty() { "-".to_string() } else { tpl })
}

// spec: installer/README.md §What init seeds — the fallback carries every
// `QUEUE_KIT_REQUIRED_SECTIONS` heading at that knob's default: the section floor is not registered
// at install, so a skeleton missing one passes open here and reds on the day they register it.
pub fn write_queue(src: &str, root: &Path, queue_file: &str) -> Result<(), String> {
    let dest = root.join(queue_file);
    if src != "-" {
        return std::fs::copy(src, &dest)
            .map(|_| ())
            .map_err(|e| format!("cannot write {}: {}", queue_file, e));
    }
    let body = format!(
        "# {}\n\n## Iteration: —\n\n---\n\n## New Features\n\n## Technical Debt\n\n## Deferred\n\n## Done\n\n## Lessons Learned\n",
        queue_file
    );
    std::fs::write(&dest, body).map_err(|e| format!("cannot write {}: {}", queue_file, e))
}

// spec: installer/README.md §What init seeds — this asks whether the agent file must *exist* for a
// kit's starting gates, a narrower question than which kit writes into it: reading this membership
// as the seeding roster is the conflation that section's rule exists to settle.
pub fn needs_agent_file(kit: &str) -> bool {
    matches!(kit, "context-kit" | "doctrine-kit")
}

fn seed_absent(root: &Path, rel: &str, body: &str, out: &mut Vec<Seeded>) -> Result<(), String> {
    let dest = root.join(rel);
    if dest.is_file() {
        return Ok(());
    }
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {}", rel, e))?;
    }
    std::fs::write(&dest, body).map_err(|e| format!("cannot write {}: {}", rel, e))?;
    out.push(Seeded::Path(rel.to_string()));
    Ok(())
}

pub fn seed(kit: &str, kit_payload: &Path, root: &Path) -> Result<Vec<Seeded>, String> {
    let mut out: Vec<Seeded> = Vec::new();
    match kit {
        "gate-sdk" => {
            let src = kit_payload.join("templates/msg-patterns.list");
            if src.is_file() {
                out.push(Seeded::Plan(
                    src.to_string_lossy().into_owned(),
                    format!("{}/msg-patterns.list", GATES_DIR),
                ));
            }
        }
        "evidence-kit" => {
            seed_absent(
                root,
                ".workflow/validate-baseline.txt",
                "# contract: evidence-kit/SPEC.md §Baseline manifest — held-constant validate baseline: <suite> <scenario> <status> [<slug>]\n",
                &mut out,
            )?;
            seed_absent(
                root,
                ".workflow/validate-evidence.txt",
                "# contract: evidence-manifest v1\n",
                &mut out,
            )?;
        }
        "lifecycle-kit" => {
            seed_absent(
                root,
                ".workflow/WORKFLOW-STATE.txt",
                "# contract: lifecycle-kit/SPEC.md §check-stage-evidence\n# One data line per stage-skill invocation: <iteration> <stage> <session-id> <date> <head>.\n\n---\n\n",
                &mut out,
            )?;
        }
        // spec: installer/README.md §What init seeds — this kit has an arm because a gate init
        // registers reads the block it writes; that is the whole test, and why no lifecycle-kit arm
        // sits beside it.
        // spec: doctrine-kit/SPEC.md §install-doctrine — the block is the kit's own installer's to
        // write, and that installer is this binary's `--install-doctrine` arm, called in-process.
        "doctrine-kit" => {
            // spec: doctrine-kit/SPEC.md §install-doctrine — the consumer root goes over as the BASE
            // and the two paths as consumer-relative spellings, because resolving them here would
            // commit this machine's absolute path into the adopter's own agent file.
            let report = crate::doctrine::install_in(root, AGENT_FILE, "doctrine-kit/DOCTRINE.md")
                .map_err(|e| format!("could not seed {}: {}", AGENT_FILE, e))?;
            // spec: doctrine-kit/SPEC.md §install-doctrine — findings go to stderr, the one channel
            // the install path does not discard, so a reconciliation the consumer owes is never
            // silent; the report line is the installer's own and init has its own to print.
            for f in &report.findings {
                eprintln!("install-doctrine: {}", f);
            }
            out.push(Seeded::Path(AGENT_FILE.to_string()));
        }
        _ => {}
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: gate-sdk/SPEC.md §check-install-disposition — every fixture name below is composed from
    // this prefix rather than written out, because assertion C holds this file free of literal gate
    // names and a test corpus spelling one out is indistinguishable to it from a roster.
    const PREFIX: &str = "check";

    // spec: gate-sdk/SPEC.md §The install disposition — the declaration is read off the gate's own
    // header, first line wins, and a header that declares none reads empty rather than defaulting.
    #[test]
    fn the_disposition_is_read_off_the_gates_own_header() {
        let dir = std::env::temp_dir().join(format!("cw-recipe-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(dir.join("checks")).expect("cannot make the scratch tree");
        let write = |name: &str, body: &str| {
            std::fs::write(dir.join("checks").join(name), body).expect("cannot write a scratch gate")
        };
        let zero = format!("{}-a.gate", PREFIX);
        let surface = format!("{}-b.sh", PREFIX);
        let bare = format!("{}-c.gate", PREFIX);
        write(&zero, "# install: zero-config\n# graph: dir=in\n");
        write(&surface, "# install: needs-surface — a glossary\n");
        write(&bare, "# graph: dir=in\n");
        write("notes.md", "# install: zero-config\n");
        assert_eq!(
            install_disposition(&dir.join("checks").join(&zero)),
            "zero-config"
        );
        assert_eq!(install_disposition(&dir.join("checks").join(&bare)), "");
        assert_eq!(gates(&dir, "starter"), vec![format!("{}-a", PREFIX)]);
        assert!(gates(&dir.join("nowhere"), "starter").is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    // spec: installer/README.md §What init seeds — the queue source is resolved over the whole kit
    // set: a set no member of which reads the queue seeds nothing, and one that does but ships no
    // template takes the inline fallback.
    #[test]
    fn the_queue_source_is_resolved_over_the_whole_kit_set() {
        let dir = std::env::temp_dir().join(format!("cw-queue-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("queue-kit/templates")).expect("cannot make the tree");
        std::fs::write(dir.join("queue-kit/templates/TASK-QUEUE.md"), "# q\n")
            .expect("cannot write the scratch template");
        assert_eq!(queue_source(&dir, &["gate-sdk".to_string()]), None);
        assert_eq!(
            queue_source(&dir, &["gate-sdk".to_string(), "canon-kit".to_string()]),
            Some("-".to_string())
        );
        assert!(queue_source(&dir, &["queue-kit".to_string()])
            .expect("the template was not resolved")
            .ends_with("queue-kit/templates/TASK-QUEUE.md"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
