# TASK-QUEUE.md — Checkwright work queue

## Iteration: spec-kit  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **spec-kit-vendored-spec-dod-scope** [needs-spec] — spec-kit's platform-default
  `exactly-one` DoD mode fails out of the box for every consumer: the vendored
  kits are a DoD-less reference-spec corpus that lands in each consumer's tree
  beside gate-sdk, and the canonical-spec finder prunes `templates/`+`gate-tests/`
  but not sibling kit roots, so `check-spec-dod-singleton` flags every kit
  `SPEC.md`. The documented escape (`at-most-one`) weakens the gate for the
  consumer's own specs. Ruling needed: prune vendored kit roots (`gate_kit_roots`)
  from the spec finders by default, vs. document the constraint in spec-kit's
  README install steps. Surfaced by the spec-kit validate scratch-consumer proof.
- **mechanize-session-id-stamp** [needs-spec] — the stage skills hand-pick the
  `<session-id>` for the WORKFLOW-STATE stamp, and the source diverged in practice:
  scope used the transcript UUID, build used the stable Claude-Session URL (which
  does not change across `/clear`). Provide a script/step that deterministically
  emits the current session's transcript UUID (the actively-written `.jsonl` under
  the projects dir) at minimal token cost, and wire it into every stage skill's
  stamp step so the id is read, not guessed. Ruling: retrieval mechanism + which id
  is canonical.
- **enforce-distinct-stage-sessions** [needs-spec] — check-stage-evidence verifies
  stamps are well-formed and current but not that different stages carry different
  sessions, so a duplicate slid through green (build == validate). Add a
  distinctness check across an iteration's stages. Open question: a build may span
  several tasks in one hot-context session (so build sub-invocations need not
  differ), but distinct iteration STAGES should require distinct session ids —
  settle that scope before gating. Pairs with mechanize-session-id-stamp.
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

## Lessons Learned
