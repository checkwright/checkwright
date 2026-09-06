# SPEC amendment: lead-grant

**`lifecycle-kit/templates/lead.md` §Opening an iteration gains the two facts it has never
carried — the *channel* authorization arrives through, and the *cardinality* a grant carries —
and §Closing an iteration is re-grounded so the one-open bound sits on the **grant** rather
than on the boundary that meets it.** The consumer half, naming which channel this tree binds,
lands in `.claude/commands/lead.md`.

**This amendment promises prose and never a gate, and that is a finding rather than a
budget.** `lifecycle-kit/SPEC.md` §Honest limit on the lead's open authorization already
records the authorization fact as **not encodable** — an authorization is a fact about a
conversation, and every encoding of it is written by the same session it binds, so an encoding
raises the cost of proceeding unauthorized without making an unauthorized open detectable
afterward. Two mechanisms were weighed on record and both were ruled out. Nothing in this
amendment changes that, and delta 5 exists so a session meeting the corrected §Opening does not
go looking for enforcement that cannot exist.

**What is settled before this amendment and is not reopened here:** that the invocation is the
grant, that one invocation grants exactly one iteration, and that the correction is owed as two
edits on two tiers. Those are the ruling record's, with their authority, date and channel; this
file carries the mechanism and no stamp, because a kit template is on the far side of the
provenance seam.

## What changes

### (1) §Opening an iteration states the channel, and the consumer binds it

Today the section opens *"obtained explicitly and separately before anything else in this
section applies"* and then names three things authorization is **not** inferred from — and it
names no channel it *is* obtained through {design-bearing}. A lead reading it learns that
something must have happened and nothing about what would count, so the only reading available
to it is that whatever already happened was not it.

**The correction splits *explicit* from *separate*, which is the whole of the defect.**
Authorization stays explicit — that half is load-bearing and untouched. What was never true in
general is *separate*: a channel may be explicit without being a second act, and where the
consumer binds one that is, the lead holds its authorization already and asks for nothing
further. So the section states:

- Authorization arrives through **one channel, named by the consumer's binding** — not
  inferred by the lead from anything else, and the three existing not-inferred-from clauses
  stay exactly as they are, because they bound the *inference* and are untouched by naming a
  channel.
- Where the consumer binds no channel, the section's default holds: the lead asks, and the
  answer is the grant. A kit that shipped a channel would be shipping one harness's layout.

**The scale ground stays and is the reason the channel is bound rather than assumed.** The
section's existing paragraph — that an open commits a boundary reset, a walk of the whole stage
set, and a session per stage, and that how much machinery to spend is the operator's to choose
— is what makes *any* channel's answer worth obtaining. It is not edited.

### (2) A grant carries a cardinality, and the default is one

The section gains the second missing fact {design-bearing}: **a grant authorizes a stated
number of opens, and absent a stated number it authorizes one.** The bound is a property of the
grant, so it is known at the moment the grant is read rather than at the far end of the
iteration it authorized.

**Why the default is one rather than unbounded**, stated because a default that is merely
asserted is a default the next reader re-argues: the scale ground in delta 1 is per-iteration —
each open commits its own reset, its own stage walk, its own session per stage — so an
unbounded default would let one answer about one iteration's worth of machinery authorize an
unbounded amount of it. A grant may state a larger number; what it may not do is leave the
number to the reader.

**The cardinality is *spent*, not *renewed*.** An iteration that opens consumes one of the
grant's opens, and a grant with none left authorizes nothing. That is the sentence §Closing
needs and does not have.

### (3) §Closing an iteration is re-grounded on the grant, keeping the coverage its current shape bought

§Closing today rules that the lead stops and reports rather than opening the next one, and
describes itself as *"one rule attached to the iteration boundary, rather than two rules
attached to the two ends, so that repairing one end can never leave the other standing"*
{design-bearing}. The rule is right and stays; its **self-description is now false**, because
the bound it describes has moved to the grant.

