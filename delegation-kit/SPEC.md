# delegation-kit — safe delegated-agent execution for budget-bounded sessions

Delegated agents are cheap to dispatch and expensive to trust. Three failure
surfaces dominate: **shared mutable state** (two committing agents race the
git index and one sweeps the other's staged files under the wrong message),
**interrupted long units** (a usage-window wall fires mid-flight and the
uncommitted investigation dies with the session), and **untrustworthy
self-reports** (a sub-agent's "passed" claim, or a gate quietly weakened to
make its commit pass). The kit packages the supervisor-side protocol that
closes all three, plus the two pieces that are mechanizable: a trustworthy
budget verdict (`usage-verdict`) and a commit-shape gate over gate tampering
(`check-gate-tamper`).

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
3. **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide otherwise.**
   Agents editing a shared file run one at a time. **The git index and HEAD
   are shared files for every agent that commits**, independent of
   source-file disjointness — committing agents are serialized *or* isolated
   in their own worktree (`isolation: worktree` — own index); the unlocked
   ≤`DELEGATION_KIT_FAN_WIDTH`-wide bound is a **read-only-fan-out ceiling
   only** — a committing fan-out serializes or takes worktrees regardless,
   which is correctness and never configurable up. "No lockfile churn" is a
   false safety signal. (The knob and its derivation: §Layout and
   configuration.)
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
   `bin/usage-verdict.sh`, not once at the start. Width is the kill axis: the
   window wall fires mid-flight and the agents that bank are the ones that
   *finished*. Project the next wave's burn from the last wave's, not from
   the current percentage alone — a read-heavy wave is far more
   window-expensive than its token total suggests. The manual run is the
   planning tool; `agent-budget-guard.sh` is its mechanical enforcement at the
   dispatch point (below).

`templates/agent-budget-guard.sh` closes the gap the manual run leaves: the
budget-check norm relied on the agent *choosing* to run the tool, and a
memory-quoted percentage acts in its place (a session quoted ~5% while the
live verdict read 29%). It is a `PreToolUse` hook (matcher `Agent`) the
harness fires on every dispatch once the consumer registers it in settings —
per-dispatch freshness, not a start-of-session reading a mid-session window
outlives. It runs `usage-verdict` at the decision point and routes on the exit
code:

- **PAUSE (1)** → `guard_block`: the verdict line plus its corrective (wait for
  the window reset, or re-run with `DELEGATION_KIT_PAUSE_PCT` deliberately
  raised). This is guard-kit's one sanctioned fail-closed deny — the hook
  matcher proves the tool identity and PAUSE is reachable only through a fresh,
  readable, over-threshold snapshot, so blocking cannot wedge a consumer with
  no producer.
- **STALE / unreadable (2)** and **OK / RESET-OK (0)** → `guard_advise`,
  feeding the verdict line back as `additionalContext` so the live reading
  rides in context at every dispatch and a memory-quoted percentage can never
  be the acting source. STALE never blocks — budget-unknown is
  decision-relevant, but a consumer with no snapshot producer must route to
  advice, not out of delegation.

The harness is the producer (it fires the hook once the settings registration
is set); the consumers are the dispatching agent, which reads the verdict line
off `additionalContext` at the dispatch-decision transition, and the
supervising user, who consumes a block by ruling — raise the knob or wait; the
agent never overrides. delegation-kit owns the verdict, its thresholds, and the
routing; guard-kit supplies only the framework primitives (its second
consumer). Registration is the opt-in valve: a consumer wanting pure advice
does not wire the hook. No new persistent state and no new key on the
`usage.txt` contract — the verdict line is the only interface.

### One template, a resident pointer

`templates/agent-execution.md` is the single source for the protocol — a
binding-shim *template* in lifecycle-kit's grammar (lifecycle-kit/SPEC.md
§check-skill-binding). The consumer creates `.claude/commands/agent-execution.md`
as a shim that names the template and binds its two slots — the shared-file
roster and the validate battery — the consumer-specialization discipline of
guard-kit's consumer-rules block, now carried by the slot/binding mechanism the
lifecycle skills already use. `check-skill-binding` enforces the slot pairing on
the consumer side.

