# SPEC amendment: config-variant

An adopter weighing a knob today edits the config seam, commits, and learns what it did from the next red. The fixture pairs prove each gate against fixed trees and say nothing about this tree under that knob. This amendment adds a shipped arm that answers the question before the commit: it runs the battery twice over a scratch copy of the tree, once as committed and once under a named variant of the config seam, and prints the gates whose verdict or output the variant moves.

The entry left three choices open. Each is settled here on the mechanism that decides it:

- **The variant is a directory of whole files, never an env file.** An indexed or keyed knob takes no environment override (§The knob file), so an env file could vary scalars alone, and most corpus and exemption knobs are indexed. A directory of knob files reuses the one grammar the seam already has, and it varies `gates.list` as well.
- **The scratch is a copy carrying history, never a worktree.** The crate refuses `git worktree add` (§upgrade-smoke, the shared-clone rule), and a gate reading `HEAD` or the log would read a one-commit scratch differently from the tree. `csmoke::tracked_history_scratch` already builds that copy.
- **The product is the diff.** The adopter's question is which gates the knob arms, disarms or reds. A full report would bury those rows under every unchanged gate.

## What changes

### (1) The `--run-config-variant` arm {design-bearing} {user-facing: config-variant-battery-harness' Deliverable, the operator ruling at consult that filed it: a shipped, bridged arm printing the per-gate verdict diff against baseline}

A new section in gate-sdk/SPEC.md §Per-component contracts, after §upgrade-smoke. **Not yet applied.**

> ### run-config-variant
>
> **`--run-config-variant <dir> [--keep]` runs the battery over a scratch copy of the tree twice, as committed and under the variant `<dir>` holds, and prints each gate the variant moves.** An adopter rehearses a knob change with it before committing the change, so the gates it arms, disarms or reds are read off one run rather than off the next commit.
>
> - **The variant.** `<dir>`, relative to the working directory, holds the files the variant replaces, by basename, in the gates directory: any `*-config.knobs` file, `gates.list`, or any other file the gates directory carries. Each is copied over its same-named file, or added where none exists. The directory is not tracked and need not be, so a scratch directory under `GATE_SDK_TMP_DIR` serves. A `<dir>` that is absent, empty, or holds anything but regular files is refused at exit 2, since a variant that replaces nothing has nothing to compare.
> - **The scratch.** The arm builds it with `csmoke::tracked_history_scratch` under `GATE_SDK_TMP_DIR`, absolutized: a shared clone carrying the tree's history, with the working-tree content of the tracked set laid over it and committed. An unstaged edit to a tracked file is therefore in both runs, and an untracked or ignored file, a `.local.knobs` overlay among them, is in neither. The running binary is placed at the path `GATE_SDK_NATIVE_BIN` resolves to inside the scratch, as `check-fence-run` places it (canon-kit/SPEC.md §check-fence-run). `--keep` retains the scratch and prints its path; otherwise it is removed on every exit path.
> - **The two runs.** The baseline is `--run` on the scratch as built. The arm then copies the variant's files in, commits them, and runs `--run` again. Each run is the running binary started as a child with the scratch as its working directory and `GATE_SDK_VERBOSE` set, so every gate prints its tail, and a child per run is forced: a process reads its knob layers once.
> - **The report.** One header line, `CONFIG-VARIANT: <dir> against the baseline, <n> gate(s) compared`, then one row per gate whose result differs, in registry order, then a count line. A gate's result is its tail and its output together. The row kinds are `red` (passed in the baseline, failed under the variant), `green` (the converse), `moved` (the same verdict, different output, which is how a knob arming or disarming a gate shows in its clean line), `added` and `removed` (registered under one registry only). A `red` or `moved` row prints the variant's output for that gate beneath it, indented. An unchanged gate prints nothing.
> - **The exit.** 0 when no gate is newly red, 1 when one is, 2 when the arm could not compare: a refused operand, a scratch it could not build, or a battery that refused at exit 2 in either run, whose output it prints. A variant knob file the binary refuses is not this case: each member then fails at its first knob read, and those rows read `red`.
> - **What it spawns.** `git`, the floor, and the running binary. It copies with the standard library and spawns no shell, so it ships to adopters on the floor alone (§The adopter constraints).
> - **Honest limits.** A scalar the caller exported outranks a variant's knob file in both runs (§The knob file), so a variant cannot move it, and an exported `<KIT>_KNOB_FILE` or `<KIT>_LOCAL_KNOB_FILE` redirects a kit's layer away from the gates directory the variant writes. The arm prints one `note:` line naming each such locator it inherited. A gate whose verdict reads an untracked or ignored file reads it in neither run, so its rows compare two scratch verdicts, neither of which need be the live tree's.
>
> Producers and consumers: the adopter, or the session walking `templates/adopt.md`, produces the variant directory and reads the report; the arm produces the scratch, the two runs and the report.

### (2) The arm's crate surface {mechanical}

- `native/src/emit/run_config_variant.rs`, a new module, holds the arm. Its row joins `ARMS` in `native/src/emit/mod.rs` as an `Arm::Run`, declaring `GATE_SDK_TMP_DIR`, `GATE_SDK_GATES_DIR` and `GATE_SDK_NATIVE_BIN`, under a `spec:` comment naming §run-config-variant and the three-way exit that forces `Arm::Run` (§The non-gate arm).
- The per-gate result reader splits a `GATE_SDK_VERBOSE` log into `(name, tail, output)` per `===== <name> =====` section. `native/src/emit/parse_gates_log.rs`, evidence-kit's `--emit parse-gates-log` arm (evidence-kit/SPEC.md §Layout and configuration), holds the one reader of the tail grammar, its private `fn tail`. That function becomes `pub(crate)`, the section split sits beside it, and the arm reads each verdict through `tail`, leaving one reader of the tail grammar. The arm's own behavior and `USAGE` do not move.
- `native/src/runner.rs`' `USAGE` gains the arm's line and paragraph.

