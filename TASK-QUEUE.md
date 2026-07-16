# TASK-QUEUE.md — Checkwright work queue

## Iteration: launch-readiness  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **launch-readiness-gate** [spec: canon-kit/SPEC-prose-tells.md] — the
  readiness bar a surge-channel (Lobsters / Show-HN-class) submission must
  clear. Build ships the durable half: `check-prose-tells`, canon-kit's
  mechanical AI-prose-tell gate over consumer-configured docs surfaces (the
  amendment owns the tell set, knobs, and valve). Validate runs the four
  readiness checks: (1) cold clone of the *public* repo, `demo/run-demo.sh`
  exits 0 with the full vendor → blocked → fix → green arc (the dev-tree pass
  is already attested; the cold public clone is the delta this asserts);
  (2) reader-facing link crawl of the rendered checkwright.dev site — dead
  internal/external links and half-finished pages, the path site-kit's
  render/deployment gates do not cover; (3) the prose gate green on this
  repo's configured docs set, plus a one-time human read for the
  non-mechanical tells; (4) install-path dry-run — docs/install.md followed
  verbatim as a new adopter, vendoring recipe and AGENTS.md adapter notes
  confirmed as written. Exit criterion: all four checks clean ⇒ a
  surge-channel submission is unblocked (the upstream criterion's "all three
  clean" reconciled to the four checks it lists). Surfaced 2026-07-16 by the
  operator's launch triage.

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
