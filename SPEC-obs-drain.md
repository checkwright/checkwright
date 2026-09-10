# SPEC amendment: obs-drain

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land; where replacement wording is given it is marked **Not yet applied** at the passage itself.
`spec` authors, build lands, and a reader arriving here mid-iteration must not read a quoted
sentence as one already in the tree.

The entry this amendment discharges records a five-year-old-feeling wedge in three sentences and
then says why nothing was ever built: *"no shape is costed and each candidate costs something
real."* Four shapes address the drain side, a fifth addressed the record side and had its premise
falsified, and the live half was narrowed at a later stage to **which push produces the
observation**. This amendment costs the shapes, declines two of them on their own arithmetic, and
rules a sixth that none of the five could reach.

## Why no shape could be costed, which is the finding the five shapes were missing

**The class has no name the machine can read.** Probed at HEAD rather than assumed: the string
`run-observed` occurs exactly three times in the tracked tree, all three in `TASK-QUEUE.md`, and
all three are prose. Its one live use was as the opaque `<reason>` field of
`[precondition-ok: run-observed]` on `platform-support-ci-matrix`, an entry since retired.
`[precondition-ok:]` is a per-entry opt-out valve for `check-queue-prose-precondition`, that gate is
its only reader, and it treats the reason as an opaque string it never inspects.

So every one of the five shapes is a **policy about a set no surface enumerates**. A policy's real
cost is dominated by who must notice that an entry belongs to the set and when — and today that is
one human, reading prose, at a moment nothing prompts. This is why the recurrence judgment landed
where it did: the entry's own 2026-08-31 record says the miss was **filing-visibility rather than
analysis**, and *"sharpening would not have helped."* Naming the class is the fix that judgment
points at, and it is the cheapest thing on the table because every shape below then becomes a rule
a session is prompted with rather than a rule it must remember.

## The bind, re-probed rather than inherited

`check-stage-entry` assertion B refuses a drain-stage entry while the configured active sections
carry any top-level entry. `[drain-exempt: <reason>]` skips an entry at the **drain stage's own**
entry, but assertion B re-runs with no exemption at every **drain successor** — every stage whose
`LIFECYCLE_KIT_PREDECESSOR` value is the drain stage. Under this tree's map (`[close]=validate`,
drain stage `validate`) `close` is a drain successor, so no valve reaches close entry: a non-empty
active queue cannot enter close, tagged or not. The entry's recorded bind is exact and this
amendment does not reopen it.

The observation producers, likewise probed: `gates` fires on a push to `master` and every
`install-smoke-*` leg is a job inside it; `publish` fires only on a `v[0-9]*` tag. That split is
what delta 3 turns on, and it is the whole reason one rule cannot cover the class.

## What changes

### (1) The class gets a name a gate can read: the `[observed-by: <producer>]` queue tag

queue-kit's tag algebra gains one tag, and it is the delta the other four rest on {design-bearing}.

`[observed-by: <producer>]`, lead-line-scoped, `<producer>` non-empty. It declares that this
entry's completion predicate is an **observation of a remote run** rather than a tree state, and
`<producer>` names which run produces it. The value is **opaque to every gate**: no arm branches on
it, and no knob carries a producer roster. That is the seam this kit already holds for
ruling-authority names — the kit ships the slot, the consumer's own workflow names fill it, and a
kit literal spelling `gates` or `publish` would publish one consumer's CI layout to every adopter.
Its one *reader of the value* is a human at the scope stage, named in delta 3.

Lead-line-scoped because delta 3's reader scans lead lines at promotion time, which is the same
ground `check-tag-lead-line` already applies to the tags it holds there. Mechanically this is one
member added to that gate's lead-line set plus the non-empty-value check the set already applies.

**Overloading `[precondition-ok:]` is refused, with its ground.** The string `run-observed` already
rides that tag's reason field, so reusing it looks free. It is not: `[precondition-ok:]` is an
opt-out for a *different* gate, it is deliberately **not** lead-line-scoped
(`check-queue-prose-precondition` honors it anywhere in the entry), and its reason is contractually
opaque. Tying the class marker to it would bind two independent conventions to one string and stop
either changing without the other — canon-kit's own argument for keeping the work-class tag outside
the delta-ID grammar, applied here.

### (2) The push-placement rule the tag turns on, and it buys no push

lifecycle-kit/SPEC.md gains one rule, stated over the tag and over no count {design-bearing}:

> An iteration whose promoted set carries any `[observed-by:]` entry places its **first** push at or
> before the stage that lands that entry's work — never at the closing stage.

