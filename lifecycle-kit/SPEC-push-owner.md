# SPEC amendment: push-owner

Two adjacent entries, kept distinct by operator direction and paired to one
amendment because both rewrite one paragraph. `push-budget-unshipped` covers the
push count: this repo's budget lives only in its CLAUDE.md, where no kit surface
reads it and no adopter is prompted to set one. `mid-iteration-push-owner-unnamed`
covers the push actor: §The state machine places an iteration's first push early
but names nobody to make it.

**The count's seam is already ruled, so this amendment applies it.** §The state
machine: "*How many* pushes an iteration may spend is consumer content and this
rule names no number", and "no arm in this kit observes that a push was or was not
spent". The entry's candidate was two lifecycle-kit knobs with this repo's values as
defaults. That is refused on those two sentences. A knob no arm reads fails
canon-kit's causal-completeness point 1: a producer with no consumer is config
whose only reader is `check-knob-citation`, the case §Layout and configuration
already refuses for the ruling-authority vocabulary. A default carrying this repo's
numbers would also ship one consumer's policy as the kit's. The count's reader is a
session, so it is a **template slot**. The same test gives `drain-inputs` a slot and
not a knob (§templates/stages/).

**The slot sits on close's push step.** Close is the stage that makes the
iteration's closing push and any hotfix push after a red one, so it is the one
session that spends the count. A mid-iteration push under the placement rule
re-places a push rather than buying one, so it needs no count.

**The actor is the session that reads the run.** A push exists to produce a remote
run. The session that needs that run is the one that must wait for it and read it:
for an `[observed-by:]` entry, the session landing the entry's work, whose Done move
is the observation; for a stage using a remote run as its oracle, that stage. The
lead is excluded. It writes no lifecycle state, and a push publishes that state.
The entry's other candidate, "validate as the owner", is refused as a fixed stage.
The `[observed-by:]` placement rule already puts the push *at or before the stage
that lands that entry's work*, and a fixed validate owner would contradict it
wherever build lands the work.

**Probed at authoring, 2026-09-19.** `grep -n -i "budget\|hotfix"
lifecycle-kit/templates/stages/close.md lifecycle-kit/templates/lead.md` returns no
push count. `grep -n "LIFECYCLE_KIT_PUSH" native/src/knobs/lifecycle_kit.rs` returns
nothing. `grep -n -i "push" CLAUDE.md` returns the budget sentence at lines 37-44.
No lifecycle surface names a push actor: `grep -n -i "owns the push\|who pushes\|makes
the push" lifecycle-kit/SPEC.md lifecycle-kit/templates/*.md
lifecycle-kit/templates/stages/*.md` returns nothing.

## What changes

### (1) close's push step gains the `push-budget` slot {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/close.md` step 13, after
"**No gate substitutes for this.**", append the slot:

> *<push-budget: how many pushes this consumer's iteration may spend, how many
> hotfix pushes a red push earns before the operator is asked, and the remote run a
> push is watched to — or a plain "no remote — nothing is pushed" line.>*

### (2) This repo binds the slot and CLAUDE.md points at it {mechanical}

Lands in the same commit as delta 1, because `check-skill-binding` reds an unbound
slot. **Not yet applied.**

- `.claude/commands/close.md` gains a `**push-budget** —` binding. It carries the
  numbers CLAUDE.md states today: one to two pushes per iteration, up to two hotfix
  pushes for a red push unasked, a third asked for. Each push is watched to green
  on the `gates` workflow.
- In `CLAUDE.md` §This repo is governed by its own kits, the clause "— and **budget
  one to two pushes per iteration**, plus up to two hotfix pushes for a red push,
  unasked; a third hotfix push is asked for." becomes "— within the close binding's
  `push-budget` (`.claude/commands/close.md`)." The surrounding sentences stand:
  the remote-oracle watch, reading a finished run for free, and the drip-push cost.

### (3) §The state machine names the push actor {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §The state machine, the paragraph
opening "**The seam is held deliberately.**" is re-phrased to carry:

- The count is consumer content, read by a session, and therefore close's
  `push-budget` slot rather than a knob (the ground above, briefly).
