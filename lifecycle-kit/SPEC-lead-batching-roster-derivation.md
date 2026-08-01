# SPEC amendment: lead-batching-roster-derivation

An iteration lead that batches by **amendment** silently drops every
amendment-free unit scope promoted. The instance: five units promoted, three
carrying an amendment and batched, two Technical Debt units carrying none — a
debt unit converges an implementation on existing spec and mints no governed
name, so it has no `[spec:]` ref to key on — and the lead dispatched "batch 2 of
2". Caught only because the build session read its own ritual's exit condition,
disagreed with the lead's framing, and escalated instead of exiting on it.

The failure is the iteration's recurring shape: a roster keyed on the wrong
thing **stops covering** rather than reddening. Nothing errors, every amendment
merges, and from the amendment set alone the iteration looks complete.

The entry left the call open between two candidate fixes with different owners —
derive the roster from scope's promotion record (a template change), or assert
build's exit condition against the live queue in a gate (a new gate) — noting
that "the second is the oracle the first lacks". **That premise turned out to be
stale, and re-verifying it is most of this amendment's value.**

## The oracle already exists

`check-stage-entry` assertion B — drain-entry queue-empty — already requires the
configured active queue sections to carry no top-level entry at drain-stage
entry, and it exists precisely to catch entry-on-incomplete-build. This roster's
drain stage is `validate` (the kit default, not overridden here), and the
configured active sections are the feature and technical-debt sections, so a
promoted **debt** unit is squarely in its population.

Run rather than reasoned about, against this very iteration's live queue with two
promoted debt units outstanding, via the cheap pre-dispatch simulation the lead
template already sanctions:

> `bash lifecycle-kit/bin/enter-stage.sh --simulate validate`
>
> `entering 'validate' but the active queue is non-empty (the prior stage is not
> drained):` — followed by the two undrained entries, each named by line and
> slug. Exit 1.

So the entry's "nothing reddens" is false. The defect is **detected late, not
undetected**: the lead would have dispatched a validate session, that session's
`enter-stage.sh` would have refused fail-closed and named the dropped units, and
the lead would have reopened build. The real cost is a wasted dispatch and a
refused entry, not a silently incomplete iteration.

**The new gate is therefore refused, not deferred.** A second assertion over the
same population, at an earlier point, would be a duplicate reading of one fact —
the enforcement-first rule's own preference is that removing the duplication
outranks gating it, and there is no duplication to remove here, only a second
gate to avoid adding. What is owed is that the lead **derive its roster
correctly** and **know where the floor is**.

Recording the refusal with its evidence is the point: the next reader of this
entry would otherwise re-derive "we need a gate" from the same stale premise,
and the run above is what stops them.

## What changes

### Delta 1 — the batching roster is derived from the promoted set {design-bearing}

Into the body of §Economics' **Batch dispatches by shared surface** bullet; the
lead-in is untouched.

The bullet today rules out both naive granularities and says to batch units that
share a kit or SPEC surface. It never says what the *set being batched* is, and
the lead filled that silence with the amendment set, because the amendments are
what a lead reads to tier a batch:

> The set being batched is **every unit the iteration promoted** — the active
> queue's feature *and* debt sections — never the amendment set. A debt unit
> converges an implementation on existing spec and mints no governed name, so it
> carries no `[spec:]` ref; keying the roster on amendments makes exactly the
> amendment-free half of the queue invisible, and it disappears silently because
> the units that vanish are the ones that left no artifact to miss. Derive the
> roster from the queue, which is the record of what was promoted, and re-read
> it rather than carrying a count.

That last clause is derivation-first applied to the lead's own bookkeeping: a
remembered "N units" is a maintained copy of a fact the queue holds, and it
drifts the moment an escalation adds or splits one.

**The irony is load-bearing, not decorative, so it is stated in the amendment
and not in the template:** this unit is itself a feature unit carrying an
amendment — the one kind of entry an amendment-keyed roster *can* see. A lead
running the defective derivation would batch the fix for the defect and drop its
siblings. That is worth a maintainer's attention once, here, and worth nothing
in the always-read template.

### Delta 2 — the mechanical floor is named where the rule is stated {design-bearing}

The template's own doctrine is "Prompts request; guards enforce" — it already
carries a §Mechanical floor section pairing a prose rule with the guard under
it. Delta 1 is a prompt-side rule and now gets its floor named in the same
place:

> The floor under this rule is `check-stage-entry`'s drain-entry assertion: the
> drain stage refuses to be entered while the active queue still carries
> entries, so a dropped unit surfaces as a **refused entry** at the next stage
> rather than never. That is a backstop, not the working signal — it costs a
> dispatch to learn from. Read it early and cheaply instead: `enter-stage.sh
> --simulate <drain-stage>` returns the same verdict with no session spent, and
> a lead about to declare a stage's batches complete should run it first.

No new mechanism: the simulation flag and its use for gating an expensive
dispatch are already the template's, at `lead.md:66` in **§The lead model** —
"or gates an expensive dispatch cheaply first with `enter-stage.sh --simulate
<stage>`", inside the never-hand-derive-prior-stage-completeness paragraph. (An
earlier draft placed it "one section earlier"; corrected at align — it is six
sections earlier, and knowing which section matters because the floor text must
cite it rather than restate it.) What is added is the *second* occasion for it —
before declaring a stage complete, not only before dispatching into one — and
the naming of which assertion it consults. The oracle-first rule made concrete
for the one decision the lead was making from memory.

**The section heading changes, and that is part of the delta.** The heading is
`## Mechanical floor — the escalation-shape guard`; a second floor under it makes
the subtitle wrong. It becomes the general floor heading (`## Mechanical floor`),
which is what its own "Prompts request; guards enforce" opening already promises.
Checked at align: no tracked file outside this amendment cites the section, so
the rename dangles nothing — but it is stated here so a build session renames it
deliberately instead of leaving a subtitle that contradicts the section's
contents.

