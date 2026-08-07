# SPEC amendment: survey-carryforward

Queue entry: **`cross-stage-census-duplication`**. Owning component:
**lifecycle-kit** (`bin/enter-stage.sh`, the stage templates, and a new committed
per-iteration surface). The basename names the *mechanism*; the ref resolves as a
bare basename tree-wide (canon-kit/SPEC.md §check-amendment-queue).

This amendment changes **one component's** contracts — lifecycle-kit's. It
*cites* delegation-kit's durability rule and does not move it; see §What reaches
a sibling unit.

## What this amendment inherits and may not re-litigate

- **TRAJECTORY.md's closed ruling of 2026-08-06**: the iteration takes token
  waste as its subject.
- **The measurement, 2026-08-06.** In `native-first-port-cohort`, scope
  dispatched a port-candidate census (8.70 USD including its two forks) and spec
  dispatched a port-candidate cohort survey (1.83 USD) forty minutes later. Both
  applied all six criteria in gate-sdk/SPEC.md §The port-candidate criteria to
  the same gate registry, and both ran `check-gate-substrate-parity` as their
  oracle. Roughly 10.50 USD bought one roster twice.
- **The entry's own finding that this is not a discipline failure.** Each
  dispatch was correct in isolation: the spec session needed an evidence-backed
  roster and had no artifact to read, because a fan-out's findings live in the
  dispatching session's context and die with it. Nothing below is to be written
  as a reminder to try harder.

## The premise this amendment must correct before designing on it

The unit's filed premise, from `stage-fanout-burn-unbilled`'s entry, was that a
dispatcher-minted-path convention *"is also the convention
`cross-stage-census-duplication` needs, which is why the two may be one design."*

**That shared premise is gone.** Batch 1 refused the dispatcher-minted
attribution key (drift-kit/SPEC.md §The stage-economics meter, the fan-out row)
on three grounds — it fails silently when a dispatcher forgets, it
relocates the harness coupling rather than removing it, and it cannot reach a
fork at all. So this unit inherits no dispatch convention and is designed on its
own merits. It is, below.

The refusal's *reasoning* survives the inheritance and is load-bearing here in
one specific way: **a convention that fails silently when someone forgets is not
an acceptable mechanism.** That constraint is honored below by making the
mechanism's one load-bearing field machine-stamped rather than author-supplied
(delta 3), and by gating the record's grammar (delta 5) rather than trusting it.

## The blocker, and why it dissolves

The entry filed `[design-pending]` on one question:

> an artifact carries a staleness problem the re-derivation does not. A census
> written at scope and read at spec is correct only while the tree it censused
> holds still, and the stages between them are exactly when that tree moves — so
> the cheap fix trades a known cost for a silent one. **Sizing the durable half,
> which censuses stay true across one iteration, is the design work.**

The framing assumes the answer must be a *taxonomy* — a judgment about which
kinds of survey stay true for how long. That judgment is not makeable: it depends
on what the next stage does, which is not known when the survey is written.

**Ruled: the question is answered mechanically per record, not categorically per
kind.** The observation that unlocks it is in the measurement itself — *both*
duplicated censuses ran an **oracle** (`check-gate-substrate-parity`). That is
not a coincidence of this instance; it is what this repo's doctrine already
demands of any survey worth carrying (CLAUDE.md §Delivery doctrine,
oracle-first: "run the gate, never emulate it"). So a survey decomposes:

- a **cheap, re-runnable half** — the oracle's verdict;
- an **expensive, judgment half** — the reading laid over that verdict against a
  stated criterion set.

The re-derivation re-buys *both*. The design is to carry the expensive half and
re-run the cheap half:

> **A carried survey is a citation with a falsifiable staleness witness, never a
> substitute for the oracle. The consuming stage re-runs the oracle and diffs the
> surveyed corpus since the recorded revision; if both hold, the recorded
> judgment stands and is cited rather than re-bought. If either moved, the stage
> dispatches only the delta.**

"Which censuses stay true across one iteration" then has a mechanical answer
rather than a guessed one: **a survey stays true exactly as long as its witness
holds**, and the witness is checkable in two commands. The cheap fix no longer
trades a known cost for a silent one, because the silent case is the one the
witness makes loud.

The honest limit ships with it: a survey whose grounds are **not** an oracle —
a judgment over prose, a reading of history — records `oracle: none`, and that
record is a **note, not a re-usable survey**. It may be read for orientation and
must be re-derived before it is relied on. Stating that costs one line per record
and is the difference between a mechanism and a false-assurance surface.