The consumer's CLAUDE.md carries no digest of the bullets, only a resident
pointer: the pre-authorization sentence (consumer judgment on what delegation
needs no ask) and `/agent-execution`. Rationale: a rule is resident only when it
has no load trigger (the load-trigger-residency doctrine), and every protocol
bullet triggers at `Agent` dispatch — which already has a mechanical seam, the
per-dispatch budget guard, whose block message names `/agent-execution`. The
doctrine lives behind the trigger, not in the always-loaded file. Honest limit:
the guard enforces budget mechanically, not protocol literacy — a session that
dispatches without invoking the skill carries only the resident pointer.

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

## usage-verdict

Emits a trustworthy budget verdict from a usage snapshot file, closing the
three failure modes a raw percentage reading leaves open:

1. **Stale reading** — `now - updated_at` beyond `STALE_AGE` → STALE
   (exit 2): re-read before trusting.
2. **Dead window** — `resets_at <= now` → RESET-OK (exit 0): the percentage
   is from the dead window and must not be read as a pause signal. The
   weekly axis carries the same rule per-axis (`seven_day_resets_at <= now`
   disarms the weekly pause without forcing the whole verdict to RESET-OK —
   the axes are judged independently).
3. **Post-login lag** — a fresh login starts a new window but the
   server-fed percentage lags it, and the file-write age check cannot see
   that. The gate reads the auth event from the credentials file's mtime
   (`LOGIN_WINDOW`); a would-be PAUSE with a that-recent login routes to
   STALE (re-read) instead of a wrongful pause. Self-limiting: once the
   mtime ages out, a genuine near-limit percentage re-reads back to PAUSE.
   An account switch swaps both windows, so the reroute applies whichever
   axis would have fired.

Exit codes: **0** OK / RESET-OK, **1** PAUSE, **2** STALE or unreadable.
Fail-closed throughout: missing keys and a non-numeric percentage route to
STALE, and each threshold compare uses `awk`, not integer-only bash
arithmetic, so a fractional percentage cannot silently skip PAUSE.

**The `width=<n>` field.** Every emitted verdict line — OK, PAUSE, STALE (the
fail-closed diagnostics included), RESET-OK — carries a `width=<n>` field
immediately before the ` -> <verdict>` arrow, read from
`DELEGATION_KIT_FAN_WIDTH` (§Layout and configuration). It is the knob's
mechanical reader: the budget check already runs before every dispatch, so the
read-only fan-out bound surfaces at exactly the wave-sizing decision point, and
`agent-budget-guard.sh` relays the verdict line verbatim (block on PAUSE, advise
otherwise) so the width rides into context with the budget verdict. The field is
a config constant, not derived from the snapshot, so it is present even on an
unreadable-snapshot STALE. No exit-code or other-field change: existing callers
that key off the exit code or the `-> <verdict>` arrow are unaffected.

**Two pause axes.** The five-hour window is the always-on axis; the weekly
(7-day) window is a second axis, armed only when both `seven_day_used_pct`
and `seven_day_resets_at` are present (a three-line snapshot keeps today's
behavior — no retroactive contract break). The weekly limit can deplete
while the 5h window sits comfortable, and a weekly PAUSE costs days, not
hours, so it must gate delegation planning, not merely appear in a log —
delegation is the discretionary spend, the first thing to stop near the
weekly ceiling so the remaining week stays with the supervisor. The axes
are judged independently against `PAUSE_PCT` and `PAUSE_PCT_7D`; either
firing is a PAUSE (exit 1), and the message names the axis that fired
(`PAUSE (7-day window)` vs `PAUSE (5h window)`, the weekly named when both
fire) because the operator's remediation differs by days. No caller
changes: the session-context hook and the per-dispatch Agent budget guard
already consume the exit code, so the weekly axis flows into every dispatch
decision the moment a producer supplies the keys.

