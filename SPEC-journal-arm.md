# SPEC amendment: journal-arm

Paired with the queue entry `journal-append-arm`. A stage session appends to its resume journal by a shell redirect onto the path its entry printed, so every append spells that path, and the shell guard's scratch-write grant is bounded to a one-statement command, so the append is a call of its own (guard-kit/SPEC.md §The generic ruleset, rule `append_scratch`). The manual-operation meter ranks the shape first: re-run at authoring over `drift-kit-tail-crosser-pass`, `printf >>` is 49 calls in 7 sessions and `cat >>` 40 in 5, across every stage row.

The entry homed the arm in delegation-kit's journal section. This amendment homes it in lifecycle-kit, and the ground is delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps itself: that kit owns the journal's contract and states no path convention, and lifecycle-kit owns the path. Every input of the arm is lifecycle-kit's: the stamp, the id derivation and the journal-path derivation. delegation-kit's section and its template cite the arm (delta 3), which is the half of the entry's homing that survives.

The call is an ordinary front-end invocation with no redirect. Probed at authoring through `--hook shell-guard` with two payloads, a bare-operand call and a quoted-heredoc call, each compounded with a `grep`: the guard decides neither, so the call rides the consumer's existing allow entry for the front end and can share a command with the read that produced the finding.

## What changes

### (1) The journal arm {design-bearing} {user-facing: the entry's deliverable, an append arm writing its operand or stdin to the journal the session's own stamp names, in the operator's selection of the unit set, direction 2026-10-06}

A new arm, `--emit-journal`, in `native/src/emit/journal.rs` with its row in the arm table of `native/src/emit/mod.rs`, a `KNOBS` roster and a `#[cfg(test)]` module. The caller-stage read moves out of `native/src/hook/workflow_state.rs` into the shared stage adapters, so the guard and the arm call one function. A new section in lifecycle-kit/SPEC.md, after §bin/enter-stage.sh. *Not yet applied.*

> ### The journal arm
>
> `run-gates.sh --emit journal [--] "<text>"` appends its operand to the resume journal of the stage the calling session entered, and with no operand appends its standard input. A stage session journals without spelling the path: the entry tool derived it once (§The state machine), and this arm derives it again from the caller's own stamp.
>
> - **Which journal.** The caller's id is §bin/session-id.sh's derivation, the one its entry stamped. Its stage is its **last stamp's** stage, the read the workflow-state guard's third rule makes (§check-dispatch-entry). The path is the journal-path derivation over that stage (§The stage-machine adapters). The stage is never the cursor's: a session resumed after its successor entered still owes its own stage's journal.
> - **The write.** The text lands verbatim in one append, a closing newline added where it lacks one, and the file and its directory are created when absent. Nothing is prefixed: the heading the opener wrote already names the session (§bin/enter-stage.sh), and `DONE` must be able to stand as the last line (delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps).
> - **Stdout** is one line, `journal: <path> <stage> <id>`. The caller reads the stage and id against its own stamp, because the derivation's newest-transcript race is this arm's too.
> - **Refusals, exit 2, nothing written:** an empty text; a second operand; an unrecognized leading `-`, with `--` ending option processing (gate-sdk/SPEC.md §The bin/-tool contract); an id no stamp carries, the message naming the id and the remedy, `--enter-stage <stage>` first; and a linked worktree, on `--emit file-gap`'s ground that a line written there never reaches the main checkout (§The committed gap inbox).
> - **`LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` is not read.** That knob gates the opener and the predecessor assertion; a session calling the arm has asked for the file.
>
> Its declared roster is `LIFECYCLE_KIT_STATE_FILE`, `LIFECYCLE_KIT_STAGES` and `LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN`. It is advisory tooling, so no fixture pair is owed; the hermetic cases live in `gate-tests/journal-arm.test.sh`. The raw append to the path the entry printed stays a legal fallback, the contract being the file's and not the writer's.
>
> **Honest limits.** A session whose derived id is a sibling's writes where that sibling's stamp points. For a same-stage sibling that is the same file, the shared journal by design. For a resumed session racing a live successor it is the successor's file, which the printed line shows and the raw append avoids. A supervising session's journal has no stamp, so the arm does not reach it (§bin/enter-stage.sh).

lifecycle-kit/SPEC.md §The stage-machine adapters, the journal-path paragraph's second and third sentences. *Not yet applied.*

