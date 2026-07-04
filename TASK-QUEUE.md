# TASK-QUEUE.md — Checkwright work queue

## Iteration: queue-kit  [stage: scope]

The lifecycle-kit gates read the header above and
`.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
the queue format itself gets formalized when queue-kit is extracted. One
iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

- **queue-kit-extraction** — extract the git-native, agent-readable task
  tracker (kit 3) per queue-kit/SPEC.md: lib/queue.sh config loader
  (QUEUE_KIT_* knobs, platform values as defaults) + templates/queue-config.sh,
  bin/queue-index.sh with the deferred tally de-hardcoded to a generic
  per-subsection count, six gates (queue-hygiene, queue-wrap,
  blocked-by-lead-line, task-names, task-conservation,
  queue-prose-precondition) with good/bad fixture pairs (task-conservation
  keeps its no-fixture annotation), help texts re-cited from the platform
  rulebook to queue-kit/SPEC.md, a starter templates/TASK-QUEUE.md skeleton,
  kit README, gates.list registration, and this repo's own queue made
  conformant (the column-0 preamble prose under the iteration header gets
  indented).

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

## Lessons Learned
