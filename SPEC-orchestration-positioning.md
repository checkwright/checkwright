# SPEC amendment: orchestration-positioning

A docs-site ruling with no owning kit (the `docs/` surface is
repo-root-governed), so this amendment lives at the repo root — the
SPEC-docs-cname-parity precedent; the sibling page's amendment is
SPEC-ddd-positioning.md.

## What changes

- `docs/orchestration.md` — the second positioning page: Checkwright as
  **the verification layer under agent orchestration**. The phrasing is
  deliberate and this page is where it lands publicly (an unqualified
  "trust layer" reads as zero-trust security, a different market);
  launch-comms may still revisit the wording. The argument the page owns:
  coordination primitives answer *who works on what, when* — the gates,
  stamps, and tamper battery answer *whether the work is right* without a
  supervisor reading all of it, which makes verification the prerequisite
  the orchestration story quietly assumes.
- Grounded in mechanisms that exist today, cited downward by name:
  delegation-kit's validate-after-agent-commit rule and `check-gate-tamper`
  (a delegated agent cannot silently weaken the gates that judge it), the
  per-dispatch budget guard, lifecycle-kit's stage stamps in
  `WORKFLOW-STATE` (evidence a stateless supervisor can audit), and
  evidence-kit's committed per-run manifest. The page claims the
  **trust/verification layer, never the orchestration layer**.
- The honesty clause is structural, not a disclaimer paragraph: the
  coordination rungs themselves are deferred and the page cites them *as
  the roadmap* — `scope-session-routing` (session-to-session routing, the
  next iteration's unit) and `multi-operator-semantics` (the contributor
  altitude) by their public queue entries — facilitator today,
  prerequisite for unattended orchestration at scale, and nothing on the
  page outruns what is built.
- Relation to the harvested essay: the page and the essay share the
  **argument, not the text** — the ruling carried by the
  orchestration-trust-framing entry in `.workflow/essay-harvest.md` (the
  gitignored essay sink the close binding's `[essay]` disposition feeds;
  local-only operator material, so the build session reads it on the
  operator's clone) — the page is written fresh against the mechanisms,
  never pasted from the essay sink.
- `docs/index.md` — adds the orchestration row to the `## Positioning`
  section SPEC-ddd-positioning.md creates; if this page lands first, it
  creates the section instead (the section is the pair's shared home —
  whichever page lands first brings it). Governance is automatic via the
  existing `docs/*.md` manifest glob.

## Producers and consumers

- Producer: the build session authors the page; GitHub Pages serves it
  from `docs/` on master (the existing site mechanism — no new publishing
  config).
- Consumers: readers arriving from the index's Positioning section —
  specifically the evaluator asking "does this replace or complement my
  orchestration setup?", who must leave with: complement, beneath it. The
  doc gates consume it as every living page; the cited queue entries
  resolve against `TASK-QUEUE.md`, whose deferred section is public by
  design.
- No new fields, states, or config: the page cites existing mechanisms
  and queue entries by name and adds none.

## Existing sections updated

- `docs/index.md` — the Positioning section gains its second row (or the
  section itself, per the landing-order rule above).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md` at the repo root).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
