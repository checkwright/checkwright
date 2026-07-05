# TASK-QUEUE.md — Checkwright work queue

## Iteration: friction-kit  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **delegation-kit-extraction** [needs-spec] — agent-execution protocol
  template, usage gating, resume-journal mechanics (kit 5).
- **context-kit-extraction** [needs-spec] — markdown/pub indexes,
  session-context hook template, always-loaded baseline metering (kit 6).
- **drift-kit-extraction** [needs-spec] — drift-report skeleton with pluggable
  KPIs and lead/lag honesty labels (kit 7).

## Done

- friction-kit-extraction
- queue-starter-template-not-spec-kit-clean

## Lessons Learned
