# SPEC amendment: overlay-locator

`lib/test-hermetic.sh` pins each kit's tracked knob file at an empty one and cannot pin the local overlay, which has no locator: every reader takes it from the gates directory. A suite that runs from a tree carrying an overlay therefore resolves an empty tracked layer under a live overlay, a pair no seat configures. Run on a copy of this tree's gates directory with its overlays removed and one row planted, `LIFECYCLE_KIT_STAGES[] = solo` in `lifecycle-config.local.knobs`: lifecycle-kit's fixture run goes from 4 failing lines to 56, 28 of them *malformed stage-machine config — the gates cannot run*. The class is every kit's. This amendment gives the overlay a locator and has the preamble pin it.

## What changes

### (1) The overlay takes a locator, `<KIT>_LOCAL_KNOB_FILE`

Item 2 of the precedence list in gate-sdk/SPEC.md §The knob file is rewritten so the overlay is read from the file `<KIT>_LOCAL_KNOB_FILE` names when it is set non-empty, and from the gates directory otherwise {design-bearing} {user-facing: no direction is recorded; the name `<KIT>_LOCAL_KNOB_FILE`, exit 2 on a named overlay that is missing and the tracked locator leaving the overlay where it is are spec's choice under the entry's Unit set line, which leaves the deliverable to this stage, and the operator, asked, left them as spec's; they change nothing for a consumer who does not set the name}.

Replacement for item 2. **Not yet applied.**

> 2. **the local overlay**, `<KIT>_LOCAL_KNOB_FILE` when set non-empty (set but missing is exit 2), else `<gates-dir>/<stem>-config.local.knobs`, gitignored by the consumer, for values that must not be tracked. Relocating the tracked file leaves it where it is: each layer moves by its own locator alone.

- **A locator, never a knob.** It is read from the environment only, as `<KIT>_KNOB_FILE` is: a knob file cannot say where a knob file is, so a line naming it in one locates nothing.
- **Set but missing is exit 2**, the tracked locator's rule. An overlay is optional at its default path and never at a path someone named, where a mistyped value would drop the values that must not be tracked without a word.
- **Reading a named tracked file alone is refused.** It would pin the overlay with no new name, and it would take the overlay from a consumer who relocates the tracked file and keeps untracked values beside the gates directory, which the sentence this delta rewrites promises.
- **Taking the overlay from beside the named tracked file is refused** on the same promise, and it would have the preamble's pin read a sibling of a file in the system temp directory, which any process can create.
- **Refused in an open kit's file with the tracked locator and the retired one.** `admits_consumer_scalar` in `native/src/knobs/mod.rs` excludes only those two names, so drift-kit's open family would admit `DRIFT_KIT_LOCAL_KNOB_FILE = x` as a consumer scalar and export it. It excludes this one too; the open-family paragraph of gate-sdk/SPEC.md §The knob file and drift-kit/SPEC.md's name the three; and `an_open_family_admits_a_consumer_scalar_and_refuses_the_rest` gains a row for it.

The readers, by `git grep -n "local\.knobs" -- native/src gate-sdk/bin gate-sdk/lib`, each of which takes the locator:

- `native/src/knobs/mod.rs`, `load`, for every kit; `cache_key`, which keys a kit's cached layers on the locators it was loaded under and gains this one; and `wire_in`, which reads each locator from the environment before it calls `load` (the callers are `installer/doctor.rs`, `installer/init.rs` and `walk.rs`) and so reads this one, or doctor and init would resolve a different overlay than a battery run does.
- `native/src/emit/hook_launcher.rs`, `bin_from`, the launcher's read of its one knob (§git-hook), for `GATE_SDK_LOCAL_KNOB_FILE`, refusing a named file that is missing as it refuses the tracked one.
- `gate-sdk/lib/gate.sh`, `_gate_prebinary_knob`, and its PowerShell twin `Get-PrebinaryKnob` in `gate-sdk/bin/run-gates.ps1`, for `GATE_SDK_LOCAL_KNOB_FILE`.

The pre-binary readers skip a file that is absent, named or not, and leave the refusal to the binary's first gate-sdk read, as they do for a malformed line (§lib/gate.sh). The crate test that holds the two front ends to the table (`native/src/emit/front_end_parity.rs`) gains the overlay-locator row beside its tracked-locator one.

### (2) The preamble pins the overlay

`lib/test-hermetic.sh` exports `<KIT>_LOCAL_KNOB_FILE` for each kit at the shared empty knob file, and §lib/test-hermetic.sh says so in place of its overlay limit {design-bearing}.

- **The export follows the binary pin.** The library resolves the gate binary through `gate_native_bin` before it pins gate-sdk's overlay, so a seat whose overlay places the binary still reaches it. The tracked pin stays where it is, ahead of that read. The gate-sdk export sits after the binary block and outside its already-set guard, so a suite run with a pre-pinned binary is pinned too; every other kit's rides the kit loop.
- **A suite that reads a real overlay un-pins it after the source**, by the two forms the section gives for the tracked file. A suite that un-pins the tracked file alone reads the tracked config with no overlay, which is what a seat without one reads.

Replacement for the section's overlay limit, the last sentence of its second paragraph. **Not yet applied.**

> Each kit's `<KIT>_LOCAL_KNOB_FILE` is exported at the same empty file, after the binary pin below is computed, so a suite reads no layer of the tree it runs from and a seat's overlay still places the binary the suite runs. A suite that reads a real overlay un-pins `<KIT>_LOCAL_KNOB_FILE` after the source by the same two forms.

The section's first paragraph, *For each kit, gate-sdk included, it exports `<KIT>_KNOB_FILE`*, gains `<KIT>_LOCAL_KNOB_FILE` beside it, and its statement of the layers never read gains the overlay beside `<gates-dir>/<stem>-config.knobs`. The header comment of `lib/test-hermetic.sh` names the three locators it pins: `<KIT>_KNOB_FILE`, `<KIT>_LOCAL_KNOB_FILE` and the retired `<KIT>_CONFIG_FILE`. The statements of the pin in §run-gate-tests and in the hermeticity contract, and the refusal list of §git-hook, take the overlay locator beside the tracked one.

### (3) The suites the pin reaches

Each suite the new pin changes is brought to it {mechanical}. Probe: `git grep -n "local\.knobs" -- '*/gate-tests/*' '*/smoke/*' 'scripts/gate-tests/*'` for suites that write an overlay, and `git grep -n "unset [A-Z_]*_KNOB_FILE\|env -u [A-Z_]*KNOB_FILE" -- '*.sh'` for suites that un-pin.

- `gate-sdk/gate-tests/native-git-hooks.test.sh` writes `gate-sdk-config.local.knobs` into its sandbox repository's gates directory and asserts the launcher reads it. It un-pins `GATE_SDK_LOCAL_KNOB_FILE` for those cases.
- `gate-sdk/gate-tests/check-graph-cap.test.sh`, `check-graph-refs.test.sh` and `check-graph-theme.test.sh` each carry `unset DELEGATION_KIT_KNOB_FILE` and the comment above it, added so delegation-kit's tracked file is read beside the overlay the pin left read. With both layers pinned the line and its comment are deleted, and the three run on kit defaults again.
- The other un-pinning suites, evidence-kit's five and `gate-sdk/gate-tests/enforcement-map.test.sh`, un-pin a tracked file and need no edit.
- `scripts/gate-tests/subagent-stop-reader.test.sh` re-exports delegation-kit's real tracked file and moves its gates directory to avoid the seat's overlay. The pin makes the move redundant; it is left in place and needs no edit. A re-pin in the export form is reached by a third probe, `git grep -n "KNOB_FILE=" -- '*/gate-tests/*'`.

**Inferred, cannot run before build:** that the three graph suites pass re-pinned on a seat carrying a delegation-kit overlay — the locator has to exist first; the three suites run on this seat, which carries one, are the oracle.

## Producers and consumers

- **`<KIT>_LOCAL_KNOB_FILE`.** Producer: an invoker's environment; in this tree `lib/test-hermetic.sh`, which every bespoke `gate-tests/*.test.sh` sources (§check-test-hermetic), and a harness aiming a member at a synthetic overlay. Consumer: delta 1's four reader files, at each process's first resolution of the kit.
- **Rosters holding the name.** The locator is no table row, so `--emit-knob-roster` and each kit's validator are untouched. `static_names()` in `native/src/knobs/mod.rs`, which `check-docs-cmd` and `check-kit-ref-liveness` union into their known set beside the tracked locator and the retired one, gains `<KIT>_LOCAL_KNOB_FILE`, so a mention of it in prose reads as known. `native/tests/closed_reader.rs` drops every `_KIT_` and `GATE_` name from its cases' environment and so drops this one. §Layout and configuration's sentence on the two layers and its *A member's knob resolution can be aimed at a config the caller chooses* paragraph name the tracked locator and take the overlay's beside it.
- **Narrowing (point 5).** The pin removes the seat's overlay from what a sourcing suite reads. A suite that reds on *finding none* is one asserting an overlay value: `native-git-hooks.test.sh`, named in delta 3 with its remedy. No sourcing suite asserts a count over overlay rows, by the first probe of delta 3.
- **Every member's satisfying value (point 6).** The obliged corpus is the overlay readers, enumerated in delta 1 with each one's locator, and the suites the pin reaches, enumerated in delta 3 with each one's act.

## Existing sections updated

- `gate-sdk/SPEC.md` §The knob file — precedence item 2 (delta 1).
- `gate-sdk/SPEC.md` §Layout and configuration — the two-layer sentence and the caller-chosen-config paragraph (delta 1).
- `gate-sdk/SPEC.md` §lib/gate.sh — the pre-binary read's precedence (delta 1).
- `gate-sdk/SPEC.md` §git-hook — the launcher's read of its knob (delta 1).
- `gate-sdk/SPEC.md` §lib/test-hermetic.sh — the pair pinned, the export's place, the limit removed (delta 2).
- `gate-sdk/SPEC.md` §The knob file — the open-family paragraph's list of refused locator names (delta 1).
- `drift-kit/SPEC.md` — the open-family paragraph's list of refused locator names (delta 1).
- `gate-sdk/SPEC.md` §run-gate-tests and the hermeticity contract — the pin's statements (delta 2); §git-hook — the refusal list beside the launcher's read (delta 1).
- `native/src/knobs/mod.rs` (`load`, `cache_key`, `wire_in`, `static_names`, `admits_consumer_scalar` and its test), `native/src/emit/hook_launcher.rs`, `native/src/emit/front_end_parity.rs` — the crate's readers, rosters and their parity row (delta 1). The crate unit tests that scrub the tracked locator from their environment (`native/src/knobs/gate_sdk.rs`, `native/src/installer/init.rs`) scrub the overlay locator beside it.
- `gate-sdk/lib/gate.sh`, `gate-sdk/bin/run-gates.ps1` — the pre-binary readers (delta 1).
- `gate-sdk/lib/test-hermetic.sh` — the export and the header comment (delta 2).
- `gate-sdk/gate-tests/native-git-hooks.test.sh`, `gate-sdk/gate-tests/check-graph-cap.test.sh`, `gate-sdk/gate-tests/check-graph-refs.test.sh`, `gate-sdk/gate-tests/check-graph-theme.test.sh` — the un-pin added and the three removed (delta 3).
- `docs/gate-sdk/SPEC.md` — generated, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — a Knob changes bullet for the new locator, since an adopter may set it: new, default unset, the file a kit's local overlay is read from, set but missing exit 2, relocating one leaves the other (delta 1).

## Retired spellings

- None — no delta removes or renames a name; delta 3 deletes three statements and no spelling another surface carries.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **The probe re-run** — the planted-overlay run at the head, with the overlay planted in a copy of the gates directory, returns the control's count once the preamble pins the pair.
- [ ] **Every kit's fixture suite green on a seat carrying overlays** — the runner per kit is README.md §This repo, governed.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Entry moved** — `hermetic-preamble-overlay-read` moves to Done in the merge commit, which lands before the drain stage.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
