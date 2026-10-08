# SPEC amendment: consult-critique

An operator-started consultation may commission an outside critique of the project from a foreign coding agent, question it, and land what survives through the consultation's own landing contract. The amendment adds a template step, a prompt frame and one knob, and no arm.

## What changes

### (1) A consultation runs the critique itself, on the operator's request {design-bearing} {user-facing: operator directions 2026-10-08 on the queue entry — the critique runs on a foreign expert adapter and is reflected on by `/consult`, which may drive the run itself and ask follow-ups; the deliverable is a clear process to trigger later}

The trigger is the operator asking for it inside a consultation they started, and those words are the run's whole grant: a foreign run spends a window the iteration's budget does not price, so no session starts one unasked. A consultation whose request is to refresh the consumer's strategic assessment may **offer** the critique and waits for the answer. A consultation a lead dispatched runs none, since no operator sits in it to ask.

The consultation is both dispatcher and reflection pass, and the two shapes the operator named are one process:

- **Driven.** The consultation writes the frame (delta 2) and its own focus to a prompt file in the scratch dir, reads the adapter's budget, and runs an audit-mode `--foreign-run` under a key it names. Where the adapter resumes, it puts each clarifying question with `--foreign-resume` and closes the session when done.
- **Saved.** The report of a run made earlier is a file: `report.txt` under the run's key, or any path the operator hands the consultation. The consultation reads it as it would the report of a run it drove, and asks nothing where the session is closed or the adapter is one-shot.

One process, because the executor already returns its work as a durable file (delegation-kit/SPEC.md §The foreign-vendor run, **What returns**) and a consultation already dispatches under the delegation protocol (templates/consult.md, step 7). A separate saved-report channel would need a home that outlives the scratch dir, which the work-unit boundary wipes, and would reach the consultation with no way to question its author. The driven shape needs neither. A report the operator wants kept across that boundary is theirs to copy out; the kit mints no store for it.

**The reflection lands through the consultation's own contract, so no write path is minted.** A critique's findings are a child's report: each is checked against the tree or the consumer's own record before it is built on. What survives is advice. A reply stays a reply; a tracked change lands as the ordinary workflow lands one, the deferred entry or gap bullet the operator names, marked as step 3 marks advice; an edit to the consumer's strategic assessment lands on whichever entry-reading surface holds it. Nothing a critique proposes starts as work: it is filed, and scope decides (doctrine-kit/DOCTRINE.md, Scope-gated intake). A finding the consultation rejects is recorded with its grounds, as step 4 records a refused alternative.

**Nothing private reaches the prompt unasked.** The clone shows the foreign agent committed state alone. The consultation appends nothing read from a file the repository ignores unless the operator, in that session, names the content to send.

Replacement text for `lifecycle-kit/templates/consult.md`, a paragraph after the numbered ritual and before the dispatched-by-a-lead paragraph. **Not yet applied.**

```text
**An outside critique runs only when the operator asks for one in this session.** Where your request is to refresh the strategic assessment and `--emit knob-values LIFECYCLE_KIT_CRITIQUE_ADAPTER` names an adapter, offer one and wait. On a yes: copy `lifecycle-kit/templates/frames/project-critique.md` to a prompt file in the scratch dir and append the focus the operator gave. Append nothing read from a file the repository ignores unless the operator names it. Run it as an audit-mode `--foreign-run` on that adapter, or the one the operator names, under delegation-kit's foreign-run bullet (delegation-kit/templates/agent-execution.md): the budget read first, backgrounded with its liveness record. Put a clarifying question with `--foreign-resume` where the adapter resumes, and close the session when you are done. A report from an earlier run is read the same way, from the path the operator gives. Check each finding before you build on it, then land what survives as step 3 lands advice and record what you reject as step 4 records a refusal.
```

The dispatched-by-a-lead paragraph gains one clause: such a consultation runs no critique.

### (2) The critique's prompt is a shipped frame {design-bearing}

