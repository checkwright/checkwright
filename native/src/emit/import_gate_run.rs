// spec: evidence-kit/SPEC.md §import-gate-run — a Statement produced elsewhere, verified without
// re-running and recorded as `origin=imported` suite lines under this tree's run key, written as the
// spine writes the manifest: under the producer-liveness claim, folded and published by rename.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because 1 nothing recorded on a
// contradiction, 2 a refusal and 3 an unreadable input are different answers, which the `--emit-`
// collapse would merge.
use super::run_validate::{self, failed, guard, Refusal};
use super::verify_gate_run;
use crate::{evidence, walk};

pub const KNOBS: &[&str] = &[
    "EVIDENCE_KIT_IMPORTED_RUNS",
    "EVIDENCE_KIT_STATEMENT_CMD",
    "EVIDENCE_KIT_SUITES",
    "EVIDENCE_KIT_RUN_*",
    "EVIDENCE_KIT_PARSER",
    "EVIDENCE_KIT_PARSER_*",
    "EVIDENCE_KIT_MANIFEST_FILE",
    "EVIDENCE_KIT_QUEUE_FILE",
    "EVIDENCE_KIT_TMP_DIR",
    "EVIDENCE_KIT_LOCK_FILE",
    "EVIDENCE_KIT_RUN_ID",
];

pub fn run(args: &[String]) -> i32 {
    match import(args) {
        Ok(code) => code,
        Err(r) => {
            eprintln!("import-gate-run: {}", r.message);
            r.code
        }
    }
}

fn import(args: &[String]) -> Result<i32, Refusal> {
    // spec: evidence-kit/SPEC.md §import-gate-run — off unless the consumer selects it, refused
    // before anything is read
    let selected = walk::knob_scalar("EVIDENCE_KIT_IMPORTED_RUNS").map_err(guard)?;
    if selected != "accept" {
        return Err(guard(format!(
            "EVIDENCE_KIT_IMPORTED_RUNS is '{}' — imports are off; select 'accept' to record a verified Statement",
            selected
        )));
    }
    let scalar = |k: &str| walk::knob_scalar(k).map_err(guard);
    let manifest = scalar("EVIDENCE_KIT_MANIFEST_FILE")?;
    let queue = scalar("EVIDENCE_KIT_QUEUE_FILE")?;
    let tmpdir = scalar("EVIDENCE_KIT_TMP_DIR")?;
    let lock = scalar("EVIDENCE_KIT_LOCK_FILE")?;
    let run_id = scalar("EVIDENCE_KIT_RUN_ID")?;

    let bytes = verify_gate_run::read_input("--import-gate-run", args)?;
    let judged = verify_gate_run::judge(&bytes)?;
    judged.report("IMPORT-GATE-RUN");
    if !judged.findings.is_empty() {
        println!("  nothing recorded");
        return Ok(1);
    }

    let queue_text = std::fs::read(&queue).ok().map(|b| String::from_utf8_lossy(&b).into_owned());
    let key = evidence::run_key(queue_text.as_deref(), &run_id).ok_or_else(|| {
        guard(format!("no evidence-line key — name the iteration in {} or set EVIDENCE_KIT_RUN_ID", queue))
    })?;
    run_validate::manifest_guard(&manifest)?;
    std::fs::create_dir_all(&tmpdir).map_err(|e| failed(format!("cannot create {}: {}", tmpdir, e)))?;
    let _claimed = run_validate::claim(&tmpdir, &lock, &key)?;

    let st = &judged.statement;
    let rows: Vec<evidence::SuiteLine> = st
        .suites
        .iter()
        .map(|s| evidence::SuiteLine {
            key: key.clone(),
            suite: s.name.clone(),
            log: s.log.clone(),
            counts: s.counts,
            verdict: s.verdict.clone(),
            date: s.date.clone(),
            commit: st.subject.clone(),
            tree: s.tree.clone(),
            cmd: s.cmd.clone(),
            origin: "imported".to_string(),
        })
        .collect();
    run_validate::fold(&manifest, &tmpdir, &key, &rows, &[]).map_err(failed)?;
    println!(
        "  recorded {} suite line(s) under '{}' in {} (origin=imported)",
        rows.len(),
        key,
        manifest
    );
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: evidence-kit/SPEC.md §import-gate-run — under the default `refuse` the arm refuses
    // before it reads, so a missing file is never reached
    #[test]
    fn the_default_selection_refuses_before_reading() {
        let env = crate::knobenv::lock();
        env.remove("EVIDENCE_KIT_IMPORTED_RUNS");
        let r = import(&["/nonexistent/statement.json".to_string()]).expect_err("an import ran under refuse");
        assert_eq!(r.code, 2);
        assert!(r.message.contains("EVIDENCE_KIT_IMPORTED_RUNS"), "{}", r.message);
    }
}