**The re-grounding is deliberately conservative, and the reason is that the paragraph below it
is load-bearing.** §Closing's *the boundary is exactly where the design seats a session most
able to roll forward* paragraph is not about where the bound lives — it is about why the
boundary is the dangerous moment, naming the session-role marker and the lead's own resume
journal as the scratch artifacts that survive the boundary reset, and observing that under the
unified posture the session at the boundary *is* the previous scope session. All of that
survives untouched. Deleting it to relocate a bound would throw away the analysis that makes
the rule obeyable.

So §Closing states the rule as the **consequence** it now is: the grant that authorized this
iteration is spent, so the next open takes a fresh one, and the lead therefore stops and
reports. One rule, attached to the grant, **met** at the boundary — which preserves the
both-postures coverage the current wording achieves by a different route, since a spent grant
is spent under either posture.

**The anti-repair the new wording must not admit, named so build does not reintroduce it:** the
old shape's stated fear was two rules at two ends, either repairable without the other. Grant
cardinality is one rule read at two moments, which is a different shape and not a return to the
refused one — but a §Closing that restated the cardinality rather than citing §Opening's
statement of it *would* be the refused shape, so it cites.

### (4) The consumer half rides a newly minted binding slot, not the existing `ruling-config` one

`lifecycle-kit/templates/lead.md` §Opening an iteration gains a **new binding slot**, and
`.claude/commands/lead.md` binds it with which channel this tree uses: **the lead skill's own
invocation**, carrying one iteration's grant {design-bearing}. A slash-command literal in a kit
template would publish one harness's layout, which is why the fact lives in the shim and not
upstream.

**Two governed surfaces disagreed about which slot carries it, and the disagreement is resolved
here rather than left for a reader to notice.** The ruling record's mechanism sentence assigns
the consumer half to the existing `ruling-config` slot; the queue entry directing the correction
calls for a new one. Both cannot hold — a shim binds a slot by a `**slot-name** —` lead line
under `## Bindings`, so content is behind one slot or another and never in a slot it does not
name. **Ruled `lead, own-authority` 2026-09-06: mint the slot**, on two surfaces neither the
escalating session nor the ruling record cited:

- **`ruling-config` declares its own subject** — *"the tracked agent-definition the lead
  dispatches and the roster it carries."* The open-authorization channel is not about the
  dispatched agent-definition at all; it is about how the **lead itself** is invoked. Putting a
  second, unrelated subject behind a token that declares one is precisely what a slot token
  cannot absorb, because the token is what `check-skill-binding` contracts on.
- **§Policy is config settles the tier outright** — *"policy binding the lead itself is standing
  too, and its tracked source is **this template** rather than the agent definition it
  dispatches."* The authorization channel is policy binding the lead itself, so its source is the
  template, and the template's way of taking a consumer value is a slot.

**The cost of minting is real, priced rather than waved off, and it is routine.** An unbound slot
is red, so a minted slot reds `check-skill-binding` in every vendoring consumer's tree until that
consumer adds the binding. This is the shipped kit's ordinary upgrade shape and it is discharged
in one place: **one release-note line under the allowed-reds shape**, naming the new slot and the
one-line remedy, exactly as a prior release note already does for this same situation. The
amendment records that obligation here because the cost is not paid where it is incurred — it is
paid in the release note, by a later session that will not have read this file.

**`check-shim-restatement` is the binding constraint on how this is written, and it is run
rather than emulated.** The shim may not share a normalized nine-word n-gram with the kit
templates or with `CLAUDE.md`, so the consumer half states the *binding* — which channel, and
what one invocation grants — and does not restate §Opening's mechanism in the shim's own voice.
This is a red condition an author trips by writing the natural sentence, so it is named here
rather than discovered at the hook.

### (5) The unencodability is recorded where the correction lands, not only where it already stands

`lifecycle-kit/SPEC.md` §Honest limit on the lead's open authorization is updated on two points
{design-bearing}:

- Its subject sentence cites *"both ends of the one rule"* to describe what is prose-only.
  Delta 3 moves the bound to the grant, so the citation is re-stated against the new shape —
  the same limit, over the channel and the cardinality as well as over the stop-and-report.
- It gains one clause saying that the **channel and the cardinality are unencodable for the
  identical reason** the open decision is: a bound channel is still a fact about a
  conversation, and a session could write "my invocation carried a two-iteration grant" as
  easily as it could proceed unauthorized today. The two mechanisms already weighed there are
  not re-weighed, and no third is proposed.

