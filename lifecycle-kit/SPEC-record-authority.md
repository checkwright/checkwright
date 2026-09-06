# SPEC amendment: record-authority

One amendment, **paired from two feature entries**, because the two halves are one design and each
states the other's boundary: the closing stage gains the obligation to **repair** the ruling record,
and the record gains the declarations that let a probe say **when a repair is owed**. Authority
without detection is a licence nobody exercises — the aged fact this amendment's worked instance
repairs has sat unrepaired through two closes that each looked straight at it. Detection without
authority is a probe that can only escalate what nobody may fix.

**The authority is already half granted, and stating that is what keeps this from reading as a
reversal.** The ruling record's own completion-time contract already says **"a fact that has aged is
corrected where it stands. A correction is never appended, and a superseded sentence is never left
standing beside the sentence that corrects it."** What is missing is narrower and exact: a closing
session that meets an aged fact **inside the text of a ruling that is still in force** stops, because
the standing rule beside it says a recorded ruling is closed and reversing, demoting or re-scoping
one is operator-class. Both rules are correct and neither says which governs. This amendment draws
that one line and adds nothing else to close's reach.

**The line is a codification of an attested practice, not a new licence.** Read off what close has
actually done: it **retires** a spent paragraph, applying a multi-conjunct discharge test unaided; it
**corrects** an aged count inside a ruling; and it **declines** exactly where the correction would
touch operator-attributed text of a ruling still in force, escalating instead — twice, on the same
fact. So the practice already has the boundary in it. What the record lacks is the sentence.

**The architecture is forced by the kit's own contract, not chosen.** `templates/consult.md` and
this SPEC both rule that write authority on a ruling record **is not a kit template's to grant** —
"it belongs to the ruling record itself, which states who may record a ruling there and under what
condition", and what a template adds "on top of that authority is **obligation**." So this amendment
splits along that seam by construction:

- the **authority boundary** lands on the *consumer's* ruling record, in its own header contract,
  where a dated operator stamp is the sanctioned form;
- the **obligation**, the declaration grammar and the probe land in *this kit*, undated and generic,
  naming "the ruling record" and never any consumer's file.

That split is also what holds the provenance seam here without further argument.

**This amendment is one of five in an iteration spanning gate-sdk, lifecycle-kit and drift-kit**, so
`check-stage-entry` assertion C is armed on the amendment file count alone, and the audit stage's
stamp will be demanded at build's entry. It is also one of **three** lifecycle-kit amendments in
flight, so the Definition of Done's none-remain assertion is discharged at the **iteration**, not at
this file's own merge commit — the branch §Merging an amendment already carries for exactly this
case.

## What changes

### (1) The record's header contract gains the correction boundary, and reversal is untouched

The consumer's ruling record gains one paragraph beside its existing record/retire symmetry, drawing
the line its two standing rules leave undrawn {design-bearing}. Three acts, named and separated:

- **Correcting an aged fact** — a slug the queue has retired, a count the tree has moved, a pointer
  whose target no longer carries the claim, a condition naming a unit or a gate that does not exist.
  The fact is wrong *about the world*; correcting it decides nothing, and the record already directs
  that it be corrected where it stands. **Session-class, including inside a live ruling's text.**
- **Retiring a spent ruling** — the ruling directed something that has already happened, so deleting
  the record decides nothing. **Session-class**, as the record already says.
- **Reversing, demoting or re-scoping** — making a closed ruling stop being the rule, narrowing what
  it reaches, or moving it down a priority order. **Operator-class, unchanged**, however
  well-grounded the finding and however urgent the fix.

**What makes the first act safe is that it is decidable against the tree, and that is the test rather
than the category.** A correction qualifies only when the aged fact is one an oracle settles — a
slug's presence in the live set, a count re-derivable by a command, a pointer's target. A "fact" that
is really a judgment about whether the ruling still ought to govern is a reversal wearing a
correction's clothes, and the sentence that distinguishes them is the one worth writing: **if the
repair would change what the ruling directs, it is not a repair.**

**The honest limit is stated with it.** This is an authoring contract, not a gate. Nothing
mechanizes *the operator closed this* and nothing should, so what the boundary buys is that a session
meeting an aged fact inside a live ruling now has an answer other than stopping — and a session
tempted to re-decide still has to write down that it is only correcting.

### (2) Retirement is tested by MOOD before it is tested by completion

A ruling's discharge test runs on what the ruling **is**, not on whether its subject has landed
{design-bearing}. A ruling that **directed work** is spent when the work lands. A ruling that
**establishes a rule** is never spent while the rule governs, however completely the mechanism that
prompted it has shipped.

