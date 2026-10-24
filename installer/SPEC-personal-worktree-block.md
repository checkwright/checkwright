# SPEC amendment: personal-worktree-block

Pairs `personal-worktree-shared-block`. Every linked worktree of a clone reads one `info/exclude`, and `init --personal` recomputes its one block whole from the running worktree's roster. So one worktree's install, re-run or `uninstall` rewrites or removes the lines another worktree's install needs. The fix is one block per worktree, on the operator's direction of 2026-10-24 (lead-relayed, not a ruling): keyed by the worktree's admin name; each run rewrites or removes only its own; blocks whose worktree is gone are pruned; a legacy unkeyed block is adopted as the main worktree's.

**What the tree does today**, probed at spec on git 2.55:

- **The exclude file is the common directory's.** From a linked worktree, `git rev-parse --git-path info/exclude` names `<common-dir>/info/exclude`. So does it from the main worktree.
- **Its lines reach every worktree.** Each block line is root-anchored, so a line one worktree wrote ignores the same path in every other.
- **The writers take one marker pair.** `native/src/installer/personal.rs` writes through `marker::install_block` between `PERSONAL_BEGIN` and `PERSONAL_END` (`native/src/installer/mod.rs`), removes through `marker::uninstall_block`, and reads the mode as any line equal to `PERSONAL_BEGIN` (`is_personal`). The marker module matches each marker by whole-line equality, so a pair carrying a key is a block of its own to it, with no change to `native/src/marker.rs`.

## What changes

### (1) The worktree's key and its marker pair {design-bearing} {user-facing: operator direction 2026-10-24, lead-relayed, "one block per worktree keyed by the worktree's admin name; a legacy unkeyed block is adopted as the main worktree's"}

`personal.rs` gains a **worktree key**. **Not yet applied.** It is read from `git rev-parse --absolute-git-dir` and `git rev-parse --git-common-dir`, absolutized against the root:

- **The main worktree has no key.** Its git directory is the common directory (`hook_launcher::same_dir`), and its markers are `PERSONAL_BEGIN` and `PERSONAL_END` unchanged. So a block any earlier release wrote, from whichever worktree, is the main worktree's, and nothing migrates.
- **A linked worktree's key** is its git directory's path relative to the common directory, `worktrees/<name>`, `<name>` being the admin name git chose. Its markers are `PERSONAL_BEGIN` and `PERSONAL_END` each followed by one space and the key: `# checkwright:personal:begin worktrees/<name>`.

`write`, `remove` and `init`'s `--dry-run` block print take the running worktree's pair. Every other block is left byte-identical.

### (2) The mode reads any block {mechanical}

`is_personal` reads a line equal to `PERSONAL_BEGIN`, or beginning with `PERSONAL_BEGIN` and a space, as the mode. **Not yet applied.** So, as today, every worktree of a clone holding a personal install reads its own install as personal, and a bare `init` in a worktree without a block of its own writes one.

### (3) Pruning a removed worktree's block {design-bearing} {user-facing: as delta 1, "prune blocks whose worktree is gone"}

After `write` or `remove` acts on its own block, it removes every keyed block whose key names no directory under the common directory. Git keeps a worktree's admin directory until `git worktree prune` or `git worktree remove` deletes it, so a block lives exactly as long as git's own record of the worktree. **Not yet applied.** Each removal goes through `marker::uninstall_block` with that key's pair. Each prints one line on stdout: `personal: dropped the exclude block of <key>, a worktree git no longer records`. A keyed block whose end marker is missing is left as it is and named in a second line, since the bounds cannot be read: `personal: left the exclude block of <key> in <exclude path>, its end marker missing`. The unkeyed block is never pruned, since the main worktree always exists.

### (4) The SPEC sections {design-bearing}

Each **Not yet applied.**