**The reader this delta exists for** is a later session that meets a §Opening carrying a
channel and a cardinality and reasonably asks which gate holds them. Without this clause the
honest answer — *none, and none can* — is recoverable only by finding a paragraph in a
different file about a differently-worded rule.

## Producers and consumers

**The bound channel.** *Producer* — the consumer's binding, written in the shim
(`.claude/commands/lead.md` in this tree) and read by the lead session at §Opening; the
enabling configuration is the shim itself, which every lead invocation loads, so the producer is
reachable on the only path a lead takes. *Consumer* — the lead session, at the one transition
where it decides whether it may open. There is no second consumer and no machine one, which is
delta 5's whole subject.

**The newly minted binding slot.** *Producer* — the `*<slot-name: …>*` token in
`lifecycle-kit/templates/lead.md` §Opening an iteration. *Consumers* — two, and one of them is
mechanical: **`check-skill-binding`** reads the template's slot set against every shim's
`## Bindings` lead lines and reds on an unbound slot or an orphan binding, and the lead session
reads the bound value at the transition where it decides whether it may open. This is the one
causal chain in the amendment with a machine reader, and it is why the slot's two edits — the
token and this tree's binding — are not separable into two commits: between them the gate is red
by construction. **The third consumer is out of this tree entirely and is the reason delta 4
carries a release-note obligation:** every vendoring consumer's own shim is a consumer of this
slot set, and each is red until it binds the new token.

**The cardinality.** *Producer* — the operator, in the grant. *Consumer* — the lead session,
twice: at §Opening when it reads what it holds, and at §Closing when it reads whether anything
is left. No artifact carries it between those two reads, which is exactly the unencodability
delta 5 records; the lead's own resume journal is where a live lead in fact carries it, and that
journal is scratch by construction.

**No new field, no new message, no new state file.** Nothing structured is added, so there is no
field whose reader must be named — the amendment adds prose to two templates, one SPEC section
and one shim.

**This delta set narrows one corpus — the mechanism sentences in `TRAJECTORY.md`'s ruling
paragraph, which that paragraph's own last sentence says relocate when both halves land — so
the causal-completeness check's point 5 binds and each reader's RED condition is enumerated
rather than its subject.**

- **`check-skill-binding`** reds on an **unbound slot** or an **orphan binding**. Its verdict is
  not monotone under this change in either direction: minting a slot without binding it reds,
  and binding a slot that does not exist reds. It is run, not inspected, and the two edits land
  in one commit.
- **`check-shim-restatement`** reds on **finding** a shared normalized nine-word n-gram between
  a shim and the dedup corpus. Adding text to `.claude/commands/lead.md` can only **introduce**
  candidate n-grams, so this reader is non-monotone under an addition and is run.
- **`check-md-refs`** reds on a reference resolving to nothing. Deleting `TRAJECTORY.md`'s
  relocated mechanism sentences can only remove references, so it is monotone here and clearable
  by inspection — except where delta 3 rewrites a sentence carrying a surviving `§` citation,
  which build runs rather than inspects.
- **`check-surface-duplication` is not a live reader here either, checked the same way as its
  sibling amendment's citation of it.** canon-kit/SPEC.md §check-surface-duplication records
  that this tree registers it in no `gates.list`, and `--for lifecycle-kit/templates/lead.md`
  confirms it at align — the twelve-gate set it returns names no such member. Delta 3's
  §Closing wording still **cites** §Opening's cardinality statement rather than restating it,
  and delta 5 cites rather than restates, on the same de-literalization ground delta 3's
  anti-repair states from the design's side — but no gate's green status turns on it.
- **`check-lifecycle-registration`** reds on `CLAUDE.md`'s marker block drifting from the block
  regenerated from the live stage machine. **Probed, not assumed:** the block derives from the
  stage machine — stage names, order, the queue file — and from no template slot, so this change
  cannot stale it. Recorded because a slot mint looks like registration-shaped work.
- **`check-docs-mirror-fresh`** byte-compares the on-site mirror against its source and is
  monotone in nothing; the `lifecycle-kit/SPEC.md` edit stales it and it clears only by
  regenerating.