> It is hoisted on the cursor's ground: its readers must name one file, or the assertion checks a path nobody was asked to write. They are `--enter-stage`, at the opener and at the entry assertion, and `--emit journal`, through which the stage session writes (§The journal arm). The **caller-stage read**, a session's last stamp's stage, is hoisted beside it for its two readers, the workflow-state guard's third rule and that arm.

lifecycle-kit/SPEC.md §The state machine, the closing sentence of the paragraph opening *A stage owes a resume journal*. *Not yet applied.*

> The entry tool opens the journal at the stamp and reports the path it wrote (§bin/enter-stage.sh), and the journal arm appends there from the caller's stamp (§The journal arm). The tools compute the derivation, so no surface gains a second spelling of it.

lifecycle-kit/README.md's arm list gains one line after the `--emit file-survey` line. *Not yet applied.*

> "$gates" --emit journal "<finding>"   # append to the resume journal of the stage you entered

`.workflow/release-declarations.md` takes the arm and delta 2's note under *Behavior changes*: new, nothing to do; a stage session may append through it where it spelled a redirect.

### (2) The entry report names the arm {mechanical} {user-facing: the entry's deliverable, so no session spells the path, as delta 1}

`native/src/emit/enter_stage.rs`, the note printed when the journal is opened. *Not yet applied.*

> note: resume journal opened at <path> — append each finding as you confirm it with --emit journal "<finding>"; your stage template's last step owns what it owes at the end.

lifecycle-kit/SPEC.md §bin/enter-stage.sh, under *The stage journals*, the sentence *The printed path is the entering session's one source for it.* *Not yet applied.*

> The printed path is the entering session's one source for it, and the note beside it names the arm that appends there (§The journal arm).

The stage templates' last step is unchanged: it names the `DONE` act and its two owners, and the entry report is where the session learns how to append.

### (3) delegation-kit cites the arm {mechanical}

delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps, the paragraph *A stage session takes no grant*: its third sentence and its last. *Not yet applied.*

> Its entry tool opens the journal and reports the path to the entering session, and its journal arm appends there for a session that spells no path (lifecycle-kit/SPEC.md §The state machine, §The journal arm). That report is the path's one source.

> This kit owns the journal contract and states no path convention, and that kit owns the path and the arm that resolves it and states no contract.

delegation-kit/templates/agent-execution.md, the **Resume journal — agent writes, scratch reset sweeps** bullet's parenthesis on the stage session. *Not yet applied.*

> (a lifecycle-kit stage session is the standing exception — its own `--enter-stage` entry derives the path and its `--emit journal` arm appends there, so it is granted no path and spells none; lifecycle-kit/SPEC.md §The journal arm)

The bullet's **Spell the append as its own bare statement** sentence stands: it binds every session that journals by redirect, which a granted child and a supervising session still do.

### (4) The arm's tests {design-bearing}

The module's unit tests cover the text shaping (a closing newline added once, never doubled; an empty text refused) and the linked-worktree refusal. `lifecycle-kit/gate-tests/journal-arm.test.sh` drives the arm through `gate_arm_run` in a non-git sandbox with its own state file and journal pattern, the caller's id pinned through `LIFECYCLE_KIT_SESSION_ID`, and asserts:

- an operand lands as the last line of the stamped stage's journal, under the heading an opener wrote, and stdout is the one line with that path, stage and id;
- a multi-line standard input lands whole, and `DONE` passed as the operand is the file's last line;
- a caller stamped for one stage while the cursor names a later one writes its own stage's journal;
- a caller with stamps for two stages writes the later stamp's, and a waiver line carrying its id is not read as a stage;
- an id no stamp carries, an empty operand, an empty standard input, a second operand and a dash-led operand each exit 2 and leave every journal byte-identical, and `--` admits a dash-led text;
- an absent journal and directory are created, with `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` at `0` as at `1`;
- a retargeted `LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN` is followed.

### (5) This repository's stage-session definition {mechanical}

`.claude/agents/stage-session.md`, the **Resume journal** bullet's third sentence. *Not yet applied.*

> Your journal is the one your own `--enter-stage` opened; append to it with `bash gate-sdk/bin/run-gates.sh --emit journal "<finding>"` (lifecycle-kit/SPEC.md §The journal arm).

## Producers and consumers

