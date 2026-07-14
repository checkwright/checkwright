# TASK-QUEUE.md — Checkwright work queue

## Iteration: operational-hygiene  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **check-plugin-exec-bit** [spec: SPEC-exec-bit.md] — a gate-sdk meta-gate
  (`check-exec-bit`) asserting index mode `100755` for by-path-invoked kit
  scripts, so a `100644` KPI plugin no longer degrades silently to
  `n/a (plugin failed)` (the kpi-overhead instance, 2026-07-11 → 2026-07-14).
  Scope ruled generalized past KPIs — glob classes over checks/, kpis/, bin/
  and the consumer gates dir; the four live `100644` bin scripts get `chmod +x`
  in the same unit. Surfaced 2026-07-14 in multi-operator-semantics close.
- **metric-dir-split** [spec: SPEC-metric-dir.md] — split the append-only
  measurement trends out of disposable scratch: gitignored `.metric/` (new
  `DRIFT_KIT_METRIC_DIR` knob) holds `overhead-log.txt` +
  `usage-history.log`; `.tmp/` becomes purely disposable, wiped at the scope
  boundary. Fixes the attested writer/reader divergence (drift-report's fixed
  export list omits `DRIFT_KIT_OVERHEAD_LOG`) by exporting the scalar
  `DRIFT_KIT_*` namespace, with a smoke assertion as the class oracle.
  **Never commit `.metric/`** — trend samples carry account UUIDs and
  session refs (the provenance seam makes the gitignore load-bearing).
  Surfaced 2026-07-14 in multi-operator-semantics close.
- **release-bump-criteria** [spec: SPEC-release-criteria.md] — no doc states
  what earns a major, minor, or patch (docs/install.md §Versioning has the
  line, not the criteria; only the deprecation machinery implies major
  semantics). Ruling: the bump floor derives from the release note's two
  sections — both "None" → patch, either non-empty → minor, a decommission →
  major, pre-1.0 breaking rides minors — landing in §Versioning with a
  `check-release-bump` floor gate in scripts/. Also ruled in: a
  `docs/releases.md` nav-slotted index deriving at render time (Liquid over
  the `release:` front-matter key) so releases are discoverable on-site with
  no second post and no new runbook step. Surfaced 2026-07-14 reviewing
  RELEASING.md against the v0.1.0 release.
- **session-boundary-knob** [spec: SPEC-session-boundary.md] — the
  fresh-session-per-stage rule becomes consumer posture:
  `LIFECYCLE_KIT_SESSION_BOUNDARY=stage|iteration` (default `stage`, today's
  behavior); at `iteration`, check-stage-evidence skips only the cross-stage
  shared-session-id check (attribution still rides the stamps). lead.md gains
  the inline-run posture sentence. This repo's own posture is decided when
  the unit lands (operator leans `iteration`; cost on record in the
  amendment). Surfaced 2026-07-14 when check-stage-evidence correctly bounced
  the lead's inline close.

## Technical Debt

## Deferred

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