**Usage-history sampling.** When `DELEGATION_KIT_USAGE_HISTORY` is non-empty,
`usage-verdict` appends one sample line (§The usage.txt contract) to that log
after every successfully parsed snapshot, whatever the verdict — the raw
harness-reported values verbatim; write-time smoothing or correction is
forbidden (a later corrective push is evidence about the earlier sample, and
only the reader has both). A STALE exit from an unreadable or unparseable
snapshot appends nothing — a sample the gate would not trust is not history.
`usage-trend` (§Trend reporter) reads the log.

### The usage.txt contract

The snapshot is the wire contract between any producer and the gate — three
`key=value` lines:

```
five_hour_used_pct=<float>
five_hour_resets_at=<epoch-seconds>
updated_at=<epoch-seconds>
```

Beyond the three mandatory lines a producer may write optional keys when its
source exposes them; `usage-verdict` reads the ones it interprets and passes
the rest through: `seven_day_used_pct` / `seven_day_resets_at` (the weekly
window — read at the verdict transition to arm the second pause axis),
`account` (the logged-in account identity — `login_at` detects a switch,
`account` says to whom, which lets `usage-trend` group a multi-account
operator's segments per account), `tier` (subscription tier — the
denominator behind the percentages), and `tokens_in` / `tokens_out`
(cumulative token counts, the axis that binds API-billed consumers for whom
no subscription percentage applies). Optional keys are omitted when their
source has no value, never written empty; keys the verdict does not read
pass through unchanged.

`templates/statusline-usage.sh` is the reference producer: a statusline hook
that parses the harness's rate-limit JSON and atomically writes the
snapshot (`tmp` + `mv`) to `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt`.
It also produces the weekly pair from the payload and `account` / `tier`
from the local account config (`.credentials.json` / `~/.claude.json`,
overridable via `DELEGATION_KIT_CRED_FILE` / `DELEGATION_KIT_ACCOUNT_CONFIG`).
It ships no `tokens_in` / `tokens_out` producer: this harness's payload
carries no cumulative token count, so under the dead-producer rule the keys
stay defined here for third-party producers but no dead producer is shipped.
Beyond the snapshot write it renders a status bar — model/effort, a context
gauge, the 5h and 7d windows with reset countdowns, and the `iteration@stage`
readout parsed from the queue header — self-contained ANSI, no external asset
(§Out of scope). The snapshot write is the contract; the bar is
reference UX a consumer may restyle or discard. Any producer honoring the
contract works — the source is pluggable (`DELEGATION_KIT_USAGE_FILE`), so no
single-operator `CLAUDE_CONFIG_DIR` assumption is baked in. Because the
statusline fires far more often than the per-session /
per-dispatch verdict calls, a consumer wanting a denser trend history can drive
sampling from the render path — `usage-verdict` stays the single append author
(§usage-verdict), so the statusline calls it (with `DELEGATION_KIT_USAGE_HISTORY`
set) rather than appending the log itself.

**The sample line.** With sampling enabled (§usage-verdict), `usage-verdict`
appends one line per parsed snapshot — the trend log's wire contract between
it and `usage-trend`:

```
updated_at=<epoch> pct=<float> resets_at=<epoch> verdict=<word> login_at=<epoch>[ account=<word>][ tier=<word>][ pct_7d=<float> resets_7d=<epoch>][ tokens_in=<n> tokens_out=<n>]
```

Space-separated `key=value`, order-insensitive; optional groups are omitted
(never written empty) when the snapshot lacks them. `login_at` is the
credentials-file mtime the post-login-lag check already reads, stamped per
sample so an account switch becomes data; `pct` / `resets_at` are the 5h
values verbatim and `pct_7d` / `resets_7d` the weekly ones. The log is
append-only (the tmp-prune / boundary-truncate conventions own cleanup), and
carries operator-local account identifiers, so it lives under the gitignored
measurement dir and never reaches a tracked file.

