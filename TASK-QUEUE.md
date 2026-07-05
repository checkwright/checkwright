# TASK-QUEUE.md — Checkwright work queue

## Iteration: kit-hardening  [stage: validate]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **queue-starter-template-not-spec-kit-clean** [needs-spec] — queue-kit's
  `templates/TASK-QUEUE.md` teaches queue grammar to a queue-only consumer, but
  its example New-Features entries carry no spec tag and a Deferred example
  points a spec ref at a nonexistent SPEC-example.md, so in a combined tree
  spec-kit's check-amendment-queue reddens on a verbatim copy. Surfaced by the
  consumer-smoke harness (whose queue install writes a minimal filled queue
  instead of copying the template verbatim). Ruling not yet settled: make the
  template cross-kit-clean (retag or relocate the teaching examples, or ship a
  paired example amendment), or document that a spec-kit consumer fills the
  template rather than copying it verbatim.

- **delegation-kit-extraction** [needs-spec] — agent-execution protocol
  template, usage gating, resume-journal mechanics (kit 5).
- **context-kit-extraction** [needs-spec] — markdown/pub indexes,
  session-context hook template, always-loaded baseline metering (kit 6).
- **drift-kit-extraction** [needs-spec] — drift-report skeleton with pluggable
  KPIs and lead/lag honesty labels (kit 7).
- **friction-kit-extraction** [needs-spec] — permission-friction reduction
  tooling: the bash command-guard, tracked `settings.json` vs. per-user
  `settings.local.json` curation, and a recurring friction-triage step in
  the close-stage skill (the source platform runs this as part of close;
  only the template placeholder made it into lifecycle-kit). Unsequenced —
  not in the original seven-kit order; slot after queue-kit if the pain
  keeps compounding.

## Done

- lifecycle-amendment-scan-prunes-templates
- spec-kit-vendored-spec-dod-scope
- mechanize-session-id-stamp
- enforce-distinct-stage-sessions
- consumer-smoke-harness

## Lessons Learned
