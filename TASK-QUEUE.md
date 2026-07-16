# TASK-QUEUE.md — Checkwright work queue

## Iteration: —  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

- **env-probe-auto-refresh** — automate the env-profile refresh
  instead of the install-time + on-demand cadence. The current design
  (context-kit/SPEC.md §bin/env-probe, "cached projection, not per-session
  probe") rests on "env changes rarely" plus a per-session latency tax; both
  are weak — on a rolling-release environment the toolchain versions change
  daily-plus (a `python3` probe-set drop was caught stale on one refresh) and
  the probe measures in tens of ms, so the latency objection does not hold.
  Design on record: call env-probe from the **session-context hook** (once per
  session — the meaningful granularity; NOT the statusline, which re-renders
  many times per session for a box that does not change mid-session; NOT
  enter-stage.sh, which under-covers non-lifecycle sessions and couples
  lifecycle-kit → context-kit). Optional refinement: give env-probe
  **change-detection** so the block and its `Probed` date rewrite only on an
  actual content change (keeps the date a real last-changed signal, avoids
  per-session file churn). Surfaces: context-kit/SPEC.md §bin/env-probe (rewrite
  the cadence / no-freshness-gate paragraph — the session-context hook is
  already its step-9 consumer, so producer and consumer co-locate),
  scripts/session-context.sh + context-kit/templates/session-context.sh (the
  call), possibly context-kit/bin/env-probe.sh (change-detection); the docs
  mirror regenerates after the SPEC edit and check-install-toolchain parity is
  unaffected (probe set unchanged). Surfaced 2026-07-16 by the operator
  questioning the install+on-demand cadence and a probe-runtime measurement.

## Deferred

- **prose-tell-abbr-append** [needs-spec] — `CANON_KIT_PROSE_TELL_ABBR_ALLOW`
  (and the sibling `CANON_KIT_PROSE_TELL_PHRASES`) are replace-not-append: a
  consumer adding one token must copy the kit's entire bundled universal set
  verbatim into its config, because assigning the array overwrites the default
  rather than unioning with it. This repo's `scripts/canon-config.sh` already
  carries the full 12-token kit default reproduced solely to append four local
  tokens — a literal duplication that silently staleness-diverges if the kit's
  bundled set ever changes, since no gate couples the copied prefix to the kit
  default. Fix candidates: (a) union semantics — the kit seeds the default and
  the consumer array *extends* it (an append/`_EXTRA` convention, or lib merges
  default+consumer); (b) a freshness gate coupling any consumer copy of the
  bundled prefix to the kit default so divergence reds. Gap generalization: the
  missing check class is consumer-config-restates-kit-default (a value the kit
  owns, copied into consumer config with no coupling gate) — (a) removes the
  duplication outright (enforcement-first: eliminate over gate), (b) gates it if
  a copy must remain. Generic mechanism only: the token vocabulary stays
  consumer config either way (the provenance seam). Surfaced 2026-07-16
  dogfooding check-prose-tells' abbreviation valve during launch-readiness.
- **drain-stage-active-residue** [needs-spec] — check-stage-entry assertion B
  (drain-entry queue-empty) has no model for active-section entries that
  legitimately persist into validate. The drain model assumes the active queue
  empties before validate, yet two active items can both legitimately remain: a
  validate-spanning feature — launch-readiness-gate, the first to hit this,
  whose build half ships and whose validate half is the four readiness checks,
  so Done is false and Deferred is wrong while it is being validated — and a
  designed-debt item that cannot move to the Deferred section without a false
  needs-spec tag (env-probe-auto-refresh, per check-amendment-queue rule (b),
  which requires every Deferred entry to carry needs-spec, and which the item's
  designed-debt nature makes false; it is correctly in Technical Debt). Both
  legitimately block the drain assertion, so entering validate needs the
  enter-stage.sh by-hand override — the gate's own designed escape for a case
  assertion B admits it cannot model. The gap's cost is threefold: (1) it blocks
  validate entry without the by-hand override; (2) the override then leaves the
  full battery red for the stage's duration (the validate-baseline holds the
  suite constant-red); and (3) that suite-level baseline granularity masks any
  *new* intra-battery gate regression introduced during validate — a per-gate
  baseline would catch a fresh red, the whole-suite one cannot. Fix candidates:
  per-iteration active-section scoping, a spanning/standing-residue exemption
  tag, or a valve on assertion B (cost 3 additionally wants per-gate baseline
  granularity so an intra-validate regression is not masked). Surfaced
  2026-07-16 entering validate for launch-readiness; the third cost surfaced in
  close's knowledge-friction triage.
