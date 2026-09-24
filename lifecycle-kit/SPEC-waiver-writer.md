# SPEC amendment: waiver-writer

lifecycle-kit/SPEC.md §check-stage-entry (C) and the align template record an operator-ruled audit waiver as a `<iter> <waiver-token> <session> <date> <head>` line in the state file, and nothing may write one. The workflow-state guard blocks every Write/Edit of that file (`native/src/hook/workflow_state.rs`). The shell guard blocks a shell append to it as a redirect into a tracked file. The `--rewrite` arm refuses it (`native/src/emit/rewrite.rs`, through `is_state_file`). `--enter-stage` has no waiver form. Only an operator-run shell append lands one today. The piggyback that worked in July, a build session carrying the line into its own commit, is now guard-blocked for every session. And assertion C's `help:` line names `k.state`, which under `--enter-stage` is the scratch candidate state, deleted when the run ends. So the one recovery the refusal offers points at a file that is gone.

**The ruling: the dispatcher declares the waiver, and the entering session's own entry writes it.** `--enter-stage --dispatch <stage> --waive <reason>` puts the declaration on the dispatch marker. The entry that consumes that marker line writes the waiver line with its stamp, in one append. The three options the queue entry records were weighed.

- **(c), the dispatch channel, is taken.** The dispatcher is the party that holds the operator's words. Under the split posture that is the lead, relaying a direction, and without a lead it is the session the operator instructs. `--dispatch` is already the dispatcher's sanctioned, non-stamping act. The marker is already scratch, and the lead template's "writes no lifecycle state" still holds. `--enter-stage` stays the state file's one writer, so the workflow-state guard's one-writer message stays true. And the entering session is structurally the recorder and never the issuer: the entry form takes no waiver of its own.
- **(b), a waiver operand on the entry itself, is refused.** It would make the entering session the issuer, which is the self-issuing assertion C's text forbids, in the one form no reader could tell apart from a self-grant.
- **(a), a pending waiver carried into the entry commit, is refused.** It names a carrier and no writer. The line still has to reach the file past the guards, which is the gap itself.

**Measured at authoring (2026-09-24):**

- **The blocked writers.** `workflow_state.rs` blocks Write/Edit on the state file. `rewrite.rs:420` calls `workflow_state::is_state_file` and refuses. `native/src/emit/enter_stage.rs` has no waiver operand (`grep -n waive native/src/emit/enter_stage.rs` returns nothing).
- **The scratch path in the help lines.** `native/src/gates/stage_entry.rs:628` and `:630` print `k.state`. `knob_or(args, 1, …)` reads that from the positional, and `--enter-stage` passes the candidate temp state there.
- **The marker's line helpers.** `stages::marker_lines` and `stages::marker_without` compare a whole trimmed line to a stage name (`native/src/stages.rs:142`, `:148`), so a longer line would be read as no stage at all.

## What changes

### (1) `--dispatch <stage> --waive <reason>` declares a waiver on the marker {design-bearing}

**Not yet applied.** In lifecycle-kit/SPEC.md §bin/enter-stage.sh, the `--dispatch` paragraph gains, after its first sentence:

> **`--dispatch <stage> --waive <reason…>` also declares an audit waiver the user ruled.** The arguments after `--waive` are joined with single spaces into the reason, which is mandatory. The pre-flight simulate then runs against a candidate state carrying the waiver line ahead of the candidate stamp. So assertion C reads the declared waiver, and every other assertion runs exactly as a plain dispatch runs it. On a clear verdict the marker line is `<stage> <waiver-token> <reason>`. The form is refused (exit 2, nothing written) in four cases: `<stage>` is not `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE`, the stage whose entry assertion C gates; the waiver token is empty, as it is on a roster with no audit stage; the reason is empty; or the reason spans a line break. The arm cannot tell who ran it, the same honest limit the pre-flight valve states. A session can declare its own waiver. What the form buys is that the declaring act is separate from the entry, named in the marker, and carried with its reason into the stamp commit.

