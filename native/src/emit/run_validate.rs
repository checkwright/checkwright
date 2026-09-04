// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the codified validate spine: the guards, the
// producer-liveness claim, then each suite run foreground, parsed, diffed and batched, and one fold
// into the tracked manifest after the whole roster has run.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the exit contract is three-state
// and the difference is the product: 1 is the verdict a suite regressed, 2 the run could not start.
use crate::evidence::{self, LockRead, PidProbe};
use crate::proc;
use crate::walk;
use std::io::Write;
use std::path::Path;

// spec: evidence-kit/SPEC.md §Layout and configuration — the declared roster, two of whose names
// are prefix families: a hardcoded top-level flag would resolve this crate's own defaults for every
// one of them and silently ignore each consumer override, which is the forced-family test.
pub const KNOBS: &[&str] = &[
    "EVIDENCE_KIT_SUITES",
    "EVIDENCE_KIT_RUN_*",
    "EVIDENCE_KIT_PARSER",
    "EVIDENCE_KIT_PARSER_*",
    "EVIDENCE_KIT_BASELINE_FILE",
    "EVIDENCE_KIT_MANIFEST_FILE",
    "EVIDENCE_KIT_SKIP_FILE",
    "EVIDENCE_KIT_QUEUE_FILE",
    "EVIDENCE_KIT_STATE_FILE",
    "EVIDENCE_KIT_TMP_DIR",
    "EVIDENCE_KIT_LOCK_FILE",
    "EVIDENCE_KIT_RUN_ID",
    "EVIDENCE_KIT_PRE_HOOK",
];

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the two non-zero exits kept apart by carrying
// the code with the message: the guards' 2 is a start-time verdict about the world, the 1 is a
// result, and a refusal shape holding only text would have to guess which it was.
struct Refusal {
    code: i32,
    message: String,
}

fn guard(message: String) -> Refusal {
    Refusal { code: 2, message }
}

pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(code) => code,
        Err(r) => {
            eprintln!("run-validate: {}", r.message);
            r.code
        }
    }
}

struct Cfg {
    suites: Vec<String>,
    run_family: Vec<(String, String)>,
    parser: String,
    parser_family: Vec<(String, String)>,
    baseline: String,
    manifest: String,
    skip: String,
    queue: String,
    tmpdir: String,
    lock: String,
    run_id: String,
    pre_hook: String,
}

