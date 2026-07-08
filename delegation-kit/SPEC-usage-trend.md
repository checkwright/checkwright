# SPEC amendment: usage-trend

## What changes

`usage-verdict` answers "may I dispatch right now" from the five-hour window
alone; nothing tracks how the budget *evolves* across a session's prompts
and tool calls, and the weekly limit — which can bottleneck while the 5h
window sits comfortable — never enters the dispatch decision. This adds a
sampling log and a trend reporter (advisory tooling, not a gate) and a
second pause axis in the verdict itself.

**Usage history log.** New knob `DELEGATION_KIT_USAGE_HISTORY` (default
empty = sampling disabled; this repo's `scripts/delegation-config.sh` sets
`.tmp/usage-history.log` — a measurement, gitignored). When non-empty,
`usage-verdict.sh` appends one sample line after every successfully parsed
snapshot, whatever the verdict — the raw harness-reported values, verbatim;
write-time smoothing or correction is forbidden (a later corrective push is
evidence about the earlier sample, and only the reader has both):

```
updated_at=<epoch> pct=<float> resets_at=<epoch> verdict=<word> login_at=<epoch>[ account=<word>][ tier=<word>][ pct_7d=<float> resets_7d=<epoch>][ tokens_in=<n> tokens_out=<n>]
```

Space-separated `key=value`, order-insensitive; optional keys are omitted
when their source has no value, never written empty. `login_at` is the
credentials-file mtime `usage-verdict` already reads for its post-login-lag
check — stamped per sample, it turns an account switch into data. `tier`
and `tokens_in`/`tokens_out` ride in from the snapshot when the producer
supplies them (below). Append-only, no rotation in-kit (the
boundary-truncate/tmp-prune conventions own cleanup). A STALE exit from an
unreadable snapshot appends nothing — a sample the gate would not trust is
not history. `templates/statusline-usage.sh` documents the same append as an
optional dense-sampling producer; the verdict path is the one this repo
deploys (session-context hook + the Agent budget guard already invoke
`usage-verdict` per session and per dispatch).

**usage.txt optional keys.** The three-line snapshot contract gains optional
keys a producer writes when its source exposes them: `account=<word>` (the
logged-in account identity — `login_at` detects a switch, `account` says to
whom, which is what lets the reporter group a multi-account operator's
segments per account instead of treating every switch-back as a stranger),
`tier=<word>`
(subscription tier — Pro/Max5x/Max20x — the denominator behind the pct),
`seven_day_used_pct=<float>`/`seven_day_resets_at=<epoch>` (the weekly
window), and `tokens_in=<n>`/`tokens_out=<n>` (cumulative token counts, the
axis that matters to API-billed consumers for whom no subscription pct is
the binding budget). `usage-verdict` passes keys it does not read through to
the sample line unchanged. Build verifies which of these the harness's
rate-limit/statusline payload actually carries: a key with no live producer
in the shipped template is dropped from the template (the contract keeps it
defined for third-party producers), per the dead-producer rule.

**Second pause axis: the 7-day window.** The weekly limit can deplete while
the five-hour window is still comfortable, and a weekly PAUSE costs days,
not hours — so it must gate *delegation planning*, not just appear in a log.
`usage-verdict` gains the 7-day axis, armed only when both `seven_day_*`
keys are present (a three-line snapshot keeps today's behavior — no
retroactive contract break):

- Threshold knob `DELEGATION_KIT_PAUSE_PCT_7D`, defaulting to the existing
  `PAUSE_PCT` value — one conservatism policy unless the consumer splits
  them. Delegation is the discretionary spend; near the weekly ceiling it is
  the first thing to stop so the remaining week stays with the supervisor.
- Per-axis dead-window rule: `seven_day_resets_at <= now` means that axis's
  pct is from a dead window and must not pause (mirroring the 5h rule); the
  axes are judged independently and either can fire.
- A PAUSE names the axis that fired (`PAUSE (7-day window)` vs
  `PAUSE (5h window)`) — the operator's remediation differs by days, so the
  finding must say which. Exit codes unchanged: 1 is PAUSE from either axis;
  the post-login STALE routing applies to both (an account switch swaps both
  windows).

No caller changes anywhere: the session-context hook and the per-dispatch
Agent budget guard already consume the exit code, so the weekly axis flows
into every dispatch decision the moment the producer supplies the keys.

**Trend reporter.** New `delegation-kit/bin/usage-trend.sh`: reads the
history, reports footprint evolution. The source signal is known-noisy —
rolling-window readings spike and revert-down when a harness over-report is
corrected by the next push — so the design separates signal from noise by
the window's physical constraint: *within one segment, true usage never
decreases.* Mechanics:

1. **Segment** samples per axis by (that axis's reset epoch, `login_at`,
   `account`, `tier`): the 5h axis keys on `resets_at`, the weekly axis on
   `resets_7d` — the windows roll independently, and a weekly segment spans
   many 5h segments. A timer reset, a `/login` (multi-account operation
   legitimately drops the pct mid-window), or an account or tier change each
   starts a segment; only within-segment comparisons are meaningful, so an
   account switch is a boundary, not a flagged anomaly.
2. **Flag** any sample whose pct is below an earlier sample in the same
   segment (per axis) as a monotonicity violation: the downward correction
   indicts the elevated sample(s) before it as reader noise, and both sides
   of the violation are excluded from rate math, never averaged in.
   Median-of-3 smoothing resolves single-sample spikes.
3. **Report** per segment and axis: first/last smoothed pct, pct-per-hour
   rate, token deltas when token keys are present, tier, sample count, and
   suspect-sample count — a high suspect ratio means the producer is
   unreliable and no number from that segment is trusted. The weekly axis
   additionally reports headroom against `DELEGATION_KIT_PAUSE_PCT_7D` at
   the current rate — the planning number for how much delegation the week
   still affords. When `account` is present, segments group under an
   account heading, so a rotating multi-account operator reads one weekly
   trajectory and one headroom number per account rather than an
   interleaved stream; the history is an operator-local measurement under
   the tmp-dir convention (gitignored), so the identifier never reaches a
   tracked file.

Exit codes: 0 report emitted; 2 knob unset or history missing/unreadable
(fail-closed, mirroring usage-verdict's STALE discipline). Never 1 — it
renders no verdict; `usage-verdict` stays the sole pause authority.

## Producers and consumers

- Sample line — produced by `usage-verdict.sh` (deployed per session-start
  and per Agent dispatch in this repo; enabling config:
  `DELEGATION_KIT_USAGE_HISTORY` in `scripts/delegation-config.sh`),
  optionally by the statusline template. Consumed by `usage-trend.sh`.
- Optional snapshot keys (`account`, `tier`, `seven_day_used_pct`,
  `seven_day_resets_at`, `tokens_in`, `tokens_out`) — produced by
  `templates/statusline-usage.sh` from the harness payload or the local
  account config, whichever exposes each (build verifies and ships only
  live producers). The `seven_day_*` pair is read by `usage-verdict.sh` at
  the verdict transition (second pause axis); the rest pass through into
  the sample line unread.
- Field readers in `usage-trend.sh`: `updated_at` — sample ordering and the
  rate denominator; `pct` / `pct_7d` — the per-axis trend values;
  `resets_at` / `resets_7d` plus `login_at`, `account`, and `tier` — the
  per-axis segment keys (read at segmentation; `account` again at report
  grouping); `verdict` — annotates where PAUSE onsets fall in the report;
  `tokens_in`/`tokens_out` — the per-segment token deltas. No field without
  a reader.
- `usage-trend.sh` output — consumed by the operator (and by drift-kit as a
  candidate KPI plugin later; not wired in this change).

## Existing sections updated

- delegation-kit SPEC §usage-verdict gains the conditional append and the
  7-day axis; the sentence "The five-hour window is the only pause axis;
  7-day keys are ignored" is replaced (7-day keys are the armed-when-present
  second axis). The failure-mode list gains the per-axis dead-window note.
- §The usage.txt contract gains the optional-keys paragraph: the three
  mandatory lines are unchanged; `account`/`tier`/`seven_day_used_pct`/
  `seven_day_resets_at`/`tokens_in`/`tokens_out` are documented optional,
  keys the verdict does not read pass through.
- §Layout and configuration knob table gains `DELEGATION_KIT_USAGE_HISTORY`
  and `DELEGATION_KIT_PAUSE_PCT_7D`.
- §Testing: `usage-tests/` gains cases for the append (on/off, STALE
  appends nothing, optional keys passed through vs omitted), for the 7-day
  axis (weekly PAUSE while 5h is comfortable, axis named in the output,
  absent keys disarm, dead weekly window does not pause), and for the trend
  reporter over a fixture history (per-axis segmentation at a reset
  boundary, at a `login_at` change, and at an account or tier change;
  per-account grouping reunites a switch-back; spike-then-correction
  flagged not averaged). No fixture pair owed — neither script is a gate.
- Consumer follow-through at build (this repo): the session-context hook's
  budget line and CLAUDE.md's budget-check bullet say "5h window" — reword
  to name the axis the verdict reports.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
