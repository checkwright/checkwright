// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the codified validate spine: the guards, the
// producer-liveness claim, then each suite run foreground, parsed, diffed and batched, and one fold
// into the tracked manifest after the whole roster has run.
// spec: gate-sdk/SPEC.md §The non-gate arm — an `Arm::Run` because the exit contract is four-state
// and the difference is the product: 1 is the verdict a suite regressed, 2 a refused invocation, 3
// a run that could not start.
use crate::evidence::{self, LockRead, PidProbe};
use crate::exit::{self, Fault};
use crate::{proc, programs};
use crate::walk;
use std::io::Write;
use std::path::Path;

// spec: evidence-kit/SPEC.md §Layout and configuration — the declared roster, three of whose names
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
    "EVIDENCE_KIT_STATEMENT_WRITE_CMD",
    "EVIDENCE_KIT_TREE_UNTRACKED",
    "EVIDENCE_KIT_CONTROLS",
    "EVIDENCE_KIT_CONTROL_*",
    "GATE_SDK_KIT_DIRS",
];

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — the non-zero exits kept apart by carrying the
// code with the message: a guard's 2 or 3 is a start-time verdict about the request or the world,
// the 1 is a result, and a refusal shape holding only text would have to guess which it was.
pub(crate) struct Refusal {
    pub(crate) code: i32,
    pub(crate) message: String,
}

pub(crate) fn guard(message: String) -> Refusal {
    Refusal { code: exit::REFUSED, message }
}

pub(crate) fn unreadable(message: String) -> Refusal {
    Refusal { code: exit::INPUT, message }
}

pub(crate) fn failed(message: String) -> Refusal {
    Refusal { code: exit::TOOL, message }
}

impl From<Fault> for Refusal {
    fn from(f: Fault) -> Self {
        Refusal { code: f.status(), message: f.message }
    }
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
    write_cmd: Vec<String>,
    untracked: String,
    controls: Vec<String>,
    control_family: Vec<(String, String)>,
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
        write_cmd: walk::knob_array("EVIDENCE_KIT_STATEMENT_WRITE_CMD").map_err(guard)?,
        untracked: scalar("EVIDENCE_KIT_TREE_UNTRACKED")?,
        controls: walk::knob_array("EVIDENCE_KIT_CONTROLS").map_err(guard)?,
        control_family: walk::knob_prefix(evidence::CONTROL_PREFIX).map_err(guard)?,
    })
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the guard reads the header before the claim: a v1
// header is the fold's to rewrite, a v2 one is kept, and any other refuses with nothing run
pub(crate) fn manifest_guard(manifest: &str) -> Result<(), Refusal> {
    if !Path::new(manifest).is_file() {
        return Err(guard(format!(
            "manifest not found: {} (seed it with a '# contract: {}' header)",
            manifest,
            evidence::MANIFEST_CONTRACT
        )));
    }
    let text = std::fs::read(manifest)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| unreadable(format!("cannot read {}: {}", manifest, e)))?;
    match evidence::manifest_header(&text) {
        evidence::Header::V2 | evidence::Header::V1 => Ok(()),
        evidence::Header::Unknown(found) => Err(unreadable(format!(
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
        ))),
    }
}

