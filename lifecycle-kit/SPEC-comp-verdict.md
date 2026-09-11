# SPEC amendment: composition-verdict

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land. Where replacement wording is given it is marked **Not yet applied** at the passage itself.

The entry this amendment discharges is `composition-verdict-unrecorded-at-unit-set-ruling`. Scope's
economic composition test is template prose, and it produces no verdict. So a unit-set escalation
can propose a lone unit without the test having run, and a lead can route it without noticing the
test never ran. The witness is `deferred-pool-triage`, which opened on one promoted unit. Scope's
escalation offered three single cuts and no bundle, and the lead recommended the lone unit without
applying the test either.

## What changes

### (1) Every proposed unit set carries a composition verdict

`lifecycle-kit/templates/stages/scope.md`, the economic composition test paragraph, gains a closing
passage {design-bearing}. The paragraph opens "**Weigh the iteration's cost before opening it**".
**Not yet applied:**

> **Record the test's verdict in the unit-set escalation — on every proposed set, not only a lone
> unit.** The escalation's Recommendation carries one line in one of two forms:
> `Composition: bundled — <the deferred entries the set joins and the surface they share>`, or
> `Composition: stands alone — <why this unit justifies an iteration's fixed cost>`. Where the
> recommendation stands alone, Options also offers the bundle the test would have formed, so the
> party ruling on the set sees the cut it declines.

**Why every set, and not the lone unit the test names.** The lone-unit case is itself a judgment:
*sub-threshold*, *lone*. A requirement gated on that judgment is skipped exactly when the judgment
is skipped, which is how the witness happened. A line present on every set has no trigger to miss.
And for a set that already bundles, the line is one sentence naming what it shares.

**Why the bundled alternative rides Options.** The witness's defect was an option set with no bundle
in it, not a missing sentence. A stands-alone argument read with no bundle beside it cannot be
weighed against anything.

### (2) The lead returns a unit-set escalation that carries no verdict

`lifecycle-kit/templates/lead.md` §Opening an iteration gains a passage {design-bearing}. It follows
the paragraph beginning "Scope's proposed unit set returns as an ordinary four-header escalation".
**Not yet applied:**

> A unit-set escalation whose Recommendation carries no `Composition:` line goes back to scope before
> it is routed. It is never relayed to the operator and never ruled. A relay carries the verdict
> verbatim. The check is presence, not quality: whether a stands-alone argument persuades is for the
> party ruling on the set, and a lead grading it would be the lead-authored judgment on the unit set
> this section refuses.

**Why presence and not quality.** This section already bars the lead from selecting the unit set.
Scoring a verdict's argument is selection by another route. Returning an escalation that carries no
verdict selects nothing: it asks for the test's output before anyone rules on the set.

### (3) The template contract's summary names the check

In lifecycle-kit/SPEC.md §templates/lead.md, the opening-an-iteration contract's parenthesis reads
"routes scope's proposed set back as an ordinary escalation" {mechanical}. It gains "once the set
carries scope's composition verdict". **Not yet applied.**

## Producers and consumers

The one new interface is the `Composition:` line in a unit-set escalation's Recommendation. It adds
no file, knob, tag or event.

- **Producer.** The **scope stage session** writes the line when it escalates a proposed unit set. It
  does so under the split posture, where the escalation goes to the lead, and with no lead, where it
  goes to the user. This happens once per iteration, at the unit-set proposal the scope template
  already obliges. **Enabling config: none.**
- **Consumers, each with the transition it reads at.**
  1. **The lead** reads the line's **presence** at the routing transition, to route the escalation or
     return it (delta 2).
  2. **The party ruling on the set** reads the line's **form and content** at the ruling: the
     operator through the lead's relay, or the user directly where no lead runs (delta 1).
- **Every field has a named reader.**
  - **The form word** (`bundled` or `stands alone`) is read by both consumers: by the lead as part of
    presence, and by the ruling party as the test's outcome.
  - **The content clause** is read by the ruling party.
- **The honest limit.** No gate reads the line. The escalation travels on the message channel, which
  is transport and never a store. Under a lead, the lead's refusal is the whole enforcement. With no
  lead, the template sentence is.
- **Readers surveyed across the whole tree.** `composition` was grepped across lifecycle-kit's
  templates, delegation-kit, `.claude/` and `CLAUDE.md`, with no stderr suppressed. Its only uses on
  the test are scope.md's ranking and test paragraphs. lifecycle-kit/SPEC.md carries no summary of the
  test, so no integration prose restates it. The escalation's four headers are owned by lead.md
  §The escalation protocol. This delta adds a line inside one header and mints no fifth header, so
  that section and the stage-session roster that points at it are unchanged.

## Existing sections updated

- `lifecycle-kit/templates/stages/scope.md`: the economic composition test paragraph (delta 1).
- `lifecycle-kit/templates/lead.md` §Opening an iteration: the returned-escalation passage (delta 2).
- `lifecycle-kit/SPEC.md` §templates/lead.md: the opening-an-iteration contract summary (delta 3).
- `docs/lifecycle-kit/SPEC.md`: the generated mirror, regenerated in the landing commit (delta 3).

## Retired spellings

- None — no delta retires a spelling. Each delta adds a sentence and leaves every existing one in
  place.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and a
      named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
