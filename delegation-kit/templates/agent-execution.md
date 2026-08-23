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
  artifact (evidence file, lock, exit marker). Never-poll governs waiting on an
  `Agent`'s completion notification, a channel a supervisor has;
  condition-waiting is the only one a dispatched role has.
  **Which primitive — the one whose wait ends when the condition goes true, not
  when a duration expires.** The harness offers two forms with different
  reactivity, and the difference is exactly the wall-clock property this rule
  exists to buy. **Background a command that *exits* on the condition** — a
  `run_in_background` shell command wrapping `until <cond>; do sleep N; done` —
  and it fires one notification the moment the condition holds, then ends: that
  is the sanctioned form, and it holds that place on measurement rather than on
  preference (delegation-kit/SPEC.md §bin/wait-probe). The harness's
  **event-stream form**, armed with a command and a deadline, emits one event per
  occurrence and stays armed to its deadline after the event fires **when the
  command it was armed with is unbounded** — a `tail -f`, a `while true` — so
  pointed at a single completion by way of such a command it converts a wait that
  should end in seconds into one that ends at the timeout. Armed with a command
  that *exits*, that watch ends too: it is the second choice for a single
  completion, not a broken one, and it is named here because a reader told only
  about the first form reaches for this one the next time the shape looks close.
  Never a bare foreground `sleep`, which ends on neither.
  **Get the loop's polarity right — the two rules in this bullet compose into a
  trap, and it is the one that has actually fired.** `until` ends when its
  condition becomes **true**, so `until` takes a *done* predicate:
  `until [ -f <marker> ]; do sleep N; done`. Liveness is a *still-running*
  predicate, so it takes `while`: `while kill -0 "$pid" 2>/dev/null; do sleep N;
  done`. Composing "wrap `until <cond>`" with "loop on the recorded PID's
  liveness" literally gives `until kill -0 "$pid"`, whose condition is true while
  the producer is **alive** — so it exits at once. **The tell** is a waiter that
  exited **zero**, in milliseconds, with its producer still running and its
  condition never met. Four attested instances read as evidence against this
  primitive until they were measured and found to be this instead; neither keyword
  costs anything at the guard, which reads the loop's span and not its keyword.
  **What you wait *on* splits two ways, and the split is what makes the artifact
  reachable.** An **`Agent` dispatch** is awaited by its **completion
  notification** — that record is the harness's, not the session's — so never go
  looking for it on disk. A **shell child** has no notification channel of its
  own, so it is awaited on an artifact *the session placed*, and that artifact is
  named: **record the child's PID at launch**, one line in the form
  `pid=<n> run=<key>`, in a file named **`<key>.run`** — the same `<key>`, so the
  name carries no field the line does not — written where the **Resume journal —
  agent writes, scratch reset sweeps** bullet below sends a journal, repo-local
  gitignored scratch in the main checkout, on that bullet's stated survivability
  grounds — widened here from the journal to any artifact a session waits on.
  That record *is* the wait target: loop on its PID's liveness —
  `while kill -0 "$pid" 2>/dev/null; do sleep N; done`, on the polarity rule above
  — and the loop ends when the PID stops answering. Write it at the launch, not after, and leave it
  behind — a completion marker answers *is it done* to a live observer, and the
  case this rule exists for is the one where the observer is gone, where a
  liveness record still answers *is it still running* to whoever arrives next.
  Whoever that is reads it with `check-producer-liveness <record>` — or, because
  the `.run` suffix makes the set a glob rather than a path someone must be told,
  `check-producer-liveness <scratch-dir>` over all of them at once — unchanged
  and already wired (evidence-kit/SPEC.md §check-producer-liveness), which is why
  the record is in that gate's grammar rather than a shape of your own.
  **Delete the record when its producer is done, and never before.** While one
  names a live PID, a guard blocks every `git` command that writes the index, the
  worktree or a ref (guard-kit/SPEC.md §The generic ruleset, rule 14) — a commit
  taken under a live producer dirties the tree it is measuring and costs that
  run's whole verdict. Deleting a record whose producer has exited is not a way
  around the block: it is a statement of fact that has become false being
  retracted.
  **And never by pattern-matching the process table.** `pgrep -f '<pattern>'`
  matches the waiter's own argv — the harness's wrapper argv matches too — so
  `until ! pgrep -f '<script>'; do …; done` can never go false and never exits,
  reddening nothing while it burns the whole foreground cap. Where liveness *is*
  the condition, match a **recorded PID** (`while kill -0 "$pid" 2>/dev/null`) and never a pattern —
  one rule, whoever started the producer, your own backgrounded child included: a
  PID is an identity, a pattern is a guess about a process table that includes the
  guesser. The
  bracket-trick repair (`pgrep -f '[r]un-smoke.sh'`) is **not** the sanctioned
  form, and its grounds for being refused are
  delegation-kit/SPEC.md §The delegation model's.
