# SPEC amendment: index-walk-root

**The index arms prune below each target, never above it.** `--emit md-index` and `--emit pub-index` share one corpus walk (`native/src/emit/mod.rs`, `corpus`). The walk itself prunes on the leaf basename, as §Index-first reading states, but a second filter after it runs `walk::path_pruned` over the whole walked path. With no path argument the target is the absolute repository toplevel, so a checkout under any directory named in `CONTEXT_KIT_PRUNE_DIRS` (`build`, `dist`, `target`, `worktrees` among the defaults) loses every file to that filter, and the arm prints its empty-case line at exit 0.

**Measured at authoring**, on a scratch repository at `.tmp/prune-probe/build/r` holding one `a.md`, with this tree's built binary:

- `--emit md-index` and `--emit pub-index`, run with no argument from the scratch root, print `No Markdown files found in <abs root>` and `No public items found in <abs root>`, each at exit 0.
- `--emit md-index .` from the same root lists `./a.md`, because a relative target puts no ancestor into the walked path.
- The same corpus at `.tmp/prune-probe/plain/r`, with no pruned leaf among its ancestors, lists `a.md` under the default target.

**Read at authoring, not run:**

- `corpus` also pushes an explicit **file** target unchanged and filters it the same way, so an absolute file path under a pruned ancestor is dropped too.
- `walk::find_link_entries_with_prune` never prunes its own starting directory and tests only the names below it. The trailing `path_pruned` filter is therefore the only place an ancestor reaches the prune set.
- Every other `walk::path_pruned` caller in `native/src` reads `git ls-files` output, which is repository-relative. The exception is `spec::comment_surface`, whose callers default its root to `.`. `gate_path_pruned` and `gate_find` in `gate-sdk/lib/gate.sh` have no production caller, only `gate-sdk/gate-tests/lib-gate.test.sh` and the `check-reads-couples` fixtures. The roster comes from `git grep -n path_pruned -- native/src` and `git grep -n 'gate_path_pruned\|gate_find ' -- '*.sh'`, with each caller's path source read.

## What changes

### (1) The corpus filter reads each path relative to the target that reached it

{design-bearing} `corpus` in `native/src/emit/mod.rs` applies `CONTEXT_KIT_PRUNE_DIRS` to the part of each walked path **below its target**. It strips the target prefix with the crate's existing relative-path helper (`walk::rel_under`, already used by `relative` beside it) before calling `walk::path_pruned`. A target is never pruned by its own name or an ancestor's, whether it is a directory or a file: a path the caller named is in the corpus. A leaf below a directory target is still pruned at any depth, as the walk's own basename rule already prunes it.

The `-not -path "*/<prune>/*"` spelling in `corpus`'s `spec:` comment is replaced by the rule this delta states. That spelling is the whole-path form this delta retires.

`walk::path_pruned` and `gate_path_pruned` keep their signatures and their behaviour. Every remaining caller feeds them a repository-relative path, and delta 3 states that precondition.

### (2) A crate test pins a root under a pruned ancestor

{mechanical} The `emit` module gains a unit test building a scratch tree whose root sits below a directory named by a default leaf (`<tmp>/build/r`). It asserts three things:

- `corpus` over the absolute root returns the root's `a.md`.
- `corpus` over that absolute file path returns the file.
- A `target/` directory **below** the root is still excluded.

It also covers the empty case: `corpus` over a root holding only pruned subtrees returns nothing.

The index-tests goldens are unchanged. Every golden invocation names one explicit file under `context-kit/index-tests/corpus/`, and no default leaf names a directory on that path in this tree.

### (3) §Index-first reading and §Layout and configuration state the boundary

{mechanical} context-kit/SPEC.md. **Not yet applied.**

In §Index-first reading, the *Traversal and order* bullet's clause "`CONTEXT_KIT_PRUNE_DIRS` matched on the leaf basename" becomes:

> `CONTEXT_KIT_PRUNE_DIRS` matched on the leaf basename **below each target**: a directory above the target, or the target itself, is never pruned, so a checkout under a `build/` or `dist/` directory indexes whole

In §Layout and configuration, the `CONTEXT_KIT_PRUNE_DIRS` bullet's "array of **leaf basenames** both index walkers exclude from their `find`" becomes "array of **leaf basenames** both index walkers exclude below each target".

In gate-sdk/SPEC.md §lib/gate.sh, the sentence "The match is on the **leaf basename**, so neither the parent nor its siblings are taken." becomes:

> The match is on the **leaf basename**, so neither the parent nor its siblings are taken. `gate_path_pruned` and `walk::path_pruned` match a leaf anywhere in the string they are handed, so a caller hands them a path relative to the walk's root, never an absolute one.

### (4) The site mirrors follow

{mechanical} `docs/context-kit/SPEC.md` and `docs/gate-sdk/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, in the commit landing delta 3.

## Producers and consumers

- **The narrowed filter.** Producer: `corpus`, at every `--emit md-index` and `--emit pub-index` run. The enabling configuration is the default: no argument, `CONTEXT_KIT_PRUNE_DIRS` at its default, and a checkout under a directory carrying one of its leaves. Consumers: the arm's stdout, read by a session; and the `--run-index-tests` runner, which passes absolute file targets out of `walk::kit_roots_abs`. The session-context hook's dirty-surface pre-run (§The session-context hook step 2) passes a relative `<component>/src/` target and is unaffected.
- **Readers whose verdict moves, and their red conditions.** The change widens the corpus for a target under a pruned ancestor and leaves every other corpus byte-identical, so causal-completeness point 5 has nothing to narrow.
  - `--run-index-tests` reds on any byte diff against a golden. Its targets carry no pruned ancestor in this tree, so its verdict here is unchanged. Under a pruned ancestor (a worktree under `.claude/worktrees/`, say) each check now indexes its file where the filter used to drop it.
  - The `index_tests` validate suite reads that runner's exit code, so it moves with it.
  - No gate reads the arms' output.
- **No new knob, arm, state or event.** `CONTEXT_KIT_PRUNE_DIRS` keeps its name, default and reader.

## Existing sections updated

Roster produced by `git grep -n 'path_pruned\|leaf basename\|-not -path' -- '*.md' native/src context-kit gate-sdk`, with each hit read.

- `native/src/emit/mod.rs` — `corpus` and its `spec:` comment (delta 1); the new unit test (delta 2).
- `context-kit/SPEC.md` — §Index-first reading, *Traversal and order*; §Layout and configuration, `CONTEXT_KIT_PRUNE_DIRS` (delta 3).
- `gate-sdk/SPEC.md` — §lib/gate.sh, the prune-set invariant (delta 3).
- `docs/context-kit/SPEC.md`, `docs/gate-sdk/SPEC.md` — the regenerated mirrors (delta 4).

## Retired spellings

- None — no delta retires a name; the whole-path match form is spelled only in the code comment delta 1 rewrites.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
