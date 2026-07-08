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

1. Register the gate — add to your `gates.list`:

   ```
   check-gate-tamper
   ```

   It resolves through gate-sdk's registry path and its `# graph:` manifest puts
   it in the generated pre-commit hook: `bash gate-sdk/bin/gen-pre-commit.sh
   --write`.

2. Copy the protocol templates into place:
   - `templates/agent-execution.md` → your skills dir as
     `.claude/commands/agent-execution.md` (the full procedure).
   - `templates/claude-md-agent-execution.md` → a `### Agent execution` block in
     your `CLAUDE.md` (the load-bearing rules, always-loaded).
   - Fill the two marked consumer sections in both: the shared-file roster and
     the validate battery.

3. Wire a `usage.txt` producer so `usage-verdict` has a snapshot to read — copy
   `templates/statusline-usage.sh` as your harness `statusLine` command, or have
   any producer honour the snapshot contract (SPEC §The usage.txt contract).
   The three mandatory lines are the floor; supplying the optional weekly keys
   arms the second (7-day) pause axis.

4. Optional — wire the Agent budget guard: copy `templates/agent-budget-guard.sh`
   into your gates dir and register it under `PreToolUse` matcher `Agent` in
   `.claude/settings.json`. It fires `usage-verdict` at every dispatch, blocking
   on a PAUSE verdict and advising otherwise (SPEC §The delegation model).
   Unwired, it is inert.

5. Optional — retune: copy `templates/delegation-config.sh` into your gates dir
   and override the budget thresholds, the gate-file globs, or the meta-layer
   prefixes. Defaults are the extracted platform's single-operator layout.

## Use

```bash
bash delegation-kit/bin/usage-verdict.sh            # budget verdict: exit 0 OK/RESET-OK, 1 PAUSE, 2 STALE
bash delegation-kit/bin/usage-verdict.sh <snapshot> # verdict for an explicit usage.txt (test injection)
bash delegation-kit/bin/usage-trend.sh              # footprint trend over the sample log (needs DELEGATION_KIT_USAGE_HISTORY)
```

With `DELEGATION_KIT_USAGE_HISTORY` set, `usage-verdict` logs one sample per
call; `usage-trend` reads that log and reports per-window footprint evolution
and weekly headroom (advisory — exit 0/2, never a pause verdict).

`check-gate-tamper` runs from the pre-commit hook and the battery; invoke it
directly with `--fixture <dir>` only for testing.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh delegation-kit/gate-tests delegation-kit/checks  # check-gate-tamper fixtures
bash delegation-kit/bin/run-usage-tests.sh                                           # usage-verdict verdict table
bash delegation-kit/bin/run-budget-guard-tests.sh                                    # budget-guard action table
bash delegation-kit/bin/run-trend-tests.sh                                           # usage-trend segmentation assertions
```
