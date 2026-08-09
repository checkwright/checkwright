# SPEC amendment: init-lifecycle-agent-block-seeding

Rules the question `init-lifecycle-agent-block-seeding` filed: `checkwright init`
seeds doctrine-kit's agent-file marker block and not lifecycle-kit's, and the
exclusion carries no stated rationale. The entry left the outcome genuinely open —
"either a considered decision whose rationale was never written down, or a genuine
gap" — and said deciding which precedes any estimate of either.

**The ruling: it was considered, the rationale is derivable from a rule the
installer already follows for gates, and the deliverable is to state that rule
rather than to change what an adopter's agent file contains.** The seeding is
correct as it stands.

## The rule

**A kit's agent-file block is seeded at install iff a gate registered at install
reads it.**

Seeding follows the gate, not the kit. It is the agent-file half of the rule the
installer already applies to gates — `recipe_gates` registers a kit's
`zero-config` members and no others (installer/README.md §What init seeds) —
extended to the one other thing `init` writes on a kit's behalf.

It resolves the filed question in both directions with nothing left over:

- **doctrine-kit is seeded** because `check-doctrine-registration` declares
  `install: zero-config`, so the gate that reads the block is in the registry
  `init` writes. The block earns its residency on the tree that receives it: it
  is read on day one, by a gate that would otherwise red.
- **lifecycle-kit is not** because every lifecycle-kit gate declares `on-surface`
  or `never` — `check-lifecycle-registration` among them — so nothing `init`
  registers would read the block. Seeding it would put resident, always-loaded
  instruction for a stage machine into every adopter's agent file, charging every
  session's context for machinery nothing yet enforces. That is the same posture
  `installer/profiles.list` already states for the kit as a whole, in its
  `delegation` roster comment: *"Vendored is not yet enforced."*

The rule is also what makes `recipe_needs_agent_file`'s two-kit membership
derivable rather than a roster: the predicate answers a narrower question than
the case arms do, and the two questions are easy to conflate.
`recipe_needs_agent_file` asks **must the agent file exist** for this kit's
starting gates — context-kit answers yes and writes nothing into it, which is why
it is in the predicate and has no `recipe_seed` arm. The case arms ask **does this
kit write into it**. doctrine-kit answers yes to both, which is what made a single
predicate look like the seeding roster it is not.

## What changes

- **(D1) State the rule in `installer/README.md` §What init seeds** — one
  paragraph, beside the starting-gate roster paragraph that already states the
  gate half. It is the widest-true tier: the rule is about what `init` writes on
  any kit's behalf, not about lifecycle-kit. **design-bearing** — the rule is the
  amendment's whole content, and its wording is what a future kit author resolves
  their own case against.

- **(D2) Point the two producers at it.** The `spec:` directive above
  `recipe_needs_agent_file` in `installer/lib/common/recipe.sh` gains the citation
  it lacks (it carries none today), and the `doctrine-kit` arm's existing
  directive gains the clause saying *why* this kit has an arm. Both cite D1's
  section; neither restates it. **mechanical** — two directive lines against a
  section D1 authors.

- **(D3) Correct `installer/README.md`'s stated reason for lifecycle-kit's
  install-time silence.** The sentence today reads that "lifecycle-kit's **two**
  want a stage attestation only a stage session can write". Eleven lifecycle-kit
  gates ship: four declare `never`, seven declare `on-surface`. Two of the seven
  read the stage attestation the sentence names; three more read surfaces a stage
  session writes, so the phrasing reaches them; and **two —
  `check-lifecycle-registration` and `check-merge-attrs` — read surfaces
  `bin/install-lifecycle.sh` writes and no stage session ever touches**, so the
  stated reason does not describe them at all. The claim is a true statement
  about two gates offered as the reason for a kit-wide posture, and a reader
  checking it against the tree finds it undercounting in a way that matters here:
  those two are precisely the gates this ruling turns on. Restate it as the
  posture it is — no lifecycle-kit gate is `zero-config`, for reasons that differ
  across the roster. **design-bearing** — the honest restatement has to be true of
  all seven without over-claiming one reason for them.

