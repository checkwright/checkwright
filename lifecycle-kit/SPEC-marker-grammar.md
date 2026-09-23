# SPEC amendment: marker-grammar

`check-stage-entry` assertion D does two jobs, and only one of them belongs where it runs. A misspelt marker is a **grammar** defect: a cannot-run marker with no ` — <reason>`, or a bold spelling that does not open its line. The authoring commit could refuse it. A well-formed not-run marker is **residue**: legitimate until the audit-entry stage, and a refusal only there. Today both are read only when the cursor enters `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` (`native/src/gates/stage_entry.rs:503`, `at_audit_entry`). So a spelling defect travels through every commit between the stage that wrote it and build entry.

**The ruling: split D inside `check-stage-entry`.** A new assertion (E) holds the marker grammar whatever the cursor. D keeps the residue alone, at audit-entry. No new gate is minted. The gate already fires on every commit that stages an amendment or the queue (its `couples=` carries `*/SPEC-*.md`, `*SPEC-*.md` and the queue knob), and `--enter-stage` runs it as its pre-flight at every stage. Only the assertion was gated by the cursor. A separate gate is justified by a trigger or dependency the host lacks (gate-sdk/SPEC.md §check-gate-binary-fresh's split test), and neither is lacking here.

E also holds the **not-run form's mandatory command**. §templates/stages/ writes it as mandatory (`**Inferred, not run:** <claim> — <command>`), but `inferred_marker` matches that form by prefix only (`stage_entry.rs:154`), so nothing has ever read the command's presence. Holding the grammar means holding both forms' tails.

**Measured at authoring (2026-09-23).**

- **The attested defect.** 76fbc3e9 committed a cannot-run marker at line 50 of the worktree-hook-bin amendment with no spaced em dash. Align's commits 1185b7f4, 46446301 and 87b3e3d0 passed the battery over it. 2efdfdd5 fixed it after `--enter-stage --simulate build` refused.
- **Where D runs.** `stage_entry.rs:546-600` is D, all inside `if at_audit_entry`. `scan_markers` (`:246-280`) puts an empty-reason cannot-run marker and a not-run marker into one `unrun` bucket (`:261`), and misplaced spellings into `misplaced`.
- **The live corpus.** `git grep -n -e "Inferred, not run:\*\*" -e "Inferred, cannot run before build:\*\*" -- 'SPEC-*.md' '*/SPEC-*.md' TASK-QUEUE.md` finds two marker lines, both this iteration's well-formed cannot-run markers (in the discharge-span and policy-residue amendments), so E lands with nothing to fix.
- **Cost.** `time native/target/release/checkwright-gates --enter-stage --simulate build`, which walks the amendment tree, took 0.056s end to end. That walk now runs at every firing.

## What changes

### (1) Assertion E, the marker grammar, at every firing {design-bearing}

**Not yet applied.** In `native/src/gates/stage_entry.rs`:

- **The walk.** `live_tree()` runs whenever the gate fires, not only at audit-entry. Assertion C still reads it only at audit-entry.
- **The classifier.** `inferred_marker` returns the tail for both forms: the text after the last spaced em dash, trimmed, and empty when the separator is absent or nothing follows it. `Marker::NotRun` gains `command_empty` beside `CannotRun`'s `reason_empty`.
- **The split.** `scan_markers` returns four sets:
  - malformed markers, meaning either form with an empty tail;
  - misplaced spellings, as today;
  - well-formed not-run markers, the residue;
  - the reasoned cannot-run count.
- **The assertion.** `// assertion E:` runs over D's corpus, the amendment walk plus active queue entries, whatever the cursor and whether or not `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` is empty. It reds on every malformed and every misplaced marker. Each finding line is `<file>:<line>: malformed marker: <line>` or `<file>:<line>: misplaced marker: <line>`. Its help lines name the fix at the site: write the missing ` — <command>` or ` — <reason>`, or move the marker so it opens its line, or put a mention of the spelling in a code span.

E reports before D. A marker is one finding: a malformed not-run marker is E's and never also D's residue.

### (2) Assertion D narrowed to residue {mechanical}

**Not yet applied.** D keeps its cursor gate and its corpus. It reds only on well-formed not-run markers. The misplaced branch and the empty-reason branch move to E. D's message drops "or a cannot-run marker with no reason", and its misplaced help line moves to E. The clean detail's `N cannot-run claim(s) carried` stays D's, printed at audit-entry.

### (3) The contract text {mechanical}

**Not yet applied.**

In lifecycle-kit/SPEC.md §check-stage-entry, the invariant paragraph's "It owns four assertions, (A)" becomes "It owns five assertions, (A)". Its closing clause, "and (D) inferred-claim residue — at audit-entry-stage entry, any on-disk amendment line or active queue-entry line carrying an `**Inferred, not run:**` marker, or a cannot-run marker whose reason is empty, is a refusal (the marker grammar: §templates/stages/)." becomes:

> (D) inferred-claim residue — at audit-entry-stage entry, any on-disk amendment line or active queue-entry line carrying a well-formed `**Inferred, not run:**` marker is a refusal; and (E) marker grammar — whatever the cursor, a marker in the same corpus whose command or reason is empty, and a misplaced spelling, is a refusal (the marker grammar: §templates/stages/).

In the paragraph that opens "**Assertion D reads C's amendment walk and B's queue read, and nothing else.**":

- The opening becomes "**Assertions D and E read C's amendment walk and B's queue read, and nothing else.**" The walk runs at every firing, since E is not cursor-gated.
- "a cannot-run marker's reason is the text after its last spaced em dash, empty when that separator is absent or nothing follows it" becomes "either marker's tail — the not-run form's command, the cannot-run form's reason — is the text after its last spaced em dash, empty when that separator is absent or nothing follows it".
- "A **misplaced marker** is refused as well" becomes "E refuses a **malformed** marker, one whose tail is empty, and a **misplaced marker**".
- The **Red** sentence names D's `<file>:<line>: <marker line>` per unrun marker and E's `malformed marker:` and `misplaced marker:` lines. The remedy for a grammar finding is made at the site, in the commit the finding refuses.
- "**Inert** where `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` is empty. **The honest limits:** a roster with no audit stage gets the template obligations without this backstop" becomes:

> D is **inert** where `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` is empty; E is not. **The honest limits:** a roster with no audit stage has its marker grammar held and its residue left to the template obligations

In the coverage paragraph, "`gate-tests/check-stage-entry.test.sh` covers B, C and D in sandbox scenarios" becomes "covers B, C, D and E in sandbox scenarios".

In lifecycle-kit/SPEC.md §templates/stages/, "The bold spelling anywhere else is a misplaced marker, which §check-stage-entry assertion D refuses" becomes "The bold spelling anywhere else is a misplaced marker, and a marker whose command or reason is empty is malformed; §check-stage-entry assertion E refuses both at the commit that writes them".

### (4) The scenarios {mechanical}

**Not yet applied.** In `lifecycle-kit/gate-tests/check-stage-entry.test.sh`, D3 and D7 move to E, rewritten under a spec-cursor sandbox. They are the empty-reason and misplaced cases, and a spec cursor proves they fire away from audit-entry. New cases:

- **E-not-run-no-command.** A not-run marker with no ` — <command>` at a spec cursor exits 1 with `malformed marker:`.
- **E-residue-not-due.** A well-formed not-run marker at a spec cursor exits 0, because the residue is not yet due.
- **E-at-audit-entry.** A malformed marker at build entry is reported once, as E, and not as D residue.

The header comment and the summary line's scenario count move with the cases. `native/src/gates/stage_entry.rs` gains unit cases for the tail parse of both forms.

## Producers and consumers

- **Assertion E.**
  - Producer: `check-stage-entry` at every firing: the generated pre-commit hook on a staged amendment, queue or state file, the battery, and `--enter-stage`'s pre-flight, `--simulate` and `--dispatch` included.
  - Consumers: the committing session, through the output contract; a stage entry, which refuses; and the lead's dispatch read, which relays it.
- **Roster-holding readers.** `check-gate-assertions` couples the invariant paragraph's count word and `(X)` labels to the `// assertion X:` markers in the implementation (gate-sdk/SPEC.md §check-gate-assertions). Delta 3's "five" and "(E)", and delta 1's `// assertion E:` marker, satisfy it. `native/target/release/checkwright-gates check-gate-assertions` reports 9 contracts coupled today. That `check-stage-entry` is among them is read off the discovery rule and is re-run at build.
- **Point 5.** No corpus narrows: E reads D's corpus at more cursors, which widens its reach. D's red condition narrows to well-formed not-run markers, and every marker it stops reading as residue, E reads as malformed. The union of red conditions at audit-entry grows by exactly the not-run marker without a command, which D reds today as residue.
- **Point 6.** The obliged corpus is every marker line in an amendment or an active entry. The probe above finds none, so no member is owed a value.

## Existing sections updated

Roster from `grep -n "assertion D\|four assertions\|covers B, C and D" lifecycle-kit/SPEC.md lifecycle-kit/README.md lifecycle-kit/templates/*.md lifecycle-kit/templates/stages/*.md` and `grep -n "assertion D" native/src/gates/stage_entry.rs lifecycle-kit/gate-tests/check-stage-entry.test.sh`, run 2026-09-23. The stage templates cite the marker grammar and never an assertion letter.

- `lifecycle-kit/SPEC.md` §check-stage-entry and §templates/stages/ (delta 3).
- `native/src/gates/stage_entry.rs` (deltas 1, 2 and 4).
- `lifecycle-kit/gate-tests/check-stage-entry.test.sh` (delta 4).
- `.workflow/release-declarations.md`, one Tightened gates bullet (delta 1): `check-stage-entry` refuses a malformed or misplaced inferred-claim marker at every commit and every stage entry, not only at the audit-entry stage, and a not-run marker with no ` — <command>` is now malformed.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/lifecycle-kit/SPEC.md`.

## Retired spellings

- None — no delta renames or deletes a spelling; D keeps its letter and E is a new one.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for assertion E.
- [ ] **Instruction surfaces: instruction only.** Not reached. No template text changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `cannot-run-marker-late-read` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
