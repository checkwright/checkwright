# TASK-QUEUE.md — Checkwright work queue

## Iteration: context-kit  [stage: scope]

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

## Technical Debt

## Deferred

- **drift-kit-extraction** [needs-spec] — drift-report skeleton with pluggable
  KPIs and lead/lag honesty labels (kit 7).
## Done

## Lessons Learned
