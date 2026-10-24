# SPEC amendment: hookspath-clobber

Pairs `hookspath-clobber`. `--install-hooks` points `core.hooksPath` at `gate-hooks` unconditionally, and git then runs no hook from anywhere else. A clone already running hooks loses them without a word. The fix is a refusal, on the operator's direction of 2026-10-24 (lead-relayed, not a ruling): refuse, widened to the default hooks directory; the help names the `--git-hook` call and the clear-and-re-run route; a prior directory with no live hook proceeds with a note; chaining is filed as a costed deferred feature.

**What the tree does today**, probed at spec on git 2.55 in a scratch repository:

- **A set `core.hooksPath` silences git's default hooks directory too.** An executable `.git/hooks/pre-commit` ran on one commit. After `git config core.hooksPath <dir>/gate-hooks` the next commit ran only the hook in `gate-hooks`. So the defect reaches every clone whose effective hooks directory holds a live hook, whether a hook manager wrote it into the default directory or set `core.hooksPath` to its own, as husky does. §install-hooks' "an adopter's existing hooks are not overwritten" is true of the files and silent on their no longer running.
- **A global `core.hooksPath` is overridden as well.** `git config --get core.hooksPath` answers the effective value whatever its scope, and the arm's local write shadows it.
- **The write is unconditional.** `native/src/emit/install_hooks.rs`, `dispatch`, calls `config("core.hooksPath", …)` after `place` with no read of the prior value.
- **The gates are callable from another hook manager.** `<bin> --git-hook pre-commit` and `<bin> --git-hook commit-msg <message-file>` are the arm every placed hook starts (§git-hook).

## What changes

### (1) The live-hook refusal {design-bearing} {user-facing: operator direction 2026-10-24, lead-relayed, "refuse, widened to the default hooks dir; exit 2 placing nothing when the prior effective hooks dir holds a live hook; the help names the two routes"}

In `native/src/emit/install_hooks.rs`, `dispatch`, after the hooks directory resolves and before `place`, the bare arm reads the **prior hooks directory**. **Not yet applied.**

- **The prior directory** is `core.hooksPath`'s effective value, read with `git config --path --get core.hooksPath` so `~` expands, resolved against the work-tree top when relative, as git resolves it. With the key unset, it is the path `git rev-parse --git-path hooks` names, absolutized against the same top.
- **The clone's own directory is no prior.** A prior directory naming the hooks directory (`hook_launcher::same_dir`) is a re-run, and the arm proceeds as today, with no read of its contents.
- **A live hook** is a regular file in the prior directory, a symlink followed, whose name is a git hook name or that name with the host's executable suffix. On a host with an executable bit it must carry one, since git skips a hook that does not. The hook names are a new `hook_launcher::GIT_HOOKS` constant, githooks(5)'s roster as git 2.55 prints it: `applypatch-msg`, `pre-applypatch`, `post-applypatch`, `pre-commit`, `pre-merge-commit`, `prepare-commit-msg`, `commit-msg`, `post-commit`, `pre-rebase`, `post-checkout`, `post-merge`, `pre-push`, `pre-receive`, `update`, `proc-receive`, `post-receive`, `post-update`, `reference-transaction`, `push-to-checkout`, `pre-auto-gc`, `post-rewrite`, `sendemail-validate`, `fsmonitor-watchman`, `post-index-change`. A `*.sample` file is no hook by this rule, since no hook name ends in `.sample`.
- **Any live hook refuses at exit 2**, before any file is placed or any config written. The refusal is two lines on stderr:

  ```text
  install-hooks: this clone runs hooks from <prior-dir> (<names>), and pointing core.hooksPath at <hooks-dir> would stop them running — nothing placed
    help: call the gates from your hook manager — '<bin> --git-hook pre-commit' from its pre-commit and '<bin> --git-hook commit-msg <message-file>' from its commit-msg — or move those hooks aside or unset core.hooksPath, then re-run --install-hooks.
  ```

  `<names>` are the live hooks in name order, joined by `, `. `<bin>` is the `GATE_SDK_NATIVE_BIN` value the arm resolved.
