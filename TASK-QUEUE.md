# TASK-QUEUE.md — Checkwright work queue

## Iteration: lifecycle-kit  [stage: build]

The lifecycle-kit gates read the header above and
`.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
the queue format itself gets formalized when queue-kit is extracted. One
iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **queue-kit-extraction** — the TASK-QUEUE format, slug namespace,
  blocked-by/needs-spec tag algebra, queue index, hygiene gates (kit 3).
- **spec-kit-extraction** — amendment lifecycle, causal-completeness
  checklist, content-tiering star topology, merge procedure (kit 4).
- **delegation-kit-extraction** — agent-execution protocol template, usage
  gating, resume-journal mechanics (kit 5).
- **context-kit-extraction** — markdown/pub indexes, session-context hook
  template, always-loaded baseline metering (kit 6).
- **drift-kit-extraction** — drift-report skeleton with pluggable KPIs and
  lead/lag honesty labels (kit 7).

## Done

- **lifecycle-kit-extraction** — stage state machine (header +
  WORKFLOW-STATE evidence), check-stage-evidence + check-stage-entry
  de-hardcoded onto stages-as-config, five stage-skill templates, fixture
  pairs + two behavioral test suites; gate-sdk resolution generalized from
  single-kit to every vendored kit's checks/.

## Lessons Learned
