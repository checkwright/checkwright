# SPEC amendment: stage-contract

A stage runs today only on the master harness: its contract is a skill that harness loads, its first step stamps, its last step journals, and its findings land in commits. A foreign coding agent in a read-only sandbox can do none of the three writes and loads no skill, so no stage's reading can be spent on another vendor's window. This is the stage-contract increment of the cross-vendor delegation entry, and its first slice: one read-heavy stage, the audit stage, read-only.

**The shape, in one sentence:** a stage session on the master harness enters and holds the stage as any session does, hands the stage's contract to a foreign agent as an audit-mode `--foreign-run`, and performs itself every write the returned report proposes.

**Why the delegating session is a stage session and never the lead.** The session that performs the state writes writes the stamp, and a lead writes no lifecycle state (lifecycle-kit/SPEC.md §The stamp protocol, the lead-stamp ruling-out). So the stage keeps one ordinary stamped session, here called the **host**, and nothing in the state machine learns a new writer.

**What the slice moves and what it leaves.** It moves the stage's *reading* onto the foreign window. The stamp, the tier verdict, the journal, every commit and the stage's oracle run stay the host's.

**The seam.** The arm, the frame's shipped text and the host protocol are kit mechanism and name no vendor. Which adapter a stage's reading rides is the dispatcher's pick by name (delegation-kit/SPEC.md §The foreign-vendor run), and the frame's path is consumer config. A per-stage executor binding is a later unit's.

## What changes

### (1) `--emit stage-contract <stage>` prints a stage's contract for a reader with no skill and no write path {design-bearing} {user-facing: operator direction 2026-10-08 on the queue entry — one read-heavy stage runs foreign in the read-only sandbox and the delegating session performs every state write}

**Not yet applied.** A new subsection of lifecycle-kit/SPEC.md §Per-component contracts. It is an `Arm::Emit` member on `--emit stage-rules`' precedent (doctrine-kit/SPEC.md §stage-rules): its contract is a document, and each failure is exit 2.

- **The document** is the frame (delta 2), one separator line, then the consumer's stage skill, `<LIFECYCLE_KIT_SKILLS_DIR>/<stage>.md`, verbatim. A binding shim's directive names its template by a repo-relative path the foreign agent reads in its clone, and a copied-and-specialized skill stands alone, so both adoption modes (§templates/stages/) are carried by the one rule and the arm resolves no template.
- **Exit 2, nothing printed:** a `<stage>` outside `LIFECYCLE_KIT_STAGES`, an absent skill file, an absent or empty frame file, a surplus operand.
- **It emits for any configured stage.** Which stage's reading is worth sending is the dispatcher's judgment in this slice, and the kit instructs one (delta 3).
- Its declared roster is `LIFECYCLE_KIT_STAGES`, `LIFECYCLE_KIT_SKILLS_DIR` and the new `LIFECYCLE_KIT_STAGE_CONTRACT_FRAME`.

**Why an arm, when a host could concatenate two files.** A prompt composed by hand per dispatch is a second spelling of the contract that nothing tests, and the later per-stage executor binding needs one derivation to call.

### (2) The frame is a kit template the consumer may replace {design-bearing} {user-facing: operator direction 2026-10-08 on the queue entry — the delegating session performs every state write, so the frame is where the foreign reader is told it performs none}

**Not yet applied.** `lifecycle-kit/templates/frames/stage-contract.md`, and the scalar knob `LIFECYCLE_KIT_STAGE_CONTRACT_FRAME` naming it, default that path, repo-root-relative. A consumer whose vendor needs different wording points the knob at its own file.

**`templates/frames/` is a third template class.** `templates/*.md` stays exactly the boundary skills and `templates/stages/*.md` exactly the stage templates (lifecycle-kit/SPEC.md §Layout and configuration). A frame is neither: no session invokes it as a skill, it carries no slot, and its one reader is the arm.

The frame is instruction only, and it tells its reader:

- **What it is doing:** executing the stage contract below on behalf of a session that holds the write path, in a clone at a committed revision where uncommitted and ignored content is absent.
- **It writes nothing.** No stamp, no commit, no journal line, no queue, spec or amendment edit, no filing. The contract's first step and last step are the delegating session's and are skipped. Every other step the contract words as a write is done as far as its reading goes and returned as a proposed write.
- **It runs only commands that change nothing**, and a command its sandbox refuses, or one needing a built artifact the clone lacks, is named in the report for the delegating session to run, never worked around.
- **The report's parts**, in order: findings, each with its path and line, the command or reading that shows it, and whether it was measured or inferred; proposed writes, each naming its surface and carrying its text; questions, each as Question / Options / Recommendation / Evidence; what was not done and why; and the reader's verdict on the stage's exit condition.

