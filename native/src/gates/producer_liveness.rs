// spec: evidence-kit/SPEC.md §check-producer-liveness — a stage entry is refused while a producer
// named by the record, or by any '*.run' record in the directory, is still alive. It is a wrapper
// through `evidence::pid_alive`, the compiled twin of the library's `ek_pid_alive`, not by own text.
use crate::evidence::{self, LockRead, PidProbe};
use crate::walk;

const NAME: &str = "check-producer-liveness";

// spec: gate-sdk/SPEC.md §Fail-closed contract — the wrapper refusal this member owns text for,
// and the one member of the class where it is a deliberate divergence rather than parity: that
// section and evidence-kit/SPEC.md §check-producer-liveness own the reasoning and the cost.
fn refuse_absent_ps() -> i32 {
    eprintln!("{}: ps not found on PATH — the gate cannot run.", NAME);
    eprintln!("  A gate that cannot run is not clean (fail-closed): without ps a PID that exists");
    eprintln!("  but cannot be signalled is indistinguishable from one that is gone.");
    2
}

// spec: evidence-kit/SPEC.md §check-producer-liveness — the per-record verdict, shared by both
// modes because set mode "adds the quantifier" and re-decides nothing else
enum Verdict {
    Corrupt,
    Free,
    Dead { pid: String, run_key: String },
    Live { pid: String, run_key: String },
}

fn verdict(path: &std::path::Path) -> Result<Verdict, PidProbe> {
    match evidence::lock_read(path) {
        LockRead::Corrupt => Ok(Verdict::Corrupt),
        LockRead::Absent => Ok(Verdict::Free),
        LockRead::Held { pid, run_key } => {
            if evidence::pid_alive(&pid)? {
                Ok(Verdict::Live { pid, run_key })
            } else {
                Ok(Verdict::Dead { pid, run_key })
            }
        }
    }
}

fn probe_failed(e: PidProbe) -> i32 {
    match e {
        PidProbe::PsAbsent => refuse_absent_ps(),
        PidProbe::Spawn(msg) => {
            eprintln!("{}: {}", NAME, msg);
            2
        }
    }
}

// spec: evidence-kit/SPEC.md §check-producer-liveness — set mode: the `*.run` glob quantifies the
// per-record verdict, and the aggregation is the mode's one new decision — exit 2 wins over red
// wins over green, so one corrupt record is never averaged away by clean ones.
fn set_mode(lock: &str) -> i32 {
    let records = walk::glob_entries(&format!("{}/*.run", lock));
    let mut corrupt: Vec<String> = Vec::new();
    let mut blocking: Vec<String> = Vec::new();

    for rec in &records {
        match verdict(std::path::Path::new(rec)) {
            Err(e) => return probe_failed(e),
            Ok(Verdict::Corrupt) => corrupt.push(rec.clone()),
            Ok(Verdict::Free) | Ok(Verdict::Dead { .. }) => {}
            Ok(Verdict::Live { pid, run_key }) => blocking.push(format!(
                "{}: {}: the producer for run key '{}' is still running (pid {})",
                NAME, rec, run_key, pid
            )),
        }
    }

    if !corrupt.is_empty() {
        for rec in &corrupt {
            eprintln!(
                "{}: {} carries no readable 'pid=<n> run=<key>' record",
                NAME, rec
            );
        }
        return 2;
    }

    if !blocking.is_empty() {
        for line in &blocking {
            println!("{}", line);
        }
        println!("  help: wait for each run named above on its own artifact — it is still writing, so anything read now can change underneath you; where its pid is gone the record is a statement of fact that has become false, and deleting that .run file retracts it");
        return 1;
    }

    if records.is_empty() {
        println!(
            "PRODUCER-LIVENESS: clean (no '*.run' record under {} — nothing in flight)",
            lock
        );
    } else {
        println!(
            "PRODUCER-LIVENESS: clean ({} '*.run' record(s) under {}, none naming a live pid — no producer in flight)",
            records.len(),
            lock
        );
    }
    0
}

// spec: evidence-kit/SPEC.md §check-producer-liveness — the single-path mode, left exactly as it
// is: one path, one writer, and the two modes told apart by the argument being a directory
fn path_mode(lock: &str) -> i32 {
    match verdict(std::path::Path::new(lock)) {
        Err(e) => probe_failed(e),
        Ok(Verdict::Corrupt) => {
            eprintln!(
                "{}: {} carries no readable 'pid=<n> run=<key>' record",
                NAME, lock
            );
            2
        }
        Ok(Verdict::Free) => {
            println!(
                "PRODUCER-LIVENESS: clean (no producer lock at {} — nothing in flight)",
                lock
            );
            0
        }
        Ok(Verdict::Live { pid, run_key }) => {
            println!(
                "{}: {}: the evidence producer for run key '{}' is still running (pid {})",
                NAME, lock, run_key, pid
            );
            println!("  help: wait for that run-validate to finish — it is still writing the evidence manifest, so anything read now can change underneath you; if pid {} is gone, the lock is stale and deleting {} clears it", pid, lock);
            1
        }
        Ok(Verdict::Dead { pid, run_key }) => {
            println!(
                "PRODUCER-LIVENESS: clean (lock at {} names dead pid {} for run key '{}' — no producer in flight)",
                lock, pid, run_key
            );
            0
        }
    }
}

