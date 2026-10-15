# SPEC amendment: foreign-build

The host protocol moves a stage's reading onto a foreign agent and keeps every write with the host, so it admits every stage whose writes a host can land from a report. The build stage's writes cannot be landed that way, since they are proved by running what they produce, and its template alone carries no pointer line (§The host protocol). The cross-vendor delegation entry's last remaining item is that stage: one whose writes a foreign agent performs.

The pieces exist. A sweep-mode `--foreign-run` lets the agent edit a clone and returns its whole change as `change.patch`. A resumable adapter carries a session across turns, and each turn regenerates the patch as the session's whole change (delegation-kit/SPEC.md §Resuming a session). What is missing is the contract a writing agent reads, and the host's half of the loop: applying the patch, running the oracle the clone cannot run, and returning a red. This amendment adds both, as a second mode of the one protocol, and points the build template at it.

## What changes

### (1) `--emit stage-contract` takes a mode, and a sweep frame ships {design-bearing} {user-facing: the heterogeneous-agent-delegation entry's Remaining item (4), a stage whose writes a foreign agent performs, accepted on stub-driven crate cases (operator direction 2026-10-15, lead-relayed, not a ruling)}

`--emit stage-contract <stage> [--mode audit|sweep]` composes the document from the frame its mode names. The mode word is `--foreign-run`'s, and `audit` is its default here as there, so a host passes one word to both arms.

- **The frame per mode.** `audit` reads `LIFECYCLE_KIT_STAGE_CONTRACT_FRAME`, unchanged. `sweep` reads a new scalar knob, `LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME`, a repo-root-relative path at the same anchor, defaulting to `lifecycle-kit/templates/frames/stage-contract-sweep.md`. Each mode reads its own knob alone, so an absent sweep frame refuses only a `sweep` emit.
- **The shape.** `--mode` takes one value, before or after the stage. A missing value, a value other than the two and a repeated `--mode` are exit 2 with the usage. Every other dash-led token stays the `unrecognized option` refusal that `gate-sdk/gate-tests/emit-arm-shape.test.sh` holds for this arm, and `--` still ends option processing.
- **The crate.** `native/src/emit/stage_contract.rs` reads the mode and the knob its mode names. Its `KNOBS` and `USAGE` name the new knob and the option. The knob is a row of `native/src/knobs/lifecycle_kit.rs`'s static table beside the audit frame's. Its unit tests add: a `sweep` document composed from the sweep frame; an absent sweep frame refused in `sweep` mode and ignored in `audit` mode; and the three malformed `--mode` shapes refused.
- **The kit README's** usage line for the arm, in `lifecycle-kit/README.md`, becomes `"$gates" --emit stage-contract <stage> [--mode sweep]   # a stage's contract for a foreign agent in a clone, which commits nothing`.

The sweep frame, a new file `lifecycle-kit/templates/frames/stage-contract-sweep.md`. **Not yet applied.**

```text
You are executing the stage contract printed below the separator line, on behalf of a delegating session that holds the commit path. You are in a clone of the repository at a committed revision: uncommitted and ignored content is absent from it.

**Your edits are your return.** Make the contract's changes to the repository's files in this clone's working tree. The delegating session takes your whole change as one patch, runs what it produces, and commits it. Never commit, and never stamp, journal, push or file anything. The contract's first step and its last step belong to the delegating session: skip both. Return as a proposal any write the contract makes through a command the clone lacks.

**Run only commands that change nothing outside this clone.** Where your sandbox refuses a command, or a command needs a built artifact the clone lacks, name that command in your report for the delegating session to run. Never work around it. Output the repository ignores stays out of your change; anything else you create joins it.

Where the contract names a template or another file by a repository-relative path, read it in the clone.

When the delegating session returns a failure, continue in this clone, where your change still stands, and fix what failed.

**Your report**, as your final output, in these parts and this order:

