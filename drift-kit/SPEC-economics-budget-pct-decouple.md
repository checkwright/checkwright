# SPEC amendment: economics-budget-pct-decouple

<!-- Owning component: drift-kit (owns the `/economics` skill and its chain).
     Root-level placement is wrong here — this is a drift-kit contract change,
     not a governance ruling. Pairs with the queue entry tagged
     [spec: SPEC-economics-budget-pct-decouple.md]. -->

## What changes

The `/economics` post-iteration narrative drops its third cost surface,
delegation-kit's `usage-trend` (the subscription **budget-%** rate-window
footprint). The cost chain narrows from three tools to two —
`overhead-meter` → `stage-economics` — and `stage-economics` becomes the **sole
cost-attribution surface** the narrative prices from.

The rationale (merges into drift-kit/SPEC.md §The `/economics` skill as prose):
budget-% is **account-wide** — confounded by overlapping sessions and by a
second operator on the same account — so it is the wrong instrument for
**per-iteration cost attribution**. `stage-economics` already prices
per-transcript, per-stage, per-model token draw (the token SSOT, immune to that
confound), so carrying budget-% beside it put a confounded advisory number next
to a clean one, which a reader could over-trust as this iteration's cost.

**Scope is removal of a *role*, not a tool.** Every delegation-kit budget
surface stands unchanged — `usage-verdict`, `agent-budget-guard`, and
`usage-trend` itself. Budget-% remains *correct* for its real job: the
pre-dispatch safety throttle against a shared-account cap, where cross-session /
cross-operator overlap is the feature, not the bug (and `usage-trend` also still
serves delegation planning — weekly headroom). Only its role as an economics
*cost* surface is removed. `bin/usage-trend.sh` is not touched; delegation-kit's
§Trend reporter contract is unchanged.

## Producers and consumers

This amendment **removes a consumer edge**, adds no new state, event, field, or
interface — so the causal-completeness obligation is that the removal is
propagated and no surviving reader dangles.

- **The removed edge.** Producer `delegation-kit/bin/usage-trend.sh`
  (rate-window budget-% footprint); former consumer the `/economics` narrative
  (`templates/economics.md` step 3, materialized as `.claude/commands/economics.md`),
  which called it read-only and folded its output into the cost story. After
  this change the narrative no longer calls it.
- **The surviving chain.** Producers `bin/overhead-meter.sh` (governance-vs-task
  byte proxy) and `bin/stage-economics.sh` (priced stage × model × iteration
  token draw); consumer the same `/economics` narrative, which now composes its
  cost story from those two alone. `stage-economics` is the sole cost surface;
  overhead-meter contributes the governance-share, not a cost figure.
- **The producer's other consumers are unaffected** — surveyed across the whole
  component set (`git grep usage-trend`): `usage-verdict` /
  `agent-budget-guard` (the pre-dispatch throttle), delegation planning
  (weekly headroom), delegation-kit/SPEC.md §Trend reporter and its README/test
  surfaces, and `scripts/delegation-config.sh`'s sampling wiring. None reads the
  `/economics` chain; the tool keeps every reader it had except the one this
  amendment retires.

## Existing sections updated

Each lands at merge in the drift-kit/SPEC.md §The `/economics` skill section (and
its generated docs mirror):

- **`drift-kit/templates/economics.md`** — delete step 3 ("Usage trend") from
  the numbered chain and reword the intro ("Chain the three reporting tools" →
  two); the composed-narrative bullets (Cost by stage, Overhead share, Posture
  verdict) already carry no dedicated usage-trend bullet, so no narrative bullet
  is removed. The template carries a `CONSUMER BINDING` header, so it
  self-excludes from the canon manifest gates (§Layout and configuration,
  `CANON_KIT_PROSE_SURFACE_GLOBS` slot-free rule) — no manifest-count concern on
  the reworded cardinal.
- **`drift-kit/SPEC.md` §The `/economics` skill** — the chain
  `overhead-meter → stage-economics → usage-trend` becomes the two-tool
  `overhead-meter → stage-economics`; drop the `usage-trend` read-only-caller
  clause and the `delegation-kit/SPEC.md §Trend reporter` cross-reference in that
  paragraph.
- **`drift-kit/README.md`** (the `bin/stage-economics.sh` one-liner) — reword the
  chain from "chains overhead-meter → stage-economics → delegation-kit's
  usage-trend into one post-iteration cost narrative" to the two-tool chain.
- **`docs/drift-kit/README.md` and `docs/drift-kit/SPEC.md`** — generated mirror
  projections of the two edited surfaces above; regenerate with
  `bash scripts/gen-docs-mirror.sh --write` (`check-docs-mirror-fresh`
  byte-gates their freshness).

**Reviewed and deliberately retained** — `drift-kit/SPEC.md` §The
stage-economics meter names `usage-trend` as a *measurement contrast* ("the
overhead meter measures … bytes and delegation-kit's usage-trend the rate-window
percentage; neither converts a stage's token draw into money"). That sentence
states what each sibling *measures*, not chain membership; it stays true and is
not edited. Scope ruled the edit surface with the operator 2026-07-19 without
listing it.

## Definition of Done

- [ ] **Causal completeness** — removal-only; the retired consumer edge is
      propagated to every surface above and the producer's surviving readers are
      surveyed whole-tree and left intact.
- [ ] **Merged with no information lost** — the two-tool chain and its rationale
      integrated into drift-kit/SPEC.md §The `/economics` skill; the template,
      README, and docs mirror read coherently for a reader who never saw this
      amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      drift-kit (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `git grep usage-trend` confirms no surface still
      names it as an `/economics` cost surface; the tool's other citations remain.
- [ ] **Gaps filed** — any cross-component gap found during the work filed as a
      debt/deferred task.
