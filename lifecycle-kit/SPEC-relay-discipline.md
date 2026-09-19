# SPEC amendment: relay-discipline

Three sibling entries, ruled together as their own bodies ask, pairing one
amendment:

- `lead-specifies-constraint-not-mechanism`: a lead hands down a *mechanism*
  where only the constraint was its to set.
- `dispatch-claim-evidentiary-tier-unmarked`: a lead relays a *claim* upgraded
  in transit, an inference sent as a finding, or a grant sent as unspent after it
  was spent.
- `relayed-rule-role-scope-unchecked`: a lead relays a *rule* whose owning surface
  binds a role other than the receiver's.

**The envelope question is answered.** Each entry was filed rather than landed
because `templates/lead.md` binds every consumer, so a rule added there changes the
envelope. The operator's direction at this iteration's scope (2026-09-19,
lead-relayed) was that the claim generalizes and all three siblings join. What
remains is wording and placement.

**Placement: the relay rule the three are cases of.** lead.md's "**Relay, never
assert.**" paragraph already rules one of them: a factual claim travels with its
provenance. The three entries are three more things that travel on the same
channel, a mechanism, a claim's tier and a rule, and each fails the same way:
authority is added in transit. The paragraph is re-phrased to cover all four
rather than gaining three siblings.

**The receiving side is where most instances were caught.** The three entries attest
eight instances. The six mechanism and claim-tier instances were each caught by the
receiving session declining to trust the prompt. The two role-scope instances were
caught by the lead after the work landed wrong, one of them at the price of a second
full spine run. No instance was caught by the lead before the relay. The lead-side rule is the envelope change. The
receiving side is this repo's standing dispatch policy, so it lands in the agent
definition the lead dispatches (`.claude/agents/stage-session.md`, the
`ruling-config` binding).

**Nothing can check either side, and that is stated rather than implied.** A
dispatch never enters the tracked tree, so no gate reads a prompt. Roles are defined
across the templates and each kit's SPEC, so a role-scope scanner would first have
to resolve a relayed sentence to its owner, which is semantic. A tier marker in a
prompt is written by the party it constrains, the same shape §The state machine's
limit on the lead's open authorization records. The limit sits in §templates/lead.md
so a later session does not build the impossible gate.

**Grants take a spent state, not a tier.** The third tier instance was a relayed
authorization that had been spent hours before the relay. Marking it *measured*
would not have caught it, because it had been true. So the relay rule treats a
grant as carrying whether it is spent, read at relay time off the target's current
state.

**Refused:** merging the three entries into one, which each body already declined
because their evidence is independent. Also refused: a three-sibling paragraph
block in lead.md, which would put three restatements of one channel rule side by
side.

**Probed at authoring, 2026-09-19.** `grep -n -i "evidentiary\|constraint"
lifecycle-kit/templates/lead.md` returns nothing, so lead.md carries no tier or
constraint rule. `grep -n "Relay, never assert" lifecycle-kit/templates/lead.md`
returns line 146, the paragraph delta 1 rewrites, which covers a factual claim's
provenance only.

## What changes

### (1) lead.md's relay rule covers claim tier, mechanism and role {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md` §The lead model, the
paragraph opening "**Relay, never assert.**" becomes:

> **Relay, never assert.** The lead manages on *optimal* rather than extensive
> context, so what it hands a stage session travels no stronger than the lead holds
> it:
>
> - **A claim carries its tier**: measured (the command, and when it ran), inferred,
>   or expected. A grant also carries whether it is spent, read off its target's
>   current state at relay time.
> - **A fix travels as the constraint it must meet**, never as the mechanism. The
>   stage session holds the oracle and finds the mechanism.
> - **A rule travels only to a role it binds.** Before relaying one, read whose role
>   its owning surface names.
>
> What the lead rules alone is scope, envelope and priority: the things no gate can
> decide and no grep can answer.

### (2) The stage-session agent reads a dispatch at its marked strength {mechanical}

**Not yet applied.** In `.claude/agents/stage-session.md` §Ruling classes, the
**Decide alone** list gains a third bullet:

> - How strongly to trust a dispatch. A claim with no tier is inferred, so run it
>   before building on it. A relayed mechanism is a proposal you verify against the
>   constraint. A relayed rule binds you only where its owning surface names your
>   role, so check before acting.

### (3) §templates/lead.md re-grounds the asymmetry paragraph {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §templates/lead.md, the paragraph
opening "**Relay-never-assert and the checked-figure rule rest on one asymmetry.**"
is re-phrased to carry:

- The asymmetry, unchanged in substance. A stage session writes state under
  oracle-first, fixtures and a battery. The lead writes none, yet steers what
  sessions land.
- The four things the relay rule now covers, as four ways of adding authority in
  transit: a claim's tier, a grant's spent state, a mechanism, and a rule's role.
  No attested instance was caught by the lead before the relay, and most were
  caught by the receiving session, which is why the consumer's agent definition
  carries the receiving half.
- **Honest limit:** no gate reads a dispatch, and a tier or spent marker is written
  by the party it constrains. The rule is prose on both ends. A role-scope scanner
  would have to resolve a relayed sentence to its owning surface, which is semantic.
- Why a grant takes a spent state rather than a tier: the attested grant was a fact
  that went stale between act and relay, so a tier would have marked it correctly
  and still carried it.

### (4) The generated mirror {mechanical}

`docs/lifecycle-kit/SPEC.md` is regenerated after delta 3.

## Producers and consumers

- **The relay rule** (delta 1). Producer: the lead, when it writes a dispatch or an
  answer. Consumer: the receiving stage session, which reads each relayed item at
  the strength marked. No field is minted: tier words are prose, and a marker
  grammar would want a reader, which no gate can be.
- **The receiving half** (delta 2). Producer: the consumer's agent definition. This
  repo binds it through `ruling-config`. Consumer: every dispatched stage session at
  every escalate-or-decide judgment.
- **Point 6.** No corpus is obliged member by member. The rule binds future
  dispatches, which are not enumerable at authoring.

## Existing sections updated

Roster probe: `git grep -n "Relay, never assert\|Relay-never-assert" -- '*.md'
':!TASK-QUEUE.md' ':!docs/posts'`.

- `lifecycle-kit/templates/lead.md` §The lead model (delta 1)
- `.claude/agents/stage-session.md` §Ruling classes (delta 2)
- `lifecycle-kit/SPEC.md` §templates/lead.md (delta 3)
- `docs/lifecycle-kit/SPEC.md` (delta 4)

## Retired spellings

- None — the relay rule keeps its name. Nothing is renamed or removed.

## Definition of Done

- [ ] **Causal completeness** — every point holds for the relay rule and its
      receiving half.
- [ ] **Instruction surfaces: instruction only** — lead.md and the agent definition
      carry the rule, and the SPEC carries its grounds and limit.
- [ ] **Merged with no information lost** — delta 3 re-phrases its paragraph.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component, discharged at the iteration.
- [ ] **Done moves** — all three paired entries move to Done in the merge commit,
      before the drain stage.
- [ ] **Release declaration** — the lead template's changed rule is declared in the
      unit that lands it.
- [ ] **Gaps filed** — any cross-component gap found during the work filed.
