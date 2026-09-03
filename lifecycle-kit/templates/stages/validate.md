The `validate` stage of an iteration. Run the full test/verification suites.
Exit condition: *<exit-condition: your validate exit condition — prefer "zero
NEW failures against a committed baseline" over bare "all pass", which is
unsatisfiable while any suite is tracked-red on a deferred blocker; each
held-constant red line carries the live task slug that blocks it>*.

**First step — stamp evidence.** Run lifecycle-kit's `bin/enter-stage.sh
validate`: it appends `<iteration> validate <session-id> <date> <head>` to
`.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`; the stamp
proves invocation, not faithful execution), reading `<session-id>` from
`bin/session-id.sh`
(the newest transcript — never hand-picked), using `date +%F`. That stamp *is*
the transition — the last stamp is the stage cursor, so nothing flips and no
queue write is involved. Commit the stamp on its own — unless the
pre-flight valve admitted this entry, which rewrites the valve ledger in the
same motion, so the two commit together (lifecycle-kit/SPEC.md
§bin/enter-stage.sh). The tool refuses (writing
nothing) if
`check-stage-entry` is red — which for `validate` additionally requires the
active queue drained before this entry (build is not done until the queue is
empty). On a refusal, **do not force the entry** — escalate to the lead (where
one exists and this is not a standalone session) and stop; a refused entry is a
gate verdict to resolve at its source, never to override.

## Session ritual

*<suites: execute your suites. Compilation is not done — a green build is not a
green test run; gate on the positive success token of your runner, not the
absence of a failure token. Capture runs to files so the evidence outlives the
session. Name the baseline-grammar owner any red-triage baseline edit writes
against.>*

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

**Excavation that spans a corpus is a survey, not a triage.** When a red sends
you across a whole corpus rather than into one failure, check the survey record
before dispatching that sweep and file its finding afterwards — the next
iteration's stages read it, and the record is discarded at the boundary anyway
(lifecycle-kit/SPEC.md §The survey record). Keep it distinct from the evidence
manifest below: that one records what your suites did, this one what a sweep
found.

**Record the evidence.** Append one line per suite to your per-iteration
evidence manifest (suite, log digest, verdict, date) and commit it at validate
completion — the evidence does not exist at the entry stamp, so it rides a
later commit, not the entry stamp. An iteration with a validate stamp but no
evidence line is the recorded-nothing gap the manifest closes. If your kit
provides a codified validate spine — a `run-validate` tool that runs each
suite, diffs the held-constant baseline, and appends the evidence line —
invoke it here rather than hand-running the suites and hand-writing the
manifest.

Do not declare validate complete until the baseline diff is clean: no
baseline-pass item regressed, every held-constant red still carries a live
blocking slug, and any recovered item has been promoted to pass.

**Commit a repair the moment you decide it, before you start or resume the
suite roster.** A suite may assert the worktree is clean against the real
checkout rather than a scratch clone, and such a suite reads your own
uncommitted repair as the failure of whatever leg it guards — the failure text
names that leg, not you, so it is diagnosed as a regression from this
iteration's diff when it is the session's own state. Batching repairs to commit
later is what buys the false red; the ordering is the whole fix. Check
`git status` against the session's own edits before excavating any suite
failure that mentions a dirty tree.

**Arm the pre-flight valve when you end on a deliberately accepted red.** A red
you understand and that is not a regression from this iteration's diff, and that
the closing stage rather than this one must file against, is a hand-off — not a
stop for an operator round-trip and not a queue edit. Append one
`<iteration> <stage> armed <reason>` line to the configured valve ledger and
commit it with your evidence, writing in the reason what close needs: which
suite, what the red is, and why it is accepted rather than fixed. The contract,
its narrowings, and the fact that reaching for it twice in one iteration is the
failure: lifecycle-kit/SPEC.md §bin/enter-stage.sh. With no ledger configured
there is no valve, and an accepted red stops here as it always did.
