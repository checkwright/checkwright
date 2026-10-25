# SPEC amendment: init-identity

Pairs `init-identity-precondition`. `init`, and `update` through it, makes one commit, and so does `uninstall` (§init, §uninstall). With no git identity configured, git refuses that commit only after every file is written and staged. This amendment adds the identity as a precondition, refused before any write, under the guard the clean-worktree precondition already has.

**What the tree does today**, probed at spec with git 2.55 and a scratch pack of this tree (the packer's output, with the built binary placed as its artifact), in a repository with `HOME` at an empty directory and `GIT_CONFIG_NOSYSTEM=1`:

- **The failure arrives after the install.** `--init --profile starter` exits 1 with 592 files staged, the manifest written and no commit made. Its stderr is git's `Author identity unknown` account, then the plain line `the commit failed — the files are staged; commit them yourself to finish the install`, then the follow-up block. The identity is never named by the installer, only by git.
- **`git var` reads the identity as `git commit` does, and needs both halves.** With no identity, `git var GIT_AUTHOR_IDENT` and `git var GIT_COMMITTER_IDENT` each exit 128, as the commit does. With `GIT_COMMITTER_NAME` and `GIT_COMMITTER_EMAIL` alone set, the committer read exits 0 while the author read and the commit exit 128. With the author pair alone, the committer read and the commit exit 128. A repository-local empty `user.name` makes both reads and the commit exit 128 under a host whose global identity is set, and a local empty `user.email` lets both reads and the commit pass. So reading the committer alone, as the entry sketched, passes a seat the commit refuses.

**Inferred, cannot run here:** on a host whose name carries a domain, git derives an email from it and the commit passes with a warning, and `git var` passes there too, since both read the identity the same strict way. This host's name carries none, so every derivation probed refused.

## What changes

### (1) The identity precondition {design-bearing} {user-facing: lead decision 2026-10-25 on the intent oracle's ruling, lead-relayed (not an operator direction): settled by the entry's deliverable, refused under the clean-worktree guard, so an identity-less no-op run refuses too}

`init` and `uninstall` each refuse at exit 1, before any write, where `git -C <root> var GIT_AUTHOR_IDENT` or `git -C <root> var GIT_COMMITTER_IDENT` exits non-zero or cannot be spawned. The check sits under the clean-worktree precondition's guard, beside it: it runs where the verb will commit, and is waived where it will not, by `--no-commit`, by a personal install and by `--dry-run`. `update` meets it by becoming `init`.

The refusal first prints git's own account of the failed read, as a failed commit does, then `checkwright <verb>: git has no identity to make the commit with`. Its `help:` names the remedy and the valve: set `user.name` and `user.email` with `git config --global` (or without `--global` for this repository alone), or pass `--no-commit` to stage the vendoring, or the removal, and commit it yourself. The status is 1, the clean-worktree refusal's and the failed commit's it replaces. One helper in `native/src/installer/mod.rs` serves both verbs, each passing its own help clause, as both already pass their own clean-worktree help. **Not yet applied.**

**Not a `doctor` finding:** `doctor`'s status is the toolchain contract, run before every install and on its own, and the identity matters only where a commit is made, which `doctor` cannot know (§The verbs).

### (2) The witness {mechanical}

Each case runs over a scratch repository whose local config sets an empty `user.name`, which refuses the identity under any host's global config, so no case touches the process environment. The act depends on whether `init-refusing-hook-witness` lands in the same build batch. Each **Not yet applied.**

- **With it in the batch:** the identity case drives the `init` verb itself, on that entry's end-to-end harness, and asserts exit 1, no manifest written, nothing staged, git's account and the refusal on stderr. A second drives `uninstall` the same way over an installed scratch and asserts nothing removed.
- **Without it:** the cases call the helper over the scratch repository and assert the refusal, its status and its help for each verb. A case with a local identity set asserts it passes. A test over each verb's guard asserts the helper is reached only where the clean-worktree check is.

### (3) The SPEC sections {design-bearing}

Each **Not yet applied.**

- installer/SPEC.md §init, the precondition list: four preconditions, adding *git has an identity to commit with*, waived with the clean worktree by `--no-commit` and a personal install, and read with `git var` for the author and the committer both, since the commit needs both (delta 1).
- installer/SPEC.md §init, *Every refusal `init` makes about the machine is this one*: scoped to the machine's toolchain, since the identity refusal is about git's configuration (delta 1).
- installer/SPEC.md §init, *What names no hook*: the plain line's causes no longer include a missing identity, refused before the commit now; a refused signature stays among them (delta 1).
- installer/SPEC.md §update, the roster of refusals arriving prefixed `checkwright init:`: adds a missing git identity (delta 1).
- installer/SPEC.md §uninstall, *Preconditions*: a bullet for the identity, on `init`'s terms and waived as the clean worktree is (delta 1).
- The release declaration surface gains a `behavior` bullet: `init`, `update` and `uninstall` refuse at exit 1 before any write where git has no identity to commit with, where they wrote and staged everything first (deltas 1 and 2).

## Producers and consumers

- **The refusal** (delta 1). Producer: the helper, called from `vendor` (`native/src/installer/init.rs`) and `remove` (`native/src/installer/uninstall.rs`) after the clean-worktree check, inside its `!f.dry && f.commit && !personal` guard. Its trigger is the adopter's git configuration: an identity that git's strict read refuses. No knob enables it. Consumers: the adopter, through stderr and the exit status; `finish`, which renders a `Refusal` for both verbs; the unit tests.
- **The no-op runs.** An `init` re-run with nothing to change, and an `uninstall` with nothing to remove, pass the guard before either knows it, so in an identity-less clone each now refuses at exit 1 where it exited 0 (delta 1).
- **No shipped caller runs a committing verb on a seat that may lack an identity**, so none newly refuses. The CI action (`installer/action.yml`) runs the battery alone, through `install.sh --run`. `prove` runs none of the three verbs, and its one commit is in its own scratch clone. `demo` runs `--init` in a scratch repository whose config sets both halves (`native/src/installer/demo.rs`). §The consumer smoke's consumers are made by `consumer()` in `native/src/emit/installer_smoke/consumer.rs`, which sets both. In `.github/workflows/gates.yml`, each repository a leg makes and runs a committing verb in sets both after its `git init`. The upgrade smoke, the consumer-smoke driver, the kits' `smoke/` scripts and the gate tests call none of the three verbs.
- **`personal-detect-non-utf8`**, in the same iteration, changes what `personal` reads, and so where the guard waives this check, as it does for the clean-worktree check. Neither act depends on the other.

The callers were found by `grep -n "the worktree is not clean" native/src/installer/*.rs`, the scratch consumers by `grep -rn '"init", "-q"' native/src/emit/installer_smoke native/src/installer` and `git grep -n "config user.email" -- .github/workflows`, and the verbs' other callers by a `git grep` for `--init`, `--update` and `--uninstall` over `native/src/emit/upgrade_smoke.rs`, `native/src/emit/csmoke.rs`, `native/src/installer/demo.rs`, `native/src/installer/prove.rs`, `installer/action.yml`, the kits' `smoke/` and the gate tests.

## Existing sections updated

- `native/src/installer/mod.rs`, the identity helper and its unit tests (deltas 1 and 2).
- `native/src/installer/init.rs`, `vendor`'s precondition block and the comment naming three preconditions (deltas 1 and 2).
- `native/src/installer/uninstall.rs`, `remove`'s precondition block (deltas 1 and 2).
- `installer/SPEC.md` §init, §update and §uninstall (deltas 1 and 3).
- `.workflow/release-declarations.md`, a `behavior` bullet (delta 3).
- `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

The SPEC surfaces were found by `git grep -n -i "three preconditions\|missing identity\|Preconditions" -- installer/SPEC.md native/src/installer`.

## Retired spellings

- None — no delta retires a name; the precondition and its refusal are new.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery and the crate's tests green. No remote run observes this entry, so its Done move lands before the drain stage, in the build batch that lands it.
