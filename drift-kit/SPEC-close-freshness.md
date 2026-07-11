# SPEC amendment: trajectory-close-freshness

Closing an iteration leaves the committed trajectory projection stale, and
the freshness gate structurally cannot catch it at the moment it happens:
the extractor emits iteration N's row only once the close stamp is in
committed history, but during the enter-close commit's own pre-commit run
the stamp is not yet history, so the gate regenerates rowlessly and passes;
the following Done-clearing commit touches only the queue file, which the
gate's manifest does not fire on, so the staleness stays invisible until
CI's unconditional full battery. This amendment rules the fix's durable
home and the enforcement shape.

Home ruling: the kit-generic statement is SPEC prose in the extractor
section — any consumer running the extractor-plus-freshness-gate pair hits
the same self-reference — while the executable step is the *consumer's*
close binding. Ruled out: drift-kit's close-knowledge template (it stays
single-purpose — knowledge-friction triage), a new kit close-stage step and
lifecycle-kit's close skill (the lifecycle stays extractor-agnostic; not
every consumer publishes a trajectory).

Enforcement ruling: no new scanner — the self-reference is inherent, since
the row cannot exist before its own stamp commits. The local catch is
trigger widening: the consumer freshness gate's manifest names the queue
file in `trigger=`, so the Done-clearing commit mechanically runs the gate
with the stamp already in history, and a close that skipped the
regeneration step is red at that commit. CI's full battery stays the outer
backstop (gate-sdk/SPEC.md §Enforcement tiers).

## What changes

- **drift-kit/SPEC.md §The published-evidence extractor** gains the
  close-coupling contract in its consumer-wiring paragraph: (a) the
  structural statement above — a pre-commit freshness gate is blind at the
  enter-close commit by construction, stated as the honest limit; (b) the
  consumer contract — the close ritual regenerates the projection in the
  first commit after the close stamp lands (for a queue-clearing close,
  the Done-clearing commit, where the regenerated file and the cleared
  queue ride together); (c) the trigger rule — the consumer freshness
  gate's manifest names the queue file in `trigger=` (a trigger, not a
  coupled surface: the projection's content derives from the state files,
  not the queue, so `couples=` is unchanged and couples⊆trigger parity
  holds).
- **Consumer worklist (this repo).**
  `scripts/check-trajectory-fresh.sh`'s manifest becomes:

  ```
  # graph: couples=docs/evidence-data.md,.workflow/WORKFLOW-STATE.txt,.workflow/validate-evidence.txt,scripts/gates.list dir=one valve=none tier=precommit trigger=docs/evidence-data.md,.workflow/WORKFLOW-STATE.txt,.workflow/validate-evidence.txt,scripts/gates.list,TASK-QUEUE.md
  ```

  then the generated projections are refreshed in the same unit: the
  pre-commit hook (`gen-pre-commit.sh --write`), the coupling-graph
  artifact, and the enforcement map if the emitted rows change. The gate's
  body and fixture pair are untouched — only when it fires changes.
- **This repo's close binding** (`.claude/commands/close.md`, the
  housekeeping slot) gains the step: after clearing Done, regenerate the
  projection (`bash drift-kit/bin/trajectory.sh --emit >
  docs/evidence-data.md`) and commit it with the Done clear — the
  check-graph/enforcement-map regeneration precedent.

## Producers and consumers

- The closing iteration's row: produced by the close session's
  Done-clearing commit via the binding step (enabling config: the binding
  ships in this unit, so the producer path is live from the next close);
  consumed by the freshness gate's byte-compare at that same commit — now
  fired by the widened trigger, with the stamp in history — and by the
  published evidence page, which cites the data file.
- The trigger token: produced by the gate's manifest line; consumed by
  gen-pre-commit (hook emission — the gate now fires on queue-file
  commits) and by check-graph's parity assertion (couples⊆trigger holds
  because couples is unchanged).
- No new fields, files, or knobs; the projection path and extractor
  interface are unchanged.

## Existing sections updated

- drift-kit/SPEC.md §The published-evidence extractor — the consumer-wiring
  paragraph grows the close-coupling contract and the honest-limit
  sentence.
- .claude/commands/close.md — the housekeeping binding names the
  regeneration step (consumer surface, listed for causal completeness).

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
