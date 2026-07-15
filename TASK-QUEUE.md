# TASK-QUEUE.md — Checkwright work queue

## Iteration: edge-discipline  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

## Deferred

- **enforcement-map-strict-config** [needs-spec] — `gate-sdk/bin/enforcement-map.sh`
  reads `EVIDENCE_KIT_CONFIG_FILE` with the retired lenient pattern (a `:=`
  default expansion followed by a skip-if-absent file test), so a
  set-but-missing path silently skips the KPI join — the silent-wrong-config
  class the strict-config-loader-shape unit eliminated from the five kit
  loaders, left here because the emitter is a cross-kit reader outside that
  unit's envelope. Converge it on the strict distinction: set-but-missing
  exits 2; unset with the default absent still skips, preserving the
  zero-config consumer path. `check-enforcement-fresh`'s byte-compare does not
  backstop this: a regen under a typo'd `EVIDENCE_KIT_CONFIG_FILE` emits a
  wrong map that then byte-matches itself on the next check — the freshness
  gate launders the wrong config rather than catching it. Surfaced 2026-07-15
  by edge-discipline's build session.
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
