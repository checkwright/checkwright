The `validate` stage of an iteration. Run the full test/verification suites.
Exit condition: *<your validate exit condition — prefer "zero NEW failures
against a committed baseline" over bare "all pass", which is unsatisfiable
while any suite is tracked-red on a deferred blocker; each held-constant red
line carries the live task slug that blocks it>*.

**First step — stamp evidence.** Append `<iteration> validate <session-id>
<date>` to `.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`;
the stamp proves invocation, not faithful execution). As the same first step,
flip the queue header's `[stage:]` line to `validate` and commit the flip
together with this stamp — the arriving-stage flip; the line and its stamp
must match, so they ride in one commit. (`check-stage-entry` additionally
requires the active queue drained before this flip — build is not done until
the queue is empty.)

## Session ritual

*<Execute your suites. Compilation is not done — a green build is not a green
test run; gate on the positive success token of your runner, not the absence
of a failure token. Capture runs to files so the evidence outlives the
session.>*

**Triage a red against the queue before excavating it.** On any failure,
first grep the queue's deferred/lessons sections — a pre-existing red is
usually already a filed task with the diagnosis written. If it's filed: note
it and move on. Only excavate a failure that is genuinely new or a suspected
regression from the current diff.

When filing a finding, place it by kind: nameable deliverable + done-state ⇒
queue task (the deferred section, design-pending); an observation about how
the work should be done ⇒ the lessons section, dispositioned at close.
Undone work parked as a lesson evaporates; a process insight parked as a
task rots.

**Record the evidence.** Append one line per suite to your per-iteration
evidence manifest (suite, log digest, verdict, date) and commit it at validate
completion — the evidence does not exist at the entry stamp, so it rides a
later commit, not the entry flip. An iteration with a validate stamp but no
evidence line is the recorded-nothing gap the manifest closes.

Do not declare validate complete until the baseline diff is clean: no
baseline-pass item regressed, every held-constant red still carries a live
blocking slug, and any recovered item has been promoted to pass.
