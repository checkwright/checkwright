# SPEC amendment: deprecation-lifecycle

## What changes

Deprecation with teeth, for consumers — the governance coupling no linter
ships. The scan itself stays consumer toolchain (clippy/ESLint-class linters
already inventory deprecation markers, and drift-kit's Out of scope
pre-rules toolchain-shaped scans as consumer plugins): the marker roster is
consumer config, a consumer's language never a kit literal. Three pieces:

**1. spec-kit gate — `checks/check-deprecation-task.sh`** (`precommit`):
every deprecation marker on a governed source resolves to a live
decommission task — the TODO(task:) analog, sharing
check-todo-task-liveness's binding grammar and queue resolution
(one grammar, two gates: the marker line must carry `task: <slug>` and the
slug must resolve to a live queue entry, active or deferred). The live/done
queue pass is gate-private awk in check-todo-task-liveness today (only its
section regexes are lib adapters); this unit lifts that pass into a
`lib/spec.sh` adapter both gates source, unchanged in behavior. Detector
roster: `SPEC_KIT_DEPRECATION_MARKERS` (one regex per element, default
empty → clean skip — the graph-vocab pattern; the kit never knows a
language's marker spelling). A detected marker with no `task:` binding, or a
dead slug, is red. Fail-closed on an unreadable source or queue file.

**2. lifecycle-kit skill template — `templates/skills/release-sweep.md`**:
the release-boundary disposition walk. At a major, walk the marker
inventory and force a per-entry disposition — decommission now, re-justify
and carry the task forward, or un-deprecate — one stamped line each (the
lesson-disposition contract shape at a release boundary). Consumer-copied;
the stamp file is operator evidence riding the release commit — the kit
wires no gate over it (a consumer may). This repo's own release-prep sweep
stays with the upgrade-path rung, as ruled there.

**3. drift-kit example plugin — `templates/kpi-deprecated-surface.sh`** on
the KPI contract: live-marker count over the same roster, so the backlog
trends between majors instead of surprising at one. A template the consumer
registers in its kpis.list, not a bundled KPI — toolchain-shaped, per
drift-kit's standing ruling.

Demand honesty: the mechanism is attested practice on a private consumer of
this lifecycle (the anticipated first consumer and the prior-art source).
This repo sets no marker roster, so the gate clean-skips here; the fixture
pair plus `check-deprecation-task.test.sh` carry the covered paths (the
check-manifest-count config-path precedent).

## Producers and consumers

- Gate producer: the generated pre-commit hook / `run-gates.sh`
  (`# graph:` couples governed sources, the queue file, and the roster's
  config home). Consumer: the committing operator; each finding names file,
  line, marker, and the unresolved slug — every field read in that message.
- `SPEC_KIT_DEPRECATION_MARKERS` read at gate startup; the captured slug
  read at the queue-resolution transition (the adapter lifted above).
- Skill template producer: the consumer copies it beside its stage skills
  and invokes it at a release boundary; consumer of the stamps: the release
  reviewer/operator (stated above — no kit reader).
- KPI template producer: the consumer's kpis.list registration; consumer:
  drift-report's plugin walk, value read in the report line.

## Existing sections updated

- spec-kit §check-todo-task-liveness: cites the shared binding grammar and
  the lifted resolution adapter now used by two gates.
- spec-kit §lib/spec.sh: gains the queue-resolution adapter with its two
  readers.
- lifecycle-kit SPEC §templates: the new skill-template row.
- drift-kit SPEC §Out of scope + §Layout: the pre-ruling now points at the
  shipped example template.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
