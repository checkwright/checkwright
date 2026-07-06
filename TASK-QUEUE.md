# TASK-QUEUE.md — Checkwright work queue

## Iteration: delegation-kit  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **context-kit-extraction** [needs-spec] — markdown/pub indexes,
  session-context hook template, always-loaded baseline metering (kit 6).
- **drift-kit-extraction** [needs-spec] — drift-report skeleton with pluggable
  KPIs and lead/lag honesty labels (kit 7).
- **stage-entry-mechanization** [needs-spec] — script the deterministic
  stamp+flip of a stage transition (an `enter-stage.sh <stage>` in
  lifecycle-kit/bin that reads the iteration from the header, pulls the id from
  session-id.sh, appends the correctly-formatted stamp, and flips the `[stage:]`
  line), running the entry preconditions as a pre-flight; the stage gates stay
  the independent verifier (the gen-pre-commit ↔ check-graph writer/asserter
  split). Judgment stays in the skill; only the mechanical stamp+flip moves.
## Done

- delegation-kit-extraction
- brace-expansion-guard-rule

## Lessons Learned