The grammar sentence of §bin/enter-stage.sh's `Arm::Run` paragraph gains the form: `--enter-stage --dispatch <stage> [--waive <reason…>]`. The surplus-argument paragraph's "`--dispatch` and `--dispatch-withdraw` exactly one `<stage>`" becomes "`--dispatch` exactly one `<stage>`, optionally followed by `--waive` and its reason, and `--dispatch-withdraw` exactly one `<stage>`".

### (2) The entry that consumes a waiver-bearing line writes the waiver with its stamp {design-bearing}

**Not yet applied.** In the same paragraph, the discharge sentence "Every stamp this tool writes, the boundary reset's included, removes one line naming the stamped stage and deletes the file once it is empty, and its report notes the discharge." becomes:

> A marker line names its stage in its first field. Every stamp this tool writes, the boundary reset's included, removes one line naming the stamped stage, preferring a waiver-bearing line to a bare one, and deletes the file once it is empty. Its report notes the discharge. Where the removed line bears a waiver and the state file carries no waiver line for the current iteration, the entry writes `<iter> <waiver-token> <id> <date> <head>` immediately before its stamp, in the same append. `<id>` is the entering session's, the recorder's. The entry's own pre-flight candidate carries both lines. The report prints the waiver with its reason, and a `body:` line after `subject:` that carries the reason into the stamp commit's body, since the state line has five fields and no room for one. A refused entry writes neither line and leaves the marker as it was. A waiver line is not a stamp, so the cursor, `check-stamp-subject` and the stamp-commit purity assertion read the entry exactly as they read a plain one.

The marker paragraph's "The marker is lifecycle-kit's scratch, like the lead journal, and never lifecycle state: no stage reads it for the cursor, and the gate below reads it for one commit-time question." becomes:

> The marker is lifecycle-kit's scratch, like the lead journal, and never lifecycle state. No stage reads it for the cursor. The entry reads a waiver-bearing line as the dispatcher's declaration and records it, and the gate below reads it for one commit-time question.

`stages::marker_lines` and `stages::marker_without` read a line's first field as its stage, and `marker_without` takes the preference above. `--dispatch-withdraw <stage>` removes one line naming `<stage>`, whichever it is.

### (3) Assertion C's recovery names the waiver form and the real state file {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §check-stage-entry, the waiver sentence "The waiver rides the same file the stamps do (auditable) and is written only on an explicit user ruling — never self-issued by the entering session;" becomes:

> The waiver rides the same file the stamps do (auditable) and is written only on an explicit user ruling. The dispatcher declares it with `--enter-stage --dispatch <stage> --waive <reason>`, and the entry consuming that declaration writes it (§bin/enter-stage.sh). The entering session never issues one;

In `native/src/gates/stage_entry.rs`, every human-facing line that names the state file names the configured `LIFECYCLE_KIT_STATE_FILE` and never the positional. The positional is the file read, which under `--enter-stage` is a scratch candidate the run deletes. The finding lines and the assertion-A/B `help:` line are both covered. Assertion C's `help:` line names the waiver form as the recovery, run by the dispatcher on an explicit user ruling.

### (4) The align and lead templates carry the act {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/align.md`, the sentence from "To skip the audit anyway," through "(lifecycle-kit/SPEC.md §check-stage-entry)." becomes:

> To skip the audit anyway, the user must explicitly rule it unwarranted. The dispatcher then declares the waiver with `--enter-stage --dispatch <stage> --waive <the ruling>`, and the build session's own entry records it. The entering build session never issues it (lifecycle-kit/SPEC.md §check-stage-entry).

In `lifecycle-kit/templates/lead.md` §The lead model, after the paragraph ending "A dispatched session that ends without entering is withdrawn with `--dispatch-withdraw <stage>`.":

