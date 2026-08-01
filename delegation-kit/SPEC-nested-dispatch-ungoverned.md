# SPEC amendment: nested-dispatch-ungoverned

Filed as "the dispatch layer's rules are written for the root dispatch and
silently exempt everything below it", over two halves — budget and tier. Both
premises were re-verified while authoring this amendment, and **one of them was
wrong**. What follows states the correction first, because it changes what is
owed.

## The budget premise was false; the tier premise was exact

**Budget.** The entry says the per-dispatch guard "fires on the `Agent` call it
gates, not on that call's children", with "no inheritance into the dispatched
session" — inferred from the guard script, which is eighteen stateless lines
with no depth counter. The script reading is correct and the inference from it
is not: hooks registered in a project's settings are **session-wide and fire
inside dispatched sessions too**, so a `PreToolUse` hook on the dispatch tool
re-arms at every depth, carrying the dispatching agent's identity in its input.

Verified first-hand rather than from documentation alone: the `spec` session
authoring this amendment *is* a dispatched stage session, and its own dispatch
of a read-only child fired the budget guard in its context, returning a live
verdict. The guard re-armed one layer below the root.

So the observation behind the entry — "it ran exactly once for this close
stage" — was accurate about what the *lead* could see and wrong about what
happened. It fired three times; two of those fires were inside a child's context,
invisible to the parent that was counting. A guard whose output is only ever
seen by the session it fires in looks absent from every other vantage point.

**What survives the correction, and it is real.** The verdict is **per call and
depth-blind**. Each fire prices the window at that instant for that one
dispatch; no fire ever prices the *subtree* a dispatch is about to open. A
dispatch that will itself fan out is therefore admitted on a reading that
describes none of its actual cost, and the root's `OK` at 92% was exactly that:
correct about the call, silent about the three sessions it authorized. The
residue is a **projection** gap, not a coverage gap — and it is cheaper than
filed, because no re-arm mechanism is owed.

**Tier.** The entry's corrected premise holds exactly, and the harness makes it
sharper than filed. Model resolution runs: a subagent-model environment override,
then the per-dispatch `model` parameter, then the agent definition's `model:`
frontmatter, then the main conversation's model. A definition that **omits**
`model:` does not fall through to a default — it defaults to the literal value
`inherit`. Declining to choose is not an absence of a choice; it is spelled, and
what it spells is *the dispatcher's tier*. Close inherited the judgment tier and
its two audit sweeps inherited it in turn, exactly as filed.

**Why this is still one unit.** The diagnosis is unchanged — a rule that stops
reaching rather than reddening at the layer boundary — and both deliverables land
on the same two bullets of the same template, read at the same transition by the
same reader. The correction changes the halves' *sizes*, not their kinship: one
collapses to a clause, the other grows the artifact the entry believed could not
exist.

**Sequencing — this unit lands LAST in the delegation lane, and the order is
directional.** The queue's surface notes say only "serialize" against
`agent-execution-backgrounding-role-scope`. Established at align: the direction
is fixed, because delta 6's agent definition **cites** that amendment's
durability rule, which does not exist until it lands. No gate supplies the
order — `check-rule-citation` scans only `delegation-kit/SPEC.md` §The
delegation model forward into `templates/agent-execution.md`, so a citation
dangling from a consumer agent definition reddens nothing. The full chain is
`resume-journal-deletion-vs-pull-channel` → `agent-execution-backgrounding-role-scope`
→ **this unit**. The two amendments' edits to §The delegation model are disjoint
paragraphs (this one corrects the per-dispatch-freshness and settings-env-block
paragraphs; that one appends two rationale paragraphs), so the shared section is
a merge concern only — the citation is what makes the order load-bearing.

## What changes

### Delta 1 — the budget rule gains its subtree clause {design-bearing}

Appended to the body of **Budget-check before *each* dispatch in a fan-out**;
the lead-in is untouched, per the same citation constraint the sibling amendment
observes.

The rule already says to project the next wave's burn from the last wave's. What
it does not say is that the projection has to reach **downward** as well as
forward:

> The guard re-arms at every depth — a dispatched session's own dispatches fire
> it too — but each verdict prices **one call**, never the subtree that call
> opens. Nothing between a dispatch and its children's children ever sees the
> whole. So budget a dispatch you expect to fan out for its **subtree**, not its
> own turn, and read a child's fan-out as your spend even though its verdicts
> land in a context you never see.

The last clause is the observational correction made into guidance: a supervisor
watching its own hook fires is watching a strict undercount of its own tree, and
knowing that is what stops the next reader repeating the inference this entry
was filed on.