- `--refresh` is unchanged. It acts only on a clone whose `core.hooksPath` already names the hooks directory, so it never meets a prior.

Unit tests in the module's `tests`, each over a scratch repository: a live `pre-commit` in the default directory refuses, places no `gate-hooks` and leaves `core.hooksPath` unset; a default directory holding only `*.sample` files proceeds; a `core.hooksPath` naming a directory with a live `pre-push` refuses and leaves the value unchanged; on a host with an executable bit, a `pre-commit` without one proceeds; a `core.hooksPath` already naming the hooks directory proceeds with a live hook in the default directory.

### (2) The note on a hooks path holding no hook {mechanical} {user-facing: as delta 1, "a prior dir with no live hook proceeds, with a note"}

Where `core.hooksPath` named a directory other than the hooks directory and that directory holds no live hook, the arm proceeds and prints, on stdout ahead of its `Installed: core.hooksPath = …` line, `install-hooks: core.hooksPath was <value>, which holds no hook git runs; this clone now runs <hooks-dir>`. `<value>` is the configured string as git returned it. A default directory holding no live hook prints nothing, since nothing the adopter set is replaced. **Not yet applied.**

### (3) The SPEC sections {design-bearing}

Each **Not yet applied.**

- gate-sdk/SPEC.md §install-hooks, the bullet "It is its own directory, never git's default `hooks`, so an adopter's existing hooks are not overwritten and `git config --unset core.hooksPath` stays the per-clone off switch." becomes:

  > - It is its own directory, never git's default `hooks`, so `git config --unset core.hooksPath` stays the per-clone off switch. Git runs hooks from one directory, so pointing it here stops every other. **A clone already running hooks is refused at exit 2, placing nothing:** where the prior hooks directory, `core.hooksPath`'s effective value at any scope or else git's default, holds a live hook, a file named for a git hook (with the host's executable suffix or without) that git would run, the refusal names the directory and the hooks. Its help names the two routes: call `<bin> --git-hook` from that hook manager (§git-hook), or move those hooks aside and re-run. A prior `core.hooksPath` holding no live hook is replaced, and a note names the value. **Honest limit:** the hook names are githooks(5)'s at the git release this build read, so a hook a later release adds reads as no hook.

- §The non-gate arm, the `--install-hooks` bullet's exit-2 list gains "a live hook in the prior hooks directory" after "no repository".
- The section's closing case paragraph gains the five unit tests of delta 1, stated as their cases.

### (4) The personal arm's live-hook step {design-bearing}

`native/src/emit/installer_smoke/personal.rs`, step 2, and installer/SPEC.md §The consumer smoke's personal arm, step 2. **Not yet applied.** Before the follow-up block runs, the arm writes a live `pre-commit` into the consumer's default hooks directory, executable where the host has the bit. The block's `--install-hooks` line must exit 2 with stderr naming `pre-commit`, leave `core.hooksPath` unset and place no `gate-hooks` directory. The plant is then removed and the block runs as today. Step 2 in installer/SPEC.md gains one sentence opening it: "A live `pre-commit` planted in the clone's default hooks directory first makes the block's `--install-hooks` exit 2 naming it, with `core.hooksPath` unset; the plant is removed and the block runs."

This is the one witness that runs the refusal through a placed binary on every install-smoke host, the Windows legs included, where the executable-bit test does not apply.

### (5) The front door {mechanical} {user-facing: as delta 1, the entry's "the personal install's front door states it"}

Each **Not yet applied.**

- `docs/install.md` §On a project that does not adopt it gains, after the sentence ending "the project's CI runs none of it.": "Where a hook manager already runs this clone's hooks, `--install-hooks` refuses rather than silence them, and `--run` is your check."
- installer/SPEC.md §The personal install, **Honest limits**, gains a last sentence: "On a clone already running hooks, `--install-hooks` refuses (gate-sdk/SPEC.md §install-hooks), so the install checks at the battery alone until those hooks call the gates, which a hook manager whose configuration the project tracks cannot do untracked."

