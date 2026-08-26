# SPEC amendment: done-exits

Closes `absorbed-duplicate-disposition`. An entry that a landed unit or a closed
ruling has mooted has an exit, and the surfaces that name that exit disagree
about whether it exists — so a correct merge was ruled correct and then ruled
unexecutable, and two entries owning one gap stayed live and cross-referenced.

**The disposition is already ruled, and this amendment applies a ruling rather
than making one.** TRAJECTORY.md §The closed rulings, 2026-08-23 by the operator,
on a measured 8:1 intake-to-exit ratio: *close's existing wontfix disposition —
the bare `## Done` line — is the ordinary exit for an entry a landed unit or a
closed ruling has mooted, taken by close under those two criteria rather than
escalated. The icebox compresses and the wontfix line retires; neither needs a
new section.* Two criteria, one existing section, no new state.

**What was actually out of position, stated plainly because the entry records the
opposite.** The 2026-08-26 lead ruling — *`## Done` asserting work that never
shipped is a false statement in a governance surface* — reads Done as a delivery
claim, and a standing operator ruling had already said it is not. Conforming a
lead ruling to a standing operator ruling is what spec-over-precedent requires;
the operator ruling is untouched and is the thing being obeyed.

**The refused branch is refused by that ruling and not by this amendment.** A
fourth section is out: *neither needs a new section*. It was never available to
be weighed here.

**One of the entry's two costs is falsified and the correction lands with the
unit.** `drift-kit/kpis/kpi-task-split.sh:27-31` classifies a Done slug by its
landing commit's subject, `feat*` versus `fix*`/`refactor*`. An entry that
shipped nothing has no such commit — its landing commit is a `chore` or a `docs`
— so it falls to **`unclassified`** and the **feat/debt split is not inflated at
all**. Only `total`, the "of N done" denominator, moves. The entry's accounting
cost is therefore smaller than filed, and the *unfalsified* half is what carries
the unit: the absent disposition made a correct merge unexecutable, so duplicates
accumulate by construction and each keeps drawing re-filings.

**Deliberately unauthored, and named so it is visibly unauthored.** This
amendment does **not** rule that Done accepts an entry under *any* disposition.
It states the operator's two criteria and no more. A disposition outside them —
a slug retired for a reason neither criterion reaches — is a fresh question with
a real fork, and it should cost an escalation rather than be pre-granted by a
generous phrasing here.

## What changes

### (1) The done section is contracted as the live pool's exit, under two named criteria

`queue-kit/SPEC.md` §The queue format's done-section bullet — today "one line per
completed task, the bare slug only" — is rewritten to state what the section
**is**: the record that an entry has **left the live pool**, one line per exit,
the bare slug only, with prose about what happened in git history. An entry
reaches it when a **landed unit or a closed ruling has mooted it**, or as a
**ruled wontfix**. **{design-bearing}**

Stated positively, and that is a requirement rather than a style note. "No longer
a delivery claim" tells a reader what the section has stopped meaning and leaves
open what it now means, which is the state the two surfaces are already in and
the state this delta exists to end. A reader arriving at a Done line must be able
to say what it asserts from this bullet alone.

**The bullet is the wrong half of a contradiction the spec already carries, which
is why it is repaired here rather than filed.** §The icebox tier's conserved-moves
bullet already says "either → done for a ruled wontfix" — an exit for work that
shipped nothing, agreeing with the operator ruling — while §The queue format says
"completed". Repairing the ruling and leaving `:45` standing would leave the
identical contradiction for the next session to re-litigate, which is the
enforcement-first rule reaching a prose defect: the fix and the surface that
caused it land together.

**Two things that do not change, because the natural reading of a widened
contract is that they did.** The grammar is untouched — a bare slug line, no tag,
no disposition token — so `check-task-conservation`, `kpi-task-split` and every
other Done reader parses exactly what it parses today. And nothing about
**delivery** stops being recorded: the landing commit's type is where a delivery
claim lives and always was, which is precisely why delta 3's `unclassified`
bucket works.

### (2) The conserved-moves enumeration takes the second criterion

`queue-kit/SPEC.md` §The icebox tier's "**Eviction is a conserved move**" bullet
enumerates "deferred → icebox, icebox → deferred on real recurrence, and either →
done for a **ruled wontfix**". It gains the operator's other criterion, so the
enumeration carries both: **either → done for a ruled wontfix, or for an entry a
landed unit or a closed ruling has mooted**. **{mechanical}**

