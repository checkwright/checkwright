# delegation-kit — safe delegated-agent execution for budget-bounded sessions

Delegated agents are cheap to dispatch and expensive to trust. Three failure
surfaces dominate: **shared mutable state** (two committing agents race the
git index and one sweeps the other's staged files under the wrong message),
**interrupted long units** (a usage-window wall fires mid-flight and the
uncommitted investigation dies with the session), and **untrustworthy
self-reports** (a sub-agent's "passed" claim, or a gate quietly weakened to
make its commit pass). The kit packages the supervisor-side protocol that
closes all three, plus the two pieces that are mechanizable: a trustworthy
budget verdict (`usage-gate`) and a commit-shape gate over gate tampering
(`check-gate-tamper`).

Extracted from the governance meta-layer of a private production platform.
The kit carries the protocol templates, the budget-verdict tool, and the
tamper gate; the consumer supplies its own validate battery, shared-file
roster, and toolchain checks — those are project rule content and never
ship.

## The delegation model

One **supervisor** session dispatches `Agent` tasks and owns every ruling;
agents execute briefs and surface anything the brief does not cover. The
protocol's load-bearing rules:

1. **Supervisor owns rulings; agents surface, never guess.** Security and
   design decisions are handed down in the brief; an agent that hits an
   uncovered case stops and reports.
2. **Background + notification, never poll.** Dispatch with
   `run_in_background`, wait for the completion notification, do not read
   the output file mid-flight — backgrounding keeps the supervisor free to
   redirect.
3. **Serialize on shared files; ≤2-wide otherwise.** Agents editing a
   shared file run one at a time. **The git index and HEAD are shared files
   for every agent that commits**, independent of source-file disjointness
   — committing agents are serialized *or* isolated in their own worktree
   (`isolation: worktree` — own index); unlocked ≤2-wide is reserved for
   read-only fan-outs. "No lockfile churn" is a false safety signal.
4. **One commit per unit, sized to finish within budget.** A unit that
   investigates long before its first commit is the only thing an interrupt
   can destroy — split it. Split triggers: >4 components, OR mixed
   mechanical + architectural work, OR >300 estimated tool calls.
5. **Gate-driven worklist where one exists.** Drive a sweep from the gate's
   own output: an interrupt loses only the in-flight unit, and a fresh
   session re-runs the gate to resume.
6. **Resume journal** (below) for mutating agents; return-value contract
   for read-only fan-outs.
7. **Validate after every agent commit** (below) — the self-report is not
   evidence.
8. **Budget-check before *each* dispatch in a fan-out** with
   `bin/usage-gate.sh`, not once at the start. Width is the kill axis: the
   window wall fires mid-flight and the agents that bank are the ones that
   *finished*. Project the next wave's burn from the last wave's, not from
   the current percentage alone — a read-heavy wave is far more
   window-expensive than its token total suggests.

### Two templates, one protocol

`templates/agent-execution.md` is the complete procedure, copied into the
consumer's skills directory (`.claude/commands/agent-execution.md`) and
invoked when delegating. `templates/claude-md-agent-execution.md` is the
compressed resident form for the consumer's CLAUDE.md: skills load on
invocation, but the safety rules must never be absent from a session that
*could* delegate, so the load-bearing bullets ride in the always-loaded
file and point at the skill for the full procedure. Consumer-specific
material in both templates — the shared-file roster, the validate battery —
lives in marked consumer sections, the same discipline as guard-kit's
consumer-rules block.

## Resume journal — agent writes, supervisor deletes

A mutating agent `Write`s a running progress journal (findings triaged,
edits applied, what remains) to the harness session directory (never a
system temp dir — a restart wipes it), updating as it goes; on success it
appends a `DONE` marker. Each finding lands in the journal *inline as it is
confirmed* — never "see final output": the agent's return message dies with
the session, so a pointer-only journal makes `DONE` lie about
recoverability. Agents have no `rm`; the supervisor deletes the journal at
the post-commit validation checkpoint. A journal present *without* `DONE`
means that unit was interrupted — resume from it.

**Caveat — the sandbox may block the journal write.** Background agents
have been observed unable to `Write` to the session dir, silently falling
back to returning findings in the final message — which defeats the journal
exactly when it matters (a long, interruptible run). So: for a read-only
fan-out the return value *is* the contract — no journal; for mutating
agents, grant the journal path explicitly before dispatch rather than
assuming the write succeeds.

## Validate after every agent commit

Re-run the relevant gates plus the consumer's validate battery (for this
repo and any gate-sdk consumer: `bash gate-sdk/bin/run-gates.sh` and the
kit fixture runners; a toolchain consumer adds its compile/lint/test set in
the template's marked section, including a zero-byte-file sweep after
renames — silent edit corruption is real). **Diff every gate change in an
agent commit before accepting**: an agent blocked by a gate will weaken it
(a false exemption) rather than fix the code — "the gate is in my way"
almost always means the code doesn't fit the convention.

