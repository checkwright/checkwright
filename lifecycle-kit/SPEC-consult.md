# SPEC amendment: consultation-landing-contract

A landing contract for operator strategy sessions: a `/consult` skill whose exit
condition is that **every closed ruling has landed in a governed surface**, every
refused alternative is recorded with its grounds, and any always-loaded surface
the ruling stales is flagged or filed.

The grounds are measured, not argued — three consultations whose output stayed a
transcript, the most recent being the one that produced this iteration's sibling
unit and only reached the queue because the operator filed it by hand.

## What changes

### 1. `/consult` is a boundary skill, not a stage {design-bearing}

This is the open question the entry names — "a consultation is not a lifecycle
stage — it has no iteration, stamps no cursor, and may span or precede one — so
where it sits relative to the state machine is the open question" — and the tree
already carries the class it belongs to. lifecycle-kit/SPEC.md §Layout and
configuration lays templates out on a stage/boundary axis: `templates/*.md` is
exactly the **boundary skills**, `templates/stages/*.md` exactly the stage-class
templates. `lead.md`, `release-sweep.md` and `upgrade.md` already sit there; each
invokes no `enter-stage.sh`, stamps no `WORKFLOW-STATE.txt`, and is never read by
`check-stage-skill-coverage`.

So the ruling costs no new mechanism and no new state: **`/consult` joins the
boundary class.** It ships as `lifecycle-kit/templates/consult.md` plus a
`.claude/commands/consult.md` binding shim, gets a `### templates/consult.md`
SPEC section like its three peers, and relates to the state machine exactly as
`lead.md` does — it may run before a scope, between stages, or across an
iteration boundary, because a boundary skill has no cursor to be at odds with.

*Ruled out: a seventh stage.* It would need a predecessor in
`LIFECYCLE_KIT_STAGES`, a mandatory position in the walk, and a stamp — and a
consultation that "may span or precede" an iteration has none of those. It would
also make `check-stage-entry` assertion A demand a consult stamp on iterations
that legitimately hold no consultation.

### 2. The exit contract, and what makes it satisfiable {design-bearing}

The skill's exit condition is the entry's: every closed ruling landed, every
refused alternative recorded with its grounds, every staled always-loaded surface
flagged or filed. Two mechanisms make that reachable rather than aspirational:

- **A ruling lands the moment it closes, not at exit.** Deltas 3-4 are what make
  this safe under a live iteration.
