# SPEC amendment: discharge-span

`check-unmarked-claim` discharges a class match anywhere in a paragraph that carries a full-line `measured:` marker (`native/src/gates/unmarked_claim.rs:71-77`). It does not read `CANON_KIT_MEASURED_SPAN`, and it does not read where in the paragraph the marker stands. `check-measured-claim` binds a full-line marker to a narrower claim. Its `collect` (`native/src/gates/measured_claim.rs:152-173`) opens a new block at the marker line, so the claim is the text *below* the marker. `block` (`:220-230`) then narrows that text to its first sentence under `sentence`. So the two gates disagree in two ways:

- **The span.** Under `sentence`, a class match in a later sentence reads as marked, though no marker binds it. This is the filed divergence, stated at canon-kit/SPEC.md §check-unmarked-claim.
- **The position**, found at authoring. A full-line marker in mid-paragraph discharges class matches *above* it at every span, including `paragraph`, although its claim begins at the marker. The same SPEC paragraph says "both gates delimit a marker's paragraph alike", and that is false today.

**The ruling: a full-line marker discharges the claim it binds, and nothing else.** The SPEC already defines the knob as setting "the claim a full-line marker binds" (§check-measured-claim), and arm A reds a match that "no `measured:` marker binds" (§check-unmarked-claim). The code and the one divergence sentence contradict those two definitions. So the discharge follows the definitions, and nothing new is ruled. gate-sdk/SPEC.md §Calibration lessons names "a marker to its claim" as one matching window, so one calibration governs it and two gates reading it two ways would be two windows. The inline rule already has this shape: "a marked figure does not vouch for an unmarked claim beside it".

**At `off`, the discharge is the block.** `off` stops arm C reading the full-line marker's cardinal. Arms A and B still hold the marker to its oracle, so the marker is still a live assertion over its block. An extent marker, the shape this tree's four live markers take, is never read by arm C at any span. So arm C's reach cannot be what decides the discharge; what the marker binds decides it, and at `off` that is the block.

**Measured at authoring (2026-09-23).**

- **The divergences.** A scratch-tree probe, run by the research dispatch under a case-local `CANON_KIT_KNOB_FILE` with one class and a `k⇥3` oracle, reproduced both:
  - Under `sentence`, a class match in sentence two passes both gates.
  - Under `paragraph`, a class match on the line above a mid-paragraph marker passes `check-unmarked-claim`, while `check-measured-claim` reds, because the marker's cardinal is absent from the claim it binds.
- **This tree.** `git grep -n -E "^\s*<!-- measured: " -- ':!*/gate-tests/*'` finds four live full-line markers, all `gate-substrates=native`: `CONTRIBUTING.md:29`, `SECURITY.md:23`, `docs/methodology.md:25` and `docs/positioning.md:26`. The other hits are the fenced specimen and its mirror. This repo binds `CANON_KIT_MEASURED_SPAN = sentence` (`scripts/canon-config.knobs`). Both gates run clean today.
- **Inferred, cannot run before build:** no verdict on this tree moves. The one class hit in a marked paragraph, `CONTRIBUTING.md:30`, falls in the first sentence the `CONTRIBUTING.md:29` marker heads, going by the sentence terminator rule. That is a reading of the rule, not a run of it — the discharge change is compiled code, so the run is the build's battery.

## What changes

### (1) One helper binds a full-line marker's claim for both gates {design-bearing}

**Not yet applied.** `native/src/spec.rs` gains the helper both gates call. Given a block's text below a full-line marker and the span, it returns the byte range of the claim. The block's text is marker text and the inline-bound sentences cut, and the result is:

- the block, at `paragraph` and at `off`;
- the block's first sentence, at `sentence`.

The helper takes the arm-C question apart from the discharge question. `measured_claim.rs` `block()` asks it for the claim text and keeps returning `None` for arm C at `off`. `unmarked_claim.rs` asks it for the discharge range.

### (2) The discharge walk follows the claim {design-bearing}

**Not yet applied.** In `native/src/gates/unmarked_claim.rs`, the whole-paragraph early return at `:71-77` goes. The paragraph splits at every full-line marker line, on `collect`'s rule, so each marker's block runs from the line below it to the next full-line marker or the paragraph's end. A class match is discharged when it starts inside a range the helper returns for a marker's block, or inside an inline marker's sentence as today. A class match above the first full-line marker is in no block and is not discharged. The member reads `CANON_KIT_MEASURED_SPAN` and declares it in its registry row in `native/src/gates/mod.rs`.

