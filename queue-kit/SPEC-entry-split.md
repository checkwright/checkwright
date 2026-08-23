# SPEC amendment: entry-split-criterion

Answers the one deliverable `entry-cap-displaces-mandated-writes` was filed on
and held for: **whether the entries that keep colliding with
`check-queue-entry-budget`'s per-entry cap are the ones that should have been
split.** The entry rules that question a *queue-composition* ruling rather than a
gate change, and rules that a counter with no split criterion behind it would
only make an already-visible cadence visible again. So this amendment authors the
criterion, not an instrument.

**It is a composition ruling and it changes no verdict.** Assertion A's cap, its
number, the recurrence discount, the compression rule and the split's
authorization gate all stand exactly as `queue-kit/SPEC.md`
§check-queue-entry-budget states them today. What is added is the **test the
authorizing session applies** when a blocked session asks to split, and the
**record a split leaves** — neither of which exists on any surface now, which is
why twenty-six measured firings produced relief every time and a composition
ruling never once.

## What changes

### (1) The split criterion: two dispositions, not two topics

`queue-kit/SPEC.md` §check-queue-entry-budget gains a stated authoring test,
beside the compression and relocation reliefs it already owns and subordinate to
the same authorization gate. **Design-bearing.**

> **An entry is a split candidate when it carries two or more deliverables that
> can take different dispositions** — where *different dispositions* means one
> could be promoted, deferred, declined or closed while the other stays open. The
> test is executed by reading the entry's deliverable statements and asking
> whether ruling one leaves the other unruled. If it does, the entry is two units
> wearing one slug, and its grounds are two ground-sets sharing one cap.

**The counter-class is what makes it a class**, and it is stated with the
criterion rather than left to inference: an entry accumulating **further grounds
for one deliverable** — recurrence evidence, a repeated measurement, an answered
objection, a corroborating firing — is **not** a split candidate however hard it
collides. Those are one unit's history. Splitting them mints a ranking peer that
competes with its own parent for the scope attention that ranks both, which is
the displacement shape §check-queue-entry-budget already names as the sharpest
one; compression by answering is the correct relief there and stays the default.

**Why *dispositionability* and not size, age, or ruling-count.** All three were
weighed against the entry's own measurements and each is refuted by them. **Size**
is the cap itself and would make the criterion circular. **Age** is refuted
directly: the 2026-08-22/23 firings record that *every collider was the most
recently ruled-on entry, not the longest-lived*, so the longest-lived reading the
2026-08-19 firings suggested does not survive its own later measurement.
**Ruling-count** is the near miss and the one to state, because it is what the
2026-08-19 reading actually saw — collisions concentrate where rulings accumulate.
It is refused as the criterion because it is a **symptom shared by both classes**:
an entry accumulating rulings on one deliverable collides exactly as hard as one
accumulating rulings on two, and a criterion that cannot separate them licenses
the split that mints a competing peer. Dispositionability separates them and is
decidable by inspection, which is what an authoring contract needs.

### (2) The collision is the criterion's trigger, and the ordering is stated

The criterion is read **at the collision**, by the session assertion A blocked,
and its output is an input to the existing authorization — never a self-serve
licence. **Design-bearing.**

The blocked session states **which side of the test its entry falls on** when it
asks to split, and the authorizing session (the iteration lead, or the operator
in the absence of one) rules with that statement in front of it. The
authorization itself is unchanged and is not weakened by an inch: a session
blocked by the cap still does not mint a new entry on its own authority, for the
reason §check-queue-entry-budget already gives — a new entry is a new unit
competing for scope attention, which is a scope-class judgment.

What changes is the **shape of the ask**. Today a blocked session asks with its
own summary of its own entry and the authorizing session has no test to apply, so
the answer is a judgment call re-derived per firing. That is the mechanism behind
the measured pattern the entry records: relief was always available, so the
question was always answered by the cheapest relief rather than by composition.

**A denied split is not a dead end**, stated so the criterion does not read as
gating relief: an entry that fails the test still has compression by answering and
the self-served relocation, which are the reliefs it should have been using, and
an entry that passes it and is still denied gets the denial's ground recorded on
it as any other ruling would be.

### (3) A split records itself through the citation grammar it already has

When a split is authorized, **the parent names the child and the child names the
parent**, each with the single-backticked slug §The tag algebra already rules a
live citation and `bin/queue-edges.sh` already aggregates. **Design-bearing.**

No tag, no declaration, no new grammar. A `split-of:` declaration is refused on
exactly the ground §check-queue-entry-budget already refuses `relocated:` on: it
would cost a counted line against this very cap, on the two entries least able to
pay it — the parent that just overflowed and the child carrying what would not
fit. The backticked slug costs no line of its own because it rides prose the
split is writing anyway.

**Why bidirectional rather than child-to-parent alone.** The failure a one-way
citation leaves is the one this whole class is about: a reader meeting the
**parent** sees an entry whose deliverable set silently shrank, with no surviving
statement of what left or why, and re-derives the missing half as a gap. That is
the same information loss the compression rule forbids, arriving through the
split instead of through a deletion. `bin/queue-edges.sh` aggregates both
directions, so the pair is readable as an edge rather than as two sentences.

### (4) The criterion's first application is this iteration's own split

`session-mechanic-grants-uncommitted` splits its **unruled** half — the
overlay-only oracles (`cargo test`, `cargo build`/`clippy`, the release binary,
`gh auth`) — into a deferred entry of its own, and the amendment records that
split as the criterion's first worked instance. **Design-bearing.**

