# TASK-QUEUE.md — Checkwright work queue

## Iteration: gap-closure  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit through extraction, then per hardening or roadmap unit;
  [README.md](README.md) records the extraction order.

---

## New Features

## Technical Debt

## Deferred

- **ddd-positioning-docs** [needs-spec] — docs page plus example consumer
  config positioning Checkwright for DDD ubiquitous-language enforcement
  (vocabulary via the check-graph/graph-vocab pattern, comment-tier
  directives); mechanism kits stay DDD-neutral — the coupling lives in docs
  and examples only; natural landing slot is alongside drift-kit (kit 7).
- **scope-session-routing** [needs-spec] — iteration-ambiguity routing across
  sessions: a build/align session forwards a question to the still-live scope
  session and relays the reply back; design it atop the harness's native
  agent-to-agent messaging (SendMessage/subagents) rather than bespoke
  plumbing — the substrate is moving fast and bespoke plumbing would be
  obsoleted; likely a companion tool or repo, not a kit; triage post kit 7.
  Extended 2026-07-07: the full ambition is the scope session as the
  iteration's *lead* — dispatching and supervising the other stage sessions,
  not only answering their questions; Q&A routing is that design's first
  rung. Design tension to rule: the lifecycle is built for stateless sessions
  with evidence stamps as the hand-off, so a live supervisor must leave the
  stamps authoritative — orchestration convenience must not become a second,
  ungated source of iteration state.
- **adoption-track** [needs-spec] — the outward-facing rung: docs site, demo
  walkthrough, announcement post, plugin-marketplace presence; this is the
  path that makes any absorption/standardization outcome possible; scope it
  as its own iteration post kit 7. Evidence artifact: upstream Claude Code
  issue #75214 (project config can't lift the Task ask-first default),
  surfaced dogfooding this repo's delegation nudge 2026-07-07.
  - *Drift-benchmark evidence sub-rung* (surfaced 2026-07-08): the demo
    walkthrough needs a benefits claim backed by data. Existing LLM/harness
    benchmarks are the wrong shape — they measure a *model* drifting in
    isolation (single-turn or single-trajectory), not a governance layer's
    effect on drift across a *series* of dependent tasks; running one "with
    Checkwright" is a category error (no repo/commit/spec for gates to bite).
    Scope a **differential experiment**, not a leaderboard number: same model,
    same multi-step task series, two arms — (A) ungoverned agent loop vs (B)
    Checkwright-governed — measuring drift *accumulation across the series*.
    - Metric axis: Drift-Bench's **"satisfiable drift"** (state stays
      internally consistent while an output silently violates a commitment made
      N turns/iterations earlier) — that is exactly what check-stage-evidence,
      the spec fences, and validate-after-every-commit are built to catch.
    - Substrate/vocab (cite primaries, not a chatbot gloss): seqBench
      (arXiv 2509.16866, tunable logical depth/backtracking) and SWE-bench for
      the dependent-task series; Drift-Bench (arXiv **2602.02455** — real title
      "Diagnosing Cooperative Breakdowns in LLM Agents under Input Faults via
      Multi-Turn Interaction"; the "Decomposing Reasoning Into Failure Types"
      expansion is confabulated, do not repeat it) for the satisfiable-drift
      framing; Lost-in-Conversation / FlowBench as supporting prior art.
    - Failure-mode → mechanism table the doc must carry: Layering Effect
      (patch-over-refactor/answer bloat) → check-comment-tier + spec-kit
      anti-restatement doctrine; premature lock-in → lifecycle state machine
      (scope gates build; check-stage-entry/evidence); satisfiable drift →
      whole-battery re-run every commit + spec fences + validate-after-commit.
    - Counterfactual (A) arm is the hard part — can't cheaply re-run real
      dogfooding "without gates." Two honest routes: (1) a cheap synthetic
      series (seqBench-style or a small sharded SWE set) run head-to-head in
      both arms; (2) self-referential — this repo already instruments its own
      (B) trajectory via drift-kit/bin/drift-report.sh + the knowledge-friction
      log, so publish the governed trajectory and state plainly that the (A)
      baseline is synthetic/held-out rather than faking a controlled A/B on
      production work. Design decision for scope stage: which route, and which
      drift-report fields feed the satisfiable-drift metric.
- **multi-operator-semantics** [needs-spec] — the lifecycle's state surfaces
  assume one operator: WORKFLOW-STATE stamps, the TASK-QUEUE stage header,
  the per-iteration scratch logs (prompt-friction, knowledge-friction), and
  the committed baselines all carry single-writer semantics. Define merge and
  conflict behavior — concurrent stage sessions, branch-per-iteration vs
  shared master, who may flip the header — before any team pilot; the kits'
  team-readiness rung. Surfaced 2026-07-07.
- **hosted-attestation-service** [needs-spec] — the team/paid rung: gates
  verified server-side by a party the committing agents cannot touch —
  hosted gate runs as a neutral attestation, cross-repo drift dashboards,
  maintained rulesets. A service, not code: cloning the kits does not clone
  the neutrality or the ops. Separate private repo, created only when
  adoption demand shows up — this entry is the public roadmap marker, not a
  scaffold; natural landing post adoption-track, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.

## Done

- commit-message-hygiene-gate
- reconsider-spec-pointers
- kit-enum-completeness-gate
- extraction-completeness-gates
- validate-evidence-layer

## Lessons Learned
