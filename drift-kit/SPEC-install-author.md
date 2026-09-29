# SPEC amendment: install-author

The install-observation record cannot tell an author's install from a non-author's. drift-kit/SPEC.md §The install-observation record exists for reds hit by someone who did not write the gate, yet its lines carry no seat: an operator-seat install, a clean-container rehearsal or a CI smoke filed through `--emit file-install` reads as external evidence, and §The install-evidence projection publishes it on `docs/install-evidence.md`.

This amendment gives the author seat a record of its own. `--emit file-install --author` files the same grammar into a second file, `DRIFT_KIT_AUTHOR_INSTALL_RECORD`, and the projection never reads that file.

One queue entry pairs it: [install-author-discriminator](../TASK-QUEUE.md#install-author-discriminator). [front-door-container-rehearsal](../TASK-QUEUE.md#front-door-container-rehearsal) files its run through the channel this lands, so it builds after this unit.

**The rulings.**

- **A second record, not a seat field.** A closed-set field on the `install` line would leave `red` and `checkin` lines to take their seat by joining on `<id>`, and a red whose `<id>` has no install line (the record admits one; §The install-evidence projection's first-useful-red block names the case) would have no seat at all. It would also change the arity of a shipped line grammar, and every record an adopter already holds would then need a reader for both arities. A separate file needs no join: a line is in the author record or it is not. Nor can it leak, since the projection has one input.
- **The author seat is the party that wrote or ships the gates under observation, and whoever installs on its behalf.** That covers the author's own install, a rehearsal the author runs on a machine they control, and a CI smoke. The filer declares the seat and nothing verifies it: an author install filed without `--author` reads as external evidence, which is the whole channel's state before this unit. That is the honest limit, and it is the same trust the closed-set fields already place in the observer.
- **Same grammar, same key rules, same refusals.** The author record is the observer's private working record. Its reader is the observer at the write-up transition, the reader `<profile>` and `<floor>` already have. A second grammar would give the observer two protocols to learn for one act. No author projection is added, since nothing publishes author evidence.
- **The seam.** The flag, the knob and its derived default are kit mechanism. The path a consumer overrides is consumer config. No consumer vocabulary enters the kit.

## What changes

### (1) The author record and the `--author` flag {design-bearing}

**Not yet applied.** drift-kit/SPEC.md §The install-observation record. The paragraph opening *The record is one file, `DRIFT_KIT_INSTALL_RECORD`* is rewritten:

*The record is two files, one per seat, each resolving under `DRIFT_KIT_METRIC_DIR` and inheriting that dir's retention and privacy contract in full (§Layout and configuration): append-only, surviving a scratch wipe, gitignored, never committed. `DRIFT_KIT_INSTALL_RECORD` holds what a non-author hit. `DRIFT_KIT_AUTHOR_INSTALL_RECORD` holds the **author seat's** observations: the party that wrote or ships the gates under observation, and whoever installs on its behalf, such as a rehearsal on a machine the author controls or a CI smoke. Both take the grammar, key rules and refusals below. The privacy half is the load-bearing one — the records key to people, so both files are the private half of this channel. §The install-evidence projection is the only thing about it that is ever published, and it reads the non-author record alone. The filer declares the seat and nothing verifies it: an author install filed without the flag reads as external evidence.*

The **Interface** paragraph's arm spelling becomes `--emit file-install [--author] [--] <kind> <field>...`. The three-line arity block is unchanged, since the flag takes no part in any kind's arity. After the paragraph's first sentence it gains: *`--author` as the first token files the line into `DRIFT_KIT_AUTHOR_INSTALL_RECORD` instead. Anywhere else before a `--` it is a flag in a positional slot and is refused like any other, so a misplaced flag never lands a line in the wrong seat. After a `--` it is a value, as every token there is.* The sentence *Its declared knob roster is the one row `DRIFT_KIT_INSTALL_RECORD`* becomes *Its declared knob roster is `DRIFT_KIT_INSTALL_RECORD` and `DRIFT_KIT_AUTHOR_INSTALL_RECORD`.* The **Named caller and transition** sentence becomes: *the caller is a human observer at an observed install or a check-in, filing with `--author` for the author seat's own; the transition where the non-author record is read is the projection's next emission, and the author record's is the observer's write-up.*

Implementation: `native/src/emit/file_install.rs`. `emit` strips a leading `--author` before `file_survey::positionals` runs, so the existing any-slot flag refusal refuses it in every other position. It then resolves the record through `anchored_capture` on the chosen knob, and `KNOBS` gains the new row. `USAGE` gains `[--author]` and one line naming the author record. The replace-in-place and append paths are unchanged and run against whichever file was chosen.

### (2) The projection reads the non-author record alone {mechanical}

**Not yet applied.** drift-kit/SPEC.md §The install-evidence projection, its second paragraph, first sentence: *It is a **pure function of the non-author record**, `DRIFT_KIT_INSTALL_RECORD`, and byte-stable over an unchanged one.* The author record is not an input, so every block's population is non-author by construction. Neither the emitter nor `check-install-evidence-fresh` changes: the gate's `# graph:` coupling through `knob:DRIFT_KIT_INSTALL_RECORD` already names the one input.

### (3) The knob row {mechanical}

**Not yet applied.** drift-kit/SPEC.md §Layout and configuration gains, after the `DRIFT_KIT_INSTALL_RECORD` row: *`DRIFT_KIT_AUTHOR_INSTALL_RECORD` — the author seat's install-observation record (§The install-observation record); default `.metric/author-install-observations.log`, derived as `${DRIFT_KIT_METRIC_DIR}/author-install-observations.log` so a set metric dir moves it (gitignored; the capture arm `mkdir -p`s the dirname). The capture arm is its one reader and writer.* `native/src/knobs/drift_kit.rs` gains the derived row beside `install_record`, under `DRIFT_KIT_METRIC_DIR`.

### (4) The README usage lines {mechanical}

**Not yet applied.** drift-kit/README.md: the `--emit file-install` usage line becomes `"$gates" --emit file-install [--author] [--] <kind> <field>...  # record one observed install, red or check-in (three kinds, three arities); --author files the author seat's own`. In the channel paragraph, after *under your metric dir*, add: *(`--author` files an install you or someone acting for you ran into a separate record the projection never reads, so author evidence never reaches a published figure)*.

### (5) The fixtures {mechanical}

**Not yet applied.** drift-kit/smoke/install.sh's install-observation block, under a throwaway `DRIFT_KIT_AUTHOR_INSTALL_RECORD` beside the existing throwaway record, asserts three things. First, `--author install …` lands its line in the author record and leaves the non-author record byte-unchanged. Second, `install-evidence` over the non-author record is byte-identical before and after that author filing. Third, `--author` after the kind exits 2 and writes neither record. drift-kit/SPEC.md §Testing's capture-arm sentence gains *the `--author` routing, the projection's blindness to it, and a misplaced flag refused* after *the per-kind key rules*. `file_install.rs` gains a unit test that a leading `--author` selects the author knob and a later one is refused.

## Producers and consumers

Probes: `git grep -n 'file-install\|INSTALL_RECORD\|install-observations'` over the tracked tree, less the queue, `docs/posts/` and the generated mirrors, and `ls .metric/`, which holds no install record on the authoring host.

- **`DRIFT_KIT_AUTHOR_INSTALL_RECORD`** (deltas 1 and 3). Producer: `--emit file-install --author`, run by the author seat's filer. Its first named producer run is front-door-container-rehearsal's. Consumer: the observer, reading the file at the write-up. No gate and no emitter reads it, and the projection's single input keeps it out of every published count. Its default is derived, so a consumer that sets `DRIFT_KIT_METRIC_DIR` moves it with the non-author record.
- **Every field of an author line** has the reader its non-author twin has at the write-up. The projection's readers (`<gate>`, `<verdict>` and the rest) never see the author record, and neither do the three fields the projection withholds.
- **The `--author` flag** (delta 1). Its one reader is `file_install::emit`. It is the arm's first flag, and the any-slot refusal still covers every other position before a `--`.
- **Roster-holding readers of the new knob name.** `native/src/knobs/drift_kit.rs` is the table the knob parity gates and `--emit knob-roster` read (delta 3), and the arm's `KNOBS` is held to what the module reads by the crate's arm-knob test (delta 1).

## Existing sections updated

Roster probe: the `git grep` above.

- `drift-kit/SPEC.md` — §The install-observation record (delta 1), §The install-evidence projection (delta 2), §Layout and configuration (delta 3), §Testing (delta 5).
- `native/src/emit/file_install.rs` (deltas 1 and 5).
- `native/src/knobs/drift_kit.rs` (delta 3).
- `drift-kit/README.md` (delta 4).
- `drift-kit/smoke/install.sh` (delta 5).
- `.workflow/surface-ceiling.txt` — the grown `drift-kit/SPEC.md` row re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (deltas 1, 2, 3 and 5).
- `docs/drift-kit/SPEC.md` and `docs/drift-kit/README.md` — the on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- None — the deltas add a flag, a knob and a record; no name, path or token is retired, and the arm's old spelling stays valid.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, checked by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Witnessed** — `--emit file-install --author install rehearsal-0 full linux 5` under throwaway records lands one line in the author record and none in the non-author one. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