**This is the trap that makes the newest rulings the most dangerous to judge**, and it is why the
test is stated before the retirement contract rather than after it: a rule-establishing ruling whose
mechanism landed in the very commit range that makes a retirement sweep due looks retirable under a
naive subject-finished read, and three of them have looked that way at once. The mood test costs one
question and it is the one question a completion test cannot ask itself.

**Where a rule-establishing ruling's content belongs when its home changes** is the move already
worked in this tree: it is retired from the record and its content **relocates** to the surface that
owns the mechanism, which is a relocation rather than a deletion and is not what the completion
trigger describes. Naming that as a third disposition beside delete-and-keep is what stops a later
session reading relocation as loss.

### (3) A ruling declares its own name, and the name is what the citing side can be reached by

The record gains a body-line declaration, `ruling: <name>`, naming the ruling in the words other
surfaces will use for it {design-bearing}. It is a declaration and not a tag, on the pattern this
format already defines for a body line whose readers scan a line of its own — a tag is refused
outright, because a tag marks a move across a pending/ready boundary and this marks none, which is
the further-tag test's own words.

**The declaration exists for the *citing* side, and that is the finding no earlier design had.** The
inbound half of this class was diagnosed as a citation resolving to a surviving section while the
ruling inside it is gone. Measured against a real cohort, that diagnosis is too narrow twice over:
in the largest instance on record — a superseded run restated in the present tense across eleven
live entries — **eight of the eleven did name the file, seven named the section, and every one of
them still resolved**, because the heading survived and only the body under it was rewritten. So a
probe over citation *targets* reaches zero of eleven. What every one of them shared was the ruling's
**proper noun** in a present-tense claim, anchored to no file at all; and across the tree that shape
is the large majority — roughly a hundred and fifteen sites cite a ruling by date or by proper noun
while naming no file.

**A near miss inside that same cohort sets the declaration's grammar.** A twelfth site survived the
drain because it wrote the ruling's noun one word differently. A single declared name is therefore
insufficient by construction, and the declaration takes **one or more** names — the ruling's own and
each variant a surface has actually used — appended and never rewritten, on the same
append-never-rewrite discipline the re-filing declaration already carries. The self-naming
discipline every body-line declaration in this format inherits applies here too and is not
re-argued.

**No retrofit.** Existing rulings are not back-filled and there is no presence gate, on the ground
this format already states for a declaration written under judgment: a scanner cannot demand one. A
ruling with no declaration is simply one the citing-side arm cannot reach, and delta 6 makes that
visible rather than silent.

### (4) A discharge condition is declared as an ORACLE, never as a predicate the kit interprets

A ruling conditioned on a future event gains `discharge: <name> <oracle>` — the ruling's declared
name and a command whose output settles whether the event has fired {design-bearing}.

**Inventing a condition language was the obvious design and is refused on a structural fact.** Every
machine-parsed conditional in this tree resolves its condition against one domain: the queue's
live/done partition. That is not an accident of implementation — a queue entry has a slug and a pool
with an exit, and a ruling has neither. A condition grammar for rulings would therefore have to mint
the first non-queue condition domain, and would have to be expressive enough for the conditions the
record actually carries: a CI leg producing an artifact, an oracle's owed count reaching zero, three
consecutive closes reading a metric at or below zero, a release channel flipping. Enumerating those
is a vocabulary that would be wrong the day after it landed.

**The oracle form reuses a doctrine this kit already ships.** The survey record's contract is that a
carried finding is "a citation with a falsifiable staleness witness, never a substitute for the
oracle" — the consuming stage re-runs the oracle and cites the finding only if the verdict holds.
A discharge condition is the same shape read forward: the ruling names the command, and the session
that would rely on the ruling runs it. The kit interprets nothing; it dispatches and reports.

**The oracle's contract is the exit status and one line of output.** Non-zero or empty means the
condition has **not** fired and the ruling stands; a clean run with output means it **has**, and the
ruling is a retirement candidate a session must judge under delta 2's mood test. A dispatch failure
is neither and reports as such — the same three-band shape this kit's own arms already use, so a
broken oracle can never read as a fired condition.

**A condition no command can settle takes the literal operand `manual`**, followed by the prose
condition. This is not an escape hatch and is the case that forced the operand: the live record
carries a ruling whose condition resolves against an untracked local brief, which no tracked oracle
can reach and which a probe scoped to tracked surfaces would silently skip. Declaring it `manual`
makes it **report as owed to judgment** instead, which is the difference between a probe that knows
what it cannot answer and one that answers wrongly.

### (5) The knobs, and the kit ships no path

