# SPEC amendment: template-consumer-copy-parity

Adds `check-template-copy-parity`: a structural gate that separates *intended*
divergence between a kit template and its vendored consumer copy from *drift*.
No mechanism keeps the two in step today — `scripts/agent-budget-guard.sh` and
`delegation-kit/templates/agent-budget-guard.sh` were both hand-edited with
nothing verifying the edits matched.

## What changes

**Byte parity is ruled out, on measurement.** All eleven `*/templates/*.sh` ↔
`scripts/*.sh` pairs diverge, and for the eight `*-config.sh` pairs divergence
*is* the contract — a template config is a starting point the consumer
customizes, so equality would be the defect. The gate never compares bytes.

**Scope: executable pairs only, derived from layout.** The pairing is
`<kit>/templates/<name>.sh` ↔ `${GATE_SDK_GATES_DIR}/<name>.sh` — already
derivable from the vendoring layout, so no roster is maintained. `*-config.sh`
is excluded by name suffix, also derivable. Three pairs are in scope today
(`session-context.sh`, `bash-guard.sh`, `agent-budget-guard.sh`); the set grows
by layout, never by an edit to the gate.

**Containment in either direction is insufficient — this is why the unit earned
a spec pass.** The rotting direction the entry names is *consumer edited,
template not updated*, which ships a stale template to adopters. But the
consumer copies legitimately add content: `session-context.sh` diverges by 41
lines and `bash-guard.sh` by 22 (this repo's own steering rules). So:

- *copy ⊆ template* is false on purpose for every executable pair.
- *template ⊆ copy* catches only the reverse direction (template updated, copy
  stale), not the one that rots.

The gate therefore asserts over the **declared contract surface**, not content,
and makes the rotting direction decidable with an explicit exemption channel.

**The three assertions.**

- **A — same `spec:` target.** Both copies' `spec:` lines resolve to the same
  `<file> §<section>`. Compared on the resolved target, not the whole line: the
  budget-guard pair differs on line 2 *deliberately*, both naming
  `delegation-kit/SPEC.md §The delegation model` with different trailing prose.
  Comparing whole lines would red a sanctioned divergence on day one.
- **B — the template's declared surface is present in the copy.** Every function
  name and every `case`-arm exit token the template declares appears in the copy.
  This catches a template-side change never propagated.
- **C — the copy declares what it adds.** Any function or exit-arm token in the
  copy that is neither in the template nor covered by a declared divergence
  marker is drift. This is the direction that rots, made decidable by C's
  marker.

**The divergence marker.** The consumer copy carries
`# copy-divergence: <reason>` lines naming what it adds beyond the template and
why. The reason is required and non-empty — an empty marker is malformed, the
`[drain-exempt:]` precedent (lifecycle-kit's `check-stage-entry`) where the
reason *is* the audit trail. One-time cost: markers on `session-context.sh` and
`bash-guard.sh`, which is two files and is named here so build does not discover
it.

**Why this is the low-false-positive shape.** The gate reads *declarations*, not
prose or logic, so a consumer rewording a message, reordering rules, or adding
steering text triggers nothing. It fires only when a declared contract element
appears on one side unexplained. Should build find assertion C's false-positive
rate unacceptable in practice, the honest fallback — stated now so it is a
ruling and not a retreat — is to ship A and B and record C's direction as an
unmechanized limit in §check-template-copy-parity, the `check-gate-tamper`
precedent of a partial floor plus a stated limit.

**Gate contracts.** The gate copies `templates/check-skeleton.sh` and ships a
`good/`+`bad/` fixture pair, satisfying the four contracts (output, fail-closed,
fixture-pair, self-lint). It registers in `scripts/gates.list` and carries the
manifest:

```
# graph: couples=*/templates/*.sh,scripts/*.sh dir=bi valve=none tier=precommit
```

`dir=bi` because parity is symmetric — either side going stale is the defect,
which is precisely what assertions B and C split between them. Landing it
regenerates the pre-commit hook, the graph artifact, and the enforcement map per
CLAUDE.md.

## Producers and consumers

**New state: the `# copy-divergence: <reason>` marker.**

- *Producer* — the author of a consumer copy, at the moment they add something
  the template does not declare. It is not machine-written; it is source, like
  the `# graph:` and `# spec:` headers it sits beside.
- *Consumer* — `check-template-copy-parity` assertion C, at the transition where
  it diffs the copy's declared surface against the template's: a token covered by
  a marker is excluded from the drift set.
- *Field reader* — the marker's one field is `<reason>`, read by the human
  reviewing a parity failure and by `check-comment-tier`, under which it is a
  directive (it changes gate behavior) rather than a restatement, so it survives
  the comment-tier floor.

**New gate: `check-template-copy-parity`.**

- *Producer* — the pre-commit hook and `bin/run-gates.sh`, via registration in
  `scripts/gates.list`; its `# graph:` manifest is what puts it in the generated
  hook, so registration and manifest land together or the gate never runs.
- *Consumer* — the committing author, and CI's `gates` workflow on master.

**No new knob.** The pairing derives from `GATE_SDK_GATES_DIR` and the
`*/templates/` layout, both already resolved by `lib/gate.sh`.

**Unchanged surfaces verified.** The eight `*-config.sh` pairs are out of scope
by the suffix rule, so no config template needs a marker. `check-spec-pointer`
already skips templates (§Self-lint) and is unaffected by assertion A, which
compares targets rather than resolving them a second time.

## Existing sections updated

- **§Self-lint** already states that a copied-out template's `spec:` line
  resolves against the vendored kit path — the paragraph nearest this gate's
  concern. It gains a pointer to the new section so a reader arriving at the
  template↔copy relationship finds the parity contract.
- **A new §check-template-copy-parity** in gate-sdk/SPEC.md, beside the other
  gate contracts: scope, the three assertions, the marker grammar, and the
  ruled-out alternatives (byte parity, both containment directions) so no future
  close re-derives them.
- **`scripts/gates.list`** — registration.
- **`docs/check-graph.html` and `docs/enforcement.md`** — regenerated
  projections, both freshness-gated.

## Seam

Fully generic, and the seam is load-bearing here rather than incidental. The
pairing derives from layout; the gate reads only *declarations*, never the
content of what a consumer copy adds. That matters specifically for
`bash-guard.sh`, whose 22 divergent lines are this repo's own steering rules —
consumer rule content that must stay in the consumer copy and must never become
a kit literal. The gate asserts those lines are *declared*, never what they say.
The divergence reasons are consumer-authored text living in the consumer's own
file.

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
- [ ] **The attesting defect is caught** — the gate reds a one-sided edit to the
      `agent-budget-guard.sh` pair (the divergence that attested this unit) and
      stays green on the pair's sanctioned line-2 `spec:` difference. Both
      directions are proven by the fixture pair, not asserted.
