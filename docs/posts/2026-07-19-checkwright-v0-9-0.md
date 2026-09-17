---
release: v0.9.0
---

# Checkwright v0.9.0

*2026-07-19*

Checkwright is the verification layer under agent orchestration, and this
release makes two of its own delivery-tooling signals tell the truth. A cost
report that carried a confounded number beside a clean one, and a recovery
contract that read completed work as interrupted, were both reporting something
other than what they measured. Neither fix adds a mechanism; both remove a claim
the tooling could not support.

## Tightened gates

None.

## Renamed knobs

None.

## Behavior changes

- **drift-kit/templates/economics.md** — the `/economics` narrative no longer
  chains delegation-kit's usage-trend as a third cost surface. The chain narrows
  to `overhead-meter` → `stage-economics`, and `stage-economics` becomes the sole
  cost-attribution surface. The budget-% rate-window footprint is account-wide —
  confounded by overlapping sessions and by a second operator on the same account
  — so it was the wrong instrument for per-iteration attribution, and carrying it
  beside the per-transcript token SSOT put a confounded advisory number next to a
  clean one a reader could over-trust as the iteration's cost. **This removes a
  role, not a tool:** `bin/usage-trend.sh` is untouched, and usage-verdict and
  the agent-budget-guard all stand. Reconcile only if you copied the template out
  and expect three cost surfaces in your report.

- **delegation-kit/SPEC.md** — the resume-journal recovery contract's
  DONE-absence clause is rescoped onto whether the supervisor consumed the
  agent's return. Agents routinely complete without appending `DONE`, so the
  former unconditional reading ("a journal without `DONE` means that unit was
  interrupted") false-read finished work as interrupted. Now: on the ordinary
  path the consumed return plus the supervisor's post-commit verification *is*
  the recovery contract, so the marker is redundant and its absence implies
  nothing; only in a **cold read** — a journal found with no return ever consumed,
  the agent's session having died before returning — is the marker the sole
  signal, and there the original reading holds. The inline-findings rule is
  unchanged and is now stated as exactly that cold arm's insurance. Mirrored into
  `templates/agent-execution.md`. Reconcile if your supervisor tooling treats a
  `DONE`-less journal as an interruption signal on the ordinary path.

- **lifecycle-kit/templates/skills/scope.md** — the template's opening and
  closing lines claimed queue promotion unconditionally while its middle narrowed
  promotion to debt only where the roster splits out a dedicated authoring stage.
  Both ends now carry that conditional, and the reason is stated with its owner:
  writing the amendment *is* promoting the entry
  (canon-kit/SPEC.md §The amendment lifecycle), so the stage that authors is
  necessarily the stage that promotes. No ruling changed — this states what was already ruled, at the two
  places a scope session actually reads.

## Upgrading

Sync the vendored kit directories wholesale at `v0.9.0`, regenerate the
generated artifacts (the pre-commit hook and the graph projection), then run the
full battery.

**No allowed reds.** Tightened gates is empty, so a clean upgrade turns nothing
red; the behavior changes above are reconciled by reading, not by a gate.

The behavior changes above are declared for reading, not a mechanical scan. If a
gate reds that this note does not name, the upgrade smoke was supposed to catch
it first — [open an issue](https://github.com/checkwright/checkwright/issues),
because that is a defect in the release rather than work for you.
