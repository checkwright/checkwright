# SPEC amendment: the dispatch completion signal

Pairs with the queue entry `lead-dispatch-requires-completion-notification`.

The lead has no stated precondition for dispatching stage N+1, and the one it
improvised is wrong: **artifact state**. A lead checked that validate's commit
had landed with complete evidence, the tree clean, the battery green, and a
simulated close entry cleared — then dispatched close into a still-running
`run-validate`. Every check passed mid-write, because `run-validate` commits its
evidence and keeps going: **the terminal commit existing is fully compatible with
the process still executing.**

## The rule

**Stage N+1 is dispatched on stage N's agent completion notification — never on
its commit, its stamp, a clean tree, a green battery, or a cleared `--simulate`.**
Completion is a fact about a *session*; every one of those others is a fact about
an *artifact*, and no artifact distinguishes "finished" from "still writing".

## Why this does not demote the rule already there — the ruling that makes it safe

`templates/lead.md` already carries a rule about not reasoning from tree state:
the lead never hand-derives prior-stage completeness, and gates an expensive
dispatch cheaply with `enter-stage.sh --simulate <stage>`. A reader could take
the new rule as reversing it. It does not, and the boundary has to be explicit or
the pair is unusable:

- The existing rule is a **gating** rule. It answers *may stage N+1 proceed?* —
  a question about preconditions the machinery owns, which `--simulate` answers
  correctly and cheaply, and which the lead must not re-derive by hand.
- This rule is a **liveness** rule. It answers *is stage N over?* — a different
  question, which no instantaneous read answers at all.

The incident is the proof that they are different questions rather than one
question stated twice: the lead followed the existing rule faithfully, ran
`--simulate`, and the simulated entry **cleared mid-write**. A rule that told the
lead to trust an instantaneous read is not at fault for failing to answer a
question about duration; it was never asked. `--simulate` keeps its whole job.
What it never was, and must now be said not to be, is evidence that the prior
stage is over.

**One qualification, because the paired amendment changes what `--simulate`
sees.** `--simulate` runs every matching entry-preflight command, so once the
producer-liveness lock ships it inherits that gate and the specific mid-write
clearing above stops happening. That is a real narrowing and the template should
not overstate the blindness. It is not a repeal: `--simulate` is still an
instantaneous read, so a producer starting a moment later is still unseen, and
the gate sees only producers that claim a lock. The rule therefore stands as
written — completion is a fact about a session — and the gate's contribution is
to shrink the window in which being wrong about it goes undetected.

The queue entry re-verified at scope that the adjacent paragraph does not already
cover this: it predates the incident and is silent on liveness. That
re-verification stands and is the reason this is an addition rather than an edit.

## The generalizable trap, asserted as part of the rule

**Any prompt-answered signal is a start signal, never a completion one.** The
lead in the incident read an operator's note about having just answered a stalled
permission prompt as the stage being finished. It means the opposite: an approval
prompt gates a command **starting**, so the note timestamps a beginning. This is
stated in the template rather than left as incident lore, because the misreading
is available to any lead on any harness that prompts, and it reads as good news
at exactly the moment the lead wants good news.

## Assertability — the limit, stated rather than implied

The precondition is **prose-only and human-enforced.** The completion
notification is harness session state with no tracked artifact, so no battery
gate can read it. That limit belongs in lifecycle-kit/SPEC.md explicitly, beside
the state machine's other honest limits, rather than being left for a reader to
infer from the absence of a gate. Precedent for a prose-only rule in this same
template: the no-sibling-dispatch clause, prose for the same reason.

Naming the limit is not a shrug. It is what routes the enforcement-first duty to
the place where it *can* be discharged — the next section.

## The paired half, and why shipping one alone is the failure

The **negative is assertable**, and that pairing is the whole disposition:
"is the producer still running?" is exactly what the liveness lock reads.
`check-producer-liveness` (evidence-kit/SPEC.md §check-producer-liveness) is that
oracle, and the two halves are
one unit — **prose rule on the dispatch side, oracle on the artifact side.**
Shipping the oracle while leaving this rule unstated is the specific failure to
avoid: it would install a detector for the consequence while leaving the decision
that causes it ungoverned, and `validate-verb-collision-and-check-routing`
establishes from two consecutive iterations that a prose fix with no oracle
recurs and is caught only by an operator. The converse holds too — this rule with
no oracle is the shape that already recurred.

There is a third member, and stating it completes the causal story rather than
widening the unit. Operative residency (delegation-kit/SPEC.md §Operative
residency) is what makes the signal
this rule depends on **truthful**: a dispatched session that ends its turn on
still-running work emits a completion notification that lies. So the set reads:
that rule makes the signal honest, this one makes the lead depend on it, and
the lock's entry-side reader detects the residual case where it is wrong anyway.
(The lock also gained a writer-side refusal for a distinct, adjacent hazard — a
second or out-of-band producer racing the manifest — which is not part of this
incident class and does not close this residual case;
evidence-kit/SPEC.md §bin/run-validate.sh.)

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

