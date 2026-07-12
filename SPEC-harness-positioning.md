# SPEC amendment: harness-layer-positioning

A docs-site ruling with no owning kit, so this amendment lives at the repo
root — the SPEC-orchestration-positioning precedent. It creates the
positioning page; two debt entries (competitive-positioning,
model-effort-guidance) land later sections on the page this unit creates
and are queue-tagged blocked on it.

## What changes

- `docs/positioning.md` — a living positioning page, "Where Checkwright
  sits": front matter `title: Where Checkwright sits`, the top-level nav
  slot `nav_order: 5` (the layout's Where-Checkwright-sits position). Three
  sections this unit owns:

- **The layer model.** Checkwright is layer-4 content: harness-loaded
  instructions (CLAUDE.md, skills) plus gates that run *outside* the model
  entirely — it shapes and audits the behavior the lower layers produce and
  is subordinate to a closed layer-3 harness prompt. The worked ceiling the
  page cites: upstream Claude Code issue #75214, where project-level config
  cannot lift the harness's Task ask-first default — an instruction no
  layer-4 artifact can override, which is exactly why the enforcement half
  of the methodology lives in gates a harness cannot re-interpret. The page
  keeps the "verification layer under agent orchestration" framing that
  `docs/orchestration.md` established; it claims no security or trust-layer
  vocabulary.
- **The tiered compatibility claim** — honest tiers, not blanket compat.
  Tier one: the gate battery (gate-sdk and every check it registers) is
  bare bash over a coreutils toolchain and runs under any harness, any CI,
  or no harness at all — nothing in a gate reads a harness surface. Tier
  two: the lifecycle stage skills, the CLAUDE.md load convention, and the
  settings pins are Claude-Code-native; for other rules-file conventions
  (AGENTS.md, .cursorrules and kin) they are adapter-shaped — the page
  names them as an adaptation surface and does not claim tested
  compatibility that does not exist.
- **The memory-off position, public form.** Durable guidance lives in
  tracked manifests the whole team and every session can read, never in
  per-user harness memory that forks silently per operator — generalized
  across harness memory conventions rather than naming one product's
  feature. Cited downward to context-kit/SPEC.md §The memory-off doctrine
  and the `check-memory-off` gate; the page positions, the SPEC owns the
  invariant.

- `docs/index.md` §Positioning gains the page's row beside the DDD and
  orchestration rows.

## Producers and consumers

- Producer: the build session authors the page; GitHub Pages serves it from
  `docs/` on master (the existing site mechanism — no new publishing
  config).
- Consumers: evaluating adopters arriving from the nav or
  `docs/index.md`'s Positioning section; the doc gates (link/command
  resolution, temporal narration, render fidelity once
  SPEC-render-fidelity.md lands) consume it as they do every living page;
  the two blocked debt entries consume the page as their landing surface.
- No new fields, states, or config: the page cites existing gates and
  doctrine by name and adds none.

## Existing sections updated

- `docs/index.md` — the Positioning section gains one row; nothing else
  moves (the kit registry's home is `docs/kits.md`, CLAUDE.md §Housekeeping).

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
