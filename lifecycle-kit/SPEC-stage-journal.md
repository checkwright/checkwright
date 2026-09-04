# SPEC amendment: stage-journal

The disposition of `stage-journal-absence-caught-only-downstream` — a stage session
can complete, stamp and commit a full evidence spine while leaving no resume
journal at all, and the only thing that notices is the next stage's entry, by which
time the session that owed it is gone. **Promoted into this iteration by the
operator, 2026-09-04, lead-relayed**, which reopened the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE) for this one non-port unit; no yield is created
for anything else and the run is not otherwise weakened.

**The unit was filed with four candidate shapes and none costed, and costing them
changed the question.** The entry reads as a *detection* problem — the assertion
fires too late — and the four shapes are four places to move the detector. Costing
them found that one is not mechanizable at all, one is unenforceable by a recorded
proof, and the third is the status quo. It also found something the entry did not
know, and it is a **cause** rather than a detector:

> **The obligation is stated on no surface a stage session loads.** Measured:
> `grep -c journal lifecycle-kit/templates/stages/*.md` returns **zero for all
> six**. Every stage template opens with a *First step — stamp evidence*; none has
> a last step, and none mentions the journal. The consumer's stage bindings are
> silent too. The only imperative in the tree lives in an **agent definition**,
> which loads only when a live lead dispatches through that agent type — so a stage
> run as an ordinary skill invocation, which this kit explicitly sanctions, never
> loads it at all.

lifecycle-kit/SPEC.md §The state machine already rules that "a stage owes a resume
journal, and the obligation is the stage's rather than the dispatch's". That
sentence is true of the kit and false of the surfaces: the obligation is asserted
at the *next* stage's entry and instructed nowhere the owing session reads. Three
of the five attested firings were sessions holding an explicit dispatch grant — and
a grant is the dispatch's, which is exactly what the ruling says the obligation is
not.

So this amendment removes the cause, keeps the existing detector from going vacuous
under the removal, and moves detection upstream with a tool that already exists. It
mints no hook, mints no permission-settings edit, and reverses nothing.

## What changes

### (1) The obligation lands on the surface the owing session loads

Each of the six stage templates gains a **last step**, mirroring the first step it
already has, naming the resume journal as the stage's own exit artifact
{design-bearing}. This is the whole causal repair, and its placement is the
residency doctrine applied rather than a preference: delegation-kit/SPEC.md
§Operative residency's test puts a rule on the surface its bound actor loads, and
the stage template is that surface for a stage session — the one thing every stage
session reads, under a live lead and without one alike.

**It is a pointer, not a restatement, and that is forced rather than tasteful.**
The journal's *contract* is delegation-kit's and its *path* is this kit's
§The state machine's; a step that restated either would put a second carrier on a
rule whose owner is one pointer away, which is the defect §Operative residency's
own restatement record prices. The step therefore names the **act** and cites the
two owners: append `DONE` as the file's last line before reporting, contract at
delegation-kit/SPEC.md §Resume journal, path the one the entry tool printed at the
stamp (delta 2). Naming the act rather than the rule is what keeps the six steps
short enough that `LIFECYCLE_KIT_SHIM_NGRAM`'s nine-word bar is not approached, and
the consumer's stage bindings **do not** restate it — a binding that did would be
`check-shim-restatement`'s subject, the kit templates being that gate's corpus.

**`DONE` is the right act, and it is the whole obligation in one instruction.** The
marker is meaningful only as the file's last line (delegation-kit/SPEC.md §Resume
journal), so appending it entails that the file exists and that the session wrote
into it. One instruction, and performing it discharges the clause set rather than
enumerating it.

**This does not touch what the entry assertion asserts.** §bin/enter-stage.sh's
first narrowing — *existence and non-emptiness, never the `DONE` marker* — stands
unchanged, on its own recorded ground that a stage entry is not a cold read and
asserting the marker would mint an obligation delegation-kit retires at that
transition. The template instructs the marker; the entry does not assert it. Stated
because the two look like one change and are not.

### (2) `enter-stage.sh` opens the journal at the stamp and prints its path