1. **Changes** — each path you changed, what changed and the contract step it carries out.
2. **Proposed writes** — each write you could not make in the clone, naming the surface it lands on and carrying its full text.
3. **Commands to run** — each command the delegating session must run, the checks your change needs among them.
4. **Questions** — each as Question / Options / Recommendation / Evidence.
5. **Not done** — every step you did not carry out, and why.
6. **Verdict** — whether the stage's exit condition holds for the work you were given, by your reading.
```

Replacement for §The stage-contract arm's opening paragraph. **Not yet applied.**

> `run-gates.sh --emit stage-contract <stage> [--mode audit|sweep]` prints a stage's contract for a reader that loads no skill and holds no commit path: a foreign coding agent in a scratch clone, which can make none of a stage's three lifecycle writes, the stamp, the journal and the commit. In `audit` mode it moves the stage's reading onto that reader, and in `sweep` mode its file writes too. Every lifecycle write stays with the stage session that called it (§The host protocol). The mode word is `--foreign-run`'s, `audit` by default in both arms, so a host passes one word to each.

Replacement for that section's **The frame** bullet. **Not yet applied.**

> - **The frame** is the file the mode's knob names: `LIFECYCLE_KIT_STAGE_CONTRACT_FRAME` in `audit` mode, shipped as `templates/frames/stage-contract.md`, and `LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME` in `sweep` mode, shipped as `templates/frames/stage-contract-sweep.md`. Each is instruction only. Each tells its reader that it executes the contract below on behalf of a session holding the commit path, in a clone at a committed revision, skipping the contract's first and last steps. Each has it name any command its sandbox refuses or that needs a built artifact rather than work around it, and orders the report's parts. The audit frame has the reader write nothing, return every write as a proposal carrying its full text and run only commands that change nothing. Its report gives findings with their evidence, each marked measured or inferred, then proposed writes, questions, what was not done, and a verdict on the exit condition. The sweep frame has the reader make the contract's file changes in the clone and never commit, and return as a proposal a write needing a command the clone lacks. Told of a failure, it continues in the same clone. Its report gives its changes in place of findings, and the commands the host must run after the proposed writes.

Replacement for that section's exit-2 bullet. **Not yet applied.**

> - **Exit 2, nothing printed:** a `<stage>` outside `LIFECYCLE_KIT_STAGES`, an absent skill file, an absent or empty frame file for the mode asked, a missing or surplus operand, and a `--mode` with no value, a value other than `audit` and `sweep`, or a second `--mode`.

Replacement for that section's roster sentence, the second sentence of its paragraph beginning *It is an `Arm::Emit` member*. **Not yet applied.**

> Its declared roster is `LIFECYCLE_KIT_STAGES`, `LIFECYCLE_KIT_SKILLS_DIR`, `LIFECYCLE_KIT_STAGE_CONTRACT_FRAME` and `LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME`, the three paths at the journal-path derivation's anchor (§The stage-machine adapters).

Replacement for the same paragraph's last sentence. **Not yet applied.**

> It is advisory tooling, so no fixture pair is owed; its cases are crate tests beside the arm. One hands the document composed for each stage whose pointer line names no mode to an audit-mode foreign run under a stub adapter, and one hands each `sweep` stage's document to a sweep-mode run under a stub that writes (delegation-kit/SPEC.md §Testing).

New row in §Layout and configuration, after `LIFECYCLE_KIT_STAGE_CONTRACT_FRAME`'s. **Not yet applied.**

> - `LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME` — the frame `--emit stage-contract --mode sweep` prints ahead of a stage's skill (§The stage-contract arm), a repo-root-relative path; default `lifecycle-kit/templates/frames/stage-contract-sweep.md`. A consumer whose foreign vendor needs different wording points it at its own file.

The release declaration (`.workflow/release-declarations.md`) gains two bullets.

- §Knob changes: `LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME` — new, read by `--emit stage-contract --mode sweep` (Behavior changes): the repo-root-relative path of the frame printed ahead of a stage's skill for a foreign agent that edits its clone, default `lifecycle-kit/templates/frames/stage-contract-sweep.md`. Nothing to do; point it at your own file where a foreign vendor needs different wording.
- §Behavior changes: **lifecycle-kit `--emit stage-contract`, new template `lifecycle-kit/templates/frames/stage-contract-sweep.md`, lifecycle-kit/templates/host-protocol.md, lifecycle-kit/templates/stages/build.md, lifecycle-kit/templates/lead.md** — the arm takes `--mode sweep`, printing a contract for a foreign agent that makes the stage's file changes in its clone. The host protocol gains that mode: the host applies the run's patch, runs its oracle, returns a red with `--foreign-resume`, and commits what passes. The build template now carries the protocol's pointer line naming `sweep`, so `LIFECYCLE_KIT_STAGE_EXECUTOR` may bind the build stage. Nothing to do; a stage you do not bind runs as before.

### (2) The host protocol gains its `sweep` mode {design-bearing} {user-facing: the heterogeneous-agent-delegation entry's Remaining item (4), as delta 1}

The protocol keeps one file and one section, and gains a mode its pointer line names. In `sweep` mode the foreign agent is the author and the host is the oracle: the host applies the patch, runs what the change produces, and returns each red to the same session.

- **Why a mode of the one protocol, not a second protocol.** The steps that enter, compose, run, record and fall back are the same steps. `check-stage-skill-coverage`'s fourth direction already admits a bound stage by its surface citing `lifecycle-kit/SPEC.md §The host protocol`, so a build template that cites it is bindable with no gate change.
- **Why the patch goes back out before a resume.** Each turn's patch is the session's whole change against its base, so a later patch applies only to the tree the first one applied to. The host edits nothing in the applied change while the session is open, since an edit the agent never saw diverges the agent's clone from the host's tree. On a one-shot adapter a red ends the foreign part and the host finishes the unit.
- **The tier.** A `sweep` host verifies authored work against its amendment and finishes any unit the run does not, so the batch's work-class labels tier it as they tier any build session. What moves foreign is the authoring spend.

Replacement for `lifecycle-kit/templates/host-protocol.md`, whole. **Not yet applied.**

```text
The host protocol: your dispatch names a foreign adapter, so this stage's work runs there and every lifecycle write stays yours. Your mode is `sweep` where your stage template's pointer line names it, else `audit`.

