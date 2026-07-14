# SPEC amendment: session-boundary

## What changes

The fresh-session-per-stage rule becomes consumer posture instead of kit
law.

**New knob** (joins lifecycle-kit/SPEC.md §Layout and configuration's
roster):

- `LIFECYCLE_KIT_SESSION_BOUNDARY` — `stage` or `iteration`; default
  `stage` (today's behavior, so existing consumers are untouched). The
  loader (`lib/stages.sh`) validates the value alongside its existing
  machine checks and exits 2 on anything else — a broken machine must not
  gate anything.

**Gate behavior** (`check-stage-evidence`): at `stage`, unchanged — within
the current iteration, two different stages may not share one session id.
At `iteration`, the gate skips only the cross-stage distinctness map; every
other assertion (stamp grammar, staleness, sentinel scoping, the
current-stage coverage stamp) holds identically. Attribution still rides
the stamps — a reused session id remains on the audit trail, it just stops
failing the gate — and the governed-surface doctrine is unchanged.

**Lead posture** (`templates/lead.md`, the missing sentence): under the
strict posture (`stage`) the lead may never run a stage inline for an
iteration it already stamped — its session id is spent, and an inline run
would be exactly the self-reported skip the gate exists to catch. Under
the relaxed posture (`iteration`) an inline stage run is the sanctioned
fallback when dispatch is blocked (e.g. the budget guard), the stamp
recording the shared id honestly.

**Tests:** fixture/unit coverage for both postures rides the existing
`gate-tests/check-stage-evidence.test.sh` (the knob is env, so one temp
tree serves both cases: a shared-id stamp file reds at `stage`, greens at
`iteration`).

This repo's own posture (`scripts/lifecycle-config.sh`) is **decided when
the unit lands, not here** — the operator leans `iteration`, with the cost
on record that the dogfood evidence then stops demonstrating the strict
posture.

## Producers and consumers

- **Producer:** consumer config (`lifecycle-config.sh` or exported env),
  loaded by `lib/stages.sh` — the loader every lifecycle gate and
  `enter-stage.sh` already sources, so the knob reaches its reader with no
  new wiring.
- **Consumer:** `check-stage-evidence.sh`, at the cross-stage
  distinctness-map transition (the only code path that reads it).
  `templates/lead.md` consumes it as posture prose (read by the lead
  session, not executed); `enter-stage.sh` does not read it — stamping is
  posture-independent.

## Existing sections updated

- lifecycle-kit/SPEC.md §Layout and configuration — the knob joins the
  roster.
- lifecycle-kit/SPEC.md §check-stage-evidence — the cross-stage invariant's
  description gains the posture qualification.
- lifecycle-kit/SPEC.md §templates/lead.md + `templates/lead.md` itself —
  the inline-run posture sentence.
- Build must also grep the "fresh session" / context-boundary claims on the
  governed prose surfaces (docs/orchestration.md, §Multi-operator
  semantics) and qualify any that state the strict rule as unconditional.
- Docs-mirror regeneration rides the SPEC edits.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