The entry tool writes the journal's opening line for the stage it is entering, and
reports the path it wrote {design-bearing}. Today the derivation
`lifecycle_stage_journal <stage>` has three readers — the entry assertion, a
dispatching supervisor, and the entering session. This adds a fourth **writer**,
and it is the same tool at the same moment, so the path is derived once and no
surface gains a second spelling of it.

Two things it buys, and the second is the one that makes delta (1) cheap:

- **The absence stops being an absence.** A session that writes nothing now leaves
  a file naming the iteration, the stage, the session id, the date and the head —
  the stamp's own five fields. The next stage's escape, which today asks a session
  to fabricate a record of work it did not do, becomes an **append under a header
  that names who owed it and when**.
- **The template's last step has a path it need not name.** Because the tool prints
  the path in its own report line, delta (1)'s step can say *the journal the entry
  tool named* rather than restating a knob-driven derivation on six surfaces.

**Three behaviours are pinned because each is a way to get this wrong:**

- **It appends where a journal exists, and never overwrites.** A stage may run
  several sessions and §The state machine rules they share one file; a second
  session's entry therefore appends a heading naming itself, which mechanizes the
  "discriminator belongs in a heading inside the journal" rule that surface already
  states and that a lead currently has to remember.
- **It writes nothing under `--simulate`.** The mode's contract is that it runs
  everything up to the write and writes nothing; a skeleton written there would
  make a read-only probe perform a real state write, which is a defect this queue
  already carries an entry for.
- **It writes after the boundary wipe, not before.** At the first stage of an
  iteration the tool truncates the state file and then wipes the scratch dir; a
  skeleton written before that wipe would be deleted by it, silently, and the first
  stage would look like every other firing of the bug this unit fixes.

### (3) The entry assertion's non-emptiness test is repaired, not widened

The assertion at the stage entry today tests existence, then non-emptiness. Under
delta (2) a journal is never empty, so **non-emptiness would become vacuous** — the
detector would pass on every skeleton and this amendment would have removed the
only thing that ever noticed the bug {design-bearing}. The test therefore becomes:
the journal exists, and it **carries content the entry tool did not write**.

**This is the existing narrowing preserved, not a new assertion.**
§bin/enter-stage.sh rules the assertion *existence and non-emptiness, never the
`DONE` marker*, and non-emptiness means *the owing session wrote something*. Delta
(2) changes what an empty file looks like on disk; this delta keeps the predicate
asserting what it always asserted. Read the other way round, it is the obligation
delta (2) creates: a tool that opens a file owes the assertion that reads it a way
to tell its own bytes from a session's.

The refusal message keeps the three-way discrimination it earns — **absent**,
**opened but unwritten**, and **written** — because absent and unwritten are now
different mistakes in the same way absent and empty already are: absent means the
entry tool never ran, unwritten means it ran and the session did not.

**The escape is untouched and stays evadable by design.** §bin/enter-stage.sh's
ground for that — what the mechanism buys is that an absence becomes deliberate and
written rather than silent — is strengthened rather than weakened here, because the
escape now appends to a header rather than inventing a file.

### (4) Detection moves upstream, with the tool that already exists

`templates/lead.md` makes the pre-dispatch `--simulate` read a **step** at the
acceptance boundary rather than one of two offered options {design-bearing}. Before
dispatching the next stage, the lead runs `enter-stage.sh --simulate <next>`; a
relayed would-be refusal names the missing journal while the owing session is at
its most likely to still be resumable.

**This is attested rather than proposed.** Of the five firings, exactly one was
caught before the owing session was gone, and it was caught this way: the
2026-08-31 align journal was found by `enter-stage.sh --simulate build` and
recovered from the still-live session, costing "a round trip and nothing else". The
four that lost work were the four where nothing ran the probe.

**And it is the sanctioned read rather than a new one.** `templates/lead.md`
already rules that the lead "never hand-derives prior-stage completeness … It
dispatches and trusts `enter-stage.sh`'s fail-closed refusal …, or gates an
expensive dispatch cheaply first with `enter-stage.sh --simulate <stage>`." What
changes is the *or*: the cheap gate becomes the step, and the ground is that its
cost is one command against a loss the record prices at a whole stage's reasoning.

**Its honest limit is stated with it**, because a step whose failure mode is
unstated reads as a guarantee: it moves detection from *the next stage's entry* to
*just before the next stage's dispatch*, which is earlier by one dispatch and not
by more. It does not reach a stage session that ends between the lead's two reads,
and it does not exist at all where no lead is running — which is why delta (1),
which reaches every stage session on every path, is the repair and this is the
backstop.

