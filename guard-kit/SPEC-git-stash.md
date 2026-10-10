# SPEC amendment: git-stash

A session ran `git stash` and `git stash pop` on a checkout it shared, to build a pre-change binary for a before/after probe. A stash rewrites the index and the working tree every session in that checkout stands on. Rule `git_mutation_under_producer` holds `stash` only while a liveness record names a live PID, and every stash verb run through the member on this tree, with no such record, fell through. This amendment adds a rule that blocks a tree-rewriting stash on the main checkout and steers to a throwaway worktree.

## What changes

### (1) A new rule, `git_stash`

§The rule roster gains the item **`git stash` on the main checkout** (`git_stash`), placed immediately after rule `commit_only_paths` {design-bearing} {user-facing: the entry's Deliverable leaves the choice between a guard-kit block and a delegation-kit template bullet to this stage, and the block is the choice}.

The item, in the roster's grammar. **Not yet applied.**

> - **`git stash` on the main checkout** (`git_stash`) — Declares `raw` and `sq dq hd`. Shells `bash` and `powershell`. A `git stash` that rewrites the working tree or the index is **blocked** where the session's working tree is the main checkout. **The subject is the shared checkout**, rule `commit_only_paths`' ground one step wider: a stash takes every uncommitted edit in the tree, a concurrent session's among them, out from under it and puts them back at a moment that session did not choose.
>   - **What rewrites.** No verb, or a first word that is an option; `push` and `save`; `pop` and `apply`; `branch`, which checks a branch out. Rule `git_mutation_under_producer`'s git global-option walk finds the subcommand and the word after it.
>   - **What passes:** `list`, `show`, `create`, `store`, `drop` and `clear`, which leave the working tree and the index as they are, and every stash from a linked worktree, which has an index and files of its own (the test rule `worktree_confinement` applies).
>   - **The corrective names two exits.** To read or build another revision, `git worktree add <scratch-dir>/<name> <rev>`, removed with `git worktree remove` when done, the scratch dir being the first `GUARD_KIT_SCRATCH_DIRS` home. To set the session's own edits aside, commit them, naming their paths.
>   - **Block, not advise**, since an advise lets the stash run and the loss is silent: the wrong block costs one re-issue, and the wrong pass costs a concurrent session edits that vanish or return under it.
>   - **Declines** on the shared directions, on a verb the walk does not know, and on a global option that moves the repository (`-C`, `--git-dir`, `--work-tree`), since the tree it names is not the one the rule read.
>   - **Placed after rule `git_mutation_under_producer`**, so a stash under a live producer meets that block and its corrective first, and ahead of the auto-allow band and rule `allowlist_chain`, whose steer to run a decorated lead bare would cost a second round.
>   - **The honest limits.** The guard cannot see whether another session shares the checkout, so the block also meets a session alone in its tree; a consumer whose sessions never share one allows the command in its consumer rules, which run first (§Consumer rules). A stash behind an alias or a script is not read. `git checkout <rev> -- .` and `git reset --hard` rewrite the same tree and stay rule `git_mutation_under_producer`'s alone.

- **The block is chosen over the template bullet** on the Enforcement-first rule: a bullet in delegation-kit's agent-execution template reaches only a session that loaded it. The block's message carries the steer to every session at the call.
- **It is admissible as generic mechanism** on the ground rule `commit_only_paths`' item states for the same kind of subject, the shared index: its subject is git's own argv and the checkout boundary, and it reads no project vocabulary.
- **No knob is minted.** An off switch is the consumer-rules allow the item names, the existing door for a rule a consumer does not want.
- **The two-part PowerShell test holds**: the subject is git's argv and the checkout boundary, and the test needs no PowerShell vocabulary.

### (2) The rows and the suites

The decision tables and the worktree suite take the rule's firing and non-firing cases {mechanical}.

- **`guard-tests/cases.tsv`**, whose sandbox is a main checkout. Block: `git stash`, `git stash -u`, `git stash push -m x`, `git stash pop`, `git stash apply`, `git --no-pager stash`. Fall through: `git stash list`, `git stash show -p`, `git stash drop`.
- **The existing `fallthrough` row `git stash`** sits among rule `git_mutation_under_producer`'s decline rows and flips to `block`. It moves to this rule's rows, and that rule's block of comments keeps a mutating form no later rule blocks.
- **`guard-tests/powershell-cases.tsv`**: `git stash` blocks and `git stash list` falls through.
- **`gate-tests/worktree-confinement.test.sh`**, which owns the linked worktree: `git stash` from the worktree falls through.
- **`gate-tests/git-mutation-under-producer.test.sh`** asserts exit 2 for `git stash` and `git --no-pager stash` under a live record. Both stay 2, by the earlier rule, and the suite needs no edit unless it asserts the later rule's absence.
- **Every row a `git stash` word reaches is re-derived**, by `grep -n stash guard-kit/guard-tests/*.tsv guard-kit/gate-tests/*.sh`.

## Producers and consumers

- **The rule.** Producer: the engine's table walk on every `Bash` and `PowerShell` payload the wired member receives. Consumer: the session, through the block's stderr.
- **Rosters holding a rule name.** §The rule roster and the crate's rule table (`native/src/guard/rules/mod.rs`), held in lockstep by `check-guard-registration`: assertion B reds on the order and C on the declaration until both carry the item. Its assertion D resolves a citation against the roster, so this file names the rule name-first and uses no citation form for it until the item lands.
- **Readers of the corrective's steer.** `git worktree add` under the scratch dir fell through the member on this tree, so the steer is followable at one permission decision; no recommended-allowlist entry is added for it.
- **Widening.** The rule adds a block. The readers that red on it are the decision tables, in both directions (§Testing), and delta 2 names the one existing row.
- **Kit prose telling a reader to stash**, by `git grep -n stash -- ':!TASK-QUEUE.md' ':!*/SPEC.md'`: the dirty-tree refusals print *Commit or stash first* in the installer (`native/src/emit/pack_installer.rs`, `native/src/emit/installer_smoke/roster.rs`, `native/src/installer/init.rs` and `native/src/installer/uninstall.rs`) and the companion's install command. Both exits remain open to the person they address, and an agent following them meets the corrective's commit exit. The same probe reaches `scripts/guard-config.knobs`' breadth probe `Bash(git stash clear)`, which passes under the rule, `clear` leaving the tree as it is.

## Existing sections updated

- `guard-kit/SPEC.md` §The rule roster — the new item after the `commit_only_paths` item (delta 1).
- `guard-kit/SPEC.md` §Testing — the linked-worktree suite's case list gains the stash case, and the sandbox precondition saying every mutating-git row asserts rule `git_mutation_under_producer`'s decline arm is re-worded, since the new block rows are mutating-git rows that do not (delta 2).
- `native/src/guard/rules/mod.rs`, `native/src/guard/rules/reach.rs` — the table row and the rule's test (delta 1).
- `guard-kit/guard-tests/cases.tsv`, `guard-kit/guard-tests/powershell-cases.tsv`, `guard-kit/gate-tests/worktree-confinement.test.sh` — the rows and the case (delta 2).
- `docs/guard-kit/SPEC.md` — generated, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — a bullet for the new block, since an adopter's session meets it (delta 1).

## Retired spellings

- None — the amendment adds a rule and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
