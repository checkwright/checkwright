# TASK-QUEUE.md — Checkwright work queue

## Iteration: adoption-track  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit through extraction, then per hardening or roadmap unit;
  [README.md](README.md) records the extraction order.

---

## New Features

- **docs-site** [spec: SPEC-docs-site.md] — docs/ on GitHub Pages: plain
  Markdown, zero build; living-pages vs dated-posts drift classes; the docs
  tier cites kit READMEs/SPECs, never restates; check-docs-kit-parity
  wrapper gate; the announcement post is the first dated post.
- **docs-cmd-gate** [spec: spec-kit/SPEC-docs-cmd.md] — check-docs-cmd:
  fenced command paths and kit-prefixed env knobs in the governed doc set
  resolve against the tree; check-md-refs' sibling, one shared governed set.
- **demo-walkthrough** [spec: SPEC-demo-walkthrough.md] — demo/run-demo.sh
  on the consumer-smoke harness: vendor → clean pass → violation blocked →
  fix → green, self-narrating transcript; registered as a validate suite so
  a bit-rotted demo is a red validate, not a stale docs page.
- **drift-trajectory** [spec: drift-kit/SPEC-trajectory.md] — trajectory.sh
  emits the per-iteration governed-trajectory table as a pure function of
  committed history; committed at docs/evidence-data.md and pinned by the
  check-trajectory-fresh consumer gate; self-referential evidence route
  ruled, the controlled A/B split out as benchmark-ab-experiment.
- **template-tiering** [spec: spec-kit/SPEC-template-tiering.md] — ruling:
  kit SPECs travel (vendor-whole install), so check-comment-tier governs
  templates/ while check-spec-pointer keeps a placeholders-by-design
  exemption; template headers thin to directive lines, installer prose
  single-sources in the owning kit's SPEC/README.

## Technical Debt

- **kpi-amendment-age-fixture-noise** — the KPI's SPEC-*.md glob counts
  gate-test fixture amendments (SPEC-example-gate.md reads as oldest, 4d);
  exclude fixture/template paths, matching the trajectory extractor's
  amendment-latency input ruling in drift-kit/SPEC-trajectory.md.

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
- **upgrade-path** [needs-spec] — the upgrade demo/tooling rung: an upgrade
  smoke that vendors tag N into a scratch consumer, re-vendors N+1 over it,
  and asserts phase-A determinism (nothing changes outside kit dirs plus
  generated artifacts) and that the battery's red set matches the release
  note's tightened-gates declaration; optionally a thin installer CLI
  wrapping the git-native vendor copy (registries stay namespace
  reservations, never a dependency channel). The two-phase upgrade
  contract itself lands with docs-site (SPEC-docs-site.md); this rung is
  buildable once a second tag exists. Surfaced 2026-07-09.
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

- adoption-track

## Lessons Learned
