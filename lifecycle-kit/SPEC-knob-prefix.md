# SPEC amendment: lifecycle-knob-prefix

lifecycle-kit's knobs ride a bare `LIFECYCLE_` prefix (the roster in
lib/stages.sh — stages, predecessor map, entry preflight, boundary
truncate, and the rest) while every other kit follows the `<KIT>_KIT_`
shape (`CANON_KIT_`, `QUEUE_KIT_`, `DELEGATION_KIT_`, …); the one
conforming knob is `LIFECYCLE_KIT_STAGES_FILE`. Rename the roster to the
`LIFECYCLE_KIT_` prefix, and rule the compat story rather than just
sweeping the names.

## What changes

- **Every `LIFECYCLE_`-prefixed knob renames to `LIFECYCLE_KIT_`** —
  the roster is derivable (`grep -rho 'LIFECYCLE_[A-Z_]*'` over the kit),
  so this amendment names the transformation, not a hand list.
- **Config-file naming joins the parity sweep.** Rule: full parity with
  the sibling kits' config seam — `LIFECYCLE_KIT_STAGES_FILE` becomes
  `LIFECYCLE_KIT_CONFIG_FILE` and the auto-sourced consumer file
  `lifecycle-stages.sh` becomes `lifecycle-config.sh` (under
  `GATE_SDK_GATES_DIR` resolution, unchanged), matching the
  `<KIT>_CONFIG_FILE` / `<kit>-config.sh` shape of every content-bearing
  kit. Half a rename would leave the kit nonconforming on the very seam
  the unit exists to align.
- **Compat ruling: hard rename, no shim.** No release tag exists and no
  external consumer can have vendored the kits (the first tag is a
  launch-comms prerequisite still pending); the platform migration has
  not begun. A compat shim (read old name when new unset, warn) would be
  code + fixtures + a deprecation window for zero readers. Precedent
  stated for the merged SPEC: **before the first release tag, knob
  renames are compat-free**; from the first tag onward, a rename owes the
  queue-bound deprecation mechanism (the deprecation-lifecycle rung) and
  a tightened-gates/release-note declaration (the upgrade-path rung).
- **The sweep is full-surface** (the rename-sweep doctrine rule): lib,
  bin, checks, templates (the skill templates' binding prose names
  `LIFECYCLE_BOUNDARY_TRUNCATE`), gate-tests fixtures, smoke,
  lifecycle-kit/SPEC.md and README, this repo's consumer surfaces
  (`scripts/lifecycle-stages.sh` content + filename,
  `.claude/commands/*.md` binding shims, CLAUDE.md, docs/), and the
  `check-kit-ref-liveness` knob-resolution surface. Done-gate is a
  tree-wide grep for the bare prefix showing only the `LIFECYCLE_KIT_`
  form (a text-level completeness scan, not a green battery alone) — note
  `check-knob-citation` derives both prefix forms from the kit dir name,
  so the short form stays validly derived and simply matches nothing.

## Producers and consumers

- Knob producers are unchanged in kind: lib/stages.sh defaults and the
  consumer config file; consumers are the kit's bin/checks/templates.
  The rename changes names only — no new state, event, or interface
  beyond the names themselves; causal completeness reduces to the sweep
  finding every reader of an old name (the done-gate grep is the named
  check).
- The renamed config file's producer is this repo (git mv +
  content-preserving rename); its consumer is lib/stages.sh's fallback
  resolution and any explicit `LIFECYCLE_KIT_CONFIG_FILE` setting. The
  fallback default changes in the same commit as the file rename — a
  split would orphan the config silently (the stage machine falls back to
  platform defaults without error when no config file is found; that
  silent-fallback behavior is by design for zero-config consumers, which
  is precisely why the rename must be atomic).
- CI, the generated hook, and run-gates.sh consume the knobs only through
  the kit's own scripts — no workflow or hook edit expected; verify
  rather than assume during build.

## Existing sections updated

- lifecycle-kit/SPEC.md §Layout and configuration — the knob roster and
  config-file name, plus the compat-precedent sentence above.
- lifecycle-kit/SPEC.md §lib/stages.sh (and every SPEC section citing a
  knob by name) — swept by the rename.
- scope.md skill bindings in this repo's `.claude/commands/` — the
  evidence-reset binding cites `LIFECYCLE_BOUNDARY_TRUNCATE`.
- CLAUDE.md — cites `LIFECYCLE_BOUNDARY_TRUNCATE` in the scope binding
  prose if present; swept.

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
      retired; nothing dangles (the bare-prefix grep is the named scan).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
