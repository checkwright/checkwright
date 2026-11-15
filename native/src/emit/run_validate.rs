// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the codified validate spine: the guards, the
// producer-liveness claim, then each suite run foreground, parsed, diffed and batched, and one fold
// into the tracked manifest after the whole roster has run.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the exit contract is three-state
// and the difference is the product: 1 is the verdict a suite regressed, 2 the run could not start.
use crate::evidence::{self, LockRead, PidProbe};
use crate::{proc, programs};
use crate::walk;
use std::io::Write;
use std::path::Path;

// spec: evidence-kit/SPEC.md §Layout and configuration — the declared roster, two of whose names
// are prefix families: a hardcoded top-level flag has no row to declare them on, so the knob-file derivation
// could not see one of them, which is the forced-family test.
pub const KNOBS: &[&str] = &[
    "EVIDENCE_KIT_SUITES",
    "EVIDENCE_KIT_RUN_*",
    "EVIDENCE_KIT_PARSER",
    "EVIDENCE_KIT_PARSER_*",
    "EVIDENCE_KIT_BASELINE_FILE",
    "EVIDENCE_KIT_MANIFEST_FILE",
    "EVIDENCE_KIT_SKIP_FILE",
    "EVIDENCE_KIT_QUEUE_FILE",
    "EVIDENCE_KIT_TMP_DIR",
    "EVIDENCE_KIT_LOCK_FILE",
    "EVIDENCE_KIT_RUN_ID",
    "EVIDENCE_KIT_PRE_HOOK",
    "EVIDENCE_KIT_STATEMENT_FILE",
    "GATE_SDK_KIT_DIRS",
];

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the two non-zero exits kept apart by carrying
// the code with the message: the guards' 2 is a start-time verdict about the world, the 1 is a
// result, and a refusal shape holding only text would have to guess which it was.
pub(crate) struct Refusal {
    pub(crate) code: i32,
    pub(crate) message: String,
}

pub(crate) fn guard(message: String) -> Refusal {
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
    statement: String,
}

fn config() -> Result<Cfg, Refusal> {
    let scalar = |k: &str| walk::knob_scalar(k).map_err(guard);
    Ok(Cfg {
        suites: walk::knob_array("EVIDENCE_KIT_SUITES").map_err(guard)?,
        run_family: walk::knob_prefix("EVIDENCE_KIT_RUN_").map_err(guard)?,
        parser: scalar("EVIDENCE_KIT_PARSER")?,
        parser_family: walk::knob_prefix("EVIDENCE_KIT_PARSER_").map_err(guard)?,
        baseline: scalar("EVIDENCE_KIT_BASELINE_FILE")?,
        manifest: scalar("EVIDENCE_KIT_MANIFEST_FILE")?,
        skip: scalar("EVIDENCE_KIT_SKIP_FILE")?,
        queue: scalar("EVIDENCE_KIT_QUEUE_FILE")?,
        tmpdir: scalar("EVIDENCE_KIT_TMP_DIR")?,
        lock: scalar("EVIDENCE_KIT_LOCK_FILE")?,
        run_id: scalar("EVIDENCE_KIT_RUN_ID")?,
        pre_hook: scalar("EVIDENCE_KIT_PRE_HOOK")?,
        statement: scalar("EVIDENCE_KIT_STATEMENT_FILE")?,
    })
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the guard reads the header before the claim: a v1
// header is the fold's to rewrite, a v2 one is kept, and any other refuses with nothing run
pub(crate) fn manifest_guard(manifest: &str) -> Result<(), String> {
    if !Path::new(manifest).is_file() {
        return Err(format!(
            "manifest not found: {} (seed it with a '# contract: {}' header)",
            manifest,
            evidence::MANIFEST_CONTRACT
        ));
    }
    let text = std::fs::read(manifest)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| format!("cannot read {}: {}", manifest, e))?;
    match evidence::manifest_header(&text) {
        evidence::Header::V2 | evidence::Header::V1 => Ok(()),
        evidence::Header::Unknown(found) => Err(format!(
            "manifest {} {} — want '# contract: {}' (a legacy '# contract: {}' is rewritten at the fold); \
             nothing run, nothing written",
            manifest,
            if text.is_empty() {
                "is empty, with no header".to_string()
            } else {
                format!("opens with '{}', a header naming no major this writer knows", found)
            },
            evidence::MANIFEST_CONTRACT,
            evidence::MANIFEST_CONTRACT_V1
        )),
    }
}

