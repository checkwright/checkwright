# SPEC amendment: close-surface-carry

**The packer carries each kit's `close-surface:` declarations into the payload, and the close-surface derivation reads them in a vendored kit root.** Today the declarations live only in each kit's SPEC.md, which the payload withholds (`GATE_SDK_PAYLOAD_WITHHOLD`, default `SPEC.md smoke`). So an adopter running lifecycle-kit beside a capture-logging kit, with the logs gitignored as guard-kit's and drift-kit's READMEs instruct for theirs, meets `check-close-surfaces` assertion A red on logs it has no declaration for. The gate's help sends it to "the SPEC section that already owns it", which it does not hold. It pairs `kit-log-declaration-transport`.

**Measured at authoring.** The capture-tier declarations, every `close-surface:` line carrying `reclaim=`, sit in three kit SPECs: guard-kit's `.workflow/wakeup-attempts.log` and `.workflow/prompt-friction.log`, delegation-kit's `.workflow/subagent-stop-liveness.log` and `.workflow/wait-primitive-evidence.txt`, and drift-kit's `.workflow/knowledge-friction.log` (`git grep -n "^ *close-surface:" -- '*/SPEC.md'`). The tracked rows the kits also declare, lifecycle-kit's gap inbox and queue-kit's Deferred and Lessons sections, never red assertion A, but they are missing from an adopter's roster for the same reason. The derivation reads each resolved kit root's `LIFECYCLE_KIT_ROSTER_BASENAME` file and every `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS` match (`native/src/emit/close_surfaces.rs`, `derive`), so a kit root without its SPEC.md contributes nothing.

**The channel is a pack-time carry, not a tracked per-kit file.** The queue entry names both. A tracked declaration file in each kit root would either move every declaration out of the section that owns its surface, taking the two `forced=` rows off the manifest surfaces where `check-spec-pointer` resolves their citations, or keep a second copy needing a generator and a freshness gate. The carry is derived at pack time from the one source the tree keeps, so nothing tracked is copied and nothing drifts. The queue entry offers both forms, so the choice is this amendment's, authored inside the entry's envelope (lead decision at align, not an operator direction).

**The carry is not the extract gate-sdk/SPEC.md §Consumer payload refuses.** That refusal is of a generated extract carrying the sections shipped pointers cite, on three grounds, and none reaches the carry:

- *It buys offline reading for a reader the publication already serves.* The carry's reader is the close-surface derivation in the vendored tree, a machine reader no publication serves.
- *It costs a standing projection and a freshness gate.* The carry is written per pack into the assembly scratch and tracked nowhere, so there is no standing copy to keep fresh.
- *It leaves the publisher's prose in the payload.* The carry holds `close-surface:` directive lines alone, a path, a mode and a reclaim command, and never a sentence of the SPEC around them.

**The carried `forced=` rows keep the manifest-surface rule** (lifecycle-kit/SPEC.md §The close-surface roster). That rule binds where a declaration is authored, because the manifest surface is what resolves its citation. A carried row is no authored declaration. It is a byte copy of one authored on a manifest surface, whose citation the publishing tree's `check-spec-pointer` resolved at the stamped commit. So the carry adds no unresolved citation, and the rule's honest limit, a `forced=` declaration authored off the manifest set, stays as it stands.

## What changes

### (1) The packer carries each kit's declarations {design-bearing} {user-facing: authored inside the entry's envelope, whose deliverable offers a shipped per-kit file or the payload carrying the declaration lines; this amendment chooses the second (lead decision at align, not an operator direction)}

In the kit loop, after a kit's tracked set is extracted, the packer reads each withheld member that is a file at the stamped commit, `git show <commit>:<root>/<member>`, and collects its `close-surface:` declaration lines with the derivation's own line reader (`close_surfaces::declaration_lines`, fenced lines skipped, made `pub(crate)` for this second caller). Where it collects at least one, it writes them, in source order after a one-line `#` comment naming lifecycle-kit/SPEC.md §The close-surface roster, to `{asm}/payload/{leaf}/close-surfaces.txt`. Where it collects none, it writes nothing. The basename is one crate constant beside the derivation, read by both the packer and the derivation, since the release that packs the file is the release whose binary reads it. It is not a knob, since a publisher's value and an adopter's default could then differ. A kit root that already tracks a file of that name is a refusal at exit 2, since the carry would overwrite a shipped file. An explicitly empty withheld set ships each SPEC.md whole and carries nothing, since the derivation then reads the SPEC itself.

