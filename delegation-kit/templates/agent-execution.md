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

- **Supervisor owns rulings; agents surface, never guess.** SECURITY and design
  rulings (e.g. a privileged caller set, a naming collision) are decided by the
  supervisor and handed down; an agent that hits anything its brief doesn't cover
  stops and reports rather than choosing.
- **Background + notification, never poll.** Always `run_in_background`; wait for
  the completion notification; do not read the output file. Backgrounding keeps
  the main loop free to redirect.
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
- **One commit per unit, sized to finish within budget.** Each unit gets its own
  commit (+ a `[blocked-by: prior]` tag where ordered). A unit that investigates
  long before its first commit is the only thing an interrupt can destroy —
  split it. Trigger to split: >4 components, OR mixes mechanical + architectural
  work, OR >300 tool calls estimated.
- **Gate-driven worklist where one exists.** Drive the sweep from the gate's
  output, so an interrupt loses only the in-flight uncommitted unit and a fresh
  session re-runs the gate to resume.
- **Resume journal — agent writes, supervisor deletes.** The agent `Write`s a
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
  journal makes `DONE` lie about recoverability. **Agents have no `rm`, so the
  supervisor deletes the journal at the post-commit validation checkpoint.** A
  journal still present *without* a `DONE` marker means that unit was
  interrupted — resume from it. **Caveat — a background agent's sandbox may block
  the journal write.** `run_in_background` agents have been observed unable to
  `Write` to the granted path and silently falling back to returning findings in
  their final message — which makes the journal mechanic non-functional exactly
  when it matters (a long, interruptible run). So: for a **read-only fan-out**
  (audit, survey), the return value *is* the contract — don't rely on a journal.
  Reserve the journal / worktree isolation for agents that **mutate files**, and
  for those grant the journal path explicitly before dispatch rather than
  assuming the write succeeds.
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
  needs — and a standing choice lands in a tracked agent-type definition, never
  per-dispatch habit.
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
