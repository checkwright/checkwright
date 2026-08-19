# SPEC amendment: sizing-input

`port-blockers --group` gains a **`lines=` column**, so the per-member cost the
budget arm's selection rule tells a sizing session to weigh is one the tool
actually prints. Today that sentence is **false**: §The first cohort, and the
rule that selects the next names "the per-member cost `--group` already prints
beside each member (shell line count and the mechanically derivable criterion
columns)", and the tool's single member-row print carries `c2=`, `c3=`, `c7=`
and an expanded `couples=` — no count, and none was ever emitted.

**The shape was left open to this stage by the operator** (2026-08-19, rider 2
of `takeable-tier-batch-and-installer-noop`). Two arms were on the record: emit
the count, or delete the clause and state what the sizing session actually
holds. This amendment takes the **first**, and §The envelope records the ground
so the choice is not re-opened at build.

## The envelope

**Asserted.** One new printed field on one advisory `bin/` tool, its default
arm untouched, plus the correction of the false clause and one behavioural
smoke assertion. The field is **exact** — a line count of a file the tool has
already resolved — and it is emitted on **every** still-shell row the arm
prints, keyed and unkeyed alike.

**Not asserted.** The field is **not a ranking** and the amendment does not make
it one. No ordering, no threshold, no derived "cheapness" verdict; §The
non-gate arm's own reading — that a size ranking is silent about interface
removals — is preserved and sharpened rather than overridden by a printed
number.

**Out of envelope — escalate rather than absorb.** Emitting any *judgment*
column (criteria 4, 5 or 6), which §port-blockers considered and **removed**
under its own field rule; adding a knob to the tool, which §port-blockers
records as introducing none; changing the default (criterion-7) arm's output;
and any change to the grouping key's two factors, which this field is
deliberately not one of.

## What changes

### 1. `port-blockers --group` prints `lines=<n>` on every still-shell member row

The count is `wc -l` over the resolved declaration path — the same `$decl` the
row's `c2=`, `c3=` and `couples=` are already read from, so the field cannot
disagree with the rest of its own row about which file it describes.
*mechanical.*

**It sits in the fixed-width run, after the member name and before `c2=`**, and
the placement is mechanical rather than aesthetic: `c7=` is variable-width — it
prints `clean`, `?`, or a comma-joined program list — so a field appended after
it is the one column in the row that cannot be aligned, and a cost column a
session scans down a list is exactly the one that must be. *mechanical.*

### 2. The field is emitted on the **unkeyed** row too, which is a ruling and not an oversight

The unkeyed row (`gate-sdk/bin/port-blockers.sh`, the `group_unkeyed` branch)
prints `?  <member><TAB><decl>` and deliberately carries **no** criterion
columns: a member empty in both key factors is reported rather than grouped,
and the arm declines to decide anything about it. A line count decides nothing
— it is a property of the file, not a verdict about it — so the reason the
criterion columns stay off that row does not reach this one. *design-bearing.*

**And the rows a session must adjudicate by hand are precisely where the clause
went false.** An unkeyed member is still owed and still a candidate; leaving it
the one row with no cost printed would reproduce the defect at a smaller scale.
The count is 0 unkeyed members in this tree today, which is why this has to be
ruled at authoring rather than discovered: the branch is live code with no
current instance, so a port or a rewrite would carry the omission forward
unnoticed. *design-bearing.*

### 3. §port-blockers' field rule is satisfied, and for a different reason than criteria 4/5/6 failed it

That section states the rule this amendment must clear — *no field is emitted
without a named reader at a named transition* — and records that the criterion
4, 5 and 6 columns were **considered and removed** under it, "since their only
honest reader would have had to disregard them". The distinction the new field
turns on: those three would have been **guesses** (a self-referential parity
oracle, an aggregate binary-less residual, whether a duplication is
machine-held), and a reader who cannot trust a column must disregard it. A line
count is **exact**. Its reader is the session composing a **budget-arm** cut and
its transition is the cut itself — the same reader and transition the arm's
existing columns already serve. *design-bearing.*

**The honest limit on the in-tree reader, stated rather than left for a later
session to discover.** After the sixth budget batch the takeable tier holds
`check-graph` alone, which is ruled out of every budget batch (§The fifth budget
batch), so the arm has no cut to compose here until a `# port-until:` hold
releases — the work `cohort-held-members-port-prerequisites` owns. The field's
first *in-tree* reader is therefore a hold release, and its nearer readers are
**consumers**: a vendoring adopter with a still-shell battery composes budget
cuts on their own corpus, which is what makes this kit mechanism rather than
this tree's instrument. Recording the gap is what keeps the field rule from
being bent to fit; a field whose reader is real but not imminent is not the same
thing as a field whose reader would disregard it. *design-bearing.*

### 4. The field carries its own honest limit, because emitting it without one manufactures a new defect

**A declaration line count is a floor on a port's size and never a ranking of
it**, and the evidence is two consecutive cuts rather than a prediction. §The
fifth budget batch records that its cost "was in *interfaces*, not in logic" —
the ranking it composed from was a ranking of logic and could not see six
command-line arguments — and that same section names
`check-template-copy-parity` as a member whose "cost is concentrated where the
line count hides it". That member is the **cheapest of the sixth cut by line
count** and carries that cut's concentrated cost, so the second cut attests what
the first one found. *design-bearing.*

**So the clause the field makes true has to say what the number is for.** A
printed count that a governed surface calls "the per-member cost" invites
exactly the reading both cuts refuted — that the sizing session may rank on it
— and a session that reads a column as an answer stops looking for the cost the
column cannot see. The corrected clause names the count as one input **beside**
the criterion columns and the session's own reading of each member's
declaration, which is what the sizing session has in fact been doing while the
surface told it otherwise. *design-bearing.*

