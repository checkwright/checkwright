# SPEC amendment: budget-arm

The port tail has no shared derivation left to compose an iteration around, and
nothing states what composes one in its absence. This amendment lands the
**third selection arm** — the budget arm — beside the size arm and the
blocker-retiring override, and rules which surface owns it.

**The composition question was ruled lead-class on 2026-08-16 and this amendment
authors it rather than re-decides it.** The ruling and its grounds are recorded
in `port-tail-cohort-batching-policy`; the guardrail carried with it binds this
file. Nothing here narrows or rewrites recorded operator-ruled text: the
`libs=fail_closed globs=-` group stays not-a-cohort (§The canonical-spec
`spec_canonical_specs` cohort, which records that rejection and its
operator ruling of 2026-08-14), the shared-derivation rule keeps selecting
wherever it still selects,
and the arm serves the 2026-08-09 ASAP directive rather than re-scoping it.

**What was left open, and is what this amendment settles: the placement.** The
sizing half arrived with the ruling. Which surface owns the rule did not, and it
is the half the provenance seam bears on — a batching rule stated as generic
mechanism is kit content, while a rule naming this project's port remainder is
this repo's rule content.

## What changes

### 1. §The first cohort, and the rule that selects the next gains a third arm

The section states an ordering rule with two arms — largest criteria-clearing
set sharing one corpus derivation, and the documented blocker-retiring override
that outranks it. A **third arm** joins them, additive, and stating its
precedence is the delta rather than stating the arm alone — **design-bearing**.

The arm: *where neither the size arm nor the blocker-retiring override selects,
an iteration composes the port increment by **budget** — N criteria-clearing
members, taken as N independent units.*

Precedence is total and is read off a run rather than a preference. The size arm
selects first. The blocker-retiring override outranks it where a blocker several
members are queued behind can be retired. The budget arm is reachable **only**
when a `bash gate-sdk/bin/port-blockers.sh --group` run reports no takeable
group — so the arm's precondition is a verdict from the instrument the section
already names for the size arm, never a session's reading of the tree.

### 2. "Never as one cohort" is stated as the property that makes a batch safe

The ruling's clause is easy to read as bookkeeping. It is the safety property,
and the section says so — **design-bearing**.

A budget batch's members carry **no joint proof**. Each takes its own `.gate`
descriptor, its own registry entry, its own fixture-pair parity run and its own
live-tree and edge-root arms. No shared walk is claimed, so no comparison spans
two members. What follows is the property worth having: **dropping a member
mid-batch invalidates nothing**. A member that turns out dearer than it was
sized leaves the batch and the rest land unchanged, where a cohort losing a
member loses the amortization it was composed on and is re-planned.

A batch is therefore never recorded, argued or merged as one unit of work. The
plural is the mechanism, not the wording.

### 3. The batch size is a budget, and the budget is deliberately not a knob

The unit's own class note offered a knob as the feature-making move. It is
refused, with grounds, so a later session does not read the absence as an
oversight — **design-bearing**.

