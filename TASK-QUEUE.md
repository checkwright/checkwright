# TASK-QUEUE.md — Checkwright work queue

## Iteration: drift-kit  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

- **knowledge-friction-loop** [spec: drift-kit/SPEC.md] — the
  knowledge-friction log convention (one-liner at the moment of
  re-derivation), the close-stage triage template (remediation is a
  doc-owner tiering edit, never a standing instruction), and
  `kpi-knowledge-friction` — drift-kit's first pluggable-KPI consumer and
  its one live lag KPI.
- **manifest-temporal-gate** [spec: spec-kit/SPEC-manifest-temporal.md] —
  spec-kit's `check-manifest-temporal`: lexical tripwire over old-behavior
  narration ("previously", "renamed from", …) in tracked manifest prose;
  section + per-site exempt valves, tuned against this repo's SPECs as the
  FP corpus. Mechanizes the lexical share of close-brevity's narration
  judgment.
- **manifest-derivable-count-gate** [spec: spec-kit/SPEC-manifest-count.md] —
  spec-kit's `check-manifest-count`: lexical tripwire over pinned integers
  quantifying growing governed collections ("six gates") in manifest prose
  — ban, don't validate; the count's owner is the collection. Threshold /
  partition / fixed-set exemptions plus per-site marker; shares the
  manifest-set finder and FP corpus with its sibling.

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
- **comment-restatement-gate** [needs-spec] — tighten spec-kit's
  `check-comment-tier`: it blesses the whole contiguous comment run its lead
  directive opens, so a `spec:` pointer trailed by relocated prose passes — yet
  the SPEC says a `spec:` directive blesses only its own one-line binding, never
  a relocated block. The gate is more permissive than its own SPEC; a build
  session took the blessing path twice this iteration before deleting. The
  mechanically-decidable core: drop run-blessing so every full-line comment
  resolves on its own line as a directive or `comment-tier-exempt:`, forcing
  relocated prose to delete-or-per-line-exempt. Scope must rule the FP tension —
  the existing multi-line rationale blocks and the corpus sweep removal implies:
  is the wrapped-rationale style restatement to delete or genuinely-local to
  exempt? Sibling to the manifest-* narration gates. Surfaced 2026-07-08.
- **budget-injection-automation** [needs-spec] — surface the fresh
  `usage-verdict.sh` reading at decision time so the raw pct is never quoted
  from memory (the charter forbids eyeballing it, yet a session stated ~5% when
  the live verdict was 29%). The norm relies on the agent choosing to run the
  tool; an automation removes the choice. Candidate shape: a PreToolUse hook on
  the Agent tool (the guard-kit additionalContext pattern) injecting the verdict
  before each dispatch, and/or the context-kit session brief carrying it. Scope
  must rule: per-turn freshness vs SessionStart-only; delegation-kit vs
  context-kit ownership; block-on-PAUSE vs advise-only. Surfaced 2026-07-08.

## Done

- drift-kit-extraction

## Lessons Learned
