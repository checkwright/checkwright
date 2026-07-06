# TASK-QUEUE.md — Checkwright work queue

## Iteration: delegation-kit  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit, in the extraction order [README.md](README.md) records.

---

## New Features

## Technical Debt

## Deferred

- **allowlist-chain-steer-rule** [needs-spec] — a friction-kit guard rule
  (bash-guard) that reads the committed `.claude/settings.json` bare `Bash(<cmd>)`
  allow entries (those without a `*` glob) and, when a submitted Bash command
  chains/pipes/redirects a leading token-sequence that exactly matches one of
  them (`&&`, `|`, `;`, `2>&1`, trailing redirect), *steers*: "run it bare — the
  bare form is statically allowed; the decoration forces a prompt." Recurs
  across stateless close/validate sessions (bit this iteration ~3x, including
  on the promoted `: > prompt-friction.log` truncation itself); the durable
  fix is mechanized steer, not a habit no session holds. Logic a glob can't
  express (criterion b). Done: guard-test fixture pair — a decorated
  allowlisted command steers, a bare one and a decorated non-allowlisted one
  pass untouched. Scope decides whether the guard reading settings.json crosses
  a coupling line worth avoiding.
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
- **comment-tier-gate** [needs-spec] — no gate monitors source-comment
  tiering: design rationale restated in code comments that a SPEC already owns
  goes uncaught (friction-kit/lib/guard.sh's per-rule comments are the
  exemplar). The platform's `check-comment-tier` deliberately stayed behind —
  its blessed-directive set is a coupling vocabulary (rule content), so per
  spec-kit/SPEC.md "What stayed on the platform" the SPEC-to-code tiering rule
  ships as prose only. Scope decides whether a Checkwright-native gate earns
  its place in this repo — and if so, whether blessed exceptions make sense at
  all: the blessed set *was* the rule-content half, so a strict "comments cite
  the owning section, never restate design" rule with no allowlist may be the
  whole extractable mechanism.
## Done

## Lessons Learned