## Where it lands, and the two homes ruled out

### Ruled out: the resume journal

The obvious home is delegation-kit's resume journal — a dispatching session
already writes one, and it already holds the findings. It is wrong on two counts,
and each alone decides it:

- **Its purpose is a different reader.** The journal is crash recovery for the
  *writing* session (or a supervisor's cold read of an interrupted one). Its name
  is deliberately per-session, and telling a live journal from a spent one is
  done "by its per-session name, the work cursor, and `git log`"
  (delegation-kit/SPEC.md §Resume journal). A hand-off surface must be
  discoverable by **content**, by a session that never knew the writer's name.
  Making journals discoverable by content would dissolve the per-session-name
  discipline that keeps a live journal from being read as a spent one.
- **Its home is untracked scratch, which has a second reclaimer.** context-kit's
  session-context hook sweeps `.tmp/` at **every session start**, age-guarded at
  24 hours (context-kit/SPEC.md §The session-context hook). An iteration
  routinely spans more than a day, so a scope-stage journal can be swept before
  the stage that would read it ever runs. The failure is silent and
  time-dependent — the worst shape available.

### Ruled out: the gap inbox

`.workflow/gap-inbox.md` has the right *tier* (committed, per-iteration,
append-only) and the wrong *semantics* on both axes. It carries `merge=union`
because "a gap filed on either side of a concurrent merge must survive", and
`bin/enter-stage.sh`'s boundary entry **refuses** while it holds bullets. A
survey is the opposite on both: it is per-iteration scratch that must be
*discarded* at the boundary, not drained, and a stale survey surviving a merge is
a hazard rather than a save. Routing surveys into the inbox would make the
boundary refuse on residue nobody owes a disposition for.

### Ruled: a committed, boundary-truncated surface of its own

`.workflow/survey-record.md` — the tier `.workflow/validate-evidence.txt` and
`.workflow/release-disposition.txt` already occupy: tracked, committed, and reset
to its `# contract:` header at the iteration boundary. That gives it, by
construction and with no new mechanism, **a lifetime of exactly one iteration** —
which is precisely the window the entry asked to have sized, delivered by the
boundary reset rather than by a judgment.

It resets as a **kit built-in member**, not through the consumer's
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` array — the rule the SPEC already states for
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` ("the kit owns that surface, so it does not
ride the consumer knob"). That is not tidiness: a defaulted bash array is
*replaced* when a consumer assigns it, so shipping the survey record as a default
member would silently lose the reset in every consumer that sets the knob for its
own reasons — the same trap the SPEC already records for
`LIFECYCLE_KIT_BOUNDARY_PRESERVE`'s `.gitkeep` exemption.

## What changes

### 1. The survey record — a new committed per-iteration surface

**{design-bearing}**

`.workflow/survey-record.md` (knob `LIFECYCLE_KIT_SURVEY_RECORD_FILE`,
§Layout and configuration), committed and append-only within an iteration.
Grammar: a `# contract:` prose header, then one block per survey:

```
## <YYYY-MM-DD> <stage> — <the one-line question this survey answered>
- corpus: <git pathspec the survey covered>
- oracle: <the command whose verdict grounds it, or the literal `none`>
- rev: <full commit sha the survey was taken at>
- finding: <the judgment, in prose>
```

Four fields, and each earns its place by being read at a named transition
(delta 2) — `corpus` and `rev` by the diff, `oracle` by the re-run, `finding` by
the consuming session. The heading is the discovery key: it states the
*question*, because a later stage searches by the question it is about to ask,
not by the corpus it has not yet chosen.

**No field for "how long this stays true."** That was the entry's framing and it
is deliberately absent: an author cannot know it, and a field carrying a guess
would be read as a warrant. `corpus` + `rev` + `oracle` let the *reader* compute
it, which is the whole ruling.

**Append-only within the iteration, and never edited in place.** A survey that
turned out wrong is superseded by a later block answering the same question, not
by a correction to the old one — the record is evidence of what was believed
when, and rewriting it destroys the only thing that makes a stale finding
diagnosable after the fact.

### 2. The re-use protocol — the witness contract

**{design-bearing}**

A session about to buy a survey reads the record first and, for any block whose
heading answers its question, runs the witness:

1. **Corpus still?** `git diff --quiet <rev>..HEAD -- <corpus>` — clean means no
   commit since the survey touched anything it covered.
2. **Oracle still?** Re-run `<oracle>` and compare its verdict to the one the
   finding was written against.

Both hold → **cite the record; do not re-buy the survey.** Either moved →
**dispatch only the delta**, and the dispatch prompt names the record block and
the diff, so the child re-surveys what changed rather than the corpus.
`oracle: none` → the block is a note; re-derive before relying on it.

The asymmetry is deliberate and is what makes this safe to ship. A **false
"stale"** costs one re-derivation — exactly today's cost, so the mechanism's
worst case is the status quo. A **false "fresh"** would cost a wrong decision on
a stale roster, and the witness is constructed so that arm requires a change that
touches neither the corpus nor the oracle's verdict — in which case the finding
is, in fact, still true. The mechanism can only degrade *toward* the behavior it
replaces, which is the property the entry's blocker doubted was reachable.

*Ruled out: a timestamp freshness window.* "Trust a survey under N hours old" is
the taxonomy in disguise — it is wrong in both directions (a one-minute-old
survey is stale if the tree moved; a week-old one is fine if it did not) and it
buys nothing the two-command witness does not.

*Ruled out: auto-invalidating a block on any commit.* Too coarse — every commit
would invalidate every survey, which returns the mechanism to zero use. The
`corpus` pathspec exists precisely to make invalidation proportionate.

### 3. `bin/file-survey.sh` — the writer

**{mechanical}**

`bin/file-survey.sh "<question>" "<corpus>" "<oracle>" "<finding>"` appends one
block, seeding the contract header when the record does not yet exist. It follows
`bin/file-gap.sh` and `bin/kfric.sh` exactly: repo-root cd, config-via-env, exit
2 on a missing or empty argument. Advisory tooling, not a gate — the raw append
stays a legal fallback, the grammar being the surface's contract rather than the
writer.

**It stamps `rev` and `<YYYY-MM-DD>` itself, and derives `<stage>` from the
cursor** (`lifecycle_current_stage`). This is the delta's one non-obvious point
and it is load-bearing: `rev` is the field the entire re-use protocol turns on,
and it is exactly the field an author would get wrong — pasting a short sha,
using the rev they *started* at, or omitting it. Machine-stamping it is how this
mechanism avoids the failure mode batch 1 refused the dispatcher-minted key for:
a convention that fails silently when someone forgets.

It **deliberately does not** inherit `file-gap.sh`'s slug-resolution behavior. A
survey's prose routinely names queue slugs as its subject, and `file-gap.sh`'s
longest-match resolver scans whole prose — a live defect already filed in this
repo's gap inbox. Adding no resolver here is a decision, not an omission.

### 4. `bin/enter-stage.sh` — the read trigger and the boundary reset

**{mechanical}**

Two additions, both to a tool every stage already invokes as its first step, so
neither adds an invocation point or a schedule:

- **Read trigger.** When the record is non-empty, the entry report prints its
  `## ` headings — the questions, not the findings. A stage session therefore
  learns *that* prior surveys exist and what they answered, at the one moment it
  is guaranteed to be looking. Headings only: printing findings would put a
  possibly-stale judgment into context ahead of its witness, which is the failure
  the witness exists to prevent.
- **Boundary reset.** The first-stage boundary entry truncates the record to its
  contract header, as a **built-in member** (see §Where it lands). It is named in
  the existing report line beside the other truncated surfaces.

**No boundary refusal.** A non-empty record must never block the boundary: unlike
the gap inbox, a survey owes nobody a disposition — it is scratch whose whole
lifetime was the iteration that just ended. Stating this explicitly matters
because the two surfaces sit one line apart in the same tool and the next reader
will otherwise assume symmetry.

### 5. `check-survey-record` — the grammar gate

**{mechanical}**

A new lifecycle-kit gate with the standard `good/`+`bad/` fixture pair, asserting
over each `## ` block in the record:

- all four keys present, in order, one per line;
- `rev` is a full 40-hex sha naming a commit that exists (`git cat-file -e`) —
  the assertion that catches the short-sha and wrong-rev cases, and the reason
  the field is machine-stamped;
- `oracle` is non-empty (the literal `none` is legal and is the honest form; an
  *empty* oracle is the silent form and is refused);
- `corpus` is non-empty.

An absent or header-only record is **clean, counted-inert** — the record is
optional, and a consumer that never files a survey must not carry a red gate.

Enforcement-first: a block missing its witness is silently unusable, which is the
exact failure class this amendment exists to close, so the gate lands in the same
unit as the surface rather than as follow-up debt. It registers in this repo's
`gates.list` at `precommit` tier — dogfooding is day-one.

*Not gated, and stated so nobody mistakes the gate for more than it is:* whether
a `finding` is *true*, and whether a session that had a fresh record actually
read it. The first is unmechanizable; the second leaves no tracked artifact —
the same class delegation-kit/SPEC.md §Operative residency rules no gate is owed
for. The read trigger in delta 4 is the affordance in place of an oracle, and it
is weaker than one by construction.

### 6. Knob, merge attribute, and the capture bullet

**{mechanical}**

- `LIFECYCLE_KIT_SURVEY_RECORD_FILE` — default `.workflow/survey-record.md`,
  joining the knob roster in §Layout and configuration beside
  `LIFECYCLE_KIT_GAP_INBOX_FILE`. One knob; no second knob for the grammar, the
  witness commands, or an opt-out, each of which would be contract rather than
  layout.
- **Merge semantics:** `merge=iteration-scoped` (keep-ours), the tier the
  boundary-truncated surfaces already carry — **not** the gap inbox's
  `merge=union`. A survey from the other side of a concurrent merge describes a
  tree state this clone never had, so surviving the merge is the hazard, not the
  save. Emitted by `bin/install-lifecycle.sh` and verified by
  `check-merge-attrs`, via `lifecycle_merge_attrs_block`, so writer and asserter
  keep reading one set.
- **CLAUDE.md §Housekeeping** gains one capture bullet in the shape the two
  existing ones already take: *Survey capture (any stage session)* — bought a
  survey a later stage will want? land it with `bin/file-survey.sh` before you
  act on it. That tier is correct because the trigger is "any session, any
  stage", exactly as knowledge-friction and gap capture are.

### 7. The stage-template obligation

**{mechanical}**

One clause in the stage templates' session ritual, at the point each already
discusses dispatching: **before buying a survey, read the record and run the
witness; after buying one whose finding a later stage will want, file it.** It
cites §The survey record and restates no mechanism.

This is prose, and prose is the weaker half of this amendment — delegation-kit's
own §Operative residency records that reachable is not obeyed. It is paired with
the delta-4 read trigger for exactly that reason: the template states the
obligation, the entry report delivers the surface, and neither substitutes for
the other.

## Producers and consumers

**New interface: the survey record block.**

- *Producer* — `bin/file-survey.sh`, invoked by any session that bought a survey;
  the raw append is the sanctioned fallback. Its enabling config is the knob's
  default, so it is live on every consumer with no action; the deployed
  configuration that must set it is none.
- *Consumer* — the next stage session, at its `enter-stage.sh` entry (which
  surfaces the headings) and at the moment it is about to dispatch a survey
  (where the template obligation and the witness protocol apply).
- *Readers, each field at a named transition:*
  - `corpus` and `rev` — read together by the consuming session's `git diff`
    witness, at the pre-dispatch check;
  - `oracle` — read by the same session at the same transition, to re-run;
  - `finding` — read by that session only *after* the witness holds, which is why
    delta 4 prints headings and not findings;
  - the `## ` heading — read by `enter-stage.sh` at every stage entry, and by
    `check-survey-record` at commit time as the block delimiter.
- *Third reader, at the iteration boundary* — `bin/enter-stage.sh`'s first-stage
  truncate, which reads the surface as a whole and discards it.

No field lacks a reader, and none is populated at a transition where it is not
read: `file-survey.sh` writes all four in one append and never updates a block.

**Nothing is produced on the delegation side.** No journal contract moves, no
dispatch shape changes, no new obligation lands on a dispatched child. The
producer is the *parent* — which is where delegation-kit's **Findings you will
act on are durable before you act on them** rule already puts the write, so this
amendment gives that existing obligation a home for one shape of finding rather
than adding an obligation.

## Existing sections updated

Each named with the delta that owns it.

1. **lifecycle-kit/SPEC.md — a new §The survey record** (deltas 1, 2, 3), placed
   immediately after §The committed gap inbox. Placement is content: the two are
   sibling committed per-iteration surfaces with deliberately opposite merge and
   boundary semantics, and a reader who meets them apart will assume symmetry.
   The new section states the contrast explicitly rather than leaving it to be
   inferred.
2. **lifecycle-kit/SPEC.md §bin/enter-stage.sh** (delta 4). The boundary-reset
   paragraph gains the record as a built-in truncate member; the entry report
   gains the headings line; and the **no boundary refusal** point lands beside
   the existing gap-inbox refusal it must not be confused with.
3. **lifecycle-kit/SPEC.md §Layout and configuration** (delta 6). The layout
   block gains `bin/file-survey.sh`, `checks/check-survey-record.sh`, and its
   fixture dirs; the knob joins the roster.
4. **lifecycle-kit/SPEC.md §check-merge-attrs** and **§bin/install-lifecycle.sh**
   (delta 6). The record joins the `merge=iteration-scoped` set that
   `lifecycle_merge_attrs_block` renders.
5. **lifecycle-kit/SPEC.md §Testing** (delta 5). The fixture pair joins the
   rostered set.
6. **The stage templates' session ritual** (delta 7).
7. **CLAUDE.md §Housekeeping** (delta 6). The capture bullet, and
   `.workflow/`'s two-tier description already covers the new file's tier
   unchanged.
8. **`scripts/gates.list`** (delta 5), and the generated projections a new gate
   stales — the pre-commit hook, the graph artifact, the enforcement map, the
   footprint and value rollup, the docs mirror. docs/site-architecture.md
   §Generated projections carries the full fan-out and each freshness gate prints
   its own regen command on red; this is named as a checklist item because a new
   gate's fan-out is the thing most often left half-done.
9. **README.md §This repo, governed** — checked and **unchanged**. lifecycle-kit
   already has a `gate-tests/` directory and a fixture-runner line, so the new
   fixture pair adds no battery row.

## What reaches a sibling unit

Stated here because a later author working one unit will not read the others.

- **To `fork-dispatch-prohibition` / `read-only-fanout-unenforceable` /
  `subagent-parent-addressing`** (this batch's first amendment,
  `delegation-kit/SPEC-dispatch-shape.md`). That amendment's delta 4 prescribes a
  **durable artifact** as a dispatched child's only upward route. This one
  prescribes a **durable artifact** for a stage's survey findings. They are
  deliberately different artifacts and must not be merged: the escalation channel
  is mid-run, dispatcher-minted, per-dispatch, and written by the *child*; the
  survey record is a hand-off, single-path, per-iteration, and written by the
  *parent* after the child returns. Their readers, lifetimes, and discovery keys
  all differ. This amendment cites that one and extends nothing in it.
- **To `stage-fanout-burn-unbilled`** (batch 1, merged into drift-kit/SPEC.md
  §The stage-economics meter, the fan-out row). Its fan-out row is the instrument that
  measures whether this amendment worked — a scope-and-spec pair that stops
  double-buying a census shows up as a fall in the two stages' fan-out rows.
  Nothing in the meter changes. The premise correction runs the other way and is
  handled in §The premise this amendment must correct above.
- **To `amendment-dod-sibling-dependence`** (deferred). This iteration has three
  amendments across three kits and **no kit carries two**, so every
  none-remain-for-the-component assertion is satisfiable at its own commit. That
  entry's defect is therefore not exercised here — recorded so a later reader
  does not take this iteration as evidence either way.
- **To `gap-inbox-commit-ownership`** (icebox). The record inherits the same open
  question — who commits an append filed by a session that is not the stage
  session — and this amendment does **not** answer it. The exposure is smaller
  (a survey lost to an uncommitted append costs a re-derivation, which is today's
  cost) but it is the same question, and that entry now covers two surfaces.

## Definition of Done

- [ ] **Causal completeness** — the new surface has a named, reachable producer
      and named consumers; all four fields plus the heading have named readers at
      named transitions.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone. The
      nine targets in §Existing sections updated are the checklist.
- [ ] **The gap-inbox contrast landed** — a merged SPEC that describes the survey
      record without stating its opposite merge semantics and its non-refusing
      boundary is a failed merge, because the next reader will infer symmetry
      from adjacency and get both wrong.
- [ ] **The new gate's projection fan-out regenerated** — hook, graph,
      enforcement map, footprint, value rollup, docs mirror
      (docs/site-architecture.md §Generated projections).
- [ ] **Amendment deleted** — this file removed on merge. The
      *none-remain-for-the-component* half is discharged at the **iteration**,
      not at this commit (canon-kit/SPEC.md §Merging an amendment, step 3); this
      is lifecycle-kit's only amendment in flight, so it is satisfiable here.
- [ ] **Dogfooded once before the unit closes** — one real survey filed and one
      real witness run in this repo. A mechanism whose first use is a fixture is
      exactly the shape `consumer-smoke-artifact-arm` is filed about.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through `bin/file-gap.sh` (a build-time causal gap is resolved that
      session, not deferred).
