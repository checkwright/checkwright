# SPEC amendment: dispatch obligations

Three obligations a session owes when it **backgrounds a child** or **dispatches
into isolation**, all of them gaps in `templates/agent-execution.md`. They share
one shape, which is why they share one amendment: in each case the protocol
already carries both halves of the rule and never states the **composition**, so
a reader who obeys each half literally reaches the failure. The record obligation
composes with the mandated wait primitive into a self-deadlock; the reap
instruction composes with the dispatcher's departure into a six-session orphan;
the isolated child's blindness composes with the isolation *mandate* into a
delegation that cannot be made lawfully and reports clean anyway.

Deltas 1–2 also rewrite a rule that carries sanctioned restatements
(SPEC.md §Operative residency), so delta 5 is not optional tidying — that section
records that an operative restatement propagates a rule's **wording**, and a
defect in the wording becomes N copies of the bug rather than N chances to catch
it.

## What changes

### (1) A waiter registers no producer record, and the protocol draws that split itself

Draw the producer/observer split in the template's own words instead of
delegating it to guard-kit by pointer — a contract between three surfaces whose
boundary decides whether a wedge is reachable, so **{design-bearing}**.

The template's **What you wait *on* splits two ways** branch today obliges a
record of *every* shell child: "a shell child is awaited on the liveness record
you write at its launch". Its own mandated wait primitive — a backgrounded
`until <cond>; do sleep N; done` — **is** a shell child, so a literal reader
records its own waiter. Where that waiter's condition is one the record set can
falsify, the record makes the condition unsatisfiable and the only thing still
blocking the loop is the loop.

The split that repairs it is not new: a **producer** writes artifacts a reader
must not race and owes a record; an **observer** writes nothing and owes none.
guard-kit already draws it mechanically — rule 15's exemption (2) exempts an
inline wait loop and exemption (3) a read-only pipeline, and that rule's
corrective already "closes on the two shapes that owe no record". The template is
the one surface out of step, and it stays out of step because it **delegates the
question by pointer**: its *Launch and record in one call* clause hands guard-kit
"the shapes that owe no record" and never says what they are. A reader of the
template alone therefore cannot derive the exemption the guard would grant it.

Replacement text for the branch's obligation sentence — **Not yet applied**:

> A **shell child** has no notification channel of its own, so it is awaited on
> an artifact *the session placed*. Which artifact depends on what the child
> **is**. A **producer** — a child that writes anything a later reader must not
> race — is awaited on a **liveness record you write at its launch**, one line
> `pid=<n> run=<key>` in a file named `<key>.run`, in repo-local gitignored
> scratch in the main checkout. An **observer** — a wait loop, a read-only
> pipeline, anything that writes nothing — **writes no record**, because a record
> states that something is mutating shared files and a waiter is not. Registering
> a waiter is not a harmless surplus: while a record names a live PID it blocks
> every tracked-tree mutation in *every* session, so a waiter that records itself
> can make its own exit condition unreachable and wedge its concurrent siblings
> at the same time.

And for the clause that delegated the question, replacing the pointer with the
rule plus the pointer — **Not yet applied**:

> **Launch and record in one call, in the spelling the guard grants**:
> guard-kit's rule *Backgrounded launch that records no producer*
> (guard-kit/SPEC.md §The generic ruleset) owns the spelling and enforces the
> same producer/observer split, exempting an **inline** wait loop and a
> read-only pipeline from the record. Spell a wait inline so it meets that
> exemption; a wait behind a script or binary name is invisible to the guard's
> span walk and takes the record like any launch, which is what delta 2 is for.

Grounds land in SPEC.md §The delegation model, beside the liveness clause that
already rules *how* a PID's liveness is decided is evidence-kit's and *when a
session waits and on what* is this kit's — the producer/observer split is the
second half of that same ownership statement and belongs with it.