- **The actor:** a push is made by the stage session whose next act reads the run it
  produces. For an `[observed-by:]` entry that is the session landing the entry's
  work, which pushes after its landing commit, waits in-turn for the run, and reads
  it before the Done move. A stage using a remote run as its oracle pushes for
  itself. The lead never pushes (§The optional lead never becomes a second state
  source). Every push, at any stage, runs the consumer's per-push identity
  precondition that close's push step names.
- The unenforceability sentence stands, since the actor is no more a tree state than
  the timing.

### (4) The lead template states it makes no push {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md` §Stamps are authoritative,
the first paragraph's list "no WORKFLOW-STATE stamps, no queue writes, no evidence
files" becomes "no WORKFLOW-STATE stamps, no queue writes, no evidence files, no
push — a push is the stage session's whose next act reads its run
(lifecycle-kit/SPEC.md §The state machine)".

### (5) build lands an observed entry by pushing and reading {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/build.md`, append to the
paragraph opening "**Run the system; don't reason about it**":

> An entry carrying `[observed-by:]` is complete only when its run is read: after
> its landing commit, run close's push precondition, push, wait in-turn for the
> run, and read it before the Done move (lifecycle-kit/SPEC.md §The state machine).

### (6) §templates/stages/ records the slot {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §templates/stages/, the paragraph
"The `close` template's **push-identity precondition** is per-push …" is re-phrased
to add that the step carries the `push-budget` slot. The ground is the same one the
`drain-inputs` paragraph states: the reader is a session, so the value is a slot and
not a knob. It also says the precondition binds a mid-iteration pusher too.

### (7) The generated mirror {mechanical}

`docs/lifecycle-kit/SPEC.md` is regenerated after deltas 3 and 6.

## Producers and consumers

- **The `push-budget` slot** (deltas 1-2). Producer: the consumer's close binding,
  which this repo sets in delta 2. Consumer: the close session at step 13, before
  each push and on a red one. Roster-holding reader: `check-skill-binding`, which
  pairs template slots with shim bindings and reds the pair apart, so deltas 1-2
  ship in one commit. `check-shim-restatement` reads the binding, which carries
  residue (numbers and a workflow name) and restates no template text.
- **The actor rule** (deltas 3-5). Producer: this contract. Consumers: the landing
  build session, through the build.md sentence; the lead, through lead.md; a stage
  using a remote oracle, through the SPEC. No gate, as the state machine already
  states.
- **Point 6, every member's value.** The corpus is the surfaces that state a push
  count. Probe: `git grep -n -i "pushes per iteration\|hotfix push" -- '*.md'
  ':!TASK-QUEUE.md' ':!docs/posts'`. Values: CLAUDE.md → delta 2 (pointer).
  `.claude/commands/close.md` → delta 2 (binding). Any other hit found by build is a
  missed site.

## Existing sections updated

Roster probe: the point-6 grep, plus `grep -n "seam is held deliberately\|push-identity
precondition" lifecycle-kit/SPEC.md`.

- `lifecycle-kit/templates/stages/close.md` step 13 (delta 1)
- `.claude/commands/close.md` and `CLAUDE.md` (delta 2)
- `lifecycle-kit/SPEC.md` §The state machine (delta 3)
- `lifecycle-kit/templates/lead.md` §Stamps are authoritative (delta 4)
- `lifecycle-kit/templates/stages/build.md` (delta 5)
- `lifecycle-kit/SPEC.md` §templates/stages/ (delta 6)
- `docs/lifecycle-kit/SPEC.md` (delta 7)

## Retired spellings

- None — the CLAUDE.md numbers move into a binding. No name is retired.

## Definition of Done

- [ ] **Causal completeness** — every point holds for the slot and the actor rule.
- [ ] **Instruction surfaces: instruction only** — the template and binding text
      carries no grounds.
- [ ] **Merged with no information lost** — deltas 3 and 6 re-phrase their
      paragraphs.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component, discharged at the iteration.
- [ ] **Done moves** — both paired entries move to Done in the merge commit, before
      the drain stage.
- [ ] **Release declaration** — the new slot is a template change a vendoring
      consumer meets. Its bullet lands in the unit that lands it.
- [ ] **Gaps filed** — any cross-component gap found during the work filed.
