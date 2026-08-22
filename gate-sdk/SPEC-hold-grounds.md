# SPEC amendment: held-member grounds

Pairs with `TASK-QUEUE.md` entry **cohort-held-members-port-prerequisites**.

## The two forks this amendment resolves

The entry states two dispositions for `check-tree-terms` and takes neither:
write its hold cause into gate-sdk/SPEC.md §check-tree-terms and keep the hold,
or retire the hold. Behind that fork sits a second one the entry names in the
same breath — `check-gate-assertions`' blocker is `paste -sd, -`, which is
`.join(",")` in the target language, and nothing says whether a blocker of that
size is a hold at all.

Both forks are one question asked twice: **what makes a member's hold real, and
where does the ground for it live?** This amendment rules the question rather
than the two instances, then applies the ruling to them.

### The evidence, probed rather than recalled

Every fact below was taken from a run or a read at authoring time.

- **Five members declare `# port-until:` today**, all naming this entry:
  `check-shellcheck` and `check-action-run-shell` (`shellcheck`),
  `check-gate-assertions` (`paste`), `check-docs-render-fidelity`
  (`ruby`, site-kit's), and `check-tree-terms`.
- **`check-tree-terms` has no external-program blocker at all.**
  `bash gate-sdk/bin/port-blockers.sh` does not list it; its rule invokes `git`,
  `grep` and `dirname`, every one of them on `GATE_SDK_PROGRAM_FLOOR`. Its
  declared ground is criterion 4, and criterion 4 alone.
- **That ground is stated in no SPEC section.** gate-sdk/SPEC.md
  §check-tree-terms says nothing about the port. The ground exists — in the
  queue entry, and only there.
- **`check-docs-render-fidelity` fails the same rule from the other side.** Its
  ground *is* written first-hand, but at site-kit/SPEC.md §Layout and
  configuration under `SITE_KIT_RENDERER`, not in §check-docs-render-fidelity,
  which mentions the port nowhere. A reader arriving from the declaration finds
  nothing.
- **Criterion 4 has never held a member.** `check-docs-cname-parity`,
  `check-gate-exemption-tasks`, `check-knob-default-coupling`,
  `check-spec-embedded-source` and `check-gate-tamper` each bind it and each
  ported, discharging it by widening the fixture pair.
- **Nothing machine-held reads the ground.** §check-gate-substrate-parity's
  assertion G holds the field's *shape* (non-empty slug, at most one, never on a
  descriptor, never beside `# no-port:`); §check-gate-exemption-tasks holds its
  slug's *liveness*. Neither reads a SPEC section, so the rule that a held
  member's own section states its cause has been discipline only.
- **The takeable tier is empty.** `bash gate-sdk/bin/port-blockers.sh --group`
  reports `5 still owed, 0 takeable at this cut` — every owed member is held, so
  `native-gate-port-remaining-corpus`'s budget arm has nothing to compose from.

### And one contradiction inside the SPEC itself

Two sections disagree about where a held member's ground lives, and the
disagreement is what licensed `check-tree-terms`' ground landing in a queue
entry:

- §The `# graph:` manifest, ruling the field's payload a bare slug: *"a
  temporary hold's home is a **queue entry**, which already carries the ground,
  the cost and the disposition"*.
- §The first cohort, and the rule that selects the next: *"`# port-until:` names
  the owning entry, **that entry carries the disposition, and the gate's own
  section carries the ground**. A member may not declare the field until its own
  section states its cause"*.

A session reading the first files the ground in the entry and is done; a session
reading the second reds on that. Both are live prose in one document. Delta 1
picks the second and deletes the claim in the first, keeping the *refusal* the
first paragraph exists for.

### Ruled: retire both holds; neither ground survives contact

**`check-tree-terms`' hold is retired.** Criterion 4 prices a port and orders
it; it has never made a member un-takeable, and the five ported members above
are the proof rather than the argument. The criterion's own text already says so
— *"A member failing this criterion still ports; what it owes is an oracle the
port cannot invalidate"* — and its discharge is an instruction (*widen first,
then port*), not a blocker. A field whose stated meaning is **not takeable now**
is the wrong instrument for a fixture widening.