### 5. The clause is corrected where it lives, not restated

§The first cohort, and the rule that selects the next owns the sentence, and it
is the only surface that promises the number; no second copy of the claim exists
to chase. `git log -S'lines=' -- gate-sdk/bin/port-blockers.sh` returns nothing,
so this is a clause that was false on the day it landed rather than a column
that was emitted and later dropped — and the correction is therefore an
addition to the tool plus a rewrite of the sentence, never a revert.
*mechanical.*

### 6. Behavioural smoke coverage, on the `bin/`-tool contract's own terms

`port-blockers` is a `bin/` tool: §The `bin/`-tool contract rules such a tool
earns **behavioural coverage in `smoke/`** rather than a fixture pair, and
§port-blockers already exercises the arm that way — `gate-sdk/smoke/install.sh`
builds a hermetic scratch registry with one planted declaration and drives both
arms over it. *mechanical.*

**The assertion is an exact count, not a shape match, and the existing fixture
is what makes that available.** The planted declaration is a heredoc of known
length, so the smoke asserts `lines=<its exact line count>` on that member's
row. A `lines=[0-9]+` shape match would pass a field that counted the wrong file
— the failure mode most worth catching, since the whole value of the column is
that it describes the member beside it. The assertion is written against the
heredoc's own length rather than a transcribed number, so widening the planted
declaration moves both together. *design-bearing.*

### 7. The generated projections this stales

None beyond the ordinary: `port-blockers` is a `bin/` tool carrying no `# graph:`
manifest and no fixture pair, and §port-blockers rules that **no freshness gate
accompanies it** — a gate would have to hold the derivation against a stored
expectation, which is the maintained roster re-entering by the back door. The
SPEC edit is an ordinary governed-prose change and stales the `docs/` SPEC
mirror like any other; the fan-out and its regen command are rostered at
docs/site-architecture.md §Generated projections. *mechanical.*

## Producers and consumers

One new **field**, and it is the only new thing here — no new state, no new
event, no new interface, no knob, no argument, no exit code.

- **Producer** — the `--group` arm's member loop in
  `gate-sdk/bin/port-blockers.sh`, at the point where it already has `$decl`
  resolved and is composing the row. Its enabling configuration is the one the
  arm already resolves (`gate_sdk_gates_dir` for the registry,
  `gate_kit_roots` for the resolve dirs), so the field introduces **no new
  default to be unset anywhere** — the property §port-blockers states for the
  arm as a whole and which this change must not break.
- **Consumer** — a **human session** composing a port cut under §The first
  cohort, and the rule that selects the next, at the transition that section
  names. This is the arm's existing consumer at its existing transition; the
  field adds a column to a report that consumer already reads, and adds no
  reader.
- **Named reader for the field, at a named transition** — the budget-arm sizing
  session, at the cut. §The first cohort's corrected clause is where that
  reader is *told* the column exists, which is the coupling the false clause
  broke: the field and the sentence that sends a reader to it land together or
  the defect changes direction rather than closing.
- **Second reader, mechanical** — `gate-sdk/smoke/install.sh`'s `--group` leg,
  at every `run-consumer-smoke` invocation and every `validate` stage that
  re-runs the suite. It reds when the field is absent or carries a count that is
  not the planted declaration's own.

**Nothing machine-parses the report**, which §port-blockers states as a property
of both arms, so the new column breaks no parser: the two arms' output is read
by the session choosing a cohort and, now, by one smoke assertion written
against this change. The arm's output **moving** is likewise already ruled — that
section records that stability was a fact about one change and "not a standing
guarantee that the arm's output never moves", with the truncation repair as the
precedent for moving it deliberately.

**The narrowing question does not arise** (canon-kit/SPEC.md §The
causal-completeness check, point 5): this change narrows no corpus. It adds a
field to a report over the same member set the arm already walks, so no reader's
violation set can grow by a file leaving a walk. The point is answered rather
than skipped, and the answer is that its precondition is absent.

**The seam holds, and it holds for a reason worth stating.** §The first cohort
records that the budget arm "names no remainder, which is what keeps it kit
content" — no gate names, no member roster, no count of any tree's corpus. A
`lines=` column is the same kind of thing: it is **derived from the consumer's
own declaration at run time** and carries no vocabulary, no roster and no
literal. The one shape that would cross the seam — a shipped table of member
sizes, or a threshold naming what counts as cheap — is exactly what §The
envelope refuses.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  the false clause is corrected: the count becomes a real printed input, and the
  sentence gains what the count is *for* (a floor, never a ranking) with the two
  cuts that attest it. Owned by deltas 1, 4 and 5.
- **gate-sdk/SPEC.md §port-blockers** — the `--group` arm's emitted-column
  roster gains `lines=`, with its named reader and transition stated in the same
  place the three removed columns' refusal is stated, so a later reader finds
  the admission and the refusals together and can tell which rule separated
  them. The unkeyed row's contract gains the field and delta 2's ground. Owned
  by deltas 1, 2 and 3.
- **gate-sdk/SPEC.md §The fifth budget batch** — no edit. Its
  interfaces-not-logic finding and its `check-template-copy-parity` sizing note
  are **cited** by delta 4 rather than restated; naming it here so the update
  target is claimed and a build session does not adopt it as an orphan.
- **`gate-sdk/smoke/install.sh`** — the `--group` leg gains the exact-count
  assertion. Not a spec section, listed because delta 6 owns it and an update
  target no delta claims reaches build as an orphan.
- **`TASK-QUEUE.md`'s `port-budget-sizing-input-absent` entry** — promoted to a
  feature section under this amendment's ref; its terminal move is an ordinary
  **Done** move, the deliverable being this one correction rather than a corpus.
  Owned by delta 5.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