> Where the operator rules the audit unwarranted, dispatch the gated stage with `--dispatch <stage> --waive <the ruling>`: the entering session records it, and the dispatch prompt relays it as the operator's direction.

## Producers and consumers

- **The waiver declaration (delta 1).**
  - Producer: `--dispatch <stage> --waive`, run by the dispatcher. That is the lead under either posture, or, with no lead, the session the operator instructs.
  - Consumer: the entry of `<stage>`, by reading the marker file (delta 2).
  - Fields: `<stage>` is read by the discharge match, `check-dispatch-entry`'s outstanding list and `--dispatch-withdraw`. `<waiver-token>` is read by the entry to tell a waiver-bearing line from a bare one. `<reason>` is read by the entry's report and `body:` line.
- **The waiver line (delta 2).**
  - Producer: `--enter-stage <stage>`, on consuming a waiver-bearing line.
  - Consumers, unchanged: assertion C, which reads any line whose second field is the waiver token for the iteration, and `check-stage-evidence`'s grammar, which already admits the token.
  - `check-stamp-subject` and `check-dispatch-entry` read added *stamps* through one shared function whose roster filter skips a waiver line, so the stamp commit's subject scope stays the stamp's stage.
- **Roster-holding readers of the marker (point 2).** `check-dispatch-entry`, through `stages::marker_lines`: it names outstanding stages and now reads the first field. `native/src/emit/enter_stage.rs`'s discharge, through `stages::marker_without`. `gate-tests/dispatch-marker.test.sh` gains the waiver cases: declaration, refusals, consumption, preference, withdraw, and a refused entry leaving the marker whole.
- **Point 5.** No corpus narrows.
- **Point 6.** Not obliged.
- **Sibling unit.** The stamp-before-write rule lifecycle-kit adds to the workflow-state guard this iteration does not read the marker, so a waiver-bearing line changes nothing for it. The same act holds whether or not that unit lands in the same build batch.

## Existing sections updated

The roster comes from `git grep -n "waiv\|marker_lines\|marker_without\|dispatch-withdraw" -- '*.md' '*.rs' lifecycle-kit/gate-tests`, run 2026-09-24.

- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh (deltas 1 and 2).
- `lifecycle-kit/SPEC.md` §check-stage-entry (delta 3).
- `lifecycle-kit/SPEC.md` §Layout and configuration, the `LIFECYCLE_KIT_DISPATCH_MARKER_FILE` and `LIFECYCLE_KIT_WAIVER_TOKEN` bullets: the marker is read by the entry too, and the token's writer is the entry (deltas 1 and 2).
- `native/src/emit/enter_stage.rs`, the `--dispatch` form, the discharge and the report (deltas 1 and 2).
- `native/src/stages.rs`, the marker helpers and their unit test (delta 2).
- `native/src/gates/stage_entry.rs`, the state-file lines and C's `help:` (delta 3).
- `lifecycle-kit/gate-tests/dispatch-marker.test.sh` (deltas 1 and 2).
- `lifecycle-kit/gate-tests/check-stage-entry.test.sh`, if a case pins the help text (delta 3).
- `lifecycle-kit/templates/stages/align.md` (delta 4).
- `lifecycle-kit/templates/lead.md` (delta 4).
- `.workflow/surface-ceiling.txt` and `docs/footprint.md`, regenerated for the templates' growth by their gates' printed commands (delta 4).
- `.workflow/release-declarations.md` (deltas 1 and 2). Under Behavior changes: an operator-ruled audit waiver is declared with `--enter-stage --dispatch <stage> --waive <reason>` and written by the entering stage's own entry.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/lifecycle-kit/SPEC.md`.

## Retired spellings

- None — the deltas add a form and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the declaration and the waiver line.
- [ ] **Instruction surfaces: instruction only.** The align and lead template edits carry the act, and their grounds sit in the SPEC text of deltas 1 to 3.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines, and the refused options' grounds move into §bin/enter-stage.sh.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `split-posture-waiver-writer` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