### (3) The contract text {mechanical}

**Not yet applied.**

In canon-kit/SPEC.md §check-unmarked-claim, the paragraph opening "**A paragraph is the unit, and it is §check-measured-claim's paragraph**" becomes:

> **A paragraph is the unit, and a marker discharges exactly the claim it binds** — §check-measured-claim's claim, read through one helper so the two gates cannot delimit it differently. A full-line marker binds the text below it up to the next full-line marker or the paragraph's end, narrowed by `CANON_KIT_MEASURED_SPAN`: its first sentence at `sentence`, the whole block at `paragraph` and at `off`, since `off` withdraws only arm C's cardinal read and the marker is still held to its oracle by arms A and B. A class match above a full-line marker, or past the sentence it binds, is unmarked. An inline marker discharges a class match that starts inside the sentence it binds, and no other, so a marked figure does not vouch for an unmarked claim beside it. Fenced blocks are skipped for the reason that section gives: a fence is grammar being shown, not a claim being made.

In canon-kit/SPEC.md §Layout and configuration, the `CANON_KIT_MEASURED_SPAN` bullet becomes:

> - `CANON_KIT_MEASURED_SPAN` — the claim a full-line `measured:` marker binds, which `check-measured-claim` arm C reads for its cardinal and `check-unmarked-claim` discharges: `paragraph`, `sentence` or `off`, default `paragraph`; any other value exits 2 (§check-measured-claim).

### (4) The span cases {mechanical}

**Not yet applied.** `canon-kit/gate-tests/check-unmarked-claim.test.sh` gains four cases under a case-local knob file:

- `sentence` with a class match in sentence two: exit 1.
- `paragraph` with the same text: clean.
- `off` with the same text: clean.
- A mid-paragraph full-line marker with a class match on the line above it, at `paragraph`: exit 1.

The `good/`+`bad/` pair runs at the default and is unchanged. `canon-kit/gate-tests/check-measured-claim.test.sh`'s span cases stay green through the helper, which is how delta 1's refactor is held to arm C's current behaviour.

## Producers and consumers

- **The discharge range.** Producer: the delta 1 helper, at each scanned paragraph. Consumer: `check-unmarked-claim` arm A, at the same transition. No field is stored.
- **`CANON_KIT_MEASURED_SPAN`'s second reader** (delta 2). The member declares the knob, so the knob-file derivation now couples `canon-config.knobs` to `check-unmarked-claim` in the regenerated hook. `check-knob-default-coupling` reads the bullet delta 3 rewrites, which still states the default.
- **Point 5.** Delta 2 narrows the discharged set, which widens arm A's red set. Arm A reds per violation, so it is monotone, and no reader of this gate asserts a count or a floor over findings. The clean line counts files, classes and paragraphs, none of which the change moves.
- **Point 6.** Not obliged: no delta obliges every member of a corpus.

## Existing sections updated

Roster from `grep -n "MEASURED_SPAN\|§check-measured-claim's paragraph" canon-kit/SPEC.md` and `git grep -n "MEASURED_MARKER\|Span::" native/src/gates`, run 2026-09-23.

- `native/src/spec.rs` and `native/src/gates/measured_claim.rs` (delta 1).
- `native/src/gates/unmarked_claim.rs` and `native/src/gates/mod.rs` (delta 2).
- `canon-kit/SPEC.md` §check-unmarked-claim and §Layout and configuration (delta 3).
- `canon-kit/gate-tests/check-unmarked-claim.test.sh` (delta 4).
- `scripts/git-hooks/pre-commit` and the graph artifact, regenerated for the new knob declaration (delta 2).
- `.workflow/release-declarations.md`, one Tightened gates bullet (delta 2): `check-unmarked-claim` discharges only the claim a full-line marker binds. Under `CANON_KIT_MEASURED_SPAN=sentence` that is the marker's first sentence, and at every span a class match above the marker is unmarked.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the helper and the second reader.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entry moved.** `unmarked-discharge-vs-span` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