Transcription of a ruled criterion into the enumeration that already holds its
sibling — the wording is given by the ruling and the placement by the bullet's
existing shape. It matters because that bullet is the surface a reader consults
for *which exits exist*, and an exit the operator ruled but the enumeration omits
is an exit a session will not take. Its neighbouring claim — "No sanctioned
disappearance needs inventing, and none exists — every exit from the
design-pending pool is conserved" — is strengthened rather than disturbed: the
added route is conserved like the rest, the slug staying visible in the file.

### (3) `kpi-task-split`'s reading is corrected where the KPI is owned

`drift-kit/SPEC.md` §Bundled KPIs' `kpi-task-split` bullet states what its
denominator counts now that delta 1 has said what Done is: `total` is **entries
that left the live pool**, not deliverables, and an exit that shipped nothing
lands in **`unclassified` by construction** — it has no `feat`/`fix`/`refactor`
commit naming its slug to be classified by. **{design-bearing}**

**This is causal completeness, not scope creep.** Delta 1 changes what a Done
line asserts, and this bullet is an existing spec section describing the prior
flow for a live reader of Done. Leaving it would put the corrected contract in
one surface and the old reading in the other, which is the two-sources defect the
whole unit is about.

**Two limits ship with it because `unclassified` is a mixed bucket.** A task that
genuinely delivered but whose landing commit was typed `docs` or `chore` also
lands there, so the bucket is *not* a count of non-shipping exits — it is the
bucket that non-shipping exits **cannot escape**. And the existing caveat holds
unchanged: a Done slug no commit message mentions is unclassified however
correctly that commit is typed, and a later `chore` naming the slug wins the
lookup over an earlier `fix` that did not. The honest claim is that the feat/debt
**split** is unpolluted by a non-shipping exit; no claim is made that
`unclassified` measures them.

### (4) The absorbed duplicate is named as an instance, and its execution stays close's

`queue-kit/SPEC.md` §The icebox tier's **Distinct from the icebox tier** reasoning
gains the case the entry was filed for: a slug **merged away as a duplicate** is
an entry a landed unit has mooted — criterion (a), not a new class — and its exit
is the bare Done line like any other. The same holds for a slug **mooted by
supersession**, the second route the entry's first recurrence found.
**{design-bearing}**

**The disposition itself is not executed here, and the boundary is the operator
ruling's own.** That ruling puts the deferred pool's exits in **close's** hands —
"taken by close under those two criteria rather than escalated". So this
amendment states the criteria that make `payload-derivation-ships-untracked-residue`
eligible — its absorbing entry has shipped and every ground it names is
discharged, which is criterion (a) exactly — and stops there. A build batch
disposing of it would be taking a move that is not this stage's and not that
stage's either.

**Naming the case is what makes the criteria usable.** Criterion (a) reads as a
statement about *supersession* and a reader meeting a duplicate merge will not
recognise it, which is the entry's own finding one level up: the class is not
"absorbed duplicate" but any slug reaching Done having shipped nothing, and the
routes into it are what a reader needs enumerated.

## Producers and consumers

**No new state, event, message, field, tag, section or knob is introduced by any
delta.** That is the operator ruling's substance — *neither needs a new section* —
and it is why the causal-completeness points discharge as statements about
existing flows rather than about a new one.

**The contracted Done line (deltas 1, 2 and 4).**

- *Producer:* the **close** stage, at the deferred-pool drain and at the Done
  move, writing the same bare slug line it writes today. **Enabling config
  actually set:** none is added; `QUEUE_KIT_DONE_SECTION` already resolves to
  `Done` in this tree and the grammar is unchanged, so every existing producer
  path is live and no configuration gates the widened criteria.
- *Consumers,* three, all existing, all reading the unchanged grammar:
  `check-task-conservation` at commit, over the live-slug diff;
  `drift-kit/kpis/kpi-task-split.sh` at the drift report, over the section's
  slug lines; and the **human reader** of the queue, at the moment a slug's
  absence from the live pool has to be explained. The third is the reader delta 1
  serves and the one both surfaces currently mislead.
- *Named reader for every field:* the line carries one field, the slug, and its
  readers are the three above. **No disposition field is added**, deliberately —
  a token distinguishing the criteria would have no reader (`check-task-conservation`
  reads membership, `kpi-task-split` reads the commit, and the human reader has
  git history), and canon-kit's rule is that a field with no named reader is
  removed rather than added.

**The corrected KPI reading (delta 3)** introduces no state. Its producer is the
existing emitter, unmodified; its consumer is the report reader at the close
transition; the correction is to the **section that describes** what the emitter
already emits, which is why no code moves.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
No delta narrows a corpus — nothing is pruned, no glob tightens, no file is
removed. Every delta **widens** what may lawfully enter an existing section.
Each affected reader is enumerated by its **red condition** anyway, because the
widening's whole risk is a reader that reds on a *membership* change:

- `check-task-conservation` — reds when a slug live at HEAD is absent from the
  worktree's live set with no sanctioned destination. **Not monotone**: it holds a
  conservation condition, so a slug *leaving* the live pool is exactly its
  trigger. Cleared by inspection and by its own help, which already sanctions the
  Done move; delta 1 changes what the destination *means* and not whether it is
  sanctioned, so the gate's verdict on every existing and every future move is
  unchanged.
- `check-task-names` — reds on a slug collision or an unresolvable
  `[blocked-by:]`. **Not monotone** (both are find-something conditions over the
  live pool). Cleared by inspection: an entry reaching Done leaves the live
  namespace, which is the collision-closing property the entry's own "Distinct
  from the icebox tier" paragraph relies on, and a `[blocked-by:]` naming a Done
  slug is *already* a stale tag this gate reds on — unchanged by the widening.
- `check-amendment-queue` — reds on `[design-pending]` in an active section and
  on a `[spec:]` ref resolving to no file. **Not monotone.** Its own coverage
  limit is the one to hold in view here: the **done section is an exempt
  population**, so a tag surviving a Done move reds nowhere. The bare-slug grammar
  is what drops it, and delta 1 keeps that grammar — which is why the grammar is
  named as unchanged rather than left implicit.
- `check-queue-sections` / `check-queue-hygiene` — red on section-order breach and
  on an exact-duplicate line. **Not monotone** (both find-something). Cleared by
  construction: no section is added, moved or renamed, and no line is added to the
  file by any delta.
- `check-queue-entry-budget` — reds on a deferred entry over the cap and on a
  missing cost field. **Holds a maximum and a minimum, so not monotone.** Cleared
  by inspection: it binds **deferred** entries, and a Done entry is a bare slug
  outside its population entirely.
- `drift-kit`'s KPI coverage for `kpi-task-split` — reds when the emitter's
  observed line differs from the asserted one. **Not monotone** (exact match) and
  **cleared by inspection**: delta 3 edits a SPEC section and not the emitter, so
  no emitted string moves. Named because a reader of delta 3 will reasonably
  expect the emitter to change and it does not.

## Existing sections updated

- `queue-kit/SPEC.md` §The queue format, the done-section bullet — the contract
  restated as the live pool's exit under the two criteria, with the grammar
  explicitly unchanged; this is the repair of the wrong half of the spec's own
  contradiction (delta 1).
- `queue-kit/SPEC.md` §The icebox tier, the "Eviction is a conserved move" bullet
  — the enumeration takes the second criterion, and the neighbouring
  no-sanctioned-disappearance claim is re-read against it (delta 2).
- `queue-kit/SPEC.md` §The icebox tier, the "Distinct from the icebox tier"
  reasoning — the absorbed-duplicate and mooted-by-supersession routes are named
  as instances of criterion (a) rather than as classes needing their own state
  (delta 4).
- `drift-kit/SPEC.md` §Bundled KPIs, the `kpi-task-split` bullet — what `total`
  counts, why a non-shipping exit lands in `unclassified` by construction, and
  the mixed-bucket limit (delta 3).
- `TRAJECTORY.md` §The closed rulings — no content change, and **the ruling is not
  rewritten**: it is the surface being obeyed. Re-read at merge to confirm the
  merged spec states its two criteria without narrowing or widening them, which is
  the one way this unit could go wrong (deltas 1 and 2).
<!-- update-target-exempt: owned by no delta — the disposition it describes is close's move under the operator ruling, so no delta may claim this entry's edit -->
- `TASK-QUEUE.md`'s `payload-derivation-ships-untracked-residue` entry — no edit
  by this unit. Its "`absorbed-duplicate-disposition` owns that missing third
  state" paragraph becomes false once this merges, and correcting it rides the
  **close** that dispositions the specimen, not a build batch.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`), discharged **at the iteration** rather
      than at this commit, a sibling queue-kit amendment being in flight
      (canon-kit/SPEC.md §Merging an amendment, step 3).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The retired **claim** is the word "completed" in
      the done-section bullet, so the grep is over the surfaces that restate it.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The two criteria are transcribed, not paraphrased wider** — the merged
      text is read back against TRAJECTORY.md §The closed rulings' sentence. A
      merged spec saying Done accepts an entry *whatever its disposition* has
      minted envelope the operator did not rule.
- [ ] **No disposition is executed** — `payload-derivation-ships-untracked-residue`
      is still live at this unit's completion. Its exit is close's move.
- [ ] **The `unclassified` behaviour is observed, not cited** — the classification
      of a non-shipping Done slug is re-run against
      `drift-kit/kpis/kpi-task-split.sh` at build rather than taken from this file.