## Trend reporter

`bin/usage-trend.sh` reads the history log and reports how the footprint
evolves — advisory tooling, never a gate: exit **0** report emitted, **2**
knob unset or history missing/unreadable (fail-closed, mirroring the
verdict's STALE discipline), never **1** (it renders no verdict; the verdict
stays the sole pause authority). The source signal is known-noisy — a
rolling-window reading spikes and reverts-down when a harness over-report is
corrected by the next push — so the design separates signal from noise by
the window's one physical constraint: within a segment, true usage never
decreases.

1. **Segment** samples per axis by that axis's reset epoch, `login_at`,
   `account`, and `tier`: the 5h axis keys on `resets_at`, the weekly axis
   on `resets_7d` (the windows roll independently — a weekly segment spans
   many 5h segments). A timer reset, a `/login`, or an account or tier
   change each starts a segment; only within-segment comparisons are
   meaningful, so an account switch is a boundary, not a flagged anomaly.
2. **Flag** any sample whose pct is below an earlier one in the same segment
   as a monotonicity violation: the downward correction indicts the elevated
   sample(s) before it as reader noise, and both sides are excluded from rate
   math, never averaged in. Median-of-3 smoothing resolves single-sample
   spikes; segment endpoints keep their own value (no 2-window averaging).
3. **Report** per segment and axis — first/last smoothed pct, pct-per-hour
   rate, token deltas when token keys ride, tier, sample count, and
   suspect-sample count (a high suspect ratio means the producer is
   unreliable and no number from that segment is trusted). The weekly axis
   additionally reports headroom against `PAUSE_PCT_7D` at the current rate —
   the planning number for how much delegation the week still affords — and,
   when `account` is present, segments group under an account heading so a
   rotating operator reads one weekly trajectory per account rather than an
   interleaved stream.

## Layout and configuration

```
delegation-kit/
  bin/usage-verdict.sh
  bin/usage-trend.sh              # footprint trend reporter over the history log
  bin/run-usage-tests.sh          # verdict decision-table runner
  bin/run-budget-guard-tests.sh   # budget-guard decision-table runner
  bin/run-trend-tests.sh          # trend-reporter assertion runner
  usage-tests/cases.tsv           # expected-verdict <TAB> scenario knobs
  usage-tests/budget-guard-cases.tsv  # expected-action <TAB> scenario knobs
  usage-tests/trend-history.log   # fixture history for the trend runner
  checks/check-gate-tamper.sh
  gate-tests/check-gate-tamper/{good,bad}/
  templates/agent-execution.md            # full protocol, bound as a skill shim
  templates/agent-budget-guard.sh         # PreToolUse(Agent) budget guard
  templates/statusline-usage.sh           # reference usage.txt producer + status bar
  templates/delegation-config.sh          # knob overrides (arrays live here)
  smoke/install.sh
  smoke/violation.sh
```

Config follows the established kit pattern: copy
`templates/delegation-config.sh` into the gates dir (or point
`DELEGATION_KIT_CONFIG_FILE` elsewhere) and override any knob; defaults
fill what the consumer left unset. The loader is fail-closed: a
`DELEGATION_KIT_CONFIG_FILE` named but absent, or a config leaving any knob
malformed, exits 2 (a broken machine gates nothing). Knobs (this repo's
layout as defaults):

- `DELEGATION_KIT_USAGE_FILE` — default
  `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt`; positional `$1`
  overrides (test injection).
- `DELEGATION_KIT_CRED_FILE` — default the usage file's sibling
  `.credentials.json`; positional `$2` overrides.
- `DELEGATION_KIT_PAUSE_PCT` — default `80`.
- `DELEGATION_KIT_PAUSE_PCT_7D` — weekly-axis pause threshold; default the
  `DELEGATION_KIT_PAUSE_PCT` value (one conservatism policy unless split).
