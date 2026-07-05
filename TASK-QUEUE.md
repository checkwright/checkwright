# TASK-QUEUE.md — Checkwright work queue

## Iteration: kit-hardening  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

- **consumer-smoke-harness** [spec: SPEC-smoke.md]
  — mechanize the validate-stage scratch-consumer proof as
  `gate-sdk/bin/run-consumer-smoke.sh` plus a per-kit `smoke/` contract; the
  design, causal completeness, and DoD live in the amendment. The finder
  kit-root prune (spec-kit-vendored-spec-dod-scope) is in, so zero-config
  green on a vendored tree is now satisfiable.

## Technical Debt

- **mechanize-session-id-stamp** — the stage skills hand-pick the
  `<session-id>` for the WORKFLOW-STATE stamp and the source diverged in
  practice (transcript UUID vs. code-session URL). Ruling: the canonical id
  is the transcript UUID's first 8 hex chars — it rotates per session,
  including across a context clear, which is exactly what per-stage
  provenance needs; the code-session URL does not. Mechanism: new
  `lifecycle-kit/bin/session-id.sh` prints the id of the most recently
  written transcript under the agent sessions dir (default: the Claude
  projects dir derived from the config home and the cwd; override
  `LIFECYCLE_SESSIONS_DIR`); newest-file selection is the documented
  single-operator assumption. Wire the script into the stamp step of all
  five `templates/skills/*.md` and this repo's `.claude/commands/*.md`, and
  amend lifecycle-kit/SPEC.md's best-effort session-id wording — the id is
  read, not guessed.
- **enforce-distinct-stage-sessions** [blocked-by: mechanize-session-id-stamp]
  — check-stage-evidence verifies stamps are well-formed and current but not
  that different stages carry different sessions, so a duplicate slid through
  green (build == validate). Ruling settled: distinct stages of the current
  iteration must carry pairwise-distinct session ids — a stage flip demands a
  fresh session (stage boundaries are context boundaries); same-stage
  re-entry stamps (a multi-session build) may share or rotate ids freely, and
  waiver-token stamps are exempt. Extend check-stage-evidence with the
  cross-stage distinctness pass, add the bad fixture (two stages, one id),
  and amend lifecycle-kit/SPEC.md §check-stage-evidence — the gate now reads
  the session-id field it previously ignored.
## Deferred

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

## Lessons Learned