### Delta 2 — the SPEC records the propagation fact {design-bearing}

§The delegation model currently describes the guard as giving "per-dispatch
freshness, not a start-of-session reading a mid-session window outlives". True
and incomplete in the direction that misled a careful reader: the sentence says
nothing about *which sessions* the hook fires in, and the script — stateless,
registered once — invites the wrong conclusion. A scope survey read it that way
and filed against it.

The section gains the fact and its honest limit: the hook is session-wide and
fires inside dispatched sessions, so the guard's coverage extends to nested
dispatch; what it cannot do is aggregate, because each fire is independent and
holds no state about the tree. That pairing — covered but not aggregated — is
what delta 1's clause exists to compensate, and stating both together is what
keeps a future reader from re-filing either half.

This is a **correction of an inference the SPEC's own silence invited**, which
is why it earns spec residency under the section's stated rule: it is a
calibration history, complete with the wrong turn it caused.

### Delta 3 — the tier rule gains the inheritance clause {design-bearing}

Appended to the body of **Match the dispatched model and effort to the unit's
shape**; the lead-in is untouched.

The rule is already role-neutral and already reaches a stage session dispatching
its own children — it says selection sits with *the dispatching session*, and a
stage session dispatching is one. It went unapplied anyway, because nothing in
it says what happens when you apply nothing:

> Selection is **affirmative**: an unselected dispatch does not fall back to a
> cheap default, it **inherits the dispatcher's tier**, so declining to choose
> silently buys the most expensive tier in reach — and it buys it precisely for
> the read-only fan-outs that are the cheapest work you dispatch. A standing
> choice belongs in the tracked agent-type definition (above); an omitted
> `model:` field there is not a neutral default but the inherit default, so the
> field is stated even when the answer is to inherit.

This is what "make the existing rule bite against a silently inheriting default"
comes to: the rule did not need widening, it needed the sentence naming the cost
of its own no-op.

### Delta 4 — the environment override is ruled out, and why {design-bearing}

The cheapest-looking lever is the wrong one, recorded so it is not rediscovered
as an improvement. The harness exposes a subagent-model **environment**
override, and this repo already has a working transport for env into hook and
harness context (the settings env block, which §The delegation model documents
for the pause-threshold raise). Setting it would pin every subagent to a cheap
tier in one line.

It is refused on **precedence**: that override sits *above* the per-dispatch
`model` parameter, so it would silently defeat the lead's per-batch tier pin —
the mechanism `lifecycle-kit/templates/lead.md` §Economics depends on for
tiering a batch by its deltas' work classes, and the mechanism this repo's own
lead binding uses to keep stage sessions on the judgment tier. A lever that
overrides the deliberate choice is not a default; it is a ceiling.

The correct rung is the **frontmatter**, third in the chain: it supplies the
standing default the rule already asks for, and the per-dispatch parameter still
overrides it, so the lead keeps its pin and an unselected dispatch stops
inheriting. Choosing the rung *because of where it sits in the precedence order*
is the whole design content of this delta.

### Delta 5 — a gate: a tracked agent definition states its tier {design-bearing}

The entry ruled the tier half ungateable — "no oracle possible, since a
sub-agent's tier leaves no tracked artifact". That is true of a per-dispatch
habit and false once delta 3 puts the standing choice in a tracked file. The
artifact exists because this unit creates it, so the oracle follows.

`check-agent-tier-explicit`, in delegation-kit: every agent definition under the
scanned directory declares a `model:` field in its frontmatter.

- **It polices silence, not the choice.** An explicit inherit-valued `model:`
  **passes** — a stage session that should ride its dispatcher's tier is a
  legitimate answer, and the gate has no business overruling it. What reds is
  omission, which is the only state that is indistinguishable from not having
  thought about it. That line is the gate's whole theory and belongs in its SPEC
  section, because a reader will otherwise expect it to enforce cheapness.
- **Config-via-env:** one knob for the directory the gate walks, defaulting to
  this harness's conventional agent-definition directory. The literal lands in
  delegation-kit/SPEC.md §Layout and configuration paired with the `lib/`
  fallback, per the default-statement grammar.
- **Counted inertness:** a consumer with no such directory scans zero
  definitions and reports a clean zero count, the same shape the kit's other
  derived-scan-set gates use. No roster, no registration list.
- **Honest limit:** it holds the *tracked* surface only. A dispatch that names
  no type and no `model` parameter inherits, and leaves no artifact for any gate
  to read. The gate converts the standing-choice class from unenforceable to
  enforced and leaves the per-dispatch class to delta 3's prose — strictly
  better than nothing, and not a claim to have closed the tier half
  mechanically.

