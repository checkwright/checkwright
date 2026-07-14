# TASK-QUEUE.md — Checkwright work queue

## Iteration: surface-trust  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

## Deferred

- **scratchpad-path-guard** [needs-spec] — the harness steers every session
  toward its per-session scratchpad under `/tmp/claude-…` while this repo's
  ruling is repo-local `.tmp/` (CLAUDE.md §Housekeeping) — a memory-dependent
  rule, re-leaked in practice (a dead poller script found in the scratchpad).
  No pre-commit gate can see the violation (the file never enters the tree);
  the enforcement point is the consumer bash-guard: block Bash commands
  referencing the harness-scratchpad path prefix, steering to `.tmp/`. Match
  the prefix only — kit mechanism legitimately uses `TMPDIR` (the hermetic
  bootstrap's shared empty file). Known limit, accepted at filing: direct
  Write/Edit tool calls are not hooked; a `Write|Edit` PreToolUse matcher is
  the follow-on if leakage persists. Surfaced 2026-07-14 by the operator
  reviewing the lead session's scratch hygiene.
- **strict-config-loader-shape** [needs-spec] — the eleven kit config
  loaders split into two shapes: six fail closed (exit 2) on a set-but-missing
  `<KIT>_CONFIG_FILE`, five (`${VAR:-default}` expansion) skip it silently —
  an operator typo in the env var silently runs kit defaults, the same
  silent-wrong-config class the hermetic bootstrap kills inside tests.
  Unify the five lenient loaders on the strict shape; the unset-default
  path stays skip-if-absent for zero-config consumers. Each shape is
  documented in its own SPEC today (divergence, not undocumented drift), so
  the SPEC lines move with the code. Surfaced 2026-07-14 by surface-trust's
  align audit.
- **releases-nav-children** [needs-spec] — surface each release note as a
  second-level nav child of the Releases page. Ruled derivation-first: the
  nav include gains a generic derived-children branch — the parent's front
  matter names a key (e.g. `nav_children_key: release`) and the children
  derive from the same `release:` query the page body already runs, newest
  first — never per-note `nav_parent` stamps (dated posts are immutable, and
  the releases page itself refuses a maintained index). Verify with the
  local Jekyll render. Surfaced 2026-07-14 by the operator reviewing the
  public site.
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

- hermetic-kit-test-config
- commit-msg-session-trailer-screen
- orchestration-headline

## Lessons Learned
