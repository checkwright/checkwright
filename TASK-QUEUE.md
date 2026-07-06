# TASK-QUEUE.md — Checkwright work queue

## Iteration: context-kit  [stage: align]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

- **context-kit-extraction** [spec: context-kit/SPEC.md] — extract the
  context-management kit: md-index/md-section/pub-index, the session-context
  hook template, the always-loaded meter with its committed baseline, the
  check-brevity gate, and the close-stage brevity-pass template; dogfood the
  hook, gate, and baseline in this repo (kit 6).
- **allowlist-chain-steer-rule** [spec: friction-kit/SPEC-allowlist-chain-steer.md] —
  generic ruleset rule 10: block-and-steer a decorated command whose lead
  matches a committed bare
  allow entry, unless every segment is allowlisted (silent-grant case); shared
  glob-match helper with compare-settings-allow; guard-test rows per branch.
- **stage-entry-mechanization** [spec: lifecycle-kit/SPEC-enter-stage.md] —
  `enter-stage.sh <stage>`: the stamp+flip ritual mechanized with a
  check-stage-entry pre-flight, first-stage boundary reset included; skills
  and templates invoke it as their first step; gates stay the independent
  verifier.
- **comment-tier-gate** [spec: spec-kit/SPEC-comment-tier.md] — spec-kit's
  check-comment-tier: comments must be machine/reason directives or exempted;
  kit-mechanism directive roster as default, consumer vocabulary via
  SPEC_KIT_COMMENT_* knobs; sweep this repo's kits (WHITELIST-drained).
- **tag-lead-line-rename** [spec: queue-kit/SPEC-tag-lead-line.md] — rename
  check-blocked-by-lead-line to check-tag-lead-line: the gate governs every
  blocked-by/spec/needs-spec tag, the name claims only blocked-by; registry,
  SPEC section, fixture dir, and the generated hook artifacts move with it.

## Technical Debt

## Deferred

- **drift-kit-extraction** [needs-spec] — drift-report skeleton with pluggable
  KPIs and lead/lag honesty labels (kit 7).
- **kit-terminology-renames** [needs-spec] — user ruling wanted on the two
  heavier renames: friction-kit to guard-kit (the kit's core is lib/guard.sh;
  "friction" is platform insider vocabulary) and delegation-kit usage-gate.sh
  to usage-verdict.sh (leave "gate" meaning one thing product-wide); cheapest
  while no external consumer exists.
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
  as its own iteration post kit 7.
## Done

## Lessons Learned
