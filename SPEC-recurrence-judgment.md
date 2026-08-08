# SPEC amendment: gap-resolver-mention-overcount

The recurrence count is the project's only aggregated signal that a finding keeps
coming back, and it is produced by a predicate that cannot represent the question
it is asked. `bin/file-gap.sh` resolves a filer's prose against the live slug set
by bounded substring match and, on a hit, rewrites the bullet it was already
writing into ``- <date> — recurrence of `<slug>`: <prose>`` — a **verdict**,
stamped from a **mention**. close's drain then re-resolves with the same
predicate. Two readers consume the resulting count: lifecycle-kit's scope
pre-emption threshold (`LIFECYCLE_KIT_RECURRENCE_THRESHOLD`) and drift-kit's
`kpi-incident-recurrence`.

**The evidence this amendment rests on, and the evidence it refuses.** This
entry carries a `recurrence:` declaration with two dates, and both were produced
by the resolver the entry indicts; whether they represent genuine recurrences is
undetermined from the tree, and they are **not** cited here as evidence the
design is right (TRAJECTORY.md, the honest limit recorded with the ruling). What
grounds the unit is a defect mechanism that is independently checkable, and a
live instance that breaks the circularity:

- **A known-ground-truth false positive, produced by a filer who had already
  ruled the opposite.** The last bullet in `.workflow/gap-inbox.md` records a
  TRAJECTORY-pruning gap. Its prose names `ruling-record-condition-staleness-probe`
  because the ruling behind it says the finding must be a *separate* entry
  cross-referencing that one. The resolver labelled the bullet a recurrence of
  exactly that slug; the filer rejected the label before the bullet landed and
  recorded the rejection in the bullet's own text. The false positive is known to
  be false independently of the mechanism that produced it.
- **It reproduces on demand.** Replaying that bullet's prose through the shipped
  tool against the live queue (sandboxed inbox, no tracked write) stamps
  ``recurrence of `ruling-record-condition-staleness-probe`:`` onto prose whose
  second sentence reads *"NOT a recurrence of
  `ruling-record-condition-staleness-probe`"*. The matcher overrides the filer's
  explicit denial **in the same bullet**.
- **Striking the marker by hand does not help, and that is what forces the
  design.** The drain re-resolves the prose itself, so the bullet will match
  again at close no matter what was struck at capture. The only thing that
  survives to the drain is prose addressed to a **reader**. A channel whose
  correction mechanism is prose is a channel whose authority is a judge.

**Nothing machine-reads the marker, checked rather than assumed.** An unsilenced
sweep for ``recurrence of `` across the whole tree returns the producer
(`lifecycle-kit/bin/file-gap.sh`), the grammar prose in
`lifecycle-kit/SPEC.md` §The committed gap inbox, the drain prose in
`lifecycle-kit/templates/stages/close.md`, the assertions of
`lifecycle-kit/gate-tests/file-gap-recurrence.test.sh`, and one queue entry's
narration. There is no parser. The marker is narration that reads as a verdict —
which is the worst of both, and why retiring it costs no reader.

**The design, stated before the deltas.** A recurrence is a claim about *what a
finding is*, not about *what a string contains*, and no syntactic tell separates
"this recurred" from "this is about that" — the queue entry says so, and the live
instance is a bullet that spells the denial out and still matches. So the
mechanism is not tightened; it is **demoted**. The capture-time matcher becomes a
prompt that asks the filer, the filer's answer goes in the prose, and close's
drain is the authoritative judge. This is the operator's ruling of 2026-08-08,
taken over a narrower debt-shaped sliver deliberately.

## What changes

**1. The `recurrence:` declaration is redefined as a recorded judgment.**
*(design-bearing)* `lifecycle-kit/SPEC.md` §The committed gap inbox gains the
rule, in the position its current sole-producer paragraph occupies:

> **A `recurrence:` date records a session's judgment that a finding
> re-occurred, made by reading the bullet's prose. A slug appearing in a bullet
> is an input to that judgment and never a verdict; no mechanism produces the
> declaration.**

**What is retired, precisely.** The property being given up is
**re-derivability**: today a stamp is reproducible by re-running a predicate over
the queue, so a reader can audit the count without trusting anyone. After this
change they cannot, and pretending otherwise would be the dishonest half. The
operator ruled the retirement in; the amendment owes the replacement.