// spec: evidence-kit/SPEC.md §Evidence manifest — `commit=` and `tree=` for the suite about to run,
// `-` for both on an unborn branch; a worktree git could not read refuses rather than guessing
fn head_state(cfg: &Cfg) -> Result<(String, String), String> {
    let Some(commit) = crate::provenance::head() else {
        return Ok(("-".to_string(), "-".to_string()));
    };
    let clean = if cfg.untracked == "count" {
        counted_clean(&cfg.tmpdir, &cfg.statement)
    } else {
        crate::provenance::worktree_clean()
    };
    let tree = match clean {
        Some(true) => "clean",
        Some(false) => "modified",
        None => return Err("cannot read the worktree state (git status) — nothing recorded".to_string()),
    };
    Ok((commit, tree.to_string()))
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the `count` read: every change and every untracked
// path the ignore rules leave, but the run's own writes, read against git's root-relative paths
fn counted_clean(tmpdir: &str, statement: &str) -> Option<bool> {
    let (here, top) = (walk::cwd().ok()?, walk::toplevel().ok()?);
    let fields = crate::listing::names(None, &["status", "--porcelain", "-z", "--untracked-files=all"]).ok()?;
    let tmp = walk::physical_against(&here, tmpdir);
    let stmt = (!statement.is_empty()).then(|| walk::physical_against(&here, statement));
    let mut fields = fields.into_iter();
    while let Some(entry) = fields.next() {
        let (code, path) = (entry.get(..2).unwrap_or(""), entry.get(3..).unwrap_or(""));
        if code.contains(['R', 'C']) {
            fields.next();
        }
        if code != "??" || !own_write(&entry_path(&top, path), &tmp, stmt.as_deref()) {
            return Some(false);
        }
    }
    Some(true)
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the exemption is a directory-entry test: the parent
// resolves physically and the leaf stays as git names it, so a link is judged where it sits
fn entry_path(top: &str, path: &str) -> String {
    let path = path.trim_end_matches('/');
    let (parent, leaf) = path.rsplit_once('/').unwrap_or(("", path));
    let parent = if parent.is_empty() { walk::physical_against(top, ".") } else { walk::physical_against(top, parent) };
    format!("{}/{}", parent.trim_end_matches(['/', '\\']), leaf)
}

// spec: evidence-kit/SPEC.md §Evidence manifest — a path under the tmp dir, or the Statement's own,
// is the run's write, so counting it would dirty the tree the next run reads
fn own_write(path: &str, tmpdir: &str, statement: Option<&str>) -> bool {
    walk::under(tmpdir, path) || statement.is_some_and(|s| walk::at_or_under(s, path))
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
    let (controls, malformed) = evidence::controls(&cfg.controls, &cfg.control_family, &cfg.suites);
    if !malformed.is_empty() {
        return Err(guard(format!("{} — nothing run, nothing written", malformed.join("; "))));
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
    manifest_guard(&cfg.manifest)?;

    std::fs::create_dir_all(&cfg.tmpdir)
        .map_err(|e| failed(format!("cannot create {}: {}", cfg.tmpdir, e)))?;
    let today = date_today().map_err(failed)?;

    let mut claimed = claim(&cfg.tmpdir, &cfg.lock, &key)?;

    // spec: evidence-kit/SPEC.md §bin/run-validate.sh — the Statement's subject and provenance are
    // read once, after the claim; an unborn branch has no subject, so it writes none
    let attest = if !renders(&cfg.statement, &cfg.write_cmd) {
        None
    } else {
        let provenance = crate::provenance::collect(&walk::kit_roots().unwrap_or_default(), None);
        match provenance.commit.clone() {
            Some(subject) => Some((subject, provenance.block())),
            None => {
                eprintln!("run-validate: HEAD names no commit (an unborn branch) — no Statement written or stored");
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
        .map_err(|e| failed(format!("cannot create the batch file {}: {}", batch, e)))?;
    claimed.batch = Some(batch.clone());

    let mut overall = 0;
    let mut rows: Vec<evidence::SuiteLine> = Vec::new();
    let mut control_lines: Vec<evidence::ControlLine> = Vec::new();
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

        // spec: evidence-kit/SPEC.md §bin/run-validate.sh — a failing pre-hook aborts the run at
        // exit 3, a run that could not start, with no evidence appended, and that ordering is the contract: it is what
        // keeps a refused run from writing a line.
        if !cfg.pre_hook.is_empty() {
            let status = spawn("EVIDENCE_KIT_PRE_HOOK", &cfg.pre_hook, Some(suite), &[], &proc::Sink::Inherit)
                .map_err(failed)?;
            if status != 0 {
                return Err(failed(format!(
                    "pre-hook failed for suite '{}' — aborting (no evidence appended)",
                    suite
                )));
            }
        }

        let (commit, tree) = head_state(&cfg).map_err(failed)?;
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
        let status = spawn("EVIDENCE_KIT_RUN_*", &cmd, None, &[], &proc::Sink::File(log.clone().into()))
            .map_err(failed)?;
        if status != 0 {
            eprintln!(
                "run-validate: suite '{}' exited {} (log: {})",
                suite, status, log
            );
        }

        let lines = evidence::parse(suite, Path::new(&log), status, &parser).map_err(unreadable)?;
        let parsed = format!("{}/validate-{}.parsed", cfg.tmpdir, suite);
        write_lines(&parsed, &lines).map_err(failed)?;
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
            log: crate::sha256::file_hex(Path::new(&log)).map_err(unreadable)?,
            counts: (npass as u64, nfail as u64, nignore as u64),
            verdict: verdict.to_string(),
            date: today.clone(),
            commit,
            tree,
            cmd: evidence::command_digest(&cmd, &parser),
            origin: "local".to_string(),
        };
        append_line(&batch, &row.render()).map_err(failed)?;
        println!(
            "run-validate: {} -> {} (pass={} fail={} ignore={})",
            suite, verdict, npass, nfail, nignore
        );

        // spec: evidence-kit/SPEC.md §Test controls — each control re-spawns its suite's argv under
        // its assignments, on the tree the suite just ran on, so the pre-hook does not re-run
        for c in controls.iter().filter(|c| &c.suite == suite) {
            let (clog, cparsed) = control_paths(&cfg.tmpdir, suite, &c.name);
            let status = spawn("EVIDENCE_KIT_RUN_*", &cmd, None, &c.env, &proc::Sink::File(clog.clone().into()))
                .map_err(failed)?;
            let lines = evidence::parse(suite, Path::new(&clog), status, &parser).map_err(unreadable)?;
            write_lines(&cparsed, &lines).map_err(failed)?;
            if lines.is_empty() {
                return Err(Refusal {
                    code: 1,
                    message: format!(
                        "parser '{}' produced no result for control '{}' of suite '{}' (log: {}) — a run failure, not an empty witness",
                        parser, c.name, suite, clog
                    ),
                });
            }
            let witnessed = evidence::witness(&lines, &c.scenario);
            let line = evidence::ControlLine {
                key: key.clone(),
                suite: suite.clone(),
                name: c.name.clone(),
                scenario: c.scenario.clone(),
                witnessed: witnessed.to_string(),
                date: today.clone(),
                commit: row.commit.clone(),
                cmd: row.cmd.clone(),
                assign: evidence::assignments_digest(c),
            };
            append_line(&batch, &line.render()).map_err(failed)?;
            if witnessed != "red" {
                overall = 1;
                println!(
                    "run-validate: control '{}' witnessed {} — {} {} did not fail under its assignments (log: {})",
                    c.name, witnessed, suite, c.scenario, clog
                );
            } else {
                println!("run-validate: control '{}' witnessed red", c.name);
            }
            control_lines.push(line);
        }
        rows.push(row);
    }

    // spec: evidence-kit/SPEC.md §bin/run-validate.sh — staged before the fold so a staging failure
    // writes nothing; the manifest publishes first and the Statement second, each by rename
    let staged = match &attest {
        None => None,
        Some((subject, block)) => {
            let target = Path::new(&cfg.statement);
            let parent = target.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
            if !cfg.statement.is_empty() && !parent.is_dir() {
                return Err(guard(format!(
                    "the Statement's directory {} does not exist — nothing recorded",
                    parent.display()
                )));
            }
            let staged = format!("{}/validate-statement.{}", cfg.tmpdir, std::process::id());
            let body = evidence::statement(&key, subject, block.clone(), &rows) + "\n";
            claimed.statement = Some(staged.clone());
            std::fs::write(&staged, &body)
                .map_err(|e| failed(format!("cannot stage the Statement {}: {} — nothing recorded", staged, e)))?;
            Some(Staged { path: staged, body: body.into_bytes(), subject: subject.clone() })
        }
    };
    fold(&cfg.manifest, &cfg.tmpdir, &key, &rows, &control_lines).map_err(failed)?;
    if let Some(staged) = staged {
        publish(&staged, &cfg.manifest, &cfg.statement, &cfg.write_cmd).map_err(failed)?;
    }
    Ok(overall)
}

struct Staged {
    path: String,
    body: Vec<u8>,
    subject: String,
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — a Statement renders where either destination is set
fn renders(statement: &str, write_cmd: &[String]) -> bool {
    !statement.is_empty() || !write_cmd.is_empty()
}

// spec: evidence-kit/SPEC.md §bin/run-validate.sh — after the manifest, the file by rename, then the
// store last, the one write that leaves the tree; a failure names each destination already published
fn publish(staged: &Staged, manifest: &str, statement: &str, write_cmd: &[String]) -> Result<(), String> {
    let mut published = vec![format!("the manifest {}", manifest)];
    if !statement.is_empty() {
        std::fs::rename(&staged.path, statement)
            .map_err(|e| format!("{} is published but the Statement {} is not: {}", published[0], statement, e))?;
        println!("run-validate: Statement written to {}", statement);
        published.push(format!("the Statement {}", statement));
    }
    let Some((head, rest)) = write_cmd.split_first() else {
        return Ok(());
    };
    let program = programs::Program::consumer("EVIDENCE_KIT_STATEMENT_WRITE_CMD", head.as_str());
    let mut args: Vec<&str> = rest.iter().map(String::as_str).collect();
    args.push(&staged.subject);
    let failed = |why: String| format!("{} — {} published, the store is not", why, published.join(" and "));
    let out = proc::run_streamed(&program, &args, &staged.body, proc::Stderr::Inherit).map_err(failed)?;
    if out.code() != 0 {
        return Err(failed(format!("EVIDENCE_KIT_STATEMENT_WRITE_CMD exited {}", out.code())));
    }
    println!("run-validate: Statement stored by EVIDENCE_KIT_STATEMENT_WRITE_CMD");
    Ok(())
}

// spec: evidence-kit/SPEC.md §Evidence manifest — the single fold, published by rename; a v1 file
// is rewritten at v2 with its v1 lines dropped, and a superseded suite's control lines go with it
pub(crate) fn fold(
    manifest: &str,
    tmpdir: &str,
    key: &str,
    rows: &[evidence::SuiteLine],
    controls: &[evidence::ControlLine],
) -> Result<(), String> {
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
        for c in controls.iter().filter(|c| c.suite == r.suite) {
            out.push_str(&c.render());
            out.push('\n');
        }
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
        .map_err(|e| failed(format!("cannot stage the lock record {}: {}", tmp, e)))?;
    let refuse = |r: Refusal| {
        let _ = std::fs::remove_file(&tmp);
        r
    };

    let mut reclaimed = false;
    while std::fs::hard_link(&tmp, lock).is_err() {
        match evidence::lock_read(Path::new(lock)) {
            LockRead::Corrupt => {
                return Err(refuse(unreadable(format!(
                    "the lock {} carries no readable 'pid=<n> run=<key>[ handoff=<path>]' record — refusing to \
                     start; delete it if no producer is running",
                    lock
                ))))
            }
            LockRead::Held {
                pid: holder,
                run_key,
                ..
            } => match evidence::pid_alive(&holder) {
                Ok(true) => {
                    return Err(refuse(guard(format!(
                        "a producer is already running for run key '{}' (pid {}) — refusing to \
                         start; wait for that run to finish, or delete {} once pid {} is gone",
                        run_key, holder, lock, holder
                    ))))
                }
                Ok(false) => {}
                // spec: evidence-kit/SPEC.md §The producer-liveness lock — the writer takes the
                // readers' disposition rather than the shell form's: reclaiming a holder the probe
                // could not classify is the false *free* answer that section forbids.
                Err(e) => return Err(refuse(failed(probe_message(e, lock)))),
            },
            LockRead::Absent => {}
        }
        // spec: evidence-kit/SPEC.md §bin/run-validate.sh — a dead or vanished holder is reclaimed
        // exactly once; a second failed claim means the slot is not ours to take, so refusing beats
        // looping — both contenders may relink, exactly one wins, and the loser re-reads a live PID.
        if reclaimed {
            return Err(refuse(guard(format!(
                "could not claim {} after reclaiming a stale lock — refusing to start rather than \
                 retrying; another producer won the reclaim race, or that path is not writable",
                lock
            ))));
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
    env: &[(String, String)],
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
    proc::run_to_env(&program, &words, env, sink)
}

// spec: evidence-kit/SPEC.md §Test controls — a control's log and parse beside its suite's; a family
// suffix carries no `.`, so neither can collide with a suite's own
fn control_paths(tmpdir: &str, suite: &str, name: &str) -> (String, String) {
    let stem = format!("{}/validate-{}.control-{}", tmpdir, suite, name);
    (format!("{}.log", stem), format!("{}.parsed", stem))
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
        let _knobs = crate::knobenv::lock();
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
        fold(&m, &dir.display().to_string(), "iter_a", &[row()], &[]).unwrap();
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

    // spec: evidence-kit/SPEC.md §Test controls — a control line folds directly after its suite's,
    // and the run's prior control lines for that suite go with the superseded suite line
    #[test]
    fn the_fold_places_controls_after_their_suite_and_supersedes_them_with_it() {
        let dir = std::env::temp_dir().join(format!("cw-fold-ctl-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let m = dir.join("manifest").display().to_string();
        let tail = "commit=- tree=- cmd=c origin=local";
        std::fs::write(
            &m,
            format!(
                "# contract: evidence-manifest v2\n\
                 iter_a gates sha256=old pass=1 fail=0 ignore=0 verdict=clean 2026-01-01 {}\n\
                 iter_a gates control=old scenario=s witnessed=red 2026-01-01 commit=- cmd=c\n",
                tail
            ),
        )
        .unwrap();
        let ctl = evidence::ControlLine {
            key: "iter_a".into(),
            suite: "gates".into(),
            name: "new".into(),
            scenario: "s".into(),
            witnessed: "green".into(),
            date: "2026-02-02".into(),
            commit: "-".into(),
            cmd: "c".into(),
            assign: "a".into(),
        };
        fold(&m, &dir.display().to_string(), "iter_a", &[row()], &[ctl]).unwrap();
        let got = std::fs::read_to_string(&m).unwrap();
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(
            got,
            format!(
                "# contract: evidence-manifest v2\n\
                 iter_a gates sha256=new pass=2 fail=0 ignore=0 verdict=clean 2026-02-02 {}\n\
                 iter_a gates control=new scenario=s witnessed=green 2026-02-02 commit=- cmd=c assign=a\n",
                tail
            )
        );
    }

    // spec: evidence-kit/SPEC.md §Evidence manifest — under `count` a path under the tmp dir and the
    // Statement's own path are the run's writes, and any other untracked path is not
    #[test]
    fn the_runs_own_writes_are_excluded_from_the_count() {
        let (t, s) = ("/r/.tmp", Some("/r/out/statement.json"));
        assert!(own_write("/r/.tmp/nested/log", t, None), "a path under the tmp dir");
        assert!(own_write("/r/out/statement.json", t, s), "the Statement's path");
        assert!(!own_write("/r/out/statement.json", t, None), "the Statement's path with no Statement configured");
        assert!(!own_write("/r/other.txt", t, s), "another untracked path");
        assert!(!own_write("/r/.tmp-sibling", t, s), "a sibling sharing the tmp dir's name as a prefix");
        assert!(own_write("C:/r/.tmp/log", "C:\\r\\.tmp", None), "either separator spells the tmp dir");
    }

    // spec: evidence-kit/SPEC.md §bin/run-validate.sh — either destination renders a Statement, and the
    // store runs last with the subject appended, a failure naming each destination already published
    #[test]
    fn the_store_runs_after_both_renames_and_names_them_when_it_fails() {
        let words = |w: &[&str]| w.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(!renders("", &[]));
        assert!(renders("", &words(&["git", "notes"])), "the write command alone renders a Statement");
        assert!(renders("out.json", &[]));

        let dir = std::env::temp_dir().join(format!("cw-store-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let at = |f: &str| dir.join(f).display().to_string();
        let stage = || {
            std::fs::write(at("staged"), "{}\n").unwrap();
            Staged { path: at("staged"), body: b"{}\n".to_vec(), subject: "a".repeat(40) }
        };
        let stored = publish(&stage(), "m.txt", "", &words(&["git", "check-ref-format", "--allow-onelevel"]));
        assert_eq!(stored, Ok(()), "the subject reaches the command as its last argument");

        let err = publish(&stage(), "m.txt", &at("statement.json"), &words(&["git", "no-such-subcommand"])).unwrap_err();
        assert!(dir.join("statement.json").is_file(), "the file is renamed before the store runs");
        assert!(err.contains("EVIDENCE_KIT_STATEMENT_WRITE_CMD exited"), "{}", err);
        assert!(err.contains("the manifest m.txt and the Statement"), "{}", err);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_control_logs_and_parses_beside_its_suite() {
        assert_eq!(
            control_paths("t", "unit", "nullguard"),
            ("t/validate-unit.control-nullguard.log".to_string(), "t/validate-unit.control-nullguard.parsed".to_string())
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
