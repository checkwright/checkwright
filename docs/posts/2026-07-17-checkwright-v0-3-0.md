---
release: v0.3.0
---

# Checkwright v0.3.0

*2026-07-17*

Checkwright is the verification layer under agent orchestration, and this
release folds releasing itself into that lifecycle: every iteration close now
dispositions the release boundary, and a consumer can make that disposition a
mechanical precondition on the next iteration. The work is in lifecycle-kit's
close template and its stage-entry tool — no battery gate moved, so the two gate
sections below are None and the changes are declared under Behavior changes.

## Tightened gates

None — no gate landed new or got stricter this release.

## Renamed knobs

None — nothing was renamed or removed. This release adds one knob,
`LIFECYCLE_KIT_BOUNDARY_REQUIRE` (array, default empty), inert until you point
it at a boundary-evidence file. There is nothing to re-point.

## Behavior changes

Both changes this release carries move no gate — they land here, the fixed
section for what shifts outside the battery, reconciled by reading.

- **lifecycle-kit close template** gains a release-disposition step: every close
  now reads a `release-policy` slot and either executes the consumer's release
  procedure or stamps an explicit no-release line into a disposition-evidence
  file (`<iteration> release <version|none> — <basis>`). If you adopt the close
  template as a shim, your close now owes this disposition each iteration — bind
  the slot to your release procedure, or to a plain `none`-every-iteration line
  if you have none. The disposition gains teeth through the new
  `LIFECYCLE_KIT_BOUNDARY_REQUIRE` knob: wire your disposition-evidence file into
  it and `enter-stage.sh` refuses the next iteration boundary until the closing
  iteration carries a disposition line (a missing member, a missing file, and a
  never-named closing iteration each take their fail-closed branch). Left at its
  empty default, nothing refuses — the enforcement is opt-in.
- **lifecycle-kit lead and scope templates** gain an Opening-an-iteration
  contract: a live iteration lead relays the operator's standing directive (a
  theme, never a slug list) verbatim into the scope dispatch and routes scope's
  proposed unit set back as an ordinary escalation. If you drive iterations with
  a lead, this settles who owns unit selection — scope proposes, the operator
  disposes, the lead relays.

## Upgrading

Sync the vendored kit directories wholesale at `v0.3.0` and regenerate the
generated artifacts, then run the full battery — it stays green on a clean tree,
because the two changes this release carries are not gates. The tightened-gates
bullets stay the mechanical allowed-red set (None this release); the behavior
changes above are declared for reading, not a mechanical scan.

If a gate reds that this note does not name, the upgrade smoke was supposed to
catch it first — [open an issue](https://github.com/checkwright/checkwright/issues),
because that is a defect in the release rather than work for you.