**What replaces it is auditability by inspection.** The drain stamps the
declaration in the same commit that truncates the inbox — already its specified
behavior, and the tree's own precedent in every drain commit that has stamped one
— so the judgment and the prose it was made from sit in **one diff**. A reader
auditing a date reads that commit and sees the bullet the judge read. Re-running
a matcher is replaced by reading the grounds, which is the correct trade when the
thing being recorded is a judgment.

**What is *not* retired, stated because the two are easy to confuse.** The
capture affordance still writes **no queue file** — that is the index-contention
constraint the inbox exists to hold, load-bearing and untouched. What loosens is
only the claim about *which session* records the declaration, next delta.

**2. The sole-producer half is retired as attested-false, and its successor
question is named rather than answered.** *(design-bearing)* §The committed gap
inbox currently asserts that the drain is the sole producer and that *no
mid-iteration path writes the declaration at all*. That is false at HEAD, and the
oracle is history rather than argument: of the nine commits that have added a
`recurrence:` date to this queue, **three stamped one from outside the drain,
with no gap-inbox bullet in the same commit** — two from non-closing stage
sessions, one from a close session's unrelated triage step. The SPEC sentence is
replaced by what is true: the drain is the only **mechanized** producer, and it is
the stage that must not skip the judgment.

Whether a direct stamp by a session that observed a recurrence outside the
capture channel should be sanctioned, forbidden, or mechanized is **the sibling
deferred entry `recurrence-drain-input-widening`'s question**, and this amendment
deliberately does not settle it. That entry's slug stays in this amendment and
out of the kit SPEC: a kit sentence naming a consumer's queue entry would cross
the provenance seam, so the kit states the generic contract and the open question
lives here.

**This also kills a gate this amendment first drafted, and the negative result is
recorded so it is not re-drafted.** A `check-recurrence-provenance` requiring any
commit that adds a `recurrence:` date to co-stage the inbox looked like the
enforced replacement for the retired property. The back-test above refuses it: it
would red one in three of the tree's own precedent, and it would foreclose the
sibling entry's likely fix by gating the very path that entry may want opened.

**3. `bin/file-gap.sh` stops writing the verdict; the bullet grammar becomes
uniform.** *(design-bearing)* The tool appends `- <YYYY-MM-DD> — <prose>` for
every filing, matching or not. The recurrence-marker slot between the date and
the prose is removed from the grammar, from the tool's `# usage:` header, and
from the SPEC's grammar sentence. One bullet shape means the drain has no
structured field to be tempted by, and a reader of the inbox sees observations
rather than conclusions.

The **matcher itself stays**, because the entry's own diagnosis is that *the
corrective is already legal and what is missing is any prompt telling the filer
to check*. Deleting the awk would delete the only thing that could produce that
prompt. Its recall was never the defect; its authority was.

**4. The advisory asks instead of asserting, and names what the filer should
do.** *(design-bearing)* On a match, stderr today reads *"this finding is already
filed under `<slug>` — recorded as a recurrence"*: a conclusion, addressed to
nobody who can act on it. It becomes a question — the prose names live entry
`<slug>`; if this bullet **re-files** that finding say so in the prose, and if it
merely cites, corrects, or argues it is *distinct* from that entry, say that
instead, because close's drain judges and reads what was written. stdout stays
the stamped bullet, so the tool's output contract is unchanged.

**The honest limit, stated here rather than discovered later.** The prompt is
silent exactly where the sibling entry `recurrence-resolver-literal-match-only`
says it is: a bullet describing a recurrence without spelling the slug triggers
no advisory. That does not leave a hole in *this* channel, because the drain
reads every bullet regardless of whether the matcher spoke — which is the
difference between a prompt and a gate, and the reason the judgment had to move
rather than the predicate get sharper.

**5. The filer's claim lives in the prose; no new field and no flag.**
*(design-bearing)* Three alternatives were available and are refused with cause:

- **A required recurrence-or-new argument on `file-gap.sh`.** Refused: it turns a
  capture affordance into an interrogation, and §The committed gap inbox already
  rules that refusing capture does not dissolve a finding, it pushes it back into
  session context — the deferred-capture antipattern the inbox exists to prevent.
  It is also asked of the wrong party: a mid-build filer routinely has not read
  the queue, which is why the matcher existed at all.
- **An optional `--recurrence <slug>` flag.** Refused: a structured claim invites
  the drain to trust it, reinstating the same defect one layer up with a better
  provenance story. The live instance shows prose already carrying the claim
  correctly — in the negative direction, which is the harder one.