// spec: evidence-kit/SPEC.md §Evidence manifest — `commit=` and `tree=` for the suite about to run,
// `-` for both on an unborn branch; a worktree git could not read refuses rather than guessing
fn head_state() -> Result<(String, String), String> {
    let Some(commit) = crate::provenance::head() else {
        return Ok(("-".to_string(), "-".to_string()));
    };
    let tree = match crate::provenance::worktree_clean() {
        Some(true) => "clean",
        Some(false) => "modified",
        None => return Err("cannot read the worktree state (git status) — nothing recorded".to_string()),
    };
    Ok((commit, tree.to_string()))
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the whole input arrives as knobs, so the
// member takes no argument and any word is refused before the lock is claimed: a `--help` read as
// nothing would start a full producer run.
fn dispatch(args: &[String]) -> Result<i32, Refusal> {
    if let Some(a) = args.first() {
        return Err(guard(format!(
            "takes no arguments (got: {}) — usage: --run-validate   (it takes no argument)",
            a
        )));
    }
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
    manifest_guard(&cfg.manifest).map_err(guard)?;

    std::fs::create_dir_all(&cfg.tmpdir)
        .map_err(|e| guard(format!("cannot create {}: {}", cfg.tmpdir, e)))?;
    let today = date_today().map_err(guard)?;

    let mut claimed = claim(&cfg.tmpdir, &cfg.lock, &key)?;

    // spec: evidence-kit/SPEC.md §bin/run-validate.sh — the Statement's subject and provenance are
    // read once, after the claim; an unborn branch has no subject, so it writes none
    let attest = if cfg.statement.is_empty() {
        None
    } else {
        let provenance = crate::provenance::collect(&walk::kit_roots().unwrap_or_default());
        match provenance.commit.clone() {
            Some(subject) => Some((subject, provenance.block())),
            None => {
                eprintln!(
                    "run-validate: HEAD names no commit (an unborn branch) — no Statement written to {}",
                    cfg.statement
                );
                None
            }
        }
    };

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
    let mut rows: Vec<evidence::SuiteLine> = Vec::new();
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
            let status = spawn("EVIDENCE_KIT_PRE_HOOK", &cfg.pre_hook, Some(suite), &proc::Sink::Inherit)
                .map_err(guard)?;
            if status != 0 {
                return Err(guard(format!(
                    "pre-hook failed for suite '{}' — aborting (no evidence appended)",
                    suite
                )));
            }
        }

        let (commit, tree) = head_state().map_err(guard)?;
        // spec: evidence-kit/SPEC.md §bin/run-validate.sh — a Statement never attests a suite at a
        // commit other than its subject, so a pre-hook that moved `HEAD` refuses the whole run
        if let Some((subject, _)) = &attest {
            if &commit != subject {
                return Err(guard(format!(
                    "suite '{}' would run at {} but the Statement's subject is {} — its pre-hook moved HEAD; \
                     aborting (nothing folded, no Statement)",
                    suite, commit, subject
                )));
            }
        }

        let parser = evidence::parser_for(&cfg.parser_family, suite, &cfg.parser);
        let log = format!("{}/validate-{}.log", cfg.tmpdir, suite);
        let status = spawn("EVIDENCE_KIT_RUN_*", &cmd, None, &proc::Sink::File(log.clone().into()))
            .map_err(guard)?;
        if status != 0 {
            eprintln!(
                "run-validate: suite '{}' exited {} (log: {})",
                suite, status, log
            );
        }

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

        let row = evidence::SuiteLine {
            key: key.clone(),
            suite: suite.clone(),
            log: crate::sha256::file_hex(Path::new(&log)).map_err(guard)?,
            counts: (npass as u64, nfail as u64, nignore as u64),
            verdict: verdict.to_string(),
            date: today.clone(),
            commit,
            tree,
            cmd: evidence::command_digest(&cmd, &parser),
            origin: "local".to_string(),
        };
        append_line(&batch, &row.render()).map_err(guard)?;
        rows.push(row);

        println!(
            "run-validate: {} -> {} (pass={} fail={} ignore={})",
            suite, verdict, npass, nfail, nignore
        );
    }

    // spec: evidence-kit/SPEC.md §bin/run-validate.sh — staged before the fold so a staging failure
    // writes nothing; the manifest publishes first and the Statement second, each by rename
    let staged = match &attest {
        None => None,
        Some((subject, block)) => {
            let target = Path::new(&cfg.statement);
            let parent = target.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
            if !parent.is_dir() {
                return Err(guard(format!(
                    "the Statement's directory {} does not exist — nothing recorded",
                    parent.display()
                )));
            }
            let staged = format!("{}/validate-statement.{}", cfg.tmpdir, std::process::id());
            let body = evidence::statement(&key, subject, block.clone(), &rows) + "\n";
            claimed.statement = Some(staged.clone());
            std::fs::write(&staged, body)
                .map_err(|e| guard(format!("cannot stage the Statement {}: {} — nothing recorded", staged, e)))?;
            Some(staged)
        }
    };
    fold(&cfg.manifest, &cfg.tmpdir, &key, &rows).map_err(guard)?;
    if let Some(staged) = staged {
        std::fs::rename(&staged, &cfg.statement).map_err(|e| {
            guard(format!(
                "the manifest {} is published but the Statement {} is not: {}",
                cfg.manifest, cfg.statement, e
            ))
        })?;
        println!("run-validate: Statement written to {}", cfg.statement);
    }
    Ok(overall)
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the single fold, published by rename; a v1 file
// is rewritten at v2 with its v1 lines dropped
pub(crate) fn fold(manifest: &str, tmpdir: &str, key: &str, rows: &[evidence::SuiteLine]) -> Result<(), String> {
    let superseded: Vec<&str> = rows.iter().map(|r| r.suite.as_str()).collect();
    let text = read_or_empty(manifest);
    let legacy = match evidence::manifest_header(&text) {
        evidence::Header::V2 => false,
        evidence::Header::V1 => true,
        evidence::Header::Unknown(found) => {
            return Err(format!("manifest {} lost its header under the claim (found '{}')", manifest, found))
        }
    };
    let mut out = String::new();
    for (i, line) in text.lines().enumerate() {
        if legacy {
            if i == 0 {
                out.push_str(&format!("# contract: {}\n", evidence::MANIFEST_CONTRACT));
            } else if evidence::data_lines(line).is_empty() {
                out.push_str(line);
                out.push('\n');
            }
            continue;
        }
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
        out.push_str(&r.render());
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
pub(crate) fn claim(tmpdir: &str, lock: &str, key: &str) -> Result<Claimed, Refusal> {
    let pid = std::process::id();
    let tmp = format!("{}/run-validate-lock.{}", tmpdir, pid);
    std::fs::write(&tmp, format!("pid={} run={}\n", pid, key))
        .map_err(|e| guard(format!("cannot stage the lock record {}: {}", tmp, e)))?;
    let refuse = |m: String| {
        let _ = std::fs::remove_file(&tmp);
        guard(m)
    };

    let mut reclaimed = false;
    while std::fs::hard_link(&tmp, lock).is_err() {
        match evidence::lock_read(Path::new(lock)) {
            LockRead::Corrupt => {
                return Err(refuse(format!(
                    "the lock {} carries no readable 'pid=<n> run=<key>[ handoff=<path>]' record — refusing to \
                     start; delete it if no producer is running",
                    lock
                )))
            }
            LockRead::Held {
                pid: holder,
                run_key,
                ..
            } => match evidence::pid_alive(&holder) {
                Ok(true) => {
                    return Err(refuse(format!(
                        "a producer is already running for run key '{}' (pid {}) — refusing to \
                         start; wait for that run to finish, or delete {} once pid {} is gone",
                        run_key, holder, lock, holder
                    )))
                }
                Ok(false) => {}
                // spec: evidence-kit/SPEC.md §The producer-liveness lock — the writer takes the
                // readers' disposition rather than the shell form's: reclaiming a holder the probe
                // could not classify is the false *free* answer that section forbids.
                Err(e) => return Err(refuse(probe_message(e, lock))),
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
                lock
            )));
        }
        reclaimed = true;
        let _ = std::fs::remove_file(lock);
    }
    let _ = std::fs::remove_file(&tmp);
    Ok(Claimed {
        lock: lock.to_string(),
        batch: None,
        statement: None,
        pid: pid.to_string(),
    })
}

