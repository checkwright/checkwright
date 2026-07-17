The `close` stage of an iteration. Harvest lessons, housekeep, optionally
merge. Exit condition: Done and Lessons Learned sections cleared (harvestable
lessons promoted first).

**First step — stamp evidence.** Run lifecycle-kit's `bin/enter-stage.sh
close`: it appends `<iteration> close <session-id> <date>` to
`.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`; the stamp
proves invocation, not faithful execution) and flips the queue header's
`[stage:]` line to `close`, reading `<session-id>` from `bin/session-id.sh`
(the newest transcript — never hand-picked), using `date +%F`, and refusing
(writing nothing) if `check-stage-entry` is red. Commit the flip together with
this stamp — the arriving-stage flip; the line and its stamp must match, so
they ride in one commit.

## Session ritual

1. **Process Lessons Learned** → durable rules or debt tasks, then clear the
   section. **Give each entry an explicit disposition before clearing, and
   stamp it** — one line per entry into `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`
   (`<iteration> lesson <kind> <ref> — <lead-line prefix>`), the record
   `check-lesson-disposition` reads when the entry leaves the queue (the
   stamp, not the commit body, is the mechanically-decidable evidence — the
   pre-commit battery has no commit message yet). The disposition set is
   →rule (name the file + section; prefer a stage-local skill file over an
   always-loaded doc for a stage-local procedure), →task (name the new slug),
   →harvest (a lesson carrying a configured harvest tag on its lead line —
   route its *body* to the sink this skill names for that tag, below), or
   →discard (state why it needs no durable home — a lesson that is private
   rule content discards to the consumer's local-only private brief, if one
   exists, rather than any tracked surface). Clearing is not
   processing: a lesson naming a concrete *unfixed gap* must become a task,
   not evaporate. When a lesson claims it was "already filed under <slug>",
   verify the target task's **body** actually carries the specific finding
   before dispositioning →task — the slug merely existing is not enough.
   Gap generalization, per lesson recording drift no check caught: name the
   check class that should have caught it, then either file the missing
   check as a deferred task or state in one line why no scanner is
   buildable — the disposition is not complete without one of the two.
   Lesson-vs-task litmus (holds at filing time in any stage, not just here):
   if the deliverable and its done-state are nameable now, it is a task —
   file it in the deferred section directly, never stage it in Lessons where
   it waits a stage and risks →discard; Lessons is for observations about
   how the work should be done, whose durable home (rule vs task vs nothing)
   genuinely needs this stage's call.
   *<harvest-routing: harvest routing table: each `QUEUE_KIT_LESSON_TAGS` tag →
   the sink file its body appends to, plus the sink's reclaim path (a gitignored
   sink needs a named trigger that empties it — the runtime-artifact lifecycle
   rule).>*
2. *<housekeeping: your housekeeping sweeps: deprecation scan, gate-runtime
   budget check, backlog-aging / premise-rot review, tooling-friction
   triage.>*
3. **Clear Done.**
4. Review top-level docs for staleness (*is it still true?*). Same
   gap-generalization obligation as step 1, per staleness actually found:
   name the check class that should have caught it, and file the missing
   check as a deferred task or state in one line why no scanner is
   buildable — a silent fix forfeits the check.
5. **Runtime-artifact lifecycle check** — any gitignored/runtime artifact
   introduced this iteration (log, cache, scratch dir) has a named cleanup
   trigger: a write-path needs a paired reclaim-path.
6. **Release disposition** — run after the surface-mutating steps above and
   **before** the brevity pass (the disposition note is itself such a write).
   Every close dispositions the iteration at the release boundary: read the
   consumer's release policy (the `release-policy` slot below) and either
   execute its release procedure or stamp an explicit no-release line. Silence
   is not a disposition — a close that says nothing about release is incomplete.
   Stamp one line into the consumer-named disposition-evidence file:
   `<iteration> release <version|none> — <one-line basis>` (the
   `check-lesson-disposition` contract shape at the release boundary, the same
   lineage release-sweep's stamp follows) — `<version>` is the tag applied,
   `none` states the both-None (or consumer-equivalent) outcome. Ordering: a tag
   names a commit, so the tag-and-host-release half of the procedure runs *after*
   the iteration's final commit lands; the note-authoring and stamp halves ride
   the close commits themselves.
   *<release-policy: the consumer's release procedure and criteria source by
   citation, the disposition-evidence path, and any boundary-only sub-procedures
   (e.g. a major-only deprecation sweep); or a plain "no release process — every
   iteration stamps none" line for a consumer without one.>*
7. **Brevity pass on the always-loaded surfaces** — run this **last**, after
   every surface-mutating step above. Scope by principle, not a fixed list:
   every surface injected into each agent session. Staleness asks *is it
   still true?*; brevity asks *is each block worth its standing per-session
   token cost?* — reword/delete over annotating; outdated context goes to git
   history. On-demand files (specs, this skill) are exempt — their cost is
   paid only when opened.
8. **Optionally merge** — an iteration can close without merging if validate
   is incomplete or a follow-up iteration is planned.