- **Narrowing the matcher** (anchor the slug, exempt a bullet containing a
  negation). Refused, and already ruled out by the operator: there is no
  syntactic tell, and a negation heuristic institutionalizes the evasion the
  entry indicts — an affordance that must be phrased around to stay accurate is
  miscalibrated, and the next filer does not know to phrase around it.

**6. close's drain step states the judge's obligation.**
*(design-bearing)* `lifecycle-kit/templates/stages/close.md` step 2 currently
tells the drain to resolve each bullet's prose against the live slug set itself
and calls the capture marker a convenience. With the marker gone the sentence
loses its subject, and what replaces it is the standard the drain is now held to:
a slug in a bullet is an input, the question is whether the finding **re-occurred
or is being cited**, the bullet's prose is the grounds, and the call is stated in
the close commit message beside the bullet's disposition — a **declined** match
stated as explicitly as a stamped one. The idempotence rule, the done-slug rule,
and the same-commit rule are unchanged; the same-commit rule is now load-bearing
rather than incidental, because delta 1 makes that commit the audit artifact.

**7. A new gate holds the capture surface: `check-gap-inbox-neutrality`.**
*(design-bearing)* `lifecycle-kit/checks/check-gap-inbox-neutrality.sh`, manifest
`# graph: couples=.workflow/gap-inbox.md dir=one valve=none tier=precommit`, on
the `check-survey-record` model (same kit, same knob shape, one optional path
argument overriding `LIFECYCLE_KIT_GAP_INBOX_FILE`). Two assertions:

- **A — grammar.** Every non-blank line after the `# contract:` header is
  `- <YYYY-MM-DD> — <prose>`. Nothing gates this today even though the raw append
  is an explicitly legal fallback, so a malformed bullet is currently found by the
  drain reading it, an iteration later.
- **B — no interposed verdict.** No bullet's prose opens with the retired
  ``recurrence of `<slug>`:`` form. This is the class assertion: it catches a
  conclusion reaching the capture surface **by any producer** — a stale vendored
  tool, a hand append copying an older bullet's shape — rather than only catching
  the tool this amendment fixes. The help line teaches the rule rather than the
  regex: the capture channel records observations, so say *why* you believe the
  bullet re-files an entry, in the prose, and let the drain judge.