Three, on the config-via-env convention {design-bearing}:

- `LIFECYCLE_KIT_RULING_RECORD` — the consumer's ruling-record path. **Default empty**, so the whole
  machinery is inert for a consumer that keeps no such record, exactly as the pre-flight valve's
  ledger knob defaults empty and no valve exists. A kit that shipped a default filename would be
  asserting that every adopter keeps this artifact under this name.
- `LIFECYCLE_KIT_RULING_CITERS` — the globs the citing-side arm sweeps. Consumer-configured with no
  kit default for the same reason the prose-surface globs elsewhere are the consumer's: which of a
  tree's surfaces argue about rulings is a fact about that tree.
- `LIFECYCLE_KIT_RULING_ORACLE_TIMEOUT` — a bound on a declared oracle, so a hung command cannot
  wedge the arm that dispatches it.

**No knob carries a condition vocabulary or an authority vocabulary**, and this is where a reader
looking for one finds out why: nothing in the machinery branches on such a value, so a knob holding
one would have the knob-citation gate as its only reader — the refusal this kit already records for
ruling-authority names, applying unchanged.

### (6) The probe is one arm with two reports, and it escalates rather than proposes

A non-gate arm reads the record and reports two things {design-bearing}.

**The discharge report** — one row per declared condition: the ruling's name, the oracle's verdict
band, and, for a fired one, the oracle's own output line as the evidence. A `manual` condition
reports as owed-to-judgment with its prose.

**The citing report** — for each ruling whose condition has fired or whose retirement a session has
recorded, the sites across the citing corpus that name it, with the citing line quoted verbatim so a
reader judges the claim rather than the match.

**Escalation-only, and the refusal is the design.** The arm proposes no edit, retires nothing, and
never says a citation is wrong. That boundary answers this kit's own standing objection to
mechanizing this class — a claim about *what a finding is* has no syntactic tell separating "this
recurred" from "this is about" — and the same objection reaches a citation: a surface may name a
retired ruling as *evidence* about the past, which is correct prose, or restate it as a *live rule*,
which is the defect, and no scanner tells them apart. The instance that proves it is inside the
eleven-entry cohort itself: one of the eleven recorded a **live** operator ruling under the retired
run's name, so the remedy there was to re-name the run and leave the ruling untouched. **A probe
blind to that distinction proposes reversals**, which is the one act this whole amendment holds
operator-class.

**The producer/checker split is the one this tree already runs on ports.** A declaration *reports*
and a separate reader *escalates when the named thing changes state* — that pairing is why a stale
hold cannot silently under-count owed work, and it is copied here rather than re-invented.

### (7) An undeclared condition is reported, not inherited

A ruling whose prose names a future event and which carries no `discharge:` declaration is reported
as **undeclared** {design-bearing}, the closure the close-surface roster already takes: "the roster
reports the hole instead of inheriting it, which is the whole difference between a derived roster and
a maintained one."

**Without this the no-retrofit decision is a silent hole rather than an honest one.** The record
carries nine future-conditioned rulings today and six of them already name a discharge event in prose
under the record's own authoring convention. None is machine-readable. If the arm reported only what
was declared, a record with one declaration would report one condition and look complete. Reporting
the undeclared count is what makes the gap between the prose convention and the declaration visible
to the session best placed to close it.

**The detection is deliberately weak and is stated as weak.** It is a prose match over
forward-looking phrasing, which is FP-bearing by construction — the same honest posture the queue's
own forward-precondition gate takes about the same problem. It reports, it does not red, and a false
positive costs a reader one line.

### (8) Close gains the obligation, generically, and the record is added to its own roster

The closing stage's ritual gains a step: **read the ruling record for aged facts and fired
conditions, and repair what the record's own contract puts in the session's hands** {design-bearing}.
The step names the record generically and states the boundary by pointing at the record's contract
rather than restating it — the kit adds obligation, the record grants authority.

**It lands beside the staleness pass, not the brevity pass.** The stage already has a step whose
predicate is *is it still true?* over top-level docs and a later one asking *is each block worth its
cost?* over every governed prose file. A record repair is the first question exactly, and running it
in the second would put a correction inside a pass whose licence is compression.

**A repair is fix-shaped by the drain's own litmus and needs no new criterion**: it adds no governed
name and lands test-and-doc-complete in the closing session's own commit, which is what that litmus
already admits. Stating it stops a later session routing a two-word correction through a Deferred
entry.

**The record joins the stage's declared inbound roster** so the obligation is derived rather than
remembered, and so a consumer that configures no ruling record sees the step skip rather than fail.

### (9) The worked instance, and it is this iteration's own

