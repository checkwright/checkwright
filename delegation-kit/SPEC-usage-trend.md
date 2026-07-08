# SPEC amendment: usage-trend

## What changes

`usage-verdict` answers "may I dispatch right now"; nothing tracks how the
budget *evolves* across a session's prompts and tool calls, so Checkwright's
own footprint is invisible and a PAUSE arrives without a trajectory behind
it. This adds a sampling log and a trend reporter — advisory tooling, not a
gate.

**Usage history log.** New knob `DELEGATION_KIT_USAGE_HISTORY` (default
empty = sampling disabled; this repo's `scripts/delegation-config.sh` sets
`.tmp/usage-history.tsv` — a measurement, gitignored). When non-empty,
`usage-verdict.sh` appends one sample line after every successfully parsed
snapshot, whatever the verdict:

```
<updated_at>\t<five_hour_used_pct>\t<five_hour_resets_at>\t<verdict-word>
```

Append-only, no rotation in-kit (the boundary-truncate/tmp-prune conventions
own cleanup). A STALE exit from an unreadable snapshot appends nothing — a
sample the gate would not trust is not history. `templates/statusline-usage.sh`
documents the same append as an optional dense-sampling producer; the verdict
path is the one this repo deploys (session-context hook + the Agent budget
guard already invoke `usage-verdict` per session and per dispatch).

**Trend reporter.** New `delegation-kit/bin/usage-trend.sh`: reads the
history, reports footprint evolution. The source signal is known-noisy —
rolling-window readings spike and revert — so the design separates signal
from noise by the window's physical constraint: *within one live window,
true usage never decreases.* Mechanics:

1. **Segment** samples by `five_hour_resets_at` — a reset boundary starts a
   segment; only within-segment comparisons are meaningful.
2. **Flag** any sample whose pct is below an earlier sample in the same
   segment as a monotonicity violation: either it or the spike before it is
   reader noise. Median-of-3 smoothing resolves single-sample spikes;
   flagged samples are excluded from rate math, never averaged in.
3. **Report** per segment: first/last smoothed pct, pct-per-hour rate,
   sample count, and suspect-sample count — a high suspect ratio means the
   producer is unreliable and no number from that segment is trusted.

Exit codes: 0 report emitted; 2 knob unset or history missing/unreadable
(fail-closed, mirroring usage-verdict's STALE discipline). Never 1 — it
renders no verdict; `usage-verdict` stays the sole pause authority.

## Producers and consumers

- Sample line — produced by `usage-verdict.sh` (deployed per session-start
  and per Agent dispatch in this repo; enabling config:
  `DELEGATION_KIT_USAGE_HISTORY` in `scripts/delegation-config.sh`),
  optionally by the statusline template. Consumed by `usage-trend.sh`.
- Field readers, all in `usage-trend.sh`: `updated_at` — sample ordering and
  the rate denominator; `five_hour_used_pct` — the trend value;
  `five_hour_resets_at` — the segment key; `verdict-word` — annotates where
  PAUSE onsets fall in the report. No field without a reader.
- `usage-trend.sh` output — consumed by the operator (and by drift-kit as a
  candidate KPI plugin later; not wired in this change).

## Existing sections updated

- delegation-kit SPEC §usage-verdict gains the conditional append (one
  sentence — the verdict contract itself is unchanged).
- §Layout and configuration knob table gains `DELEGATION_KIT_USAGE_HISTORY`.
- §Testing: `usage-tests/` gains cases for the append (on/off, STALE
  appends nothing) and for the trend reporter over a fixture history
  (segmentation at a reset boundary, spike flagged not averaged). No fixture
  pair owed — neither script is a gate.

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
