The `build` (implementation) stage of an iteration. Implement the queued work
in queue order. Exit condition: task queue empty for this iteration.

Build is **mandatory every iteration** and processes **both feature and debt
units**: it merges a promoted feature's amendment into the canonical spec and
makes a debt unit's direct edits (a debt unit skips the authoring stage
entirely), and it **owns the task-completion Done move** for either kind. The
authoring stage authors amendments only — it makes no direct spec/code edit and
moves nothing to Done.

**Step 0 — audit-readiness recheck (before the stamp).** The
entry stamp re-fires the state-coupled gates but **not** your
spec-consistency battery (it couples the spec corpus, not the evidence file), so a
build session can enter on an unverified align corpus. Close that hop:
**iff** an `align` stamp for the current iteration exists in
`.workflow/WORKFLOW-STATE.txt`, run *<consistency-gate: your aggregate
consistency gate>* and refuse to stamp if it is red — fix the drift first (or return to
`align`). Absent an align stamp, align did not run this iteration; build's
prior stage is scope, whose exit the entry already re-fires — **except** when
`check-stage-entry` assertion C fires: a cross-component amendment signal with
no align stamp blocks the entry until the iteration carries either an align
stamp or an explicit `<iter> align-waived <session> <date>` waiver line,
written **only on the user's explicit ruling** — never self-issued by this
entering session.

**First step — stamp evidence.** Run lifecycle-kit's
`bin/enter-stage.sh build`: it appends `<iteration> build <session-id> <date>`
to `.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`; the
stamp proves invocation, not faithful execution), reading `<session-id>` from
`bin/session-id.sh`
(the newest transcript — never hand-picked), using `date +%F`, and refusing
(writing nothing) if `check-stage-entry` is red. On a refusal, **do not force
the entry** — escalate to the lead (where one exists and this is not a standalone
session) and stop; a refused entry is a gate verdict to resolve at its source,
never to override. That stamp *is* the
transition — the last stamp is the stage cursor, so nothing flips and no queue
write is involved. Commit the stamp on its own.

Build runs one fresh session per task: a fresh session rehydrates the
governing docs + queue state at full fidelity from disk, where an in-session
summary would lossily erode the approved plan and re-derived premises. Prefer
a session reset at a task boundary; reach for mid-task summarization only as
a fallback before a commit, never as the routine per-task reset.

**Every session still stamps** — re-run `bin/enter-stage.sh build` each
session: it appends a fresh `<iter> build <session-id> <date>` line with this
session's id, so WORKFLOW-STATE keeps the per-session audit trail
(`check-stage-evidence` tolerates multiple `build` stamps). A sibling stamp
naming `build` leaves the cursor where it already is, so there is no
once-per-stage write to coordinate — only a re-run within one session that
already stamped is reported as an idempotent no-op. Prefer committing the new
stamp standalone when the task will land as several commits, so the evidence
is durable rather than sitting uncommitted across a long session.

## Session ritual

*<ritual: your build ritual: pick the first unblocked task; read the spec for
every component the task touches and merge its amendments into the canonical
spec on completion; implement exactly what the spec describes — no features
beyond it; write tests as you go; present a plan for approval before code where
your process requires it; name your amendment-merge procedure owner and the
queue-entry grammar the task-completion move writes against.>*

Build-time question triage — who rules, and where the ruling lives: a
question *within the amendment's envelope* (calibration, mechanics) is ruled
in-session and written into the spec text being merged — a ruling that lives
only in conversation evaporates. A *change to the envelope* (narrowing or
widening asserted behavior, user-facing semantics) stops and surfaces to the
user — never pick a "conservative alternative" silently. A *cross-component
causal gap* is not a TODO: stop, resolve it this session, update the spec.

**Run the system; don't reason about it** — when a running system is
reachable, reproduce first, read second. Recorded evidence rots: a static
trace in a task body is a dated hypothesis to re-verify, not a premise. A
change carrying an artifact surface — a change whose real output is a deployed
or generated artifact — names that surface and exercises the artifact
deployment-faithfully before the stage exits; a green battery is tree-correct,
not artifact-correct (Oracle-first).
