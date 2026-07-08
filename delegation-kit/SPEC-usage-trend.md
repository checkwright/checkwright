# SPEC amendment: usage-trend

## What changes

`usage-verdict` answers "may I dispatch right now"; nothing tracks how the
budget *evolves* across a session's prompts and tool calls, so Checkwright's
own footprint is invisible and a PAUSE arrives without a trajectory behind
it. This adds a sampling log and a trend reporter — advisory tooling, not a
gate.

**Usage history log.** New knob `DELEGATION_KIT_USAGE_HISTORY` (default
empty = sampling disabled; this repo's `scripts/delegation-config.sh` sets
`.tmp/usage-history.log` — a measurement, gitignored). When non-empty,
`usage-verdict.sh` appends one sample line after every successfully parsed
snapshot, whatever the verdict — the raw harness-reported values, verbatim;
write-time smoothing or correction is forbidden (a later corrective push is
evidence about the earlier sample, and only the reader has both):

```
updated_at=<epoch> pct=<float> resets_at=<epoch> verdict=<word> login_at=<epoch>[ tier=<word>][ tokens_in=<n> tokens_out=<n>]
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
keys a producer writes when its source exposes them: `tier=<word>`
(subscription tier — Pro/Max5x/Max20x — the denominator behind the pct) and
`tokens_in=<n>`/`tokens_out=<n>` (cumulative token counts, the axis that
matters to API-billed consumers for whom the five-hour pct is not the
binding budget). `usage-verdict` ignores keys it does not read — the verdict
contract is unchanged — and passes them through to the sample line. Build
verifies which of these the harness's rate-limit/statusline payload actually
carries: a key with no live producer in the shipped template is dropped from
the template (the contract keeps it defined for third-party producers), per
the dead-producer rule.

**Trend reporter.** New `delegation-kit/bin/usage-trend.sh`: reads the
history, reports footprint evolution. The source signal is known-noisy —
rolling-window readings spike and revert-down when a harness over-report is
corrected by the next push — so the design separates signal from noise by
the window's physical constraint: *within one segment, true usage never
decreases.* Mechanics:

1. **Segment** samples by the triple (`resets_at`, `login_at`, `tier`) — a
   timer reset, a `/login` (multi-account operation legitimately drops the
   pct mid-window), or a tier change each starts a segment; only
   within-segment comparisons are meaningful, so an account switch is a
   boundary, not a flagged anomaly.
2. **Flag** any sample whose pct is below an earlier sample in the same
   segment as a monotonicity violation: the downward correction indicts the
   elevated sample(s) before it as reader noise, and both sides of the
   violation are excluded from rate math, never averaged in. Median-of-3
   smoothing resolves single-sample spikes.
3. **Report** per segment: first/last smoothed pct, pct-per-hour rate,
   token deltas when token keys are present, tier, sample count, and
   suspect-sample count — a high suspect ratio means the producer is
   unreliable and no number from that segment is trusted.

Exit codes: 0 report emitted; 2 knob unset or history missing/unreadable
(fail-closed, mirroring usage-verdict's STALE discipline). Never 1 — it
renders no verdict; `usage-verdict` stays the sole pause authority.

## Producers and consumers

- Sample line — produced by `usage-verdict.sh` (deployed per session-start
  and per Agent dispatch in this repo; enabling config:
  `DELEGATION_KIT_USAGE_HISTORY` in `scripts/delegation-config.sh`),
  optionally by the statusline template. Consumed by `usage-trend.sh`.
- Optional snapshot keys (`tier`, `tokens_in`, `tokens_out`) — produced by
  `templates/statusline-usage.sh` from the harness payload (build verifies
  which are actually exposed and ships only those); consumed by
  `usage-verdict.sh`'s pass-through into the sample line.
- Field readers, all in `usage-trend.sh`: `updated_at` — sample ordering and
  the rate denominator; `pct` — the trend value; `resets_at`, `login_at`,
  `tier` — the segment key (read at segmentation); `verdict` — annotates
  where PAUSE onsets fall in the report; `tokens_in`/`tokens_out` — the
  per-segment token deltas. No field without a reader.
- `usage-trend.sh` output — consumed by the operator (and by drift-kit as a
  candidate KPI plugin later; not wired in this change).

## Existing sections updated

- delegation-kit SPEC §usage-verdict gains the conditional append (one
  sentence — the verdict contract itself is unchanged).
- §The usage.txt contract gains the optional-keys paragraph: the three
  mandatory lines are unchanged; `tier`/`tokens_in`/`tokens_out` are
  documented optional, unknown keys ignored by the verdict.
- §Layout and configuration knob table gains `DELEGATION_KIT_USAGE_HISTORY`.
- §Testing: `usage-tests/` gains cases for the append (on/off, STALE
  appends nothing, optional keys passed through vs omitted) and for the
  trend reporter over a fixture history (segmentation at a reset boundary,
  at a `login_at` change, and at a tier change; spike-then-correction
  flagged not averaged). No fixture pair owed — neither script is a gate.

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
