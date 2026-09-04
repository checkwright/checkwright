# delegation-kit

Safe delegated-`Agent` execution for budget-bounded sessions. A supervisor
dispatches sub-agents that are cheap to spawn and expensive to trust; this kit
packages the supervisor-side protocol plus the two mechanizable pieces — a
trustworthy budget verdict (`usage-verdict`) and a commit-shape gate over gate
tampering (`check-gate-tamper`).

Why: three failure surfaces dominate delegation. **Shared mutable state** — two
committing agents race the git index and one sweeps the other's staged files
under the wrong message (the index and HEAD are shared for *every* committing
agent, disjoint source files notwithstanding). **Interrupted long units** — a
usage-window wall fires mid-flight and the uncommitted investigation dies with
the session. **Untrustworthy self-reports** — a sub-agent's "passed" claim, or
a gate quietly weakened to make its commit pass. The protocol closes all three;
`usage-verdict` and `check-gate-tamper` are its mechanical floors. See
[SPEC.md](SPEC.md) for the full contracts.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Register the gates — add to your `gates.list`:

   <!-- gate-roster:begin -->
   ```
   check-gate-tamper
   check-rule-citation   # holds SPEC §The delegation model's rule citations to the template
   check-agent-tier-explicit   # every tracked agent definition states a model: tier
   ```
   <!-- gate-roster:end -->

   They resolve through gate-sdk's registry path and their `# graph:` manifests
   put them in the generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh
   --write`.

2. Bind the protocol skill and add its resident pointer:
   - Create `.claude/commands/agent-execution.md` as a binding shim naming
     `templates/agent-execution.md` and binding its two slots — the shared-file
     roster and the validate battery (SPEC §One template, a resident pointer).
   - Add a `### Agent execution` block to your `CLAUDE.md`: your delegation
     pre-authorization sentence plus the `/agent-execution` pointer. The
     protocol loads behind that trigger, so the block stays a pointer, never a
     digest.

3. Wire a `usage.txt` producer so `usage-verdict` has a snapshot to read — point
   your harness `statusLine` at `bash gate-sdk/bin/run-gates.sh --statusline`,
   point `DELEGATION_KIT_REFRESH_CMD` at
   `bash gate-sdk/bin/run-gates.sh --usage-poll` so every
   verdict call refreshes the snapshot on demand (or wire the same poller under
   a timer), keeping it fresh while a supervising session sits static, or
   have any producer honour the snapshot contract (SPEC §The usage.txt contract).
   The statusline producer also renders a status bar, whose field set is
   SPEC §The statusline arm's; the three mandatory snapshot lines are the
   floor, and supplying the optional weekly keys arms the second (7-day) pause axis.

4. Optional — wire the Agent budget guard: register
   `bash gate-sdk/bin/run-gates.sh --hook agent-budget-guard` under `PreToolUse`
   matcher `Agent` in `.claude/settings.json`. There is nothing to copy: the
   guard is a binary arm, and the `command` field names it directly. It fires `usage-verdict` at every dispatch, blocking
   on a PAUSE verdict and advising otherwise (SPEC §The delegation model).
   Unwired, it is inert.

5. Optional — wire the turn-end liveness hook: register
   `bash gate-sdk/bin/run-gates.sh --hook subagent-stop-liveness` under
   `SubagentStop` in `.claude/settings.json` (that event takes no matcher).
   It logs one line per subagent turn end saying whether any launch record named
   a live producer, and **refuses the turn end** — exit 2, its stderr the
   blocking reason — when that reading is `red`, `corrupt` or `unresolved`; it emits no hook
   JSON on either path (SPEC §The turn-end liveness hook). There is
   no knob: unwiring is the valve. Unwired, it is inert; wiring it is a
   permission-surface change, so it is yours to make.

6. Optional — retune: copy `templates/delegation-config.sh` into your gates dir
   and override the budget thresholds, the gate-file globs, or the meta-layer
   prefixes. Defaults are this repo's single-operator layout.

## Use

```bash
bash gate-sdk/bin/run-gates.sh --usage-verdict            # budget verdict: exit 0 OK/RESET-OK, 1 PAUSE, 2 STALE
bash gate-sdk/bin/run-gates.sh --usage-verdict <snapshot> # verdict for an explicit usage.txt (test injection)
bash delegation-kit/bin/usage-trend.sh              # footprint trend over the sample log (needs DELEGATION_KIT_USAGE_HISTORY)
bash gate-sdk/bin/run-gates.sh --wait-probe sweep          # wait-primitive probe: the harness-uninvolved reproducer (sleeps for its declared sweep)
bash gate-sdk/bin/run-gates.sh --wait-probe report         # classify the recorded trials and print the verdict (exit 1 when none are)
```

With `DELEGATION_KIT_USAGE_HISTORY` set, `usage-verdict` logs one sample per
call; `usage-trend` reads that log and reports per-window footprint evolution
and weekly headroom (advisory — exit 0/2, never a pause verdict).

`check-gate-tamper` runs from the pre-commit hook and the battery; invoke it
directly with `--fixture <dir>` only for testing.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh delegation-kit/gate-tests delegation-kit/checks  # every gate's fixture pair
```

The `usage-verdict` decision table and the `usage-trend` assertions are no
longer shell runners: they retired into the gate binary's crate test lane, where
they read the same `usage-tests/` fixtures off disk and spawn the two shell
subjects (SPEC §Testing).
