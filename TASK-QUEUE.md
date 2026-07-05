# TASK-QUEUE.md — Checkwright work queue

## Iteration: friction-kit  [stage: close]

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
- **stage-entry-mechanization** [needs-spec] — script the deterministic
  stamp+flip of a stage transition (an `enter-stage.sh <stage>` in
  lifecycle-kit/bin that reads the iteration from the header, pulls the id from
  session-id.sh, appends the correctly-formatted stamp, and flips the `[stage:]`
  line), running the entry preconditions as a pre-flight; the stage gates stay
  the independent verifier (the gen-pre-commit ↔ check-graph writer/asserter
  split). Judgment stays in the skill; only the mechanical stamp+flip moves.
- **brace-expansion-guard-rule** [needs-spec] — extend the friction-kit generic
  ruleset with a rule that pre-empts unquoted brace expansion the way rule 6
  pre-empts `$(...)`: the harness prompts on the bare `{` glyph before allowlist
  matching (git-ref shorthand `@{u}`/`HEAD@{n}`, `{a,b}`/`{a..b}` lists), so
  steer to the explicit form (`origin/<branch>..HEAD`, written-out lists).
  Scope call for /scope: how not to over-block legitimate brace-glob
  convenience (`mkdir -p a/{b,c}`). Ships with guard-tests; updates SPEC
  §ruleset.

## Done

## Lessons Learned