1. **The dispatch precondition in `lifecycle-kit/templates/lead.md`** —
   {design-bearing}
   A paragraph stating the rule, the artifact-state prohibition, and the
   prompt-answered-signal trap, placed beside the post-delegation verify
   discipline — which says what to check *after* a stage and is silent on how the
   lead knows the stage is over. Design-bearing rather than mechanical because
   its placement must not read as reversing the adjacent gating rule: the
   gating-versus-liveness boundary above is part of the delta, not preamble to
   it.

2. **The assertability limit in `lifecycle-kit/SPEC.md`** — {design-bearing}
   A statement that this precondition is prose-only and human-enforced because
   the completion notification is harness session state with no tracked artifact,
   and that its assertable complement is the producer-liveness gate a consumer
   wires into `LIFECYCLE_KIT_ENTRY_PREFLIGHT`. Design-bearing: it is a ruling
   about what the kit does *not* assert, and an under-stated version reads as an
   oversight to be fixed by a later session building the impossible gate.

3. **The ruling-config roster line** — {mechanical}
   Where the lead template's consumer binding enumerates standing dispatch
   policy, the precondition is standing policy and is named there rather than
   improvised per dispatch, per the template's own policy-is-config rule.

## Producers and consumers

This amendment introduces **no new tracked state and no new field.** It names an
existing signal and makes it a precondition, so the causal-completeness check is
answered by naming the signal's ends honestly:

- **The signal** — the agent completion notification for a dispatched stage
  session.
- **Producer** — the harness, when the dispatched session's turn ends. The
  producer is **outside the governed tree**, which is not a gap to be closed but
  the direct cause of the prose-only limit above: no code in any kit emits it, so
  no gate can read it. Its enabling configuration is the dispatch itself — the
  lead dispatches in the background with notification, which is delegation-kit's
  existing mechanic and is already asserted there, so the producer is reachable
  on the ordinary path rather than only in principle.
  Its **truthfulness** is not the harness's to guarantee and is the subject of
  the paired residency amendment; that dependency is named above rather than
  assumed.
- **Consumer** — the lead, at the dispatch transition for stage N+1. The
  mechanism is the lead's own wait: it holds the dispatch until the notification
  arrives, which is the delegation protocol's never-poll pattern that a
  supervisor role already has.
- **The assertable complement** — `check-producer-liveness` at the stage-entry
  transition, specified in evidence-kit/SPEC.md §check-producer-liveness. It consumes no
  part of this signal; it independently answers the negative from the artifact
  side, which is why the pair covers the case where the signal itself is wrong.

## Existing sections updated

- `lifecycle-kit/templates/lead.md` §The lead model — the paragraph on never
  hand-deriving prior-stage completeness gains the gating-versus-liveness
  boundary, so the existing rule is not read as answering the new question. This
  is an **edit to existing prose**, not an addition beside it; leaving it
  untouched is what would let the two rules read as contradictory. Owned by
  delta 1.
- `lifecycle-kit/templates/lead.md` — the dispatch precondition itself, beside
  the post-delegation verify discipline. Owned by delta 1.
- `lifecycle-kit/templates/lead.md` §Policy is config, not prose — the standing
  policy roster. Owned by delta 3.
- `lifecycle-kit/SPEC.md` — the assertability limit, beside the state machine's
  existing limits. Owned by delta 2.
- `.claude/agents/stage-session.md` — **not** updated by this amendment. The rule
  is the lead's, and the dispatched session's half is the residency amendment's.
  Recorded so build does not split one rule across two surfaces.

## Out of the envelope, stated so build does not drift into it

The **verb** by which the lead checks a stage's committed work after the fact is
`validate-verb-collision-and-check-routing`'s ground, not this amendment's: this
one governs the moment *before* a dispatch, that one the moment *after*. They
land in the same template and must not be merged — the precondition is about
liveness, the check-routing rule is about which check is safe to run on which
kind of stage.

## Definition of Done

- [ ] **Causal completeness** — the signal's producer and consumer are named, the
      producer's out-of-tree location is stated as the cause of the prose-only
      limit, and the assertable complement is named.
- [ ] **The pair ships together** — this amendment and the producer-liveness
      oracle are both merged, or neither is. A
      merge that lands the oracle alone leaves the dispatch decision ungoverned,
      which is the failure this set exists to end.
      **Discharged at the iteration, not the commit** (operator-relayed ruling,
      2026-08-06): the oracle merged first at `aba9960`, and a symmetric reading
      would deadlock both halves since neither could go first. Build stays in
      build until the queue empties and close refuses entry on a non-empty active
      queue, so the iteration cannot close with this half unmerged.
- [ ] **Merged with no information lost** — the gating-versus-liveness boundary
      lands in the template's prose, not as an appendix.
- [ ] **Amendment deleted** — this file removed on merge; `ls lifecycle-kit/SPEC-*.md`
      returns none.
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
