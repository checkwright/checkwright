# SPEC amendment: close-generated-finding-route

lifecycle-kit/SPEC.md §The committed gap inbox already states the defect's
mechanism in its own words: the drain "runs once, early in close, while close's
own later steps — audits, lesson disposition, staleness reads, release
disposition — necessarily generate findings that postdate it, leaving the one
stage downstream of every drain no legal channel at all." Its recurrence half was
closed by `recurrence-drain-input-widening`. What stays open, and what this
amendment closes, is the rest: a close-generated finding that is **not** a
recurrence has no drainer inside its own iteration.

**The route is ruled — route (a), the refusal distinguishes a post-close bullet
from a close-skipped one and routes the first into scope's ordinary intake.**
The operator's alternative, refusing capture at `bin/file-gap.sh` once close has
run, is ruled out on the ground §The committed gap inbox already carries:
refusing capture does not dissolve a finding, it pushes it back into session
context, which is the deferred-capture antipattern this channel exists to
prevent. Neither is re-argued here.

**What the current fallback actually costs, measured rather than predicted.**
The refusal's post-close recovery today tells the entering session to "promote
each directly to the deferred queue (or fix it), truncate the inbox to its
header, commit, and re-run enter-stage". Read against this repo's own history,
that instruction produces a **queue write made before any stamp exists, by a
session that has entered no stage**: `28ea8128 chore(scope): drain the post-close
gap bullet as the ninth firing` precedes `687cf956 chore(scope): stamp scope
entry, reset the iteration boundary`, and one boundary earlier `ce10d8c1
chore(scope): drain the post-close gap bullet as attesting 7` precedes that
iteration's stamp the same way. The commits attribute themselves to a stage that
has not started, and the running counts in their own subjects — "attesting 7",
"the ninth firing" — are the attestation that this is a standing shape, not an
incident.

## What changes

### 1. The boundary gap-inbox check keeps one detector and gains two dispositions

`bin/enter-stage.sh`'s iteration-boundary gap-inbox check is unchanged in
**detection** — it still fires on any `- ` bullet in the inbox at a first-stage
entry — and gains a **discriminator** on which of two dispositions it takes.
**Design-bearing.**

- **Close-skipped** — the closing iteration's cursor never reached the last
  configured stage. **Refuse**, exactly as today, with the *first* recovery only:
  run the closing stage's gap-drain step. A stage was skipped; the recovery is to
  run it, and the entry must not proceed.
- **Post-close** — the cursor sits at the last configured stage, so the closing
  stage has run and none is coming back. **Admit the entry.** The bullets are
  printed as an advisory on stderr naming them as this iteration's scope intake,
  and the stamp proceeds.

This adds **no second detector**, which matters because §The committed gap inbox
already ruled one out: "the existing refusal already detects it, and what was
missing was the message's actionability." One detector, two dispositions, is that
ruling carried out rather than revisited.

**Two edges, both following precedents the script already carries.** A closing
iteration that was never named (`$cur_iter` is the `—` placeholder) has no close
to have skipped, so it takes the post-close disposition — the same guard the
`LIFECYCLE_KIT_BOUNDARY_REQUIRE` block applies one block down for the same
reason. And an inbox holding bullets with no cursor at all (a fresh consumer's
first boundary) is the same case: nothing was skipped, so nothing is refused.

**Why admitting, and not merely a better-worded refusal.** A refusal that picks
the right message still leaves the queue write where the measurement above found
it — before the stamp, outside the state machine, attributed to a stage that has
not started. "Routes the first into scope's ordinary intake" is only satisfiable
inside scope: the intake is scope's, so the entry has to succeed for the finding
to reach it. What the change buys is that the disposition becomes an ordinary
in-stage queue write in the stage that writes the queue anyway.