§The non-gate arm's list of members the exit contract forces out of the `--emit-` family gains one bullet after `--install-hooks`. **Not yet applied.**

> - `--run-config-variant` (§run-config-variant) returns 0 no gate newly red, 1 a gate the variant reds, 2 no comparison made. Its caller reads the rows, and the 1 alone answers a caller asking whether the variant is safe to commit.

### (3) The named caller is the adopt walk {mechanical} {user-facing: config-variant-battery-harness' Deliverable — an adopter sees which gates a knob arms, disarms or reds before committing the knob}

Replacement for `gate-sdk/templates/adopt.md` step 3's last two sentences, from *Tell the user each change* to its end. **Not yet applied.**

> Tell the user each change and the paths that ground it. Rehearse it first: copy the `*-config.knobs` file in the gates directory named for the knob's kit into a scratch directory under `.tmp/`, make the change there (one `NAME = value` line for a scalar, one `NAME[] = element` line per element for an `indexed` knob), and run `--run-config-variant` on that directory. Its rows name each gate the change arms, disarms or reds. Then write the change into the gates directory's file. Never edit a vendored kit file.

`gate-sdk/README.md` gains one bullet after the `--run-consumer-smoke` bullet. **Not yet applied.**

> - the `--run-config-variant` arm — rehearses a config change: runs the battery over a scratch copy of your tree as committed and again with the knob files in a directory you name, and prints each gate the change reds, greens or moves.

### (4) The crate cases {mechanical}

The arm's `#[cfg(test)]` module, over a scratch repository it builds with a two-member `gates.list` naming binary-dispatched members whose verdict a knob moves:

- a variant setting a knob that reds one member prints that member's `red` row with its output and exits 1;
- a variant arming a member's clean line without changing its verdict prints a `moved` row and exits 0;
- a variant `gates.list` dropping a member prints a `removed` row, and one adding a member an `added` row;
- an absent, empty or subdirectory-bearing `<dir>` exits 2 before any scratch is built;
- an unstaged edit to a tracked file reaches both runs, and an untracked file reaches neither;
- the scratch is gone after a run without `--keep` and present, its path printed, after one with it;
- an exported `<KIT>_KNOB_FILE` prints its `note:` line.

### (5) The release declaration {mechanical}

`.workflow/release-declarations.md`, §Behavior changes, gains one bullet: **gate-sdk `--run-config-variant`** — a new arm that rehearses a config change: it runs the battery over a scratch copy of your tree as committed and again with the knob files in a directory you name, and prints each gate the change moves, exiting 1 when one goes red. Nothing to do; `templates/adopt.md` now rehearses each knob change with it.

## Producers and consumers

- **The variant directory.** Producer: the adopter or the adopt walk (delta 3). Consumer: the arm, by directory read at its start. Its one field, each file's basename, is read at the overlay as the gates-directory name it replaces.
- **The scratch.** Producer: `csmoke::tracked_history_scratch`, called by the arm (delta 1). Consumers: the two `--run` children, by working directory. `--keep`'s printed path is read by the caller.
- **The two logs.** Producer: each `--run` child under `GATE_SDK_VERBOSE`. Consumer: the arm's result reader (delta 2), by captured stdout. The tail is read for the verdict, the output for `moved`.
- **The report and the exit.** Producer: the arm. Consumers: the adopter reading the rows; a caller reading the 1 alone.
- **Roster-holding readers of the new arm name**, by `grep -rln 'run-consumer-smoke' native/src gate-sdk/README.md README.md docs/*.md gate-sdk/templates installer/README.md`, the sibling arm's readers (`smoke_entry_guard.rs` names the smoke's guard, not the arm roster): the `ARMS` table (delta 2), the arm-table test module census in `native/src/emit/mod.rs`, which requires the new file's test module (delta 4), `runner.rs`' `USAGE` (delta 2), and `gate-sdk/README.md` (delta 3). The front-door pages `check-front-door-verbs` reads carry no line for it, since `README.md`'s battery block lists only the arms validate runs (`check-battery-roster`), and this arm is no validate suite.
- **Knobs.** None is minted; the three declared are gate-sdk's existing rows.
- **The seam.** The arm, the overlay rule and the report are kit mechanism. A variant's content is the adopter's own config.

## Existing sections updated

- `gate-sdk/SPEC.md` §Per-component contracts — the new §run-config-variant (delta 1).
- `gate-sdk/SPEC.md` §The non-gate arm — the exit-contract bullet (delta 2).
- `native/src/emit/run_config_variant.rs`, `native/src/emit/mod.rs`, `native/src/runner.rs` — the module, its row and its usage (deltas 2 and 4).
- `native/src/emit/parse_gates_log.rs` — the section split beside `tail` (delta 2).
- `gate-sdk/templates/adopt.md` — step 3 (delta 3).
- `gate-sdk/README.md` — the arm bullet (delta 3).
- `.workflow/release-declarations.md` — the §Behavior changes bullet (delta 5).
- `docs/gate-sdk/SPEC.md` and `docs/gate-sdk/README.md` — generated mirrors, stale once a delta lands (all deltas).

## Retired spellings

- None — the amendment adds an arm and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — `config-variant-battery-harness` moves to Done with `--queue done` in the merging commit, ahead of the drain stage.