- **The arm** (delta 1): produced by a stage session's call, which the entry report (delta 2) and this repository's stage-session definition (delta 5) instruct. Its three knobs have working defaults, so the shipped configuration enables it; a consumer with `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` at its default is told of the arm by no entry note and can still call it.
- **The appended text** (delta 1): read by the readers the journal already has, the predecessor-journal assertion, a lead's pull read and a resuming session. The assertion's written predicate reads a line the opener did not write, which an arm-appended line is.
- **The stdout line** (delta 1): read by the calling session. `path` tells it where the line went, and `stage` and `id` let it check the derivation picked its own stamp. No count or timestamp is printed, since no reader takes one.
- **The caller-stage read** (delta 1): one function with two callers after the move, the workflow-state guard's third rule and the arm. The guard's verdicts are unchanged, which `lifecycle-kit/gate-tests/stamp-before-write.test.sh` holds.
- **The id derivation** (delta 1): `native/src/sessions.rs`, called as `--enter-stage` calls it, unchanged. The arm is reached through the front end from the repository root, where that route and the entry's agree on the working directory the derivation reads (lifecycle-kit/SPEC.md §bin/session-id.sh).
- **The entry note** (delta 2): read by the entering session. Its retired wording is pinned by no test (`git grep` over the tracked tree finds it in the source alone).
- **Roster-holding readers of the new arm**, each red when it is missing: the arm table's own unit test, which resolves each row to its file and reds a file with no test module (gate-sdk/SPEC.md §The non-gate arm); this repository's `check-crate-arms`, which runs the crate's tests; and `check-reads-couples` and `check-gate-substrate-parity`, which read a row's declared knobs. lifecycle-kit/README.md's arm list is hand prose no gate holds.
- **Readers left as they are:** guard-kit's rule `append_scratch` still grants the redirect form, which a granted child, a supervising session and the fallback use. The stage templates' last step and `check-stage-skill-coverage`'s read of it are untouched.
- **Corpus narrowing:** none.

## Existing sections updated

- `lifecycle-kit/SPEC.md` — §The journal arm, new; §The stage-machine adapters; §The state machine (delta 1); §bin/enter-stage.sh (delta 2).
- `native/src/emit/journal.rs`, new; `native/src/emit/mod.rs`, the arm-table row; `native/src/hook/workflow_state.rs` and `native/src/stages.rs`, the caller-stage read moved (delta 1).
- `native/src/emit/enter_stage.rs` — the note (delta 2).
- `lifecycle-kit/README.md` — the arm list (delta 1).
- `delegation-kit/SPEC.md` — §Resume journal — agent writes, scratch reset sweeps (delta 3).
- `delegation-kit/templates/agent-execution.md` — the journal bullet's parenthesis (delta 3).
- `lifecycle-kit/gate-tests/journal-arm.test.sh`, new (delta 4).
- `.claude/agents/stage-session.md` (delta 5).
- `.workflow/release-declarations.md` — *Behavior changes* (deltas 1 and 2).
- `docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md` and `docs/delegation-kit/SPEC.md` — the generated mirrors, regenerated by the command their freshness gate prints (deltas 1, 2 and 3).
- `.workflow/surface-ceiling.txt` — the rows of lifecycle-kit/SPEC.md, delegation-kit/SPEC.md, delegation-kit/templates/agent-execution.md and `.claude/agents/stage-session.md`, each re-stamped in the growing commit where the merge takes its file past it (deltas 1, 2, 3 and 5).

Produced by `git grep -n -i journal` over the kit SPECs, the kit templates, `.claude/` and `native/src`, read for each site that names how a stage session reaches or writes its journal.

## Retired spellings

- `land your findings there as you confirm them` — the entry note's wording, replaced by the sentence naming the arm (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, its stdout line and the moved read.
- [ ] **Instruction surfaces: instruction only** — the entry note, the template parenthesis and the agent-definition sentence carry no grounds; §The journal arm carries them.
- [ ] **Merged with no information lost** — each replaced sentence is rephrased in place, and the new section reads as one document with §bin/enter-stage.sh's journal paragraphs.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repository root.
- [ ] **Removals propagated** — the one declared spelling survives nowhere.
- [ ] **Gaps filed** — a cross-component gap found at build is resolved that session.
- [ ] **Built and run** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, and the fixture suites of lifecycle-kit and delegation-kit, with `gate-tests/journal-arm.test.sh` and `gate-tests/stamp-before-write.test.sh`.
- [ ] **Run in the session that lands it** — the build session appends a line and its `DONE` through the arm, and the printed stage and id are its own stamp's.
- [ ] **Entry moved** — `--queue done journal-append-arm` in the build batch that lands this, a stage before the drain stage; no remote oracle gates it.