pub fn run(args: &[String]) -> i32 {
    // spec: evidence-kit/SPEC.md §check-producer-liveness — `${1:-$EVIDENCE_KIT_LOCK_FILE}`: an
    // empty first positional takes the knob exactly as an absent one does, and every argument past
    // the first is ignored, so the entry hook's trailing `<queue> <state>` passes through.
    let lock = match args.first().filter(|a| !a.is_empty()) {
        Some(a) => a.clone(),
        None => match walk::knob_scalar("EVIDENCE_KIT_LOCK_FILE") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", NAME, e);
                return 2;
            }
        },
    };

    if std::path::Path::new(&lock).is_dir() {
        set_mode(&lock)
    } else {
        path_mode(&lock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::lock_read;

    fn scratch(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!(
            "checkwright-liveness.{}.{}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("cannot make the scratch dir");
        d
    }

    // spec: evidence-kit/SPEC.md §The producer-liveness lock — the grammar's negative arms, which
    // the fixture pair cannot carry: a pair asserts a line's presence and never its absence, and
    // both cases here have to be *files* whose content is the assertion
    #[test]
    fn the_record_grammar_admits_only_a_whole_well_formed_line() {
        let d = scratch("grammar");
        let cases: &[(&str, &str, bool)] = &[
            ("ok", "pid=1234 run=alpha\n", true),
            ("no-newline", "pid=1234 run=alpha", false),
            ("empty", "", false),
            ("garbage", "garbage\n", false),
            ("zero-pid", "pid=0 run=alpha\n", false),
            ("leading-zero", "pid=0123 run=alpha\n", false),
            ("two-spaces", "pid=1234  run=alpha\n", false),
            ("no-run-field", "pid=1234 alpha\n", false),
            ("empty-run-key", "pid=1234 run=\n", false),
            ("trailing-space", "pid=1234 run=alpha \n", false),
            ("carriage-return", "pid=1234 run=alpha\r\n", false),
        ];
        for (name, body, wellformed) in cases {
            let p = d.join(name);
            std::fs::write(&p, body).expect("cannot write the case");
            let held = matches!(lock_read(&p), LockRead::Held { .. });
            assert_eq!(
                held, *wellformed,
                "the record grammar read '{}' as {} — a record read wrong is a lock treated as \
                 free or a free slot treated as corruption",
                name, held
            );
        }
        let _ = std::fs::remove_dir_all(&d);
    }

    // spec: evidence-kit/SPEC.md §The producer-liveness lock — an absent file is the *free*
    // reading and an unreadable one is corruption, and folding either into the other is the
    // reading that section forbids
    #[test]
    fn an_absent_lock_is_free_and_never_corruption() {
        let d = scratch("absent");
        assert!(matches!(lock_read(&d.join("nope")), LockRead::Absent));
        assert!(matches!(lock_read(&d), LockRead::Absent));
        let _ = std::fs::remove_dir_all(&d);
    }

    // spec: evidence-kit/SPEC.md §check-producer-liveness — the pid predicate's non-numeric arms,
    // which never reach a spawn: `ek_pid_alive` returns 1 on them before it probes anything
    #[test]
    fn a_pid_that_is_not_a_pid_is_dead_without_a_spawn() {
        for bad in ["", "0", "01", "12x", " 12", "abc", "-1"] {
            assert!(
                !evidence::pid_alive(bad).unwrap_or(true),
                "the pid predicate accepted {:?}, so a malformed record would probe a process",
                bad
            );
        }
    }

    // spec: evidence-kit/SPEC.md §check-producer-liveness — the liveness leg answers for a process
    // this test owns, the arm the fixture pair can only reach as init
    #[test]
    fn the_liveness_leg_answers_for_a_live_process_and_a_dead_one() {
        assert!(
            evidence::pid_alive(&std::process::id().to_string())
                .expect("the pid probe could not answer"),
            "the probe read this very process as dead"
        );
        assert!(
            !evidence::pid_alive("2147483646").expect("the pid probe could not answer"),
            "the probe read a pid past the system maximum as alive"
        );
    }
}