**What the admission costs, and why the loss is smaller than it looks.** The
invariant "no gap outlives its iteration untriaged" is not weakened, because a
post-close bullet never had a drainer in the iteration it was filed in — that is
the defect, and no message could have supplied one. On admission the bullet stops
being post-close: it becomes an ordinary mid-iteration bullet of the **new**
iteration, and it therefore acquires the drainer it never had, the new
iteration's mandatory close drain. Scope is directed to take it earlier because
scope is writing the queue anyway; if scope does not, close does. That is the
forcing function, and it is the existing one rather than a new one.

**The honest limit, stated because it is real.** The finding's *disposition* then
lands in the next iteration's ledger, which is the record defect the entry names
and which cannot be repaired: the finding postdates its own iteration's close, so
no iteration-correct ledger position exists for it. What is repaired is legibility
— §3 obliges the promoted entry's provenance sentence to carry the bullet's own
date and the iteration whose close generated it, so a reader sees where the
finding came from even though its disposition sits one iteration later. A
record that is late and says so is strictly better than one that is late and
silent, which is what "silently pushes one iteration's record across the
boundary" describes today.

### 2. The discriminator is a cursor read, derived once

The predicate is **"did the closing iteration's cursor reach the last configured
stage"**, read from the state file the script already holds. **Mechanical.**

`bin/file-gap.sh` already computes exactly this predicate for its capture-time
warning, and `bin/enter-stage.sh` already parses the last stamp into its iteration
and stage tokens before the gap-inbox block runs, so neither tool needs new input.
What the two need is to stop spelling one test twice: `lib/stages.sh` gains
`lifecycle_closing_stage_reached [<state-file>]`, returning success when the
cursor equals the last member of `LIFECYCLE_KIT_STAGES`, and both call sites read
it. That is derivation-first on a predicate whose two spellings must agree by
construction — a filer warned at capture that "none is left to drain it" is warned
by the same test that later admits the bullet, rather than by a lookalike.

**No knob is minted and none is possible here.** The last configured stage is
already `LIFECYCLE_KIT_STAGES`'s last member, so the predicate is config-derived
in every consumer with nothing to configure; a knob would be a second source for
a fact the stage roster already owns.

### 3. Candidate 4 — deriving post-close-ness from git — is refused, with its grounds

A fourth candidate was relayed for weighing: post-close-ness is git-derivable,
"has the inbox been truncated since the close stamp commit?", separating the
cases with no marker field. It is refused on evidence, and recorded here so it is
not re-drafted. **Design-bearing.**

- **It is wrong on the case that matters.** The predicate assumes close's drain
  and close's stamp are commensurable in commit order. They are not: measured
  over the four most recently closed iterations, the truncation is **never** in
  the stamp's commit and is **always** a later, separately authored one
  (`f0f2060c`→`22c5b0a4`, `93fcf17f`→`4e194b8a`, `c9f73f7f`→`27176682`,
  `4c863623`→`b2adf007`). So "truncated since the close stamp" is true of every
  normally-closed iteration, and the one case it distinguishes is a close that
  stamped and then skipped its drain — where it answers *close-skipped* and hands
  the entering session a recovery naming a stage that is gone. The cursor read is
  right in that case, because close did run.
- **It cannot answer the other case at all.** Where close was genuinely skipped
  there is no close-stamp commit to anchor on, so the git predicate degenerates
  into "is there a close stamp?" — which is the cursor read, obtained from a file
  the script already reads.
- **It would be the first `git` invocation this tool has ever made.** `enter-stage.sh`
  shells out to git nowhere today; every decision it takes is a read of `$QUEUE`
  and `$STATE`. Adding history-dependence to the state machine's only writer makes
  the entry decision non-hermetic, and §4's fixture — the first this refusal has
  ever had — would have to construct commit history rather than three files.
- **Its stated virtue does not discriminate.** "Needs no marker field, so
  §check-gap-inbox-neutrality's two-field bound stays intact" is equally true of
  the cursor read, which writes nothing anywhere. The bound is untouched by both,
  so it selects neither.

