# SPEC amendment: terminology-renames

A governance ruling spanning two kits (hence repo root): two renames, ruled
by the user 2026-07-07, done now because a rename is cheapest while no
external consumer exists.

## What changes

1. **friction-kit → guard-kit.** The kit's core is `lib/guard.sh`; "friction"
   is source-platform insider vocabulary, "guard" says what the kit does.
   Renamed surfaces:
   - the kit directory `friction-kit/` → `guard-kit/` (all contents ride);
   - the consumer config `scripts/friction-config.sh` → `scripts/guard-config.sh`,
     and the template `templates/friction-config.sh` → `templates/guard-config.sh`;
   - every `FRICTION_KIT_*` knob → `GUARD_KIT_*` (same `<KIT>_<KNOB>` shape);
   - prose/registry references: README.md kit table, CLAUDE.md,
     `.claude/commands/close.md` (friction-triage step), context-kit and
     delegation-kit SPEC cross-references, `gate-sdk/checks/check-shellcheck.sh`
     and `spec-kit/checks/check-comment-tier.sh` kit-root lists.
   - **Not renamed:** the *concept* "permission friction" and the friction-log
     artifact names (`friction.log`, scan-prompts, close-stage friction
     triage) — the kit is the guard framework; friction is the phenomenon it
     measures. `scripts/bash-guard.sh` keeps its name.

2. **delegation-kit `bin/usage-gate.sh` → `bin/usage-verdict.sh`.** "Gate"
   means exactly one thing product-wide: a commit-blocking check under
   gate-sdk's four contracts. This script emits a budget verdict
   (OK/PAUSE/STALE) for dispatch decisions — it blocks nothing. Renamed
   surfaces: the script, any `USAGE_GATE_*`-shaped knobs in
   `lib/delegation.sh` / `templates/delegation-config.sh` /
   `scripts/delegation-config.sh`, `bin/run-usage-tests.sh`,
   `templates/statusline-usage.sh`, the `/agent-execution` skill and its
   templates, delegation-kit README/SPEC, CLAUDE.md.

Both renames are one mechanical sweep each: `git mv` + a grep-driven
reference pass; no behavior change.

## Producers and consumers

No new state, event, or interface — every producer/consumer pair already
exists and only the names change. The causal check reduces to: after the
sweep, a tree-wide grep for `friction-kit`, `FRICTION_KIT_`,
`friction-config`, `usage-gate`, and `USAGE_GATE` finds no live reference
(historical mentions in Done/Lessons queue sections and git history are
exempt); the full gate battery and both kits' fixture/guard-test runners
stay green.

## Existing sections updated

At merge, the renames land in the canonical specs as the new names
throughout (friction-kit/SPEC.md — merged as guard-kit/SPEC.md — and
delegation-kit/SPEC.md §usage-gate); no section keeps the old name except
nothing — the old names do not survive as aliases. README.md kit table row
and CLAUDE.md references updated in the same commit.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition. (Vacuous here: renames only.)
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