- **rendered-site-link-monitor** [needs-spec] — durable coverage for the
  reader-facing link liveness of the rendered checkwright.dev site. Internal
  and external link rot recurs, and the tree-side reference gates
  (check-md-refs, check-docs-nav-reachable, check-docs-render-fidelity) plus
  the site-health.yml deployment probe cover render and deployment truth but
  not the rendered-site external-URL crawl a reader actually hits. A hermetic
  gate is ruled out on record: site-kit/SPEC.md §The monitor boundary —
  external-link liveness reds on causes no commit produced (DNS, a moved
  target, an incident), breaking the low-false-positive contract. So the
  durable form is a **monitor**, a scheduled crawl step extending site-kit's
  site-health.yml, signalling through an issue and its own red run, never a
  blocked merge. Demand-gated like the other rungs: promote when the one-time
  launch crawl (launch-readiness-gate validate) shows recurrence worth
  automating. Surfaced 2026-07-16 in the launch triage that scoped
  launch-readiness-gate.
- **plugin-marketplace** [needs-spec] — harness plugin/marketplace packaging
  of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped
  surface parity. Design against the live manifest format at promotion — the
  plugin substrate moves fast (the scope-session-routing ruling applies).
  Surfaced 2026-07-09 in adoption-track's split; evidence artifact retained:
  upstream Claude Code issue #75214 (project config can't lift the Task
  ask-first default), surfaced dogfooding the delegation nudge 2026-07-07.
- **benchmark-ab-experiment** [needs-spec] — the controlled differential
  experiment: same model, same dependent-task series, two arms (ungoverned
  loop vs Checkwright-governed), drift *accumulation across the series* as
  the metric — a governance layer's effect, not a model leaderboard number.
  Metric axis: Drift-Bench's "satisfiable drift". Substrate/vocab primaries:
  seqBench (arXiv 2509.16866), Drift-Bench (arXiv 2602.02455 — real title
  "Diagnosing Cooperative Breakdowns in LLM Agents under Input Faults via
  Multi-Turn Interaction"; the "Decomposing Reasoning Into Failure Types"
  expansion is confabulated, do not repeat it), Lost-in-Conversation /
  FlowBench as prior art. Surfaced 2026-07-08 inside adoption-track; split
  out 2026-07-09 — the self-referential route (drift-trajectory) ships
  first and this rung upgrades the claim only if demand attests it.
  Includes the experiment's measurement half: a **stage-burn meter** landing
  in drift-kit's bin/ on the overhead-meter pattern (sessions-dir resolution,
  config via env, advisory exit-0) — per-stage, per-model token burn read off
  harness transcripts and price-weighted, replacing the local-only prototype
  scripts parked in `.metric/`. Nearer use: verifying the split-lead posture's
  savings (lifecycle-kit/templates/lead.md §Economics). Surfaced 2026-07-15
  by the per-stage budget analysis that motivated that posture.
- **prose-profile** [needs-spec] — the non-code universality rung: a third
  consumer shaped as a prose/documentation repo (no build, no test suite)
  stress-tests whether the kits govern non-code work. Core dilution is ruled
  out on record — if pursued, this is an adapter/profile delivered as
  optional consumer config, never a kit literal (the provenance seam).
  Demand-gated: it attests only when a non-code consumer actually vendors a
  kit and hits friction; until then this entry is the roadmap marker. Seeds:
  gate-sdk, guard-kit, context-kit, drift-kit, and canon-kit's
  one-owner/coupling core are workflow-agnostic today; lifecycle-kit's stage
  semantics, evidence-kit's test baseline, and canon-kit's spec framing are
  software-coupled — the abstraction axis is "code + spec" artifacts
  generalizing to "governed surface". `check-prose-tells` (the
  launch-readiness-gate build) is the first concretely prose-shaped kit
  mechanism and the natural profile seed. Surfaced 2026-07-16 in the same
  launch triage that scoped launch-readiness-gate.
- **hosted-attestation-service** [needs-spec] — the team/paid rung: gates
  verified server-side by a party the committing agents cannot touch —
  hosted gate runs as a neutral attestation, cross-repo drift dashboards,
  maintained rulesets. A service, not code: cloning the kits does not clone
  the neutrality or the ops. Demand-gated — this entry is the public
  roadmap marker, not a scaffold; hosting and sequencing decisions are on
  record in the operator's local brief, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.
## Done

## Lessons Learned