### (5) The obligation gains a gate, in the same unit as the fix

`check-stage-skill-coverage` gains a **third direction**: every configured stage's
template names the journal obligation {design-bearing}. Without it, delta (1) is a
sentence a later template edit drops with nothing noticing — which is this unit's
own failure class, reproduced one level up, and the enforcement-first doctrine
names the fix and its gate as one unit.

**That member rather than a new one, and the reason is its existing invariant.** It
already rules that "the configured stage set and the skills dir cover each other,
both directions", already resolves `LIFECYCLE_KIT_STAGES`, and already uses a
mechanical marker in a file to separate a stage surface from an ordinary one. A
third direction over the stage **template** set is a direction on that invariant,
not a second member; and the corpus widening is precedented one section away, since
`check-skill-binding` already couples the stage-template directory beside the
skills directory.

**The marker is a citation, never a sentence.** The assertion is that each stage
template carries the step's citation of `lifecycle-kit/SPEC.md §The state machine`
— a token `check-spec-pointer` already resolves — so no prose is duplicated for a
gate to match and no gate becomes a second author of the step's wording. A gate
matching a sentence would pin the sentence, which is how a gate acquires editorial
control over prose it does not own.

The member's `good/`+`bad/` fixture pair widens with it: a fixture stage set whose
templates all carry the citation, and one where a single stage's does not.

### (6) The three shapes not taken, each refused on a measured ground

The entry lists four candidate shapes. One is taken as deltas (1) through (3); the
other three are refused here with their grounds, so a later reader finds the
disposition beside the option rather than re-deriving it {design-bearing}.

**Refused — an assertion at the session's own turn end, wired to the harness's
`SubagentStop` event.** This is the shape that looks strongest, because it puts the
assertion at exactly the moment the owing session is still live, and there is a
registered refusing hook at that event already. It is **not mechanizable as
stated**, on a measurement this tree already recorded: delegation-kit/SPEC.md §The
delegation model records that `SubagentStop` **is not the session-end event** — it
fired seventeen times inside one dispatched session that had ended no turn at all,
spaced by assistant steps. An assertion wired there would fire from the session's
first assistant step, when no journal legitimately exists yet, and has no signal
that distinguishes an intermediate step from a return. Two further holes are
structural rather than incidental: a stage run with no live lead is the top-level
session and fires `Stop`, which delegation-kit deliberately leaves unregistered on
the ground that the main-session turn end has no attested firing; and a
worktree-isolated session's stop log resolves against its own cwd and dies with the
worktree. **Recorded rather than dropped**, because the shape is the first thing a
later reader will reach for, and because the refutation is a fact about the harness
that no amount of design care would have surfaced.

**Refused — the returning report asserts the path it wrote.** Two grounds, and the
first is a proof rather than an appetite. delegation-kit/SPEC.md §Resume journal
already rules that no gate reaches the relayed-return half: "a return lives in the
parent's context and leaves no artifact, so tree state is byte-identical whether
the return was held or invented … A gate would therefore have to assert a fact
about a conversation, and no scanner over a repository reaches one." A
report-asserted path is exactly that class, so the shape is unenforceable by
construction rather than by anyone's choice. Second, it points the wrong actor at
the wrong evidence: `templates/lead.md` forbids the lead hand-deriving prior-stage
completeness and directs it at the machinery instead, which is what delta (4) has
it run. **This is also the shape that would have put the unit in delegation-kit**;
refusing it is what keeps the whole unit in lifecycle-kit.

**Refused — accept the escape and trust the successor.** This is the status quo,
and the record prices it: five firings, three of them in the two iterations before
this one, two of them in a single iteration where both owing sessions had already
ended. What the escape buys is real and unchanged — an absence that is deliberate
and written — but it is bought after the reasoning is gone, by the one session
structurally unable to reconstruct it.