fn config() -> Result<Cfg, Refusal> {
    let scalar = |k: &str| walk::knob_scalar(k).map_err(guard);
    Ok(Cfg {
        suites: walk::knob_array("EVIDENCE_KIT_SUITES").map_err(guard)?,
        run_family: walk::knob_prefix("EVIDENCE_KIT_RUN_"),
        parser: scalar("EVIDENCE_KIT_PARSER")?,
        parser_family: walk::knob_prefix("EVIDENCE_KIT_PARSER_"),
        baseline: scalar("EVIDENCE_KIT_BASELINE_FILE")?,
        manifest: scalar("EVIDENCE_KIT_MANIFEST_FILE")?,
        skip: scalar("EVIDENCE_KIT_SKIP_FILE")?,
        queue: scalar("EVIDENCE_KIT_QUEUE_FILE")?,
        tmpdir: scalar("EVIDENCE_KIT_TMP_DIR")?,
        lock: scalar("EVIDENCE_KIT_LOCK_FILE")?,
        run_id: scalar("EVIDENCE_KIT_RUN_ID")?,
        pre_hook: scalar("EVIDENCE_KIT_PRE_HOOK")?,
    })
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the whole input arrives through the bridged
// environment, so the member takes no positional at all: there is no free text for the argv-shape
// refusal or the `--` escape to bind on, and usage lives in the front-end.
fn dispatch(_args: &[String]) -> Result<i32, Refusal> {
    let cfg = config()?;

    if cfg.suites.is_empty() {
        return Err(guard(
            "no suites configured (EVIDENCE_KIT_SUITES) — nothing to run".to_string(),
        ));
    }
    let queue_text = std::fs::read(&cfg.queue)
        .ok()
        .map(|b| String::from_utf8_lossy(&b).into_owned());
    let Some(key) = evidence::run_key(queue_text.as_deref(), &cfg.run_id) else {
        return Err(guard(format!(
            "no evidence-line key — name the iteration in {} or set EVIDENCE_KIT_RUN_ID",
            cfg.queue
        )));
    };
    if !Path::new(&cfg.manifest).is_file() {
        return Err(guard(format!(
            "manifest not found: {} (seed it with a '# contract: {}' header)",
            cfg.manifest,
            evidence::MANIFEST_CONTRACT
        )));
    }

    std::fs::create_dir_all(&cfg.tmpdir)
        .map_err(|e| guard(format!("cannot create {}: {}", cfg.tmpdir, e)))?;
    let today = date_today().map_err(guard)?;

    let mut claimed = claim(&cfg, &key)?;

    // spec: evidence-kit/SPEC.md §Evidence manifest — the batch joins the claim's own destructor, so
    // a run that dies between the last suite and the fold leaves no orphan under the scratch dir.
    let batch = format!(
        "{}/validate-evidence-batch.{}",
        cfg.tmpdir,
        std::process::id()
    );
    std::fs::write(&batch, b"")
        .map_err(|e| guard(format!("cannot create the batch file {}: {}", batch, e)))?;
    claimed.batch = Some(batch.clone());

    let mut overall = 0;
    let mut rows: Vec<String> = Vec::new();
    let baseline_text = read_or_empty(&cfg.baseline);
    let skip_text = if !cfg.skip.is_empty() && Path::new(&cfg.skip).is_file() {
        read_or_empty(&cfg.skip)
    } else {
        String::new()
    };

    for suite in &cfg.suites {
        let cmd = evidence::suite_cmd(&cfg.run_family, suite);
        if cmd.is_empty() {
            return Err(guard(format!(
                "suite '{}' has no EVIDENCE_KIT_RUN_{} command configured",
                suite, suite
            )));
        }

        // spec: evidence-kit/SPEC.md §bin/run-validate.sh — a failing pre-hook aborts the run at the
        // guards' code with no evidence appended, and that ordering is the contract: it is what
        // keeps a refused run from writing a line.
        if !cfg.pre_hook.is_empty() {
            let status = spawn(&cfg.pre_hook, Some(suite), &proc::Sink::Inherit).map_err(guard)?;
            if status != 0 {
                return Err(guard(format!(
                    "pre-hook failed for suite '{}' — aborting (no evidence appended)",
                    suite
                )));
            }
        }

        let log = format!("{}/validate-{}.log", cfg.tmpdir, suite);
        let status = spawn(&cmd, None, &proc::Sink::File(log.clone().into())).map_err(guard)?;
        if status != 0 {
            eprintln!(
                "run-validate: suite '{}' exited {} (log: {})",
                suite, status, log
            );
        }

        let parser = evidence::parser_for(&cfg.parser_family, suite, &cfg.parser);
        let lines = evidence::parse(suite, Path::new(&log), status, &parser).map_err(guard)?;
        let parsed = format!("{}/validate-{}.parsed", cfg.tmpdir, suite);
        write_lines(&parsed, &lines).map_err(guard)?;
        if lines.is_empty() {
            return Err(Refusal {
                code: 1,
                message: format!(
                    "parser '{}' produced no result for suite '{}' (log: {}) — a run failure, not an empty diff",
                    parser, suite, log
                ),
            });
        }

        let npass = lines.iter().filter(|l| l.ends_with(" pass")).count();
        let nfail = lines.iter().filter(|l| l.ends_with(" fail")).count();
        let nignore = lines.iter().filter(|l| l.ends_with(" ignore")).count();

        let observed = lines.join("\n") + "\n";
        let verdict = if evidence::diff(&baseline_text, suite, &observed, &skip_text).new_failure {
            overall = 1;
            "new-failures"
        } else {
            "clean"
        };

        let hash = crate::sha256::file_hex(Path::new(&log)).map_err(guard)?;
        let row = format!(
            "{} {} sha256={} pass={} fail={} ignore={} verdict={} {}",
            key, suite, hash, npass, nfail, nignore, verdict, today
        );
        append_line(&batch, &row).map_err(guard)?;
        rows.push(row);

        println!(
            "run-validate: {} -> {} (pass={} fail={} ignore={})",
            suite, verdict, npass, nfail, nignore
        );
    }

    fold(&cfg.manifest, &cfg.tmpdir, &key, &rows).map_err(guard)?;
    Ok(overall)
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the single fold: this iteration's prior line for
// every suite the run covered is superseded and the batch re-appended in roster order. It publishes
// by rename, which is what keeps §The producer-liveness lock's torn-read claim true.
fn fold(manifest: &str, tmpdir: &str, key: &str, rows: &[String]) -> Result<(), String> {
    let superseded: Vec<&str> = rows
        .iter()
        .filter_map(|r| r.split_whitespace().nth(1))
        .collect();
    let text = read_or_empty(manifest);
    let mut out = String::new();
    for line in text.lines() {
        let mut f = line.split_whitespace();
        let same_key = f.next() == Some(key);
        let covered = f.next().is_some_and(|s| superseded.contains(&s));
        if same_key && covered {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    for r in rows {
        out.push_str(r);
        out.push('\n');
    }
    let staged = format!("{}/validate-evidence.{}", tmpdir, std::process::id());
    std::fs::write(&staged, out).map_err(|e| format!("cannot write {}: {}", staged, e))?;
    std::fs::rename(&staged, manifest).map_err(|e| {
        let _ = std::fs::remove_file(&staged);
        format!("cannot publish {}: {}", manifest, e)
    })
}

// spec: evidence-kit/SPEC.md §The producer-liveness lock — the claim sits after the guards and the
// scratch creation and before any evidence work; the record is built whole and hard-linked into
// place, the link being that section's asserted mechanism rather than one spelling of atomicity.
fn claim(cfg: &Cfg, key: &str) -> Result<Claimed, Refusal> {
    let pid = std::process::id();
    let tmp = format!("{}/run-validate-lock.{}", cfg.tmpdir, pid);
    std::fs::write(&tmp, format!("pid={} run={}\n", pid, key))
        .map_err(|e| guard(format!("cannot stage the lock record {}: {}", tmp, e)))?;
    let refuse = |m: String| {
        let _ = std::fs::remove_file(&tmp);
        guard(m)
    };

    let mut reclaimed = false;
    while std::fs::hard_link(&tmp, &cfg.lock).is_err() {
        match evidence::lock_read(Path::new(&cfg.lock)) {
            LockRead::Corrupt => {
                return Err(refuse(format!(
                    "the lock {} carries no readable 'pid=<n> run=<key>' record — refusing to \
                     start; delete it if no producer is running",
                    cfg.lock
                )))
            }
            LockRead::Held {
                pid: holder,
                run_key,
            } => match evidence::pid_alive(&holder) {
                Ok(true) => {
                    return Err(refuse(format!(
                        "a producer is already running for run key '{}' (pid {}) — refusing to \
                         start; wait for that run to finish, or delete {} once pid {} is gone",
                        run_key, holder, cfg.lock, holder
                    )))
                }
                Ok(false) => {}
                // spec: evidence-kit/SPEC.md §The producer-liveness lock — the writer takes the
                // readers' disposition rather than the shell form's: reclaiming a holder the probe
                // could not classify is the false *free* answer that section forbids.
                Err(e) => return Err(refuse(probe_message(e, &cfg.lock))),
            },
            LockRead::Absent => {}
        }
        // spec: evidence-kit/SPEC.md §bin/run-validate.sh — a dead or vanished holder is reclaimed
        // exactly once; a second failed claim means the slot is not ours to take, so refusing beats
        // looping — both contenders may relink, exactly one wins, and the loser re-reads a live PID.
        if reclaimed {
            return Err(refuse(format!(
                "could not claim {} after reclaiming a stale lock — refusing to start rather than \
                 retrying; another producer won the reclaim race, or that path is not writable",
                cfg.lock
            )));
        }
        reclaimed = true;
        let _ = std::fs::remove_file(&cfg.lock);
    }
    let _ = std::fs::remove_file(&tmp);
    Ok(Claimed {
        lock: cfg.lock.clone(),
        batch: None,
        pid: pid.to_string(),
    })
}

fn probe_message(e: PidProbe, lock: &str) -> String {
    match e {
        PidProbe::PsAbsent => format!(
            "ps not found on PATH — the holder of {} cannot be classified, and a lock that cannot \
             be read as free must not be reclaimed; refusing to start",
            lock
        ),
        PidProbe::Spawn(m) => m,
    }
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the reclaim-on-every-path property the shell
// form spelled as an `EXIT` trap, spelled here as the destructor `--enter-stage` already uses; the
// release stays conditional, because a freed slot may already carry a second producer's record.
struct Claimed {
    lock: String,
    batch: Option<String>,
    pid: String,
}

impl Drop for Claimed {
    fn drop(&mut self) {
        if let Some(b) = &self.batch {
            let _ = std::fs::remove_file(b);
        }
        if let LockRead::Held { pid, .. } = evidence::lock_read(Path::new(&self.lock)) {
            if pid == self.pid {
                let _ = std::fs::remove_file(&self.lock);
            }
        }
    }
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — a configured command word-splits by design: the
// suite runner, the parser and the pre-hook are consumer seams the port may not narrow, so the
// value's words are the argv and this member's spawned-program set is the consumer's to widen.
fn spawn(command: &str, operand: Option<&str>, sink: &proc::Sink) -> Result<i32, String> {
    let mut words: Vec<&str> = command.split_whitespace().collect();
    if words.is_empty() {
        return Ok(0);
    }
    let program = words.remove(0);
    if let Some(o) = operand {
        words.push(o);
    }
    proc::run_to(program, &words, sink)
}

fn date_today() -> Result<String, String> {
    let c = proc::run("date", &["+%F"])?;
    match c.stdout() {
        Some(o) => Ok(String::from_utf8_lossy(o).trim().to_string()),
        None => Err("could not read today's date — nothing recorded.".to_string()),
    }
}

fn read_or_empty(path: &str) -> String {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default()
}

fn write_lines(path: &str, lines: &[String]) -> Result<(), String> {
    let mut out = String::new();
    for l in lines {
        out.push_str(l);
        out.push('\n');
    }
    std::fs::write(path, out).map_err(|e| format!("cannot write {}: {}", path, e))
}

fn append_line(path: &str, line: &str) -> Result<(), String> {
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|e| format!("cannot append to {}: {}", path, e))?;
    writeln!(f, "{}", line).map_err(|e| format!("cannot append to {}: {}", path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: evidence-kit/SPEC.md §Evidence manifest — the fold's two halves, which no gate holds:
    // this iteration's prior line is dropped for a suite the run covered and kept for one it did
    // not, and another iteration's line for the same suite survives either way.
    #[test]
    fn the_fold_supersedes_by_key_and_suite_together() {
        let dir = std::env::temp_dir().join(format!("cw-fold-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let m = dir.join("manifest").display().to_string();
        std::fs::write(
            &m,
            "# contract: evidence-manifest v1\n\
             iter_a gates sha256=old pass=1 fail=0 ignore=0 verdict=clean 2026-01-01\n\
             iter_a demo sha256=keep pass=1 fail=0 ignore=0 verdict=clean 2026-01-01\n\
             iter_b gates sha256=other pass=1 fail=0 ignore=0 verdict=clean 2026-01-01\n",
        )
        .unwrap();
        let rows = vec![
            "iter_a gates sha256=new pass=2 fail=0 ignore=0 verdict=clean 2026-02-02".to_string(),
        ];
        fold(&m, &dir.display().to_string(), "iter_a", &rows).unwrap();
        let got = std::fs::read_to_string(&m).unwrap();
        assert_eq!(
            got,
            "# contract: evidence-manifest v1\n\
             iter_a demo sha256=keep pass=1 fail=0 ignore=0 verdict=clean 2026-01-01\n\
             iter_b gates sha256=other pass=1 fail=0 ignore=0 verdict=clean 2026-01-01\n\
             iter_a gates sha256=new pass=2 fail=0 ignore=0 verdict=clean 2026-02-02\n"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