### 4. The refusal gains its first fixture

The iteration-boundary gap-inbox path has **no fixture coverage today** — no test
under `gate-tests/` drives `enter-stage.sh` with a non-empty inbox, so both its
refusal and its recovery text are unpinned. A new `gate-tests/gap-inbox-route.test.sh`
pins the branch: a sandboxed queue, state file and inbox, entered through
`bin/enter-stage.sh`, asserting the close-skipped case refuses with the drain
recovery and writes nothing, the post-close case stamps and carries the bullets
with the advisory on stderr, and the never-named-iteration edge takes the
post-close branch. **Design-bearing.**

`gate-tests/boundary-scratch-wipe.test.sh` is the harness template — it already
drives `enter-stage.sh scope` against a sandbox — and no fixture *pair* is owed:
this is a `bin/` tool, not a gate.

### 5. Scope takes the carried bullets as an intake lane

`templates/stages/scope.md` gains a bounded intake step for bullets carried
across the boundary: each gets exactly one disposition — promoted to a queue
entry, fixed inline that session, or discarded with cause in the commit message —
after which the entering session truncates the inbox to its header in the same
commit. That is the drain's own disposition set, run by the stage that can now
legally run it. **Design-bearing.**

Two properties are obliged rather than suggested. The disposition happens **after
the stamp**, in-stage, which is the whole point of §1. And a promoted entry's
provenance sentence carries the **bullet's own date** and names the iteration
whose close generated it, which is what §1's honest limit trades legibility for.
No linked-and-skipped middle state, per the gap-disposition rule.

### 6. The lead's discharge of the direct-stamp obligation is named

`recurrence-drain-input-widening` ruled the direct recurrence stamp *obliged*,
attaching the duty to the judgment rather than to the channel. A **lead** cannot
discharge it as written: `templates/lead.md` §Stamps are authoritative forbids the
lead every queue write, so a lead that judges a recurrence has exactly one
channel, this inbox — and a lead's judgment is made at the boundary, which is
where the refusal this amendment fixes stands. **Design-bearing.**

The discharge is stated rather than invented, because the channel already
supplies it: **the lead's obligation is discharged by filing the judgment and its
grounds into the bullet's prose**, and the stamp is then made by the session that
may write the queue, judging from that prose. That is exactly the shape the drain
already has — §The committed gap inbox already makes the bullet's prose "the
grounds the drain judges the recurrence from" — so no new authority and no new
producer is created. What §1 adds is that the route no longer dead-ends: a
boundary-filed lead judgment reaches a judge instead of a refusal.

This settles the **authority** half only. `recurrence-obligation-residency` is
the **reach** half — stages that may write the queue but never load the rule —
and stays deferred, untouched.

### 7. The simulate arm reports the branch it would take

Under `--simulate`, the post-close disposition reports that the entry **would
proceed**, naming how many bullets it would carry into scope's intake; the
close-skipped disposition keeps reporting a refusal. **Mechanical.**

**Seam with a sibling unit in this same iteration.** `simulate-recovery-unrelayed`
owns whether the *recovery text* is relayed under simulate — every `help:` line
in this block sits after the simulate branch's own `exit 1`. This amendment owns
**which branch fires**; that entry owns **what the refusing branch prints**. Both
edit the same block, so whichever lands second re-reads the other's shape; neither
decides the other's question.

## Producers and consumers

**No new state, no new event, no new field, and no new configuration.** The inbox
grammar is untouched at two fields; the state file's grammar is untouched; no knob
is minted. What is introduced is one shell function, one branch, and one advisory
message.