**The standing objection is met rather than dodged.** §The stamp protocol rules
that "a self-asserted 'stage complete' marker would prove a claim, not completion —
the kit deliberately has none", and delta (1) instructs a session to write
something at its own exit, which resembles one. The distinction is the one
delegation-kit draws in the paragraph quoted above: the tree cannot observe what a
session **received**, and it can observe what a session **wrote**. A journal is a
durable artifact whose existence and content the entry already reads mechanically;
the template step instructs a checkable **act**, and no surface anywhere reads the
session's word for it. The refused shape is the one where the *claim* is the
evidence — which is the second refusal above, and it is refused.

### (7) What this unit does not touch, named so the boundary is legible

Three deferred entries sit adjacent and none of them moves {mechanical}:

- **The two boundary keep-list entries** — the one about the keep-list enumerating
  named members where the property is a lifetime, and the one about `! -name` being
  unanchored — own the wipe's reach over journals that **exist**. This unit's
  subject is a journal never written at all, which is the distinction the entry
  itself draws.
- **The concurrent same-stage append entry** owns two sessions of one stage
  appending to one file with no coordination beyond append atomicity. Delta (2)
  adds a writer to that file and therefore **touches the same page without
  answering the question**: the entry tool's append is one more unsynchronised
  writer, and it is a serial one — a session's entry precedes its own work — so it
  does not create the concurrency that entry describes and does not resolve it
  either. Said plainly rather than left for that entry's owner to discover.

### (8) The queue entry is promoted, and the promotion is a section move measured to fit

`stage-journal-absence-caught-only-downstream` moves out of `## Deferred` into
`## New Features` with `[design-pending]` swapped for
`[spec: SPEC-stage-journal.md]` {mechanical}. The move is the promotion:
canon-kit/SPEC.md §check-amendment-queue assertion (b) reds a `[spec:]`-tagged
entry left in a design-pending section, which is the probe that established at
scope that this move is this stage's rather than that one's.

Both numbers are measured, not estimated:

- **The lead line fits.** It is 77 columns; less `[design-pending] ` it is a
  60-column base; the bare-basename ref costs 30, for **90** against
  `QUEUE_KIT_WRAP_BUDGET=100`. The repo-relative spelling would cost 44 and
  **overflow at 104**, so the bare basename is required rather than preferred.
- **The 50-line cap does not follow it.** The entry's extent is 52 lines and its
  counted size sits at the cap. queue-kit/SPEC.md §check-queue-entry-budget rules
  the active sections **uncapped**, because an active entry's residency is one
  iteration by the drain rule, so the entry lands legal where it goes with no
  compression bought to move it. Its `Cost while deferred` field travels with it
  unchanged; assertion (C) binds top-level *deferred* entries, so it neither
  refuses the field nor requires it there.

Unlike the port-cut host, this entry **completes at build** rather than demoting:
it delivers one unit and exits to `## Done`, so nothing re-prices it against the
cap on the way back.

### (9) The regeneration fan-out this unit stales

The six stage templates, `templates/lead.md` and `lifecycle-kit/SPEC.md` all carry
on-site mirrors, and the gate widening of delta (5) changes a gate's assertion set
{mechanical}. `check-docs-mirror-fresh` is the red for the first;
`check-enforcement-fresh` and `check-value-rollup-fresh` are the reds for a gate
whose assertion roster moved, and `check-graph` for the generated hooks if the
widened `# graph:` manifest's couples change. All are rostered with their triggers
and regen commands in `docs/site-architecture.md` §Generated projections and are
discharged in the landing commit.

**Two that do not move, probed rather than assumed**: `scripts/measured-claims.sh`
derives `tree-shell-owed`, `ported-gate-members` and `gate-substrates`, and this
unit deletes no shell file, registers no new gate member and adds no `.gate`
descriptor — the widening lands on an existing member — so no measured value moves
and the baked hook is stale only if the manifest is.

## Producers and consumers

The amendment introduces **one new artifact state** — a journal that exists before
its owing session has written into it — **one new writer**, **one strengthened
predicate**, and **one gate direction**. It introduces **no new knob**: the path is
`LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN`'s existing derivation and the assertion stays
behind `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE`, both already defined with defaults in
`lifecycle-kit/lib/stages.sh` and both already validated fail-closed there.

- **Producer of the opened journal** — `bin/enter-stage.sh`, at the stage-entry
  write, after the boundary wipe and never under `--simulate` (delta 2). Its
  enabling config is the pattern knob, which is already resolved at that point in
  the same run for the assertion; nothing new is configured per install, and at
  `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=0` — the kit default — the opener is inert
  along with the assertion it serves, so a consumer that has not taken the feature
  sees no new file.