The arithmetic that makes this the cheap shape is **already ruled and already on the entry**: a
finished run is read for free, and the commits that record the reading accumulate locally and ride
the close push, which the 2026-09-01 lead ruling establishes costs no push of its own. So the rule
**re-places an existing push rather than buying one**, and the observation arrives *between* the
iteration's two pushes — early enough to be read, committed, and drained before the drain gate is
reached. The entry's own 2026-09-05 correction derived exactly this and left it as a property of
the budget; this delta makes it an obligation keyed to a tag.

**The seam is held deliberately.** The *number* of pushes an iteration may spend is consumer
content — `CLAUDE.md` owns it here — and this rule names no number. It constrains only **where** the
first one falls, which is mechanism about an ordering the state machine already owns.

**This delta's envelope was contested and is settled — `operator 2026-09-11`, ruled through
AskUserQuestion in the lead session and relayed by that lead.** The authoring stage escalated
whether a *standing* obligation in shipped kit mechanism may constrain where an operator-owned
resource falls, offered three dispositions — confirm as authored, soften to a recommendation the
scope stage weighs, or relay — and recommended the first. The operator took it: **the placement rule
is inside this amendment's envelope and the authored form stands.**

The record is here rather than only in the thread that carried it because the adjacency is the whole
reason the question was operator-class and not the lead's to self-clear: the push budget had been
ruled on the day before, and `CLAUDE.md`'s standing one-to-two line was deliberately left unedited
so it would keep reading as standing. A later session meeting this rule with no such record finds a
standing push constraint and no evidence that anyone holding the authority to place it agreed —
which is the state this paragraph exists to prevent. Nothing above was rewritten in consequence; the
ruling confirmed the text rather than changing it.

### (3) The branch the placement cannot reach, decided at scope where the unit set is bounded

Where `<producer>` is the **release run** — tag-triggered, so no mid-iteration push can fire it —
delta 2's placement is unsatisfiable, and the entry splits {design-bearing}.

The scope stage template gains the clause: at promotion, read each `[observed-by:]` entry's
`<producer>`. If a mid-iteration push produces it, delta 2 governs and nothing else is owed. If only
the release run produces it, **split the entry at scope** into a produce half promoted now and an
observe half filed deferred carrying the producer name, so the next iteration's first act is reading
a run that already exists.

This is the recorded shape 3 (*split each into a produce half and an observe half at scope*),
**narrowed from the class to the residue**. That narrowing is the whole cost saving: shape 3 as
recorded prices every member of the class at the cost of its worst member, and delta 2 has just
removed every member a mid-iteration push can reach. Scope is the right owner because it is the one
moment the unit set is bounded and the one session that may write the queue.

### (4) Two recorded shapes are declined on their arithmetic, and the demote workaround becomes the fallback

The entry's candidate set is dispositioned in full rather than left as an open menu a later reader
re-costs {design-bearing}:

- **Shape 1, a two-iteration protocol for the class — declined.** It prices every member at the
  cost of its worst member. Deltas 2 and 3 buy the same guarantee, paying the two-iteration cost
  only on the residue that cannot avoid it.
- **Shape 2, a standing extra push — declined.** It converts a per-entry cost into a standing budget
  change, which is the widest possible fix for the narrowest observed cause. The one grant of that
  shape on record is **iteration-scoped by its own terms**, with a self-discharging oracle and the
  standing line deliberately left unedited beside it. Declining to generalize it is consistent with
  that record and reverses nothing in it; and a budget number is consumer content this kit may not
  set in any case.
- **Shape 4, demote at build once the work has landed — kept, and demoted to the fallback.** It
  stays legal (the demote ritual is already specified) but stops being the normal path, because
  delta 2 makes the observation arrive in-iteration. Its recorded price — *"the grammar cannot mark
  a unit landed-but-unobservable, so a demoted entry reads as unstarted"* — is therefore no longer
  paid on the normal path.
- **The tag that price argues for is declined, and this is the one decline worth stating twice.** A
  `landed-but-unobservable` marker is a marker for a state deltas 2 and 3 stop producing. Buying it
  now would be buying drift: a governed name whose only occasions are the fallback's, maintained by
  every adopter, exercised by almost none. If the fallback is ever measured as the common path, the
  marker is the fix and this paragraph is where a later reader finds that trigger already stated.
- **Shape 5, writing a post-push outcome onto the entry — nothing owed.** Its premise was falsified
  on the record (it spends no push) and this amendment inherits that finding unchanged.

### (5) The refusal that already fires learns to name its own remedy

No new assertion; one message branch {mechanical}.