`check-gate-tamper` is the mechanical floor under that duty. Two
assertions, blocking the two attested tamper shapes:

- **A — gate-edit isolation.** A commit that touches a gate file (the
  `DELEGATION_KIT_GATE_FILES` globs) may touch only meta-layer paths
  (`DELEGATION_KIT_META_PATHS` prefixes + root `*.md`); co-staging product
  code with a gate edit is blocked. Split the gate change into its own
  commit.
- **B — no self-serving exemption.** A newly added path/glob entry in any
  gate's `# exception-list:` array must not match a file staged in the same
  commit — an exemption never excuses the very change it lands with.

**Honest limit:** the gate blocks by commit *shape*; it does not catch
semantic weakening inside a legitimate scripts-only commit — the by-eye
diff review of agent gate edits remains a supervisor duty. `--fixture
<dir>` injects `staged-files` / `added-exemptions` lists (fixture-pair test
capability); live mode reads `git diff --cached`.

## usage-gate

Emits a trustworthy budget verdict from a usage snapshot file, closing the
three failure modes a raw percentage reading leaves open:

1. **Stale reading** — `now - updated_at` beyond `STALE_AGE` → STALE
   (exit 2): re-read before trusting.
2. **Dead window** — `resets_at <= now` → RESET-OK (exit 0): the percentage
   is from the dead window and must not be read as a pause signal.
3. **Post-login lag** — a fresh login starts a new window but the
   server-fed percentage lags it, and the file-write age check cannot see
   that. The gate reads the auth event from the credentials file's mtime
   (`LOGIN_WINDOW`); a would-be PAUSE with a that-recent login routes to
   STALE (re-read) instead of a wrongful pause. Self-limiting: once the
   mtime ages out, a genuine near-limit percentage re-reads back to PAUSE.

Exit codes: **0** OK / RESET-OK, **1** PAUSE (over `PAUSE_PCT` of the live
window), **2** STALE or unreadable. Fail-closed throughout: missing keys
and a non-numeric percentage route to STALE, and the threshold compare uses
`awk`, not integer-only bash arithmetic, so a fractional percentage cannot
silently skip PAUSE. The five-hour window is the only pause axis; 7-day
keys are ignored.

### The usage.txt contract

The snapshot is the wire contract between any producer and the gate — three
`key=value` lines:

```
five_hour_used_pct=<float>
five_hour_resets_at=<epoch-seconds>
updated_at=<epoch-seconds>
```

`templates/statusline-usage.sh` is a minimal producer: a statusline hook
that parses the harness's rate-limit JSON and atomically writes the
snapshot (`tmp` + `mv`) to `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt`.
Any producer honoring the contract works — the source is pluggable
(`DELEGATION_KIT_USAGE_FILE`), which de-hardcodes the source platform's
single-operator `CLAUDE_CONFIG_DIR` assumption.

## Layout and configuration

```
delegation-kit/
  bin/usage-gate.sh
  bin/run-usage-tests.sh        # decision-table runner
  usage-tests/cases.tsv         # expected-verdict <TAB> scenario knobs
  checks/check-gate-tamper.sh
  gate-tests/check-gate-tamper/{good,bad}/
  templates/agent-execution.md            # full protocol skill
  templates/claude-md-agent-execution.md  # resident CLAUDE.md section
  templates/statusline-usage.sh           # minimal usage.txt producer
  templates/delegation-config.sh          # knob overrides (arrays live here)
  smoke/install.sh
  smoke/violation.sh
```

