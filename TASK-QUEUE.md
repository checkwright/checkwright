# TASK-QUEUE.md — Checkwright work queue

## Iteration: lifecycle-machinery  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

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
- **per-gate-validate-baseline** [needs-spec] — per-gate granularity for the validate
  baseline's gates suite. Cost carried while deferred: the exit-code parser holds the whole
  battery as one scenario, so any future suite-level held-red baseline row masks a fresh
  intra-suite regression until this lands (a per-gate baseline would catch the new red).
  The standing driver — the drain-entry override holding the gates suite constant-red for
  a whole validate stage — is removed by drain-stage-active-residue's assertion-B model.
  Implementation path on record so a later scope does not re-derive it: consumer-side
  only — an EVIDENCE_KIT_PARSER consumer command mapping the run-gates log to per-gate
  scenario lines plus per-gate baseline rows (evidence-kit/SPEC.md §Layout and
  configuration); no kit change expected. Surfaced 2026-07-16 as drain-stage-active-residue's
  third cost in close's knowledge-friction triage; deferred 2026-07-16 by lead ruling at
  lifecycle-machinery scope, holding the bundle to one iteration.
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
- **demand-driven-usage-refresh** [needs-spec] — demand-driven refresh
  replaces timer polling as the usage-snapshot freshness mechanism: a
  `DELEGATION_KIT_REFRESH_CMD` knob (empty default) that `usage-verdict` runs
  before reading the snapshot, so the budget guard and any verdict call poll
  at decision time; `usage.txt` survives as last-known-good cache,
  source-agnostic seam, and test seam — never deleted (operator ruling
  2026-07-16). First live poll proved the gap: the statusline snapshot said
  2-5% while the endpoint said 28% (a delegated build burning while the
  supervising session idled). Bundled by the same ruling, the login-reroute
  hoist (technical debt): `usage.txt` lags an account-switching `/login` by
  about a minute — the producer stamps the new account id while pct/resets_at
  still carry the dead login's window, so the reading is a fresh-looking
  chimera — and the reroute check living only inside the PAUSE branch lets a
  lagging pct at-or-under the threshold print OK; hoist it ahead of the
  threshold comparison. Boundary case on record: pct exactly equal to
  `DELEGATION_KIT_PAUSE_PCT` misses the strict `>` comparison. Interim
  discipline until this lands: run `templates/usage-poller.sh` then
  `usage-verdict.sh` before each dispatch. Surfaced 2026-07-16 by the live
  lead dispatches of the lifecycle-machinery iteration.
## Done

## Lessons Learned