- **Consumer of the opened journal, first** — the entering stage session, which
  finds the file at the path the tool printed and appends to it as findings are
  confirmed. The transition is every turn of the stage; the mechanism is the
  template's last step (delta 1), which is the surface that session loads.
- **Consumer of the opened journal, second** — the **next** stage's entry
  assertion, at `bin/enter-stage.sh`'s pre-flight, reading the predecessor's
  derived path and now discriminating opened-but-unwritten from written (delta 3).
  This is the reader that makes the strengthened predicate load-bearing: without
  it the new file state has no reader at all and would be a field with no reader,
  which is the shape §The causal-completeness check removes.
- **Consumer of the opened journal, third** — the lead, at the acceptance
  boundary, through `enter-stage.sh --simulate <next>` (delta 4). It reads the
  tool's relayed would-be refusal rather than the file, which is what keeps the
  read out of the hand-derivation `templates/lead.md` forbids.
- **Consumer of the printed path** — the entering session, and the lead's dispatch
  prompt, which delegation-kit/SPEC.md §Resume journal already rules "a restatement
  of a derivation rather than its only source". This amendment gives that sentence
  a second true reading: the derivation now has an artifact on disk beside it, so a
  granted path that disagrees with the derived one is visible rather than merely
  wrong.
- **Consumer of the citation** — `check-stage-skill-coverage`'s third direction, at
  every pre-commit touching the skills or stage-template directories (delta 5), and
  `check-spec-pointer`, which resolves the cited heading in the same battery.
- **Producer of the standing obligation's text** — the six stage templates, whose
  reader is every stage session on every dispatch path; and, unchanged, the agent
  definition, whose reader is a lead-dispatched stage session only. The second is
  deliberately left as it is: it is a correct statement for the actor it reaches,
  and the finding was that it is the **only** one, not that it is wrong.

**No new field is added to any record.** The journal's opening line reuses the
stamp's five fields, which have readers already; the refusal message's third
discrimination is prose read by a session, at the transition the other two are read
at.

**One corpus is narrowed and one is widened, with the readers' red conditions
enumerated rather than their subjects** (canon-kit/SPEC.md §The causal-completeness
check, point 5):

- `check-stage-skill-coverage` — its red condition gains *a configured stage whose
  template does not carry the citation*. Under the widening it can find strictly
  more, so it is non-monotone by construction and is cleared by delta (1) landing
  the citation on all six templates in the same commit, never by inspection.
- `check-skill-binding` — named because delta (1) edits every stage template and a
  reader assumes slot binding must follow. Probed: the step adds no `*<slot: …>*`
  token, so the slot set is unchanged and the bindings stay exact. Monotone,
  cleared by inspection.
- `check-shim-restatement` — its red condition is a binding shim sharing a
  normalized n-gram of `LIFECYCLE_KIT_SHIM_NGRAM` words with the corpus, and the
  kit templates **are** the corpus. Non-monotone: a consumer binding that restated
  the new step would red. Cleared by delta (1)'s rule that the bindings do not
  restate it, and checked by running the gate rather than by reading the six steps.
- `check-spec-pointer` — red on a `<path>.md §<heading>` citation that does not
  resolve. The new citations all name `lifecycle-kit/SPEC.md §The state machine`,
  which exists; non-monotone in principle and cleared by the heading being
  untouched.
- `check-comment-tier` — gains the `# spec:` directives on the new `enter-stage.sh`
  and `lib/stages.sh` code, which is an obligation rather than a risk.
- `check-docs-mirror-fresh`, `check-enforcement-fresh`, `check-value-rollup-fresh`,
  `check-graph` — red on a stale mirror, roster, rollup or hook; non-monotone for
  the baked-value reason. Cleared by delta (9).
- `check-stage-evidence`, `check-stage-entry`, `check-evidence-manifest` — named
  because the unit edits the tool that writes the evidence file and a reader
  assumes the stamp protocol moves. It does not: the stamp line's format, its
  position and its idempotence guard are untouched, and the journal write is a
  separate file. Monotone, cleared by inspection.