- `DELEGATION_KIT_STALE_AGE` — default `600` (seconds).
- `DELEGATION_KIT_LOGIN_WINDOW` — default `600` (seconds).
- `DELEGATION_KIT_USAGE_HISTORY` — sample-log path; default empty (sampling
  off). This repo sets `.tmp/usage-history.log`, a gitignored measurement.
- `DELEGATION_KIT_FAN_WIDTH` — read-only-fan-out width bound; default `2`,
  validated a positive integer by the loader. It bounds read-only fan-outs
  only: a committing fan-out serializes or takes its own worktree regardless
  (rule 3), so this knob is never a licence to widen concurrent committers.
  The default is the loss-bounding invariant — bound the in-flight loss to
  what the window can absorb when the wall fires mid-flight — at a Pro-class
  subscription window; a Max-class window absorbs more, and an API-billed
  operator has no mid-flight window wall at all, so there spend rate and the
  provider's rate limits replace the rationale. A consumer retunes from the
  invariant, not the number. **Supervision ceiling:** operator attention over
  N concurrent reports does not scale with the budget window — a bigger
  window raises the affordable width, not the reviewable one, so budget is
  never the sole input to widening. `usage-verdict` surfaces the live value
  as the `width=` field (§usage-verdict), the knob's mechanical reader.