- **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide otherwise.**
  Agents that edit a shared file (see the roster below) run one at a time —
  dispatch, await notification, verify, dispatch the next. Independent
  read-only units may run ≤`DELEGATION_KIT_FAN_WIDTH`-wide. **The git index and
  HEAD are shared files for every agent that commits**, independent of
  source-file disjointness: two committing agents racing `git add`/`git commit`
  interleave and one sweeps the other's staged files under the wrong message. So
  agents that each commit must be **serialized** *or* run under
  `isolation: worktree` (own index); reserve the unlocked
  ≤`DELEGATION_KIT_FAN_WIDTH`-wide bound for **read-only** fan-outs. "No lockfile
  churn" is a false safety signal — the index is shared regardless.
  Serialization is by **completion**, not by notification: the next dispatch
  begins after the previous one has *returned and been verified*. A supervisor
  reaches that point via the completion notification; a **dispatched agent**
  reaches it in-turn, awaiting each child before dispatching the next (it cannot
  end its turn to wait — see the backgrounding rule). The ordering requirement
  is identical for both; only the waiting mechanism differs, and neither role
  may overlap two agents on a shared file or the git index.
- **Never dispatch a fork to narrow a child.** A fork is wrong exactly when the
  dispatch's purpose is that the child does *less* than the parent: a fork
  inherits the parent's context, toolset and model tier whole and disclaims
  nothing, so every narrowing survives only as a sentence in the prompt. Three
  dispatching contexts, each with its own grounds. A **read-only audit or
  survey** wants narrower *authority* — the fork carries the parent's full
  reach, so the restriction is a request, and one such fork completed a whole
  stage, commits included. A **tier-split oracle** wants narrower *cost* — the
  fork inherits the dispatcher's model, which is exactly the split the posture
  exists to make (a lead posture that dispatches its own oracle is the named
  instance). A **rule-injection dispatch** wants a *different* brief — attested
  where five sibling forks each carried a line explaining that the roster's
  agent definitions do not carry it, so fork was chosen to inject rules the
  types lack and every one still paid to re-materialize its parent's context.
  The boundary, stated honestly: a fork stays correct where the child does the
  **same job at the same authority** and only parallelism or isolation is
  wanted. The dispatch guard blocks there too, because a hook cannot read
  intent and no sanctioned fork use exists in this doctrine — the valve is
  unregistering the hook, never a per-dispatch knob, which would restore the
  honour system the rule replaced (delegation-kit/SPEC.md §The delegation
  model). Grounds measured rather than asserted: in one audited iteration ten
  of twenty-four dispatched agents were forks, carrying 20.90 USD of 158.70 USD
  of priced burn.
- **A read-only claim is made by isolation, not by sentence.** A brief saying
  "read-only, no edits" makes no claim: a subagent inherits its toolset from its
  **type**, never from the instruction text. The claim is made by
  `isolation: worktree`. Take it for an agent that *will* commit — an **own
  index**, the **Serialize on shared files** rule above — and for any agent
  *claimed* read-only, as **write confinement**. Grounds for this rule and the
  next: delegation-kit/SPEC.md §The delegation model.
- **Isolation charges three harness costs, and paying them is the parent's job.**
  **(1) The worktree's base is configuration, and the default is not HEAD.**
  `worktree.baseRef` selects it: `fresh` (the default) branches from
  `origin/<default-branch>`, `head` from the dispatcher's local HEAD. Set it
  deliberately, and pin it, before you design around it. Under `fresh` a child
  reads the *pre-change* tree: read your exposure with
  `git log origin/<default-branch>..HEAD` before dispatching. Naming a commit in
  the prompt does not make the checkout that commit, so state the rev and have
  the child verify it with `git rev-parse HEAD` and read every target with
  `git show <rev>:<path>`. **A child whose target is unreadable at that rev stops
  and says so**, and never falls back to the dispatch prompt's own paraphrase of
  it. **(2) The worktree lands inside the repo and untracked**, so an in-flight
  isolated agent reads as a dirty tree and aborts every clean-tree precondition —
  a consumer smoke, a packaging step, any commit. Gitignore the path, and reap
  agents at the boundary with `git worktree list` rather than off `git status`.
  **(3) An isolated child sees only committed state.** Untracked and gitignored
  files are in no commit, so no base ref reaches them and naming a rev does not
  help. **A sweep whose corpus includes an untracked surface is not delegable to
  an isolated agent**: read that surface yourself, or pass its content in the
  prompt.
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
  Reserve the **journal** for agents that **mutate files**, and for those grant
  the journal path explicitly before dispatch rather than assuming the write
  succeeds. **Worktree isolation is not reserved with it** — the two answer
  different questions, and bundling them read as *a read-only fan-out needs no
  isolation*, which is the reading that licensed an unreviewed commit on the
  shared branch. A read-only fan-out takes isolation precisely *because* it is
  claimed read-only (the **A read-only claim is made by isolation, not by
  sentence** rule above). A contract has two ends, though: the child owes
  no journal, and the **parent** owes the durable landing of what it received
  before acting on it (next bullet). The caveat is about a backgrounded child's
  write failing silently, so it never licensed the parent — which has already
  demonstrated it can write, being the session that granted the path.
