# SPEC amendment: template-consumer-copy-parity

Adds `check-template-copy-parity`: a structural gate that separates *intended*
divergence between a kit template and its vendored consumer copy from *drift*.
No mechanism keeps the two in step today — `scripts/agent-budget-guard.sh` and
`delegation-kit/templates/agent-budget-guard.sh` were both hand-edited with
nothing verifying the edits matched.

**That attesting divergence is already repaired, and the gate must not be built
expecting to find it live.** The align audit diffed the pair: it now differs on
**line 2 only**, the sanctioned `spec:` prose difference assertion A is
explicitly designed to tolerate. `verdict-reader-honesty`'s close removed the
`2)` arm from both copies. So the tree is green on this pair by construction,
and the Definition of Done's "reds a one-sided edit" is proven by a **fixture
that synthesizes** the one-sided edit, never by capturing a live one. The
motivating defect is history; the risk it proved — two hand-edited copies with
no verifier — is present and unmechanized, which is what this unit closes.

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

**An unpaired template is silently out of scope, not a failure.** A template
with no same-named file under the gates dir has not been vendored here and has
no copy to be in parity with. Six are unpaired today (`statusline-usage.sh`,
`usage-poller.sh`, `kpi-deprecated-surface.sh`, `check-skeleton.sh`,
`escalation-guard.sh`, `wakeup-guard.sh`), and two of them are wired live *from
the template path itself* — `.claude/settings.json:70,104` runs
`delegation-kit/templates/usage-poller.sh` and `statusline-usage.sh` without
copying them out. Running a template in place is a legitimate adoption mode, so
the gate must skip it rather than fail closed on six files; stated because a
fail-closed default would red the tree on the gate's first run.

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

  **A reds on two of the three pairs today — a backfill, not a gate defect.**
  The align audit ran A by hand, which was its first real run, and found the
  drift A exists to catch already present:

  | pair | template target | copy target |
  |---|---|---|
  | `agent-budget-guard.sh` | `delegation-kit/SPEC.md §The delegation model` | same — **green** |
  | `session-context.sh` | `context-kit/SPEC.md §The session-context hook` | `… §The session-context hook (template)` |
  | `bash-guard.sh` | `guard-kit/SPEC.md §Consumer rules` | `guard-kit/SPEC.md`, no section |

  These survived because `check-spec-pointer` skips templates by design
  (§Self-lint; canon-kit/SPEC.md §check-spec-pointer), so a template's target has
  never been resolved by anything — A is the first mechanism to compare it
  against a copy that *is* resolved. A therefore inherits a useful second
  property: it surfaces a dangling template pointer that the pointer gate cannot
  see. The session-context arm is determinate — the real heading is
  `## The session-context hook (template)` (context-kit/SPEC.md:71), so the
  *template* carries the wrong target and is corrected to match its copy. The
  bash-guard arm is not determinate and is escalated rather than guessed:
  `guard-kit/SPEC.md` has no `§Consumer rules` section at all, so both sides are
  wrong and the correct target does not yet exist.

  This backfill is three files (two templates, one copy) and is **additional to**
  the two marker files costed below; the amendment as authored costed only the
  markers.
- **B — the template's declared surface is present in the copy.** Every function
  name and every `case`-arm exit token the template declares appears in the copy.
  This catches a template-side change never propagated.
- **C — the copy declares what it adds.** Any function or exit-arm token in the
  copy that is neither in the template nor covered by a declared divergence
  marker is drift. This is the direction that rots, made decidable by C's
  marker.

**Measured: the declared surface B and C read is nearly empty — open, escalated.**
The align audit counted the surface rather than assuming it, and the result
undercuts B as specified:

| pair | funcs (tpl/copy) | `case` (tpl/copy) | lib calls (tpl/copy) | divergence |
|---|---|---|---|---|
| `session-context.sh` | 0 / 0 | 1 / 1 | — | 21 added, 20 **removed** |
| `bash-guard.sh` | 0 / 0 | **0** / 1 | 4 / 7 | 19 added, 3 removed |
| `agent-budget-guard.sh` | 0 / 0 | 1 / 1 | 3 / 3 | line 2 only |

**Zero function declarations exist in any of the six files** — these are thin
hook scripts that source a kit lib and delegate, so B's "every function name"
clause is inert on every pair today and structurally likely to stay so. And the
`bash-guard` template declares **no `case` block at all**, so B's surface for
that pair is empty in both clauses: the template declares nothing the copy could
fail to contain. B meaningfully asserts on one pair, via `case` arms alone.

The pre-recorded fallback below anticipates the wrong failure — it retreats from
C on noise, while the measured weakness is B on vacuity. Note also that
`session-context` **removes** 20 template lines rather than only adding: that is
B's direction, the one arm currently unable to see it.

This is a change to what the gate asserts, so it is escalated rather than
settled here. The audit's recommendation is to widen the declared surface from
{function names, `case`-arm exit tokens} to additionally include **sourced-lib
API calls** (`guard_block`, `guard_advise`, `guard_read_command`) and the
`<KIT>_*` knob names each side reads. Both are declarations rather than content,
so the seam below is untouched, and the table shows both are non-empty on every
pair — 4/7 and 3/3 lib calls where `case` arms give 0/1.

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
