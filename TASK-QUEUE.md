# TASK-QUEUE.md — Checkwright work queue

## Iteration: multi-operator-semantics  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

- **check-plugin-exec-bit** — no gate asserts a by-path-invoked kit script
  carries the executable bit, so a KPI authored without `+x` degrades silently
  to `n/a (plugin failed)` in every drift report until a human notices.
  Concrete instance: `drift-kit/kpis/kpi-overhead.sh` shipped `100644` and its
  overhead KPI was dropped from `drift-report` from its 2026-07-11 introduction
  until fixed in this close. `drift-report.sh:102` invokes each KPI by-path
  (`"$path"`), which needs `+x`. Buildable scanner: a meta-gate asserting mode
  `100755` in the git index for every registered KPI plugin (scope decision at
  build: KPIs only, or generalize to all by-path-invoked `*/kpis/*.sh` +
  `*/bin/*.sh` kit scripts). Fixture pair: a `100644` plugin reds, a `100755`
  plugin passes. Surfaced 2026-07-14 in multi-operator-semantics close.

## Deferred

- **session-boundary-knob** [needs-spec] — make the fresh-session-per-stage
  rule consumer posture: `LIFECYCLE_KIT_SESSION_BOUNDARY=stage|iteration`
  (default `stage`, today's behavior). At `iteration`, check-stage-evidence
  skips the cross-stage shared-session-id check — attribution still rides the
  stamps, so reuse stays on the audit trail; the governed-surface doctrine is
  unchanged. lead.md gains the missing sentence: strict posture bars the lead
  from running a stage inline for an iteration it stamped; relaxed posture
  sanctions inline as the fallback when dispatch is blocked (e.g. budget guard).
  Fixture pairs for both postures; knob joins the §Layout and configuration
  roster. Operator leans `iteration` for this repo (cost acknowledged: the
  dogfood evidence stops demonstrating the strict posture); the repo setting is
  decided when the unit lands, not at filing. Surfaced 2026-07-14 when
  check-stage-evidence correctly bounced the lead's inline close.
- **metric-dir-split** [needs-spec] — `.tmp/` conflates disposable scratch with
  append-only cross-session measurement trends (`overhead-log.txt` → kpi-overhead;
  `usage-history.log` → delegation usage-trend), so a boundary `rm .tmp/*` can't
  wipe scratch without destroying the trends. Split: a dedicated gitignored
  `.metric/` holds the persistent trends; `.tmp/` becomes purely disposable.
  **Never commit `.metric/`** — `usage-history.log` carries account UUIDs + budget/
  login timestamps and `overhead-log.txt` per-session refs; the public-repo seam
  (CLAUDE.md) bars it, so the gitignore is load-bearing. Kit fix (the needs-spec
  point): `drift-report.sh` exports a fixed `DRIFT_KIT_*` list that OMITS
  `DRIFT_KIT_OVERHEAD_LOG`, so a consumer override diverges the writer
  (`overhead-meter.sh` sources `drift-config.sh`) from the reader (`kpi-overhead.sh`
  reads only the exported env via `drift-report`) — silently, since the overhead
  KPI is advisory lead-tier, not a blocking gate. Decide the knob shape: add
  `DRIFT_KIT_OVERHEAD_LOG` to the export list, a dedicated `DRIFT_KIT_METRIC_DIR`
  knob, or have `kpi-overhead.sh` source its config. Gap generalization — the
  writer/reader-divergence class has no scanner today; the buildable one is a
  drift-kit smoke assertion that the write path and the read path resolve the same
  log, filed within this task. Delegation side: point
  `DELEGATION_KIT_USAGE_HISTORY` at `.metric/` (verify usage-verdict writer +
  usage-trend reader both source `delegation-config.sh`). `gate-timings.txt` STAYS
  in `.tmp/` (regenerated every `run-gates`, so a wipe is harmless; relocating it
  needs a new gate-sdk knob). Then: migrate the two logs (`mv` to preserve history),
  add `.metric/` to `.gitignore`, update CLAUDE.md §Housekeeping (`.metric/`
  gitignored persistent/account-bearing measurements never committed; `.tmp/`
  purely disposable, wiped at the scope boundary), land the deferred scope-hygiene
  `rm .tmp/*` line (scope skill) that this unblocks, and extend the `git clean
  -x/-X` guard message to name `.metric/`. Surfaced 2026-07-14 in
  multi-operator-semantics close, triaging the resume-journal relocation package.
- **commit-msg-session-trailer-screen** [needs-spec] — screen commit messages
  against harness-injected trailers that reference an internal session: a
  default session-reference trailer can ride into a commit body unnoticed and
  publish a private reference into public history (the commit-msg battery has
  no screen for it today, so the guard is manual). Gate shape: a banned-pattern
  class over the trailer block, sibling to check-commit-msg's pattern-file
  mechanism, kept in the gitignored-local pattern list so the public repo ships
  no session-shaped literal (the provenance seam). Surfaced 2026-07-14 during
  multi-operator-semantics acceptance.
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