- **A child's only upward route is a durable artifact.** Probed, not assumed: a
  child's sends to an invented dispatcher name and to its own agent type both
  failed, only a send addressed to the **top-level session** arrived, and
  neither level knows its own identity or its parent's. There is therefore no
  address a child can use to reach the agent that dispatched it, and four
  dispositions follow. **(1) A durable artifact, for anything mid-run** — the
  dispatcher mints a path, names it **absolute into the main checkout** in the
  prompt, and the child writes as it goes. That is already the journal rule
  above; it is now also the *only* upward route there is, and it is stronger
  than a message would be, because a file survives a child that dies before
  returning. A **stop signal takes the same shape** — a sentinel the child
  checks — so it is cooperative by construction and a wedged child stays out of
  reach. **(2) Return-value only, otherwise** — a fan-out needing no mid-run
  channel needs no artifact, which is the read-only carve-out above. **(3) The
  downward half is a dispatch-shape rule** — a dispatcher that wants control
  over its fan-out dispatches in the background and keeps the handle; what
  actually failed was a dispatcher blocked on a foreground dispatch, holding
  neither the handle nor a turn. **(4) Naming the address a child should use is
  refused** — there is none, and the refusal is stated rather than omitted
  because it is the first thing both original filings reached for. Two
  corollaries defeat the workarounds a reader would otherwise invent: a message
  that does reach the top-level session is attributed to the sender's **type**,
  so a fan-out wider than one is ambiguous at the receiving end; and a child
  resumed by a message from its dispatcher still cannot reply, so being
  addressed creates no return path. One live hazard rides with them: the
  message and stop tools may be **deferred** for a dispatched agent — callable
  only after a discovery call it has to know to make — so a child that needs to
  escalate may not hold the channel at all. Hence the obligation, which is the
  part that binds you: **a dispatcher handing out work that will need a mid-run
  channel grants a durable path with it**, and reads that path itself.

- **A finished child is addressed by the task-id its completion notification
  carried, never by its name.** Resuming a returned agent to correct or extend
  its work is the cheap alternative to a cold re-dispatch, and it is the one
  that keeps the child's own reasoning instead of paying to rebuild it. The name
  is not a durable address for that: measured here, a name live at dispatch
  resolved to *no agent of that name is reachable* once the dispatching session
  had been compacted, with the agent listing reporting none reachable at all,
  while the task-id from the notification resumed the same child from its
  transcript. Which of the two voids the name — the compaction, or the child
  merely having finished — is untested; the task-id survived both, and the
  notification states outright that it is the handle the child may notify under
  again, so it is the address to hold. It is also the address to **write down**:
  the notification lives in the context that dies, so a dispatcher who may want
  that child later records the task-id under the durability rule below rather
  than trusting the scroll to still be there.

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
- **Verify after every agent commit** — a sub-agent's "passed" claim is not
  trustworthy. Re-run the relevant gates (the sweep's own gate) and the consumer
  validate battery below — the act is `verify`, the artifact is the battery.
  **Never verify a unit by re-running the producer of what it delivered**: where
  the unit's deliverable *is* an evidence artifact, the verify is a **read** of
  the committed artifact, because re-running its sole writer overwrites or
  duplicates the record instead of checking it. Having already run the wrong
  check, read delegation-kit/SPEC.md §Verify after every agent commit for which
  of the two re-runs you are holding — only one of them leaves evidence to
  restore. **Diff every gate change in an agent commit before
  accepting** — a gate modification inside a feature commit is a supervisor-owned
  ruling; an agent blocked by a gate will weaken it (a false exemption) to make
  its commit pass instead of fixing the code. "The gate is in my way" almost
  always means the code doesn't fit the convention — fix the code.
  `check-gate-tamper` is the mechanical floor under this bullet — it blocks the
  two attested shapes (a gate edit co-staged with product code; a new path/glob
  exemption co-staged with the file it excuses) — but it does **not** catch
  semantic weakening inside a legitimate scripts-only commit, so the by-eye diff
  review remains your duty (delegation-kit/SPEC.md §Verify after every agent
  commit — the honest limit).
- **A child's citation is a pointer to verify, never evidence in itself.** A
  returned `file:line` is not checked by confirming the range resolves: that
  catches a wrong number and misses a wrong spelling, a moved passage, and a
  sibling instance the sweep never reported. **Re-read the passage** in the parent
  before acting on it. Both halves are attested here — a census that fabricated a
  `file:line` and a negative claim, both false; and a re-verification of nineteen
  agent-sourced cites that found no bad ranges yet still surfaced a marker cited
  under a spelling the tree does not use, plus an entire family member the census
  never mentioned. Quotation with attribution is precisely the signal a reader uses
  to decide a claim has already been checked, so an unverified one defeats normal
  trust rather than merely being wrong.
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
