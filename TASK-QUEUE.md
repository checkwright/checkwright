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

- A kit's starter template must pass the kit's own gates when copied verbatim.
  queue-kit's `templates/TASK-QUEUE.md` tripped check-blocked-by-lead-line as a
  live queue — explanatory prose carried a bracketed blocked-by tag on a
  continuation line. The template is not a governed file, so the repo's own
  battery never sees it; only the validate-stage consumer-install proof does.
  Later kits ship templates too (spec/delegation/context/drift) — check each
  template as a live surface during that kit's validate. Fixed for queue-kit in
  commit 0b38879.