- **`check-value-rollup-fresh`** and the footprint emitter: the footprint's measured set
  includes a kit's `templates/` markdown, so editing `lifecycle-kit/templates/lead.md` moves the
  footprint and therefore the rollup block that joins it. This is the fan-out the amendment
  would otherwise name one surface of, and it is stated because the trigger is the *template*
  edit rather than the SPEC edit that looks like the bigger change.

## Existing sections updated

- **`lifecycle-kit/templates/lead.md` §Opening an iteration** — the *obtained explicitly and
  separately* sentence, the channel, and the cardinality (deltas 1 and 2). The three
  not-inferred-from clauses and the scale paragraph are read and deliberately left standing.
- **`lifecycle-kit/templates/lead.md` §Closing an iteration** — the rule re-grounded on the
  grant and its *one rule attached to the iteration boundary* self-description corrected, with
  the roll-forward paragraph beneath it untouched (delta 3).
- **`lifecycle-kit/templates/lead.md` §Policy is config, not prose** — its *policy binding the
  lead itself is standing too, and its tracked source is this template* clause is one of the two
  sentences delta 4's ruling turns on, and its `ruling-config` token's declared subject is the
  other; both are read and neither is edited, since the new slot sits in §Opening beside the rule
  it configures rather than in this section's roster (deltas 2 and 4).
- **`lifecycle-kit/SPEC.md` §Honest limit on the lead's open authorization** — its subject
  citation and the unencodability clause (delta 5).
- **`.claude/commands/lead.md`** — the consumer binding naming this tree's channel and its
  cardinality (delta 4).
- **`TRAJECTORY.md`, the lead-invocation open-grant ruling paragraph** — two edits, and the
  second is corrective rather than relocational. First, its mechanism sentences relocate to the
  homes this amendment gives them, on that paragraph's own instruction that they do so when both
  halves land; the **ruling, its date and its channel stay**. Second, its consumer-half sentence
  **names `ruling-config`, and delta 4 rules the other way**, so the sentence is corrected where
  it stands to name the minted slot. **That correction is not a reversal and the distinction is
  the paragraph's own:** the ruling is the two limbs — the invocation is the grant, and one
  invocation grants one iteration — with their date and channel; the slot assignment is the
  *mechanism* sentence, which the paragraph itself declares transitional in the words *"this
  paragraph's mechanism relocates to those two homes"*. Correcting a sentence already marked for
  relocation is the third act of the relocation, not an edit to a closed ruling (all deltas).
- <!-- update-target-exempt: generated projections with their own freshness gates and regen commands, rostered in docs/site-architecture.md §Generated projections and their freshness gates; their content is derived byte-for-byte from the sources the deltas above edit, so no delta owns it --> the `docs/` mirror of `lifecycle-kit/SPEC.md`, plus `docs/footprint.md` and `docs/value.md`'s rollup block.

## Definition of Done

- [ ] **Causal completeness** — the bound channel, the cardinality and the minted slot each name
      a reachable producer and a named consumer; the one machine reader in the set,
      `check-skill-binding`, is run rather than inspected, and the non-monotone readers above are
      run.
- [ ] **The slot's two edits land in one commit** — the template's token and this tree's binding,
      because between them `check-skill-binding` is red by construction.
- [ ] **The mint's cost is booked where it is paid** — one release-note line under the
      allowed-reds shape, naming the new slot and its one-line remedy, so a vendoring consumer
      meets the red with its fix rather than as a breakage.
- [ ] **Merged with no information lost** — §Closing's roll-forward analysis and §Opening's
      scale ground both survive the re-grounding; `TRAJECTORY.md` keeps the ruling, its date and
      its channel after its mechanism relocates.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every surface citing *obtained explicitly and separately* or the
      boundary-attached bound re-checked; the docs mirror, footprint and rollup regenerated;
      `check-md-refs` green.
- [ ] **No gate is promised** — the merged §Opening and the merged SPEC limit both say the
      channel and the cardinality are prose-only and unencodable, so no later session hunts for
      enforcement that cannot exist.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to the gap inbox.
