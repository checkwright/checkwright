Execute the template at lifecycle-kit/templates/skills/close.md, applying the bindings below.

## Bindings

**harvest-routing** — harvest routing (`QUEUE_KIT_LESSON_TAGS`,
`scripts/queue-config.sh`): stream each tagged entry's body through `bash
queue-kit/bin/lesson-sink.sh <tag>`, which resolves the sink from the local
`QUEUE_KIT_LESSON_SINKS` overlay or falls open to the default
`.workflow/<tag>-harvest.md` staging append.
  - `[essay]` — no sink command is configured here, so the body stages to
    `.workflow/essay-harvest.md` (gitignored operator material feeding the
    `launch-comms` methodology essay; reclaim: merged into the essay, then
    cleared — the runtime-artifact rule).

**housekeeping** — tooling-friction triage (guard-kit/templates/close-triage.md):
`bash guard-kit/bin/scan-prompts.sh` and resolve each recurring pattern by the
triage criterion — allowlist (safe & already in the form to reinforce), guard
rule (a better form exists, or logic a glob can't express), or habit change (a
true one-off); `bash guard-kit/bin/compare-settings-allow.sh` and prune the
listed local entries, then by judgment prune remaining one-off local entries
and promote recurring safe patterns to committed `settings.json` globs; clear
the log (`: > .workflow/prompt-friction.log`). Then knowledge-friction triage
(drift-kit/templates/close-knowledge.md): walk `.workflow/knowledge-friction.log`
and remediate each captured re-derivation as a doc-owner edit — give the fact a
home under the tier contract, or a pointer from where the session looked to the
owner — never a standing session-start instruction; clear the log
(`: > .workflow/knowledge-friction.log`).
