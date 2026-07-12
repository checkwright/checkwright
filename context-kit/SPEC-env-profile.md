# SPEC amendment: env-profile

## What changes

A kit-owned local machine profile so a session adapts to the box it runs on
— package manager, toolchain versions, absent tools, and the hand-authored
gotchas a probe cannot know — without the machine's facts ever landing in
the public tree.

**Owner ruling: context-kit** — the kit owns what a session loads; no new
concern is opened.

**Probe (kit mechanism).** New `bin/env-probe.sh`: derives the profile —
OS/distro (`uname`, `/etc/os-release`), package manager (detection walk over
the known managers), toolchain versions (the probe set: bash, git, jq, awk,
python3, and the kit tools' own dependencies), absent-tools list — and
writes it as a marker-bounded generated block
(`<!-- context-kit:env:begin -->` / `:end`, via the shared gate-sdk inject
helper the resident-trigger amendment extracts) into the file named by new
knob `CONTEXT_KIT_ENV_PROFILE_FILE` (default `ENV.local.md`). The block
carries its probe date. Derivation-first: the probed half is never
hand-maintained.

**Content seam (consumer-local, gitignored).** Hand-authored gotchas — the
"no `dig`/`host`; use `getent`/DoH" class — live *outside* the markers in
the same file and survive every re-probe. The file is
`BRIEF.local.md`-class: local-only, gitignored (this repo's `.gitignore` and
the CLAUDE.md housekeeping line gain it), so machine facts stay private.

**Cadence ruling: cached projection, not per-session probe.** Env changes
rarely and hook latency is a per-session tax, so the probe runs at install
("seed your env profile" joins context-kit's install steps) and on demand;
the block's probe date is the staleness signal. No freshness gate — env
truth is not cheaply machine-verifiable and the probe is already the
derivation (the enforcement-first carve-out; a stated install step is the
enforcement).

**Surfacing.** The session-context hook template gains a guarded optional
step: if the profile file exists, emit it into the session brief (the
drift-line seam precedent; the consumer owns the file's brevity since they
author the gotchas). No always-loaded cost — the harness's weak
`Platform:` line stays the fallback where no profile exists; the value-add
is a richer, tool-consumable, harness-independent profile a script can also
read.

**Memory-off reconciliation.** context-kit/SPEC.md §The memory-off doctrine
gains the distinction line this feature forces: an *explicit, derived,
operator-curated* local file is config, not memory — the banned class is
harness-side silent accumulation; `ENV.local.md` sits with
`BRIEF.local.md` on the config side of that line.

**Dogfood.** Seed this box's profile locally (the re-derived toolchain facts
from the docs-site work move into it); content stays untracked.

## Producers and consumers

- **Producer:** `bin/env-probe.sh`, run at install and on operator demand —
  the install step is the enabling config that makes the producer reachable
  in every adoption.
- **Consumer:** the session-context hook (emits the file at SessionStart);
  any gate or script reading the profile file directly (the tool-consumable
  half of the point).
- **Knob reader:** `CONTEXT_KIT_ENV_PROFILE_FILE` read by the probe and the
  hook step; documented in §Layout and configuration's roster.
- **Field readers:** probe date (operator's staleness judgment), probed
  facts + gotchas (the session adapting commands to the box).

## Existing sections updated

- `context-kit/SPEC.md` — new §bin/env-probe contract; §The session-context
  hook gains the optional step; §The memory-off doctrine gains the
  config-vs-memory distinction; §Layout and configuration gains the knob;
  install steps gain the seeding step.
- `context-kit/templates/session-context.sh` + this repo's
  `scripts/session-context.sh` — the step.
- `.gitignore` + `CLAUDE.md` §Housekeeping — the local file registered.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
