# SPEC amendment: env-probe-refresh

<!--
  Delta artifact for the env-probe-auto-refresh task (TASK-QUEUE.md). Owning
  component: context-kit (both surfaces — bin/env-probe.sh and the
  session-context hook — are context-kit's own, so this is a single-component
  amendment: no lifecycle-kit → context-kit coupling, no cross-component audit
  trigger). Merged into context-kit/SPEC.md and deleted when the work ships.
  Describes only the delta.
-->

## What changes

Two coupled changes replace env-probe's install-time-plus-on-demand cadence
with a per-session auto-refresh, and keep the profile's `Probed` date an honest
last-*changed* signal rather than a last-*run* one.

1. **Cadence — per-session auto-refresh (was: cached projection).** The
   session-context hook runs `bin/env-probe.sh` once per session, at the point
   it already emits the profile (step 9), so a session always reads a profile
   probed against the box as it is now. The prior "env changes rarely + hook
   latency is a per-session tax" rationale is retired: on a rolling-release box
   the toolchain versions move daily-plus (a stale `python3`-class probe-set
   fact was caught on one manual refresh), and the probe measures in tens of
   ms — below the latency threshold that motivated caching. Session granularity
   (not statusline, which re-renders many times per session for a box that does
   not change mid-session) is the meaningful cadence.

2. **Change-detection in `bin/env-probe.sh`.** The probe rewrites the
   marker-bounded block **only when the probed content actually differs** from
   the block already on disk; on a no-change probe it writes nothing, leaving
   both the block body and its `Probed` date untouched. "Content" for this
   comparison is every probed line *except* the `Probed <date>` line itself
   (the date is derived from the write, so including it would make every probe
   look changed and defeat the detection). The result: the `Probed` date marks
   the last time the machine profile genuinely changed — a real staleness
   signal — and a per-session probe on an unchanged box produces no file write
   (no mtime churn, no wasted I/O).

**Opt-in preserved (envelope ruling).** The hook runs the probe **only when the
profile file already exists** — the same `[[ -f … ]]` guard that today decides
whether step 9 emits anything. Auto-refresh refreshes what the operator already
opted into; it does not auto-seed a profile (with private machine facts) for a
consumer who never ran the install-time seed. The "silent when the file is
absent — no always-loaded cost is paid where no profile was seeded" contract is
therefore unchanged; only the *refresh* of an adopted profile is automated. The
install-time seed step remains the one-time adoption action; on-demand
`env-probe.sh` remains available and unchanged in interface.

No new config knob: the hook reuses `CONTEXT_KIT_ENV_PROFILE_FILE` (already
resolved at step 9) to locate the file, and `env-probe.sh` resolves its own
config as it does today. The `PROBE_SET` array is untouched, so
`check-install-toolchain`'s name-set parity with `docs/install.md` is
unaffected.

## Producers and consumers

- **Producer** — the session-context hook (`scripts/session-context.sh` in this
  repo; `context-kit/templates/session-context.sh` the shipped template), at
  step 9, invokes `bin/env-probe.sh` before emitting the profile body. Its
  enabling condition is the existing profile-file-present guard; the probe's own
  enabling config (`CONTEXT_KIT_ENV_PROFILE_FILE` and any
  `CONTEXT_KIT_CONFIG_FILE`) is already resolved in both the hook and the probe,
  so the producer is reachable in the real deployed hook, not only in tests. The
  probe's stdout/stderr status line ("replaced/appended the env profile block")
  is suppressed at the call site so it never pollutes the session brief.
- **Consumer** — the same step-9 block, which then `cat`s the (now just
  refreshed) profile file into the brief, exactly as today. The reader is the
  session: it adapts its commands to the freshly probed box.
- **Change-detection field** — the `Probed <date>` line has one named reader:
  an operator (or a future staleness check) reading the profile to judge how
  current the machine facts are. Change-detection is what keeps that reader's
  signal truthful; no new machine-parsed field is introduced.

## Existing sections updated

**§bin/env-probe — the "Cadence" paragraph** is rewritten. Replace the current
"Cadence — cached projection, not per-session probe" paragraph with:

> **Cadence — per-session auto-refresh.** The session-context hook re-probes
> once per session, at its step-9 profile emit (§The session-context hook), so
> a session always adapts to the box as it is now — the install step seeds the
> profile once, and every session thereafter refreshes it. Change-detection
> keeps this cheap and the date honest: the probe rewrites the block only when
> the probed content differs from what is on disk (the `Probed` date line
> excluded from the comparison), so an unchanged box writes nothing and the
> date marks the last real change, not the last run. Still no freshness gate —
> env truth is not cheaply machine-verifiable and the probe is already the
> derivation (the enforcement-first carve-out; the per-session re-probe is now
> the enforcement, replacing the install-step-only cadence). The hook re-probes
> only when the profile file already exists, so a never-seeded consumer pays no
> cost and seeds nothing unbidden.

**§bin/env-probe — the seed-scaffold sentence** ("when the file is absent the
probe seeds that scaffold once, then only ever rewrites the block") gains: the
per-session hook does not trigger that seeding, because it re-probes only a
file that already exists; seeding stays a first-run/on-demand action.

**§The session-context hook — step 9** gains one clause: before emitting the
profile body, the step runs `bin/env-probe.sh` to refresh it (output
suppressed), inside the existing file-present guard — so the "Silent when the
file is absent" contract is unchanged and producer and consumer co-locate at
step 9.

## Definition of Done

- [ ] **Causal completeness** — producer (step-9 hook call, guarded, config
      reachable, output suppressed) and consumer (step-9 emit) named; the
      `Probed` date's reader named; no new field without a reader.
- [ ] **Merged with no information lost** — the Cadence paragraph, the
      seed-scaffold sentence, and step 9 rewritten in place in context-kit/SPEC.md;
      the merged spec reads coherently for a reader who never saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge; `ls context-kit/SPEC-*.md`
      shows none remain.
- [ ] **Both copies updated** — `scripts/session-context.sh` (repo consumer copy)
      and `context-kit/templates/session-context.sh` (shipped template) both carry
      the step-9 probe call; the docs mirror is regenerated
      (`scripts/gen-docs-mirror.sh --write`) after the SPEC edit.
- [ ] **Change-detection verified** — a second probe on an unchanged box writes
      nothing and preserves the prior `Probed` date; a changed probe rewrites the
      block with today's date.
- [ ] **Parity unaffected** — `PROBE_SET` unchanged, so `check-install-toolchain`
      stays green with no `docs/install.md` edit.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed
      as debt.