- **`lifecycle_closing_stage_reached` (new interface, `lib/stages.sh`).**
  *Producer:* the library, sourced unconditionally by every kit tool — reachable
  with no enabling config, since `LIFECYCLE_KIT_STAGES` has a platform default and
  the function derives from it. *Consumers, both named and both existing:*
  `bin/enter-stage.sh`'s boundary gap-inbox block, at the transition where it
  chooses a disposition; and `bin/file-gap.sh`'s capture-time warning, at the
  transition where it tells a filer which consequence they are buying. It has no
  third consumer and must not acquire one silently — it is a predicate over the
  cursor, and `lifecycle_current_stage` remains the general reader.
- **The post-close advisory (new message).** *Producer:* `bin/enter-stage.sh`, on
  the admitted branch, on stderr — stderr because the stamp confirmation is this
  tool's stdout contract. *Consumers:* the entering scope session, which acts on
  it at §5's intake step; and, under `--simulate`, the lead, which reads it at the
  dispatch-gating transition `templates/lead.md` directs it to.
- **The scope intake step (new obligation).** *Producer:* the scope stage session.
  *Consumer:* the queue file, and the inbox itself — truncated to its header in
  the same commit as the disposition, which is the audit-artifact property §The
  committed gap inbox already rules load-bearing. Reachable at every boundary with
  no enabling config.
- **Every field has a named reader.** No field is added on any surface. The
  bullet keeps its two fields with their existing readers; the provenance date
  §5 obliges is ordinary entry prose in the promoted entry, read by the same
  reader that reads the rest of that entry.

**Red conditions — this delta narrows a corpus, so each reader's red condition is
named rather than its subject.** The narrowing is deliberate: the set of states in
which the boundary entry refuses gets **smaller** by exactly the post-close case.

- **`bin/enter-stage.sh`'s own gap-inbox refusal** — reds (exit 1, nothing
  written) on *finding some* bullet at a first-stage entry. Monotone, and the
  violation set is shrunk on purpose; what is given up is stated in §1's cost
  paragraph and nowhere else, so it cannot be lost.
- **`check-gap-inbox-neutrality`** — reds on *finding some* malformed bullet or
  interposed verdict below the header. Monotone, and its corpus (the inbox's
  content) is not narrowed at all: nothing here writes, reshapes or removes a
  bullet. Clearable by inspection.
- **`check-knob-default-coupling`** — reds on *finding none*, specifically on a
  knob with no literal default stated in its owning SPEC. **Non-monotone, and
  named for that reason:** it is the reader a knob-minting design would trip. §2
  mints no knob, which is what clears it — not the narrowing.
- **`check-merge-attrs`** — reds on *finding none*, a supersede- or union-set
  member with no matching `.gitattributes` line. Non-monotone. Cleared because
  the sets are untouched: the inbox stays `merge=union` and stays out of
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`, which is precisely what lets an admitted
  bullet survive the boundary reset.
- **`check-close-surfaces`** — reds on *finding none*, a declared close-surface
  whose forcing pointer does not resolve. Non-monotone. The inbox's declaration
  (`close-surface: .workflow/gap-inbox.md forced=lifecycle-kit/SPEC.md
  §bin/enter-stage.sh`) keeps both its surface and its forcing pointer; the
  forcing function becomes branch-conditional, which the pointed-at section
  restates, so the declaration stands unchanged and is verified rather than
  assumed at merge.
- **`check-shim-restatement`** — reds on *finding some* n-gram shared between a
  consumer skill and its template beyond the budget (`LIFECYCLE_KIT_SHIM_NGRAM`).
  Monotone, but it is the reader §5 most plausibly trips, because the intake step
  lands in the template and this repo's `.claude/commands/scope.md` binds it. The
  binding states what is consumer-specific and points at the template for the
  rule; it does not restate the step.
- **`check-comment-tier` / `check-spec-pointer`** — red on *finding some*
  non-directive comment, and on *finding some* `spec:` citation resolving to no
  heading. Monotone. Every new `# spec:` line in `lib/stages.sh` and
  `bin/enter-stage.sh` is a one-line binding to a real heading.
