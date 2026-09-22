# SPEC amendment: no-change-landing

A lead decision whose content is "make no change" is invisible to every later stage, because the landing rule presupposes an answer with something to write. §The stamp protocol has the stage session land a direction's content with its class, and `lifecycle-kit/templates/lead.md` has the stage session land an answer that amounts to a design ruling. Neither reaches an answer that changes nothing. The attested instance: a scope escalated whether a recurrence date was owed, and the lead declined it and said nothing further was owed. Scope's gap bullet recorded the question as escalated and unresolved. Three stages later, close re-derived the judgment from the owner doc. The opposite answer would have tripped the recurrence threshold and forced a unit-set change, which only the lead may rule.

**The ruling: a no-change answer is landed like any other, on the surface that carries the question.** The entry offered two rules. The first lands the resolution beside the question. The second forbids the lead to assert "nothing further is owed" about a durable record it did not read. The first is taken because it closes the measured cost: the record stops reading as open, so the drain reads a closed question and re-derives nothing. The second is refused as the fix. It limits what the lead may say, but a lead that says nothing leaves the same open record behind, and the drain re-derives just the same. Where the question sits in the gap inbox, the landing is a new bullet, because the inbox is append-only and a bullet is never edited. The drain then reads the question and its resolution together, and dispositions both.

## What changes

### (1) §The stamp protocol: an answer that changes nothing is still landed {design-bearing}

**Not yet applied.** In the paragraph beginning "**A stage session landing a direction writes its class and date in the same commit as its content.**", add after its first sentence:

> An answer whose content is that nothing changes is landed too, because a question recorded as open reads as open until something says otherwise. It lands on the surface that recorded the question, stating the answer, its class and its grounds. A question held in the gap inbox takes a new bullet through `--emit file-gap` naming the bullet it resolves, since the inbox is append-only (§The committed gap inbox), and the drain dispositions the two together.

### (2) templates/lead.md: the relay of a no-change answer says it lands {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md` §Stamps are authoritative, the paragraph beginning "An answer that amounts to a design ruling is landed **by the stage session**" gains after its first sentence:

> So is an answer that nothing changes: relay it as an answer to be landed, naming the surface that recorded the question, never as a close of the matter (lifecycle-kit/SPEC.md §The stamp protocol).

## Producers and consumers

- **The landed no-change answer** (delta 1). Produced by the stage session that receives the answer, at the moment it acts on it, which every escalation round reaches with no enabling config. Consumed by the next reader of that surface. For a gap bullet that is the close drain (or the first stage's intake), which reads every bullet. For a queue entry or an amendment it is the next stage that reads the entry. Its fields are the answer, the class and the grounds, which the drain reads to disposition the question without re-deriving it.
- **The lead's relay wording** (delta 2). Read by the stage session at its resume, which then lands it under delta 1.
- **`check-gap-inbox-neutrality`.** A resolution bullet has the one bullet shape, the date and the prose, so assertion A reads it as any other bullet. It records that a decision was taken, and by which class, which is an event the drain reads as input. It is not the retired recurrence verdict assertion B refuses, and it takes no disposition out of the drain's hands. Nothing changes for the gate.
- **Point 5.** Not reached; no corpus narrows.
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -e "landing a direction" -e "amounts to a design ruling" -e "nothing further" -- lifecycle-kit`, run 2026-09-22.

- `lifecycle-kit/SPEC.md` §The stamp protocol, the direction-landing paragraph (delta 1).
- `lifecycle-kit/templates/lead.md` §Stamps are authoritative, the answer-landing paragraph (delta 2).

## Retired spellings

- None — no delta retires a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the landed answer.
- [ ] **Instruction surfaces: instruction only.** The lead-template sentence carries the instruction; the grounds sit in §The stamp protocol.
- [ ] **Merged with no information lost.** The landing paragraph is extended by one rule and restates nothing.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `lead-no-change-decision-has-no-landing-site` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
