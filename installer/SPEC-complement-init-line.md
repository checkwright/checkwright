# SPEC amendment: complement-init-line

Pairs `companion-complement-init-line`. `init` names the install it made by its profile alone, in the `INIT:` report and in the vendoring commit's subject (installer/SPEC.md §init). A selection adjusts the profile's kit set (§Selecting kits and gates), so a companion `complement` install, `full` with three `--without-kit`, reports and commits "the full profile" while the manifest records the narrower set. This amendment has both lines name the kits a selection adds or removes.

**Kits, not the whole selection.** The lines report what was vendored and committed. A kit adjustment changes that set; a gate adjustment and a recipe change the registry and the seams, vendor no kit, and stay in the manifest (§The manifest), where `--dry-run` and the lock already show them. Naming them would lengthen a commit subject with what the subject's noun, kits, does not cover.

**Derived, not echoed.** The named kits are the difference between the selected set and the profile's, not the flags as typed. A `--with-kit` naming a kit the profile already carries adds nothing and is not named, and a re-run re-applying the recorded selection prints the same phrase as the run that recorded it.

## What changes

### (1) `init` names the install by the profile and its kit adjustments {mechanical} {user-facing: companion-complement-init-line's Deliverable, "the `INIT:` line and the commit subject naming the left-out kits or the selection"}

Replacement for `installer/SPEC.md` §init, the sentence of the paragraph beginning **It writes the kit directories** that opens "It makes **one commit**". **Not yet applied.**

> It makes **one commit** naming the install and the version, then prints a **follow-up block**: the commands that finish the setup, each with its reason. The install is named by the **profile phrase**, which its `INIT:` report shares: the profile, then ` with <kits>` for the kits the selected set holds beyond the profile's and ` without <kits>` for those it drops, each list in payload order and joined by `, ` with ` and ` before its last member, as in `full profile without doctrine-kit, lifecycle-kit and queue-kit`.

One function in `native/src/installer/init.rs` renders the phrase from the profile's kit set and the selected one. Every `INIT:` line and the commit message take it in place of the bare profile name: the vendored and staged reports and the two already-at reports. A crate test holds the phrase for no adjustment, an addition, a removal and both. The `--dry-run` header keeps its `profile:` field, since its next line lists every kit it would vendor.

### (2) The complement legs assert the phrase {mechanical}

Replacement for `installer/SPEC.md` §The consumer smoke, **The `complement` legs** paragraph's last sentence. **Not yet applied.**

> The manifest's `kits` and `selection.without-kits` must hold the left-out kits out after the install, and again after a bare re-run and a bare `update`, and the install's `INIT:` line and its commit's subject must each name every left-out kit.

## Producers and consumers

- **The profile phrase (delta 1).**
  - Producer: `init`, on every run reporting `INIT:` or committing, reached through `update` too. Its inputs are the profile's kit set and the selected one, both already resolved before any write.
  - Consumers: the adopter reading the report and the history; the complement legs (delta 2). Readers of the two lines, from `grep -rn "INIT: vendored\|vendor Checkwright kits\|INIT:" --include=*.rs --include=*.yml --include=*.md .`: the PowerShell install-smoke leg in `.github/workflows/gates.yml` matches `INIT: vendored \d+ kit` and `vendor Checkwright kits`, and the smoke's `moves.rs` and `profiles.rs` print the first line opening `INIT:`. The phrase follows the profile name, so each prefix stands and no reader is updated beyond delta 2.
  - Fields: the added and removed kit lists, read by the adopter and by delta 2's assertion.
- **Point 5:** no delta narrows a corpus.
- **Point 6:** delta 2 obliges each toolkit's `complement` leg. The members, from `ls -d companion/*/recipe`, are speckit and openspec, and each satisfies with its `companion/exclusions.list` kits named in the phrase.

## Existing sections updated

The roster below came from `grep -n "one commit\|INIT:\|complement" installer/SPEC.md` and the readers' grep above, over the tracked tree.

- `installer/SPEC.md` §init and `native/src/installer/init.rs` (delta 1).
- `installer/SPEC.md` §The consumer smoke and `native/src/emit/installer_smoke/companion.rs` (delta 2).
- `docs/installer/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — the commit subject and the `INIT:` lines keep every prefix their readers match, and the phrase only follows the profile name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Fixture suites** — the installer consumer smoke, its companion arm's complement legs included, and the installer crate cases run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