This is the packer's fourth write into the assembly. Like the README rewrite and the license placement, it lands in the scratch the pack step tears down, so it reaches no tracked file. **Not yet applied.**

### (2) The derivation reads a kit root's carried declarations {design-bearing} {user-facing: the entry's deliverable, a declaration channel that reaches a vendored tree, in the form delta 1 chooses inside the entry's envelope (lead decision at align, not an operator direction)}

For each resolved kit root, `close_surfaces::derive` adds `<root>/close-surfaces.txt` as a declaration surface when it is a file, beside `<root>/<LIFECYCLE_KIT_ROSTER_BASENAME>`. A row's owner column names the surface it was read from, so a vendored tree's rows name the carried file. In this repository no kit root holds the file, since the packer writes it only into the assembly, so no declaration is read twice. A tree vendoring a kit with its SPEC.md, as §Vendoring without the installer allows, reads the SPEC and carries no file. **Not yet applied.**

### (3) `check-close-surfaces`' help names the vendored remedy {mechanical} {user-facing: a calibration inside the entry's envelope, authored (lead decision at align, not an operator direction): the help no longer sends an adopter to a file it does not hold}

The help line's sentence *in the SPEC section that already owns it — never a central list* becomes *in the section that owns the surface — never a central list; for an undeclared kit-owned log in a vendored tree, run `update`*. **Not yet applied.**

### (4) The consumer smoke proves the carry reaches the gate {design-bearing}

A new arm, `close-surface arm (the kits' capture logs declared in a vendored tree)`, in a fresh consumer at the payload-derived profile:

1. It reads the **expected** set from the tree under test, not from the payload: every `close-surface:` line carrying `reclaim=` in the canonical spec of each kit the consumer's manifest `kits` names. At least one must be found, so the arm cannot pass by vacuity.
2. It creates each such path in the consumer, adds each to the consumer's `.gitignore`, as guard-kit's and drift-kit's READMEs instruct for their logs, and commits.
3. It runs `check-close-surfaces` by name through the consumer's gate binary, since no profile registers it, and asserts the clean line.
4. As a control, it creates one more gitignored log under the workflow directory that nothing declares, asserts the gate red with that path `(undeclared)`, and removes it.

A payload that dropped the carry reds step 3 on every expected log. **A delta whose act depends on a sibling unit's landing:** where the `compiled-consumer-smoke-driver` entry's driver has landed in an earlier batch, the arm is written in the compiled driver, in its arm order before the demo arm; where it has not, the arm is written in `installer/consumer-smoke/run-smoke.sh` at the same position, with its header in the parsed grammar, and the driver port carries it. Either way `.workflow/validate-baseline.txt` gains the `installer_smoke close-surface-arm pass` row in the commit that lands the arm. **Not yet applied.**

### (5) The owning sections state the carry {design-bearing}

**Not yet applied.**

- **lifecycle-kit/SPEC.md §The close-surface roster**, the paragraph opening **Declaration lives with the owner**, gains: *A vendored kit root holds no SPEC.md, so the payload carries each kit's declarations into its root as `close-surfaces.txt`, generated at pack time from the SPEC and read by the derivation beside it (gate-sdk/SPEC.md §Consumer payload). The SPEC stays the declarations' one home.*
- **lifecycle-kit/SPEC.md §The close-surfaces emit arm**, the declaration-surfaces sentence becomes: *The declaration surfaces are each resolved kit root's `LIFECYCLE_KIT_ROSTER_BASENAME` file and its carried `close-surfaces.txt`, each where present, plus every `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS` match, resolved consumer-first with kit shadowing like every kit registry; duplicate surfaces collapse.*
- **lifecycle-kit/SPEC.md §The close-surface roster**, the paragraph opening *A `forced=` declaration belongs on a **manifest surface***, gains before its honest limit: *A carried `close-surfaces.txt` row is no authored declaration: it copies one authored on a manifest surface, whose citation the publishing tree resolved, so carrying it adds no unresolved citation.*
- **gate-sdk/SPEC.md §Consumer payload**, after the license paragraph: *Each packed kit also carries its withheld spec's `close-surface:` declarations, as `close-surfaces.txt` in its root, because the close-surface derivation must find a kit-owned capture log declared in a tree the SPEC never reaches (lifecycle-kit/SPEC.md §The close-surface roster). They are generated from the stamped commit at pack time, so the tree keeps one copy, and a root with none carries no file.*
- **gate-sdk/SPEC.md §Consumer payload**, the paragraph opening **Neither withheld member is load-bearing for a consumer.**, gains after its first sentence: *A SPEC's `close-surface:` declarations are the one exception, and the packer carries them out of it (above), so withholding the SPEC still withholds nothing a consumer's gate reads.*
- **gate-sdk/SPEC.md §Consumer payload**, the **Refused:** paragraph on a generated extract gains: *The declaration carry is not that extract: its reader is a gate no publication serves, it is written per pack and tracked nowhere, and it holds directive lines, never the SPEC's prose.*
- **installer/SPEC.md §The packer**, *exactly three places* becomes *exactly four places*, the list gains *inside the kit loop, each kit's carried declarations at `{asm}/payload/{leaf}/close-surfaces.txt`, read from its withheld members at the stamped commit (gate-sdk/SPEC.md §Consumer payload)*, and the recipes paragraph's *the write set stays three* becomes *the write set stays four*.
- **installer/SPEC.md §The consumer smoke** gains delta 4's arm, beside the arms it orders against.