The alternative — write the criterion-4 ground into §check-tree-terms and keep
the hold — is refused because it would make the SPEC say two incompatible things
about criterion 4: that it never excludes a member, and that it holds this one.
The sizing fact is real and it does move to §check-tree-terms (delta 5); what it
stops doing is standing as a hold.

**`check-gate-assertions`' hold is retired too**, on the ruling delta 3 makes:
a criterion-7 blocker holds only where the program *is* the rule's semantic
content. `paste -sd, -` joins a sorted label list with commas. It is not what
the gate asserts; it is how the gate spells a string. The port re-expresses it
and the dependency is gone, which is ordinary port work priced by the cut.

**The other three holds stand and are strengthened by the same ruling.**
`shellcheck` *is* what §check-shellcheck and §check-action-run-shell assert;
the renderer *is* the contract §check-docs-render-fidelity renders through.
Removing any of the three changes the verdict, so designing it away is a
sub-project. After this amendment the held tier is exactly *the members whose
blocking program is the rule's own semantic content* — three members, one
criterion, no residue.

**Neither retirement ports its member here.** The port is
`native-gate-port-remaining-corpus`'s deliverable and that entry is not promoted
this iteration; taking it here would be this amendment scoping in another
entry's work. What the retirements deliver is exactly what that entry says it is
waiting for: *"every member still owed now sits behind a prerequisite, so the
budget arm cannot select until `cohort-held-members-port-prerequisites`
moves"*. Two members return to the takeable tier and the budget arm can compose
a cut again.

## What changes

### (1) One home for a held member's ground

§The `# graph:` manifest stops claiming the queue entry carries a held member's
ground, and §The first cohort's split — **entry carries the disposition and the
cost, the gate's own SPEC section carries the ground** — becomes the single
statement of it. **{design-bearing}**

The refusal that paragraph exists for is **unchanged and re-grounded**: the
field's payload stays a bare slug, and a `<cause>` half is still refused. What
changes is the reason. It was *"a second place the same ground lives"* with the
queue entry as the first; it is now a second place the same ground lives with
**the gate's own SPEC section** as the first. The refusal survives the
correction because its shape never depended on which surface was the ground's
home, only on there being exactly one.

