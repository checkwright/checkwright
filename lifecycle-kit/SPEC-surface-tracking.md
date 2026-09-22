# SPEC amendment: surface-tracking

A close-surface roster row does not say whether its path is tracked or gitignored. A session deciding whether a row's read may go to an isolated agent therefore re-derives the bit by hand. delegation-kit's isolation cost (3) makes that bit decisive: an isolated child sees only committed state, so a sweep over an untracked or gitignored surface is not delegable to it, and for a type held to isolation not delegable at all.

**The ruling: the roster prints the bit, as a sixth column, and the gate reads it from there.** The entry left three answers open: a field naming the tier, a field naming the trackedness bit, or no field, with the constraint living in the protocol template alone. The template already carries the constraint, and its reader re-derives the bit each time, so no field leaves the cost where it is. The field names **trackedness**, in three values, because the two readers ask two different questions. Assertion C of `check-close-surfaces` asks whether a path is gitignored, which is the capture tier. The delegating session asks whether a path is in the index, which is what an isolated child can see. A path can be neither, for example a declared file that does not exist yet, and a two-value field would answer one of the questions wrongly for it.

**The premise the drain corrected, and what it costs.** The emitter calls `git check-ignore` only in the workflow-directory walk that mints `(undeclared)` rows. The gate calls it again for each declared row under the workflow directory to decide assertion C (`native/src/gates/close_surfaces.rs`). So the fact is computed twice for one set of rows and printed for none. With the column the emitter computes it once per row, over the whole roster, and the gate reads the column. The two can then never disagree, which is the ground §check-close-surfaces already gives for calling the derivation in process.

**Measured at authoring (2026-09-22).** `--emit close-surfaces` prints twelve rows. `git check-ignore --stdin` over their file paths returns six, all under `.workflow/`, and each carries a `reclaim=` command. So taking assertion C's scope from the column, which reaches a gitignored declared path outside the workflow directory too, reds nothing on this tree.

## What changes

### (1) §The close-surfaces emit arm: the `<tracking>` column {design-bearing}

**Not yet applied.** "tab-separated `<path>	<mode>	<reclaim>	<owner>	<state>`" becomes "tab-separated `<path>	<mode>	<reclaim>	<owner>	<state>	<tracking>`". After the `<state>` paragraph, add:

> `<tracking>` is the row's file as git sees it under the computed base: `tracked` when the index holds it, `ignored` when `git check-ignore` matches it (the capture tier), and `untracked` otherwise. A `<file>#<section>` row reads its file's value. It is computed once per row by two batched git calls over the roster's file paths, and a `git` call that cannot decide is the arm's existing exit-2 cause. Two readers ask two questions of it. `check-close-surfaces` assertion C reads `ignored`. A session delegating a row's read reads anything but `tracked` as not delegable to an isolated agent, which sees only committed state. The column is appended, like `<state>`, so every position a reader indexes stays put.

### (2) §check-close-surfaces: assertion C reads the column {design-bearing}

**Not yet applied.** Assertion (C) becomes "**every declaration whose `<tracking>` reads `ignored` names a reclaim command**". In the in-process paragraph, "The gate splits the row's fifth field, `<state>`, off and reads it as nothing — a four-way split would fold it into the owner of every error line" becomes "The gate splits the row's fifth and sixth fields off, reading `<state>` as nothing and `<tracking>` for assertion C. A four-way split would fold both into the owner of every error line." Add: "Assertion C no longer calls `git check-ignore` itself, so the emitter and the gate cannot disagree about which rows are capture-tier."

### (3) The emit and gate modules carry the column {mechanical}

**Not yet applied.** `native/src/emit/close_surfaces.rs`: `derive` appends `<tracking>` to every row, from one `git ls-files` and one `git check-ignore --stdin` over the rows' file paths (the part before any `#`), relative to the computed base. A failed call is the arm's existing error return. `native/src/gates/close_surfaces.rs`: the row split takes six fields, and assertion C reads `ignored` instead of spawning `check-ignore` per row under the workflow directory. Unit tests: a tracked, an ignored and an untracked row each print their value, and a section row prints its file's. `lifecycle-kit/gate-tests/check-close-surfaces.test.sh`: its three cases still pass. The declared-without-reclaim case now fails on the `ignored` value, and a new case places a gitignored declared path outside the workflow directory with no reclaim, expecting assertion C to red.

### (4) The close template names the delegation read {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/close.md` step 4, after "so confirm the writer or file the gap.", add: "A row whose `<tracking>` is not `tracked` is read by you, never by an isolated agent, which sees only committed state (delegation-kit/templates/agent-execution.md, isolation cost 3)."

### (5) The pending release declaration names the column {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`: a `Behavior changes` bullet for `--emit close-surfaces` (a sixth tab-separated column, `<tracking>`, reading `tracked`, `ignored` or `untracked`; a reader splitting on five fields folds it into `<state>`), and a `Tightened gates` bullet for `check-close-surfaces` (assertion C now reaches a gitignored declared path outside the workflow directory).

## Producers and consumers

- **`<tracking>`** (deltas 1 and 3). Produced by the emit arm on every call, with no enabling config, wherever the roster derives. Consumed by `check-close-surfaces` assertion C in process (delta 2), and by the closing session at step 4, which reads the printed row (delta 4). Each of its three values has a reader: `ignored` for assertion C, and `tracked` against the other two for the delegation read.
- **Roster-holding readers of the row shape.** `check-close-surfaces`, which splits the row (delta 2). No other tracked reader parses the arm's output: `git grep -n "close-surfaces"` finds the close template's two steps, which read rows by eye, and the arm's own tests.
- **Point 5 (narrowing).** Assertion C's scope moves from "declared under the workflow directory and gitignored" to "gitignored". That widens the set it reads, so it can add reds and clear none. On this tree it adds none (measured above).
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -e "close-surfaces" -e "close_surfaces" -- lifecycle-kit native/src .workflow/release-declarations.md`, run 2026-09-22.

- `lifecycle-kit/SPEC.md` §The close-surfaces emit arm, the row grammar and a `<tracking>` paragraph (delta 1).
- `lifecycle-kit/SPEC.md` §check-close-surfaces, assertion C and the in-process paragraph (delta 2).
- `native/src/emit/close_surfaces.rs`, `native/src/gates/close_surfaces.rs` and `lifecycle-kit/gate-tests/check-close-surfaces.test.sh` (delta 3).
- `lifecycle-kit/templates/stages/close.md`, step 4 (delta 4).
- `.workflow/release-declarations.md` (delta 5).

## Retired spellings

- None — no delta retires a name; the five existing columns keep their names and positions.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the column.
- [ ] **Instruction surfaces: instruction only.** The close-template sentence carries the instruction; the grounds sit in §The close-surfaces emit arm.
- [ ] **Merged with no information lost.** Assertion C and the split sentence are re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `close-surface-row-trackedness-undeclared` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
