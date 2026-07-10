# SPEC amendment: memory-off-enforcement

## What changes

The no-per-user-memory stance graduates from a consumer CLAUDE.md rule to
kit doctrine plus enforcement, split on the tree-vs-environment seam.

**Doctrine (new SPEC section).** Harness memory is an always-loaded surface
the meter cannot read and no gate scans — standing per-session context that
accretes outside the tier contract, ungoverned by construction. The
replacement routes stay the star topology the methodology already runs:
durable facts to doc owners (the knowledge-friction loop), iteration-scoped
attention to the lesson channels, private context to the operator's local
brief. Blast radius honesty rides the doctrine: gates hold the tree
regardless of a polluted session, so this is a lightweight gate pair, not
machinery.

**Gate A — `checks/check-settings-pins.sh`** (hermetic, `precommit`): every
pin in `CONTEXT_KIT_SETTINGS_PINS` (default `scripts/settings-pins.conf`;
grammar one `<jq path> = <expected JSON>` per line, `#` comments and blanks
ignored) holds against the tracked harness settings file
(`CONTEXT_KIT_SETTINGS_FILE`, default `.claude/settings.json`) — the
identity.conf pattern pointed at harness config. General-purpose by
construction: any settings key is pinnable; this consumer's first pins hold
the memory-disabling keys. Absent pins file: clean skip (opt-in surface).
An unreadable settings file, a pin path absent from it, or no `jq`:
fail-closed (exit 2).

**Gate B — `checks/check-memory-off.sh`** (local-environment class, the
check-identity precedent): red when the harness's per-project memory dir
(`CONTEXT_KIT_MEMORY_DIRS`, glob default the current harness layout under
the operator's home) contains content files, or when the untracked
settings.local.json overrides a pinned memory key. CI-neutral and clean
where the surface is absent, with the fail-open-on-absent caveat stated in
the clean parenthetical — an absent dir proves nothing about another clone.
The paths are knobs because the harness layout moves (the
plugin-marketplace ruling: design against the live layout, keep it config).

**Session-context backstop.** The session-context hook template gains one
optional warning line when the memory dir is non-empty — gates fire only at
commit; the hook surfaces pollution at session start, between commits.

## Producers and consumers

- Gate A producer: the generated pre-commit hook / `run-gates.sh`
  (`# graph:` couples the settings file and the pins file). Consumer: the
  committing operator; each failed pin reported once with path, expected,
  and actual — all three fields read in that finding.
- Gate B producer: `run-gates.sh` on the operator's machine (its graph
  couples no tree path; it scans environment). Consumer: the operator; in
  CI the surface is absent and the gate reports the stated clean-skip.
- `CONTEXT_KIT_SETTINGS_FILE`, `CONTEXT_KIT_SETTINGS_PINS`,
  `CONTEXT_KIT_MEMORY_DIRS` are read by their gate at startup; the hook
  template reads `CONTEXT_KIT_MEMORY_DIRS` at session start for the warning
  line.
- Downstream docs edit at build: docs/methodology.md cites the doctrine
  section downward; CLAUDE.md's no-memory rule stays the consumer statement
  and gains the gate pair as its enforcement cite.

## Existing sections updated

- §The session-context hook (template): the warning line.
- §Layout and configuration: the three knobs, `<KIT>_<KNOB>` shape, this
  repo's layout as defaults.
- §Out of scope stays honest: the kit governs presence/pins, never memory
  *content* — a session's live context is not a scannable surface.
- Fixtures: good/bad pairs for both gates (settings + pins fixtures; a
  memory-dir fixture reached via the knob override into the case dir, the
  established pattern for environment-scanning gates); registration in this
  repo's `gates.list`.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