The floor's honest limit belongs with it: it fires on the queue's *residue*, so
it catches a dropped unit and says nothing about a unit batched onto the wrong
surface or tiered wrongly. Those stay prompt-side, which is the ordinary
prompts-request/guards-enforce split rather than a gap.

### Delta 3 — the SPEC's template summary tracks both {design-bearing}

lifecycle-kit/SPEC.md §templates/lead.md enumerates what the template owns,
including "the dispatch-granularity rule (batch units sharing a kit or SPEC
surface, split on a model-tier change or a delegation-kit split trigger)". That
enumeration is a reader's index into the template and is now incomplete: the
granularity rule has gained its roster derivation and a named floor.

The sentence gains both, in the section's established compressed style —
citing, never restating — so the index stays a projection of the template rather
than a second statement of the rule. §check-stage-entry needs no edit: assertion
B is unchanged, and this unit adds a second *reader* of it, not a second
behavior.

**Which clause absorbs the floor, because the section already mentions
`--simulate` and must not say it twice.** Found at align:
`lifecycle-kit/SPEC.md` already carries "the lead never hand-derives prior-stage
completeness from WORKFLOW-STATE or the git log (it dispatches and trusts
`enter-stage.sh`'s fail-closed refusal, **or gates an expensive dispatch with
`--simulate`**, §bin/enter-stage.sh)" as a corollary of the
stamps-authoritative invariant — the same mechanism this delta would otherwise
introduce a second time, two clauses away, in one section. So the split is:

- The **dispatch-granularity clause** gains only the roster derivation (delta 1)
  — the promoted unit set rather than the amendment set.
- The **existing `--simulate` corollary** gains the second occasion (delta 2) —
  it already says "gates an expensive dispatch"; it becomes "gates an expensive
  dispatch, and reads drain-entry before declaring a stage's batches complete".

One mechanism, one home, which is the content-tiering rule applied to this
amendment's own edit rather than only to the template's.

## Producers and consumers

This unit adds a rule and a reading discipline, not machinery. No new state,
event, message, or field is introduced — so the field-reader arm of the check is
satisfied vacuously, and deliberately: a "promoted unit count" written somewhere
for the lead to read against was considered and rejected as exactly the
maintained copy delta 1 rules out.

- **The promoted unit set** — *Producer:* the **scope** stage, whose promotion
  commit moves entries out of the deferred section into the active feature and
  debt sections; the queue file is the durable record and the only one.
  *Consumers, three, each at a named transition:* (a) the **lead**, at
  batch-cut, deriving its roster by reading those sections (delta 1); (b)
  `check-stage-entry` assertion B, at drain-stage entry, asserting the same
  sections are empty (delta 2's floor); (c) the **stage session** itself, at its
  ritual's exit condition — the reader that caught the live instance. All three
  read one surface, which is what makes the derivation trustworthy: a lead that
  disagrees with the gate is disagreeing about a file both can open.
- **The drain-entry verdict** — *Producer:* `check-stage-entry`, run by
  `enter-stage.sh` at a real entry **or** under `--simulate` with no state
  written. *Consumers:* the entering stage session, which is refused; and now
  the lead, which may read the identical verdict ahead of time without spending
  a session (delta 2). The simulate path is the pre-existing mechanism this
  delta gives a second occasion to; its enabling wiring is the flag, already
  shipped and already documented.
- **The template's granularity rule** — *Producer:* `templates/lead.md`
  §Economics. *Consumers:* a lead session at batch-cut, and
  lifecycle-kit/SPEC.md §templates/lead.md as its index (delta 3).

**Seam.** Pure mechanism, nothing owed to consumer config. The rule quantifies
over "the configured active queue sections" and the "configured drain stage" —
both already knobs, both already resolved by the stage-config loader — so a
consumer whose roster names different sections or a different drain stage
inherits the rule unchanged. No section name, stage name, or unit vocabulary is
baked into the template text.

## Existing sections updated

- **`lifecycle-kit/templates/lead.md` §Economics — batch, and compact where it
  pays** — the **Batch dispatches by shared surface** bullet body (delta 1).
- **`lifecycle-kit/templates/lead.md` §Mechanical floor** — the drain-entry
  backstop and the early `--simulate` read, plus the heading's rename to the
  general `## Mechanical floor` (delta 2). The section is presently about the
  escalation-shape guard alone; it becomes the template's floor section
  generally, which is the placement its own "Prompts request; guards enforce"
  opening already promises.
- **lifecycle-kit/SPEC.md §templates/lead.md** — the dispatch-granularity clause
  gains the roster derivation, and the existing `--simulate` corollary of the
  stamps-authoritative invariant gains the declare-complete occasion (delta 3;
  the two-clause split is stated under that delta).
- **The docs-site mirror of lifecycle-kit/SPEC.md** and the other generated
  projections, regenerated; each freshness gate names its own regen command on
  red (delta 3's landing — this unit adds no delta of its own for the
  projections, so they ride the SPEC edit that stales them).

No change to `check-stage-entry`, its fixtures, or its SPEC section: this unit
adds a reader of assertion B, not a behavior.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
