The `close` stage of a Checkwright iteration. Exit condition: Done and
Lessons Learned cleared.

**First step — stamp evidence.** Run
`bash lifecycle-kit/bin/enter-stage.sh close`: it appends `<iteration> close
<session-id> <date>` to `.workflow/WORKFLOW-STATE.txt` and flips the
TASK-QUEUE.md `[stage:]` line to `close`, committed together. It reads
`<session-id>` from `bin/session-id.sh` itself (never hand-picked), uses
`date +%F`, and refuses (writing nothing) if `check-stage-entry` is red.

## Session ritual

1. **Process Lessons Learned** — explicit disposition per entry: →rule
   (CLAUDE.md for cross-stage, a `.claude/commands/<stage>.md` for
   stage-local — preferred, it loads on demand), →task (name the new queue
   slug), or →discard (state why). A lesson naming an unfixed gap becomes a
   queue entry, never evaporates. Lessons that belong to the *platform's* seam
   ruling go to BRIEF.local.md (local-only).
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
5. Review README.md kit table, CLAUDE.md, and each kit README for staleness
   (*is it still true?*).
6. Runtime-artifact check: any new gitignored artifact has a reclaim path.
7. Brevity pass on CLAUDE.md (the only always-loaded surface here): is each
   block worth its standing per-session token cost?
8. Optionally push/merge — an iteration can close without publishing if a
   follow-up is planned.
