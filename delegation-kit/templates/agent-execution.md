CONSUMER BINDING — create `.claude/commands/agent-execution.md` as a binding
shim naming this template (a header `Execute the template at <path>, applying
the bindings below.` then a `## Bindings` section) and bind its two slots — the
shared-file roster and the validate battery. The full delegated-`Agent`
execution protocol (any stage, any purpose — an audit, a build sweep, a one-off
investigation). A resident pointer in CLAUDE.md §Agent execution keeps the
protocol reachable from a delegating session; this skill is the complete
procedure.

For a **deletion, rename, or heavy cross-spec audit** dispatch, also load the
mechanical pre-flight in [dispatch-checklists.md](dispatch-checklists.md) — a
reach-through, not a change to this protocol; every rule below still applies.

Two bullets below — **Background + notification, never poll** and **Findings you
will act on are durable before you act on them** — bind a dispatched role that
fires no trigger loading this template, so each is also stated as a bare
imperative in the consumer's always-loaded agent definition under
delegation-kit/SPEC.md §Operative residency. That copy is sanctioned rather than
drift: do not delete it on sight, and when either rule changes here, propagate.

- **Supervisor owns rulings; agents surface, never guess.** SECURITY and design
  rulings (e.g. a privileged caller set, a naming collision) are decided by the
  supervisor and handed down; an agent that hits anything its brief doesn't cover
  stops and reports rather than choosing.
- **Background + notification, never poll.** Always `run_in_background`; wait for
  the completion notification; do not read the output file. Backgrounding keeps
  the main loop free to redirect. Backgrounding survives a **turn**, not a
  **session**. A supervisor's turn ends and its session lives on, so a
  backgrounded dispatch wakes it by notification — that is the sanctioned
  pattern just stated. A **dispatched agent's** turn end *is* its session end,
  and what that reliably kills is the **observer, not the work**: an `Agent`
  child is reaped with its parent, but a shell `run_in_background` child
  survives — orphaned, still writing, with nothing left able to read it. The
  survivor is the worse case, because it keeps mutating shared files while the
  next actor moves against them. Either way the caller gets no result, so a
  dispatched agent **never ends a turn on work still running** — it awaits its
  own long-running work to completion, however long, and reports only results
  it holds. Awaiting, not "dispatch in the foreground": foreground is a choice
  for a shell command and not reliably one for an `Agent` dispatch, which the
  harness may detach whether or not backgrounding was requested. And not by
  ending the turn to wait for the completion notification — that is the one act
  that revokes the channel, which is how a *careful* reader reaches this
  failure. Wait **in-turn on a condition** instead: loop on the work's own
  artifact (evidence file, lock, exit marker) with the harness's waiting
  primitive. Never-poll governs waiting on an `Agent`'s completion notification,
  a channel a supervisor has; condition-waiting is the only one a dispatched
  role has.
- **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide otherwise.**
  Agents that edit a shared file (see the roster below) run one at a time —
  dispatch, await notification, validate, dispatch the next. Independent
  read-only units may run ≤`DELEGATION_KIT_FAN_WIDTH`-wide. **The git index and
  HEAD are shared files for every agent that commits**, independent of
  source-file disjointness: two committing agents racing `git add`/`git commit`
  interleave and one sweeps the other's staged files under the wrong message. So
  agents that each commit must be **serialized** *or* run under
  `isolation: worktree` (own index); reserve the unlocked
  ≤`DELEGATION_KIT_FAN_WIDTH`-wide bound for **read-only** fan-outs. "No lockfile
  churn" is a false safety signal — the index is shared regardless.
  Serialization is by **completion**, not by notification: the next dispatch
  begins after the previous one has *returned and been validated*. A supervisor
  reaches that point via the completion notification; a **dispatched agent**
  reaches it in-turn, awaiting each child before dispatching the next (it cannot
  end its turn to wait — see the backgrounding rule). The ordering requirement
  is identical for both; only the waiting mechanism differs, and neither role
  may overlap two agents on a shared file or the git index.
- **One commit per unit, sized to finish within budget.** Each unit gets its own
  commit (+ a `[blocked-by: prior]` tag where ordered). A unit that investigates
  long before its first commit is the only thing an interrupt can destroy —
  split it. Trigger to split: >4 components, OR mixes mechanical + architectural
  work, OR >300 tool calls estimated.