`lifecycle-kit/templates/frames/project-critique.md`, beside the stage-contract frame and instruction only. It tells its reader that it is in a clone of a project at a committed revision, reading on behalf of a session that will check its work. It writes nothing and runs only commands that change nothing. It asks for the project's strategic weaknesses rather than its defects: what the project claims and where the tree fails to back the claim, what a prospective adopter would meet first and stumble on, what the project depends on that it does not control, and what it has left unbuilt that its stated direction needs. And it orders the report: findings, each with the path or command that shows it and whether it was measured or inferred; what the reader could not assess from the tree alone; and the three findings it would act on first.

The frame names no vendor, no competitor and no assessment format, so it ships. A consumer wanting its own copies the consult template and cites its own frame, the fork mode every template has; a frame-path knob was weighed and refused, since its only reader would be the sentence that cites the file.

Replacement text for the frame. **Not yet applied.**

```text
You are reading a project on behalf of a delegating session that will check your work. You are in a clone of the repository at a committed revision: uncommitted and ignored content is absent from it.

**You write nothing**, and you run only commands that change nothing. Where your sandbox refuses a command, name it in your report. Never work around it.

**Critique the project's strategic position, not its defects.** Read its front door, its stated direction and its work queue, then the tree behind them. Report:

- what the project claims and where the tree does not back the claim;
- what a prospective adopter meets first, and where they would stop;
- what the project depends on that it does not control;
- what its stated direction needs that it has left unbuilt.

**Your report**, as your final output, in these parts and this order:

1. **Findings** — each with the path or command that shows it, and whether you measured it or inferred it.
2. **Not assessable from the tree** — what you would need that the clone does not carry.
3. **First three** — the findings you would act on first, and why.

A later message may ask you to clarify a finding. Answer from the tree, and say where an answer is inference.
```

### (3) `LIFECYCLE_KIT_CRITIQUE_ADAPTER` names the adapter {design-bearing} {user-facing: operator direction 2026-10-08 on the queue entry — the executor and its model are named by the operator, and the entry's deliverable keeps model literals in consumer config}

A scalar knob holding one adapter name from `DELEGATION_KIT_FOREIGN_ADAPTERS`; default empty, which is off: the consultation offers nothing, and an operator may still name an adapter in the session. The adapter's argv carries the vendor's program, model and effort, so this knob holds a name and no model. The table validator refuses a value outside `[a-z0-9-]` and does not read delegation-kit's table, on the ground the stage-executor knob's validator takes (lifecycle-kit/SPEC.md §Layout and configuration, `LIFECYCLE_KIT_STAGE_EXECUTOR`). Its readers are the validator and the consultation, through `--emit knob-values`, the read `DELEGATION_KIT_TIER_MODEL` has. Like that binding it usually lives in the gitignored overlay beside the adapters it names.

**Where the stage-executor amendment does not land in the same iteration**, this knob's validator sentence states its own ground, a kit's validator reading its own table, in place of the citation.

### (4) §templates/consult.md states the step, its grounds and its limits {mechanical}

The section gains what deltas 1, 2 and 3 rule, re-phrased into its existing paragraphs: the trigger and why a dispatched consultation runs none; the one-process ruling and the refused saved-report store; the landing through steps 3 and 4; the privacy rule. Its **Honest limits**:

- **Nothing verifies the critic's class.** An adapter binds to no tier class (delegation-kit/SPEC.md §The foreign-vendor run), so the consultation's own floor is the only tier check on what lands.
- **A follow-up needs a resumable adapter.** On a one-shot adapter the critique is one turn, and a question becomes a new run that has forgotten the first.
- **The critic reads committed, tracked state.** A strategic fact the consumer keeps outside the tree is invisible to it, so its report is one input to the assessment and never the assessment.
- **The step is prose.** No gate reads a consultation, as none reads a request (§templates/lead.md).

The count of consumer slots stays two; the step binds through a knob, which reds no adopter's shim.

### (5) This repo binds its expert adapter, in the overlay {mechanical}