An absent inbox is **clean, not fail-closed** — never having filed a gap is a
legal state for a fresh consumer, unlike `check-lifecycle-registration`'s missing
agent file, which is an install that did not finish. Fail-closed applies to the
tool failures inside the gate (`fail_closed` on the scanner's exit status), per
gate-sdk's contract. Fixture pair under `lifecycle-kit/gate-tests/`
`check-gap-inbox-neutrality/{good,bad}/`, plus registration in this repo's
`scripts/gates.list` and the kit's `smoke/install.sh` check roster.

**Why the tool-side contract is pinned separately.** The gate reads the surface;
it cannot see which producer wrote a line. `file-gap-recurrence.test.sh` is what
holds the *tool* to the contract, and delta 9 inverts it. The two together are
the enforcement: the fixture-runner catches the producer regressing, the gate
catches a verdict arriving by any other route.

**8. The two cross-kit consumers are told the count changed meaning.**
*(mechanical)* The ruling puts both inside this amendment's envelope, and both
need one clause rather than a redesign:

- **`lifecycle-kit/templates/stages/scope.md`'s pre-emption rule.** Its mechanics
  are untouched — the count is still the number of dates on the declaration, still
  one anchored grep, still scoped to deferred entries. What it gains is that those
  dates are **judged recurrences**, so a count reaching the threshold is a signal
  to act on rather than a number to sanity-check first.
- **`drift-kit/SPEC.md` §Bundled KPIs, `kpi-incident-recurrence`.** Its lag
  paragraph says a recurrence nobody files is uncounted; after this, one no
  session **judged** is uncounted too — the same structure, one step later in the
  chain. It also gains the comparability clause: the count is a judged count, so
  changing the judging rule breaks comparison across the change. That clause is
  kit-generic and carries **no date and no repo-local instance** — the seam again:
  when this project's own series became incomparable is a fact about this
  project, and it belongs in the queue entry's Done record and the release notes,
  not in a kit SPEC.

**The threshold's value is deliberately not changed, and the non-change is
stated.** `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` stays `2`. Removing false
positives lowers observed counts, which is an argument for lowering the
threshold — and there is not one post-fix observation to lower it against, so
re-tuning now would replace a measured default with a guessed one. The next
scope that meets a judged count is the first party with evidence.

**9. `queue-kit/SPEC.md` §The tag algebra stops calling the line
machine-written.** *(mechanical)* It says the declaration *"is **machine-written**
by the closing stage's gap-inbox drain and hand-read"*. After delta 1 it is
session-written under judgment and hand-read, and that sentence is exactly the one
a later session would cite to justify re-mechanizing it. It gains the corrected
producer and a pointer to lifecycle-kit's owning section; the grammar, the
self-naming-slug mechanism, and the two-readers sentence are unchanged, because
none of them depended on who writes the line.

**10. `file-gap-recurrence.test.sh` is inverted.** *(mechanical)* Its current
assertions demand the marker; they become the opposite, against the same fixture
queue: a live deferred slug produces the **plain** bullet *and* the advisory on
stderr, the longest-match and hyphen-boundary arms now assert which slug the
**advisory names** rather than which the bullet carries, done / Lessons / sub-task
slugs still produce silence, and the no-queue-write assertion stands untouched.
One arm is added: a bullet's prose that denies the recurrence in words still
produces no marker — the live instance, frozen as a fixture so the defect cannot
return unobserved. The file's `spec:` line is rewritten to the new contract.

**11. The projections the new gate stales are regenerated.** *(mechanical)* A new
gate is the widest fan-out this tree has: the pre-commit hook, the graph
artifact, the enforcement map, the footprint and value rollups, and the docs
mirror for all four edited SPECs. Each freshness gate prints its own regen
command on red; the roster and the ordering rule are
`docs/site-architecture.md` §Generated projections' and are followed, not
restated here.

## Producers and consumers

The interface this amendment changes is the **`recurrence:` declaration** and the
**gap-inbox bullet grammar** feeding it. One field is removed and none is added,
so the causal chain is stated as producer, consumer, and — for the removed field
— the readers that lose it.

- **Producer of the declaration (mechanized)** — close's drain step, in the
  commit that also truncates the inbox. Reachable at every close with no enabling
  config: the drain is mandatory and `bin/enter-stage.sh`'s boundary refusal
  forces it. Unchanged in position by this amendment; changed in **basis**, from
  a re-resolution to a judgment (deltas 1, 6).
- **Producer of the declaration (direct)** — a session stamping a recurrence it
  observed outside the capture channel. Attested at three commits, named rather
  than sanctioned; disposition deferred to the sibling entry (delta 2).
- **Producer of the bullet** — `bin/file-gap.sh`, and the raw append, which stays
  a legal fallback. Both now produce exactly one bullet shape (delta 3), and
  `check-gap-inbox-neutrality` is what holds **both** of them to it (delta 7) —
  the enabling config is the knob default, live wherever the kit is vendored.
- **Producer of the advisory** — the same tool, on stderr, at capture (delta 4).
- **Consumer 1 — close's drain**, reading every bullet's prose as the grounds for
  its judgment. This is the consumer that gains authority: previously it received
  a marker it was told not to depend on, and now it receives prose it must read.
- **Consumer 2 — the filer**, reading the advisory at the one moment the answer
  is cheap to give: while the finding is still in mind and the bullet is still
  being written. This is the *new* named reader, and the field it reads is the
  reason the matcher survives demotion.
- **Consumer 3 — scope's pre-emption rule** (`LIFECYCLE_KIT_RECURRENCE_THRESHOLD`),
  by one anchored grep over the deferred section at the iteration boundary.
  Mechanism unchanged, semantics restated (delta 8).
- **Consumer 4 — drift-kit's `kpi-incident-recurrence`**, summing the dates for
  the trend report. Mechanism unchanged, semantics and comparability restated
  (delta 8).
- **Consumer 5 — `check-gap-inbox-neutrality`**, reading the inbox at pre-commit
  (delta 7), and **`file-gap-recurrence.test.sh`**, reading the tool's stdout and
  stderr in the fixture-runner battery (delta 10).

**The removed field's readers, listed because a field with no reader is what
licenses removing it.** §The committed gap inbox names three readers for a
bullet's fields; the marker's are *"the filer at capture (via stderr) and the
drain as a convenience it does not depend on"*. Delta 4 keeps the filer's reader
and moves it to the channel it always was — stderr, which is where the filer
actually reads it — and delta 6 removes the drain's, which the SPEC already said
was not depended on. The tree-wide sweep above confirms no third reader exists.
The date and the prose keep their readers unchanged.

**Why the defect was silent.** Both ends of the channel ran the *same* predicate,
so the drain's re-resolution — the step designed to be the check on the capture
marker — agreed with it by construction. Two independent-looking confirmations
were one measurement, which is also why the entry's own two recurrence dates
cannot settle anything: the instrument was reading itself. Delta 1 breaks the
symmetry by changing what the second end *is*, rather than by giving it a second
predicate.

## Existing sections updated

- **`lifecycle-kit/SPEC.md` §The committed gap inbox** — owns the surface, the
  grammar, the affordance, and the producer/consumer statement. Gains the
  judgment rule and the audit-by-inspection replacement; loses the marker from
  the grammar sentence, the sole-producer sentence, the "marker is a capture-time
  convenience" paragraph, and the marker's row in the three-fields paragraph.
  *(Owns deltas 1, 2, 3, 4.)*
- **`lifecycle-kit/SPEC.md` §Per-component contracts** — gains
  `### check-gap-inbox-neutrality` in the roster's established shape.
  *(Owns delta 7.)*
- **`lifecycle-kit/templates/stages/close.md` step 2** — owns the drain
  procedure. Its marker sentence is replaced by the judge's obligation and the
  stated-call rule. *(Owns delta 6.)*
- **`lifecycle-kit/templates/stages/scope.md`'s pre-emption rule** — owns what
  the count buys at the boundary. Gains the judged-count clause.
  *(Owns delta 8.)*
- **`queue-kit/SPEC.md` §The tag algebra** — owns the declaration's grammar.
  Its machine-written sentence is corrected. *(Owns delta 9.)*
- **`drift-kit/SPEC.md` §Bundled KPIs** — owns `kpi-incident-recurrence`'s
  definition and fidelity tier. Gains the judged basis and the comparability
  clause. *(Owns delta 8.)*
- **`lifecycle-kit/bin/file-gap.sh`'s `# usage:` header and `spec:` lines** — the
  marker line goes, the advisory line arrives. *(Owns deltas 3, 4.)*

**Three components' contracts change** — lifecycle-kit, queue-kit, drift-kit —
so this amendment fires the audit stage's third trigger, and the next stage entry
will demand that stamp.

No section is listed that no delta claims. In particular **`bin/enter-stage.sh`'s
boundary refusal is not updated**: it reads the inbox's emptiness, not its
grammar, and stays correct across every delta here. Neither
`queue-kit/bin/queue-index.sh` nor drift-kit's KPI implementation changes — both
read dates off a line whose shape is untouched, which is the property that let
the semantics move without a migration.

**The sibling entries' stated grounds move, and the queue is not edited here.**
`recurrence-resolver-literal-match-only` names "the same contract call
`gap-resolver-mention-overcount` already names" as pending; after this it is
made. `recurrence-drain-input-widening` inherits delta 2's open question as its
own. Whether either entry is thereby resolved, narrowed, or stands unchanged is a
queue disposition rather than an amendment's call; it is escalated with this
stage's report, and build corrects whatever the ruling settles when the unit
lands — the same handling this iteration's second amendment gave its own entry's
falsified blocker.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The named target is the marker grammar: an
      unsilenced tree-wide sweep for ``recurrence of `` returns only prose that
      *describes* the retirement.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The gate is red before it is green** — `check-gap-inbox-neutrality`'s
      assertion B is run against a bullet carrying the retired marker and
      confirmed to red, before the fixture pair is called done. The defect class
      is currently unobservable in the tree because the last instance was struck
      by hand, so the `bad/` fixture is the only place it can be seen failing.
- [ ] **The live instance is frozen as a fixture** — delta 10's added arm uses
      the real bullet's shape (prose naming a slug and denying the recurrence in
      words), so the regression that motivated the unit is the thing the battery
      watches.
- [ ] **No verdict survives anywhere in the channel** — after the change, filing
      the live instance's prose through the tool produces a plain bullet and an
      advisory that asks. Run it; do not infer it from the diff.
- [ ] **The count's readers are consistent** — `scope.md`, `drift-kit/SPEC.md`,
      `queue-kit/SPEC.md`, and `lifecycle-kit/SPEC.md` describe the same basis
      for the same number, checked by reading the four sentences side by side.
      Four owners describing one derived value is exactly where this kind of
      change rots.
