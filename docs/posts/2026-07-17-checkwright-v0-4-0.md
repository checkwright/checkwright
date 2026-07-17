---
release: v0.4.0
---

# Checkwright v0.4.0

*2026-07-17*

Checkwright is the verification layer under agent orchestration, and this
release closes a fail-open hole in the consumer-smoke contract: the smoke
scripts that mutate the invoking tree now refuse to run outside their harness,
and a new meta-gate holds that refusal across the roster. One gate landed new,
so the Tightened gates section names it and the upgrade smoke carries it as the
release's one allowed red.

## Tightened gates

- **check-smoke-entry-guard** — new gate. Every `smoke/install.sh` and
  `smoke/violation.sh` that mutates the invoking tree must open with the
  entry-point guard `: "${SMOKE_KIT_ROOT:?…}"` before its first mutating
  command, so a bare invocation outside `run-consumer-smoke.sh` refuses instead
  of writing into the caller's repo. The gate asserts guard **presence** across
  the roster (position stays review's — gate-sdk/SPEC.md
  §check-smoke-entry-guard states the honest limit). The nine shipped kit
  `violation.sh` scripts gained the guard this release, so a clean upgrade stays
  green; the gate reds only against a kit whose smoke scripts you have copied
  out or authored without the guard. It reuses `GATE_SDK_KIT_DIRS` — no new
  knob.

## Renamed knobs

None — nothing was renamed or removed.

## Upgrading

Sync the vendored kit directories wholesale at `v0.4.0` and regenerate the
generated artifacts (the pre-commit hook and the graph projection), then run the
full battery. On a clean tree it stays green: the fail-open fix inserted the
entry-point guard into all nine shipped `violation.sh` scripts and promoted it
to a gate-sdk Consumer-smoke contract clause, so the shipped roster satisfies
the new gate.

**The one allowed red.** If you have shadowed a kit or copied its `smoke/`
scripts into your own tree, `check-smoke-entry-guard` reds any mutating smoke
script of yours that lacks the `${SMOKE_KIT_ROOT:?…}` guard. The fix is the
guard line the note names, placed before the script's first mutating command —
the same one-line refusal the shipped scripts now carry.

Two changes this release moves no gate and lands here by design: the
budget-guard block-vs-advise override note moved to its point of use in
lifecycle-kit's lead template (documentation residency, nothing to reconcile),
and scope's boundary scratch wipe now spares a live lead's `.tmp/session-role`
marker so a lead outliving an iteration boundary keeps its role suppression
(context-kit's session-context signal — a fix, no knob or schema change).

Both fixed sections above are the mechanical allowed-red set: the gates a clean
upgrade may turn red. If a gate reds that this note does not name, the upgrade
smoke was supposed to catch it first —
[open an issue](https://github.com/checkwright/checkwright/issues), because that
is a defect in the release rather than work for you.
