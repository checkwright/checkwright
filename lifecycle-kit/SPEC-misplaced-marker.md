# SPEC amendment: misplaced-marker

§check-stage-entry assertion D reads an inferred marker only where its full bold spelling opens a physical line (`inferred_marker` in `native/src/gates/stage_entry.rs`). A marker an author writes mid-line, or whose spelling a hard wrap splits across two lines, is therefore not read at all, and its unrun claim passes the build entry. Both attested instances were mid-line, in one amendment. They were found only because a third, well-formed marker in the same file redded `--simulate build`.

**The ruling: a bold marker spelling D does not read as a marker is a misplaced marker, and D refuses it.** §templates/stages/ rules a mid-line mention to be prose, and the entry named the cost of narrowing that: prose mentions in an amendment would red. The narrowing is taken anyway, because a mention has a spelling that stays prose and a marker has none. A mention in a code span (`` `**Inferred, not run:**` ``) is prose, and that form is what every tracked mention already uses. An unbackticked bold spelling is what a marker looks like, so it is either a real marker in the wrong place or a mention written as one. Neither can be told apart from a misplaced marker, and both are fixed by one edit at the site. The line-start rule is unchanged: where a well-formed marker is read today, it is read the same way.

**Measured at authoring (2026-09-22).** `git grep -n -E` for either bold spelling outside `docs/`, less the lines where it opens the line or sits in a code span, returns only Rust and shell source: `native/src/gates/stage_entry.rs` and `lifecycle-kit/gate-tests/check-stage-entry.test.sh`, which D does not read. Every markdown mention already takes one of the two lawful forms. The one verdict that flips is a sandbox specimen the test writes: case D4 builds an amendment holding `prose naming **Inferred, not run:** mid-line` and expects it clean. No amendment was on disk before this iteration's set, and the set's amendments and promoted entries write every spelling in a code span.

## What changes

### (1) §check-stage-entry: assertion D refuses a misplaced marker {design-bearing}

**Not yet applied.** In the paragraph beginning "Assertion D reads C's amendment walk", after the sentence ending "empty when that separator is absent or nothing follows it.", add:

> A **misplaced marker** is refused as well: either bold spelling standing outside a code span where it does not open its line, or split across a line break, which is where a hard wrap puts it. The split is found by reading each line joined to the next with one space, so a consumer that wraps gets the same answer as one that does not. The fence rule applies unchanged. A mention of a spelling is written in a code span, and a misplaced one is fixed by moving the marker to open its own line or by backticking the mention.

Then in the **Red** sentence, "prints one `<file>:<line>: <marker line>` per marker" becomes "prints one `<file>:<line>: <marker line>` per unrun marker and one `<file>:<line>: misplaced marker: <line>` per misplaced one", and the help line names both remedies.

### (2) §templates/stages/: a bold spelling is a marker or a misplaced one {design-bearing}

**Not yet applied.** Replace "A marker is recognized only at the start of a line, after optional indentation and one optional `- ` or `> ` lead, and never inside a fence, so a mention of the spelling in running prose or in backticks is not a marker." with:

> A marker opens its line, after optional indentation and one optional `- ` or `> ` lead, outside a fence. A mention of a spelling is written in a code span. The bold spelling anywhere else is a misplaced marker, which §check-stage-entry assertion D refuses, because nothing tells it apart from a marker written in the wrong place.

### (3) The gate module reads misplaced spellings {mechanical}

**Not yet applied.** `native/src/gates/stage_entry.rs`: `scan_markers` also returns the misplaced markers, and assertion D reds on them beside the unrun ones. On each in-scope line outside a fence where `inferred_marker` returns `None`, an occurrence of `NOT_RUN` or `CANNOT_RUN` that falls outside a code span is misplaced. A code span is a backtick run and the next run of the same length on that line. An occurrence that starts on a line and ends on the next, found by searching the line, one space and the next line, is misplaced at the first line when the next line is in scope and outside a fence. `inferred_marker` and its unit test `a_marker_is_read_only_at_line_start` keep their verdicts. New unit tests: a mid-line bold spelling is misplaced; a backticked mid-line spelling is not; a spelling split across two lines is misplaced at the first; a line-start marker is read as today and is not also counted as misplaced.

### (4) The scenario test pins the new verdicts {mechanical}

**Not yet applied.** `lifecycle-kit/gate-tests/check-stage-entry.test.sh`: case D4 drops its `prose naming **Inferred, not run:** mid-line` line and gains a backticked mid-line mention, so it stays the fence, prose and stub not-a-marker case. A new bad case carries a mid-line bold spelling and a spelling split across two lines, expecting exit 1 and the `misplaced marker:` line for each.

### (5) The pending release declaration names the misplaced refusal {mechanical}

**Not yet applied.** Assertion D is unreleased, so its pending `Tightened gates` bullets in `.workflow/release-declarations.md` are corrected in place: the bullet introducing assertion D adds "and a bold marker spelling standing mid-line or split across a line break, refused as a misplaced marker; write a mention in a code span".

## Producers and consumers

- **The misplaced-marker finding** (deltas 1 and 3). Produced by assertion D at the audit-entry stage's entry, wherever `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` is set, which the kit default does (`build`). It reads the corpus D already reads, so no new knob or `--reads` root. Its consumer is the session whose `--enter-stage` is refused, through the red line and help, and the lead's `--simulate` read, which relays the same verdict.
- **Point 5.** This widens what D reds on and narrows nothing, so no reader loses a finding. Its only corpus is D's own.
- **Point 6.** The corpus is enumerable. The member with a changing value is test case D4, named in delta 4. Every other current spelling is already in a code span or opens its line (measured above).

## Existing sections updated

Roster from `git grep -n -F "Inferred, not run"` and `git grep -n -F "Inferred, cannot run"`, run 2026-09-22, with `docs/` and `TASK-QUEUE.md` excluded.

- `lifecycle-kit/SPEC.md` §check-stage-entry, the assertion D reading paragraph and its Red sentence (delta 1).
- `lifecycle-kit/SPEC.md` §templates/stages/, the marker-recognition sentence (delta 2).
- `native/src/gates/stage_entry.rs`, `scan_markers`, assertion D's red and the unit tests (delta 3).
- `lifecycle-kit/gate-tests/check-stage-entry.test.sh`, cases D4 and the new misplaced case (delta 4).
- `.workflow/release-declarations.md`, the pending assertion D bullet (delta 5).

## Retired spellings

- None — no delta retires a name; the marker spellings keep their tokens.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the misplaced-marker finding.
- [ ] **Instruction surfaces: instruction only.** Not reached; no stage template changes.
- [ ] **Merged with no information lost.** The recognition sentence in §templates/stages/ is replaced, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `inferred-marker-malformed-placement-passes-unseen` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
