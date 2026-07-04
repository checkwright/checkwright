The `close` stage of an iteration. Harvest lessons, housekeep, optionally
merge. Exit condition: Done and Lessons Learned sections cleared (harvestable
lessons promoted first).

**First step — stamp evidence.** Append `<iteration> close <session-id>
<date>` to `.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`;
the stamp proves invocation, not faithful execution). As the same first step,
flip the queue header's `[stage:]` line to `close` and commit the flip
together with this stamp — the arriving-stage flip; the line and its stamp
must match, so they ride in one commit.

## Session ritual

1. **Process Lessons Learned** → durable rules or debt tasks, then clear the
   section. **Give each entry an explicit disposition before clearing** —
   →rule (name the file + section; prefer a stage-local skill file over an
   always-loaded doc for a stage-local procedure), →task (name the new slug),
   or →discard (state why it needs no durable home). Clearing is not
   processing: a lesson naming a concrete *unfixed gap* must become a task,
   not evaporate. When a lesson claims it was "already filed under <slug>",
   verify the target task's **body** actually carries the specific finding
   before dispositioning →task — the slug merely existing is not enough.
2. *<Your housekeeping sweeps: deprecation scan, gate-runtime budget check,
   backlog-aging / premise-rot review, tooling-friction triage.>*
3. **Clear Done.**
4. Review top-level docs for staleness (*is it still true?*).
5. **Runtime-artifact lifecycle check** — any gitignored/runtime artifact
   introduced this iteration (log, cache, scratch dir) has a named cleanup
   trigger: a write-path needs a paired reclaim-path.
6. **Brevity pass on the always-loaded surfaces** — run this **last**, after
   every surface-mutating step above. Scope by principle, not a fixed list:
   every surface injected into each agent session. Staleness asks *is it
   still true?*; brevity asks *is each block worth its standing per-session
   token cost?* — reword/delete over annotating; outdated context goes to git
   history. On-demand files (specs, this skill) are exempt — their cost is
   paid only when opened.
7. **Optionally merge** — an iteration can close without merging if validate
   is incomplete or a follow-up iteration is planned.
