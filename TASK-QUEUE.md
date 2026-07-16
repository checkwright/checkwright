# TASK-QUEUE.md — Checkwright work queue

## Iteration: env-probe-refresh  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **env-probe-auto-refresh** [spec: SPEC-env-probe-refresh.md] — automate the
  env-profile refresh: the session-context hook re-probes once per session (at
  its step-9 profile emit) in place of the install-plus-on-demand cadence, and
  `bin/env-probe.sh` gains change-detection so the block and its `Probed` date
  rewrite only on an actual content change — the date stays a real last-changed
  signal and an unchanged box writes nothing. Single-component (context-kit):
  both the producer (env-probe) and the step-9 consumer are context-kit's own,
  so no cross-kit coupling. The hook re-probes only when the profile file
  already exists, so the silent-when-absent / no-cost-where-unseeded contract
  holds.
  Full design (producers/consumers, the Cadence-paragraph rewrite,
  change-detection semantics, DoD) is on record in
  context-kit/SPEC-env-probe-refresh.md. Surfaced 2026-07-16 by the operator
  questioning the install+on-demand cadence and a probe-runtime measurement.

## Technical Debt

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
  needs-spec tag. Both legitimately block the drain assertion, so entering
  validate needs the enter-stage.sh by-hand override — the gate's own designed
  escape for a case assertion B admits it cannot model. The gap's cost is
  threefold: (1) it blocks validate entry without the by-hand override; (2) the
  override then leaves the full battery red for the stage's duration (the
  validate-baseline holds the suite constant-red); and (3) that suite-level
  baseline granularity masks any *new* intra-battery gate regression introduced
  during validate — a per-gate baseline would catch a fresh red, the whole-suite
  one cannot. Fix candidates: per-iteration active-section scoping, a
  spanning/standing-residue exemption tag, or a valve on assertion B (cost 3
  additionally wants per-gate baseline granularity so an intra-validate
  regression is not masked).
  Reframe on record (operator ruling, same /lead session as lead-seam-redesign):
  Feature-versus-Debt triage is scope work — a task surfaced mid-iteration files
  as Deferred [needs-spec] and a later scope promotes it to New Features
  (spec-ready) or Technical Debt, never plugged straight into Debt late in an
  iteration. Under that rule case 2's designed-debt item was partly a mis-filing
  (the env-probe example that seeded it should have been Deferred [needs-spec]
  then scope-promoted, as it since was), not purely a gate-model gap, so this
  iteration must re-decide whether case 2 remains a real assertion-B gap. Coupled
  gate (enforcement-first, land fix and gate together): a check-amendment-queue
  rule that a spec-ready entry (one carrying a spec tag) belongs in New Features.
  Surfaced
  2026-07-16 entering validate for launch-readiness; the third cost surfaced in
  close's knowledge-friction triage.
- **lead-seam-redesign** [needs-spec] — the iteration lead's stage-transition
  seam over-reads prior-stage completeness by hand instead of trusting the
  machinery. Four coupled parts, surfaced in a /lead design session and none in
  a governed surface before this entry: (1) an enter-stage.sh `--simulate`
  (read-only preflight) — the script already separates its preflight
  (check-stage-entry + LIFECYCLE_KIT_ENTRY_PREFLIGHT + the lessons check, run
  against a temp flipped queue) from its write, so a flag that runs the preflight
  and skips the write hands the lead the exact gate the dispatched stage will
  run, read-only, in place of re-deriving completeness from WORKFLOW-STATE and
  git log (oracle-first made concrete). (2) A lead-template ruling: the lead
  stops hand-deriving prior-stage completeness — it dispatches and trusts
  enter-stage's fail-closed refusal plus the agent's report, or runs `--simulate`
  to gate a dispatch cheaply; lead-does-stamping is ruled out (it breaks the
  "Stamps are authoritative" invariant and check-stage-evidence under the strict
  LIFECYCLE_KIT_SESSION_BOUNDARY, where a lead stamp is the self-reported skip
  that gate catches). (3) A session-context role signal:
  scripts/session-context.sh and context-kit/templates/session-context.sh key
  every injection off the queue's `[stage:]` field with no signal for whether the
  session is a lead, a stage agent, or a manual run, so a lead dispatching build
  draws "craft rules for the build stage" written for the executor and a
  split-posture lead that no longer authors amendments still draws
  amendment-authoring craft — fix is a session-role signal the hook reads (set by
  /lead, unset for manual or stage). (4) Operator-authorized rename or revalue of
  LIFECYCLE_KIT_SESSION_BOUNDARY to separate manual from /lead modes; lean on
  record — the boundary is honestly the session-span and evidence axis (one
  stamp, one session id; values could read per-stage/per-iteration) while
  manual-versus-lead is the driver/role axis, the same signal as part (3), so add
  a role signal rather than overload the boundary knob. Also on record: the
  lead-durability gap this session exposed — the lead template's "a ruling lands
  in the stage session that acts on it" assumes an imminent session and has no
  model for a ruling whose acting session is iterations away, so such rulings
  must be filed to a durable surface in the moment, not held in-session (this
  entry is that filing). Bundled at scope with drain-stage-active-residue as one
  lifecycle-machinery iteration, split if the envelope grows. Surfaces:
  lifecycle-kit (bin/enter-stage.sh, checks/check-stage-entry.sh, SPEC.md,
  templates/lead.md), context-kit (the session-context hook), delegation-kit
  (lead economics), CLAUDE.md. Surfaced 2026-07-16 in a /lead design session.
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
