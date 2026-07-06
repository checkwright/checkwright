# SPEC amendment: tag-lead-line rename

## What changes

One gate renamed, no behavior change:
`queue-kit/checks/check-blocked-by-lead-line.sh` becomes
`queue-kit/checks/check-tag-lead-line.sh`. The name is stale, not merely
suboptimal: the gate's own contract line says it governs **every**
`blocked-by`/`spec`/`needs-spec` tag ("the only line the tag readers
scan"), yet the name claims only `blocked-by` — a leftover from before
the amendment-lifecycle tags joined the surface. A reader debugging a
`[spec:]` placement failure is pointed at a gate whose name says it has
nothing to do with `[spec:]`.

Renamed with it, mechanically:

- `scripts/gates.list` — the registry entry (registry-by-name means the
  rename is one line here; no consumer shadow files exist yet to chase).
- `queue-kit/gate-tests/check-blocked-by-lead-line/` — the fixture pair
  directory (`run-gate-tests.sh` resolves fixtures by gate name).
- `queue-kit/SPEC.md §check-blocked-by-lead-line` — section retitled
  `§check-tag-lead-line`; the gate header's `# spec:` pointer follows.
- The generated artifacts: `bash gate-sdk/bin/gen-pre-commit.sh --write`
  and the `check-graph --emit` refresh, since the hook and
  `CHECK-GRAPH.html` carry the gate name (`check-graph` asserts both are
  fresh — the rename cannot land without them).
- `queue-kit/README.md` and `queue-kit/smoke/install.sh` — listing and
  smoke references.

No knob changes; the gate keeps its `QUEUE_KIT_QUEUE_FILE` input and
output contract verbatim. New name on a governed surface:
`check-tag-lead-line` (feature litmus satisfied). The old name is
retired, not aliased — pre-1.0, with no external consumers, a
compatibility shim would outlive its one honest use.

## Producers and consumers

- **Producer:** the pre-commit battery and `run-gates.sh`, resolving the
  new name through `gates.list` exactly as before.
- **Consumers:** the committing agent (findings text carries the gate
  name); gate-sdk meta-gates (fixture-coverage resolves the renamed
  fixture dir by the new name); `check-graph` consumes the regenerated
  artifacts.
- **Fields:** none added or changed.

## Existing sections updated

- `queue-kit/SPEC.md` — section retitle plus a grep for the old name
  (removals propagated; nothing may dangle).
- `queue-kit/README.md`, `queue-kit/smoke/install.sh`,
  `scripts/gates.list` — reference updates.
- `scripts/git-hooks/pre-commit` and `.workflow/CHECK-GRAPH.html` —
  regenerated, never hand-edited.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