**The four gate contracts, named rather than left to the meta-gates to
discover** (gate-sdk/SPEC.md §The gate model; an earlier draft specified only
the fixture pair, and a contract a new gate meets by accident is a build-stage
surprise):

- **Output** (§The gate model, the output contract). Clean is exactly one line
  `AGENT-TIER-EXPLICIT: clean (<parenthetical>)` — the stable upper-token id
  paired with the counted scan set, which is also how the counted-inert case
  reports (zero definitions scanned is a clean line, not a skip). A finding is
  one line per offending definition giving its path and the missing field, then
  a `help:` line naming the concrete action — add an explicit `model:`, and
  `inherit` is a valid answer. Exits: 0 clean, 1 finding, 2 harness error.
  Enforced by `check-gate-output` plus the `good/` fixture.
- **Fail-closed** (§The gate model, the fail-closed contract). The gate branches
  on the **exit status** of every subprocess it captures — the definition
  enumeration and the frontmatter parse — never on the emptiness of their
  output, since an empty parse is exactly what a crashed parser and a
  `model:`-bearing file both look like. `fail_closed` at each capture; no
  `# fail-closed-exempt:`. Enforced by `check-gate-fail-closed`.
- **Fixture pair** (§The gate model). A `good/`+`bad/` pair under the kit's
  tests dir, from the check skeleton — delta 7. No `# no-fixture:` claim: a
  reddening violation is trivially craftable (a definition with no `model:`).
  Enforced by `check-gate-fixture-coverage`.
- **Self-lint** (§The gate model). ShellCheck at `-S warning`, with any
  suppression inline and justified — never a `.shellcheckrc`. Enforced by
  `check-shellcheck`.

### Delta 6 — a tracked agent type for the read-only audit class {design-bearing}

The rule says an audit or survey "rides the type whose description commits to
review work, never one that disclaims it (an excerpt-locator serves pure search,
not audit)". Applying it today has no landing place: the harness's read-only
search type explicitly disclaims audit, and the general-purpose type commits to
no tier — so a stage session dispatching an audit sweep either violates the
rule or inherits. That absence is why close's two sweeps rode the judgment tier.

This repo adds one consumer agent definition for the class: read-only tooling,
a description that commits to audit and review work so the rule's
type-selection sentence resolves to it, and an **explicit** cheaper-class
`model:` — which is also the second subject that makes delta 5's gate meaningful
on landing rather than vacuous.

**Which model literal is a build-time read of the live roster, not a value this
amendment bakes** — the protocol is explicit that a baked model-name list in any
doc is drift by construction. What the amendment fixes is the *class* (cheaper
than the judgment tier) and the *rung* (frontmatter, delta 4). The assignment
joins the re-judge-on-roster-churn roster the lead binding already carries.

The definition's standing policy cites the sibling amendment's durability rule
rather than restating it: a read-only child owes no journal, and its parent lands
what it returned before acting on it.

### Delta 7 — landing {mechanical}

A `good/`+`bad/` fixture pair for the new gate from the check skeleton, its
`# graph:` manifest coupling the agent-definition surface (`tier=` lives there,
not in the registry line), registration in this repo's `gates.list` as a bare
name, a §check-agent-tier-explicit section in delegation-kit/SPEC.md, and the
generated projections regenerated — the pre-commit hook, the graph artifact, the
enforcement map, and the docs mirror, each recovered by running the battery
rather than transcribed here. This iteration's release note carries a
Tightened-gates bullet for the new gate.

**Four more obligations a new gate incurs here, added at align — each verified
against the surface that would go stale, because "the battery recovers it" is
true only of the freshness-gated ones.**

- `delegation-kit/README.md`'s gate-roster marker block, held to `checks/`
  basename parity by `check-readme-roster`. Freshness-gated, so the battery does
  catch it — named because the kit-landing checklist owns it explicitly.
- `docs/value.md`'s rollup, where delegation-kit's gate count is the literal
  `2`. Regenerated by its own emitter and byte-gated by
  `check-value-rollup-fresh`.
- `docs/footprint.md`'s token-cost table, the same shape.
- `delegation-kit/smoke/install.sh`'s `gates.list` append is a **judgment call,
  not an automatic add**: it registers `check-gate-tamper` alone today and
  deliberately not `check-rule-citation`, because the smoke consumer is a
  scratch tree that need not carry the kit's own SPEC and template. This gate is
  counted-inert on a tree with no agent-definition directory, so registering it
  is safe and demonstrates the inert path; land it registered unless the smoke
  run says otherwise.

