# TASK-QUEUE.md — Checkwright work queue

## Iteration: norm-hardening  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

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
- **validate-evidence-layer** [needs-spec] — the one large gap from
  platform-second-scan: lifecycle-kit's stage-evidence proves a stage was
  *invoked*, never that it produced its green result. The platform closes this
  with three coupled mechanisms — a held-constant test-baseline manifest (each
  known-red test marked fail/ignore and coupled to a live tracking slug, diffed
  per-test so a new regression and an unpromoted recovery can't net to zero), a
  committed per-run evidence manifest (one line per suite, a log-digest pinning
  the producing run, gated at stage-close and merge, truncated at the iteration
  boundary), and a run-validate contract (regenerate projection → run → diff
  baseline → record verdict, never editing the baseline, failures surfaced
  verbatim). Test-runner/suite/scenario globs are consumer config; the teardown
  ops stay on the platform. Extends lifecycle-kit (validate stage) or a new
  evidence-kit; the evidence-manifest format is also the natural wire contract
  for the deferred hosted-attestation-service (its attestation payload).
  Surfaced 2026-07-08.
- **extraction-completeness-gates** [needs-spec] — the small missed-mechanism
  cluster from platform-second-scan: content-free structural gates the first
  extraction pass skipped. Not build-ready — scope must rule per-gate which earn a
  slot on checkwright under §Minimal footprint (a real, cheap, non-redundant drift
  axis on a surface this repo actually has) versus platform-only mechanism that
  does not apply here or is already covered: handbook-coverage presumes an
  always-loaded / on-demand doc split checkwright lacks; todo-refs presumes
  `TODO(task:<slug>)` markers the tree may not use; md-refs / required-section
  overlap `check-spec-pointer` and the queue-index heading dependency. The scan
  map to rule against, closest kit in parens:
  - spec-fence-balance (spec-kit) — assert an even fence-delimiter count; spec-kit's
    fence-parsing gates desync and fail *open* on an odd count.
  - md-refs (spec-kit/context-kit) — resolve internal markdown links + `#anchor`
    heading slugs across a configured doc set (dead-link rot).
  - stage-skill-coverage (lifecycle-kit) — couple the stage set to its
    `.claude/commands/<stage>.md` files, both directions.
  - handbook-coverage (context-kit) — hold an always-loaded directive's section
    index and its on-demand reference's section set verbatim-identical.
  - todo-refs (queue-kit) — resolve every `TODO(task:<slug>)` marker to a known
    queue slug; ban bare FIXME/HACK on governed surfaces.
  - backlog-aging (queue-kit → drift-kit) — advisory tripwire on deferred entries /
    permanent exemptions aged past a git-derived re-verification window.
  - gate-runtime-budget (gate-sdk → drift-kit) — advisory over the gate-timings
    measurement; flag any gate over 2× budget or the battery over aggregate.
  - hook-exec-bit (gate-sdk) — assert governed scripts + git hooks keep the exec
    bit; git silently skips a non-executable hook.
  - required-section-presence (queue-kit/spec-kit) — assert manifest files retain the
    mandatory `##` headings the index tooling locates work by.
  - slug-component-collision (queue-kit) — flag a live task slug equal to a
    component-directory name.
  - root-tiering (spec-kit/gate-sdk) — assert the repo root holds only an allowlisted
    orientation set; workflow machinery stays under the workflow dir.
  - script-names (gate-sdk) — the `check-<area>` naming convention plus bidirectional
    script↔doc citation coverage. Surfaced 2026-07-08.
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
- platform-second-scan

## Lessons Learned