**The two alternatives are ruled out, not unconsidered.** Teaching
`check-producer-liveness` to ignore a record whose run key names the stage being
entered fixes one consumer of three: the attested wedge reached guard-kit rule
14's tracked-tree block and the `SubagentStop` turn-end hook as well as the
stage-entry preflight, and a carve-out in one reader leaves the other two
holding the same record. Refusing the self-naming record at write time is
guard-kit's surface and has already shipped there as exemption (2); restating it
as a second enforcement here would be a second implementation of a decided
thing.

### (2) A wait that must take a record owes the self-deadlock test

State the residual the guard's own exemption cannot reach, as an authoring check
whose boundary is the whole content — get it wrong and the wedge re-opens — so
**{design-bearing}**.

Exemption (2) sees a wait loop only when it is spelled **inline**; guard-kit
states that limit itself and accepts it, on the ground that "the record costs a
waiter nothing it does not already carry: its only effect is that rule 14 holds
tracked-tree mutations for the waiter's lifetime, which the producer it waits on
already holds." That ground is sound for a waiter waiting on a **producer** and
unsound for a waiter waiting on anything else. The attested instance waited on a
**stage-entry precondition**; there was no producer holding anything, and the
waiter's own record was the only thing blocking the condition.

So the residual gets a test rather than a second mechanism. New text, placed
immediately after delta 1's replacement — **Not yet applied**:

> **A recorded wait owes one check before it starts: can my own record falsify
> my condition?** A wait that must take a record — one spelled behind a script
> or binary name, invisible to the guard's span walk — is only safe where its
> condition is independent of the record set. A condition that *reads* the
> record set, directly or through a gate that does (a stage entry, a commit, any
> tracked-tree write), is falsified by the waiter's own record and can never go
> true. Two lawful answers: respell the wait inline so it owes no record, or
> wait on the artifact itself rather than on the gate that reads the record set.

This is an authoring check with no oracle behind it, and that is stated rather
than hidden: the guard cannot read a wait's condition at `PreToolUse`, and the
wedge's signature — a loop that never exits — is indistinguishable from a
producer that is genuinely slow.

### (3) The reap is owed at the dispatching session's own turn end

Move the reap from the iteration boundary to the dispatcher's own turn end —
an obligation crossing a boundary a standing ruling sits near, so establishing
its clearance of that ruling *is* the work, and **{design-bearing}**.

The template today sends the reap to a boundary: "reap agents at the boundary
with `git worktree list`", with the iteration-boundary refusal named as the
enforcement. Nothing asserts the reap at the **dispatching** session's own turn
end, so an orphan minted mid-iteration survives every later session until the
boundary meets it. Attested: an orphan minted at one iteration's spec stage
survived align, three build sessions, validate and close without any of six
sessions noticing, and only the boundary refusal surfaced it. Those five later
sessions had no reason to look, and the party who knew what the worktree was for
was gone six sessions before the refusal fired.

Replacement text for the reap step — **Not yet applied**:

> Gitignore the path, and **reap what you minted at your own turn end**, by
> `git worktree list` rather than off `git status`: once ignored, the status is
> clean while the worktree still stands. Your turn end is the earliest moment
> the reap is *sound* — reclamation is tied to the child's own return, so by
> then every child of this turn has returned and the harness's auto-clean has
> either fired or failed — and the last moment it is *informed*, because the
> only party who knows what a tree was for is the session that minted it. Reap
> **both** halves: `git worktree remove` clears the directory and leaves the
> agent's branch ref standing, so delete that ref in the same motion, or the
> refusal clears while the refs accrete unseen. Reap **your own** trees, never
> every tree you can see: a concurrent session's checkout is live work, and the
> iteration-boundary refusal — which refuses to enter while any linked worktree
> exists (lifecycle-kit/SPEC.md §bin/enter-stage.sh) — stays the **backstop**,
> not the schedule.

