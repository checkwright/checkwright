CONSUMER COPY — copy this into your skills directory as
`.claude/commands/agent-execution.md` and fill the two marked consumer sections
(the shared-file roster and the validate battery). The full delegated-`Agent`
execution protocol (any stage, any purpose — an audit, a build sweep, a one-off
investigation). The load-bearing safety rules are also resident in CLAUDE.md
§Agent execution so they are never absent from a delegating session (copy
`claude-md-agent-execution.md` there); this skill is the complete procedure.

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
  running progress journal (findings triaged, edits applied, what remains) to the
  harness session directory (never a system temp dir — a restart wipes it),
  updating it as it goes; on success it appends a `DONE` marker. Each finding is
  written into the journal *inline as it is confirmed* — never "see final
  output": the agent's return message dies with the session, so a pointer-only
  journal makes `DONE` lie about recoverability. **Agents have no `rm`, so the
  supervisor deletes the journal at the post-commit validation checkpoint.** A
  journal still present *without* a `DONE` marker means that unit was
  interrupted — resume from it. **Caveat — a background agent's sandbox may block
  the journal write.** `run_in_background` agents have been observed unable to
  `Write` to the session dir and silently falling back to returning findings in
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
  review remains your duty (delegation-kit/SPEC.md §check-gate-tamper honest limit).
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

<!-- ===== consumer section: shared-file roster — edit this ==================
The files two agents must never edit concurrently (in ADDITION to the git index
and HEAD, which are always shared for committing agents). List your build's
generated-config scripts, shared test-environment fixtures, and any amendment
file under active edit. Example shape:

    - deploy/scripts/gen-configs.sh
    - <your shared integration-test fixture>
    - the amendment file a unit is editing
===== end consumer section: shared-file roster =========================== -->

<!-- ===== consumer section: validate battery — edit this ====================
The command set the supervisor re-runs after every agent commit (never the
agent's self-report). For a gate-sdk consumer this always includes:

    bash gate-sdk/bin/run-gates.sh
    <the kit fixture runners for the gates this sweep touched>

A toolchain consumer adds its compile/lint/test set here, e.g. for a Cargo
workspace:

    cargo check --workspace --all-targets
    cargo clippy --workspace -- -D warnings
    cargo test  --workspace
    find <renamed-dirs> -name '*.rs' -size 0    # zero-byte corruption after renames

Renames are the trap: a stale build cache can green a tree with silently
zero-byted files — sweep for them explicitly.
===== end consumer section: validate battery ============================= -->