The page sentence adds 20 words to a page `check-docs-page-length` holds at 1500. `personal-install-prove`'s delta to the same section rewrites one sentence of it; that gate's verdict on the page with both in place is the oracle, and a cut, where owed, takes prose restating an owner doc, never these sentences.

### (6) The chaining gap {mechanical}

**Applied at spec**, being a gap-inbox write, in the commit that lands this amendment: chaining, the clone's prior hooks run from the placed hooks and a forwarding launcher placed for each other live hook, filed with `--emit file-gap` and its cost, for the personal install on a project whose hook manager the project tracks. Not built here.

## Producers and consumers

- **The prior hooks directory** (delta 1). Producer: git's config, every scope, else the default directory, read once per bare run. Consumers: the live-hook test and delta 2's note. A re-run, the directory being the clone's own, reads nothing.
- **The live-hook refusal** (delta 1). Producer: the bare arm, before `place`. Its red condition is one live hook. Consumers:
  - the adopter, by the two stderr lines;
  - `init`'s follow-up block, whose `--install-hooks` line an adopter runs. `init` never runs the arm itself (installer/SPEC.md §init), so no `init` status moves;
  - delta 4's step, which reads exit 2 and the hook's name;
  - the build script's refresh (§build-native), unaffected: `--refresh` never reaches the test.
- **`hook_launcher::GIT_HOOKS`** (delta 1). Producer: the constant. Consumer: the live-hook test alone. It is distinct from `SERVED`, the two hooks the launcher answers to, which it does not change.
- **The note** (delta 2). Producer: the bare arm, on a replaced value with no live hook. Consumer: the adopter. No smoke step reads it, since no consumer the smoke builds sets `core.hooksPath` beforehand; the unit test holding a no-hook prior reads the line.
- **This repository.** Its clone's `core.hooksPath` names its own `gate-hooks`, so a re-run, `build-native.sh`'s refresh included, is the clone's-own case and proceeds as today.
- **Every fresh consumer the smoke and the fixtures build** carries git's template hooks, all `*.sample`, so no existing arm meets the refusal. `gate-tests/native-git-hooks.test.sh` builds its repositories with `git init` too.

**Inferred, cannot run before build:** that Git for Windows runs a suffixless script hook in the default directory as live, so delta 1's suffix-or-none rule matches it — no Windows host is on this machine. The witness is delta 4's step on the Windows install-smoke legs. Those run at `init-copy-follows-links`' conditional mid-iteration push where that push happens, and at the closing push otherwise.

## Existing sections updated

- `native/src/emit/install_hooks.rs` (`dispatch` and its tests) and `native/src/emit/hook_launcher.rs` (`GIT_HOOKS`) (deltas 1 and 2).
- gate-sdk/SPEC.md §install-hooks and §The non-gate arm's `--install-hooks` bullet (delta 3).
- `native/src/emit/installer_smoke/personal.rs` and installer/SPEC.md §The consumer smoke, the personal arm's step 2 (delta 4).
- `docs/install.md` §On a project that does not adopt it and installer/SPEC.md §The personal install (delta 5).
- `.workflow/gap-inbox.md`, one bullet (delta 6).
- `docs/llms-full.txt`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (delta 5).
- `docs/gate-sdk/SPEC.md` and `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 3, 4 and 5).

The arm's surfaces were found by `grep -n "core.hooksPath" native/src/emit/install_hooks.rs native/src/emit/hook_launcher.rs`. The doc surfaces by `git grep -n -i "install-hooks\|hooksPath" -- '*.md'` over the tracked tree, read for the passages stating what the arm does to a clone.

## Retired spellings

- None — no delta retires a name. `GIT_HOOKS` and the refusal's and note's lines are new.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests, the gate-sdk fixture suite and the `installer_smoke` suite green.
- [ ] **The Done move** lands before the drain stage, on the local oracle. Delta 4's step on the Windows legs is read at whichever push runs them first. Where that is the mid-iteration push, the move waits for its runs watched green; where it is the closing push, a red there is filed at close.
