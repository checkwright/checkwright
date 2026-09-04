// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the deterministic stamp half of a stage
// transition, mechanized (judgment stays in the skill) spec: gate-sdk/SPEC.md §The non-gate arm —
// an `Arm::Run` because the exit contract is three-state and every code is load-bearing: 0 a
use crate::proc;
use crate::registry;
use crate::stages;
use crate::walk;
use std::path::Path;

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the declared roster: the whole
// `LIFECYCLE_KIT_*` family this arm reads, plus the three names outside it that must cross
// because this arm *bridges* them onward.
pub const KNOBS: &[&str] = &[
    "LIFECYCLE_KIT_*",
    "GATE_SDK_TMP_DIR",
    "GATE_PRUNE_DIRS",
    "GATE_KIT_ROOTS_HERE",
];

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the unnamed-iteration placeholder, resolved
// once: the boundary reset's header rewrite and bootstrap stamp, the boundary-require skip, and
// --rename's refusal to write it all read this rather than repeating the glyph
const UNNAMED: &str = "—";

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the two pre-flight arms refuse on the same
// terms and recover the same way, so the recovery is one string rather than two that must be
// kept in step
const HELP_PREFLIGHT: &str =
    "resolve the finding above, or (to override deliberately) perform the stamp by hand.";

pub fn run(args: &[String]) -> i32 {
    match dispatch(args) {
        Ok(code) => code,
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the ledger's fail-closed refusal names
        // every malformed line itself, so an empty message is the already-reported one
        Err(e) if e.is_empty() => 2,
        Err(e) => {
            eprintln!("enter-stage: {}", e);
            2
        }
    }
}

struct Cfg {
    stages: Vec<String>,
    first_stage: String,
    queue: String,
    state: String,
    valve: String,
    gap_inbox: String,
    lesson_evidence: String,
    survey_record: String,
    boundary_truncate: Vec<String>,
    boundary_preserve: Vec<String>,
    boundary_require: Vec<String>,
    entry_preflight: Vec<String>,
    journal_pattern: String,
    journal_require: String,
    worktree_check: String,
    worktree_re: String,
    tmpdir: String,
}