- **`check-stage-evidence` and `check-stage-entry`** — unaffected in both
  directions. Neither reads the inbox, and the stamp protocol is untouched: the
  admitted entry stamps exactly as any first-stage entry does.

## Existing sections updated

- **lifecycle-kit/SPEC.md §The committed gap inbox** — owned by deltas 1, 5 and 6,
  and the section carrying the substantive change. Its "**The boundary refusal**"
  paragraph is rewritten: one detector, two dispositions, the discriminator named
  as the cursor read, and the post-close branch described as an admission with a
  carried-bullet advisory rather than a second recovery message. Its sentence "No
  second detector is added for the post-close window" is **kept and re-read** —
  it is now satisfied more exactly than before, and the amendment says so rather
  than deleting it. The "Producers and consumers" paragraph gains scope's intake
  as a second consumer beside close's drain. The direct-stamp ruling paragraph
  gains §6's one-sentence statement of the lead's discharge. The refused-shape
  paragraphs gain §3's fourth refusal.
- **lifecycle-kit/SPEC.md §bin/enter-stage.sh** — owned by deltas 1 and 7. Its
  refusal roster gains the branch, and its `--simulate` roster gains the
  would-proceed report. The roster's known gap-inbox omission is
  `simulate-recovery-unrelayed`'s to fix, not this amendment's, and the two edits
  land in the same paragraph.
- **lifecycle-kit/SPEC.md §lib/stages.sh** — owned by delta 2, for
  `lifecycle_closing_stage_reached`: what it derives, from which knob, and the
  two callers that read it.
- **lifecycle-kit/SPEC.md §Testing** — owned by delta 4, for the new
  fixture-runner member and what it pins.
- **lifecycle-kit/templates/stages/scope.md** — owned by delta 5, gaining the
  intake step. **lifecycle-kit/templates/stages/close.md** — owned by delta 1,
  one clause on its drain step: a finding generated by a later close step routes
  to the inbox and is carried, rather than being back-dated into a drain that has
  already run.
- **lifecycle-kit/templates/lead.md §Stamps are authoritative** — owned by delta
  6, one clause naming the inbox as the lead's discharge of the direct-stamp
  obligation, since that section is where the lead's no-queue-write rule is
  stated and therefore where a reader hits the question.
- **`.claude/commands/scope.md`** — owned by delta 5, this repo's binding for the
  new step; written as a binding, not a restatement (see the
  `check-shim-restatement` red condition above).
- **docs mirror of every file above** — regenerated, not edited.

**Not updated, stated so the omission is read as a decision.**
§check-gap-inbox-neutrality is untouched: no field is added, so its two-field
bound is neither widened nor cited as licence. §Multi-operator semantics is
untouched: the inbox's `merge=union` and the supersede set are unchanged.
`bin/file-gap.sh`'s warning text is unchanged — only the predicate behind it
moves — because the consequence it names stays true: once the closing stage has
finished, no stage of *that* iteration is left to drain the bullet.

## The seam ruling

**Kit mechanism:** all of it. The discriminator, the two dispositions, the
library predicate, the intake step and the fixture are generic lifecycle
mechanism, true of any consumer that vendors lifecycle-kit and runs a stage
roster with a closing stage. Nothing here reads a term list, a vocabulary or a
product constant.

**Consumer config:** none added, and the refusal to add one is the ruling. The
discriminator derives from `LIFECYCLE_KIT_STAGES`, which every consumer already
configures; a knob offering the old always-refuse behaviour would let a consumer
configure away an intake route rather than choose a value, which is the kind of
thing a SPEC rules and a knob does not offer.

**Private rule content:** none is involved. The measurements in the opening
paragraph and in §3 are over this repo's own public commit history, and what
reaches governed prose from them is qualitative — an ordering claim and a branch
— with no count and no private name crossing into a kit literal. The commit shas
stay in this amendment, which is deleted at merge, and the merged prose states
the ordering rather than the sample.

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
