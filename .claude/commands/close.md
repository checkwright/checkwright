The `close` stage of a Checkwright iteration. Exit condition: Done and
Lessons Learned cleared.

**First step — stamp evidence.** Append `<iteration> close <session-id>
<date>` to `.workflow/WORKFLOW-STATE.txt`; flip the TASK-QUEUE.md `[stage:]`
line to `close` in the same commit.

## Session ritual

1. **Process Lessons Learned** — explicit disposition per entry: →rule
   (CLAUDE.md for cross-stage, a `.claude/commands/<stage>.md` for
   stage-local — preferred, it loads on demand), →task (name the new queue
   slug), or →discard (state why). A lesson naming an unfixed gap becomes a
   queue entry, never evaporates. Extraction-methodology lessons that belong
   to the *platform's* seam ruling go to EXTRACTION.md (local-only).
2. **Clear Done.**
3. Review README.md kit table, CLAUDE.md, and each kit README for staleness
   (*is it still true?*).
4. Runtime-artifact check: any new gitignored artifact has a reclaim path.
5. Brevity pass on CLAUDE.md (the only always-loaded surface here): is each
   block worth its standing per-session token cost?
6. Optionally push/merge — an iteration can close without publishing if a
   follow-up is planned.