1. Enter, stamp and verify your tier as any session of your stage does.
2. Write `--emit stage-contract <your stage> --mode <your mode>` to a prompt file in the scratch dir and append what your dispatch gave you that varies. Append nothing read from a file the repository ignores.
3. Run it as a `--foreign-run` in your mode on the named adapter with `--class <the class your dispatch names for the foreign work, else your own tier class>`, under delegation-kit's foreign-run bullet (delegation-kit/templates/agent-execution.md): the budget read first, backgrounded with its liveness record.
4. Check each finding before you build on it. Perform each proposed write you accept yourself, in your stage's own commits. Answer a question from the governed surfaces or escalate it, and return an answer with `--foreign-resume` where the adapter resumes.
5. In `sweep` mode, apply the run's `change.patch` to your checkout and verify it as any agent's work, running your stage's oracle over it. On a red, take the patch back out and return the red with `--foreign-resume`, then apply only the newest turn's patch, which holds the session's whole change. Where the adapter does not resume, finish the unit yourself. Edit the change yourself only once the session is closed, and commit only a change your oracle passed.
6. Run yourself every command the report names as refused or as needing a built artifact, your stage's gates among them.
7. Record the run's verdict line in your journal and in the message of the commit that lands its work.
8. A run that ends `FAILED` or `REFUSED` returns nothing to build on: execute the contract yourself, and where that needs a tier above your dispatch's, escalate a re-tier.
```

Replacement for §The host protocol, whole. **Not yet applied.**

> ### The host protocol
>
> **A stage whose work runs foreign is held by a host: a session of that stage whose dispatch names a foreign adapter.** The host enters, stamps and holds the stage as any session does. It hands the stage's contract to the foreign agent as a `--foreign-run` of `--emit stage-contract`'s document in the stage's mode (§The stage-contract arm), and lands every write itself: each proposal the report carries and, in `sweep` mode, the run's patch. The host is a stage session and never the lead, because the session that performs the state writes writes the stamp, and a lead writes no lifecycle state (§The stamp protocol). So nothing in the state machine learns a new writer. A run that fails or is refused returns nothing to build on, and the host executes the contract itself. Which adapter the work rides is the dispatcher's pick by name, and a consumer may bind a stage's pick in `LIFECYCLE_KIT_STAGE_EXECUTOR` (§Layout and configuration).
>
> **The steps are one file, `templates/host-protocol.md`, worded for whichever stage the host entered.** A stage template that admits the protocol carries one line pointing at the file and restates none of it, and names `sweep` where the stage admits that mode. The file is load-triggered: a session reads it only when its dispatch names an adapter, so a stage that runs on the master harness pays nothing for it.
>
> **The prompt carries nothing read from a file the repository ignores.** The host appends to the emitted document what its dispatch gave it that varies, and no more. The clone already withholds ignored content from the foreign agent (delegation-kit/SPEC.md §The foreign-vendor run, **The tree**), and a host that pasted a private brief into the prompt would hand a vendor what the clone was built to keep back.
>
> **The host runs every command the report names as refused or as needing a built artifact**, its stage's gates among them. The frame has the foreign agent name each one rather than work around it (§The stage-contract arm), and the clone carries no built gate binary.
>
> **A stage admits the protocol in the mode its writes are proved by.** A stage whose work separates into a reading and writes a session can land from a report admits `audit`: the frame returns every write as a proposal carrying its full text, which the host checks and lands. Authoring, auditing, surveying and reviewing separate that way. The build stage's work does not, since its writes are proved by running what they produce, a loop that needs the write path and a built artifact inside it. It admits `sweep`, in which the foreign agent makes the writes in its clone and the host runs the loop's oracle half. Every shipped stage template carries the pointer line, and the build stage's alone names `sweep`.
>
> **In `sweep` mode the agent authors and the host is the oracle.** The host applies the run's `change.patch` to its checkout and verifies it as any agent's work (delegation-kit/SPEC.md §Verify after every agent commit), running what the change produces. A red goes back to the agent with `--foreign-resume` once the host has taken the patch back out. Each turn's patch is the session's whole change against its base (delegation-kit/SPEC.md §Resuming a session), so the host applies only the newest. Each red costs a turn of the foreign vendor's window. The host edits the change only once the session is closed, since an edit the agent never saw diverges its clone from the host's tree. It commits only a change its oracle passed, under its own attribution (delegation-kit/SPEC.md §The foreign-vendor run, **What returns, and how it lands**). On an adapter that does not resume, the first red ends the foreign part and the host finishes the unit.
>
> **What foreign work cannot reach, in either mode:**
>
> - An oracle needing a built artifact, the gate binary first. The host runs it, so an `audit` stage that is mostly oracle-running returns mostly a command list, and each red in `sweep` mode costs a turn. A consumer weighs that before sending one.
> - Content the repository ignores or has not committed. A stage whose work rests on a private surface gets it done without that surface, and the host supplies that part.
> - A remote act: a push, a release, an authenticated read of a hosting service. The first two are writes and the third is the sandbox's to refuse; each is the host's.
> - A lifecycle write. The stamp, the journal, the commit and a queue move through the gate binary are the host's in both modes.
>
> **The stamp says who held the stage, and the commit message says whose work it lands.** The stamp grammar is unchanged: the id is the host's, and the tier verdict a stamp id resolves to is the host's model. The foreign work is on record in the host's journal and the landing commit's message alone, each carrying the run's verdict line. **A host is tiered by what its writes demand**, templates/lead.md §Economics' rule for every batch. An `audit` report whose proposals the host can land after an oracle run is mechanical work, and one the host must turn into authored spec text is not. A `sweep` host verifies authored work against its amendment and finishes any unit the run does not, so the batch's work-class labels tier it as they tier any build session.

Replacement for §templates/stages/'s sentence *A template whose stage admits a foreign reading carries one line pointing at the host protocol*. **Not yet applied.**

> **A template whose stage admits the host protocol carries one line pointing at it**, naming `sweep` where the stage admits that mode, for a session whose dispatch names a foreign adapter (§The host protocol).

### (3) The build template carries the pointer line, naming `sweep` {mechanical} {user-facing: the heterogeneous-agent-delegation entry's Remaining item (4), as delta 1}

`lifecycle-kit/templates/stages/build.md` gains one paragraph, placed before *Then read your successor's entry*, where the spec, scope, validate and close templates place theirs. **Not yet applied.**

```text
**Where your dispatch names a foreign adapter**, this stage's file changes are made there in `sweep` mode and every lifecycle write stays yours: follow lifecycle-kit/templates/host-protocol.md (lifecycle-kit/SPEC.md §The host protocol).
```

The five other templates' pointer lines are unchanged, and name no mode, so their hosts run `audit`.

`admitted_stages` in `native/src/emit/foreign_run.rs` asserts today that the build template alone carries no pointer line. It becomes two reads over the shipped stage templates. Every template carries the pointer, and the templates whose pointer line names `sweep` are exactly `build`. `a_stage_contract_document_runs_as_an_audit_prompt` runs over the audit set, and delta 5 adds the sweep set's case.

### (4) The lead, the binding and the delegation template speak of a stage's foreign work {mechanical}

Three sentences say *reading* where a `sweep` stage sends writes too. **Not yet applied.**

- `lifecycle-kit/templates/lead.md`, *Under either posture*: "Dispatch a stage whose reading runs foreign as the same stage-session type …" becomes "Dispatch a stage whose work runs foreign as the same stage-session type …".
- `lifecycle-kit/templates/lead.md`, *Read the stage's executor before you dispatch it*: "with the class the reading rides" becomes "with the class the foreign work rides".
- `lifecycle-kit/SPEC.md` §templates/lead.md: "**A stage whose reading runs foreign is dispatched as an ordinary stage session**" becomes "**A stage whose work runs foreign is dispatched as an ordinary stage session**".
- `lifecycle-kit/SPEC.md` §Layout and configuration, the `LIFECYCLE_KIT_STAGE_EXECUTOR` row: "one `<stage>=<adapter>` element per stage whose reading runs foreign" becomes "… per stage whose work runs foreign", and its first sub-bullet's "from the class the reading names" becomes "from the class the foreign run names".

### (5) delegation-kit names the sweep-mode stage contract {mechanical}

**Not yet applied**, each sentence in `delegation-kit/SPEC.md`:

- §The foreign-vendor run, the opening paragraph's second sentence becomes: "An audit-mode run may also carry a stage's contract under lifecycle-kit's host protocol (lifecycle-kit/SPEC.md §The host protocol), a reading that carries its stage's class, and a sweep-mode run may carry the contract of a stage whose writes run foreign, or a project critique a consultation commissions (lifecycle-kit/SPEC.md §templates/consult.md)."
- §The foreign-vendor run, the verdict line's reader: "and the commit message that records a foreign reading" becomes "and the commit message that lands a foreign run's work".
- §The foreign-vendor run's honest limit **A foreign reading runs no oracle that needs a built artifact.** becomes "**A foreign run executes no oracle that needs a built artifact.** The clone is committed state and the gate binary is not tracked, so a stage's host runs the gate itself, and in `sweep` mode each red it finds costs a turn."
- §Testing, the last sentence of the paragraph on `native/src/hook/verdict.rs` becomes: "One foreign-run row hands the contract composed for each stage whose pointer line names no mode to an audit-mode run under the stub adapter, which reads it whole, and one hands the build stage's `sweep` contract to a sweep-mode run under a stub that writes a file, whose `change.patch` carries it (lifecycle-kit/SPEC.md §The stage-contract arm)."

The crate case in `native/src/emit/foreign_run.rs`: the sweep set's documents, composed with a sweep-frame stand-in, each run under a stub adapter that reads its prompt whole and writes one file. The line is `mode=sweep … -> OK`, `change.patch` carries the file, and no commit is taken.

`delegation-kit/templates/agent-execution.md`'s foreign-run bullet already defers a stage's contract to its stage template ("A stage's contract runs there only as its stage template says"), and needs no change.

## Producers and consumers

- **The `--mode` option.** Producer: a host, at host-protocol step 2, with the mode its stage template's pointer line names. Consumer: `stage_contract::emit`, which reads it to pick the frame knob. The one value has that one reader.
- **`LIFECYCLE_KIT_STAGE_CONTRACT_SWEEP_FRAME`.** Producer: lifecycle-kit's static table default, which a deployed configuration reaches with no binding, and a consumer's knob file where it rebinds. Consumer: `stage_contract::emit` in `sweep` mode. Roster-holding readers of the knob set: the static table (`native/src/knobs/lifecycle_kit.rs`), the arm's `KNOBS`, §Layout and configuration and the release declaration (delta 1). The SPEC row and the table row land in one commit.
- **The sweep frame file.** Producer: the kit, shipped. Consumer: the foreign agent, by reading the prompt file a sweep-mode run hands its adapter on standard input. Its six report parts are read by the host at steps 4 to 6: changes against the applied patch, proposed writes by the host's own landing, commands by its own runs, questions by step 4, what was not done by its own completion, and the verdict as the agent's claim, which step 4 checks.
- **The pointer line naming `sweep`.** Producer: the build template (delta 3). Consumers: a build host, which reads its mode from it at the protocol's opening line; `check-stage-skill-coverage`'s fourth direction, which reads the citation alone and is unchanged; and `admitted_stages` (delta 3).
- **The applied patch, the red and the resume.** Producer: the host, at step 5. Consumer: the foreign agent's next turn in the same clone, on the vendor's resume. The red reaches it as the prompt file of `--foreign-resume`. Its one field, the oracle's output, is read by the agent's fix. The newest patch is read by the host's next application.
- **Readers of the host protocol's surfaces**, by `grep -rln 'host-protocol\|host protocol\|stage-contract\|STAGE_EXECUTOR\|STAGE_CONTRACT_FRAME'` over the tracked tree with the scratch dir, the crate's build output and the git directory excluded. Each is placed:
  - `lifecycle-kit/SPEC.md` — deltas 1, 2 and 4;
  - `lifecycle-kit/templates/host-protocol.md` — delta 2;
  - `lifecycle-kit/templates/stages/build.md` — delta 3;
  - `lifecycle-kit/templates/stages/{align,scope,spec,validate,close}.md` — their audit pointer lines read unchanged under delta 2's default mode;
  - `lifecycle-kit/templates/lead.md` — delta 4;
  - `lifecycle-kit/README.md` — delta 1;
  - `delegation-kit/SPEC.md` — delta 5;
  - `native/src/emit/stage_contract.rs`, `native/src/knobs/lifecycle_kit.rs` — delta 1;
  - `native/src/emit/foreign_run.rs` — deltas 3 and 5;
  - `native/src/emit/mod.rs` — the arm's row, which passes `stage_contract::KNOBS` and `USAGE` through by name, unchanged;
  - `native/src/gates/stage_skill_coverage.rs`, `native/src/gates/mod.rs`, `lifecycle-kit/checks/check-stage-skill-coverage.gate`, its test and fixture pair — the fourth direction, which reads the citation and is unchanged;
  - `gate-sdk/gate-tests/emit-arm-shape.test.sh` — its `--help` refusal for this arm holds under delta 1's shape;
  - `.workflow/release-declarations.md` — delta 1;
  - `.workflow/audit-roster.txt`, `.workflow/surface-ceiling.txt`, `docs/evidence-data.md`, `TASK-QUEUE.md` — history or measurement rows citing the names. The ceiling is the surface ratchet's, re-baselined where the battery reds on a template's growth (all deltas);
  - `docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md`, `docs/delegation-kit/SPEC.md` — generated (all deltas).
- **The seam.** The mode, the knob, the frame and the protocol's steps are kit mechanism and name no vendor. Which stage a consumer binds, and to which adapter, stays its configuration. This repo binds none, and this amendment binds none.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §The stage-contract arm — the opening, the frame bullet, the exit-2 bullet and the roster and testing sentences (delta 1).
- `lifecycle-kit/SPEC.md` §Layout and configuration — the new knob row (delta 1) and the executor row's two phrases (delta 4).
- `lifecycle-kit/SPEC.md` §The host protocol — the section, whole (delta 2).
- `lifecycle-kit/SPEC.md` §templates/stages/ — the pointer-line sentence (delta 2).
- `lifecycle-kit/SPEC.md` §templates/lead.md — the dispatch sentence (delta 4).
- `lifecycle-kit/templates/frames/stage-contract-sweep.md` — new (delta 1).
- `lifecycle-kit/templates/host-protocol.md` — whole (delta 2).
- `lifecycle-kit/templates/stages/build.md` — the pointer line (delta 3).
- `lifecycle-kit/templates/lead.md` — two phrases (delta 4).
- `lifecycle-kit/README.md` — the arm's usage line (delta 1).
- `native/src/emit/stage_contract.rs` — the mode, the knob, `KNOBS`, `USAGE` and the unit tests (delta 1).
- `native/src/knobs/lifecycle_kit.rs` — the knob row (delta 1).
- `native/src/emit/foreign_run.rs` — `admitted_stages` (delta 3) and the sweep case (delta 5).
- `delegation-kit/SPEC.md` §The foreign-vendor run and §Testing — four sentences (delta 5).
- `.workflow/release-declarations.md` — two bullets (delta 1).
- `.workflow/surface-ceiling.txt` — re-baselined where the ratchet reds (all deltas).
- `docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md`, `docs/delegation-kit/SPEC.md` — generated, stale once a delta lands (all deltas).

## Retired spellings

- None — the amendment adds an option, a knob, a frame and a protocol mode, and rewords prose without retiring a name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry demoted** — the entry outlives this amendment: its deliverable is the cross-vendor corpus, and a live foreign run of a `sweep` stage stays unobserved under the operator's direction of 2026-10-15. In the merging commit, ahead of the drain stage, `--queue demote heterogeneous-agent-delegation` returns it to its position. Its *Remaining* line is rewritten to the live run of a `sweep` stage and the two feed observations its budget-oracle paragraph names, and the entry is compressed inside the size cap. Whether it then closes is the operator's, since a roadmap-tagged entry is outside every exit (queue-kit/SPEC.md §The icebox tier).
