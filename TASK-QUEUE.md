# TASK-QUEUE.md — Checkwright work queue

## Iteration: surface-trust  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **commit-msg-session-trailer-screen** [spec: SPEC-commit-msg-session-trailer-screen.md]
  — screen commit messages against harness-injected session-reference
  trailers before they publish a private reference into public history. The
  class ships as tracked generic patterns (a scope-stage seam correction of
  the local-only filing; the amendment owns the ruling and the shared-source
  anchoring constraint). Surfaced 2026-07-14 during multi-operator-semantics
  acceptance; promotion signal: two live attested instances during
  operational-hygiene's build, caught only by manual scan.
- **orchestration-headline** [spec: SPEC-orchestration-headline.md] — promote
  the orchestration story to headline positioning: index-hero differentiator
  framing, a multi-operator walkthrough section on docs/orchestration.md
  citing lifecycle-kit/SPEC.md §Multi-operator semantics downward, and the
  status-changelog section rewritten present-tense. Surfaced 2026-07-14 by
  the operator reviewing the public site.
- **hermetic-kit-test-config** [spec: SPEC-hermetic-kit-test-config.md] —
  bespoke `gate-tests/*.test.sh` runs inherit the consumer's cwd-relative
  kit config and can green wrongly (attested: 689cd9c). A shared
  test-hermetic bootstrap pins every kit's `_CONFIG_FILE` to an empty file;
  sweep all 30 tests onto it; meta-gate `check-test-hermetic` enforces the
  pairing. Surfaced 2026-07-14 during operational-hygiene build acceptance.

## Technical Debt

## Deferred

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
