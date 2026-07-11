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

**housekeeping** — measure, then triage. First meter this closing session:
`bash drift-kit/bin/overhead-meter.sh` logs its governance-vs-task byte proxy
(drift-kit/SPEC.md §The overhead meter), the per-session producer feeding
`kpi-overhead`. Then two triage sweeps, each owned by its kit template and run
in order: tooling-friction per guard-kit/templates/close-triage.md, then
knowledge-friction per drift-kit/templates/close-knowledge.md. Any task a sweep
files follows queue-kit/SPEC.md §The tag algebra.