- **Gate-driven worklist where one exists.** Drive the sweep from the gate's
  output, so an interrupt loses only the in-flight uncommitted unit and a fresh
  session re-runs the gate to resume.
- **Resume journal — agent writes, scratch reset sweeps.** The agent `Write`s a
  running progress journal (findings triaged, edits applied, what remains) to a
  repo-local gitignored scratch dir in the main checkout — `.tmp/` here — never a
  temporary worktree (deleted with the worktree) nor a system temp dir (a restart
  wipes it); name the path **absolute into the main checkout** in the dispatch
  prompt so a worktree-isolated agent writes to the surviving tree, not its doomed
  one. Repo-dir-local scratch is reboot-survivable, cheap to clean, and predictable
  across coding agents. The agent updates it as it goes; on success it appends a
  `DONE` marker. Each finding is
  written into the journal *inline as it is confirmed* — never "see final
  output": the agent's return message dies with the session, so a pointer-only
  journal makes `DONE` lie about recoverability. **Nobody deletes a journal
  while the work it covers is live** — not the agent (no `rm`) and not the
  supervisor, whose deletion would take the file out from under a resumed agent
  and destroy its own pull channel mid-unit. Cleanup is the consumer's scratch
  reset at its next work-unit boundary, not a chore in this protocol. `DONE` is
  the completion marker and counts **only as the file's last line**: a session
  resumed past its own marker appends after it. A journal *without* a `DONE`
  marker signals interruption only in a **cold read**
  — the supervisor found a journal but never consumed the agent's return (its
  session died before returning); there, no `DONE` = interrupted, resume from
  it. On the ordinary path the supervisor consumed the return and ran its
  post-commit verification, which *is* the completion attest, so the missing
  marker is redundant, not a signal of interruption. Since every journal now
  outlives its session, presence alone signals nothing — tell a live journal
  from a spent one by its per-session name, the work cursor, and `git log`
  (delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset
  sweeps). **Caveat — a background agent's sandbox may block
  the journal write.** `run_in_background` agents have been observed unable to
  `Write` to the granted path and silently falling back to returning findings in
  their final message — which makes the journal mechanic non-functional exactly
  when it matters (a long, interruptible run). So: for a **read-only fan-out**
  (audit, survey), the return value *is* the contract — don't rely on a journal.
  Reserve the journal / worktree isolation for agents that **mutate files**, and
  for those grant the journal path explicitly before dispatch rather than
  assuming the write succeeds. A contract has two ends, though: the child owes
  no journal, and the **parent** owes the durable landing of what it received
  before acting on it (next bullet). The caveat is about a backgrounded child's
  write failing silently, so it never licensed the parent — which has already
  demonstrated it can write, being the session that granted the path.

- **Findings you will act on are durable before you act on them.** A child's
  return value lives only in your context, and your context dies with your
  session. Before you *act* on a dispatched agent's findings — edit against
  them, rule from them, plan the next wave on them — land them somewhere that
  survives you: a commit, or your own journal. The write is the parent's,
  discharged on receipt, not a chore delegated back to the child. This binds
  whoever holds findings they will act on, at any depth. It costs one write per
  returned child, not a running narration. The obligation is **durability**, and
  committing and journalling are two ways to discharge it — a session uses
  whichever is available to it. A dispatched session journals, to the path it was
  granted, because it cannot commit. A top-level session commits, because it can.
  A top-level session that *cannot* commit right now — another session is holding
  the shared index, the normal condition while an iteration is running — journals
  for as long as that holds, and discharges by committing when the index frees.
  Neither the supervising role nor a session running outside any dispatch is
  exempt: the axis is what the session can do at this moment, never what it is.
