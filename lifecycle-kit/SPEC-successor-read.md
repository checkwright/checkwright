# SPEC amendment: successor-read

Every stage template ends on the same step, the resume journal's `DONE` append (`lifecycle-kit/templates/stages/*.md`; all six carry the paragraph byte-identical). Nothing a stage session does reads whether its **successor** may enter. A refusal of that entry therefore surfaces first at the lead's `--enter-stage --dispatch <next>` read, after the session has reported, and where no lead runs it surfaces at the successor's own entry. At gate-sdk-blind-spots, align reported done over a marker that `--enter-stage --simulate build` refused, and align was resumed to fix it.

**A premise correction.** The entry reads lifecycle-kit/templates/lead.md's "The repair that reaches every stage session on every path is the stage template's own last step" as naming a step no template has. In context the sentence repairs one refusal class, the predecessor's missing resume journal, and the journal step does repair that (lifecycle-kit/SPEC.md §The state machine, "The repair is the stage template's **last step**"). The sentence is false only when it is read as covering everything the dispatch read catches: an unrun marker, an undrained queue, a live producer, a boundary precondition. So the repair is to widen the step and the sentence together.

**The ruling: each stage template reads its successor's entry with `--simulate`, after its last commit and before the journal's `DONE`.** The read already exists and writes nothing. It runs the successor's `check-stage-entry` and every matching pre-flight against the tree the session leaves (lifecycle-kit/SPEC.md §bin/enter-stage.sh). What was missing was the instruction on the surface the owing session loads. The successor is named per template, because only the template knows its stage's relation to the next one:

- **Scope and the authoring stage** read the stage they recommend. That stage may be trigger-gated, so the recommendation is the stated answer for which stage comes next.
- **The audit stage** reads the build stage.
- **The build stage** reads the drain stage only from a batch whose Done moves emptied the active sections. A batch that leaves work active would be refused by assertion B on purpose, and its successor is the next batch's re-entry of the build stage.
- **The drain stage** reads the close stage.
- **The close stage** reads the roster's first stage, whose iteration-boundary checks read what close leaves.

**Measured at authoring (2026-09-23).**

- `grep -h "^\*\*Last step" lifecycle-kit/templates/stages/*.md | sort | uniq -c` prints one line, count 6.
- `native/target/release/checkwright-gates --enter-stage --simulate build` exited 0 with `git status --porcelain` empty before and after.
- `native/src/emit/enter_stage.rs:671-672` reads the working-tree queue and state, and `head_of` (`:1989-1995`) reads `HEAD` only for the candidate stamp's `<head>`. So the read's verdict describes uncommitted edits too, and it belongs after the session's final commit.
- With `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=1`, which this repo binds, the simulated entry also asserts that the cursor's journal is written (lifecycle-kit/SPEC.md §bin/enter-stage.sh). The cursor is the reading session's own stage, so that assertion reads the session's own journal. It asserts non-emptiness and never `DONE`, which is why the read goes before the `DONE` append.

## What changes

### (1) Each stage template gains the successor read {design-bearing}

**Not yet applied.** In each of the six templates, a new paragraph goes immediately before the unchanged `**Last step — the resume journal.**` paragraph. Its text:

> **Then read your successor's entry.** After your last commit, run `--enter-stage --simulate <stage>` — the lifecycle arm your first step ran — for *<successor>*. It runs that stage's entry checks against the tree you leave and writes nothing (lifecycle-kit/SPEC.md §bin/enter-stage.sh). An exit of 1 relays a refusal: fix one your own work caused and run the read again, and carry any other into your report verbatim.

`<successor>` per template:

- `scope.md` and `spec.md`: "the stage you recommended".
- `align.md`: "the build stage".
- `build.md`: "the drain stage, when your batch's Done moves emptied the active sections — a batch that leaves work active skips the read, since that entry refuses on active work by design and the next batch re-enters this stage".
- `validate.md`: "the close stage".
- `close.md`: "the roster's first stage, whose iteration-boundary checks read what this close leaves".

The journal paragraph stays byte-identical and last, so `check-stage-skill-coverage`'s third direction still finds its citation. The paragraph binds no slot, so no consumer shim changes.

