# TASK-QUEUE.md — Checkwright work queue

## Iteration: norm-hardening  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

- **platform-second-scan** — read-only audit of the source platform's
  meta-layer against the shipped kits (the extraction completed with
  drift-kit): surface generic mechanism the extraction missed; file small
  finds as debt in this iteration, larger ones as deferred design-pending
  entries. Ruled over prioritizing the platform's own kit-adoption task —
  that stays a platform-queue task per the seam ruling, and this scan is
  its prerequisite map: adoption lands on verified-complete kits.

## Deferred

- **kit-enum-completeness-gate** [needs-spec] — a meta-gate asserting the kit set
  is enumerated completely wherever it is hand-listed: every `gate_kit_roots`
  member appears in `DELEGATION_KIT_META_PATHS` and in each whole-tree gate's
  `# graph:` couples (scoped to the kit files that gate's subject actually
  covers). Surfaced by the drift-kit omission this iteration — the kit set is
  hand-maintained across many manifests and silently drifted as kits 6–7 landed
  (drift-kit fell out of META_PATHS plus the comment-tier / spec-pointer /
  shellcheck couples, all fixed; context-kit's `check-brevity` is still uncoupled
  from the gate-family meta-gates — an instance this gate must catch). Derive the
  check from `gate_kit_roots`, the canonical enumeration. Surfaced 2026-07-08.
- **reconsider-spec-pointers** [needs-spec] — the operator's standing doubt that
  `spec:` pointers earn their keep: only the forward side is gate-checked
  (`check-spec-pointer` confirms the target resolves), the reverse (code still
  satisfies the cited section) is a review concern, and the basename↔§heading
  convention already derives most of the coupling — so the pointer is largely
  redundant and its gloss is SPEC restatement. Rule whether to drop `spec:` from
  the comment-tier roster, narrow it to a bare pointer (no gloss), or keep it as
  a forcing-function slot; a large reversal (the star topology,
  `check-spec-pointer`'s reason to exist, the pointers across the tree) that
  belongs in scope, not a build session. Surfaced 2026-07-08.
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

- budget-injection-automation
- comment-restatement-gate

## Lessons Learned