### (6) Tests cover both halves {mechanical}

**Not yet applied.**

- `native/src/emit/pack_installer.rs`'s unit tests: a withheld SPEC carrying a fenced and an unfenced declaration carries the unfenced one alone; a SPEC carrying none writes no file; a tracked `close-surfaces.txt` in a kit root refuses.
- `native/src/emit/close_surfaces.rs`'s unit tests: a kit root holding `close-surfaces.txt` and no SPEC.md contributes its rows with the carried file as owner.
- `lifecycle-kit/gate-tests/check-close-surfaces.test.sh` gains a sandbox case: a vendored-shaped kit root with its carried file and a gitignored log reads clean, and the same root without the file reads the log `(undeclared)`.

## Producers and consumers

- **`close-surfaces.txt` in a packed kit root.** Producer: `--pack-installer`, on every pack, from the publishing path and the consumer smoke alike, with no enabling config: the default `GATE_SDK_PAYLOAD_WITHHOLD` withholds SPEC.md. Consumers: `init`, which vendors it as a payload file and records it in the manifest, so `diff`, `update` and `uninstall` treat it as any other; and the close-surface derivation in the vendored tree, reached by `check-close-surfaces` in process and by close's `--emit close-surfaces`. Each line's fields are the declaration's own, read by the derivation as a SPEC's are.
- **The arm's verdict.** Producer: the consumer smoke. Consumers: the validate suite's parsed roster and its baseline row, and each CI leg's status.
- **Roster-holding readers.** The installer smoke's withholding arm asserts no vendored root carries a SPEC.md or a `smoke/`, which the carried file is neither. The manifest's file-by-file agreement covers it as a written file. `check-packed-links` reads packed READMEs only. The payload's kit-set derivation walks directories, not files.
- **Red conditions.** The derivation's corpus widens, never narrows. Assertion A can only clear on a newly declared row, and assertion B and the mode, reclaim and rotation checks C and D apply to the carried rows as to the SPEC's, which already pass them in this repository's own battery. The packer gains one refusal, a tracked file at the carried name.

## Existing sections updated

Probe for the reader roster: `git grep -n "close_surfaces::\|declaration surfaces\|exactly three places\|write set stays"` over the tracked tree less `docs/posts`.

- `lifecycle-kit/SPEC.md` §The close-surface roster and §The close-surfaces emit arm (deltas 2 and 5).
- `gate-sdk/SPEC.md` §Consumer payload (deltas 1 and 5).
- `installer/SPEC.md` §The packer and §The consumer smoke (deltas 1, 4 and 5).
- `native/src/emit/pack_installer.rs` — the carry and its tests (deltas 1 and 6).
- `native/src/emit/close_surfaces.rs` — the carried surface, the shared constant and its tests (deltas 1, 2 and 6).
- `native/src/gates/close_surfaces.rs` — the help line (delta 3).
- `lifecycle-kit/gate-tests/check-close-surfaces.test.sh` — the sandbox case (delta 6).
- The consumer smoke driver, `installer/consumer-smoke/run-smoke.sh` or its compiled successor, per delta 4's two acts (delta 4).
- `.workflow/validate-baseline.txt` — the arm's row (delta 4).
- `.workflow/release-declarations.md` — the carried file a re-vendored kit gains, and the cleared red (deltas 1, 2 and 3).
- `docs/lifecycle-kit/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md` — the generated mirrors (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the help line names the remedy and carries no grounds.
- [ ] **Merged with no information lost** — each section reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **The entry is done** — `kit-log-declaration-transport` moves to Done when the consumer smoke's arm is green on the CI legs that run it, by build's remote-oracle rule, before the drain stage; it rides the compiled driver's mid-iteration push where that has landed, else the closing push.
