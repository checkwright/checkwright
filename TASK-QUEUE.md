# TASK-QUEUE.md — Checkwright work queue

## Iteration: gap-closure  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit through extraction, then per hardening or roadmap unit;
  [README.md](README.md) records the extraction order.

---

## New Features

- **kit-enum-completeness-gate** [spec: gate-sdk/SPEC-kit-enum.md] — kill the
  hand-maintained kit-set drift axis: `kit:` couples token expanded from
  `gate_kit_roots`, meta-paths auto-union in delegation-kit's loader, and the
  residual `check-kit-enum` meta-gate over hand-lists.
- **extraction-completeness-gates** [spec: SPEC-completeness-gates.md] — the
  admitted structural-gate set: check-spec-fence-balance + check-md-refs
  (spec-kit), check-stage-skill-coverage (lifecycle-kit), check-hook-exec-bit +
  check-root-tiering (gate-sdk), check-queue-sections (queue-kit); the ruled-out
  candidates and rationale live in the amendment. Build after kit-enum (the new
  gates' couples use the `kit:` token).
- **commit-message-hygiene-gate** [spec: gate-sdk/SPEC-commit-msg.md] — a
  generated `commit-msg` hook (`tier=commit-msg` in the graph grammar) running
  check-commit-msg, plus check-tree-terms over the same pattern files; generic
  patterns ship as kit defaults, private term lists stay local-only consumer
  config.
- **validate-evidence-layer** [spec: SPEC-evidence-kit.md] — evidence-kit, a new
  kit: held-constant test baseline, committed per-run evidence manifest (the
  versioned wire contract for the deferred attestation rung), run-validate
  spine, and lifecycle-kit's generic boundary-truncate knob.

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

- reconsider-spec-pointers

## Lessons Learned