The asymmetry with `# no-port:` is restated where it now reads differently:
`# no-port:` carries free text because a permanence ruling's only home is prose
and the field *is* the pointer to it; `# port-until:` carries a slug because a
hold has two facts in two homes — the ground (the gate's section) and the
disposition (the entry) — and the slug reaches the one a *reader of the queue*
needs. Delta 4 supplies what the other hop was missing: an oracle.

### (2) A criterion-4 bind prices a port; it never holds one

Criterion 4 gains an explicit statement that it is never a hold ground, and the
five members that bind it and ported are named as the evidence.
**{design-bearing}**

The criterion already says a failing member still ports and already names its
discharge — an oracle the port cannot invalidate, which is the fixture pair
carrying every arm of the derivation being ported. What it never said is the
consequence for the *held tier*: a member whose only unmet criterion is 4 is
**takeable**, carrying a widening in its price. Saying it closes the reading
that produced `check-tree-terms`' declaration, where a real and correctly
identified criterion-4 bind was spelled as a hold.

The ordering constraint criterion 4 shares a paragraph with — *the shared
snapshot*, which protects every comparison from a sibling's port — is untouched
and explicitly so: §The port-candidate criteria already rules the two
independent facts, and this delta narrows neither. A cohort still sequences a
criterion-4 member deliberately; it just does not declare it held.

### (3) A criterion-7 blocker holds only where the program is the rule's semantic content

Criterion 7 gains the hold-worthiness test that decides whether a derived
blocker is a hold or a line item in the port's price. **{design-bearing}**

Two classes, and the test between them is **whether removing the program changes
the gate's verdict**:

- **The program is the rule.** `shellcheck` decides what
  §check-action-run-shell and §check-shellcheck assert; `cargo` decides what
  §check-crate-arms asserts; the renderer decides what §check-docs-render-fidelity
  asserts. Designing the dependency away means re-deciding the rule, which is a
  sub-project with its own design, and the member is genuinely **not takeable
  now**. This is the class `# port-until:` was minted for.
- **The program is incidental spelling.** A text utility the rule uses to
  assemble, split or order a string the port re-expresses in the target
  language: `paste -sd, -` is `.join(",")`, and the gate's verdict is identical
  either side of the substitution. The blocker is **priced by the cut, never a
  hold** — it is one line of the port's own work, exactly like a criterion-4
  widening.

The criterion's opening claim is unaffected: a blocker is still never an
eligibility screen and every member still ports. What the test decides is the
narrower question the declarable spelling made answerable — *is this member
takeable at this cut?* — which the criterion has been silent on since the field
landed.

**The tool cannot make this call and is not asked to.** `port-blockers.sh`
reports *which program a rule invokes*; the class is a judgment about what the
rule asserts, which no tokenizer sees. Declaring it is therefore presence-shaped,
on the terms assertion G already fixed for both fields: presence is the verdict,
absence means takeable, and an **undeclared** member is over-counted as takeable
rather than lost. The failure direction is unchanged by this delta — a
misclassified class-(i) member reads takeable, is selected, and the cut
discovers the blocker at composition time, which is where the report already
puts it.

**Two refusals, recorded so they are not re-proposed as ergonomics.** *The
program is not on the floor* is not the test — it is the report's test, and
reading it as the hold's is what put two ungrounded declarations on this tree.
And *the port is more work than an ordinary member* is not the test either;
every criterion prices work, and pricing is not holding.

### (4) assertion H — a held declaration's ground is reachable in one hop

§check-gate-substrate-parity gains an eighth assertion: for every declaration
carrying `# port-until:`, the SPEC section its own `# spec:` header field points
at names the field. **{design-bearing}**

**Resolution is the declaration's own pointer, not a derivation.** Every gate
declaration carries a `# spec: <path> §<section>` header field, and
canon-kit/SPEC.md §check-spec-pointer already holds that pointer to a tracked
file and an existing heading. Assertion H therefore resolves *nothing*: it opens
the section that field names and reads it. That is the literal shape of the
property the rule states — a reader reaches the ground from the declaration in
one hop — and it costs no second heading-matching implementation. It also
handles the two heading levels in play without a special case (`### <gate>` in
gate-sdk/SPEC.md, `## <gate>` in site-kit/SPEC.md), because the pointer names
the heading text and the extraction runs to the next heading at the same or
shallower level.

**Red conditions, enumerated:**

- A declaration carries `# port-until:` and **no** `# spec:` header field — the
  ground is unreachable from the declaration, whatever else is true. Red, naming
  the missing field. This is not `check-spec-pointer`'s job: that gate holds a
  present pointer's *resolution* and asserts no pointer's presence.
- A declaration carries `# port-until:` and a `# spec:` header field whose named
  section body contains no `port-until` token — the hop lands somewhere that
  does not discuss the hold. Red, naming the section.
- The `# spec:` field's target file is unreadable at the resolved path. Red,
  fail-closed, on §Fail-closed contract's ordinary terms: a gate that cannot
  read its corpus is not clean.

**Not asserted, deliberately:** that a member *without* the field has no hold.
That is assertion G's presence question, refused there for reasons this delta
does not disturb — a gate deriving which members hold one would have to read
argument prose, and the error direction of not asserting it is the status-quo
over-count.

**The corpus and the coupling.** The subject set is the declarations assertion A
already resolves; the second corpus is the SPEC files their pointers name. The
manifest's `couples=` gains `kit:SPEC.md`, so an edit to any vendored kit's SPEC
re-runs the gate — correct coupling, since deleting a paragraph is exactly how
this assertion goes from green to red. The existing `gate-sdk/SPEC.md` literal
is **kept rather than folded into the new token**: it is assertion C's
conservation-doc coupling, and a consumer whose `GATE_SDK_KIT_DIRS` does not
name gate-sdk would otherwise lose it silently. Two couples for two assertions,
stated so the redundancy does not read as an oversight.

**Why it folds into this gate rather than shipping as its own.** Assertion C is
the precedent and it is exact: C asserts that a *SPEC document records a
disposition* for every substrate-sensitive member, which is the same
prose-placement claim over the same declaration set, in the same gate, read from
the same two-positional usage (`[gates-dir] [conservation-doc]`). The refusal
recorded at §check-gate-exemption-tasks — that slug liveness would give this
gate a **queue-file** coupling it deliberately has none of — does not reach a
SPEC coupling it already has. And the gate is permanently shell under exception
class (a), so widening it raises no substrate question and adds no member to the
conservation table, exactly as assertion G's own widening recorded.

**Its honest limit, stated rather than discovered.** The assertion holds
*reachability*, not truth: a section could name the field and say nothing
useful, and no gate can rule whether a ground is a good one. §When a gate earns
its place bars a trivially-true proxy, and the measurement that clears this one
is that **two of the five live declarations fail it today** — it is a real drift
axis with a real corpus, not a heading-presence check. What stays human is the
irreducibly semantic judgment alone.

**Fixture cases.** The `good/`+`bad/` pair gains, without disturbing the
assertions it already proves in the same invocation: a declaration with
`# port-until:` whose pointed-at section names the field (clean); the same
declaration whose section does not (red); and a `# port-until:` declaration with
no `# spec:` header field at all (red). The SPEC surface lives inside the case
tree and the pointer is case-relative, so the pair proves the resolution rather
than the live tree's accident of already being green.

**The contract's own count moves with it.** §check-gate-substrate-parity opens
*"Seven assertions"* with a labelled span `(A)`…`(G)`; both become eight and
`(A)`…`(H)`, and the source gains its `# assertion H:` marker.
`check-gate-assertions` is the reader that holds the three in agreement — the
same member whose hold delta 3 retires, which is coincidence rather than
coupling and is noted only so it does not read as one.

### (5) The two retired members' sections record what their ports cost

gate-sdk/SPEC.md §check-tree-terms and §check-gate-assertions each gain the
paragraph that states its port price and records that its hold is retired and
why. **{design-bearing}**

§check-tree-terms gains the criterion-4 fact the queue entry has been holding:
its corpus is `git ls-files` over the whole tracked tree, pruned only by the
shared prune dirs and the `msg-patterns` basenames, so **every** registry
member's declaration path lies inside the corpus it scans as content —
criterion 4's predicate verbatim, and reached through the walk rather than
through the trigger field, which is `couples=scripts/msg-patterns.list` and
selects nothing. What that buys the port is named: a fixture pair carrying every
arm of the derivation, widened before the port rather than after. And the
verdict: **priced, not held** — the declaration is retired and the member is
takeable.

§check-gate-assertions' existing hold paragraph is rewritten rather than
deleted, because *how the requirement surfaced* is worth keeping: the roster
reported this member clean for its whole life because the scan abandoned the
declaration before reaching the call. What changes is the verdict on it. `paste`
is off the floor and the report is right to say so; it is class (ii) under delta
3 — a join the compiled rule spells directly — so the member is takeable and the
declaration is retired. The GNU-awk requirement in the same section rides the
same reasoning and is named, so a later reader does not restore the hold on it.

Both sections keep the criterion citation. A member the port has not reached
still owes the work its criteria name, and a section that stopped naming them
would lose the sizing this delta is moving into it.

### (6) `check-docs-render-fidelity`'s section gains its ground

site-kit/SPEC.md §check-docs-render-fidelity gains the hold paragraph the three
gate-sdk holders already carry, pointing at the knob bullet that owns the
ground. **{design-bearing}**

This member's ground is not misplaced by accident and is **not moved**: the
dependency is a property of `SITE_KIT_RENDERER`'s value, so §Layout and
configuration is its right home, and a consumer who repoints the knob changes
the blocker. What is missing is the hop. The section a reader reaches from the
declaration says nothing about the port at all, which is the same failure
`check-tree-terms` has by a different route — one ground in the wrong document,
one ground in the right document under the wrong heading.

The fix is content-tiering's own answer: the gate's section states that the
member is held, that the blocker is the first element of `SITE_KIT_RENDERER`,
and that the dependency and its consumer-dependence are owned at §Layout and
configuration — a pointer, never a restatement of the knob bullet's argument.

**This is what makes the amendment cross-component, and it is taken
deliberately.** Calibrating assertion H to pass this member would mean asserting
something weaker than the rule — *the ground is somewhere in the kit's SPEC* —
which is green against `check-tree-terms` too and catches nothing. An oracle
tuned to pass the member it was built to catch is not an oracle. The audit stage
is owed at the next entry and this amendment names it rather than letting
`check-stage-entry` assertion C surface it as a surprise.

### (7) The declarations retire and every derived surface is re-read

The two `# port-until:` header lines are deleted, and every reader of the
declaration set is re-run rather than reasoned about. **{mechanical}**

`gate-sdk/checks/check-tree-terms.sh:4` and
`gate-sdk/checks/check-gate-assertions.sh:4` lose their `# port-until:` lines.
The queue entry's roster prose drops both members and cites the rulings above
rather than restating them. `bash gate-sdk/bin/port-blockers.sh --group` is run
and its trailer recorded at §The first cohort as the post-amendment reading of
the takeable tier — a dated oracle read, never a number this amendment holds.
The full battery plus the gate-sdk fixture suite is the verification.

## Producers and consumers

**New interface: assertion H (delta 4).** No new state, no new field, no new
message — the amendment adds one assertion over two corpora that already exist.

- **Producer** — the assertion H loop inside
  `gate-sdk/checks/check-gate-substrate-parity.sh`, over the declaration set
  assertion A resolves. Its enabling configuration is the gate's registration in
  `scripts/gates.list`, which is live today: the member runs on every commit at
  `tier=precommit` through the generated hook, and in the full battery. Nothing
  new must be set anywhere for it to fire.
- **Re-run trigger** — the `# graph:` manifest's `couples=`, widened with
  `kit:SPEC.md`. `gen-pre-commit` reads that manifest to emit the hook, and
  `check-graph` holds the emitted hook against the manifest, so the trigger's
  own consumer is machine-held. Without the widening the assertion would be
  produced only when a *declaration* changed, and its most likely red — a
  deleted SPEC paragraph — would arrive a tier late.
- **Consumer** — the committing session and CI, through the gate's exit code and
  its findings text; and the clean line's counters, whose reader is the session
  auditing the held tier at a cohort cut.
- **Named reader for the one new datum on the clean line** — the count of held
  declarations whose ground was verified. Its reader is the cohort-cut session
  choosing the next cut, at exactly the transition §The first cohort's selection
  rule describes: a zero there in a tree that declares holds is the vacuous-pass
  tell, which is why the count is emitted rather than the verdict alone. No
  field is added that this reader does not read.

**Existing integration prose (delta 1) has a producer too**, and it is the
reason that delta exists: the two disagreeing paragraphs are both read by the
same session at the same moment — an author writing a hold. Correcting one and
leaving the other is how the contradiction survived its first landing.

**Corpus narrowing, and its red conditions (delta 7).** Deleting two
`# port-until:` lines *narrows* the declaration set every reader of that field
walks, so each reader's red condition is enumerated rather than cleared by
inspection:

- **`check-gate-substrate-parity` assertion G** reds on a bare payload, a second
  declaration of the same field, either field on a descriptor, or both fields on
  one declaration — every one of them a property of a *present* declaration.
  Monotone in the violation set; removing two declarations removes potential
  violations and adds none. Its clean line reports a **count**, not a floor, and
  no arm asserts that count is non-zero.
- **New assertion H** is monotone by the same argument and **must not acquire a
  non-zero floor**, stated here because the vacuity instinct would add one: a
  tree with no held member has nothing to ground, and a floor would red every
  consumer that never declared a hold. The anti-vacuity signal is the emitted
  count, not a refusal.
- **`check-gate-exemption-tasks`** reds on a `# port-until:` slug that is
  Done-only or missing. Removing a declaration removes a subject. Its own
  non-zero claim is about the **skipped** set in a vendoring tree, not about the
  in-scope set, and it stays zero in this authoring tree either way.
- **`port-blockers.sh --group`** is a report, exit 0, no verdict to flip. Its
  trailer *grows* the takeable field and shrinks the held one, which is the
  intended effect and is recorded by running it (delta 7), never predicted.
- **The two edited declaration files** are read by `check-comment-tier`
  (a deleted header line removes a subject), `check-spec-pointer` (the surviving
  `# spec:` fields are untouched) and `check-graph` (the `# graph:` lines are
  untouched). None holds a count or a floor over these files.

**And the widening's own direction (delta 4).** `couples=` gaining
`kit:SPEC.md` *widens* a corpus, so point 5 does not bind on it; what it changes
is that `check-gate-substrate-parity` now fires on any kit SPEC edit. That cost
is named rather than absorbed: the member is shell, precommit, and already walks
the declaration set and every kit root, so the added work is one section read per
held declaration — bounded by the held tier, which this amendment leaves at
three.

## Existing sections updated

- gate-sdk/SPEC.md §The `# graph:` manifest — the held field's payload
  paragraph loses its claim that the queue entry carries the ground, and states
  the entry/section split; the `<cause>`-half refusal is re-grounded, not
  removed (delta 1).
- gate-sdk/SPEC.md §The first cohort, and the rule that selects the next — its
  ground/disposition split becomes the single statement rather than one of two,
  and it records the post-amendment takeable-tier reading (deltas 1 and 7).
- gate-sdk/SPEC.md §The port-candidate criteria, criterion 4 — gains the
  never-a-hold statement and the five ported members that evidence it (delta 2).
- gate-sdk/SPEC.md §The port-candidate criteria, criterion 7 — gains the
  hold-worthiness test, its two classes, the two recorded refusals, and the
  restatement that the tool reports invocations rather than classes; its
  `check-gate-assertions` worked-example paragraph keeps *how* the requirement
  surfaced and loses the implication that it holds the member (deltas 3 and 5).
- gate-sdk/SPEC.md §check-gate-substrate-parity — the contract's count-word and
  label span move from seven/`(A)`…`(G)` to eight/`(A)`…`(H)`; assertion H is
  specified beside assertion G; the `couples=` widening and its two-couples
  rationale are recorded; the fixture-coverage paragraph gains the three new
  cases (delta 4).
- gate-sdk/SPEC.md §check-tree-terms — gains its criterion-4 price and the
  retirement verdict (delta 5).
- gate-sdk/SPEC.md §check-gate-assertions — its hold paragraph becomes a
  port-price paragraph, keeping the surfacing history and reversing the verdict
  (delta 5).
- site-kit/SPEC.md §check-docs-render-fidelity — gains the hold paragraph
  pointing at §Layout and configuration's `SITE_KIT_RENDERER` bullet (delta 6).
- site-kit/SPEC.md §Layout and configuration, `SITE_KIT_RENDERER` — unchanged in
  content; the bullet is the ground's home and gains only the inbound pointer's
  counterpart if the merge finds the two read as duplicates (delta 6).
- `scripts/git-hooks/pre-commit` — regenerated from the `# graph:` manifest
  delta 4 widens, with `check-graph` the freshness oracle and
  docs/site-architecture.md §Generated projections the regen command's owner.
- `TASK-QUEUE.md` — **cohort-held-members-port-prerequisites** promotes with
  this amendment's ref and its roster prose drops the two retired members
  (delta 7).

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