- The `lifecycle-kit` fixture suite's own `boundary-stage-journal` test is the
  strongest reader here and is **widened rather than merely re-run**: it asserts
  absent-versus-empty today and must assert absent-versus-opened-versus-written
  after delta (3), plus the three pinned behaviours of delta (2).

**Cross-component signal: this amendment's component set is two** — lifecycle-kit
(§The state machine, §bin/enter-stage.sh, §check-stage-skill-coverage, the six
stage templates and the lead template) and delegation-kit (§Resume journal, whose
"agent writes" clause acquires a case where the file pre-exists the agent) — so
`check-stage-entry` assertion C fires and the **align stamp is demanded at the
build stage's entry**, independently of the port cut this iteration also carries.

**A batching note the lead reads and the build session does not**: this unit and
`wait-probe-cut` share **no kit directory and no gate**, so they are independently
batchable — except for one file. Both edit `delegation-kit/SPEC.md`, in disjoint
sections (§Resume journal here; §bin/wait-probe, §Layout and configuration and
§Testing there). Two sessions batched in parallel would contend on that one file's
index entry and nothing else.

## Existing sections updated

- `lifecycle-kit/SPEC.md §The state machine` — the "a stage owes a resume journal"
  paragraph gains the finding that made this unit necessary: the obligation was
  asserted at the next entry and instructed on no surface the owing session loads,
  and the repair is the template's last step rather than a second assertion
  (delta 1). The journal-path derivation's reader list gains its fourth member, and
  that member is a **writer** (delta 2).
- `lifecycle-kit/SPEC.md §bin/enter-stage.sh` — the entry tool gains the journal
  open, with its three pinned behaviours: append-never-overwrite, nothing under
  `--simulate`, and after the boundary wipe (delta 2). The predecessor-journal
  assertion's paragraph keeps all three of its narrowings verbatim and restates
  what non-emptiness now means on a file the tool itself opened, with the
  three-way refusal message (delta 3). The `--simulate` paragraph gains its second
  named caller — the lead's pre-dispatch step — beside the entry probe it already
  serves (delta 4).
- `lifecycle-kit/SPEC.md §check-stage-skill-coverage` — the invariant gains its
  third direction, the corpus widens to the stage-template directory on
  `check-skill-binding`'s precedent, and the citation-rather-than-sentence marker
  rule is recorded with its ground (delta 5).
- `lifecycle-kit/SPEC.md §Layout and configuration` — the tree block gains nothing
  and the knob roster gains nothing; the section says so, because a unit that adds
  a writer and a gate direction reads as one that should have added a knob, and the
  reason it does not is that both inputs are already resolved for the assertion
  (deltas 2, 5).
- `lifecycle-kit/templates/stages/scope.md`, `spec.md`, `align.md`, `build.md`,
  `validate.md`, `close.md` — each gains the last step and its citation (deltas 1,
  5). Six files, one step, no two of them restating the other at length.
- `lifecycle-kit/templates/lead.md` — the acceptance boundary gains the
  pre-dispatch `--simulate` step, and the sentence that offers it as one of two
  options becomes the sentence that requires it, with the attested recovery as its
  ground and its honest limit beside it (delta 4). §Channel design is untouched:
  the journal stays a pull channel the lead reads and does not delete.
- `delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps` —
  the four-clause contract acquires a case it did not have: under a stage machine
  the file may **pre-exist** the agent, opened by the entry tool, so *agent writes*
  becomes *the machine opens, the agent writes, the reset sweeps*. The clause that
  the grant is "a restatement, not a source" gains its second true reading — there
  is now an artifact on disk beside the derivation (deltas 1, 2). The retention
  paragraph's observation that presence "signals nothing by itself" is
  **strengthened** here rather than contradicted, and this section says so, because
  delta (3) is precisely the predicate that repairs what presence stopped meaning.
- `lifecycle-kit/lib/stages.sh` — the journal-path derivation gains its writer-side
  helper beside the reader it already hoists, so opener and assertion share one
  spelling of the path and of the skeleton's shape (deltas 2, 3).
- `lifecycle-kit/bin/enter-stage.sh` — the entry flow gains the journal open after
  the boundary wipe, and the assertion block's predicate and message change
  (deltas 2, 3).