**Unenforced literal this unit falsifies, costed rather than skipped.** The
battery goes from 89 gates to 90, and the "89 gates" literal is restated in
`TASK-QUEUE.md` (the `gate-battery-spawn-hoists` entry and one Deferred entry)
and in `site-kit/SPEC-docs-renderer-batch-contract.md`'s measurement table. No
gate holds any of them: `check-manifest-count` scans canonical specs, `README.md`
and `CLAUDE.md`, and excludes amendments and the queue. The site-kit amendment
has been marked at align to state its count as a dated reading; the queue
literals are read-once measurement records that name their own run, so they are
left as filed rather than chased.

**The new agent definition joins a scan set beyond this gate's.**
`scripts/canon-config.sh` lists `.claude/agents/*.md` in
`CANON_KIT_PROSE_SURFACE_GLOBS`, and canon-kit's loader folds slot-free
candidates from that glob into the manifest set the manifest-narration gate
family reads (`canon-kit/lib/spec.sh`, §check-spec-pointer). So delta 6's file is
governed by that family from the moment it lands — a `# spec:`-style pointer
obligation, not merely a frontmatter field.

## Producers and consumers

- **The `model:` frontmatter field on a tracked agent definition** — the one new
  state this unit introduces. *Producer:* whoever authors or edits an agent
  definition (delta 6 writes the first new one). *Consumers, two, both named:*
  (a) the **harness**, at subagent-model resolution, third in the precedence
  chain — the transition where the field actually changes behavior; (b)
  `check-agent-tier-explicit`, at pre-commit, which reads only its
  presence/absence. There is no third field added: an omitted-but-defaulted
  value, a depth counter, and a per-type budget were each considered and each
  would have had no reader, so none is added.
- **`check-agent-tier-explicit`'s verdict** — *Producer:* the gate under the
  battery, fired at the pre-commit tier by the generated hook's trigger block
  and at the CI tier by the battery runner. *Consumers:* the committing author,
  and CI's required check. Its enabling registration is real rather than
  test-only: the `gates.list` line in delta 7 is what puts it in the battery,
  and the fixture-coverage meta-gate reds without the pair.
- **The directory knob** — *Producer:* `lib/delegation.sh`'s default fill, or a
  consumer's delegation config file. *Consumer:* the gate, at its scan-set
  derivation. Emitted on every battery run in this tree via the loader default,
  so the scan is live rather than fixture-only.
- **The budget verdict at nested depth** — *not* a new interface; this
  amendment only corrects the record about where the existing one fires.
  *Producer:* the harness, firing the registered hook in every session including
  dispatched ones. *Consumer:* the dispatching agent at that depth, reading the
  verdict line off its own hook context. The correction's own reader is the
  maintainer, via delta 2.
- **The tier and subtree clauses (deltas 1, 3)** — *Producer:* the protocol
  template. *Consumer:* the dispatching session at its dispatch-decision
  transition, at any depth. Reachable because the guard's block message names
  the protocol skill, and because this repo's stage-session definition loads
  unconditionally and cites it.

**Seam.** Mechanism on the kit side: the gate reads a frontmatter field's
presence and carries no consumer vocabulary, no model names, and no rule
content — the churning model roster is exactly the private-to-the-moment content
the kit must not hold, which is why the class-to-literal mapping stays a
dispatch-time read and the tracked literal stays in the consumer's own agent
file. The directory is the config seam per config-via-env. Everything
tier-specific — which model, for which class — is consumer config by
construction.

## Existing sections updated

- **`delegation-kit/templates/agent-execution.md`** — the **Budget-check before
  *each* dispatch in a fan-out** body (delta 1) and the **Match the dispatched
  model and effort to the unit's shape** body (delta 3). Both lead-ins verbatim.
- **delegation-kit/SPEC.md §The delegation model** — the hook-propagation fact
  and its no-aggregation limit, correcting the per-dispatch-freshness paragraph
  (delta 2), and the ruled-out environment override with its precedence reason
  (delta 4). The paragraph already documenting the settings env block as the one
  working knob transport gains the note that the same transport is *not*
  sanctioned for the model override, so the two do not read as one permission.
- **delegation-kit/SPEC.md §Layout and configuration** — the new directory knob
  and its default literal (delta 5).
- **delegation-kit/SPEC.md** — a new §check-agent-tier-explicit section (deltas
  5, 7).
- **`.claude/agents/`** — the new read-only audit type (delta 6); the existing
  stage-session definition is unchanged by this unit, its `model:` already
  explicit.
- **`scripts/gates.list`** and the generated projections (delta 7).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
