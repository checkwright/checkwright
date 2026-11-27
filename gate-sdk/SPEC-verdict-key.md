# SPEC amendment: verdict-key

Pairs `verdict-key-arm`. An outside engine asks `--verdict-lookup` for a stored verdict by its key, `treeId`, `tool.version` and `gateSet`, but only a registry run computes that key, so the engine re-derives it and misses every stored verdict once its derivation drifts from the binary's. `--verdict-key [--gates-dir D]` prints the key a bare registry run would store under, for the tree as it stands, and runs nothing. Its fields come from the code `--run`'s own key reading calls, so a key and a lookup cannot disagree.

**What the tree does today**, each bullet with the probe that settled it:

- `--run`'s key reading is `provenance_of(scope, &selected, &resolve_dirs)` in `native/src/runner.rs`. For a registry run, `selected` is the registry's members with no arguments, resolved through `GATE_SDK_GATES_DIR` or `--gates-dir`, `registry::list_path`, `exit::read_configured`, `walk::kit_roots_abs` and `registry::resolve_dirs`, all inline in `battery()` (`grep -n 'fn battery' -A110 native/src/runner.rs`; `grep -n 'fn provenance_of' -A6 native/src/runner.rs`).
- `native/src/verdict_store.rs` holds the key: a private `key(&Provenance)` takes `treeId` where the worktree is clean, `tool.version` and `gateSet`, each required, and digests them through `file_name` (`grep -n 'fn key\|fn file_name' -A3 native/src/verdict_store.rs`).
- `native/src/provenance.rs` folds a git read that fails into an absent field, as it folds an unborn `HEAD` and a modified worktree (`grep -n 'pub fn head\|pub fn worktree_clean' -A4 native/src/provenance.rs`).
- The front ends pass an arm they do not name through to the binary (`git grep -n 'verdict-lookup' -- gate-sdk/bin installer`, no hit).

## What changes

### (1) The `--verdict-key` arm {design-bearing} {user-facing: a new arm printing the store key of the tree as it stands, for a caller to hand `--verdict-lookup` (adopter direction 2026-11-27, lead-relayed; the spelling, the shared code and the exit mapping a lead decision of 2026-11-27 under the operator's delegation, the absent `--json` adopter-confirmed)}

**Not yet applied** — gate-sdk/SPEC.md §run-gates, a paragraph, its table and its grounds after the lookup's grounds, before **The provenance block**:

> **`--verdict-key [--gates-dir D]` prints the key a registry run would store under.** It is an `Arm::Run` row. It reads the tree from the working directory, as `--run` does, and runs no member. `--gates-dir` selects the registry as `--run`'s does. It prints one document on stdout, keys sorted and no insignificant whitespace, with no `--json` option: `contract`, `1.0.0` under this section's verdict-contract rule, and the three key fields under their provenance block spellings, `treeId`, `tool` holding `version`, and `gateSet`, valued as a bare registry run reads them before its first member. The fields are the operands `--verdict-lookup` takes, in its order `treeId`, `tool.version`, `gateSet`.
>
> | Case | Status | Document |
> | --- | --- | --- |
> | the three fields read | 0 | the key |
> | an operand other than one `--gates-dir <dir>` | 2 | the error document, `usage` |
> | no repository, an unborn `HEAD`, a modified worktree, or a binary with no release stamp | 2 | the error document, `refused` |
> | a malformed `GATE_SDK_GATES_DIR` or kit-dirs value, or a registry absent or naming no gates | 2 | the error document, `refused` |
> | a registry that cannot be read | 3 | the error document, `input` |
> | a refused repository | 3 | the error document, `tool` |
>
> The refusal on stderr names the first field absent, `treeId` before `tool.version` before `gateSet`, and its remedy: commit or clean the worktree, run a stamped build, or make each member's file readable. **The goldens** are `key.json` and `error.json`, the last a modified worktree's refusal, in `v1/` of the directory under `docs/contracts/` named for the arm, byte-compared by a unit test in the arm's module and published at the same path under `/contracts/`. `gate-tests/verdict-store.test.sh` runs the arm after a stored run on a stamped build and hands its three fields to `--verdict-lookup`, which answers the stored verdict.
>
> - **Why the run's own code.** The key is worth asking for only where it is the one the store files under. The arm calls the registry resolution `battery()` calls, then `provenance_of` at the registry scope, then the store's own key fields, so a change to any of them moves the arm with the run.
> - **Why the fields and not the file name.** The lookup takes the fields, and the digest naming the file is the store's own layout, which a caller never reads.
> - **Why a modified worktree or an unstamped binary is refused.** Neither has a key: a modified worktree has no tree object, and a binary with no stamp stores nothing (the honest limit above). The caller's fix is to commit or to run a stamped build, so the status is the caller's 2, and a key printed anyway would be one no run ever stores.
> - **Honest limit.** The arm judges nothing, so the key is the one a run *would* store under; a run that errors, or whose two readings differ, stores nothing, and the lookup answers `absent`. A git read that fails inside the provenance producer reads as the field's absence, so it is refused at 2 like the state it hides; a member file that cannot be read folds the same way, as `--run`'s reading does, into an absent `gateSet`, refused at 2 naming it. Through the front end, a call outside a repository is the front end's own refusal at 2, with no error document.

