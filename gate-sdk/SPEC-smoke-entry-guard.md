# SPEC amendment: smoke-entry-guard

Queue slug: `smoke-violation-fail-open`. The consumer-smoke `violation.sh`
scripts fail open: run outside their harness entry point they mutate the
invoking repo (observed — a bare invocation wrote and staged files into the
real tree). `install.sh` already fails closed via its `SMOKE_KIT_ROOT` guard;
this amendment makes that guard a stated contract clause for every mutating
smoke script and lands the meta-gate that holds it across the roster, so kit
#10 cannot ship the gap silently.

## What changes

1. **Contract clause** — §Consumer smoke's per-kit contract gains one
   sentence: every `smoke/` script that mutates the invoking tree
   (`install.sh` and `violation.sh` both qualify) opens with the entry-point
   guard `: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"` before its
   first mutating command, so a bare invocation refuses instead of writing
   into the caller's repo. `install.sh`'s shipped guard is the precedent
   being promoted from convention to contract; `violation.sh` joins it.

2. **Meta-gate `check-smoke-entry-guard`** (gate-sdk `checks/`, registered in
   this repo's `scripts/gates.list`): for every kit root in the roster (the
   same kit-roots resolution the existing roster meta-gates use), every
   `smoke/install.sh` and `smoke/violation.sh` must contain a
   `${SMOKE_KIT_ROOT:?` guard expansion. Missing guard = FAIL naming the
   file. Honest limit (stated in the merged section): presence is asserted,
   position before the first mutation is not — a guard below a mutating line
   passes the gate; review owns ordering. Ships with the four gate contracts:
   output, fail-closed (unreadable smoke dir = exit 2), `good/`+`bad/`
   fixture pair, self-lint; `# graph:` manifest so `gen-pre-commit` and the
   graph artifact pick it up.

3. **Nine guard insertions** — the mechanical fix: the guard line lands in
   each existing `violation.sh` (gate-sdk, lifecycle-kit, queue-kit,
   evidence-kit, delegation-kit, context-kit, doctrine-kit, site-kit,
   canon-kit), placed as in `install.sh` (immediately after
   `set -euo pipefail`, before any mutation).

## Producers and consumers

- **Guard line** — producer: the kit author at kit-landing time (this change
  retrofits the nine existing scripts); consumer: bash's `:?` expansion at
  script entry — unset `SMOKE_KIT_ROOT` aborts before the first mutating
  command. The enabling config is set by the only sanctioned caller:
  `run-consumer-smoke.sh` (and every `csmoke_vendor_and_install` caller)
  exports `SMOKE_KIT_ROOT` per kit, so harness runs are unaffected.
- **Contract clause** — producer: §Consumer smoke; consumers: kit authors
  (the kit-landing checklist reader) and `check-smoke-entry-guard`, which is
  the clause's enforcement reader.
- **Meta-gate verdict** — consumer: the pre-commit hook and battery like
  every registered gate; its FAIL line names the unguarded file (the fix
  worklist).
- No new knob, no new file convention, no new field: the gate name and the
  contract sentence are the only new names.

## Existing sections updated

- §Consumer smoke, the `smoke/violation.sh` bullet: gains the guard clause
  (change 1) — the bullet currently specifies cwd/env contract and the
  expected-gate first line only.
- §Per-component contracts: `check-smoke-entry-guard` gets its subsection at
  merge, alongside the other roster meta-gates.
- README gate-roster block (`<!-- gate-roster:begin -->`): the new gate joins
  it — `check-readme-roster` holds the parity, and the enforcement map +
  graph artifacts regenerate per CLAUDE.md's regen commands.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
