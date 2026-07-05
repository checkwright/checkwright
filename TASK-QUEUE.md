# TASK-QUEUE.md — Checkwright work queue

## Iteration: spec-kit  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

- **spec-kit-extraction** [spec: spec-kit/SPEC.md] — extract the spec
  discipline (kit 4) per spec-kit/SPEC.md: lib/spec.sh config loader
  (SPEC_KIT_* knobs, platform values as defaults) + templates/spec-config.sh,
  a templates/SPEC-amendment.md skeleton carrying the DoD checklist, five
  gates (amendment-queue with [spec:] refs generalized to accept a
  repo-relative path, spec-dod-singleton with an at-most-one mode,
  spec-derivable-section, spec-embedded-source, surface-duplication) with
  good/bad fixture pairs, help texts re-cited from the platform rulebook to
  spec-kit/SPEC.md, kit README, gates.list registration where each surface
  exists (no glossary in this repo, so surface-duplication stays
  unregistered), and this repo's queue made conformant (the needs-spec tag
  added to every deferred entry's lead line).

## Technical Debt

## Deferred

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