- `DELEGATION_KIT_GATE_FILES` — globs naming gate files for tamper
  assertion A; default
  `("${GATE_SDK_GATES_DIR:-scripts}/check-*.sh")` plus the gate-sdk lib and
  runners (this repo's consumer config widens it to `*/checks/*.sh`).
- `DELEGATION_KIT_META_PATHS` — prefixes counted as meta-layer for
  assertion A; default `("${GATE_SDK_GATES_DIR:-scripts}/"
  "${GATE_SDK_WORKFLOW_DIR:-.workflow}/" ".claude/")`; root-level `*.md` is
  always meta. When `gate.sh` is resolvable (`GATE_SDK_LIB`, else the vendored
  sibling), the loader unions every `gate_kit_roots` member into this array as a
  root-relative `dir/` prefix — a vendored kit's edits are meta-layer by
  definition, so the consumer's config need not name kit dirs at all (this
  repo's keeps only the non-kit prefixes). The union is additive, never a
  filter: a
  prefix the consumer declared cannot be lost, and without `gate.sh` the config
  is used exactly as written.

`check-gate-tamper` registers in the consumer's `gates.list`
(tier: precommit) — in this repo's too; dogfooding is day-one, and agents
commit here.

`agent-budget-guard.sh` is not a gate — it is a hook, so it registers not in
`gates.list` but under `PreToolUse` matcher `Agent` in the consumer's
`.claude/settings.json` (beside the guard-kit Bash guard). Copy the template
into the gates dir and wire `bash scripts/agent-budget-guard.sh`; it resolves
`guard-kit/lib/guard.sh` and `bin/usage-verdict.sh` at their vendored paths,
overridable with `GUARD_KIT_LIB` / `DELEGATION_KIT_VERDICT_BIN`. Registration
is the whole opt-in: unwired, the guard is inert. This repo registers it, and
its consumer session brief (`scripts/session-context.sh`) additionally prints
the verdict line at SessionStart for planning-time visibility — a consumer-side
edit; the context-kit template stays uncoupled from delegation-kit.

## Testing

`check-gate-tamper` speaks the full gate contract (`GATE-TAMPER: clean
(…)` / findings + `help:` lines / exit 0-1-2) and ships the standard
`good/`+`bad/` fixture pair driven through `--fixture` by gate-sdk's
`run-gate-tests.sh`.

`usage-verdict` does not fit the gate contract (a three-state verdict, not a
clean/violation pair), so — like guard-kit's guard-tests — the kit ships
its own decision-table runner: `usage-tests/cases.tsv` pairs an expected
verdict token (`OK`/`PAUSE`/`STALE`/`RESET-OK`) and exit code with scenario
knobs (percentage, snapshot age offset, reset offset, credential age);
`bin/run-usage-tests.sh` materializes each case as a generated snapshot
file (timestamps must be computed relative to *now* — static fixtures
would age into permanent STALE) and asserts verdict and exit code. Each
case runs in a throwaway sandbox with no consumer config on the lookup
path, so the gate exercises its own defaults hermetic to the host repo;
`cases.tsv` columns are `verdict exit pct age_off reset_off cred_age pct_7d
reset7d_off append axis desc` (the offsets seconds from *now*; `pct_7d` `-`
omits the weekly keys, `append` is the expected sample-line count, `axis`
asserts which window a PAUSE names). Every verdict branch and both pause axes
carry a firing and a non-firing case — the fixture-pair discipline,
transplanted — covering a weekly PAUSE while 5h is comfortable, the axis
named in the output, absent keys disarming the weekly axis, a dead weekly
window not pausing, and the sample-append discipline (a parsed snapshot
appends one line whatever the verdict; a non-numeric snapshot appends none;
`pct_7d` present-vs-omitted in the passed-through line).

`usage-trend` is likewise not a gate (it renders no clean/violation
verdict), so it ships an assertion runner, `bin/run-trend-tests.sh`, over a
static fixture history `usage-tests/trend-history.log` (static epochs are
safe — the reporter measures within-segment deltas, never against *now*). It
asserts per-axis segmentation at a reset boundary, a `login_at` change, and
an account change; per-account grouping reuniting a weekly trajectory across
a switch-back; a spike-then-correction flagged and excluded rather than
averaged; token deltas and weekly headroom on the report; and the
fail-closed exits (knob unset / history missing → 2). No fixture pair owed —
neither script is a gate.

`agent-budget-guard.sh` is a hook, not a gate, so it speaks exit-2 + hook
JSON rather than the gate output contract — and like the verdict it ships a
decision-table runner beside the verdict tests: `usage-tests/budget-guard-cases.tsv`
pairs an expected action (`block`/`advise`) with the same snapshot knobs
(`action pct age_off reset_off cred_age desc`, `pct=UNREADABLE` for the
no-snapshot case), and `bin/run-budget-guard-tests.sh` drives the *template*
with each injected snapshot — feeding an Agent hook JSON on stdin, pointing
`DELEGATION_KIT_USAGE_FILE` at the generated fixture — and classifies the
result: PAUSE → `block` (exit 2, verdict on stderr), OK/RESET-OK/STALE/unreadable
→ `advise` (exit 0, verdict in `additionalContext`). The block branch carries
its firing (PAUSE) and non-firing (the four advise cases) — the fixture-pair
discipline again — and every case asserts the live verdict text rides the
output so a memory-quoted percentage cannot be the acting source.

`smoke/install.sh` copies the templates and `bin/` tools into the scratch
consumer, registers the tamper gate, and drives one crafted snapshot
through `usage-verdict` asserting a verdict — self-verifying install.
`smoke/violation.sh` stages a gate edit co-staged with a product file in
the scratch consumer and asserts the battery reds (assertion A) — the
violation is craftable, so the file is mandatory
(gate-sdk/SPEC.md §Consumer smoke).

## Out of scope

A consumer's validate battery (its compile/lint/test command set and rename
corruption sweeps), its shared-file roster, and its width/burn anecdotes tied
to specific sweeps are rule content, referenced only as marked-section
examples. A task-output tailer is not shipped: such a tool hardcodes local
harness paths and exists to violate protocol rule 2 for debugging. The
`usage.txt` write contract and a reference producer rendering the gauge bars
and `iteration@stage` readout do ship (the producer's ANSI is self-contained,
pulling no external asset); a consumer's *particular* statusline styling and
any asset its own gauges pull are not the rendering mechanism and stay with
it. The split is protocol and mechanisms here, a consumer's rosters,
batteries, and bespoke UX in its own copies.
