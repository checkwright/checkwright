# SPEC amendment: demand-driven-usage-refresh

Demand-driven refresh replaces timer polling as the usage-snapshot freshness
mechanism: the decision point (a verdict call) triggers the poll, so the budget
guard and any verdict caller read live data instead of whatever the last
statusline render left behind. The first live poll proved the gap: the snapshot
said 2-5% while the endpoint said 28% — a delegated build burning while the
supervising session (the push producer's only trigger) idled. Bundled by the
same operator ruling: the login-reroute hoist and the pause-threshold boundary
fix, both verdict-ordering defects in `bin/usage-verdict.sh`.

## What changes

1. **`DELEGATION_KIT_REFRESH_CMD`** (string, default empty) — a consumer
   command `usage-verdict` runs before reading the snapshot. Empty keeps
   today's read-only behavior. **Fail-soft**: a non-zero refresh exit leaves
   the snapshot untouched and the verdict proceeds on the cached file — the
   existing staleness machinery judges its trust. `usage.txt` survives as
   last-known-good cache, source-agnostic seam, and test seam — never deleted
   (operator ruling on record in the queue entry).

2. **`DELEGATION_KIT_REFRESH_MIN_AGE`** (seconds, default `60`) — the refresh
   short-circuit: the refresh command runs only when the snapshot is missing,
   unreadable, or its `updated_at` age is at least this value. The statusline
   calls `usage-verdict` on every render for trend sampling; without the floor
   a configured refresh would hammer the endpoint on the render path. At
   dispatch-decision time a stale-enough snapshot still polls.

3. **Login-reroute hoist** — the post-login-lag reroute moves ahead of the
   pause-axis comparisons: within `LOGIN_WINDOW` of an auth event, *every*
   parsed verdict routes to STALE, not only a would-be PAUSE. Today the
   reroute lives inside the PAUSE branch, so a lagging pct at-or-under the
   threshold prints OK — a fresh-looking chimera (the producer stamps the new
   account id while pct/resets_at still carry the dead login's window). The
   budget guard treats STALE as advise, never block, so the hoist creates no
   dispatch blackout. Check order after the hoist: parse → RESET-OK →
   age-STALE → login-STALE → pause axes → OK.

4. **Threshold boundary** — both pause-axis compares become at-or-over
   (`>=`): a reading exactly at `DELEGATION_KIT_PAUSE_PCT` /
   `DELEGATION_KIT_PAUSE_PCT_7D` pauses. Fail-closed direction: the boundary
   reading is judged at the limit, not under it. Verdict messages read
   "at or over" accordingly.

Interaction noted for calibration (consumer-side, no kit default change):
a refresh inside `LOGIN_WINDOW` rewrites `updated_at`, but the server-fed pct
may still lag the login by about a minute — the hoisted reroute correctly
keeps those readings STALE for the window's duration.

## Producers and consumers

- **`DELEGATION_KIT_REFRESH_CMD`** — produced by consumer config; this repo's
  `scripts/delegation-config.sh` sets it to the poll producer
  (`bash delegation-kit/templates/usage-poller.sh`) in this amendment's build,
  which is the deployed enabling config (not test-only). Consumed by
  `usage-verdict` at verdict time — the single reader. The kit default stays
  empty: an unconfigured consumer keeps the push/timer producers unchanged.
  The template runs **in place**: this repo has no `scripts/usage-poller.sh`
  copy and the poller carries no local edit, so a copy would only drift
  (`statusline-usage.sh` is the in-place precedent in `.claude/settings.json`;
  `agent-budget-guard.sh` is copied because its block message is consumer
  prose). §The usage.txt contract's scheduling example names
  `scripts/usage-poller.sh` — the same paragraph this amendment rewrites, so
  the rewrite presents the copy as one option rather than the shape, and the
  in-place config below contradicts nothing. Surfaced by the align audit.
- **`DELEGATION_KIT_REFRESH_MIN_AGE`** — produced by consumer config or the
  default; read by `usage-verdict` alone (the short-circuit compare).
- **The refreshed snapshot** — produced by the refresh command through the
  existing `usage.txt` wire contract (no new key, `tmp`+`mv` atomicity as
  before); consumed by `usage-verdict`'s existing parse.
- **Hoisted STALE / at-or-over PAUSE lines** — consumed by
  `agent-budget-guard.sh`'s verbatim relay (advise on STALE, block on PAUSE)
  and by the trend log via the existing `append_sample` path; no exit-code or
  line-shape change, so existing callers keying the `-> <verdict>` arrow are
  unaffected.

## Existing sections updated

- **§usage-verdict** — failure mode 3 (post-login lag): the reroute is
  unconditional within the window and precedes the axis compares; the pause
  compares are at-or-over; the refresh step joins the flow description.
- **§The usage.txt contract** — the poller paragraph's "without that timer
  entry the producer is dead; the timer entry is the enabling config" sentence
  gains the second sanctioned scheduling mode: `DELEGATION_KIT_REFRESH_CMD`
  runs the same one-cycle poller at verdict time; the timer remains for
  consumers wanting continuous trend density.
- **§Layout and configuration** — two knob rows (`REFRESH_CMD`,
  `REFRESH_MIN_AGE`).
- The interim discipline recorded in the queue entry (run the poller by hand
  before each dispatch) dies when this lands.

## Tests

`usage-tests` suite additions: refresh-armed-stale (stub `REFRESH_CMD` writes
a fresh snapshot; the verdict reads the refreshed values), refresh-fail-soft
(non-zero stub; verdict proceeds on the cached snapshot), refresh-skip-fresh
(young snapshot; the stub is not invoked), login-hoist (fresh credentials
mtime with an under-threshold pct routes to STALE), boundary (pct exactly at
the threshold pauses).

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
