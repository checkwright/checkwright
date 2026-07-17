---
release: v0.2.0
---

# Checkwright v0.2.0

*2026-07-17*

The first upgrade. Checkwright is the verification layer under agent
orchestration, and `v0.1.0` was a baseline nobody had to move off yet — this
release is the first one that owes you a migration story. Everything below is
that story, in the shape the [upgrade contract](../install.md) fixes.

This release also ships the contract's own proof. The upgrade smoke
(`gate-sdk/bin/upgrade-smoke.sh`) drives both phases against a scratch consumer
and asserts the red set stays inside the declaration below, so a gate that reds
your tree without a bullet here is a bug in the release rather than a surprise
you absorb. `v0.1.0 → v0.2.0` is the first upgrade it runs.

## Tightened gates

- `check-exec-bit` — new (gate-sdk). Every tracked `*.sh` matching the exec
  globs must carry git index mode `100755`. A by-path-invoked script committed
  non-executable degrades silently to a skipped check in a fresh clone, which is
  a gate that looks green because it never ran; this reds instead. Expect it red
  on first run if your tree carries one — `git update-index --chmod=+x` is the
  fix. Scope is yours through `GATE_SDK_EXEC_GLOBS` and `GATE_SDK_EXEC_PRUNE`.
- `check-test-hermetic` — new (gate-sdk). Every bespoke `gate-tests/*.test.sh`
  sources `lib/test-hermetic.sh` or carries a `# hermetic-exempt:` marker. A
  fixture test that reads the live tree passes for the wrong reason; the marker
  is the audited opt-out, not a silent one.
- `check-merge-attrs` — new (lifecycle-kit). Bidirectional parity between the
  derived iteration-scoped supersede set and the `merge=iteration-scoped` lines
  in `.gitattributes`. It reds a missing driver line, and — the reverse
  direction — an ours-driver attribute smuggled onto a path outside the set.
- `check-prose-tells` — new (canon-kit). Threshold-gated mechanical AI-prose
  tells over `CANON_KIT_PROSE_TELL_GLOBS`. That roster is your editorial scope
  and defaults to empty, so an unconfigured consumer gets a clean pass: this
  gate cannot red your upgrade until you point it at your own prose.
- `check-amendment-queue` — stricter (canon-kit). A `[spec:]`-tagged entry
  sitting in an active non-feature section now reds: a spec-ready entry belongs
  in a feature section. `v0.1.0` caught missing tags and misfiled deferred
  entries, but let this one through.
- `check-stage-entry` — stricter (lifecycle-kit). The drain-entry queue-empty
  assertion now runs at the drain successor's entry too, and the new
  `[drain-exempt:]` tag does not exempt there — nothing may stay active past the
  drain stage, which is what makes the exemption safe to grant at drain entry
  itself. A `[drain-exempt:]` carrying an empty reason is malformed and reds;
  the reason is the audit trail.
- `check-tag-lead-line` — stricter (queue-kit). `drain-exempt` joins the
  governed tag set, so a `[drain-exempt:]` written off its bullet's lead line
  now reds. The lead line is the only line the tag readers scan, so a tag below
  it is invisible to the gate that would have honoured it.

## Renamed knobs

None — nothing was renamed and no knob was removed. This release adds knobs
(the `*_EXTRA` append arrays, `EVIDENCE_KIT_PARSER_<suite>`,
`LIFECYCLE_KIT_SESSION_BOUNDARY`, `GATE_SDK_EXEC_GLOBS` / `GATE_SDK_EXEC_PRUNE`,
`DRIFT_KIT_METRIC_DIR`, `DELEGATION_KIT_REFRESH_CMD` /
`DELEGATION_KIT_REFRESH_MIN_AGE`, `CONTEXT_KIT_SESSION_ROLE_FILE`, and the
`CANON_KIT_PROSE_TELL_*` set), and each defaults to the prior behaviour. There
is nothing to re-point.

## Behavior changes

Four changes alter what the kits do without moving a battery gate. None
announces itself as a red on a clean tree, so reconcile them by reading.

- **`<KIT>_CONFIG_FILE` seams fail closed.** A `<KIT>_CONFIG_FILE` knob that is
  *set* to a path that does not exist now exits 2 and names the knob. At
  `v0.1.0`, gate-sdk, context-kit, doctrine-kit, drift-kit, and guard-kit
  silently fell back to the default `scripts/<kit>-config.sh`, so a typo'd or
  stale path ran the battery against default configuration and passed; the
  remaining kits already refused, and this release converges those five on that
  shape. An unset knob still resolves the default path, and a missing default is
  still no error, so an unconfigured consumer sees nothing change. This is not a
  tightened-gate bullet because it is not one gate: `gate-sdk/lib/gate.sh` is
  sourced by every gate, so a broken `GATE_SDK_CONFIG_FILE` reds all of them at
  once rather than naming a migration, and `gate-sdk/bin/enforcement-map.sh`
  refuses the same way, verifying every explicitly set registry knob before its
  first stdout byte. Check your seams before you sync — the failure is loud and
  names the knob to fix.
- **`bin/run-validate.sh` fails closed on an unbaselined failure.** evidence-kit's
  per-scenario diff — shared by `bin/run-validate.sh` and `bin/diff-baseline.sh`
  — now classifies an observed `fail` with no baseline row as a new failure. It
  previously scored only against the baseline's own rows, so a scenario that
  failed while absent from the manifest passed silently, precisely the hole a
  baseline exists to close. If your baseline is incomplete, a previously green
  validate goes red and names the scenario: the gap surfacing, not a regression.
  An absent `pass` is the stated classification cost and an absent `ignore` is a
  non-verdict; neither is a red.
- **guard-kit ruleset** gains a rule steering a bare `rm` of a tracked path to
  `git rm`, so a command that previously passed the bash guard now blocks with
  that suggestion.
- **`bin/usage-verdict.sh`** (delegation-kit) compares its pause thresholds
  at-or-over rather than strictly over, so a window sitting exactly on a
  threshold now pauses where it previously proceeded.

## Upgrading

Run the two phases the [install guide](../install.md) §The upgrade contract
owns: sync the vendored kit directories wholesale at `v0.2.0`, regenerate the
pre-commit hook and the graph projection, then run the full battery and
disposition the red set against the tightened-gates bullets above. The
behavior-changes section holds what shifted outside the battery — reconcile
those by reading, they will not surface as reds on a clean tree.

If a gate reds that this note does not name, the upgrade smoke was supposed to
catch it first — [open an issue](https://github.com/checkwright/checkwright/issues),
because that is a defect in the release rather than work for you.
