The `close` stage of an iteration. Harvest lessons, housekeep, optionally
merge. Exit condition: Done and Lessons Learned sections cleared (harvestable
lessons promoted first).

**First step — stamp evidence.** Run lifecycle-kit's `bin/enter-stage.sh
close`: it appends `<iteration> close <session-id> <date>` to
`.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`; the stamp
proves invocation, not faithful execution), reading `<session-id>` from
`bin/session-id.sh`
(the newest transcript — never hand-picked), using `date +%F`, and refusing
(writing nothing) if `check-stage-entry` is red. On a refusal, **do not force
the entry** — escalate to the lead (where one exists and this is not a standalone
session) and stop; a refused entry is a gate verdict to resolve at its source,
never to override. That stamp *is* the
transition — the last stamp is the stage cursor, so nothing flips and no queue
write is involved. Commit the stamp on its own.

## Session ritual

**Where close's own captures file.** Close never writes the active queue
sections — promoting a unit into them is scope's. Every finding close itself
captures files instead as a Deferred `[design-pending]` entry for a later scope
to promote: a lesson turned task, a drained gap, a housekeeping or triage
finding, or a dispatch's "capture as debt". The active sections must be empty
at the drain boundary, so a finding mis-filed into one is caught only later as
a red `check-stage-entry` at the next iteration's entry; filing to Deferred
keeps the boundary clean (lifecycle-kit/SPEC.md §check-stage-entry owns the
deferred-filing model for ruled-but-unpromoted work).

**The survey record is not a close surface.** It carries no disposition
obligation and blocks no boundary — the next first-stage entry truncates it,
and that is the whole reclaim (lifecycle-kit/SPEC.md §The survey record). Close
owes it exactly one thing: a survey close *buys* while triaging is filed like
any other, so a finding worth an iteration's carry does not die with this
session.

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
2. **Drain the gap inbox** (`LIFECYCLE_KIT_GAP_INBOX_FILE`,
   §The committed gap inbox) → disposition every `- <date> — <gap>` bullet,
   then **truncate the inbox to its `# contract:` header**. The disposition set
   is →promote (file a deferred `[design-pending]` queue entry for the gap),
   →fix (resolve it inline this session), or →discard (state why in the close
   commit message — the bullet's own prose is the disposition body). The date
   feeds the staleness read (an aged bullet is a signal, not a free pass).
   Draining is not deleting: a bullet naming a concrete unfixed gap must become a
   task or a fix, not evaporate — the same clearing-is-not-processing rule as
   Lessons.
   **Re-verify before dispositioning.** A bullet's prose is the filer's claim, not
   an established fact: capture is deliberately frictionless, so nothing upstream
   checked it and a bullet can assert a mechanism its filer inferred rather than
   ran. Per bullet, name the claim the disposition turns on and the command that
   establishes it, run that command, and disposition against what it returned.
   Record the outcome in the close commit message beside the bullet's disposition
   — which bullets were re-verified and what fell — so a corrected premise is
   visible rather than silently absorbed. A claim no cheap command settles is
   dispositioned *as a claim*: say so, and let the promoted entry carry the
   unverified premise openly (§The committed gap inbox rules why this sits at the
   drain and not at filing time).
   The next iteration's first-stage entry refuses a non-empty inbox **when this
   stage was skipped** (§bin/enter-stage.sh), so an undrained gap blocks the
   boundary rather than crossing it silently.
   **A finding your own later steps generate postdates this drain, and it routes
   to the inbox like any other.** The audits, the lesson disposition, the
   staleness read and the release disposition all run after this step, so what
   they turn up has no drainer left in this iteration. File it (`bin/file-gap.sh`)
   and let it be carried: the next first-stage entry admits it and takes it as
   that session's intake (§The committed gap inbox). Do **not** back-date it into
   a drain that has already run — a second pass over an inbox your own later
   steps keep refilling is the loop that shape was rejected for.
   **Judge the recurrence, in addition to the disposition — you are the judge,
   and nothing upstream ruled.** That framing is the **general** contract rather
   than this stage's local one — every session that judges a recurrence is obliged
   to stamp it, and this drain is only its mechanized instance (§The committed gap
   inbox owns the rule and what a stamp outside a drain owes instead). Your
   obligation here is undiminished by that reach.
   A slug appearing in a bullet is an *input*, never
   a verdict; the question is whether the finding **re-occurred** or is merely
   being cited, corrected, or argued against, and the bullet's prose is your
   grounds (the filer was asked at capture to write the claim there). State the
   call in the close commit message beside the bullet's disposition — a
   **declined** match stated as explicitly as a stamped one, since an unstated
   decline is indistinguishable from an unread bullet. On a judged recurrence,
   append the bullet's date to that entry's `recurrence:` declaration (creating
   the line when absent), **in this same queue-writing commit** — never *instead
   of* the bullet's own disposition. That same-commit rule is load-bearing rather
   than housekeeping: the declaration is no longer re-derivable, so this commit
   *is* the audit artifact, carrying the judgment beside the prose it was made
   from. It is idempotent per (slug, date): a slug already carrying today's date
   gains nothing. A slug that resolves only in the done section is not a
   recurrence — the finding recurred after its fix landed, which is a new defect,
   and it files as one.
3. **Sweep the inbound triage surfaces** — run
   `bash gate-sdk/bin/run-gates.sh --emit close-surfaces` and
   disposition every row (§The close-surface roster). The roster is derived, not
   enumerated here or in the binding below: a `forced=` row has a structural
   forcing function and cannot be skipped silently; an `advisory` row may be
   skipped, but the skip is a judgment to state, not an omission to leave
   invisible. An `(undeclared)` row is a capture surface nobody declared — file
   the missing declaration rather than reading past it.
   *<housekeeping: your housekeeping sweeps beyond the roster: deprecation scan,
   gate-runtime budget check, backlog-aging / premise-rot review, and the
   per-surface triage procedures the roster's rows route to.>*
4. **Clear Done.**
5. Review top-level docs for staleness (*is it still true?*). Same
   gap-generalization obligation as step 1, per staleness actually found:
   name the check class that should have caught it, and file the missing
   check as a deferred task or state in one line why no scanner is
   buildable — a silent fix forfeits the check.
6. **Runtime-artifact lifecycle check** — any gitignored/runtime artifact
   introduced this iteration (log, cache, scratch dir) has a named cleanup
   trigger: a write-path needs a paired reclaim-path. For a workflow-directory
   artifact the roster already answers it: the `reclaim=` field is that named
   trigger, and `check-close-surfaces` blocks a capture-tier declaration without
   one. What stays a judgment here is the artifact *outside* that directory.
7. **Release disposition** — run after the surface-mutating steps above and
   **before** the brevity pass (the disposition note is itself such a write).
   Every close dispositions the iteration at the release boundary: read the
   consumer's release policy (the `release-policy` slot below) and either
   execute its release procedure or stamp an explicit no-release line. Silence
   is not a disposition — a close that says nothing about release is incomplete.
   Stamp one line into the consumer-named disposition-evidence file:
   `<iteration> release <version|none|deferred:<version>> — <one-line basis>`
   (the `check-lesson-disposition` contract shape at the release boundary, the
   same lineage release-sweep's stamp follows) — `<version>` is the tag applied,
   `none` states the both-None (or consumer-equivalent) outcome, and
   `deferred:<version>` records criteria that were met while the release was held
   back, flooring the next qualifying note at that version. A consumer's release
   policy may make deferral the default disposition, so this third form is the
   common one under such a policy rather than an exception. Ordering: a tag
   names a commit, so the tag-and-host-release half of the procedure runs *after*
   the iteration's final commit lands; the note-authoring and stamp halves ride
   the close commits themselves.
   *<release-policy: the consumer's release procedure and criteria source by
   citation, the disposition-evidence path, and any boundary-only sub-procedures
   (e.g. a major-only deprecation sweep); or a plain "no release process — every
   iteration stamps none" line for a consumer without one.>*
8. **Brevity pass on the always-loaded surfaces** — run this **last**, after
   every surface-mutating step above. Scope by principle, not a fixed list:
   every surface injected into each agent session. Staleness asks *is it
   still true?*; brevity asks *is each block worth its standing per-session
   token cost?* — reword/delete over annotating; outdated context goes to git
   history. On-demand files (specs, this skill) are exempt — their cost is
   paid only when opened.
9. **Optionally merge** — an iteration can close without merging if validate
   is incomplete or a follow-up iteration is planned.
