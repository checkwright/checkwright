The `close` stage of a Checkwright iteration. Exit condition: Done and
Lessons Learned cleared.

**First step — stamp evidence.** Run
`bash lifecycle-kit/bin/enter-stage.sh close`: it appends `<iteration> close
<session-id> <date>` to `.workflow/WORKFLOW-STATE.txt` and flips the
TASK-QUEUE.md `[stage:]` line to `close`, committed together. It reads
`<session-id>` from `bin/session-id.sh` itself (never hand-picked), uses
`date +%F`, and refuses (writing nothing) if `check-stage-entry` is red.

## Session ritual

1. **Process Lessons Learned** — explicit disposition per entry, **stamped**
   one line each into `.workflow/lesson-evidence.txt`
   (`<iteration> lesson <kind> <ref> — <lead-line prefix>`, the record
   `check-lesson-disposition` reads at the queue-diff transition): →rule
   (CLAUDE.md for cross-stage, a `.claude/commands/<stage>.md` for
   stage-local — preferred, it loads on demand), →task (name the new queue
   slug), →harvest (route the entry's body per the table below), or →discard
   (state why). A lesson naming an unfixed gap becomes a
   queue entry, never evaporates. Gap generalization, per lesson that
   records drift no gate caught: name the enforcement class that should
   have caught it, then either file the missing check as a Deferred
   `[needs-spec]` task or state in one line why no scanner is buildable —
   the disposition is not complete without one of the two. A lesson belonging to the *private* seam
   ruling stamps `discard private-seam (BRIEF.local.md)` and its content goes
   to BRIEF.local.md (local-only).
   Harvest routing (`QUEUE_KIT_LESSON_TAGS`, `scripts/queue-config.sh`): stream
   each tagged entry's body through `bash queue-kit/bin/lesson-sink.sh <tag>`,
   which resolves the sink from the local `QUEUE_KIT_LESSON_SINKS` overlay or
   falls open to the default `.workflow/<tag>-harvest.md` staging append.
   - `[essay]` — no sink command is configured here, so the body stages to
     `.workflow/essay-harvest.md` (gitignored operator material feeding the
     `launch-comms` methodology essay; reclaim: merged into the essay, then
     cleared — the Housekeeping gitignored-artifact rule).
   Lesson-vs-task litmus (also at filing time, any stage): if the deliverable
   and its done-state are nameable now, it is a task — file it in Deferred
   `[needs-spec]` directly, never stage it in Lessons where it waits a stage
   and risks →discard; Lessons is for observations about how the work should
   be done, whose durable home (rule vs task vs nothing) genuinely needs the
   close-stage call.
2. **Tooling-friction triage** (guard-kit/templates/close-triage.md):
   `bash guard-kit/bin/scan-prompts.sh` and resolve each recurring pattern
   by the triage criterion — allowlist (safe & already in the form to
   reinforce), guard rule (a better form exists, or logic a glob can't
   express), or habit change (a true one-off); `bash
   guard-kit/bin/compare-settings-allow.sh` and prune the listed local
   entries, then by judgment prune remaining one-off local entries and promote
   recurring safe patterns to committed `settings.json` globs; clear the log
   (`: > .workflow/prompt-friction.log`).
3. **Knowledge-friction triage** (drift-kit/templates/close-knowledge.md): walk
   `.workflow/knowledge-friction.log` and remediate each captured re-derivation
   as a **doc-owner edit** — give the fact a home under the tier contract, or a
   pointer from where the session looked to where the owner is — never a
   standing session-start instruction; clear the log
   (`: > .workflow/knowledge-friction.log`).
4. **Clear Done.**
5. Review the docs' un-gated remainder for staleness (*is it still true?*):
   README.md row descriptions, CLAUDE.md prose, and each kit README —
   `check-kit-registration` already holds kit-table membership and the
   fixture-runner lines, so this step is the prose those gates do not read.
   Same gap-generalization obligation as step 1, per staleness actually
   found: name the enforcement class that should have caught it, and file
   the missing check as a Deferred `[needs-spec]` task or state in one line
   why no scanner is buildable — a silent fix forfeits the gate.
6. Runtime-artifact check: any new gitignored artifact has a reclaim path.
7. Brevity pass on CLAUDE.md (the only always-loaded surface here): is each
   block worth its standing per-session token cost?
8. Optionally push/merge — an iteration can close without publishing if a
   follow-up is planned.