#[cfg_attr(unix, allow(unused_variables))]
fn probe_message(e: PidProbe, lock: &str) -> String {
    match e {
        #[cfg(not(unix))]
        PidProbe::PsAbsent => format!(
            "ps not found on PATH — the holder of {} cannot be classified, and a lock that cannot \
             be read as free must not be reclaimed; refusing to start",
            lock
        ),
        PidProbe::Unanswered(m) => m,
    }
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the reclaim-on-every-path property the shell
// form spelled as an `EXIT` trap, spelled here as the destructor `--enter-stage` already uses; the
// release stays conditional, because a freed slot may already carry a second producer's record.
pub(crate) struct Claimed {
    lock: String,
    batch: Option<String>,
    statement: Option<String>,
    pid: String,
}

impl Drop for Claimed {
    fn drop(&mut self) {
        for staged in [&self.batch, &self.statement].into_iter().flatten() {
            let _ = std::fs::remove_file(staged);
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
fn spawn(
    ground: &'static str,
    command: &str,
    operand: Option<&str>,
    sink: &proc::Sink,
) -> Result<i32, String> {
    let mut words: Vec<&str> = command.split_whitespace().collect();
    if words.is_empty() {
        return Ok(0);
    }
    let program = programs::Program::consumer(ground, words.remove(0));
    if let Some(o) = operand {
        words.push(o);
    }
    proc::run_to(&program, &words, sink)
}

fn date_today() -> Result<String, String> {
    let today = crate::emit::kpi::today_iso();
    if today.is_empty() {
        return Err("could not read today's date — nothing recorded.".to_string());
    }
    Ok(today)
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

    #[test]
    fn an_argument_is_refused_before_any_config_is_read() {
        for word in ["--help", "--nope", "suite"] {
            let err = dispatch(&[word.to_string()]).expect_err("an argument ran the spine");
            assert_eq!(err.code, 2);
            assert!(err.message.contains(word), "{}", err.message);
        }
    }

    // spec: evidence-kit/SPEC.md §Evidence manifest — the fold's two halves, which no gate holds:
    // this iteration's prior line is dropped for a suite the run covered and kept for one it did
    // not, and another iteration's line for the same suite survives either way.
    fn row() -> evidence::SuiteLine {
        evidence::SuiteLine {
            key: "iter_a".into(),
            suite: "gates".into(),
            log: "new".into(),
            counts: (2, 0, 0),
            verdict: "clean".into(),
            date: "2026-02-02".into(),
            commit: "-".into(),
            tree: "-".into(),
            cmd: "c".into(),
            origin: "local".into(),
        }
    }

    fn folded(name: &str, before: &str) -> String {
        let dir = std::env::temp_dir().join(format!("cw-fold-{}-{}", name, std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let m = dir.join("manifest").display().to_string();
        std::fs::write(&m, before).unwrap();
        fold(&m, &dir.display().to_string(), "iter_a", &[row()]).unwrap();
        let got = std::fs::read_to_string(&m).unwrap();
        std::fs::remove_dir_all(&dir).ok();
        got
    }

    #[test]
    fn the_fold_supersedes_by_key_and_suite_together() {
        let tail = "commit=- tree=- cmd=c origin=local";
        let got = folded(
            "v2",
            &format!(
                "# contract: evidence-manifest v2\n\
                 iter_a gates sha256=old pass=1 fail=0 ignore=0 verdict=clean 2026-01-01 {t}\n\
                 iter_a demo sha256=keep pass=1 fail=0 ignore=0 verdict=clean 2026-01-01 {t}\n\
                 iter_b gates sha256=other pass=1 fail=0 ignore=0 verdict=clean 2026-01-01 {t}\n",
                t = tail
            ),
        );
        assert_eq!(
            got,
            format!(
                "# contract: evidence-manifest v2\n\
                 iter_a demo sha256=keep pass=1 fail=0 ignore=0 verdict=clean 2026-01-01 {t}\n\
                 iter_b gates sha256=other pass=1 fail=0 ignore=0 verdict=clean 2026-01-01 {t}\n\
                 iter_a gates sha256=new pass=2 fail=0 ignore=0 verdict=clean 2026-02-02 {t}\n",
                t = tail
            )
        );
    }

    // spec: evidence-kit/SPEC.md §Evidence manifest — the migration: a v1 header is rewritten at v2
    // and every v1 data line dropped, the suite the run did not cover included; a comment survives
    #[test]
    fn the_fold_rewrites_a_v1_file_at_v2_dropping_its_lines() {
        let got = folded(
            "v1",
            "# contract: evidence-manifest v1\n\
             # a note\n\
             iter_a gates sha256=old pass=1 fail=0 ignore=0 verdict=clean 2026-01-01\n\
             iter_a demo sha256=keep pass=1 fail=0 ignore=0 verdict=clean 2026-01-01\n",
        );
        assert_eq!(
            got,
            "# contract: evidence-manifest v2\n\
             # a note\n\
             iter_a gates sha256=new pass=2 fail=0 ignore=0 verdict=clean 2026-02-02 commit=- tree=- cmd=c origin=local\n"
        );
    }

    // spec: evidence-kit/SPEC.md §Evidence manifest — any header but the two majors refuses before
    // the claim, the file left as found
    #[test]
    fn the_guard_refuses_an_unknown_header_and_a_headerless_file() {
        let dir = std::env::temp_dir().join(format!("cw-guard-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let m = dir.join("manifest").display().to_string();
        for (body, ok) in [
            ("# contract: evidence-manifest v2\n", true),
            ("# contract: evidence-manifest v1\n", true),
            ("# contract: evidence-manifest v3\n", false),
            ("k s sha256=x\n", false),
            ("", false),
        ] {
            std::fs::write(&m, body).unwrap();
            assert_eq!(manifest_guard(&m).is_ok(), ok, "{:?}", body);
            assert_eq!(std::fs::read_to_string(&m).unwrap(), body);
        }
        std::fs::remove_dir_all(&dir).ok();
    }
}