It passes delta (1)'s test cleanly and is worth reading as the shape: the entry
carried a **ruled** design (a `PreToolUse` guard rule for the redirect target) and
an **unruled** half (which overlay-only oracles a committed grant should carry),
and the ruled half was promoted while the unruled half stayed unruled — two
dispositions, taken on the same day, on one slug. It is also the instance that
shows the criterion is not retrospective bookkeeping: the split is what
discharged the hold ground that entry carried on 2026-08-22, so applying the test
unblocked a promotion rather than reorganizing a queue.

**Two recorded colliders are ruled the other way, and recording them is what
keeps the criterion a class rather than a licence.** `entry-cap-displaces-mandated-writes`
itself accumulates firings for one deliverable and is **not** a split candidate —
its twenty-six measured firings are one unit's evidence. `scratch-execution-control-is-bash-only`
accumulates measurements for one deliverable and is **not** one either; its fifth
measurement reversed the trend the third rested on, which is compression by
answering working exactly as specified.

### (5) The gate's red text routes to the criterion, and only that

`check-queue-entry-budget`'s assertion-A failure text already cites
§check-queue-entry-budget rather than inlining the split recipe. That citation is
where the criterion now lives, so the routing needs **no change** — and this delta
exists to record that it was checked rather than assumed. **Mechanical.**

No counter is minted, on the entry's own ground. The clean-path headroom line
already exposes every entry's distance to the cap at zero extra computation, so a
firing count would be a second measurement of a quantity the gate already prints,
maintained by hand, against a criterion that reads an entry's composition rather
than its collision history.

## Producers and consumers

**The split criterion** (new authoring contract, no new state and no new field).
*Producer:* the session assertion A blocks — an existing, live trigger, since
assertion A already reds on that session today and its failure text already routes
to the section the criterion lands in. No new configuration enables it and none
can disable it. *Consumers:* two, both existing roles reading an existing surface —
the **blocked session**, which reads the test to state which side its entry falls
on, and the **authorizing session** (lifecycle-kit's iteration lead, or the
operator), which reads that statement plus the test when ruling the ask. *Named
reader of the counter-class clause:* the same blocked session, at the same
transition, which is what stops the criterion reading as a licence.

**The split's bidirectional citation** (no new grammar — an existing one applied
to a new subject). *Producer:* the session that executes an authorized split,
writing one backticked slug into each of the two entries in the commit that makes
the split. *Consumers:* `bin/queue-edges.sh`, which already aggregates
single-backticked slugs into edges and needs no widening to see these; and any
later reader of either entry, at the transition where it asks what the entry's
deliverable set is. *Red condition:* none — no gate reads this, and the amendment
does not claim one does. It is an authoring contract on the same footing as the
compression rule beside it, and §check-queue-entry-budget already states why that
class cannot be gated: nothing distinguishes a sentence that was split off from
one that was dropped.

**No corpus is narrowed by any delta here**, so causal-completeness point 5 has no
reader to enumerate: every change is prose landing in one SPEC section plus two
queue writes. Assertion A's count, extent and discount are untouched, so its
verdict on every entry in the tree is unchanged — which is checkable by running
the gate before and after and is the acceptance for delta (5).

**Existing prose describing the prior flow**, surveyed across the whole component
set rather than a hand-picked subset: `grep -rn` over every tracked `SPEC.md`,
`CLAUDE.md`, `DOCTRINE.md` and template for the split relief and its
authorization finds §check-queue-entry-budget's own two-acts paragraph and
lifecycle-kit's `check-stage-entry` assertion-C waiver, which that paragraph
already names as the split authorization's lineage. Both are inventoried below.

## Existing sections updated

Each names the delta that owns it.

- **queue-kit/SPEC.md §check-queue-entry-budget** — the *Compression is lossless;
  relocation is self-served* paragraph's second bullet, **Splitting the unit**,
  gains the criterion, its counter-class and the three refused alternatives
  (delta 1); the collision-time ordering and the shape of the ask land beside it,
  leaving the authorization sentence itself byte-unchanged (delta 2); the
  bidirectional citation and its refusal of a `split-of:` declaration land beside
  the identical refusal of `relocated:`, so the two refusals read as one rule
  (delta 3). The *why the line falls between the two acts* paragraph is re-read at
  merge and edited only where the criterion makes one of its sentences false; no
  edit is planned, and this clause exists so the check is not skipped.
- **queue-kit/SPEC.md §The tag algebra** — re-read at merge for the citation
  grammar's reader set, which delta (3) adds a subject to and no rule to; edited
  only if it enumerates citation subjects, which it is not expected to
  (delta 3).
- **lifecycle-kit/templates/lead.md** — the lead is delta (2)'s named authorizing
  consumer, and this is the surface that tells it what it is being asked. It gains
  one sentence, and the sentence **cites** the criterion at its owner rather than
  restating it — content-tiering, one content tier per surface: a split ask
  arrives with the blocked session's statement of which side of `queue-kit/SPEC.md`
  §check-queue-entry-budget's split test its entry falls on, and the lead rules
  against that test rather than re-deriving one (delta 2).
- **TASK-QUEUE.md** — `session-mechanic-grants-uncommitted` loses its
  overlay-only-oracle half to a new deferred entry, both entries carrying the
  bidirectional citation delta (3) mints (deltas 3, 4).
- **queue-kit/SPEC.md §check-queue-entry-budget, clean-path headroom** — re-read
  at merge against delta (5)'s no-counter ruling; no edit is planned and the
  clause is here so the no-counter decision is recorded against the surface that
  would have carried one (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`), discharged at the iteration rather
      than at the commit.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The change retires no name, so the grep's
      expected result is empty and running it is what establishes that.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
