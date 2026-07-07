# TASK-QUEUE.md — Checkwright work queue

## Iteration: drift-kit  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

- **drift-kit-extraction** [spec: drift-kit/SPEC.md] — extract the drift
  kit: the drift-report skeleton (lead/lag sections with the honesty
  labels, `--trend` line for the session hook), the `kpis.list` registry
  with gate-sdk-style resolution, the kpi-plugin contract, and the bundled
  generic KPI set; dogfood the report and registry in this repo (kit 7).
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
- **adoption-track** [needs-spec] — the outward-facing rung: docs site, demo
  walkthrough, announcement post, plugin-marketplace presence; this is the
  path that makes any absorption/standardization outcome possible; scope it
  as its own iteration post kit 7. Evidence artifact: upstream Claude Code
  issue #75214 (project config can't lift the Task ask-first default),
  surfaced dogfooding this repo's delegation nudge 2026-07-07.

## Done

## Lessons Learned
