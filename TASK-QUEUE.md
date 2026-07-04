# TASK-QUEUE.md — Checkwright work queue

## Iteration: queue-kit  [stage: validate]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit now formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **spec-kit-extraction** — amendment lifecycle, causal-completeness
  checklist, content-tiering star topology, merge procedure (kit 4).
- **delegation-kit-extraction** — agent-execution protocol template, usage
  gating, resume-journal mechanics (kit 5).
- **context-kit-extraction** — markdown/pub indexes, session-context hook
  template, always-loaded baseline metering (kit 6).
- **drift-kit-extraction** — drift-report skeleton with pluggable KPIs and
  lead/lag honesty labels (kit 7).
- **friction-kit-extraction** — permission-friction reduction tooling: the
  bash command-guard, tracked `settings.json` vs. per-user
  `settings.local.json` curation, and a recurring friction-triage step in
  the close-stage skill (the source platform runs this as part of close;
  only the template placeholder made it into lifecycle-kit). Unsequenced —
  not in the original seven-kit order; slot after queue-kit if the pain
  keeps compounding.

## Done

- queue-kit-extraction

## Lessons Learned