- `lifecycle-kit/gate-tests/boundary-stage-journal.test.sh` — widened from
  absent-versus-empty to absent-versus-opened-versus-written, plus the three pinned
  opener behaviours (deltas 2, 3).
- `lifecycle-kit/checks/check-stage-skill-coverage.gate` and its `good/`+`bad/`
  fixture pair — the third direction, its coupled surface, and a fixture stage set
  with one template missing the citation (delta 5).
- `.claude/commands/scope.md`, `spec.md`, `align.md`, `build.md`, `validate.md`,
  `close.md` — **not** updated, and the omission is the delta rather than an
  oversight: a binding that restated the step would be `check-shim-restatement`'s
  subject, the kit templates being its corpus (delta 1).
- `.claude/agents/stage-session.md` — **not** updated. Its journal clause is
  correct for the actor it reaches; the finding was that it is the only surface
  carrying one, and delta (1) fixes that by adding a surface rather than by moving
  this one (delta 1).
- `TASK-QUEUE.md`, the `stage-journal-absence-caught-only-downstream` entry — moved
  from `## Deferred` to `## New Features` with `[design-pending]` swapped for this
  amendment's `[spec:]` ref (delta 8).
- The generated projections this unit stales — the on-site mirrors of the edited
  SPEC and templates, and any generated hook whose baked manifest the widened gate
  changes. All are rostered with their triggers and regen commands in
  `docs/site-architecture.md` §Generated projections (delta 9).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone, and the
      merge holds CLAUDE.md §The provenance seam: the dated ruling and its authority
      above land in git history, never in a kit SPEC's prose.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`), the none-remain half discharged at
      the iteration rather than at the commit.
- [ ] **The instruction reaches every path, proved by reading rather than by
      intent** — all six stage templates carry the step, and a stage run as an
      ordinary skill invocation with no lead reaches it, which is the path the
      agent definition never covered (delta 1).
- [ ] **The opener is inert at the kit default** — with
      `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=0` no journal is written, demonstrated,
      so a consumer who has not taken the feature gains no file (delta 2).
- [ ] **The three pinned opener behaviours each have a firing case** — a second
      session of one stage appends a heading and does not overwrite; `--simulate`
      writes nothing, verified by comparing the scratch dir before and after; and a
      first-stage entry's skeleton survives the boundary wipe, which is the
      ordering a naive implementation loses (delta 2).
- [ ] **The strengthened predicate is proved on the vacuity it exists to close** —
      an opened-but-unwritten journal **refuses** the next entry, and the refusal
      names it as unwritten rather than as empty. Proving this against the *old*
      predicate first is the point: it passes there, which is the failure delta (3)
      removes (delta 3).
- [ ] **The escape still works from the new state** — a session meeting the
      unwritten refusal can append the written stand-in and re-enter, so the
      refusal stays off the deadlock class (delta 3).
- [ ] **The lead step is exercised at least once this iteration** — the
      `--simulate` pre-dispatch read is run before a real dispatch and its output
      recorded, rather than the step landing as prose no session has taken
      (delta 4).
- [ ] **The gate fails on the fixture it was widened for** — the `bad/` fixture's
      one stage template lacking the citation reds, and the `good/` fixture passes;
      taken by running the pair, not by reading the predicate (delta 5).
- [ ] **No binding restates the step** — `check-shim-restatement` is green with all
      six templates carrying it, taken as a gate run (deltas 1, 5).
- [ ] **The refused shapes are recorded with their grounds** — the SPEC carries the
      turn-end refutation, the relayed-return proof and the status-quo pricing, so
      the next reader finds the disposition beside the option (delta 6).
- [ ] **The adjacent entries are untouched and said to be** — the two boundary
      keep-list entries and the concurrent-append entry are unedited, and the third
      one's page is named as touched-but-unanswered rather than silently shared
      (delta 7).
- [ ] **The promotion fits** — the promoted lead line measures 90 columns against a
      100-column budget, and the entry exits to `## Done` at build rather than
      demoting (delta 8).
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      mirrors, the enforcement and value rollups, and any generated hook the
      widened manifest stales (delta 9).
- [ ] **Removals propagated** — grepped every spec, template, skill, agent
      definition and committed workflow surface for the prior wording of the
      assertion's message and for any surface claiming the journal is the
      dispatch's obligation; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