- **Validate after every agent commit** — a sub-agent's "passed" claim is not
  trustworthy. Re-run the relevant gates (the sweep's own gate) and the consumer
  validate battery below. **Diff every gate change in an agent commit before
  accepting** — a gate modification inside a feature commit is a supervisor-owned
  ruling; an agent blocked by a gate will weaken it (a false exemption) to make
  its commit pass instead of fixing the code. "The gate is in my way" almost
  always means the code doesn't fit the convention — fix the code.
  `check-gate-tamper` is the mechanical floor under this bullet — it blocks the
  two attested shapes (a gate edit co-staged with product code; a new path/glob
  exemption co-staged with the file it excuses) — but it does **not** catch
  semantic weakening inside a legitimate scripts-only commit, so the by-eye diff
  review remains your duty (delegation-kit/SPEC.md §Validate after every agent
  commit — the honest limit).
- **Budget-check before *each* dispatch in a fan-out**, not once at the start.
  `bash delegation-kit/bin/usage-verdict.sh` (verdict exit 0/1/2 from `usage.txt` —
  it folds in the reading-age and window-validity checks so a dead-window pct
  can't read stale-high; a PAUSE names its axis — a 5h wall clears in hours, a
  7-day wall in days). If the verdict is PAUSE and the work is large, pause for
  reset. Width is the kill axis: the 5h wall fires mid-flight, and the agents
  that bank are the ones that *finished* — ≤`DELEGATION_KIT_FAN_WIDTH`-wide
  bounds the loss to the in-flight wave.
  **Project the next wave's burn from the last wave's, not just the current
  pct** — a read-heavy `DELEGATION_KIT_FAN_WIDTH`-wide sweep is far more
  window-expensive than its
  subagent-token total suggests, so size waves to leave the next one headroom, or
  accept one wave per window.
  **The projection reaches downward as well as forward.** The guard re-arms at
  every depth — a dispatched session's own dispatches fire it too — but each
  verdict prices **one call**, never the subtree that call opens. Nothing
  between a dispatch and its children's children ever sees the whole. So budget
  a dispatch you expect to fan out for its **subtree**, not its own turn, and
  read a child's fan-out as your spend even though its verdicts land in a
  context you never see.
  **Do not read live budget off the statusline while parked on a dispatch.**
  Observed: the harness statusline does not refresh while the main session waits
  on a background agent's completion notification, so its displayed budget
  freezes for the length of the dispatch — the longer the unit, the staler the
  number. Poll `usage-verdict.sh` externally when you need a live reading
  mid-dispatch (`watch -n 30 bash delegation-kit/bin/usage-verdict.sh`); this is
  the one sanctioned poll and it does not weaken the never-poll rule above,
  which governs waiting for *completion*. Enforcement is unaffected either way:
  the per-dispatch budget check reads a fresh verdict at dispatch time, so a
  pause/proceed decision is never made off the frozen display — only passive
  display is stale.
- **Match the dispatched model and effort to the unit's shape.** Delegation
  levers tokens only when selection follows the work: a read-heavy or mechanical
  unit (audit, survey, rename/merge sweep) rides a cheaper model class via the
  dispatch's model parameter or a dedicated agent type; a unit carrying design
  judgment stays on the supervisor's class. Derive the class ladder from the
  harness's **live model roster at dispatch time** — model families churn faster
  than kit text, so a baked model-name list in any doc is drift by construction.
  Agent-**type** selection derives the same way — from the dispatch-time
  agent-roster descriptions, not a baked list: an audit or survey rides the type
  whose description commits to review work, never one that disclaims it (an
  excerpt-locator serves pure search, not audit). Selection sits with the
  dispatching session — its context holds what selection
  needs — and it is **affirmative**: an unselected dispatch does not fall back
  to a cheap default, it **inherits the dispatcher's tier**, so declining to
  choose silently buys the most expensive tier in reach — and buys it precisely
  for the read-only fan-outs that are the cheapest work you dispatch. That is
  why a standing choice lands in a tracked agent-type definition rather than
  per-dispatch habit, and why an omitted `model:` field there is not a neutral
  default but the inherit default: the field is stated even when the answer is
  to inherit.
- **Never revert substantial completed work on your own design judgment** —
  especially an expensive delegated sweep. Surface the tension and wait for the
  explicit go-ahead before discarding it: a self-judged revert forfeits the
  sweep's whole spend and pays it again on the re-run.

## Shared-file roster

*<shared-file-roster: the files two agents must never edit concurrently, in
addition to the git index and HEAD (always shared for committing agents) — your
generated-config scripts, shared test-environment fixtures, and any amendment
file under active edit.>*

## Validate battery

*<validate-battery: the command set the supervisor re-runs after every agent
commit, never the agent's self-report — for a gate-sdk consumer the
`run-gates.sh` battery plus the fixture runner for each kit the sweep touched; a
toolchain consumer adds its compile/lint/test set, and after any rename sweeps
explicitly for zero-byte files a stale build cache would green.>*