**The kit parses no report.** The parts are for the host's reading, as delegation-kit classifies no report.

### (3) The audit stage's template carries the host protocol {design-bearing} {user-facing: operator direction 2026-10-08 on the queue entry — the delegating session performs every state write; the lead's record of the same answer names the audit stage}

**Not yet applied.** One paragraph in `lifecycle-kit/templates/stages/align.md`, after the survey-record paragraph, read by a session whose dispatch names a foreign adapter. Instruction only:

1. Enter, stamp and verify your tier as any session of this stage does.
2. Write `--emit stage-contract align` to a prompt file in the scratch dir and append what your dispatch gave you that varies.
3. Run it as an audit-mode `--foreign-run` on the named adapter, under delegation-kit's foreign-run bullet: the budget read first, backgrounded with its liveness record.
4. Check each finding before you build on it. Perform each proposed write you accept yourself, in this stage's own commit. Answer a question from the governed surfaces or escalate it, and return an answer with `--foreign-resume` where the adapter resumes.
5. Run this stage's consistency gate yourself: the clone carries no built gate binary.
6. Record the run's verdict line in your journal and in the message of the commit that lands its findings.
7. A run that ends `FAILED` or `REFUSED` returns nothing to build on: execute the contract yourself, and where that needs a tier above your dispatch's, escalate a re-tier.

**The stamp says who held the stage, and the commit message says who read.** The five-field stamp grammar is unchanged: the id is the host's, and the tier verdict a stamp id resolves to is the host's model. The foreign reading is on record in the journal and the landing commit's message alone.

**Tiering.** The lead tiers a host by what its writes demand, the rule it already applies to every batch (lifecycle-kit/templates/lead.md §Economics): a report whose proposals the host can land after an oracle run is mechanical work, and one the host must turn into authored spec text is not.

### (4) The lead dispatches a foreign-read stage as an ordinary stage session {mechanical}

**Not yet applied.** `lifecycle-kit/templates/lead.md`, one sentence in the dispatch protocol: a stage whose reading runs foreign is dispatched as the stage-session type with the adapter named in the prompt, the lead running no `--foreign-run` for a stage itself. lifecycle-kit/SPEC.md §templates/lead.md carries the ground: a lead that ran it would hold a report whose writes it may not perform.

### (5) The foreign-vendor run admits a stage's reading {mechanical}

**Not yet applied.**

- `delegation-kit/SPEC.md` §The foreign-vendor run, the opening sentence: beside a read-only audit and a mechanical sweep, an audit-mode run may carry a stage's contract under lifecycle-kit's host protocol. The mechanical tier class stays the reading of the first two; a stage's reading carries its stage's class, and the adapter is still picked by name.
- The same section's honest limits gain two. **A foreign reading runs no oracle that needs a built artifact**, since the clone is committed state and the gate binary is not tracked. **Nothing verifies the reader's class**: the tier verdict reads the host's transcript, and an adapter binds to no tier class.
- `delegation-kit/templates/agent-execution.md`, the foreign-run bullet: a stage's contract runs there only as its stage template says.

## Producers and consumers

- **The emitted document.** *Producer:* the arm, called by a host (delta 3). *Enabling config:* the defaults; this repo sets neither knob the arm reads beyond its stage roster, and its skills dir holds a shim per stage. *Consumer:* `--foreign-run`, as a prompt file, which reads bytes.
- **The frame's report parts.** *Reader:* the host, at step 4. No program reads them.
- **`LIFECYCLE_KIT_STAGE_CONTRACT_FRAME`.** *Reader:* the arm. *Roster-holding readers of a new lifecycle-kit knob,* by `git grep -l LIFECYCLE_KIT_SHIM_NGRAM -- ':!docs'`: lifecycle-kit/SPEC.md §Layout and configuration, the kit's static table in `native/src/knobs/lifecycle_kit.rs` and `native/src/knobs/mod.rs`; `scripts/lifecycle-config.knobs` holds only knobs this repo sets and takes no row.
- **Roster-holding readers of a new `--emit` member,** by `git grep -l 'stage-rules\|stage_rules' -- ':!docs'`: the arm table in `native/src/emit/mod.rs`, the owning SPEC and README, and `.workflow/release-declarations.md`. The remaining hits read that one arm's output and hold no roster of arms.
- **Roster-holding readers of a new template file,** by `grep -n RATCHET scripts/context-config.knobs` and `grep -n templates .workflow/surface-ceiling.txt`: context-kit's surface ratchet, whose path set reaches every `*/templates/*.md` and whose ceiling file carries one row per file. `check-skill-binding` and `check-shim-restatement` couple the kit template dirs too; the frame carries no slot and no binding directive.
- **The host protocol.** *Producer of the trigger:* a dispatch prompt naming an adapter, or an operator saying so to a standalone session. *Reader:* the stage session, on the one surface every session of that stage loads.
- **No corpus is narrowed and no enumerable corpus is obliged member by member**, so causal-completeness points 5 and 6 bind nothing here.