Config-via-env exists for configuration that varies **by consumer**. A batch
size varies by **iteration**, against that iteration's other work: it is a
judgment the composing session makes with the queue in front of it. A knob would
freeze a default that reads as the answer, and the next session would size
against the default rather than against its budget — which is precisely the
defect the unit was filed on (*"the sizing is decided identically and invisibly
every iteration"*), reintroduced as configuration and harder to see for being
declared.

What the sizing session weighs instead, stated once so it is not re-derived: the
per-member cost `--group` already prints beside each member (shell line count
and the mechanically derivable criteria columns), the iteration's non-port work,
and the fixed per-iteration ceremony the unit measured.

### 4. A budget batch records only findings, never a member roster

Every prior cohort earned a `###` section here because each had a shared
derivation to explain. A budget batch has none, so the default record is
**nothing** — **design-bearing**.

The rule: *a budget batch adds a section to this SPEC only where it has a
finding to record* — an adjudication a later selector would otherwise re-make, a
primitive it landed, a criterion it discharged. Membership and progress are
**derived**: a ported member leaves a `.gate` beside its deleted `.sh`, and the
count is `scripts/measured-claims.sh`'s `ported-gate-members`, which
§check-measured-claim already holds to the tree.

Without this rule the section count scales with the tail — roughly one section
per batch across the remaining singletons — and the fixed ceremony overhead this
unit was filed against gets paid a second time, in prose, on the one surface a
porting session must read end to end.

### 5. The placement is ruled, and both rejected homes are recorded with grounds

The unit named three candidate owners and the ruling picks one. Recording why
the other two lost is what stops the next session re-opening it —
**design-bearing**.

**Rejected — lifecycle-kit's scope template.** It owns the general
economic-composition test, and a port-specific selection arm placed there would
bind every consumer's iteration including the consumers who run no port, and
would make the general iteration contract carry a term only gate-sdk can define.
The reach it would have been bought for is already bought:
`native-gate-port-remaining-corpus` names §The first cohort as canonical for
every cohort, so a scope session composing a port iteration reaches the arm
along the path it already walks. Placement here would buy reach that exists and
pay for it in a downward dependency.

**Rejected — TRAJECTORY.md, and by that file's own contract rather than on a
preference.** It records rulings the **operator** closed and never authors one
(TRAJECTORY.md §Who may record a ruling here: *"A session records a ruling the
operator closed; it never authors one"*). This ruling is lead-class. Recording
it there would be a session attesting to a consent it does not hold, which that
file names as the thing worth less than the rule. Its pointer-first convention
points the same way: a ruling whose mechanism has a canonical home is registered
there, and the home is the section taken below.

**Taken — gate-sdk/SPEC.md §The first cohort, and the rule that selects the
next.** It already owns the two arms this one joins, it is the surface a
selecting session already loads, its reach is exactly the port and nothing
wider, and it is kit content, so the rule ships as mechanism to every consumer
who ports gates.

### 6. The seam holds because the arm names no remainder

The arm is stated generically — no gate names, no member roster, no count of
this tree's remaining corpus — **design-bearing**.

This project's figures stay where they already live: the queue entry and
`.workflow/survey-record.md`. A kit literal naming this tree's singleton set
would publish one project's work queue as everyone's mechanism, the same defect
the `check-graph` / `graph-vocab.sh` split exists to prevent.

### 7. `native-gate-port-remaining-corpus`'s promotion cadence reads per increment

That entry says it is *"promoted at spec and demoted at build once per cohort"*.
Under the budget arm an iteration's port increment may be a batch rather than a
cohort, so the cadence term becomes the increment whatever composed it —
**mechanical**.

## Producers and consumers

This amendment introduces **one selection arm and nothing else**: no field, no
message, no file, no tag, no knob, no script, no descriptor. The checklist is
answered against that, point by point, rather than skipped as inapplicable.

- **Producer, named and reachable.** The session composing a port iteration —
  scope when it ranks the queue, and the authoring stage when it writes the
  batch amendment. Its **trigger** is a `bash gate-sdk/bin/port-blockers.sh
  --group` run whose verdict is *no takeable group*. The enabling condition is
  live rather than test-only: that run is the same one §The first cohort already
  names as the size arm's instrument, so the arm's precondition is read off a
  command this repo runs at every cohort cut, not off a new one that some
  configuration would have to start emitting.
- **Consumer, named, with the mechanism.** The same session, by reading
  gate-sdk/SPEC.md — reached through `native-gate-port-remaining-corpus`, which
  names §The first cohort as canonical for every cohort. Second consumer: the
  iteration lead at batch-cut, which reads the batch amendment's per-delta
  work-class labels; the arm is what tells it a batch's deltas are independent
  and may be cut apart, where a cohort's may not.
- **Every new field has a named reader.** There is no new field, and this is
  stated rather than left inferred so a later reader does not go looking for a
  machine reader that was never intended. The arm is an authoring contract, like
  the two arms it joins — neither of those has a machine reader either, and
  §check-queue-entry-budget's own *"the gate cannot hold this"* paragraph is the
  house form for saying so.
- **Narrowing? No.** No delta prunes a file, tightens a glob, or drops a scan
  root, so causal-completeness point 5 does not bind and no reader's red
  condition can flip by inspection. Recorded as a verdict rather than a silent
  skip, because the point's whole warning is that *"a narrower corpus can only
  remove violations"* is the first argument a narrowing delta reaches for.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  deltas 1 through 6. The paragraph opening *"The next cohort is the largest set
  of criteria-clearing gates sharing one corpus derivation"* currently states the
  ordering rule as though the size arm and its blocker-retiring override were
  exhaustive; it gains the third arm and the precedence between the three. The
  sentence *"Shared derivation is the axis because it is what made this cohort
  cheap"* keeps its ground and gains its bound — it is the axis **while it
  selects**, and the arm below is what the section says when it stops.
- **TASK-QUEUE.md, `native-gate-port-remaining-corpus`** — delta 7, the cadence
  term. Named here because it is prose describing the prior flow, and leaving it
  saying *per cohort* while the arm composes batches is the drift this section
  exists to catch.
- **No other section, and each is a verdict rather than an omission.** §The
  port-candidate criteria is untouched: the criteria order the work within an
  increment, the arm composes the increment, and the 2026-08-09 ruling already
  settled that no criterion is an eligibility screen. §Meta-gate conservation for
  the binary substrate is untouched: it binds a port's members, never the
  iteration's composition. §port-blockers is untouched: the arm reads its
  existing `--group` verdict and asks it for nothing new.

## Definition of Done

- [ ] **Causal completeness** — the arm's producer (the composing session, on a
      `--group` verdict of no takeable group) and its consumers (that session,
      and the lead at batch-cut) are named in §The first cohort itself; the
      no-new-field verdict is stated there rather than left to this file.
- [ ] **Merged with no information lost** — deltas 1–6 land inside §The first
      cohort in its own voice, integrated with the two existing arms rather than
      appended after them; delta 7 lands in the queue entry.
- [ ] **Amendment deleted** — this file removed on merge; `ls gate-sdk/SPEC-*.md`
      checked, with the iteration horizon in mind (a sibling amendment in flight
      for this component means only the batch merging the last one can satisfy
      the none-remain half).
- [ ] **Removals propagated** — nothing is retired by this amendment; grepped to
      confirm, so the absence is a verdict.
- [ ] **Gaps filed** — cross-component gaps discovered while merging filed
      through `bash lifecycle-kit/bin/file-gap.sh`, and a build-time causal gap
      resolved that session rather than deferred.
