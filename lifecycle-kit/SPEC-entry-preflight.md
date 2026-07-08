# SPEC amendment: entry-preflight

## What changes

`enter-stage.sh` pre-flights exactly one gate, `check-stage-entry`, so a
stage whose real precondition lives in another kit's gate deadlocks at
pre-commit instead of failing loudly at the flip. The attested case: entering
close with no clean validate evidence writes the flip, then evidence-kit's
`check-evidence-manifest` assertion A blocks the commit — and the `gates`
suite that would produce the missing evidence re-runs that same red gate
against the already-flipped header, self-referentially.

**New knob: `LIFECYCLE_ENTRY_PREFLIGHT`** — a consumer-config array of
`<stage>=<command and leading args>` entries. For each entry whose stage
matches the stage being entered, `enter-stage.sh` runs the command with two
appended positionals — the header-flipped temp queue copy (the same one the
built-in `check-stage-entry` pre-flight already builds) and the state file —
at the same point in the sequence, after the built-in pre-flight. A non-zero
exit refuses the transition exactly as a red `check-stage-entry` does: exit
1, findings printed, nothing written. Same advisory semantics — no
`--force`, the easy path is the compliant one; the idempotent early-exit for
an already-stamped re-run is unchanged and still precedes the pre-flights.
An empty or unset knob (the default) changes nothing.

Seam ruling: this is the second instance of the `LIFECYCLE_BOUNDARY_TRUNCATE`
pattern — a generic per-stage hook knob, no consumer surface named in the
kit; the evidence coupling lives entirely in consumer config.

**Consumer wiring (this repo):** `scripts/lifecycle-stages.sh` adds

```
LIFECYCLE_ENTRY_PREFLIGHT=('close=evidence-kit/checks/check-evidence-manifest.sh .workflow/validate-evidence.txt')
```

`check-evidence-manifest`'s argument mode (manifest, queue, state) receives
the flipped queue and state file as its trailing positionals, so its
close-entry assertion fires *before* the flip is written, with its existing
finding text pointing at `run-validate` — the deadlock becomes a refusal
with instructions. Belt-and-braces behind the validate skill's run-validate
wiring, not a replacement for it.

## Producers and consumers

- Preflight invocation — produced by `enter-stage.sh <stage>` (deployed:
  every stage skill's first step runs it); enabling config is the consumer's
  `LIFECYCLE_ENTRY_PREFLIGHT` (this repo sets the close entry above).
  Consumer: the configured command, exec'd with the two appended paths.
- Entry fields, read in `enter-stage.sh` at pre-flight time: the `<stage>`
  key — matched against the entered stage; the command string — exec'd on
  match. Refusal output — consumed by the operator at the terminal (same
  channel as the existing pre-flight refusal).

## Existing sections updated

- lifecycle-kit SPEC §bin/enter-stage.sh — the pre-flight sentence widens to
  "the built-in check-stage-entry plus each matching LIFECYCLE_ENTRY_PREFLIGHT
  command"; knob list gains the knob.
- lifecycle-kit SPEC §lib/stages.sh — knob roster gains
  `LIFECYCLE_ENTRY_PREFLIGHT`.
- evidence-kit SPEC §lifecycle-kit integration — "one generic knob" becomes
  two: truncation at the boundary, refusal at close entry; the close-entry
  assertion's enforcement point moves one step earlier for consumers that
  wire it.
- lifecycle-kit `smoke/install.sh` gains a scenario: a stub failing preflight
  refuses the entry (no writes), then passes once the stub is green.
  Advisory tooling — no fixture pair owed.

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