### (2) The lead's backstop sentence covers the read {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md`, "The repair that reaches every stage session on every path is the stage template's own last step (lifecycle-kit/SPEC.md §templates/stages/); this is the backstop." becomes:

> The repair that reaches every stage session on every path is the stage template's own close — its successor read and its journal (lifecycle-kit/SPEC.md §templates/stages/); this is the backstop.

### (3) The contract text {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §templates/stages/:

- In the opening roster sentence, "and the **resume-journal last step**" becomes "the **successor read** and the **resume-journal last step**".
- After the paragraph opening "**The step instructs a marker the entry does not assert**", add:

> **The successor read precedes the last step, on the same residency ground.** A refusal of the next stage's entry is found by the session whose work caused it only if that session reads the entry itself; the lead's dispatch read finds it one report later, and without a lead nothing does until the next session enters. The read is `--simulate` because it writes nothing and runs the entry's own checks, so no second detector is minted. It runs after the last commit because it reads the working tree, and before the `DONE` append because the journal it asserts is the reading session's own and `DONE` must stay the file's last line. Each template names its successor, the one fact only it knows: a trigger-gated successor is the one the session recommended, and a build batch that leaves work active has no successor to read.

- In the Definition-of-Done placement paragraph, "the lead's pre-completion `--enter-stage --simulate <next stage>` read (§templates/lead.md) refuses exactly this case, provided it runs before the push" becomes "the build stage's successor read refuses exactly this case at the batch that drains the queue, and the lead's pre-completion `--enter-stage --simulate <next stage>` read (§templates/lead.md) is its backstop, provided it runs before the push".

## Producers and consumers

- **The successor read.**
  - Producer: each stage session at its close, under a lead or without one.
  - Consumers: the session itself, which fixes its own refusal, and the session's report, which carries any other. The read writes nothing, so no state surface reads it.
  - Enabling config: none. `--simulate` is unconditional, and the journal half of the read is live wherever `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=1`.
- **Roster-holding readers.**
  - `check-stage-skill-coverage`'s third direction holds each executed template to its journal citation. Delta 1 leaves that paragraph untouched.
  - `check-shim-restatement` reads the kit templates as its corpus against the consumer shims. The shims bind slots only, and the new paragraph has no slot.
  - `check-surface-ratchet` and `check-footprint-fresh` measure template growth. Their ceiling file and the footprint page regenerate in the landing commit.
- **Point 5.** No corpus narrows.
- **Point 6.** The obliged corpus is the six stage templates. Each one's successor clause is named in delta 1.

## Existing sections updated

Roster from `grep -n "Last step" lifecycle-kit/templates/stages/*.md`, `grep -n "stage template's own last step\|pre-completion" lifecycle-kit/templates/lead.md lifecycle-kit/SPEC.md` and `grep -n "resume-journal last step" lifecycle-kit/SPEC.md`, run 2026-09-23.

- `lifecycle-kit/templates/stages/scope.md` (delta 1).
- `lifecycle-kit/templates/stages/spec.md` (delta 1).
- `lifecycle-kit/templates/stages/align.md` (delta 1).
- `lifecycle-kit/templates/stages/build.md` (delta 1).
- `lifecycle-kit/templates/stages/validate.md` (delta 1).
- `lifecycle-kit/templates/stages/close.md` (delta 1).
- `lifecycle-kit/templates/lead.md` (delta 2).
- `lifecycle-kit/SPEC.md` §templates/stages/ (delta 3).
- `.workflow/surface-ceiling.txt` and `docs/footprint.md`, regenerated by their gates' printed commands (all deltas).
- `.workflow/release-declarations.md`, one Behavior changes bullet (all deltas): every stage template now reads its successor's entry with `--enter-stage --simulate` before its journal step, so a refusal reaches the session that caused it.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/lifecycle-kit/SPEC.md`.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the successor read.
- [ ] **Instruction surfaces: instruction only.** The six template paragraphs and the lead sentence carry the act and its successor; the grounds sit in delta 3's SPEC paragraph.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `stage-exit-no-successor-check` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