fn cfg() -> Result<Cfg, String> {
    Ok(Cfg {
        stages: walk::knob_array("LIFECYCLE_KIT_STAGES")?,
        first_stage: walk::knob_scalar("LIFECYCLE_KIT_FIRST_STAGE")?,
        queue: walk::knob_scalar("LIFECYCLE_KIT_QUEUE_FILE")?,
        state: walk::knob_scalar("LIFECYCLE_KIT_STATE_FILE")?,
        valve: walk::knob_scalar("LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE")?,
        gap_inbox: walk::knob_scalar("LIFECYCLE_KIT_GAP_INBOX_FILE")?,
        lesson_evidence: walk::knob_scalar("LIFECYCLE_KIT_LESSON_EVIDENCE_FILE")?,
        survey_record: walk::knob_scalar("LIFECYCLE_KIT_SURVEY_RECORD_FILE")?,
        boundary_truncate: walk::knob_array("LIFECYCLE_KIT_BOUNDARY_TRUNCATE")?,
        boundary_preserve: walk::knob_array("LIFECYCLE_KIT_BOUNDARY_PRESERVE")?,
        boundary_require: walk::knob_array("LIFECYCLE_KIT_BOUNDARY_REQUIRE")?,
        entry_preflight: walk::knob_array("LIFECYCLE_KIT_ENTRY_PREFLIGHT")?,
        journal_pattern: walk::knob_scalar("LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN")?,
        journal_require: walk::knob_scalar("LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE")?,
        worktree_check: walk::knob_scalar("LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK")?,
        worktree_re: walk::knob_scalar("LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE")?,
        tmpdir: walk::knob_scalar("GATE_SDK_TMP_DIR")?,
    })
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --simulate: read-only preflight, every line
// prefixed 'enter-stage (simulate):' so a transcript can never read as a stamp
struct Say {
    sim: bool,
}

impl Say {
    fn out(&self, line: &str) {
        if self.sim {
            for l in line.split('\n') {
                println!("enter-stage (simulate): {}", l);
            }
        } else {
            println!("{}", line);
        }
    }
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the recovery relay: a refusal's help line
    // is its actionable half, so it prints under --simulate too. The mode's designed consumer is
    // the lead, which gates an expensive dispatch on it
    fn help(&self, line: &str) {
        if self.sim {
            eprintln!("enter-stage (simulate):   help: {}", line);
        } else {
            eprintln!("  help: {}", line);
        }
    }
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — a refused check's own output, relayed
    // verbatim by a real entry and prefixed under --simulate
    fn body(&self, text: &str) {
        let t = text.strip_suffix('\n').unwrap_or(text);
        if self.sim {
            for l in t.split('\n') {
                eprintln!("enter-stage (simulate): {}", l);
            }
        } else {
            eprintln!("{}", t);
        }
    }
}

fn usage(stages: &[String]) -> String {
    format!(
        "usage: run-gates.sh --enter-stage [--simulate] <stage>          (stage ∈ {})\n       \
         run-gates.sh --enter-stage [--simulate] --rename <name>  (rename the iteration: queue \
         header + column 1 of every stamp)\n       run-gates.sh --enter-stage [-h|--help]",
        stages.join(" ")
    )
}

fn read(path: &str) -> String {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default()
}

fn dispatch(args: &[String]) -> Result<i32, String> {
    let c = cfg()?;
    let first = args.first().map(String::as_str).unwrap_or("");

    // spec: gate-sdk/SPEC.md §The bin/-tool contract — the help half; this tool's positionals are
    // membership-validated, so it owes usage on stdout at exit 0 but no leading-'-' refusal
    if first == "-h" || first == "--help" {
        println!("{}", usage(&c.stages));
        return Ok(0);
    }

    let (sim, rest) = if first == "--simulate" {
        (true, &args[1..])
    } else {
        (false, args)
    };
    let say = Say { sim };

    if rest.first().map(String::as_str) == Some("--rename") {
        return rename(&c, &say, &rest[1..]);
    }
    stamp(&c, &say, rest)
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --rename: the two-surface iteration rename in
// one motion, no stamp appended and no stage token written, so the cursor is untouched and this
// is not stage motion
fn rename(c: &Cfg, say: &Say, rest: &[String]) -> Result<i32, String> {
    if rest.len() != 1 {
        eprintln!("enter-stage: --rename takes exactly one <name> — nothing written.");
        eprintln!("{}", usage(&c.stages));
        return Ok(2);
    }
    let name = &rest[0];
    if name.is_empty() {
        eprintln!("enter-stage: --rename <name> is empty — nothing written.");
        return Ok(2);
    }
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the placeholder refusal precedes the slug
    // grammar that would already reject it, so a session trying to un-name an iteration is told
    // which writer owns that value rather than that its name is malformed
    if name == UNNAMED {
        eprintln!(
            "enter-stage: --rename must not write the unnamed placeholder '{}' — only the \
             iteration-boundary reset writes it; nothing written.",
            UNNAMED
        );
        return Ok(2);
    }
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the slug grammar is a refusal because
    // column 1 is whitespace-delimited: a two-word name silently shifts every field of every stamp
    if !is_slug(name) {
        eprintln!(
            "enter-stage: --rename '{}' is not a queue slug ([a-z0-9][a-z0-9-]*) — nothing written.",
            name
        );
        return Ok(2);
    }
    if !Path::new(&c.queue).is_file() {
        eprintln!("enter-stage: queue file not found: {}", c.queue);
        return Ok(2);
    }
    if !Path::new(&c.state).is_file() {
        eprintln!("enter-stage: state file not found: {}", c.state);
        return Ok(2);
    }

    let queue_text = read(&c.queue);
    let state_text = read(&c.state);
    let hdr = match stages::header(&queue_text) {
        Some(h) => h.to_string(),
        None => {
            eprintln!(
                "enter-stage: no '## Iteration:' header in {} — nothing written.",
                c.queue
            );
            return Ok(2);
        }
    };
    let cur = stages::header_iter(&hdr);
    let moves = stages::data_lines(&state_text)
        .iter()
        .filter(|l| l.split_whitespace().next() != Some(name.as_str()))
        .count();

    if cur == *name && moves == 0 {
        if say.sim {
            println!(
                "enter-stage (simulate): the iteration is already named '{}' in {} and in every \
                 stamp of {} — the real rename would be an idempotent no-op.",
                name, c.queue, c.state
            );
            return Ok(0);
        }
        println!(
            "enter-stage: the iteration is already named '{}' in {} and in every stamp of {} — \
             idempotent no-op, nothing written.",
            name, c.queue, c.state
        );
        return Ok(0);
    }

    let new_queue = rewrite_header(&queue_text, name);
    let new_state = rewrite_column_one(&state_text, name);

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the columns-2-to-last witness: the writer
    // proves it touched only the field it meant to.
    if fields_two_to_last(&state_text) != fields_two_to_last(&new_state) {
        eprintln!(
            "enter-stage: the rename would alter columns 2 through NF (stage, session id, date, \
             head) of {} — refusing, nothing written.",
            c.state
        );
        return Ok(2);
    }

    let scratch = Scratch::new(&c.tmpdir, "rename.")?;
    let tq = scratch.path("queue");
    let ts = scratch.path("state");
    write_file(&tq, &new_queue)?;
    write_file(&ts, &new_state)?;

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the rename pre-flight names the gate and
    // never a substrate: the resolver yields the shell member's argv or the binary's, and an argv
    // the bridge refused to build is exit 2 — never a rename that proceeds unchecked
    match preflight_gate("check-stage-evidence", &tq, &ts)? {
        GateRun::Undispatchable => {
            eprintln!(
                "enter-stage: check-stage-evidence could not be dispatched (see above) — the \
                 rename could not be pre-flighted; nothing written."
            );
            return Ok(2);
        }
        GateRun::Refused(out) => {
            if say.sim {
                eprintln!(
                    "enter-stage (simulate): check-stage-evidence would refuse the rename to '{}':",
                    name
                );
            } else {
                eprintln!(
                    "enter-stage: check-stage-evidence refuses the rename to '{}' — nothing written:",
                    name
                );
            }
            say.body(&out);
            return Ok(1);
        }
        GateRun::Passed => {}
    }

    if say.sim {
        say.out(&format!(
            "--rename '{}' would rewrite both surfaces — no write:",
            name
        ));
        say.out(&format!(
            "{}: '## Iteration: {}' -> '## Iteration: {}'",
            c.queue, cur, name
        ));
        say.out(&format!(
            "{}: column 1 of {} stamp(s) -> '{}'; columns 2 through NF proved unchanged",
            c.state, moves, name
        ));
        return Ok(0);
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the rename is a whole-file rewrite by
    // construction (every data line's column 1 moves), which is why it is the writer's own mode
    // rather than an append like the stamp path
    write_file(&c.queue, &new_queue)?;
    write_file(&c.state, &new_state)?;
    println!(
        "enter-stage: renamed the iteration to '{}' — header in {}, column 1 of {} stamp(s) in \
         {}; columns 2 through NF proved unchanged.",
        name, c.queue, moves, c.state
    );
    println!(
        "  next: commit {} and {} together — the rename writes both, and check-stage-evidence \
         requires them to agree.",
        c.queue, c.state
    );
    Ok(0)
}

fn is_slug(s: &str) -> bool {
    let mut it = s.bytes();
    match it.next() {
        Some(b) if b.is_ascii_lowercase() || b.is_ascii_digit() => {}
        _ => return false,
    }
    it.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn rewrite_header(text: &str, name: &str) -> String {
    let mut done = false;
    let mut out = String::new();
    for l in lines_with_ends(text) {
        if !done && l.starts_with("## Iteration:") {
            out.push_str(&format!("## Iteration: {}\n", name));
            done = true;
            continue;
        }
        out.push_str(l);
    }
    out
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — every data line's column 1 is rewritten, not
// only the last: the first-stage entry truncates the state file, so every line below the
// separator belongs to the current iteration by construction — which is what check-stage-evidence
fn rewrite_column_one(text: &str, name: &str) -> String {
    let mut out = String::new();
    let mut below = false;
    for l in lines_with_ends(text) {
        let bare = l.strip_suffix('\n').unwrap_or(l);
        if !below {
            out.push_str(l);
            if is_separator(bare) {
                below = true;
            }
            continue;
        }
        if bare.split_whitespace().next().is_none() {
            out.push_str(l);
            continue;
        }
        let mut f: Vec<&str> = bare.split_whitespace().collect();
        f[0] = name;
        out.push_str(&f.join(" "));
        out.push('\n');
    }
    out
}

fn fields_two_to_last(text: &str) -> Vec<String> {
    stages::data_lines(text)
        .iter()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ")
        })
        .collect()
}

fn stamp(c: &Cfg, say: &Say, rest: &[String]) -> Result<i32, String> {
    let stage = match rest.first() {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            eprintln!("{}", usage(&c.stages));
            return Ok(2);
        }
    };
    if !stages::stage_known(&c.stages, &stage) {
        eprintln!(
            "enter-stage: '{}' is not a lifecycle stage ({})",
            stage,
            c.stages.join(" ")
        );
        eprintln!("{}", usage(&c.stages));
        return Ok(2);
    }
    if !Path::new(&c.queue).is_file() {
        eprintln!("enter-stage: queue file not found: {}", c.queue);
        return Ok(2);
    }
    if !Path::new(&c.state).is_file() {
        eprintln!("enter-stage: state file not found: {}", c.state);
        return Ok(2);
    }

    let queue_text = read(&c.queue);
    let state_text = read(&c.state);
    let hdr = stages::header(&queue_text).unwrap_or("").to_string();
    let cur_iter = if hdr.is_empty() {
        String::new()
    } else {
        stages::header_iter(&hdr)
    };
    let first = stage == c.first_stage;
    let stamp_iter = if first {
        UNNAMED.to_string()
    } else {
        cur_iter.clone()
    };

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the session id is derived **in process**:
    // after the port this arm sits behind the front-end that cds to the git toplevel, so the cwd
    // source 3 slugs is the project directory the harness keys transcripts on.
    let id = crate::emit::session_id::emit(&[])
        .map_err(|e| format!("could not read the session id ({}) — nothing written.", e))?
        .trim()
        .to_string();
    let today = date_today()?;
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the sole production writer of <head>, read
    // in the state file's own work tree at the instant of the append; 'none' where there is no work
    // tree or no commit to name is a value, never an omission
    let head_at = head_of(&c.state);
    let stamp_line = format!("{} {} {} {} {}", stamp_iter, stage, id, today, head_at);

    // spec: lifecycle-kit/SPEC.md §The state machine — the idempotence guard keys on the head
    // too, so a re-entry after HEAD moved appends rather than reporting a no-op: re-running this
    // tool IS the stated remedy for a stale recorded head, and a guard blind to the head would
    if let Some(last) = stages::data_lines(&state_text).last() {
        let f: Vec<&str> = last.split_whitespace().collect();
        if f.first() == Some(&stamp_iter.as_str())
            && f.get(1) == Some(&stage.as_str())
            && f.get(2) == Some(&id.as_str())
            && f.get(4) == Some(&head_at.as_str())
        {
            if say.sim {
                println!(
                    "enter-stage (simulate): '{}' is already the last stamp in {} — the real entry \
                     would be an idempotent no-op.",
                    stamp_line, c.state
                );
                return Ok(0);
            }
            println!(
                "enter-stage: '{}' already stamped in {} — idempotent no-op, nothing written.",
                stamp_line, c.state
            );
            return Ok(0);
        }
    }

    let scratch = Scratch::new(&c.tmpdir, "")?;
    let tmpstate = scratch.path("state");
    let tmpqueue = scratch.path("queue");
    let mut truncated: Vec<String> = Vec::new();
    let mut wiped: Vec<String> = Vec::new();

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the pre-flight hand-off: the cursor is
    // the last stamp, so the temp file carrying the candidate transition is the STATE file, not
    // the queue.
    let new_queue_boundary;
    let pre_queue: &str = if first {
        new_queue_boundary = rewrite_header(&queue_text, UNNAMED);
        write_file(&tmpqueue, &new_queue_boundary)?;
        let header_only = through_separator(&state_text);
        write_file(&tmpstate, &format!("{}\n\n{}\n", header_only, stamp_line))?;
        &tmpqueue
    } else {
        let mut s = state_text.clone();
        if !s.is_empty() && !s.ends_with('\n') {
            s.push('\n');
        }
        s.push_str(&stamp_line);
        s.push('\n');
        write_file(&tmpstate, &s)?;
        &c.queue
    };

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the built-in pre-flight names the gate and
    // never a substrate, the rename pre-flight's own resolution: an argv the bridge refused to
    // build is exit 2, never an entry that proceeds unchecked.
    match preflight_gate("check-stage-entry", pre_queue, &tmpstate)? {
        GateRun::Undispatchable => {
            eprintln!(
                "enter-stage: check-stage-entry could not be dispatched (see above) — the entry \
                 could not be pre-flighted; nothing written."
            );
            return Ok(2);
        }
        GateRun::Refused(out) => {
            if say.sim {
                eprintln!(
                    "enter-stage (simulate): check-stage-entry would refuse the entry to '{}':",
                    stage
                );
            } else {
                eprintln!(
                    "enter-stage: check-stage-entry refuses the entry to '{}' — nothing written:",
                    stage
                );
            }
            say.body(&out);
            say.help(HELP_PREFLIGHT);
            return Ok(1);
        }
        GateRun::Passed => {}
    }

    let mut valve = Valve::new();
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_ENTRY_PREFLIGHT: each entry
    // matching the entered stage runs after the built-in pre-flight with the same '<queue> <state>'
    // argv the built-in gets; a non-zero exit refuses the entry, nothing written
    for pf in &c.entry_preflight {
        let Some((key, cmd)) = pf.split_once('=') else {
            continue;
        };
        if key != stage {
            continue;
        }
        let argv: Vec<String> = cmd.split_whitespace().map(String::from).collect();
        if argv.is_empty() {
            continue;
        }
        let out = run_preflight_command(&argv, pre_queue, &tmpstate)?;
        if out.ok {
            continue;
        }
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the valve reaches this arm and no
        // other: LIFECYCLE_KIT_ENTRY_PREFLIGHT is the consumer-wired precondition, and a
        // consumer-wired precondition is the only one whose deadlock a consumer can reach at all
        valve.query(c, &cur_iter, &stage)?;
        if valve.armed {
            valve.report.push(format!(
                "the pre-flight valve admitted this entry past a refusing \
                 LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '{}' — the findings it would have \
                 refused on:",
                stage
            ));
            valve.report.push(indent_two(&out.text));
            continue;
        }
        if say.sim {
            eprintln!(
                "enter-stage (simulate): LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '{}' would \
                 refuse the entry:",
                stage
            );
        } else {
            eprintln!(
                "enter-stage: LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '{}' refuses the entry — \
                 nothing written:",
                stage
            );
        }
        say.body(&out.text);
        say.help(HELP_PREFLIGHT);
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the refusal names the configured
        // ledger and its state, so a typo'd path cannot masquerade as a never-armed valve
        if !c.valve.is_empty() {
            let why = if Path::new(&c.valve).is_file() {
                format!("carries no 'armed' line for '{} {}'", cur_iter, stage)
            } else {
                "does not exist (header-only is its resting state, so this is 'not armed' rather \
                 than an error — check the path if you meant to arm it)"
                    .to_string()
            };
            say.help(&format!(
                "or, for the one cause the pre-flight valve is sanctioned for — a stage whose \
                 entry pre-flight is refused by a precondition only a later stage can clear — \
                 append '{} {} armed <reason>' to the valve ledger {}, which {}, and re-run \
                 enter-stage {}. Reaching for it twice in one iteration is the failure rather than \
                 a supported mode, and the admitted entry prints the count that makes the second \
                 reach visible.",
                cur_iter, stage, c.valve, why, stage
            ));
        }
        return Ok(1);
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the predecessor-journal assertion, the
    // same refusal contract as the boundary-precondition family: exit 1, the expected path
    // printed, nothing written.
    if !first && c.journal_require == "1" {
        let pred_stage = stages::current_stage(&state_text);
        if !pred_stage.is_empty() {
            let pred_journal = journal_path(&c.journal_pattern, &pred_stage);
            // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — three-way, because the opener made
            // unwritten a state distinct from absent: absent means this tool never ran for that
            // stage, unwritten means it ran and the session did not
            let why = if !Path::new(&pred_journal).is_file() {
                Some("does not exist")
            } else if std::fs::metadata(&pred_journal).map(|m| m.len()).unwrap_or(0) == 0 {
                Some("is empty")
            } else if !journal_written(&pred_journal) {
                Some(
                    "carries only this tool's own opening line(s) — it was opened at that stage's \
                     entry and never written into",
                )
            } else {
                None
            };
            if let Some(why) = why {
                if say.sim {
                    eprintln!(
                        "enter-stage (simulate): entry to '{}' would be refused — the predecessor \
                         stage '{}' left no resume journal: {} {}.",
                        stage, pred_stage, pred_journal, why
                    );
                } else {
                    eprintln!(
                        "enter-stage: entry to '{}' refused — the predecessor stage '{}' left no \
                         resume journal: {} {} (nothing written).",
                        stage, pred_stage, pred_journal, why
                    );
                }
                say.help(&format!(
                    "append to {} yourself, stating plainly that '{}' left none, then re-run \
                     enter-stage {}. The assertion is evadable by design and this is the escape: \
                     what it buys is that the absence becomes deliberate and written instead of \
                     silent and unnoticed, at the one moment someone is looking. Where the file \
                     exists, its opening line already names the session that owed it and when, so \
                     the stand-in is an append under a header rather than a record invented from \
                     nothing.",
                    pred_journal, pred_stage, stage
                ));
                return Ok(1);
            }
        }
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the iteration-boundary entry refuses on a
    // non-empty ## Lessons Learned: an untriaged lesson must not cross into the next iteration
    if first {
        let lessons = section_bullets(&queue_text, "## Lessons Learned");
        if !lessons.is_empty() {
            let body = lessons.join("\n");
            if say.sim {
                eprintln!(
                    "enter-stage (simulate): iteration-boundary entry to '{}' would be refused — \
                     ## Lessons Learned is non-empty:",
                    stage
                );
            } else {
                eprintln!(
                    "enter-stage: iteration-boundary entry to '{}' refused — ## Lessons Learned is \
                     non-empty; the close stage must disposition every lesson before the next \
                     iteration begins (nothing written):",
                    stage
                );
            }
            say.body(&body);
            say.help(&format!(
                "run the close ritual's disposition step (rule/task/harvest/discard, stamping {}), \
                 clear the section, then re-run enter-stage {}.",
                c.lesson_evidence, stage
            ));
            return Ok(1);
        }
    }

    // spec: lifecycle-kit/SPEC.md §The committed gap inbox — the iteration-boundary gap-inbox
    // check: one detector (any bullet), two dispositions. Close-skipped refuses as it always did;
    // post-close admits, because no stage of the closing iteration is coming back.
    if first && Path::new(&c.gap_inbox).is_file() {
        let gaps: Vec<String> = read(&c.gap_inbox)
            .lines()
            .filter(|l| is_bullet(l))
            .map(String::from)
            .collect();
        if !gaps.is_empty() {
            let n = gaps.len();
            let body = gaps.join("\n");
            let cursor = stages::current_stage(&state_text);
            let closing = c.stages.last().cloned().unwrap_or_default();
            // spec: lifecycle-kit/SPEC.md §The committed gap inbox — the discriminator, one cursor
            // read shared with the --emit-file-gap arm. A never-named closing iteration has no
            // close to have skipped, and a boundary with no cursor at all is that case too.
            if cur_iter == UNNAMED || cursor == closing || cursor.is_empty() {
                if say.sim {
                    eprintln!(
                        "enter-stage (simulate): iteration-boundary entry to '{}' would not refuse \
                         for the gap inbox — it would carry {} bullet(s) from {} into '{}''s own \
                         intake:",
                        stage, n, c.gap_inbox, stage
                    );
                } else {
                    eprintln!(
                        "enter-stage: {} holds {} bullet(s) and no stage of the closing iteration \
                         is coming back for them — they do not refuse this entry; they are this \
                         iteration's '{}' intake:",
                        c.gap_inbox, n, stage
                    );
                }
                say.body(&body);
                say.help(&format!(
                    "disposition each bullet in this session, after the stamp: promote it to a \
                     queue entry, fix it inline, or discard it with cause in the commit message — \
                     then truncate {} to its header in the same commit. Deleting a bullet without \
                     a disposition is not a drain.",
                    c.gap_inbox
                ));
                say.help(
                    "a promoted entry's provenance sentence carries the bullet's own date and \
                     names the iteration whose close generated it — the finding's disposition \
                     lands in this iteration's ledger, and saying so is what keeps that legible.",
                );
            } else {
                if say.sim {
                    eprintln!(
                        "enter-stage (simulate): iteration-boundary entry to '{}' would be refused \
                         — {} holds {} untriaged gap bullet(s) and the cursor never reached '{}', \
                         the closing stage of '{}':",
                        stage, c.gap_inbox, n, closing, cur_iter
                    );
                } else {
                    eprintln!(
                        "enter-stage: iteration-boundary entry to '{}' refused — {} holds {} \
                         untriaged gap bullet(s) and the cursor never reached '{}', the closing \
                         stage of '{}' (nothing written):",
                        stage, c.gap_inbox, n, closing, cur_iter
                    );
                }
                say.body(&body);
                say.help(&format!(
                    "run the closing stage's gap-drain step — disposition each bullet (promote to \
                     a deferred [design-pending] entry, fix inline, or discard with cause in the \
                     commit message), truncate the inbox to its header, then re-run enter-stage {}.",
                    stage
                ));
                return Ok(1);
            }
        }
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the iteration-boundary linked-worktree
    // refusal: at an iteration boundary no linked worktree should be live.
    if first && c.worktree_check == "1" && inside_git() {
        let rows = worktree_scan(&c.worktree_re);
        if !rows.is_empty() {
            let mut lines: Vec<String> = Vec::new();
            let (mut live, mut orphaned, mut unclassified) = (0, 0, 0);
            for r in &rows {
                match r.class {
                    Class::Live => {
                        live += 1;
                        lines.push(format!("live         {} — held by pid {}", r.path, r.pid));
                    }
                    Class::Orphaned => {
                        orphaned += 1;
                        lines.push(format!(
                            "orphaned     {} — {}",
                            r.path,
                            worktree_loss(&r.path, &r.head)
                        ));
                    }
                    Class::Unclassified => {
                        unclassified += 1;
                        lines.push(format!(
                            "unclassified {} — {}",
                            r.path,
                            worktree_loss(&r.path, &r.head)
                        ));
                    }
                }
            }
            if say.sim {
                eprintln!(
                    "enter-stage (simulate): iteration-boundary entry to '{}' would be refused — \
                     {} linked worktree(s) still stand:",
                    stage,
                    rows.len()
                );
            } else {
                eprintln!(
                    "enter-stage: iteration-boundary entry to '{}' refused — {} linked worktree(s) \
                     still stand (nothing written):",
                    stage,
                    rows.len()
                );
            }
            say.body(&lines.join("\n"));
            if live > 0 {
                say.help(&format!(
                    "a live worktree's holder is still working: wait for the named pid to return, \
                     then re-run enter-stage {}. Do not remove it and do not force it — the reap \
                     advice below is for the other classes.",
                    stage
                ));
            }
            if orphaned > 0 {
                say.help(
                    "an orphaned worktree's holder is gone, so its lock states a fact that has \
                     become false: reap it with 'git worktree remove --force --force <path>' — git \
                     requires --force TWICE to remove a LOCKED worktree, once being enough only \
                     for an unlocked dirty one — and delete the branch ref it leaves behind, since \
                     'worktree remove' clears the directory only and a reap that stops there \
                     accretes refs this check cannot see.",
                );
            }
            if unclassified > 0 {
                say.help(&format!(
                    "reap each path with 'git worktree remove <path>' (or --force where the child \
                     left it locked) and delete the branch ref it leaves behind — 'worktree \
                     remove' clears the directory only, so a reap that stops there accretes refs \
                     this check cannot see. Then re-run enter-stage {}.",
                    stage
                ));
            }
            return Ok(1);
        }
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the mid-iteration worktree advisory: the
    // same scan away from the boundary, orphaned paths only and never a refusal.
    if !first && c.worktree_check == "1" && inside_git() {
        let adv: Vec<String> = worktree_scan(&c.worktree_re)
            .iter()
            .filter(|r| matches!(r.class, Class::Orphaned))
            .map(|r| format!("orphaned     {} — {}", r.path, worktree_loss(&r.path, &r.head)))
            .collect();
        if !adv.is_empty() {
            if say.sim {
                eprintln!(
                    "enter-stage (simulate): {} orphaned worktree(s) stand — advisory, this entry \
                     would not refuse:",
                    adv.len()
                );
            } else {
                eprintln!(
                    "enter-stage: {} orphaned worktree(s) stand — advisory, this entry is not \
                     refused:",
                    adv.len()
                );
            }
            say.body(&adv.join("\n"));
            say.help(
                "the holder of each is gone: reap with 'git worktree remove --force --force \
                 <path>' and delete the branch ref it leaves behind. Left standing they refuse the \
                 next iteration boundary.",
            );
        }
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_BOUNDARY_REQUIRE: at the
    // iteration boundary each member must carry a data line whose first token is the closing
    // iteration's name, else the entry refuses (fail-closed on a missing file)
    if first && cur_iter != UNNAMED {
        for br in &c.boundary_require {
            let msg = if !Path::new(br).is_file() {
                Some(format!(
                    "required boundary-disposition file not found: {}",
                    br
                ))
            } else if !names_iteration(&read(br), &cur_iter) {
                Some(format!(
                    "no disposition line naming the closing iteration '{}' in {}",
                    cur_iter, br
                ))
            } else {
                None
            };
            let Some(msg) = msg else { continue };
            if say.sim {
                eprintln!(
                    "enter-stage (simulate): iteration-boundary entry to '{}' would be refused — {}",
                    stage, msg
                );
            } else {
                eprintln!(
                    "enter-stage: iteration-boundary entry to '{}' refused — {} (nothing written).",
                    stage, msg
                );
            }
            say.help(&format!(
                "the close stage must disposition the iteration at the release boundary, stamping \
                 a '<iteration> release <version|none> — <basis>' line into {} before the next \
                 iteration begins.",
                br
            ));
            return Ok(1);
        }
    }

    if say.sim {
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --simulate reports the would-be
        // admission and leaves the ledger byte-identical: the real entry would proceed, so the
        // mode's verdict is exit 0, and the line it names is the one a real entry would consume
        if !valve.report.is_empty() {
            for r in &valve.report {
                say.out(r);
            }
            say.out(&format!(
                "the valve line at {}:{} would be consumed (state 'armed' -> 'used'); its reason: {}",
                c.valve, valve.line, valve.reason
            ));
            say.out(&format!(
                "this iteration already carries {} used valve line(s) — no write, the ledger is \
                 untouched.",
                valve.used
            ));
        }
        println!(
            "enter-stage (simulate): entry to '{}' would proceed — no stamp, nothing written.",
            stage
        );
        return Ok(0);
    }

    if first {
        move_file(&tmpstate, &c.state)?;
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the two kit-owned surfaces (lesson
        // evidence, survey record) reset as built-in members
        let mut members = vec![c.lesson_evidence.clone(), c.survey_record.clone()];
        members.extend(c.boundary_truncate.iter().cloned());
        for bt in members {
            if !Path::new(&bt).is_file() {
                continue;
            }
            let kept = truncate_to_header(&read(&bt));
            write_file(&bt, &kept)?;
            truncated.push(bt);
        }
        move_file(&tmpqueue, &c.queue)?;
    } else {
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the live stamp is an append, never a
        // rewrite of the pre-flight temp copy: a concurrent session's stamp landing between the
        // copy and the write would be lost by a whole-file move
        append_line(&c.state, &stamp_line)?;
    }
    scratch.clear();

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the consumption rides the write, not the
    // match: this tool writes no ledger line ever, it rewrites the state token of exactly one line
    // the arming session already wrote, so the ledger's line set stays the arming session's alone
    if !valve.report.is_empty() {
        let body = read(&c.valve);
        write_file(&c.valve, &consume_valve_line(&body, valve.line))?;
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the boundary scratch wipe, distinct from
    // the truncate above. Runs last so this run's own temporaries are already gone and never
    // candidates. '.gitkeep' is the kit invariant LIFECYCLE_KIT_BOUNDARY_PRESERVE cannot unset.
    if first && Path::new(&c.tmpdir).is_dir() {
        wiped = wipe(&c.tmpdir, &c.boundary_preserve);
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the journal open runs after the boundary
    // wipe and never under --simulate: a skeleton written ahead of the wipe is deleted by it
    // silently, and the first stage would then look like every firing of the absence this opener
    let mut journal = String::new();
    if c.journal_require == "1" {
        let p = journal_path(&c.journal_pattern, &stage);
        match journal_open(&p, &stage, &stamp_iter, &id, &today, &head_at) {
            Ok(()) => journal = p,
            Err(_) => {
                eprintln!(
                    "enter-stage: the stamp landed, but the resume journal at {} could not be \
                     opened — write it yourself.",
                    p
                );
            }
        }
    }

    if !valve.report.is_empty() {
        for r in &valve.report {
            println!("{}", r);
        }
        println!("  valve reason: {}", valve.reason);
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the prior-use count is the whole
        // mechanism: nothing prohibits the second reach, the count announces it to the session
        // taking it, in its own transcript, at the one moment someone is looking
        println!(
            "  note: this iteration carried {} used valve line(s) before this one — reaching for \
             the valve twice in one iteration is the failure rather than a supported mode.",
            valve.used
        );
        println!(
            "  next: commit {} with the stamp, and file the blocking task this reason names before \
             the closing stage ends.",
            c.valve
        );
    }
    if first {
        println!(
            "enter-stage: iteration-boundary reset — stamped '{}'; header set to '## Iteration: \
             {}'.",
            stamp_line, UNNAMED
        );
        println!(
            "  next: commit {} and {} together (the boundary reset writes both), hook enabled.",
            c.queue, c.state
        );
    } else {
        println!(
            "enter-stage: stamped '{}'; the cursor is now '{}' (no queue write — stage motion never \
             touches it).",
            stamp_line, stage
        );
        println!("  next: commit {}, hook enabled.", c.state);
    }
    if !truncated.is_empty() {
        println!(
            "  note: boundary-truncated to the '# contract:' header: {} — commit alongside the \
             reset.",
            truncated.join(" ")
        );
    }
    if !wiped.is_empty() {
        println!(
            "  note: boundary-wiped from {}: {}",
            c.tmpdir,
            wiped.join(" ")
        );
    }
    if !journal.is_empty() {
        println!(
            "  note: resume journal opened at {} — land your findings there as you confirm them; \
             your stage template's last step owns what it owes at the end.",
            journal
        );
    }

    // spec: lifecycle-kit/SPEC.md §The survey record — the read trigger: the entry report prints
    // the record's headings (the questions), never the findings, at the one moment a stage
    // session is guaranteed to be looking. Findings stay behind the witness.
    if Path::new(&c.survey_record).is_file() {
        let qs: Vec<&str> = read_headings(&c.survey_record);
        if !qs.is_empty() {
            println!(
                "  note: {} carries {} survey(s) this iteration — before buying one of these \
                 again, run its witness (diff the corpus since its rev, re-run its oracle) and \
                 cite it if both hold:",
                c.survey_record,
                qs.len()
            );
            for q in qs {
                println!("    {}", q);
            }
        }
    }
    Ok(0)
}

struct Valve {
    queried: bool,
    armed: bool,
    line: usize,
    used: usize,
    reason: String,
    report: Vec<String>,
}

impl Valve {
    fn new() -> Valve {
        Valve {
            queried: false,
            armed: false,
            line: 0,
            used: 0,
            reason: String::new(),
            report: Vec::new(),
        }
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the ledger is read only where the question
    // "is it armed?" is actually asked, which is a LIFECYCLE_KIT_ENTRY_PREFLIGHT refusal. Parsing
    // it on every entry would let a malformed ledger wedge entries that never needed a valve.
    fn query(&mut self, c: &Cfg, iter: &str, stage: &str) -> Result<(), String> {
        if self.queried {
            return Ok(());
        }
        self.queried = true;
        if c.valve.is_empty() || !Path::new(&c.valve).is_file() {
            return Ok(());
        }
        let body = read(&c.valve);
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the valve ledger's two fail-closed
        // shapes, refused together so one pass names every malformed line: a data line under four
        // fields, and a state token that is neither 'armed' nor 'used'.
        let mut bad: Vec<String> = Vec::new();
        for (i, l) in body.lines().enumerate() {
            if l.starts_with('#') || l.trim().is_empty() {
                continue;
            }
            let f: Vec<&str> = l.split_whitespace().collect();
            if f.len() < 4 {
                bad.push(format!(
                    "  line {} carries {} field(s), fewer than the four <iteration> <stage> \
                     armed|used <reason...> requires: {}",
                    i + 1,
                    f.len(),
                    l
                ));
                continue;
            }
            if f[2] != "armed" && f[2] != "used" {
                bad.push(format!(
                    "  line {} carries state token {}, which is neither armed nor used: {}",
                    i + 1,
                    f[2],
                    l
                ));
            }
        }
        if !bad.is_empty() {
            eprintln!(
                "enter-stage: the pre-flight valve ledger {} cannot be parsed, so whether this \
                 entry is armed is unanswerable — nothing written:",
                c.valve
            );
            for b in &bad {
                eprintln!("{}", b);
            }
            return Err(String::new());
        }
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the match: the FIRST armed line whose
        // iteration and stage both equal the entering entry's; the prior-used count is every used
        // line of this iteration, whatever stage it names.
        for (i, l) in body.lines().enumerate() {
            if l.starts_with('#') || l.trim().is_empty() {
                continue;
            }
            let f: Vec<&str> = l.split_whitespace().collect();
            if f.len() < 4 {
                continue;
            }
            if f[0] == iter && f[2] == "used" {
                self.used += 1;
            }
            if !self.armed && f[0] == iter && f[1] == stage && f[2] == "armed" {
                self.armed = true;
                self.line = i + 1;
                self.reason = f[3..].join(" ");
            }
        }
        Ok(())
    }
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the consumption rewrites field 3 of exactly one
// line rather than substituting the token's text, because an iteration or stage name may itself
// contain the token and a text substitution would rewrite the wrong field on that line
fn consume_valve_line(body: &str, n: usize) -> String {
    let mut out = String::new();
    for (i, l) in lines_with_ends(body).into_iter().enumerate() {
        if i + 1 == n {
            let bare = l.strip_suffix('\n').unwrap_or(l);
            let mut f: Vec<&str> = bare.split_whitespace().collect();
            if f.len() >= 3 {
                f[2] = "used";
            }
            out.push_str(&f.join(" "));
            out.push('\n');
        } else {
            out.push_str(l);
        }
    }
    out
}

fn indent_two(text: &str) -> String {
    let t = text.strip_suffix('\n').unwrap_or(text);
    t.split('\n')
        .map(|l| format!("  {}", l))
        .collect::<Vec<String>>()
        .join("\n")
}

enum GateRun {
    Passed,
    Refused(String),
    Undispatchable,
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — both arms name a gate and never a substrate.
// spec: gate-sdk/SPEC.md §run-gates — the member runs as a **child process** and never as an in-
// process call: `native/src/runner.rs` records the ground as the declared-knob discipline, fault
fn preflight_gate(name: &str, queue: &str, state: &str) -> Result<GateRun, String> {
    let dirs: Vec<String> = walk::kit_roots_abs()?
        .into_iter()
        .map(|r| format!("{}/checks", r))
        .collect();
    let Some(src) = registry::resolve(name, &dirs) else {
        eprintln!(
            "gate_command: {} resolves in none of: {} — the gate could not run; treating as \
             failure (not clean)",
            name,
            dirs.join(" ")
        );
        return Ok(GateRun::Undispatchable);
    };
    let mut argv: Vec<String>;
    let mut env: Vec<(String, String)> = Vec::new();
    if src.ends_with(".gate") {
        let Some(declared) = crate::gates::knobs(name) else {
            eprintln!(
                "checkwright-gates: no such gate subcommand: {} — the check could not run; \
                 treating as failure (not clean)",
                name
            );
            return Ok(GateRun::Undispatchable);
        };
        let exe = std::env::current_exe()
            .map_err(|e| format!("cannot resolve this binary's own path: {}", e))?;
        argv = vec![exe.display().to_string(), name.to_string()];
        env = child_knobs(declared);
    } else {
        argv = vec![src];
    }
    argv.push(queue.to_string());
    argv.push(state.to_string());

    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let merged = run_merged_with_env(&argv[0], &args, &env)?;
    if merged.0 {
        Ok(GateRun::Passed)
    } else {
        Ok(GateRun::Refused(merged.1))
    }
}

// spec: gate-sdk/SPEC.md §lib/gate.sh — the child's declared knob environment, built by filtering
// the bridged set this arm itself received: a member receives the GATE_SDK_KNOB_* variables its
// own registry entry declares and no others, which is what keeps the declared-knob discipline
fn child_knobs(declared: &[&str]) -> Vec<(String, String)> {
    super::child_knobs(declared)
}

struct PreflightOut {
    ok: bool,
    text: String,
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — each LIFECYCLE_KIT_ENTRY_PREFLIGHT entry's
// <command> is split on whitespace and spawned as argv with **no interpreter word prepended**, so
// the configured path rides its own exec bit.
fn run_preflight_command(
    argv: &[String],
    queue: &str,
    state: &str,
) -> Result<PreflightOut, String> {
    let mut args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    args.push(queue);
    args.push(state);
    let (ok, text) = run_merged_with_env(&argv[0], &args, &[])?;
    Ok(PreflightOut { ok, text })
}

fn run_merged_with_env(
    program: &str,
    args: &[&str],
    env: &[(String, String)],
) -> Result<(bool, String), String> {
    let mut cmd = std::process::Command::new(program);
    cmd.args(args);
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().map_err(|e| {
        format!(
            "cannot run {}: {} — the entry could not be pre-flighted; nothing written.",
            program, e
        )
    })?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    Ok((out.status.success(), text))
}

enum Class {
    Live,
    Orphaned,
    Unclassified,
}

struct Row {
    class: Class,
    pid: String,
    path: String,
    head: String,
}

fn inside_git() -> bool {
    proc::run("git", &["rev-parse", "--git-dir"])
        .map(|c| c.stdout().is_some())
        .unwrap_or(false)
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the linked-worktree scan: one porcelain parse
// classified live/orphaned/unclassified, shared by the boundary refusal and the mid-iteration
// advisory so the two cannot disagree about what a path is. The main checkout is skipped.
fn worktree_scan(re: &str) -> Vec<Row> {
    let Ok(c) = proc::run("git", &["worktree", "list", "--porcelain"]) else {
        return Vec::new();
    };
    let Some(raw) = c.stdout() else {
        return Vec::new();
    };
    let text = String::from_utf8_lossy(raw).into_owned();

    let mut entries: Vec<(String, String, bool, String)> = Vec::new();
    let (mut path, mut head, mut locked, mut reason) =
        (String::new(), String::new(), false, String::new());
    let flush = |path: &mut String, head: &mut String, locked: &mut bool, reason: &mut String, out: &mut Vec<(String, String, bool, String)>| {
        if !path.is_empty() {
            out.push((path.clone(), head.clone(), *locked, reason.clone()));
        }
        path.clear();
        head.clear();
        *locked = false;
        reason.clear();
    };
    for l in text.lines() {
        if let Some(p) = l.strip_prefix("worktree ") {
            flush(&mut path, &mut head, &mut locked, &mut reason, &mut entries);
            path = p.to_string();
        } else if let Some(h) = l.strip_prefix("HEAD ") {
            head = h.to_string();
        } else if l == "locked" || l.starts_with("locked ") || l.starts_with("locked\t") {
            locked = true;
            reason = if l.len() > 7 { l[7..].to_string() } else { String::new() };
        }
    }
    flush(&mut path, &mut head, &mut locked, &mut reason, &mut entries);

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the main checkout is skipped; it is the
    // porcelain's first entry
    entries
        .into_iter()
        .skip(1)
        .map(|(path, head, locked, reason)| {
            let mut pid = String::new();
            let class = if re.is_empty() {
                Class::Unclassified
            } else if !locked {
                Class::Orphaned
            } else if let Some(captured) = capture_group_one(re, &reason) {
                pid = captured;
                match crate::evidence::pid_alive(&pid) {
                    Ok(true) => Class::Live,
                    _ => Class::Orphaned,
                }
            } else {
                Class::Unclassified
            };
            Row {
                class,
                pid: if pid.is_empty() { "-".to_string() } else { pid },
                path,
                head,
            }
        })
        .collect()
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the lock-reason pattern is **consumer
// config**, so it is interpreted rather than transported: the crate's own ERE engine reports a
// whole-match span and carries no capture group at all (gate-sdk/SPEC.md §The POSIX ERE matcher
fn capture_group_one(re: &str, subject: &str) -> Option<String> {
    let script = "[[ \"$2\" =~ $1 ]] || exit 1; printf '%s' \"${BASH_REMATCH[1]}\"";
    let c = proc::run("bash", &["-c", script, "bash", re, subject]).ok()?;
    c.stdout()
        .map(|o| String::from_utf8_lossy(o).into_owned())
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the loss report: the two facts that decide
// whether removing a residue worktree loses anything, read mechanically at the moment of refusal
// rather than left for the session to re-derive per path with a hand-run 'git status'.
fn worktree_loss(p: &str, h: &str) -> String {
    if !Path::new(p).is_dir() {
        return "directory already gone — prunable residue".to_string();
    }
    let dirty = match proc::run("git", &["-C", p, "status", "--porcelain"]) {
        Ok(c) => match c.stdout() {
            Some(o) => !String::from_utf8_lossy(o).trim().is_empty(),
            None => false,
        },
        Err(_) => false,
    };
    let commits = match proc::run("git", &["rev-list", "--count", h, "^HEAD"]) {
        Ok(c) => match c.stdout() {
            Some(o) => {
                let s = String::from_utf8_lossy(o).trim().to_string();
                if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
                    s
                } else {
                    "?".to_string()
                }
            }
            None => "?".to_string(),
        },
        Err(_) => "?".to_string(),
    };
    if !dirty && commits == "0" {
        return "clean, no commit unreachable from HEAD — removal is lossless".to_string();
    }
    format!(
        "{}, {} commit(s) unreachable from HEAD",
        if dirty { "dirty" } else { "clean" },
        commits
    )
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the header run stops at a markdown '## '
// section heading as well as at the first data line: on a markdown surface whose blocks are '## '
// headings (the survey record) a bare /^#/ predicate reads the first block's heading as part of
fn truncate_to_header(text: &str) -> String {
    let mut out = String::new();
    let mut pend = String::new();
    for l in text.lines() {
        if l.trim().is_empty() {
            pend.push_str(l);
            pend.push('\n');
            continue;
        }
        let mut b = l.bytes();
        let heading = b.next() == Some(b'#') && b.next() != Some(b'#');
        if heading {
            out.push_str(&pend);
            pend.clear();
            out.push_str(l);
            out.push('\n');
            continue;
        }
        break;
    }
    out
}

fn through_separator(text: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for l in text.lines() {
        out.push(l);
        if is_separator(l) {
            break;
        }
    }
    out.join("\n")
}

fn is_separator(l: &str) -> bool {
    l.starts_with("---") && l[3..].bytes().all(|b| b == b' ' || b == b'\t')
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the wipe deletes every member whose basename
// is neither '.gitkeep' nor a LIFECYCLE_KIT_BOUNDARY_PRESERVE entry, **at any depth the walk
// reaches**.
fn wipe(dir: &str, preserve: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    wipe_walk(Path::new(dir), preserve, &mut out);
    out
}

fn wipe_walk(dir: &Path, preserve: &[String], out: &mut Vec<String>) {
    // spec: gate-sdk/SPEC.md §The crate's crosser — the directory listing goes through `walk`,
    // the crate's one filesystem-walking module: a direct traversal here would be invisible to
    // the recorder that holds that invariant, and `list_dir` sorts, which is what makes the wiped
    let Ok(kids) = walk::list_dir(dir) else {
        return;
    };
    for (base, is_dir) in kids {
        let p = dir.join(&base);
        // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the keep-list filters the delete and
        // never the descent, so a preserved directory's members are still candidates
        if is_dir && !p.is_symlink() {
            wipe_walk(&p, preserve, out);
        }
        if base == ".gitkeep" || preserve.contains(&base) {
            continue;
        }
        out.push(p.display().to_string());
        if is_dir && !p.is_symlink() {
            let _ = std::fs::remove_dir(&p);
        } else {
            let _ = std::fs::remove_file(&p);
        }
    }
}

// spec: lifecycle-kit/SPEC.md §The state machine — the one expansion every reader shares: a
// dispatcher granting the path, a stage session writing it, and the entry asserting its
// predecessor's must name one file or the assertion reads a file nobody wrote
pub fn journal_path(pattern: &str, stage: &str) -> String {
    pattern.replace("<stage>", stage)
}

// spec: lifecycle-kit/SPEC.md §The state machine — the opening line's fixed lead, spelled once
// because two readers must agree on it by construction: the opener writes it and the entry
// assertion tells the tool's own bytes from a session's by it
pub const JOURNAL_MARK: &str = "# stage-journal ";

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the predicate the entry assertion reads.
// Non-emptiness went vacuous the moment the opener started writing the file, so "the owing session
// wrote something" is a line that is neither blank nor an opener heading.
pub fn journal_written(p: &str) -> bool {
    read(p)
        .lines()
        .any(|l| !l.trim().is_empty() && !l.starts_with(JOURNAL_MARK))
}

// spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the opener: append, never overwrite, so a stage
// running several sessions accumulates a heading per session instead of losing its predecessors'
// work. The heading carries the stamp's own five fields, so an unwritten journal names who owed it.
pub fn journal_open(
    p: &str,
    stage: &str,
    iter: &str,
    id: &str,
    date: &str,
    head: &str,
) -> Result<(), String> {
    if let Some(parent) = Path::new(p).parent() {
        if !parent.as_os_str().is_empty() && !parent.is_dir() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    let nonempty = std::fs::metadata(p).map(|m| m.len() > 0).unwrap_or(false);
    let mut body = String::new();
    if nonempty {
        body.push('\n');
    }
    body.push_str(&format!(
        "{}{} — {} {} {} {}\n",
        JOURNAL_MARK, stage, iter, id, date, head
    ));
    append_raw(p, &body)
}

fn is_bullet(l: &str) -> bool {
    let b = l.as_bytes();
    b.first() == Some(&b'-') && matches!(b.get(1), Some(c) if c.is_ascii_whitespace())
}

fn section_bullets(text: &str, heading: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut inside = false;
    for l in text.lines() {
        if l.trim_end() == heading {
            inside = true;
            continue;
        }
        if l.starts_with("## ") {
            inside = false;
        }
        if inside && is_bullet(l) {
            out.push(l.to_string());
        }
    }
    out
}

fn names_iteration(text: &str, iter: &str) -> bool {
    text.lines().any(|l| {
        !l.starts_with('#')
            && !l.trim().is_empty()
            && l.split_whitespace().next() == Some(iter)
    })
}

fn read_headings(path: &str) -> Vec<&'static str> {
    // comment-tier-exempt: the leak is a lifetime device local to this one reader — the record is
    // small and read exactly here — and states nothing the SPEC owns
    let body: &'static str = Box::leak(read(path).into_boxed_str());
    body.lines()
        .filter(|l| {
            let mut b = l.bytes();
            b.next() == Some(b'#') && b.next() == Some(b'#') && matches!(b.next(), Some(c) if c.is_ascii_whitespace())
        })
        .collect()
}

fn lines_with_ends(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0usize;
    let b = text.as_bytes();
    for i in 0..b.len() {
        if b[i] == b'\n' {
            out.push(&text[start..=i]);
            start = i + 1;
        }
    }
    if start < text.len() {
        out.push(&text[start..]);
    }
    out
}

fn date_today() -> Result<String, String> {
    let c = proc::run("date", &["+%F"])?;
    match c.stdout() {
        Some(o) => Ok(String::from_utf8_lossy(o).trim().to_string()),
        None => Err("could not read today's date — nothing written.".to_string()),
    }
}

fn head_of(state: &str) -> String {
    let dir = Path::new(state)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| ".".to_string());
    match proc::run("git", &["-C", &dir, "rev-parse", "--short", "HEAD"]) {
        Ok(c) => match c.stdout() {
            Some(o) => {
                let s = String::from_utf8_lossy(o).trim().to_string();
                if s.is_empty() {
                    "none".to_string()
                } else {
                    s
                }
            }
            None => "none".to_string(),
        },
        Err(_) => "none".to_string(),
    }
}

struct Scratch {
    dir: String,
    tag: String,
    pid: u32,
}

impl Scratch {
    fn new(dir: &str, tag: &str) -> Result<Scratch, String> {
        std::fs::create_dir_all(dir)
            .map_err(|e| format!("cannot create the scratch dir {}: {}", dir, e))?;
        Ok(Scratch {
            dir: dir.to_string(),
            tag: tag.to_string(),
            pid: std::process::id(),
        })
    }
    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — `check-stage-entry` echoes the candidate
    // state file's path in its refusal, so the temp name is observable output rather than a detail
    fn path(&self, what: &str) -> String {
        format!("{}/enter-stage.{}{}.{}", self.dir, self.tag, what, self.pid)
    }
    fn clear(&self) {
        for what in ["queue", "state"] {
            let _ = std::fs::remove_file(self.path(what));
        }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        self.clear();
    }
}

// comment-tier-exempt: a cross-filesystem fallback local to this helper — GATE_SDK_TMP_DIR may
// name another mount, where rename fails and the reset must still land
fn move_file(src: &str, dst: &str) -> Result<(), String> {
    if std::fs::rename(src, dst).is_ok() {
        return Ok(());
    }
    write_file(dst, &read(src))?;
    let _ = std::fs::remove_file(src);
    Ok(())
}

fn write_file(path: &str, body: &str) -> Result<(), String> {
    std::fs::write(path, body).map_err(|e| format!("cannot write {}: {}", path, e))
}

fn append_raw(path: &str, body: &str) -> Result<(), String> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("cannot open {}: {}", path, e))?;
    f.write_all(body.as_bytes())
        .map_err(|e| format!("cannot append to {}: {}", path, e))
}

fn append_line(path: &str, line: &str) -> Result<(), String> {
    append_raw(path, &format!("{}\n", line))
}

#[cfg(test)]
mod tests {
    use super::*;

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the columns-2-to-last witness reads to
    // the last field rather than to a pinned column, so a sixth field a later grammar appends is
    // inside the witness rather than outside it.
    #[test]
    fn the_rename_witness_reaches_the_last_field_whatever_the_arity() {
        let five = "# h\n\n---\n\nold a bb ccc dddd\n";
        let six = "# h\n\n---\n\nold a bb ccc dddd eeeee\n";
        assert_eq!(fields_two_to_last(five), vec!["a bb ccc dddd"]);
        assert_eq!(fields_two_to_last(six), vec!["a bb ccc dddd eeeee"]);
        assert_eq!(
            fields_two_to_last(&rewrite_column_one(six, "new")),
            fields_two_to_last(six)
        );
        assert!(rewrite_column_one(six, "new").contains("new a bb ccc dddd eeeee"));
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the header run stops at a '## ' heading as
    // well as at the first data line, and the retained blank run does not grow by one per boundary
    #[test]
    fn the_truncate_stops_at_a_section_heading_and_holds_blanks_pending() {
        let rec = "# contract: x\n\n## 2026-01-01 scope — q?\n- finding: body\n";
        assert_eq!(truncate_to_header(rec), "# contract: x\n");
        assert_eq!(truncate_to_header(&truncate_to_header(rec)), "# contract: x\n");
        let two = "# a\n# b\n\ndata\n";
        assert_eq!(truncate_to_header(two), "# a\n# b\n");
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the consumption rewrites field 3 of exactly
    // one line rather than substituting the token's text, so an iteration whose *name* carries the
    // token is not rewritten on the wrong field
    #[test]
    fn the_valve_consumption_rewrites_one_field_of_one_line() {
        let led = "# c\narmed-iteration build armed a reason armed twice\narmed-iteration close armed another\n";
        let got = consume_valve_line(led, 2);
        assert_eq!(
            got,
            "# c\narmed-iteration build used a reason armed twice\narmed-iteration close armed another\n"
        );
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the journal-written predicate tells the
    // opener's own bytes from a session's, which a bare non-emptiness test cannot once the tool
    // writes the file itself
    #[test]
    fn an_opened_but_unwritten_journal_is_not_written() {
        let dir = std::env::temp_dir().join(format!("enter-stage-journal-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join("build-journal.md").display().to_string();
        let _ = std::fs::remove_file(&p);
        journal_open(&p, "build", "iter", "deadbeef", "2026-01-01", "none").expect("open");
        assert!(!journal_written(&p), "a bare skeleton read as written");
        append_raw(&p, "a session wrote this\n").expect("append");
        assert!(journal_written(&p));
        journal_open(&p, "build", "iter", "cafe0001", "2026-01-02", "none").expect("reopen");
        let body = read(&p);
        assert!(body.contains("a session wrote this"), "the opener overwrote");
        assert_eq!(body.matches(JOURNAL_MARK).count(), 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the wipe keeps '.gitkeep' and every
    // PRESERVE basename at any depth, and that at-any-depth reach is preserved rather than
    // corrected: the filed defect is not this cut's to fix
    #[test]
    fn the_wipe_keeps_the_invariant_and_every_preserved_basename_at_any_depth() {
        let dir = std::env::temp_dir().join(format!("enter-stage-wipe-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("doomed-sub")).expect("mk");
        std::fs::create_dir_all(dir.join("mixed-sub")).expect("mk");
        std::fs::write(dir.join(".gitkeep"), "").expect("w");
        std::fs::write(dir.join("keep-me"), "live").expect("w");
        std::fs::write(dir.join("doomed.log"), "stale").expect("w");
        std::fs::write(dir.join("doomed-sub/nested.txt"), "stale").expect("w");
        std::fs::write(dir.join("mixed-sub/keep-me"), "live").expect("w");
        let wiped = wipe(&dir.display().to_string(), &["keep-me".to_string()]);
        assert!(dir.join(".gitkeep").exists(), "the kit invariant was deleted");
        assert!(dir.join("keep-me").exists(), "a PRESERVE member was deleted");
        assert!(dir.join("mixed-sub/keep-me").exists(), "the at-any-depth reach was lost");
        assert!(!dir.join("doomed.log").exists(), "an unlisted file survived");
        assert!(!dir.join("doomed-sub").exists(), "an all-unlisted subdir survived");
        assert!(dir.is_dir(), "the scratch dir itself was removed");
        assert!(wiped.iter().any(|w| w.ends_with("doomed.log")));
        assert!(!wiped.iter().any(|w| w.ends_with("/keep-me")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the lock-reason pattern is consumer config
    // interpreted by bash, so the shipped pattern's capture group yields the holder's pid; the
    // crate's own engine reports a whole-match span and would yield nothing
    #[test]
    fn the_consumer_lock_pattern_captures_the_holders_pid_through_bash() {
        let re = r"^claude agent [^ ]+ \(pid ([0-9]+) start [0-9]+\)$";
        assert_eq!(
            capture_group_one(re, "claude agent a1b2 (pid 4321 start 99)").as_deref(),
            Some("4321")
        );
        assert_eq!(capture_group_one(re, "some other tool is holding this"), None);
        let compiled = crate::ere::Ere::compile(re).expect("the shipped pattern must parse");
        assert!(compiled.is_match("claude agent a1b2 (pid 4321 start 99)"));
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the slug grammar is a refusal because
    // column 1 is whitespace-delimited: a two-word name silently shifts every field of every stamp
    #[test]
    fn the_rename_slug_grammar_refuses_what_would_shift_a_stamps_fields() {
        assert!(is_slug("a-name-9"));
        assert!(is_slug("0"));
        assert!(!is_slug(""));
        assert!(!is_slug("two words"));
        assert!(!is_slug("Upper-Case"));
        assert!(!is_slug("-leading-dash"));
        assert!(!is_slug(UNNAMED));
    }

    // spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — `--simulate`'s contract is that it runs
    // everything up to the write and writes nothing.
    #[test]
    fn no_pre_dispatch_refusal_path_writes_a_byte() {
        let dir = std::env::temp_dir().join(format!("enter-stage-nowrite-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mk");
        let q = dir.join("TASK-QUEUE.md").display().to_string();
        let s = dir.join("WORKFLOW-STATE.txt").display().to_string();
        let v = dir.join("valve.txt").display().to_string();
        std::fs::write(&q, "# q\n\n## Iteration: demo\n\n## Done\n").expect("w");
        std::fs::write(&s, "# c\n\n---\n\ndemo scope aaaaaaaa 2026-06-01 none\n").expect("w");
        std::fs::write(&v, "# c\ndemo build armed a reason\n").expect("w");
        let before: Vec<(String, Vec<u8>)> = [&q, &s, &v]
            .iter()
            .map(|p| ((*p).clone(), std::fs::read(p).expect("read")))
            .collect();

        assert_eq!(rewrite_header(&read(&q), "renamed").matches("## Iteration: renamed").count(), 1);
        let mangled = read(&s).replace("scope", "spec");
        assert_ne!(fields_two_to_last(&read(&s)), fields_two_to_last(&mangled));
        std::fs::write(&v, "# c\ndemo build\n").expect("w");
        let bad_before = std::fs::read(&v).expect("read");
        let mut valve = Valve::new();
        let c = Cfg {
            stages: vec!["scope".into(), "build".into()],
            first_stage: "scope".into(),
            queue: q.clone(),
            state: s.clone(),
            valve: v.clone(),
            gap_inbox: String::new(),
            lesson_evidence: String::new(),
            survey_record: String::new(),
            boundary_truncate: vec![],
            boundary_preserve: vec![],
            boundary_require: vec![],
            entry_preflight: vec![],
            journal_pattern: "<stage>.md".into(),
            journal_require: "0".into(),
            worktree_check: "0".into(),
            worktree_re: String::new(),
            tmpdir: dir.display().to_string(),
        };
        assert!(valve.query(&c, "demo", "build").is_err(), "a malformed ledger must refuse");
        assert_eq!(std::fs::read(&v).expect("read"), bad_before, "the fail-closed refusal wrote the ledger");

        for (p, body) in before.iter().take(2) {
            assert_eq!(&std::fs::read(p).expect("read"), body, "a refusal path wrote {}", p);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