- installer/SPEC.md §The personal install, first paragraph: "a marker-bounded block in the clone's `info/exclude`, the file `git rev-parse --git-path info/exclude` names, lists each one" becomes "a marker-bounded block of this worktree's own in the clone's `info/exclude`, the file `git rev-parse --git-path info/exclude` names, lists each one". After "recomputed whole on every run." it gains: "Every worktree of the clone reads that one file, so each writes only its own block, keyed by the worktree's admin name and unkeyed for the main worktree, and drops the block of a worktree git no longer records."
- The same section, **Honest limits**: "The block belongs to the clone, so every linked worktree of the clone reads its install as personal." becomes "Git reads every worktree's block in every worktree, so one worktree's install hides the same paths in the others, and any block makes each worktree's install personal. A block an earlier release wrote from a linked worktree reads as the main worktree's, and goes when the main worktree's `uninstall` runs or by hand."
- §uninstall, the paragraph opening **The placed hooks are removed**: "A personal install's exclude block goes last" becomes "A personal install's exclude block, this worktree's own, goes last".

### (5) The unit tests {mechanical}

In `personal.rs`'s `tests`, over a scratch repository with one linked worktree added by `git worktree add`. **Not yet applied.**

- **Two blocks, each worktree's own.** A write from each worktree over different rosters leaves two blocks, and each roster's paths read ignored from both worktrees through `git check-ignore`.
- **A removal takes one block.** `remove` from the linked worktree leaves the main worktree's block byte-identical and its paths ignored.
- **A removed worktree's block is pruned.** After `git worktree remove`, a write from the main worktree drops the linked worktree's block and prints the dropped line.
- **An unkeyed block is the main worktree's.** One written with the bare pair is replaced, never duplicated, by a write from the main worktree.
- **The key itself.** The key is `worktrees/<name>` in the linked worktree and none in the main one.

## Producers and consumers

- **The worktree key** (delta 1). Producer: git's two directory answers, read on each `write`, `remove` and dry-run print. Consumers: the marker pair those three pass to the marker module, and delta 3's pruning test, which reads each keyed begin line's key back as a path under the common directory.
- **The keyed marker pair** (delta 1). Producer: `write`. Consumers:
  - git's ignore rules, which read every line between any markers alike;
  - `is_personal` (delta 2), and through it `init`'s mode, `uninstall`'s waiver and `prove`'s lockless branch (the personal proof, once `personal-install-prove` lands);
  - `remove` and delta 3's pruning;
  - the consumer smoke's personal arm. It counts lines equal to the bare begin marker and finds one in its single-worktree consumer, so it is unchanged (`native/src/emit/installer_smoke/personal.rs`, `MARK`).
- **The prune** (delta 3). Producer: `write` and `remove`, after their own block. Its red condition is a keyed block whose key names no directory. Consumers: the adopter's exclude file and the stdout line.
- **Sibling units in this iteration:**
  - **`personal-write-before-refusal`**, which moves a begin-without-end check ahead of `init`'s first write. Where it lands first, its check reads the running worktree's pair once this delta lands, and this delta changes the pair the check is given. Where this delta lands first, that unit's check is written against the running worktree's pair from the start. Either way, the check never refuses over another worktree's block.
  - **`marker-uninstall-newline`** fixes `marker::uninstall_block`, which delta 3's pruning calls. So its fix reaches every pruned block whichever lands first, with no act of this unit's changing.
  - **`personal-collapse-hides-files`** changes `body`, the lines inside a block, and not which block they go in. No act here depends on it.

## Existing sections updated

- `native/src/installer/personal.rs` (the key, `write`, `remove`, `is_personal`, the prune and the tests) and `native/src/installer/init.rs`'s `--dry-run` block print (deltas 1, 2, 3 and 5).
- installer/SPEC.md §The personal install and §uninstall (delta 4).
- `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 4).

The marker's readers were found by `grep -rn "PERSONAL_BEGIN\|PERSONAL_END\|is_personal\|personal::" native/src --include=*.rs`, and the smoke's by `grep -n MARK native/src/emit/installer_smoke/personal.rs`.

## Retired spellings

- None — no delta retires a name. The bare marker pair stays the main worktree's, and the keyed pair is new.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and the `installer_smoke` suite green. The Done move lands before the drain stage, on that oracle.