`check-stage-entry` assertion B already refuses the drain-stage entry that this class produces. What
it does not do is say *why this particular entry could not drain*, and the entry's recorded cost is
precisely that **"the deferral is invisible until the close that cannot drain it."** When the
entries blocking a drain-stage entry include any carrying `[observed-by:]`, the refusal names them
separately and cites delta 2's placement rule and delta 3's two branches.

**The honest limit, stated rather than left to be discovered.** This does not enforce the placement
rule — a push's timing is not a tree state a gate can read, and no arm in this kit can observe that
a push was or was not spent. What the branch buys is that the failure, when it happens, names its
own remedy at the moment it fires instead of costing a fresh derivation. The enforceable half of
this amendment is delta 1's tag hygiene; deltas 2 and 3 are session obligations carried by prompts,
and calling them gated would be the false claim.

## Producers and consumers

**The `[observed-by: <producer>]` tag** is the one new interface; there is no new file, no new
state file, and no new emitted event.

- **Producer.** The **scope stage** writes the tag at promotion, and a filing session may write it
  on a deferred entry at filing (the tag is true of the entry, not of its promotion). Both write it
  by hand into the queue's lead line, the same way every other tag in the algebra is produced —
  there is no `--emit` affordance and none is proposed, because the tag is authored with the entry
  rather than captured from a running session. **Enabling config: none.** The tag needs no knob to
  be live; a consumer that never writes one sees no behavior change anywhere, which is the same
  vacuity `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK` argues from.
- **Consumers, all three named with the transition each reads at.**
  1. `check-tag-lead-line` — reads the tag's **presence and position** at every gate run, to assert
     it sits on the lead line with a non-empty value (delta 1).
  2. The **scope stage session** — reads the tag's **value** at the promotion transition, to select
     delta 3's branch. This is the `<producer>` field's named reader, and it is the only one.
  3. `check-stage-entry` assertion B — reads the tag's **presence** at drain-stage entry, to select
     its refusal message (delta 5).
- **Every new field has a named reader.** The tag carries exactly one field, `<producer>`, and
  consumer 2 above is its reader at a named transition. Stated explicitly because the temptation is
  to add a second field for the run id: **refused** — a run id is produced after the tag is written,
  belongs to the observation rather than to the declaration, and would be a field with no reader at
  the moment it is minted.
- **Readers surveyed across the whole component set, not a hand-picked subset.** The survey ran over
  every tracked surface for `run-observed` (three hits, all prose, all `TASK-QUEUE.md`) and over
  `queue-kit/SPEC.md` §The tag algebra for every existing reader of every tag, with no stderr
  suppressed on any path grep. No cross-component reader exists today, because the class has no
  token today — which is delta 1's whole premise.

**No producer/consumer edge crosses into this iteration's other batch.** Deltas 1–5 touch
`queue-kit`, `lifecycle-kit` and this repo's queue; the platform batch touches `native/`,
`installer/` and `gate-sdk/`. Nothing either batch emits is the other's input.

## Existing sections updated

- `queue-kit/SPEC.md` §The tag algebra — the tag roster gains `[observed-by: <producer>]` with its
  grammar, its lead-line scoping, and the refused `[precondition-ok:]` overload recorded as a
  non-target so a later reader does not retire the new tag as redundant (delta 1).
- `queue-kit/SPEC.md` §check-tag-lead-line — the lead-line set gains the new member (delta 1).
- `lifecycle-kit/SPEC.md` §The state machine — the push-placement rule, stated over the tag and over
  no count (delta 2).
- `lifecycle-kit/SPEC.md` §check-stage-entry — assertion B's refusal-message branch, with its
  honest limit (delta 5).
- `lifecycle-kit/templates/stages/` — the scope stage's promotion step gains the read-the-producer
  clause and its two branches (delta 3).
- `lifecycle-kit/SPEC.md` §Deviation transitions — the demote ritual gains one sentence recording
  that it is the fallback for this class rather than its normal path (delta 4).
- `TASK-QUEUE.md`, the entry itself — compressed on promotion, its five-shape menu replaced by this
  amendment's disposition of it (delta 4). **Not yet applied** as prose; the promotion commit this
  stage lands carries the compression, and build owns nothing further here.

## Retired spellings

- None — no delta of this amendment retires a spelling. `run-observed` survives in three
  `TASK-QUEUE.md` passages that describe the class historically, and delta 1 **adds** a token rather
  than replacing one: `run-observed` was never a formal spelling, only an opaque reason string on a
  retired entry, so there is nothing for a sweep to chase.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls SPEC-*.md lifecycle-kit/SPEC-*.md queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The fixture pair** — `check-tag-lead-line`'s `good/`+`bad/` pair gains the new tag member, so
      the lead-line assertion is exercised on it rather than asserted about it.
