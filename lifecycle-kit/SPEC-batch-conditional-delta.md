# SPEC amendment: batch-conditional-delta

A spec amendment can state a delta in the unconditional voice when the act that satisfies it depends on which units share its build batch, and the authoring template gives no way to say so. The attested case is two door amendments in one iteration. Each said two door sites were left red deliberately, because repairing them was a paired debt entry's deliverable. That holds only for a batch **without** the debt unit. The lead batched the debt unit in on a producer/consumer read, and the correct act became repairing the two sites. The dispatched session reported it would have left them red on the amendment's letter. Only the lead inverting the instruction in its prompt prevented that, and a prompt dies with the lead. A later session reading the merged amendment alone reaches the wrong act and calls it correct.

**The ruling: an authoring rule. A delta whose act depends on a sibling unit's landing states the act for each case.** The entry offered that rule, or a spec-stage check that a delta citing a sibling unit's deliverable states what happens when both land together. The rule is taken. The check would have to decide from prose whether a delta's act depends on the batch, which is the semantic judgment the author already makes while writing the delta. The rule puts that judgment on the passage, where build and the lead read it, and the lead's batch cut then selects a case the amendment already stated rather than inverting one.

## What changes

### (1) templates/stages/spec.md: a batch-dependent delta states both acts {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/spec.md`, after the paragraph beginning "**Replacement text re-phrases; it never appends.**", add:

> **A delta whose act depends on a sibling unit's landing states both acts.** Where the right act changes with whether a named sibling entry lands in the same build batch, write the act for each case and name the sibling, never one case in the unconditional voice (lifecycle-kit/SPEC.md §templates/stages/).

### (2) §templates/stages/: why the delta names its condition {mechanical}

**Not yet applied.** After the paragraph beginning "`spec.md`'s **authoring-exit pass**", add:

> `spec.md` has a delta **state both acts** where its act depends on a sibling unit sharing the build batch, because the author writes the delta before the lead cuts the batches. A delta written for one case is wrong for the other. The session that builds it has only the amendment to read, and the merged text outlives any prompt that corrected it. So the author states both acts on the passage, and the batch cut picks the one that applies. No check reads it: whether an act depends on the batch is semantic, and the author is the party already judging it. A check deciding it from prose would be a second, weaker judge of the same question.

## Producers and consumers

- **The stated condition** (delta 1). Produced by the spec session while authoring a delta that cites a sibling unit's deliverable. Consumed by the build session that merges the delta, which reads the case its batch is in, and by the lead at batch cut, which reads the deltas already (`lifecycle-kit/templates/lead.md` §Economics, the producer/consumer read) and no longer inverts an instruction in its prompt.
- **Point 5.** Not reached; no corpus narrows.
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -e "re-phrases; it never appends" -e "authoring-exit pass" -- lifecycle-kit`, run 2026-09-22.

- `lifecycle-kit/templates/stages/spec.md`, after the re-phrasing paragraph (delta 1).
- `lifecycle-kit/SPEC.md` §templates/stages/, after the authoring-exit-pass paragraphs (delta 2).

## Retired spellings

- None — no delta retires a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the stated condition.
- [ ] **Instruction surfaces: instruction only.** The template sentence carries the instruction; the grounds sit in §templates/stages/.
- [ ] **Merged with no information lost.** §templates/stages/ gains one paragraph and restates nothing the template carries.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `delta-instruction-batch-dependence-unmarked` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