The record's directive paragraph names a unit slug the queue retired and superseded, and names a
discharge event — that unit landing and its gate turning green — where neither the unit nor the gate
exists {mechanical}. It is a live ruling, so it is not retirable; its named slug is an aged fact, so
under delta 1 it is correctable where it stands; and the correction re-points the name and the
discharge event at the surviving successor without touching one word of what the ruling directs.

**Two prior closes met this fact and escalated instead**, each recording that editing a recorded
ruling's text is not the closing stage's call. That is the exact judgment delta 1 answers, and the
repair is this amendment's proof that the line is drawable rather than merely stateable.

### (10) Three false premises are corrected, two of them in the surfaces that carry this class

{mechanical}

- **The host entry's claim that the 2026-08-08 pruning directive authorizes the prune is false**, and
  the directive says so in its own words: it sanctioned the practice, named neither a trigger nor an
  authority, and stated that nothing in it licensed a session to prune. What authorized pruning was
  the amendment that landed the record's header contract. The entry conflates the directive, the
  amendment, and a slug the queue has since retired — and cites the dead slug while doing it, which
  makes the entry that owns this defect class an instance of it.
- **The audit roster's claim that the prunes left the conditioned-ruling corpus empty is false.** The
  corpus is nine deep, enumerated this session. That claim is the reason no later sweep looked.
- **The inbound half's diagnosis is too narrow**, per delta 3: a citation naming a surviving section
  is one shape, and the measured majority name no file at all.

### (11) A discharged directive is recorded as a closed ruling on the same pass

Where an operator directive is discharged by **falsification** — the premise it rested on measured
false rather than the work being done — the closing stage records that discharge as a closed-ruling
line on the record, in the pass delta 8 obliges {mechanical}.

**The reason is the one this whole amendment turns on.** A directive discharged by falsification
leaves no landed unit and no queue trace, so a later session reading only the directive re-derives
the entire investigation that falsified it. Recording the discharge costs one line and is the
cheapest instance of the record doing its job. This iteration carries exactly one such directive and
it is the first application.

## Producers and consumers

**New declaration: `ruling: <name>[ <name>…]`.**
*Producer* — a session recording or retiring a ruling, by hand, at the moment it writes the ruling;
its enabling configuration is `LIFECYCLE_KIT_RULING_RECORD` being set, which this repo's config seam
sets and an adopter without a record leaves empty, making the whole grammar inert rather than broken.
*Consumers* — the probe's **citing report** (delta 6), which uses each name as the search key over
`LIFECYCLE_KIT_RULING_CITERS`, at the transition where a session asks what would go stale if this
ruling were retired; and a human reader, who gets the ruling's canonical noun without inferring it.
*Every field has a named reader*: the first name is read by both consumers; each additional name is
read by the citing report alone, and exists because a variant spelling has already cost one missed
site.

**New declaration: `discharge: <name> <oracle>`.**
*Producer* — the same session, at the same moment, under the record's existing authoring convention
that a ruling able to name its own discharge event says so in its own text; the declaration makes
that convention resolvable rather than introducing it. *Consumer* — the probe's **discharge report**,
which dispatches the oracle at the transition where a closing session runs delta 8's step.
*Field readers*: `<name>` is read by the report to join the row to its ruling and to delta 3's names;
`<oracle>` is read by the dispatcher, or, when it is the literal `manual`, by the report's
owed-to-judgment row and by no dispatcher at all.

**New arm: the ruling-staleness probe.**
*Producer* — a bridged-arm table row, reachable through the front-end with no edit to it, resolving
the three knobs delta 5 declares. Its family is the reporting one: it renders a document and its
exit carries no verdict a caller branches on, because it escalates to a reader rather than gating an
act. *Consumers* — the closing stage's step (delta 8), reading both reports; and any session that
runs it by hand before relying on a ruling.

**Existing surface whose contract changes: the ruling record's header.**
*Producer* — the operator, through the ruling this amendment's own commit records; the record is the
one surface in this tree where a dated operator stamp is the sanctioned form, so the provenance lands
there and nowhere in the kit. *Consumers* — every session that meets an aged fact in the record, at
the moment it would otherwise stop; and the closing stage, through delta 8's step.

**Existing surface whose obligation changes: the closing stage's template.**
*Producer* — this amendment. *Consumers* — every closing session, through the ritual it loads; and
the stage's declared inbound roster, which reports the record as a close surface so the obligation is
derived rather than remembered.

