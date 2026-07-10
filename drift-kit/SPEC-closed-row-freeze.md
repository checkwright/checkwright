# SPEC amendment: closed-row-freeze

A closed iteration's published-evidence row must be immutable, but
`trajectory.sh` anchors only the gate-count column at the close commit:
the feat/debt split and amendment latency run to HEAD for the *last*
closed iteration, so any post-close `feat`/`fix` commit mutates a
published row and reds `check-trajectory-fresh` until a regen (queue:
`trajectory-closed-row-freeze`).

## What changes

**Every range-scoped column freezes at the close boundary.** Iteration
N's harvest range is `(close(N-1), close(N)]` — the commits after the
previous iteration's close commit up to and including its own. The
gate-count column keeps its close-commit sample; the commit-shape
(feat/debt) classification and the amendment-latency harvest now run over
the same range instead of to HEAD. No column reads HEAD, so the emission
is a pure function of the *closed* history: re-emission is byte-identical
until a new close commit lands, whatever is committed in between.

Consequences ruled here:

- **Interstitial commits are owned, not orphaned.** The design note asked
  who owns post-close, pre-next-scope commits; under this range rule they
  fall into the *next* iteration's range and appear when it closes.
  Nothing is unowned, totals conserve across rows, and the cross-boundary
  under-count edge dissolves.
- **The post-close append property is preserved by construction and
  strengthened.** The operator constraint — filing a queue entry after
  close must never disturb a published row — holds today only because the
  feat/debt harvest ignores `docs(queue):` subjects; under the freeze even
  a post-close `feat`/`fix` hotfix leaves every published row
  byte-identical (it waits for the next close). `TASK-QUEUE.md` stays
  append-friendly with no special-casing.
- **Amendment attribution**: an amendment belongs to the iteration whose
  range contains its *delete* (merge) commit; its add-date may precede the
  range start — latency measures commitment-to-merge wherever the
  commitment was made, unchanged.
- **First row**: `close(0)` is the empty boundary — iteration 1's range is
  everything up to its close commit, matching current behavior.

One-time consumer effect: the committed projection `docs/evidence-data.md`
is regenerated at build (the last row's range-scoped cells may shift once,
stated in the build commit); `check-trajectory-fresh` needs no change —
it byte-compares whatever the extractor emits. No new knob; no interface
change to `--emit`.

## Producers and consumers

- Producer: `trajectory.sh` computing per-row ranges from the ordered
  close-stamp commits it already harvests from `WORKFLOW-STATE.txt`
  history — no new input surface.
- Consumers unchanged: the committed `docs/evidence-data.md` projection,
  `scripts/check-trajectory-fresh.sh` (byte-compare), and the
  `docs/evidence.md` framing page, which hand-copies no numbers.

## Existing sections updated

- §The published-evidence extractor: the harvest bullets for commit shape
  and amendment latency state the `(close(N-1), close(N)]` range; the
  purity paragraph tightens from "pure function of committed git history"
  to "pure function of closed history — byte-stable across any commit
  that is not a close".

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Freeze verified** — after regen, a synthetic post-close `feat` commit
      leaves `trajectory.sh --emit` byte-identical (the property the fixture
      pair pins where hermetically craftable).