Config follows the established kit pattern: copy
`templates/delegation-config.sh` into the gates dir (or point
`DELEGATION_KIT_CONFIG_FILE` elsewhere) and override any knob; defaults
fill what the consumer left unset. The loader is fail-closed: a
`DELEGATION_KIT_CONFIG_FILE` named but absent, or a config leaving any knob
malformed, exits 2 (a broken machine gates nothing). Knobs (platform values
as defaults):

- `DELEGATION_KIT_USAGE_FILE` — default
  `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt`; positional `$1`
  overrides (test injection).
- `DELEGATION_KIT_CRED_FILE` — default the usage file's sibling
  `.credentials.json`; positional `$2` overrides.
- `DELEGATION_KIT_PAUSE_PCT` — default `80`.
- `DELEGATION_KIT_STALE_AGE` — default `600` (seconds).
- `DELEGATION_KIT_LOGIN_WINDOW` — default `600` (seconds).
- `DELEGATION_KIT_GATE_FILES` — globs naming gate files for tamper
  assertion A; default
  `("${GATE_SDK_GATES_DIR:-scripts}/check-*.sh")` plus the gate-sdk lib and
  runners (this repo's consumer config widens it to `*/checks/*.sh`).
- `DELEGATION_KIT_META_PATHS` — prefixes counted as meta-layer for
  assertion A; default `("${GATE_SDK_GATES_DIR:-scripts}/"
  "${GATE_SDK_WORKFLOW_DIR:-.workflow}/" ".claude/")`; root-level `*.md` is
  always meta.

`check-gate-tamper` registers in the consumer's `gates.list`
(tier: precommit) — in this repo's too; dogfooding is day-one, and agents
commit here.

## Testing

`check-gate-tamper` speaks the full gate contract (`GATE-TAMPER: clean
(…)` / findings + `help:` lines / exit 0-1-2) and ships the standard
`good/`+`bad/` fixture pair driven through `--fixture` by gate-sdk's
`run-gate-tests.sh`.

`usage-gate` does not fit the gate contract (a three-state verdict, not a
clean/violation pair), so — like guard-kit's guard-tests — the kit ships
its own decision-table runner: `usage-tests/cases.tsv` pairs an expected
verdict token (`OK`/`PAUSE`/`STALE`/`RESET-OK`) and exit code with scenario
knobs (percentage, snapshot age offset, reset offset, credential age);
`bin/run-usage-tests.sh` materializes each case as a generated snapshot
file (timestamps must be computed relative to *now* — static fixtures
would age into permanent STALE) and asserts verdict and exit code. Each
case runs in a throwaway sandbox with no consumer config on the lookup
path, so the gate exercises its own defaults hermetic to the host repo;
`cases.tsv` columns are `verdict exit pct age_off reset_off cred_age desc`
(the offsets seconds from *now*). Every verdict branch carries at least one
firing and one non-firing case — the fixture-pair discipline, transplanted.

`smoke/install.sh` copies the templates and `bin/` tools into the scratch
consumer, registers the tamper gate, and drives one crafted snapshot
through `usage-gate` asserting a verdict — self-verifying install.
`smoke/violation.sh` stages a gate edit co-staged with a product file in
the scratch consumer and asserts the battery reds (assertion A) — the
violation is craftable, so the file is mandatory
(gate-sdk/SPEC.md §Consumer smoke).

## What stayed on the platform

The platform's validate battery (its compile/lint/test command set and
rename corruption sweeps), its shared-file roster, and its width/burn
anecdotes tied to specific sweeps — consumer rule content, referenced only
as marked-section examples. Its task-output tailer (`read-task-outputs.sh`)
stays: it hardcodes local harness paths and exists to violate protocol
rule 2 for debugging. The full statusline (gauge bars, iteration display)
stays as platform UX — only the `usage.txt` write contract and the minimal
producer template leave. The platform remains the exemplar consumer:
protocol and mechanisms here, its rosters and batteries in its own copies.