**Not yet applied** — gate-sdk/SPEC.md §The non-gate arm, the sentence "An arm minted for a machine caller alone, with no person reader, prints its document by default and takes no option: `--verdict-lookup` is the one such arm." becomes "An arm minted for a machine caller alone, with no person reader, prints its document by default and takes no format option, so no `--json`: `--verdict-lookup` and `--verdict-key` are such arms. An operand selecting what the arm reads, such as `--verdict-key`'s `--gates-dir`, is no format option." The paragraph after it, "The members are `--verify-gate-run` (evidence-kit/SPEC.md §verify-gate-run), `--run` and `--verdict-lookup` (both §run-gates).", becomes "… `--run`, `--verdict-lookup` and `--verdict-key` (all §run-gates)."

**Not yet applied** — gate-sdk/SPEC.md §Output contract, the grounds' "reads `--run`, `--verify-gate-run` and `--verdict-lookup` alone" becomes "reads `--run`, `--verify-gate-run`, `--verdict-lookup` and `--verdict-key` alone".

`native/src/runner.rs`: the registry resolution leaves `battery()` for one function both callers take, returning the gates dir's list, the resolve dirs and the bare selection, its refusals carrying their kinds; `battery()` narrows the selection after it as today. `native/src/verdict_store.rs`: `key` splits into the three fields, shared by the file name and the arm's document, and the arm, its contract constant and its error documents live beside `lookup`. The arm-table row in `native/src/emit/mod.rs` declares the knobs the shared resolution reads, and the usage text in `native/src/runner.rs` gains the arm beside `--verdict-lookup`.

### (2) The goldens, the tests and the declaration {mechanical}

- **Goldens** at `docs/contracts/verdict-key/v1/key.json` and `error.json`, the first over a fixed provenance, byte-compared by a unit test in `native/src/verdict_store.rs`.
- **Unit tests** in `native/src/verdict_store.rs`: the document over a provenance missing each field in turn refuses naming it, and the file name and the document read one triple.
- **The end-to-end case** in `gate-sdk/gate-tests/verdict-store.test.sh`, on the stamped build it already runs: a stored run's key through `--verdict-key` answers `pass` from `--verdict-lookup`; a modified worktree prints the `refused` document at 2; an unknown operand prints the `usage` document at 2.
- `.workflow/release-declarations.md`: *Behavior changes* gains the arm, its document, its statuses and the goldens' path.

## Producers and consumers

- **The key document.** Producer: the arm, from the shared resolution, `provenance_of` and the store's key fields. Consumer: an outside engine, which passes the three fields to `--verdict-lookup` before it decides to run the battery. Each field has that one reader.
- **The shared resolution.** Producer: the extracted function in `native/src/runner.rs`. Consumers: `battery()` and the arm.
- **The goldens.** Producer: the arm's serializer, frozen. Consumers: the unit test and an adopter's reader tests, through the published path.
- **Point 2, roster-holding readers.** The arm table, read by the dispatcher and its argument-grammar check; the usage text; §The non-gate arm's member sentences; `docs/contracts/`, published by the site build.

## Existing sections updated

Roster probe: `git grep -n 'verdict-lookup' -- ':!docs/llms-full.txt' ':!docs/gate-sdk/SPEC.md' ':!docs/posts' ':!TASK-QUEUE.md'`, and `grep -n 'fn battery\|fn provenance_of' native/src/runner.rs`, over the tracked tree.

- `gate-sdk/SPEC.md` — §run-gates' paragraph, table and grounds, §The non-gate arm's two sentences and §Output contract's grounds (delta 1).
- `native/src/runner.rs` — the extracted resolution and the usage text (delta 1).
- `native/src/verdict_store.rs` — the key fields and the arm (delta 1), the golden and unit tests (delta 2).
- `native/src/emit/mod.rs` — the arm-table row (delta 1).
- `docs/contracts/verdict-key/v1/` — the goldens (delta 2).
- `gate-sdk/gate-tests/verdict-store.test.sh` — the end-to-end case (delta 2).
- `.workflow/release-declarations.md` — the arm (delta 2).
- `docs/gate-sdk/SPEC.md` — generated mirror (all deltas).
- `docs/llms-full.txt` and `docs/requirements.md`'s install-size block — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; `--verdict-lookup` and the store keep theirs.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, its document and the shared resolution.
- [ ] **Instruction surfaces: instruction only** — the usage line names the arm and its effect, the grounds staying in the section.
- [ ] **Merged with no information lost** — each passage is re-phrased in place, not appended to.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Run once** — the arm on this tree, clean and modified, by hand with a stamped build, its key handed to `--verdict-lookup` after a stored run.
- [ ] **Release declared** — `.workflow/release-declarations.md` carries the arm.
- [ ] **Entry moved** — `verdict-key-arm` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed as a debt task.
