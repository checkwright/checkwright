Execute the template at lifecycle-kit/templates/skills/close.md, applying the bindings below.

## Bindings

**harvest-routing** — harvest routing (`QUEUE_KIT_LESSON_TAGS`,
`scripts/queue-config.sh`): stream each tagged entry's body through `bash
queue-kit/bin/lesson-sink.sh <tag>`, which resolves the sink from the local
`QUEUE_KIT_LESSON_SINKS` overlay or falls open to the default
`.workflow/<tag>-harvest.md` staging append.
  - `[essay]` — no sink command is configured here, so the body stages to
    `.workflow/essay-harvest.md` (gitignored operator material feeding the
    `launch-comms` methodology essay; merged into the essay, then cleared). This
    binding owns that sink, so its close-surface declaration lands here:

    close-surface: .workflow/essay-harvest.md advisory reclaim=: > .workflow/essay-harvest.md

**housekeeping** — measure, then triage. First meter this closing session:
`bash drift-kit/bin/overhead-meter.sh` logs its governance-vs-task byte proxy
(drift-kit/SPEC.md §The overhead meter), the per-session producer feeding
`kpi-overhead`. The roster step above already names which surfaces to read; what
this binding adds is the procedure each row routes to — the prompt-friction row
to guard-kit/templates/close-triage.md, the knowledge-friction row to
drift-kit/templates/close-knowledge.md, the essay-harvest row to the essay merge
named under `harvest-routing`. Any task a sweep files follows
queue-kit/SPEC.md §The tag algebra.
  - **Audit-roster review** — read `.workflow/audit-roster.txt`; for each
    un-gateable class, judge which `due:` events fired since its `last:` stamp,
    then perform or explicitly defer each due audit (a deferral is costed per
    the Gap-disposition rule, not flagged-and-skipped) and set `last:` to this
    iteration for every audit performed. This is the cadence the
    Enforcement-first carve-out owes (doctrine-kit/DOCTRINE.md
    §Methodology-maintenance rules).
  - **Trajectory projection** — after the template's Clear-Done step lands the
    `close` stamp in history, regenerate the projection (`bash
    drift-kit/bin/trajectory.sh --emit > docs/evidence-data.md`) and commit it
    with the Done clear; the gate is blind at the enter-close commit by
    construction, and the widened `trigger=` fires it on this commit
    (drift-kit/SPEC.md §The published-evidence extractor).

**release-policy** — the procedure is RELEASING.md's reordered per-iteration
close-stage steps; the bump criteria and the note grammar are docs/install.md
§Versioning and §The upgrade contract (cited, never restated here). Derive the
bump off the dated `docs/posts/` note's two sections; a qualifying iteration runs
RELEASING.md's tag / GitHub Release / badge steps (a major runs release-sweep
first, its boundary-only sub-procedure), a both-None iteration stamps `none`.
Disposition evidence: `.workflow/release-disposition.txt` (committed,
boundary-required and boundary-truncated per `scripts/lifecycle-config.sh`).
