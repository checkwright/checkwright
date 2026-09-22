# SPEC amendment: supersession

The scope template aggregates a candidate's **inbound** citations before ranking it: siblings that converge on it, subsume it or block against it (`lifecycle-kit/templates/stages/scope.md`, the aggregation paragraph). It has no test for a deferred entry that makes another obsolete when neither cites the other. A tactical entry is then ranked as work when a strategic sibling would moot it. The attested instance: one scope recommended three line-cap fixes and left the entry that ruled markdown unwrapped, which mooted all three, to an iteration of its own. The operator reversed that choice.

**The ruling: a supersession pass over the shortlist, with its verdict written into the unit-set escalation.** The queue-edges arm cannot find this relation, because it reads citations and the relation has none. What the two entries share is a **mechanism**: the strategic entry's deliverable replaces, removes or re-conventions something the tactical entry's deliverable edits. So the pass searches by the mechanism's names, not by slug. For each shortlisted candidate it takes the names its deliverable edits (a gate, a knob, a file, a convention), greps the deferred pool and the icebox for entries whose deliverable changes one of them, and reads each hit. The verdict joins the `Composition:` line on the same channel and for the same reader, the party ruling on the set. It is not a queue write, and no gate can judge it.

**Why the shortlist and not the pool.** The pairing is a read of two bodies per hit, and the ranking already opens bodies only for its shortlist. Run over the pool, the pass costs a body read per pair across every entry, most of which the ranking never proposes. A strategic entry that moots an unshortlisted entry costs nothing this iteration. The close stage's moot sweep and the pool-wide triage remain its backstops.

## What changes

### (1) The scope template carries the supersession pass {design-bearing}

**Not yet applied.** In `lifecycle-kit/templates/stages/scope.md`, after the aggregation paragraph (the one ending "so splitting an entry is safe only against that total."), add:

> **Then test each shortlisted candidate for supersession, which no citation records.** Take the names its deliverable edits (its gate, knob, file or convention), grep the deferred pool and the icebox for entries whose deliverable replaces, removes or re-conventions one of them, and read each hit. Rule each pairing **mooted** (the other entry's landing removes the need: rank that entry instead, or bundle the two), **reshaped** (the fix changes shape once the other lands: say which goes first) or **independent**. The unit-set escalation's Recommendation carries one line per non-independent pairing, `Supersession: <candidate> — <mooted|reshaped> by <entry>`, or `Supersession: none — <names searched>` when nothing paired (lifecycle-kit/SPEC.md §templates/stages/).

### (2) §templates/stages/: the supersession pass and its limit {design-bearing}

**Not yet applied.** After the paragraph beginning "The `scope` template weighs a unit set against the pool's refill rate", add:

> The `scope` template tests its shortlist for **supersession** because the inbound-edge read sees only citations, and one entry can moot another without citing it: what they share is a mechanism one replaces and the other patches. The pass is keyed on the mechanism's names for that reason, and bounded by the shortlist because the ranking already reads those bodies and no others. Its line travels on the message channel beside the `Composition:` line, for the party ruling on the set, so it is unconditional on the same ground: a `none` line names the searched terms, and a thin search then shows as thin. **The honest limit:** mooting is semantic, so no gate judges the verdict, and a search whose names miss the shared mechanism reports `none`. The close stage's moot sweep (above) catches a mooting that lands, and the pool-wide triage the rest.

### (3) templates/lead.md: the presence check reads both lines {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md` §Opening an iteration, "A unit-set escalation whose Recommendation carries no `Composition:` line (lifecycle-kit/templates/stages/scope.md, the economic composition test)" becomes "A unit-set escalation whose Recommendation lacks its `Composition:` line or its `Supersession:` lines (lifecycle-kit/templates/stages/scope.md, the composition test and the supersession pass)". The rest of the paragraph is unchanged: the check is presence, not quality.

## Producers and consumers

- **The `Supersession:` line** (deltas 1 and 3). Produced by the scope session on every unit-set escalation, which every iteration makes, so it is live with no enabling config. Consumed by the lead's presence check at its routing transition and by the party ruling on the set, who reads the verdict. Its three fields are read: the candidate and the other entry name the pair the ruling weighs, and the verdict names what the ruling decides (rank, bundle, order).
- **The `none` form's searched names.** Read by the ruling party, to judge whether the search was wide enough.
- **Point 5.** Not reached; no corpus narrows.
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -e "Composition:" -e "inbound edges" -e "supersed" -- lifecycle-kit`, run 2026-09-22.

- `lifecycle-kit/templates/stages/scope.md`, after the aggregation paragraph (delta 1).
- `lifecycle-kit/SPEC.md` §templates/stages/, after the refill-rate paragraph (delta 2).
- `lifecycle-kit/templates/lead.md` §Opening an iteration, the presence-check paragraph (delta 3).

## Retired spellings

- None — no delta retires a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the supersession line.
- [ ] **Instruction surfaces: instruction only.** The scope and lead template edits carry the instruction; the grounds sit in §templates/stages/.
- [ ] **Merged with no information lost.** §templates/stages/ gains one paragraph and restates nothing the template carries.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `scope-supersession-unchecked` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