## What this amendment deliberately does not do

**It does not wire `bin/install-lifecycle.sh` into `recipe_seed`.** That is the
adoption-surface change the entry's other branch predicted, and the ruling above
declines it on the merits rather than on cost. Recording the cost anyway, because
it is what a later reader will want and it is bought:

`install-lifecycle.sh` performs three writes, not one — the agent-file block, a
marker-bounded block in `.gitattributes` (which it mints when absent, unlike the
agent file), and a **per-clone `git config merge.iteration-scoped.driver`**. It
ships no `--remove` mode, where `install-doctrine.sh` does.
`installer/lib/uninstall.sh` hardcodes both the doctrine-kit membership test and
the doctrine remover's payload path, and the consumer smoke asserts the
consumer's tree object equals the one it had **before `init` ran**
(installer/README.md §The consumer smoke). So seeding the block would owe: a
`--remove` mode on `install-lifecycle.sh`, a second branch in `uninstall.sh`, a
disposition for a `.gitattributes` file `init` minted, and an answer for a git
config write that lives outside the tree object the equality assertion measures —
where the assertion cannot see it, so the residue would be silent.

That is a unit, not a clause. It is **filed to the gap inbox at this amendment's
authoring**, with the cost above, so the declined branch is costed and filed
rather than flagged and skipped.

## Producers and consumers

This amendment introduces no state, event, message, or interface — it states a
rule over producers that already exist. The causal-completeness check is
discharged by naming them rather than by inventing a field roster:

- **Producer of the seeded skeleton** — `installer/lib/init.sh`'s per-kit loop,
  guarded by `recipe_needs_agent_file`. Enabling config: none; the predicate is
  reached on every non-dry `init` for every kit in the resolved profile.
- **Producer of the doctrine block** — `recipe_seed`'s `doctrine-kit` arm,
  invoking `doctrine-kit/bin/install-doctrine.sh` from the payload copy.
- **Consumer of the block** — `check-doctrine-registration`, registered by
  `recipe_gates` because it declares `zero-config`. This is the reader whose
  existence D1's rule turns on, and it is reachable on the tree `init` makes: the
  gate is in the registry `init` writes and the block is in the file `init` wrote.
- **Consumer of the rule itself (D1's prose)** — the author of the next kit that
  ships an agent-file installer, who otherwise resolves the question by reading
  two case statements and inferring. The rule has a named reader because that is
  the reader the filed entry describes: "each reader who notices re-derives the
  question and reaches the same undetermined answer."
- **Reverse path, named because it constrains any future yes** —
  `installer/lib/uninstall.sh`'s `TRIM_AGENT` branch, which fires only when the
  adopter has edited the agent file since install and the removed kit set includes
  doctrine-kit.

No new field is introduced, so the every-field-has-a-reader point is vacuous here
rather than skipped; D3 removes a claim rather than adding one.

## Existing sections updated

- `installer/README.md` §What init seeds — D1 adds the rule paragraph; D3
  corrects the starting-gate paragraph's lifecycle-kit sentence. Both land in the
  section that already describes this flow.
- `installer/lib/common/recipe.sh` — D2's two `spec:` directives. Directives, not
  prose: the section D1 writes owns the content.
- No lifecycle-kit surface changes. `lifecycle-kit/SPEC.md` §bin/install-lifecycle.sh
  already describes the installer as the adopter's own step and needs no edit, and
  `check-lifecycle-registration`'s `on-surface` disposition is *confirmed correct*
  by this ruling rather than revised by it — the gate arms when the adopter runs
  the installer, which is exactly what `on-surface` names.

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
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred). The declined seeding unit is already in the gap inbox, filed
      2026-08-09 at this amendment's authoring.