- **Refusals land beside rulings.** A refused alternative recorded with its
  grounds is what stops the next session re-proposing it; TRAJECTORY.md already
  models the form ("Building from vendored crate source at install time is void.
  It was ruled once and is recorded here rather than deleted, so the next session
  reaching for the cheap answer finds it already costed").

### 3. TRAJECTORY-class write authority is stated where the surface is defined {design-bearing}

The consult exit contract needs to know what it may write to the ruling record,
and **nothing in the tree says.** Verified across the whole corpus: TRAJECTORY.md
states how a session *reads* a ruling (§How to read a ruling recorded here) and
where grounds live, and never who may write it or when; the only lifecycle
reference to it is a **read** grant in scope's binding; its other registrations
(`canon-config.sh`'s manifest set, `core-files.list`, `root-allowlist.list`) are
gate membership, not authorization. The scope survey re-verified this and reached
the same result independently.

The rule, landing in the ruling record itself as a sibling of §How to read a
ruling recorded here:

> **A session records a ruling the operator closed; it never authors one.** The
> authority is the ruling's existence, not the recording session's role — so no
> roster of permitted writers is maintained, and none needs to be. A recording
> names the ruling's date, and where the mechanism has a canonical home it points
> there rather than restating it. Reversing, demoting or re-scoping a recorded
> ruling stays operator-class, unchanged.

Placed there rather than in the consult skill because that is the widest tier
true for every reader of it: **every** session that closes a ruling with the
operator faces this question, not only a consult session. What the skill adds on
top is obligation — a consultation that closed a ruling and did not land it has
not exited — where any other session is merely permitted.

*The honest limit:* this is an authoring contract, not a gate. Nothing mechanizes
"the operator closed this", and nothing should — the alternative is a session
self-attesting to consent, which is worth less than the prose rule.

### 4. The journal rule is restated at its true altitude, not carved out again {design-bearing}

The entry frames this as a choice: widen the top-level carve-out, or give
`/consult` its own in-flight surface. Both are refused in favour of a third
reading that is smaller than either.

The carve-out today (`delegation-kit/templates/agent-execution.md:115-118`) says
"a top-level session has no journal in this contract and discharges by
committing, **which it is by construction able to do**". The premise is what
fails: a session running alongside a committing agent cannot commit on demand,
which is the normal condition during an active iteration — measured, this
iteration's lead wrote ad-hoc scratch notes on three separate occasions, each
time because a live stage session held the shared index.

The rule restated:

> The obligation is **durability**, and committing and journalling are two ways
> to discharge it. A session uses whichever is available: a dispatched session
> journals because it cannot commit; a top-level session commits because it can;
> a top-level session that *cannot* commit right now journals for as long as that
> holds, and discharges by committing when the index frees.

This is not a new rule so much as the one agent-execution already carries —
*findings you will act on are durable before you act on them* — applied where its
own carve-out had assumed the answer. It reaches the lead role and any consult
session together, which is why a `/consult`-specific surface is the worse of the
two options the entry offered: it would fix one role and leave the other
re-inventing scratch notes, three times measured.

### 5. What a consult session does when its exit commit is blocked {design-bearing}

This falls out of delta 4 and is stated explicitly because the entry demands it —
"else the exit requirement is unsatisfiable exactly when the iteration is
busiest". A consult session journals each ruling as it closes, and lands them by
the shared-index discipline the repo already runs: check for a foreign staged
path before staging, or stage and commit in one motion. If the index is held, the
session waits — **and waiting is safe precisely because the finding is already
durable.** The journal is what converts a blocked commit from data loss into
latency.

### 6. The dispatch contract, by citation {design-bearing}

Per the operator's direction on this entry, and none of it inherited:

- The skill **cites** `delegation-kit/templates/agent-execution.md` the way
  CLAUDE.md §Agent execution and the stage-session agent definition already do.
  It does not copy the protocol — a copy is a second content tier that drifts,
  which is the whole failure the citation form exists to prevent.
- It carries its own **standing dispatch policy** naming tier selection
  explicitly. The hazard is sharper here than for a stage session: an unselected
  dispatch inherits the dispatcher's tier rather than defaulting cheap, and a
  consult session is judgment-tier by nature while most of its dispatches are
  read-only research — so an unselected fan-out buys the most expensive tier for
  the cheapest work.

### 7. The two slots, and the seam they draw {design-bearing}

The template carries exactly two named slots, and the split is the provenance
seam applied rather than a convenience:

- `entry-reading` — the surfaces a consultation reads on entry.
- `landing-surfaces` — the surfaces a closed ruling may land on.

Both are **consumer bindings**, never kit literals. This repo's entry set
includes its private brief, and a kit literal naming it would publish a private
surface; its landing set names this repo's own ruling record and queue, and a kit
shipping those names would ship one project's governance layout as everyone's.
What stays kit mechanism is the shape: *a consultation has an entry read-set and
an exit landing-set, and every closed ruling reaches the landing set before the
session ends.*

### 8. The graph manifest the gates will not catch for you {mechanical}

`check-skill-binding.sh`'s `# graph:` couples boundary templates **by name**, not
by glob — `lead.md`, `release-sweep.md`, `upgrade.md` and
`delegation-kit/templates/agent-execution.md` are each listed. A new boundary
template left off it is one whose edits fire nothing while the gate stays green.
So `lifecycle-kit/templates/consult.md` joins that couples list as part of this
unit's delivery; no existing gate reports its absence.

### 9. Enumeration and projection updates {mechanical}

- lifecycle-kit/SPEC.md §Layout and configuration enumerates the boundary skills
  in prose (`lead.md`, `release-sweep.md`, `upgrade.md`) — the enumeration gains
  `consult.md`.
- A new `*/templates/*.md` member restales `docs/footprint.md` and `docs/value.md`
  (`check-footprint-fresh`, `check-value-rollup-fresh` both couple that glob), so
  both projections regenerate in the same unit.
- The shim must clear `check-shim-restatement`: no normalized 9-word span shared
  with CLAUDE.md or any kit template. Worth naming because the natural way to
  write a consult shim is to paraphrase agent-execution, which is exactly what
  that gate exists to stop.

## Producers and consumers

**`/consult`, the skill** (new interface)

- **Producer** — the operator, invoking it. There is no automated trigger and
  deliberately none: a consultation is an operator act, and a scheduled one would
  be a stage in disguise.
- **Consumer** — the agent session that executes the template through the shim,
  by the binding-shim grammar `check-skill-binding` enforces.
- **Every slot has a named reader.** `entry-reading` is read by the skill's entry
  step; `landing-surfaces` is read by its exit check. Two slots, two readers, and
  `check-skill-binding` mechanically requires the shim to bind exactly this set —
  an unbound slot or an orphan binding is red, so the field-with-no-reader failure
  is caught by a gate here rather than only by review.

**The consult session's journal** (new state)

- **Producer** — the consult session, on the widened rule (delta 4).
- **Consumer** — the same session, on resume; and the operator, who may read it
  where a session was interrupted.
- **Reclaim** — the existing scratch reset. `enter-stage.sh`'s boundary wipe
  clears `.tmp/` at the next first-stage entry, which is the correct lifetime: the
  journal's content is discharged into a commit at exit, so nothing durable is
  lost. **No `LIFECYCLE_KIT_BOUNDARY_PRESERVE` entry is added** — a preserved
  journal would outlive the session that can interpret it.
- **No close-surface declaration is owed**, and this is stated so the absence
  reads as a ruling rather than an omission. The roster's automatic source is
  gitignored members of the *workflow* directory; `.tmp/` is not one, and the
  existing resume journals carry no declaration for the same reason — a journal
  is scratch discharged by its own session, not an inbound triage surface close
  must drain.

**The TRAJECTORY-class write rule** (new rule, no new state)

- **Producer** — the operator closing a ruling; the recording session transcribes.
- **Consumers** — every session that closes a ruling with the operator (permitted),
  and a consult session (obliged, by delta 2's exit contract).

**The widened durability rule** (changed rule, no new state)

- **Consumers** — every session reading `agent-execution.md`, which is every
  dispatching session and every dispatched agent; the lead role, whose measured
  three ad-hoc journals are what the widening covers; and `/consult`.

## Existing sections updated

Each names the delta that owns it:

- **`lifecycle-kit/SPEC.md` §Layout and configuration** — the boundary-skill
  enumeration (deltas 1, 9).
- **`lifecycle-kit/SPEC.md`, new `### templates/consult.md`** — the skill's
  contract and slot set, on the shape §templates/lead.md and
  §templates/release-sweep.md already use (deltas 1, 2, 6, 7).
- **`lifecycle-kit/templates/consult.md`** — new (deltas 1-2, 5-7).
- **`.claude/commands/consult.md`** — new shim, binding both slots (deltas 7, 9).
- **`lifecycle-kit/checks/check-skill-binding.sh`** — the `# graph:` couples list
  (delta 8).
- **`delegation-kit/templates/agent-execution.md`** — the top-level carve-out
  paragraph (delta 4).
- **`delegation-kit/SPEC.md` §Resume journal — agent writes, scratch reset sweeps**
  — the same rule at its owning tier (delta 4).
- **`TRAJECTORY.md`** — the new recording-authority section (delta 3).
- **`docs/footprint.md`, `docs/value.md`** — regenerated projections (delta 9).

**The seam is held.** No private rule content enters a kit surface: the private
brief is reachable only through a consumer binding, and the two slots are exactly
where this repo's surface names live.

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