**Inferred, cannot run before build:** that a shim's nine-word n-grams and the frame's do not collide under `check-shim-restatement` — the frame's text is build's to author, and the gate reads it once written.

## Acceptance, and what it leaves unobserved

**No foreign run is owed.** Operator direction 2026-10-08, relayed by the lead: no foreign run this iteration unless one is mandatory for this acceptance. It is not mandatory. Everything this amendment mints is a document, a template and prose, and each is accepted without a vendor:

- the crate's cases for the arm: the composed document for a shim and for a standalone skill, and each refusal;
- one case handing the emitted document to `--foreign-run` under the stub adapter the crate's foreign-run cases already use, in audit mode;
- one read-only child on the master harness, handed the emitted document for the audit stage and nothing else, its report read against the frame's parts. It shows the document is sufficient without the skill. It is not a foreign observation.

**Unobserved where no foreign run happens:** a foreign agent executing the contract in its vendor's read-only sandbox. That covers whether it holds to the no-write frame (a breach is caught as `REFUSED`, never prevented), whether its sandbox lets the contract's read-only commands run, and whether its report keeps the parts. The line is carried on the queue entry at the demotion.

**Where a foreign run is granted and its window is open at build**, the one granted run is this acceptance's last act: a host run of the audit stage's contract over the landed tree. It discharges the unobserved line, and its commit message archives the grant.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §Per-component contracts — the arm's subsection (delta 1); §Layout and configuration — the knob bullet and the third template class in the layout sentence (delta 2); §templates/stages/ — the host protocol's grounds, the stamp-and-commit record and the tiering reading (delta 3); §templates/lead.md — the dispatch sentence's ground (delta 4).
- `native/src/emit/` — the arm and its table row (delta 1); `native/src/knobs/lifecycle_kit.rs` — the knob row (delta 2).
- `lifecycle-kit/templates/frames/stage-contract.md` — new (delta 2); `.workflow/surface-ceiling.txt` — its row (delta 2).
- `lifecycle-kit/templates/stages/align.md` (delta 3); `lifecycle-kit/templates/lead.md` (delta 4).
- `lifecycle-kit/README.md` — the arm beside the arms it lists (delta 1).
- `delegation-kit/SPEC.md` §The foreign-vendor run — the opening sentence and two honest limits (delta 5); `delegation-kit/templates/agent-execution.md` — the bullet's clause; by `grep -n 'A foreign-vendor run is mechanical' .claude/agents/*.md delegation-kit/templates/*.md` no operative copy restates the bullet (delta 5).
- `.claude/agents/stage-session.md` — the list of what a dispatch prompt varies gains the adapter (delta 4).
- `.workflow/release-declarations.md` — a row for the arm, the knob and the frame (deltas 1 and 2).
- `docs/lifecycle-kit/`, `docs/delegation-kit/` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).
- `TASK-QUEUE.md`, the cross-vendor delegation entry — its remaining-work line, the landed slice and the unobserved line, rewritten inside the entry cap in the demoting commit (all deltas); the per-stage executor entry's gate sentence, which this slice opens (delta 1).

## Retired spellings

- None — no delta of this amendment retires a name; delta 5 rewrites one opening sentence in place.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Acceptance as stated above**, the unobserved line either discharged by the granted run or carried on the entry.
- [ ] **The marker discharged** — the cannot-run claim under Producers and consumers is read off the gate once the frame is written.
- [ ] **Amendment deleted** — this file removed on merge; no root amendment of this unit remains (`ls SPEC-stage-contract.md`).
- [ ] **Queue entry demoted, not done** — the entry outlives this amendment, its later stage-contract slices being unbuilt: `--queue demote heterogeneous-agent-delegation`, in the merge commit and before the stage that drains the queue, its body brought inside the entry cap in that commit.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