On the build host, in the gitignored overlays: `LIFECYCLE_KIT_CRITIQUE_ADAPTER` set to the expert adapter the delegation overlay already configures at the operator-named model and medium effort, and a resume form for an expert adapter that keeps its vendor session, since the configured one opens ephemeral and so cannot be questioned. Nothing tracked changes and no foreign turn is spawned: the witness is `--emit knob-values LIFECYCLE_KIT_CRITIQUE_ADAPTER` printing the name and `--foreign-run <adapter> --budget` returning a keyed verdict rather than its unknown-adapter refusal. The build session reports the lines it wrote, since no commit carries them.

This repo's assessment has a home already, the private brief its consult binding lists under entry-reading, so the binding's slots are not edited.

## Producers and consumers

- **The critique step** — producer: an operator's request in a consultation they started. Consumer: that consultation. No deployed session reaches it unasked, by delta 1.
- **The frame** — producer: this amendment. Consumer: the consultation, which copies it into a prompt file; then the foreign agent, on standard input. No gate reads its wording.
- **`LIFECYCLE_KIT_CRITIQUE_ADAPTER`** — producer: a consumer's knob file or overlay; this repo's overlay sets it (delta 5). Consumers: the table validator, and the consultation through `--emit knob-values`. Its one field is the adapter name `--foreign-run` takes.
- **The run's report and session** — existing artifacts of the executor, with the dispatcher their one writer; the consultation is that dispatcher and closes the session.
- **What lands** — existing landing surfaces, through steps 3 and 4. No new state, tag or file is minted for a critique's findings.
- **Roster-holding readers of the minted knob name** — the crate's knob table and rendered roster, the kit's knob template, the kit SPEC's knob list and the release declarations, each an update target below.
- **A narrowed corpus** — none. **An obligation on every member of a corpus** — none.

## Existing sections updated

Produced by `git grep -n -l 'templates/consult.md\|frames/stage-contract' -- ':!docs/' ':!TASK-QUEUE.md'` and a read of `.claude/commands/consult.md`.

- `lifecycle-kit/SPEC.md` §templates/consult.md (deltas 1, 2 and 4); §Layout and configuration — the knob bullet, the table validator's sentence and the layout listing's frames line (deltas 2 and 3).
- `lifecycle-kit/templates/consult.md` (delta 1); `lifecycle-kit/templates/frames/project-critique.md`, new (delta 2); `lifecycle-kit/templates/lifecycle-config.knobs` and `lifecycle-kit/README.md` where it lists knobs or templates (deltas 2 and 3).
- `native/src/knobs/lifecycle_kit.rs` — the row and the validator's refusal, with its case (delta 3).
- `delegation-kit/SPEC.md` §The foreign-vendor run — its opening sentence names a consultation's critique beside the audit, the sweep and the stage reading as a unit the executor runs (delta 1).
- `.workflow/release-declarations.md` — a row for the knob, the frame and the template step (deltas 1, 2 and 3).
- `.workflow/surface-ceiling.txt` — the consult template's row re-stamped with `--emit always-loaded --ceiling` in the commit that grows it, and a row for the new frame where the ratchet governs it (deltas 1 and 2).
- `docs/lifecycle-kit/`, `docs/delegation-kit/` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).
<!-- update-target-exempt: the grep returns these three and none changes: the binding's entry-reading slot already lists the surface holding the assessment, the dispatched-consultation definition cites the template's dispatched mode, which delta 1 rewrites at its owner, and the plugin skill is a one-line pointer at the template -->
- `.claude/commands/consult.md`, `.claude/agents/consult-session.md`, `plugin/skills/consult/SKILL.md` — read and unchanged.

## Retired spellings

- None — no delta retires a name; every delta adds a step, a file or a knob.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **No foreign run is owed, and none is granted.** Acceptance is the validator's case, the frame handed to an audit-mode run under the crate's stub adapter with one stubbed resume turn, and delta 5's two witnesses. A live critique is the operator's to trigger from a consultation.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`), which the iteration's last lifecycle-kit batch discharges.
- [ ] **Queue entry done** — `--queue done foreign-project-critique` in the merge commit, before the stage that drains the queue.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
