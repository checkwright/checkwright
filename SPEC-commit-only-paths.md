# SPEC amendment: commit-only-paths

**The shared-index remedy this repo offers does not close the race it is offered for.** CLAUDE.md tells a session sharing the git index to "stage and commit in one motion", which sessions spell `git add <paths> && git commit -m …`. A pathless `git commit` commits the *whole* index, so a path a concurrent session staged between the two halves, or before them, rides the commit under the wrong message. One motion narrows the window and leaves it open.

**`git commit -o <paths>`, the only-paths form, closes it.** Measured 2026-09-30 in a throwaway repository, git 2.55.0:

- With a foreign path staged, `git commit -o A -m x` committed `A` alone, and the foreign path stayed staged.
- A path git does not yet track is refused (`error: pathspec 'N' did not match any file(s) known to git`), so a new file is `git add`ed first. `git add N && git commit -o N -m y` committed `N` alone and left the foreign path staged.
- A deletion staged by `git rm -q D` commits under `git commit -o D`.
- `git commit -o` with no path is fatal (`No paths with --include/--only does not make sense`).
- The pre-commit hook runs against a temporary index holding only the named paths: `GIT_INDEX_FILE` names a `next-index-*.lock` file, and `git diff --cached --name-only` lists the `-o` paths alone. This repo's gates read the staged set only through `git` subprocesses, which inherit `GIT_INDEX_FILE` (`git grep -n 'GIT_INDEX_FILE\|\.git/index' native/src` finds no direct index read outside a test), so the battery judges exactly the commit being made.

`git commit <paths>` with no `-o` has the same semantics, since `--only` is git's default when a path is given. The steer names `-o` anyway, so the command states its intent.

**What stays as it is.** delegation-kit's rule that committing agents serialize or take a worktree stays. The only-paths form closes the sweep of a foreign staged path. It does not remove contention for the index lock and `HEAD`, which a second committer meets as a refused commit rather than a corrupted one. rule `rm_tracked`'s `git rm -q` steer stays too, since its staged deletion commits under `-o` like any other path.

## What changes

### (1) CLAUDE.md names the only-paths form

The always-loaded shared-index sentence and the gap-capture bullet that points at it name `git commit -o <paths>` in place of "one motion" {mechanical}. **Not yet applied.**

- **§This repo is governed by its own kits**, the sentence "The git index is shared with any concurrent session: check `git status` for a foreign staged path before `git add`, or stage and commit in one motion." becomes: "The git index is shared with any concurrent session: commit with `git commit -o <paths>`, which takes only the named paths; `git add` a new file first."
- **§Housekeeping, the gap-capture bullet**, "staged and committed in one motion under the shared-index rule above" becomes "committed with the only-paths form under the shared-index rule above".

The line stays one line, under the always-loaded shape rule, and the replacement is no longer than the sentence it replaces, so `check-surface-ratchet`'s ceiling on the always-loaded surface does not move.

### (2) guard-kit gains rule `commit_only_paths`

A new generic rule blocks a command that stages with `git add` and then commits the whole index in the same call, steering to the only-paths form {design-bearing}. **Not yet applied.** Its roster item, inserted in guard-kit/SPEC.md §The rule roster directly after rule `background_no_record`:

> - **`git add` then a commit of the whole index** (`commit_only_paths`) — Declares `sq dq hd`. Shells `bash` and `powershell`. A command whose statements carry a `git add` and, after it, a `git commit` that commits the whole index is **blocked** with the steer to the only-paths form: `git commit -o <path>… -m '<msg>'`, naming every path the commit is for, with a new file `git add`ed first, in the same call or its own. A commit takes the whole index when it names no path, or when it carries `-a`/`--all` or `-i`/`--include`, each found by rule `git_mutation_under_producer`'s git global-option walk, which finds the subcommand and the words after it. A word after `--` is a path. A value-taking option's value is not a path: `-m`, `-F`, `-C`, `-c`, `-t`, `--author`, `--date`, `--fixup`, `--squash`, `--cleanup` and `--trailer`, in their attached and separate spellings, git's unambiguous long-option prefixes included. `--pathspec-from-file` counts as naming a path. **The subject is the shared index.** Every session in one checkout stages into one index, so a path a concurrent session staged rides a pathless commit under the wrong message. The only-paths form commits the named paths whatever else is staged, and `-a` additionally sweeps a concurrent session's unstaged edits to tracked files. Block, not advise: the sweep is silent, it lands in history, and the corrective is one spelling away. **Conservative by construction**: a statement the walk cannot align declines, as do an expansion and a backtick, and a commit naming a path without `-a` or `-i` declines, so the rule biases toward passing rather than a false steer. **Only the compound fires.** A lone pathless `git commit` is the ordinary commit of a single-session tree, and a rule refusing it would steer every consumer's every commit. CLAUDE.md's always-loaded sentence carries that case in this repository, and the rule stays silent on it. Placed after rules `git_mutation_under_producer` and `background_no_record`, so a mutation under a live producer meets that block first, and ahead of the auto-allow band and rule `allowlist_chain`, whose steer to run a decorated lead bare would otherwise split the add from the commit and widen the window rather than close it.