**Narrowing check.** This delta set narrows one thing: the set of acts a session must escalate rather
than perform, which shrinks by exactly the correction class delta 1 defines. No file, field,
invocation, fixture or corpus is removed, so point 5's monotonicity question does not arise for any
scanner — and this paragraph says so rather than leaving the absence to be read as an omission. The
readers that do red are named with their conditions. The manifest-family gates that scan the ruling
record red on a **temporal marker outside an exempt site**, on a **markdown link that does not
resolve**, on a **tracking claim disagreeing with git**, and on a **measured value disagreeing with
its oracle** — every one of them non-monotone under an edit, which is why a repair is committed with
the full battery rather than cleared by inspection. **And one thing a session will get wrong without
being told**: those gates' triggers are narrower than their corpora, so a commit touching only the
ruling record fires almost none of them. The battery is the oracle here, never the hook.

## Existing sections updated

- **The consumer's ruling record — its header contract** — the correction boundary beside the
  existing record/retire symmetry, the mood test before the completion triggers, and the two
  declarations' grammar as the record's own authoring convention made resolvable (deltas 1, 2, 3
  and 4).
- **The consumer's ruling record — the aged directive paragraph** — the retired slug and the
  discharge event naming a unit and a gate that do not exist, corrected where they stand (delta 9).
- **lifecycle-kit/templates/stages/close.md** — the ritual gains the record-repair step, placed with
  the staleness pass and pointing at the record's own contract for the boundary rather than
  restating it; and the falsification-discharge obligation (deltas 8 and 11).
- **lifecycle-kit/SPEC.md §templates/stages/** — the closing stage's contract gains the new step in
  its inventory, and the statement that a repair is fix-shaped by the drain's existing litmus rather
  than by a new criterion (delta 8).
- **lifecycle-kit/SPEC.md §The close-surface roster** — the ruling record joins the declared roster,
  with the empty-knob skip stated (delta 8).
- **lifecycle-kit/SPEC.md §Layout and configuration** — the three knobs, their defaults, and the
  refusal of a condition-vocabulary knob beside the existing refusal of an authority-vocabulary one
  (delta 5).
- **lifecycle-kit/SPEC.md — a new per-component section for the probe arm** — the two reports, the
  oracle contract's three bands, the `manual` operand, the undeclared closure, and the
  escalation-only boundary with the standing no-syntactic-tell objection answered (deltas 4, 6
  and 7).
- **lifecycle-kit/SPEC.md §The committed gap inbox** — its no-syntactic-tell paragraph gains the
  citing-side instance, since the same objection now has a second application and a second answer
  (delta 6).
- **The consumer's audit roster** — its ruling-retirement row, whose recorded claim that the
  conditioned-ruling corpus is empty is false and whose due-events now have a probe behind them
  (deltas 7 and 10).
- **The host entries' own bodies** — the pruning-directive premise, corrected on promotion rather
  than carried (delta 10).
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in the consumer's site-architecture page; it stales on a kit SPEC byte no delta authors --> the on-site SPEC mirror.
- <!-- update-target-exempt: derived surfaces that stale on a templates/ markdown byte, each with its own freshness gate and regen command --> the per-kit footprint page and the value rollup block.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and a
      named consumer; every new field has a named reader at a named transition.
- [ ] **The seam is held at the word level** — the kit files name "the ruling record" and never a
      consumer path; they carry no date, no authority-plus-channel attribution, no queue slug and no
      pointer to any consumer's record. The dated operator provenance for the authority boundary
      lands on the record itself, where that form is sanctioned, and in the commit.
- [ ] **No tag is minted** — both new grammars are body-line declarations, and the further-tag test
      is stated as the reason rather than left implicit.
- [ ] **The probe is escalation-only, proved by its own fixtures** — a case where a fired ruling is
      cited as past evidence and a case where it is restated as a live rule produce the *same*
      report, because the arm does not distinguish them and must not claim to.
- [ ] **The worked instance lands with the mechanism** — the aged directive paragraph is repaired in
      the same unit that grants the repair, and the repair changes no word of what the ruling
      directs.
- [ ] **The undeclared count is non-zero on the live record and is reported as such** — a probe that
      reports a complete record over a corpus with nine conditioned rulings and no declarations is
      reporting its own blindness.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge. The none-remain half is discharged **at the
      iteration**, three lifecycle-kit amendments being in flight, so only the batch merging the last
      of them satisfies `ls lifecycle-kit/SPEC-*.md`.
- [ ] **Removals propagated** — grepped every spec, template and doc for the corrected pruning-
      directive premise and for the retired slug; nothing dangles, and the generated mirror is
      regenerated rather than hand-edited.
- [ ] **Both host entries move together** — the pairing is bidirectional and one file backs two
      refs, so neither entry's terminal move is legal while the other still cites this file.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
