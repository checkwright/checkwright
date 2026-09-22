# SPEC amendment: claim-span

`check-measured-claim` binds a full-line `measured:` marker to the paragraph below it, and arm C reds only when the marker's cardinal is absent from that whole paragraph. On a tree that keeps markdown unwrapped a paragraph is one line of any length, so a marker whose own sentence has drifted can still agree with a cardinal elsewhere in the paragraph. The ambiguity close (a bound claim with more than one distinct cardinal fails closed) already confines the false green to a paragraph carrying exactly one distinct cardinal equal to the marker's. What is left is the residue this amendment reaches.

**The ruling: the full-line claim span is a consumer-selected calibration, `paragraph`, `sentence` or `off`, and this repo binds `sentence`.** A second adopter could reasonably want either unit, and neither breaks the gate's contract: `paragraph` suits a wrapped tree whose marked paragraphs are short, and `sentence` suits an unwrapped one. `off` stops arm C for full-line markers only, for a consumer that marks extent claims alone and wants arms A and B without the cardinal check. The inline marker's span is not part of the choice: it is the sentence ending at the marker by the marker's own grammar, and no alternative keeps that grammar. The kit default is `paragraph`, the span the gate shipped with, so an upgrade moves no adopter's verdict. A consumer that wants the tighter binding says so in its own config, which is what this repo does.

The two refused shapes. A code-point reach on the paragraph was weighed and refused: a reach cuts a sentence at an arbitrary point, and arm C would then find or miss a cardinal by where the cut fell rather than by what the sentence says. A stated reason to keep the paragraph alone was refused because the paragraph is right for some trees and not others, which is the case for a knob and not for a kit constant.

**Measured at authoring (2026-09-22).** `git grep -n -E "^<!-- measured: " -- ':!*/gate-tests/*'` finds five full-line markers outside fixtures. Four are `gate-substrates=native` (`CONTRIBUTING.md`, `SECURITY.md`, `docs/methodology.md`, `docs/positioning.md`), a non-cardinal value, so arm C never reads their span. The fifth, `canon-kit/SPEC.md` §check-measured-claim, is the grammar specimen inside a fence, with its generated mirror in `docs/canon-kit/SPEC.md`. So binding `sentence` here moves no verdict on this tree.

## What changes

### (1) The knob `CANON_KIT_MEASURED_SPAN` {design-bearing}

**Not yet applied.** canon-kit's table (`native/src/knobs/canon_kit.rs`) gains the scalar `CANON_KIT_MEASURED_SPAN`, default `paragraph`. Its validator refuses any value other than `paragraph`, `sentence` or `off` at exit 2 under the kit's malformed-config lead line.

`check-measured-claim` reads it once per run and applies it to full-line markers only:

- `paragraph`: the claim is the paragraph below, less every sentence an inline marker in it binds, less marker text. This is today's span, unchanged.
- `sentence`: the claim is the first sentence of that same text, with the sentence boundary §check-provenance-seam's sentence already defines.
- `off`: arm C is skipped for full-line markers. Arms A and B still run on them, and inline markers keep arm C.

The ambiguity close applies to whatever claim the span yields. The registry entry in `native/src/gates/mod.rs` declares the knob beside the two it already declares, and its comment's "the knob set is its two knobs alone" is rewritten to say the gate reads its surface, its oracle and its span.

Fixtures: the `good/` case keeps its current config, so it exercises `paragraph` unchanged. `check-measured-claim.test.sh` gains three cases over one document whose full-line marker's cardinal sits in the paragraph's second sentence: green under `paragraph`, red under `sentence` naming the marker, and green under `off`, plus a fourth case where an unknown span value exits 2.

### (2) §check-measured-claim states the span as the knob's {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-measured-claim, the sentence "The claim a full-line marker binds is the paragraph below it, ending at a blank line, a fence, a second full-line marker or the end of file, less any sentence an inline marker in it binds." becomes:

> The claim a full-line marker binds is set by `CANON_KIT_MEASURED_SPAN`: at `paragraph` it is the paragraph below the marker, ending at a blank line, a fence, a second full-line marker or the end of file, less any sentence an inline marker in it binds; at `sentence` it is the first sentence of that text; at `off` arm C does not read a full-line marker at all. The span is the consumer's to choose because the right unit depends on the tree: a short wrapped paragraph binds tightly enough whole, while an unwrapped paragraph is one line of any length, over which a marker whose own sentence has drifted can still find its cardinal elsewhere. A code-point reach is refused because it would cut a sentence at an arbitrary point, and arm C would then find or miss a cardinal by where the cut fell.

The paragraph beginning "Producer: the generated pre-commit hook" is unchanged.

### (3) The knob joins canon-kit's roster, and this repo binds `sentence` {mechanical}

**Not yet applied.** canon-kit/SPEC.md §Layout and configuration gains, after the `CANON_KIT_MEASURED_CLAIMS_CMD` bullet:

> - `CANON_KIT_MEASURED_SPAN` — the claim a full-line `measured:` marker binds for `check-measured-claim` arm C: `paragraph`, `sentence` or `off`, default `paragraph`; any other value exits 2 (§check-measured-claim).

This repo's `scripts/canon-config.knobs` gains `CANON_KIT_MEASURED_SPAN = sentence` beside its two other `CANON_KIT_MEASURED_*` lines. `canon-kit/templates/canon-config.knobs` is a one-line comment-only template that lists no knob, so it is unchanged.

## Producers and consumers

- **The span value** (delta 1). Producer: canon-kit's table, overridden by a consumer's knob file; this repo sets it. Consumer: `check-measured-claim`, read once per run, at the claim-building transition for each full-line marker. The validator reads it at the kit's first resolution.
- **Point 2, roster readers of the minted name.** The registry declaration in `native/src/gates/mod.rs` (a knob a member reads undeclared is a finding under the crate's declaration tests); canon-kit/SPEC.md §Layout and configuration (the knob roster `check-knob-default-coupling` and `check-knob-citation` read); `--emit knob-roster`, which reads the table and needs no edit; `check-docs-cmd`'s static knob names, which derive from the tables. The knob file couple is derived (gate-sdk/SPEC.md §The `# graph:` manifest), so the descriptor needs no edit.
- **Point 5.** `sentence` narrows the claim, and arm C reds on a cardinal *absent from* the claim, so narrowing can only add reds. That is the tightening this repo chooses, and the measurement above shows it adds none here. The default `paragraph` narrows nothing.
- **Point 6.** Not reached.

## Existing sections updated

Roster from `git grep -n -E "CANON_KIT_MEASURED|check-measured-claim" -- ':!docs/' ':!TASK-QUEUE.md' ':!*/gate-tests/*'`, run 2026-09-22, read for surfaces describing the full-line span.

- `native/src/knobs/canon_kit.rs`, the table row and validator arm (delta 1).
- `native/src/gates/measured_claim.rs`, the claim builder for full-line markers (delta 1).
- `native/src/gates/mod.rs`, the `check-measured-claim` registry entry and its comment (delta 1).
- `canon-kit/gate-tests/check-measured-claim.test.sh`, the span cases (delta 1).
- `canon-kit/SPEC.md` §check-measured-claim, the span sentence (delta 2).
- `canon-kit/SPEC.md` §Layout and configuration, the knob bullet (delta 3).
- `scripts/canon-config.knobs`, this repo's binding (delta 3).
- `.workflow/release-declarations.md`, a Behavior changes bullet naming the new knob and its unchanged default (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`.

## Retired spellings

- None — no delta retires a name; the span sentence is rewritten in place and every name it used survives.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the span knob.
- [ ] **Instruction surfaces: instruction only.** Not reached; no template or shim carries grounds.
- [ ] **Merged with no information lost.** The span sentence is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entry moved.** `measured-claim-span-unbounded` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