The crate half is a row in `native/src/guard/rules/mod.rs`'s table, at the roster's position, and a rule function beside rule `git_rewrite`'s in `native/src/guard/rules/reach.rs`. Its declared views and shells are whatever the item says, since `check-guard-registration`'s arms B and C hold the item and the row to one order and one declaration. If the walk it reuses reads a view the item does not name, build names that view in the item. The engine already runs a borrowed test under its owner's declaration, so no second view needs naming for it.

### (3) The decision table carries the rule's firing and non-firing cases

`guard-kit/guard-tests/cases.tsv` and `powershell-cases.tsv` gain rows for rule `commit_only_paths` {mechanical}. **Not yet applied.**

- **Firing, `block`:** `git add tracked.md && git commit -m x`; `git add -A; git commit -m x`; `git add tracked.md && git commit -am x`; `git add tracked.md && git commit -i tracked.md -m x`; and under PowerShell `git add tracked.md; git commit -m x`.
- **Non-firing, `fallthrough`:** `git add scratch.txt && git commit -o scratch.txt tracked.md -m x`; `git add scratch.txt && git commit scratch.txt -m x`; `git add tracked.md && git commit -m 'git commit -a' -- tracked.md`; a lone `git commit -m x`; and a lone `git add tracked.md`, whose existing row stays.

The rule converts a fall-through into a block wherever it fires, so every existing row carrying both a `git add` and a `git commit` has its expected column re-derived, under §Testing's non-monotone rule. `git grep -n 'git add' guard-kit/guard-tests/*.tsv` found no row carrying both at authoring.

### (4) delegation-kit's shared-index bullet names the commit form

delegation-kit/templates/agent-execution.md's **Serialize on shared files** bullet names the only-paths form as the commit a shared-checkout agent makes, and keeps serialization for what that form does not close {mechanical}. **Not yet applied.** Its sentence "So agents that each commit must be **serialized** *or* run under `isolation: worktree` (own index); …" is preceded by: "Commit with the only-paths form, `git commit -o <paths>`: it commits exactly the named paths whatever else is staged, which closes the sweep and not the contention for the index lock and `HEAD`." Replacement text for an instruction surface carries no grounds, so the grounds stay in this amendment and in rule `commit_only_paths`'s item.

## Producers and consumers

- **Rule `commit_only_paths`.** Producer: the shell guard's rule table, reached on every `Bash` or `PowerShell` payload the `Bash|PowerShell` matcher hands the member, in every consumer that wires `templates/settings-hooks.json` or the plugin's hooks, so its enabling config is the default wiring. Consumer: the agent, through the block's stderr. Roster-holding readers of the name: `check-guard-registration` (arms A to D hold the roster item, the crate table's order and declarations, and every citation), the decision tables, and the generated `docs/guard-kit/SPEC.md` mirror, held by `check-docs-mirror-fresh`.
- **The CLAUDE.md sentence.** Producer: the always-loaded file. Consumer: every session in this repository, and the stage-session and consult-session agent definitions, which point at the section rather than restating it. `git grep -n -i 'one motion\|shared-index\|shared index' -- ':!docs'` produced that pointer set at authoring: `.claude/agents/stage-session.md` and `.claude/agents/consult-session.md` cite the section by name and stay unedited, and lifecycle-kit/templates/lead.md cites the agent definition.
- **The template bullet.** Consumer: any dispatching session that loads `/agent-execution`. No gate reads its wording.

No field, state or event is added beyond the rule's verdict.

## Existing sections updated

- `CLAUDE.md` — §This repo is governed by its own kits and §Housekeeping (delta 1).
- `guard-kit/SPEC.md` — §The rule roster gains the item (delta 2). §The generic ruleset's two-part PowerShell test needs no edit: the rule's subject is git's argv, a class that test already names.
- `native/src/guard/rules/mod.rs` and `native/src/guard/rules/reach.rs` — the table row and the rule (delta 2).
- `guard-kit/guard-tests/cases.tsv` and `guard-kit/guard-tests/powershell-cases.tsv` — the rows (delta 3).
- `delegation-kit/templates/agent-execution.md` — the **Serialize on shared files** bullet (delta 4).
- `docs/guard-kit/SPEC.md` — the generated mirror, regenerated by the arm `check-docs-mirror-fresh` prints (delta 2). `docs/delegation-kit/` mirrors no template, so delta 4 reaches no mirror.

The roster came from `git grep -n -i 'one motion\|shared index\|shared-index\|foreign staged\|commit -o'` over the tracked tree, and `git grep -n 'git_rewrite'` for the readers a rule name reaches.

## Retired spellings

- None — the phrase "stage and commit in one motion" is re-phrased at its two sites, and the other "one motion" occurrences in the tree are unrelated uses a declared spelling would match.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for rule `commit_only_paths` and the two re-phrased instructions.
- [ ] **Instruction surfaces: instruction only** — the CLAUDE.md sentence and the template clause carry no grounds.
- [ ] **Merged with no information lost** — the roster item reads as one rule among its neighbours; this amendment's measurements that the item does not carry stay in its commit.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — the retired phrase is gone from both CLAUDE.md sites.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `one-motion-commit-race-remains-open` moves to Done in the landing commit, a stage before the drain stage.