**This is not a reversal of the dispatch-time-sweep ruling, and the clearance is
the delta's load-bearing half.** SPEC.md §The delegation model refuses a sweep
**in the dispatch guard**, on two grounds: a verdict surface may not own
lifecycle, and a dispatch-time sweep runs while sibling dispatches are in
flight, so its predicate would have to establish that a worktree belongs to no
live child and a wrong guess destroys a running sibling's checkout. Neither
ground reaches this delta. The subject here is a **session**, not a verdict
surface, so the first is silent. And the timing is the inverse of the refused
one: a dispatcher at its own turn end is past every child it started this turn
and is scoped to trees it minted, so it needs no predicate about a sibling's
liveness and takes no action outside its own set. The refused ruling's own
description of why the iteration boundary is safe — it runs at a moment whose
definition is that the work is finished — is the property being moved earlier,
not weakened.

**Registering the worktree for a later sweep is ruled out.** `git worktree list`
is already the registry; what is missing is not discoverability but attribution
and judgment, and a record written by the dispatcher shares the reap's own
structural uncheckability — a session that skips the reap writes no record
either, so there is no absence a check could have been told to expect
(SPEC.md §Operative residency's own reasoning for the launch record).

**No new oracle, and the reason is the predicate rather than the budget.** The
act is a turn end. `SubagentStop` does see a dispatched session's turn end and
the turn-end liveness hook already refuses there — but a worktree carries no
attribution to the ending session without the consumer's optional lock-reason
configuration, so a refusal keyed on `git worktree list` would refuse a
dispatcher for a concurrent session's live tree, at the most expensive moment to
be wrong. What this delta buys is **latency**, not a newly caught class: the
existing boundary refusal already catches the harm, measured at six sessions of
carry, and the obligation shortens that to one turn.

### (4) An isolated child's blindness composes with the isolation mandate

Rule where the blindness constraint is stated, and state it — a choice between
two named candidate owners that this delta answers with a third, so
**{design-bearing}**.

Isolation cost (3) already states the parent-side rule: untracked and gitignored
files are in no commit, so "a sweep whose corpus includes an untracked surface
is not delegable to an isolated agent — read that surface yourself, or pass its
content in the prompt." Two things are missing, and each is the half that turns
a caveat into a trap.

**First, the composition with D2.** Cost (3)'s phrasing reads as *use a
non-isolated agent instead* — and for a type in `DELEGATION_KIT_READONLY_TYPES`
that is exactly what the dispatch guard's D2 rule forbids, blocking any such
dispatch without `isolation: worktree`. The two rules are both this kit's and
their composition is stated nowhere, so the sweep is not delegable **at all** to
a read-only type and the protocol never says so. Attested: a close-surface
roster read delegated under that mandate reported "absent" for four surfaces
that carried content, and a close that trusted the numbers would have cleared
nothing, dispositioned four surfaces as absent, and reported clean.

**Second, the missing child-side clause.** The sibling cost (4) tells the child
what to do when isolation makes a gate unreadable: an unavailable verdict is the
expected reading, not a defect to repair — "name the gate that could not run,
say why, and return". Cost (3) has no such clause, so an isolated child asked to
read a gitignored path reports **absence** as a finding, which is the exact
shape a parent cannot tell from a true empty.

Replacement text for cost (3) — **Not yet applied**:

> **(3) An isolated child sees only committed state.** Untracked and gitignored
> files are in no commit, so no base ref reaches them and naming a rev does not
> help. **A sweep whose corpus includes an untracked or gitignored surface is
> not delegable to an isolated agent**: read that surface yourself, or pass its
> content in the prompt. **For a type the dispatch guard's D2 rule holds to
> isolation, that means not delegable at all** — D2 blocks such a dispatch
> without `isolation: worktree`, so the escape this rule otherwise offers,
> dispatching the same sweep unisolated, is the one shape that cannot be
> spelled. Classify the corpus before you dispatch rather than after you read
> the answer; `git check-ignore` decides it. **And the child's side of it, on
> cost (4)'s pattern:** a target absent inside isolation because it is untracked
> or gitignored is a **blindness**, never a finding of absence — name the path,
> say it was unreadable at this rev, and return. A parent cannot tell a reported
> absence from a true empty, so the distinction has to be drawn where it is
> known.

**Two candidate owners were weighed and both refused.** *At the dispatch guard*:
the payload carries `subagent_type`, `isolation` and prompt text, and never the
sweep's corpus — so a guard rule would have to key on whether the prompt "names
a path", which D3's own design already rules out as firing on the word and
teaching dispatchers to game the wording, buying a green hook and no channel.
*At the close-surface roster emitter*: it would work — the derivation already
takes a `git check-ignore` verdict per path and drops it before printing — but it
would repair one instance of a general class and leave the class open, since a
delegable sweep's corpus is frequently not that roster. The constraint belongs
where the dispatcher reads its protocol. **The roster-side print is real and is
filed rather than taken** — to the gap inbox at this stage, for a later scope to
cost and promote — so the narrower repair stays available without being smuggled
into this unit's surface, which its own tag bounds to this component.

### (5) The changed wording reaches every restatement carrier

Propagate deltas 1, 2 and 4's changed wording into every restatement carrier —
the target is symmetric even where the starting text is not, so each carrier
takes an addition sized to its own voice rather than a paste and condition (b)
bars importing any of the grounds above, which is judgment rather than a merge
sweep and therefore **{design-bearing}**.

Deltas 1 and 2 rewrite the **Background + notification, never poll** rule, which
SPEC.md §Operative residency names as an attested restatement subject; the
template's own header carries the standing *when either rule changes here,
propagate* obligation. Delta 4's **child-side** clause additionally qualifies for
residency on its own (a)–(c) reading: it binds a dispatched read-only agent,
which fires no trigger that loads the template, and the carrier already states
cost (4)'s sibling clause with an adjacent citation, so the new one lands beside
it.

Delta 3's obligation is **deliberately not propagated**, and that is a ruling
rather than an omission: it binds a session that **has dispatched**, and such a
session loads the template through its own trigger, so condition (a) fails and
the rule stays template-tier — the same reading §Operative residency records for
the provenance floor.

The carriers are enumerated by probe in the roster below, and the roster is a
floor: §Operative residency records that N is discovered by grepping the rule's
phrasing rather than by consulting a list of known carriers, so the merging
session re-runs the probe rather than trusting this enumeration.

## Producers and consumers

**These deltas mint no state, no event, no message and no name.** Every change is
an obligation on a session, landing on surfaces that already exist, read by
readers that already read them. Point 4 of the causal-completeness check is
therefore vacuous here and is recorded as vacuous rather than skipped: no field
is added, so none can lack a reader.

**Producer of each obligation, and the enabling configuration.**

- Deltas 1 and 2 fire at the moment a session backgrounds a shell child — the
  template's own mandated wait primitive is the most frequent instance. Enabling
  config: none needed; the guard-side exemption they align with
  (guard-kit rule 15) is registered in this tree's `PreToolUse` hook, which is
  live rather than test-only.
- Delta 3 fires at a dispatching session's turn end, on a session that took
  `isolation: worktree` during that turn. The isolation path is live here and
  mandated for `DELEGATION_KIT_READONLY_TYPES` members, whose roster is
  non-empty in this consumer (`scripts/delegation-config.knobs:4`,
  `audit-sweep`) — so the producer is reachable in deployed configuration and
  not only in fixtures.
- Delta 4 fires twice: at the parent, before a dispatch whose corpus it must
  classify; and at the child, when a target is unreadable at its rev. Same
  enabling roster as delta 3.

**Consumers, by mechanism.**

- The **reading session** consumes all five through the template, loaded by the
  `/agent-execution` trigger, and consumes deltas 1, 2 and 4's child-side clause
  through the two agent definitions delta 5 writes — read at dispatch by the
  harness, with no trigger to fire.
- `check-producer-liveness`, guard-kit rule 14, the `SubagentStop` turn-end
  liveness hook and the stage-entry preflight consume delta 1's output — the
  `.run` record set — unchanged. No grammar moves; what changes is which
  sessions write a record at all.
- The **iteration-boundary refusal** (lifecycle-kit `bin/enter-stage.sh`)
  consumes delta 3's effect as a reduced incidence of linked worktrees at entry.
  Its predicate is untouched.
- **Roster-holding readers of the surfaces these names land on**: none is
  obliged. No delta mints a name that a roster must carry — no gate, no knob, no
  path, no tag, no comment directive on a governed surface. The one generated
  roster in reach is the on-site SPEC mirror, which is a byte projection of the
  edited files rather than a name registry, and it is carried as an update
  target below.

**Point 5 — the narrowing, and each reader's red condition.** Delta 1 **narrows**
the record-writing corpus: a class of shell child that writes a record today
writes none after it. Point 5 binds, and the three shapes it warns about were
checked against the readers rather than cleared by inspection.

- `check-producer-liveness` in set mode over a **record set that is empty** exits
  **0, green** — probed this session, `bash gate-sdk/bin/run-gates.sh --only
  check-producer-liveness -- <empty-dir>`. It reds on a live producer, never on
  finding none; it asserts no exact count and holds no coverage floor.
- **guard-kit rule 14** blocks a tracked-tree mutation *while a record names a
  live pid*. Fewer records can only mean fewer blocks; it has no
  red-on-finding-none arm.
- **The turn-end liveness hook** refuses on `red`, `corrupt` and `unresolved`.
  `unresolved` is the one arm that involves an empty record set, and its
  predicate is *reader exit 2* over an empty set — a reader that could not run at
  all — never an empty set as such, so a green reader over no records exits 0.
- **The stage-entry preflight** is the same reader over `.tmp`, so it takes the
  first row's verdict.

No reader reds on finding none, asserts an exact count, or holds a minimum, so
the narrowing is safe on all four.

**Point 6 — the enumerable corpus, and each member's satisfying value.** Delta 5
obliges each member of the restatement-carrier set. The set is enumerated by
probe, not by memory:
`git grep -ln 'shell child is awaited' -- ':!docs/'`, run this session over the
tracked tree, which returns exactly two members.

- `.claude/agents/stage-session.md` — satisfying value: its wait paragraph
  states the producer/observer split of delta 1 and delta 2's self-deadlock
  test, as bare imperatives with the adjacent citation (c) requires, and carries
  no delta-3 clause.
- `.claude/agents/audit-sweep.md` — satisfying value: the same two, in that
  file's own voice; **plus** delta 4's child-side clause, beside the cost-(4)
  sentence it already carries.

Both members have a satisfying value, so the assertion needs no narrowing. The
probe's own output is the floor, not the proof — delta 5 says why.

## Existing sections updated

Probe for this roster:
`git grep -ln 'shell child is awaited' -- ':!docs/'` and
`git grep -ln 'worktree list|worktree remove' -- ':!docs/'`, both run this
session over the tracked tree, plus a read of
`delegation-kit/templates/agent-execution.md` and
`delegation-kit/SPEC.md` §The delegation model / §Operative residency end to
end. The roster is a floor the merging session re-derives.

- `delegation-kit/templates/agent-execution.md` — the **What you wait *on*
  splits two ways** branch of the *Background + notification, never poll* rule,
  and its *Launch and record in one call* clause (deltas 1 and 2).
- `delegation-kit/templates/agent-execution.md` — isolation cost **(2)**, the
  reap step, whose "reap agents at the boundary" instruction delta 3 replaces.
  (delta 3)
- `delegation-kit/templates/agent-execution.md` — isolation cost **(3)**, in
  full (delta 4).
- `delegation-kit/SPEC.md` §The delegation model — the liveness-clause
  paragraphs that rule what delegation-kit owns about waiting take the
  producer/observer split and the self-deadlock residual as grounds; the
  worktree-reap paragraphs take delta 3's clearance of the dispatch-time-sweep
  ruling; the D2 paragraphs take delta 4's composition and its two refused
  owners. (deltas 1, 2, 3 and 4)
- `delegation-kit/SPEC.md` §Operative residency — the propagate obligation fires
  a sixth time and the section records what it cost, including the ruling that
  delta 3 is *not* a residency subject. (delta 5)
- `.claude/agents/stage-session.md` — the wait paragraph (deltas 1 and 2).
- `.claude/agents/audit-sweep.md` — the wait paragraph (deltas 1 and 2) and the
  read-the-governing-surface bullet carrying the cost-(4) citation (delta 4).
- `docs/delegation-kit/SPEC.md` and the on-site mirror's sibling pages — a
  generated projection, stale the moment any delta lands; regenerate with
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`
  (docs/site-architecture.md §Generated projections and their freshness gates).
  (all deltas)

## Provenance seam

**What ships as kit mechanism.** All five deltas. Each is generic: the
producer/observer split, the self-deadlock test, the turn-end reap, the
blindness composition and the propagate obligation are statements about any
session that backgrounds a child or dispatches into isolation, and none names a
product, a term list, a vocabulary or a constant set. Deltas 1–4 land in
`templates/agent-execution.md` and `SPEC.md`; delta 5 lands in the consumer's
own surfaces and is the kit stating an obligation, never the kit shipping the
text that discharges it.

**What must not ride into the kit SPEC at merge, and it is named here because
the merge is where it would slip.** Three consumer-specific facts appear above
because the causal-completeness check demands them and cannot be answered
generically:

- `scripts/delegation-config.knobs` and the roster value `audit-sweep`, cited
  under point 1 to show the producer is reachable in *deployed* configuration
  rather than in fixtures alone. The kit SPEC states the rule against
  `DELEGATION_KIT_READONLY_TYPES` as a knob whose roster is the consumer's, on
  the standing config-via-env convention; the value is a consumer fact and stays
  one.
- The two `.claude/agents/` paths, enumerated under point 6 because the
  obligation is over a corpus and the check requires each member's satisfying
  value be named. `SPEC.md` §Operative residency already refers to such carriers
  as *this consumer's* definitions and is the shape to match; a kit literal
  naming these two files would publish this consumer's agent roster and would be
  wrong for every other adopter besides.
- The attested sightings, which are stated **undated and unattributed** above on
  purpose. Their dates, the sessions that met them, and the operator ruling that
  scoped delta 3's parent unit out of an earlier envelope are provenance: they
  live on the queue entries and in git history, and the kit SPEC states each
  rule undated.

**What becomes consumer config.** Nothing new. No delta mints a knob, and none
is owed: the only configurable input any of them reads is
`DELEGATION_KIT_READONLY_TYPES`, which already exists and whose empty-roster
degradation is already ruled — with it empty, delta 4's *not delegable at all*
clause is simply never reached, which is the correct inert behavior and needs no
second knob to express.

## Retired spellings

- None — no delta of this amendment retires a spelling. The deltas replace
  instruction prose and mint no name; the phrases they rewrite ("reap agents at
  the boundary", "a shell child is awaited on the liveness record") are sentences
  rather than tokens, and each is rewritten in place at the single surface that
  carries it plus the carriers delta 5 enumerates, so there is no disjoint token
  space for a survivor scan to reconcile against.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Carrier probe re-run before the merge counts as complete** — the roster
      above is a floor; `git grep -ln 'shell child is awaited' -- ':!docs/'`
      re-run at merge, and any member it returns that this roster does not name
      is a carrier the roster missed.
