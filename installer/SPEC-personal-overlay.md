# SPEC amendment: personal-overlay

Pairs `personal-adoption`, its verification half alone. Under the entry's operator direction (2026-10-24, on consult advice, lead-relayed, not a ruling), a developer installs Checkwright's checks for their own work in a repository whose project does not adopt it. The install is an untracked per-clone overlay made by `init --personal`. It is the first consumer of installer/SPEC.md §Profiles' statement of the two halves: the halves part on tracking, so the verification half runs untracked and the workflow half, which commits its record, is refused here. That half is `personal-workflow-export`, deferred.

**What the tree does today**, probed at spec against this tree's own pack at `877b08b0e`. Each consumer was a fresh repository carrying a tracked `README.md`, a tracked shell script, a tracked `scripts/build.sh` and a tracked `CLAUDE.md`. The overlay was simulated as `init --no-commit`, then `git reset`, then the lock's `files` paths and `checkwright.lock` written to `.git/info/exclude`:

- **The overlay is clean and its battery green**, for `starter`, `prose` and `delegation`. `git status --porcelain` is empty. Under `GATE_SDK_VERBOSE` every clean line equals a tracked install's, with two exceptions:
  - `check-pipe-membership` scans only the project's tracked shell, and no longer the vendored kits'. That is the overlay governing the project rather than itself.
  - Under `delegation`, `check-task-conservation` reads "TASK-QUEUE.md not at HEAD — no prior live slugs to conserve". A workflow gate reading committed state goes permanently vacuous, the ground delta 2 refuses on.
- **The hooks gate the project's commits.** `--install-hooks` placed them. A commit breaking a README link was refused by `check-md-refs`, and a clean commit landed with `pre-commit: 13 gate(s) passed.`
- **A tracked `scripts/` and a tracked `CLAUDE.md` coexist with the overlay**: `prose` writes neither path, and the project's script stays tracked beside the excluded gate files.
- **Today's verbs over the overlay**:
  - a bare `init` re-run exits 2 at staging, since `git add` refuses an excluded path;
  - `prove` refuses with "HEAD carries no checkwright.lock" and a help line saying to commit what `init` wrote;
  - `uninstall` already removes every file and reports "none of which this repository was tracking — nothing to commit", leaving the exclude lines behind.
- **`init` seeded `.github/workflows/gates.yml` into the overlay**, a workflow no CI of the project runs.
- **1020 exclude lines** for a `prose` overlay, one per recorded path. Delta 3 collapses them.

## What changes

### (1) `--personal`, its flag row and its preconditions {design-bearing} {user-facing: the entry's operator direction of 2026-10-24, "an `init --personal` arm making an untracked per-clone overlay"}

`--personal` joins `FLAGS` in `native/src/installer/mod.rs` for `init` and `update`, since every `init` flag stays valid on `update` (§update). It also joins `init`'s parser, its `USAGE` line and its `--help`. **Not yet applied.** Under it, these are refused at exit 2, before any write:

- **Beside `--no-commit`**, which stages, while a personal install stages nothing: a usage refusal naming both.
- **A kit set carrying a kit that commits a process record.** A kit does so when its seed plan over an empty tree names a path under the workflow directory, or when it ships the queue template, the queue-owner predicate §What init seeds already applies. Both are read off the kit, never off this tree, so a project already holding such a file cannot pass the test. The refusal names each such kit and the path or template. Its `help:` names the profiles whose kit sets pass, derived by the same predicate over each profile: `starter` and `prose` today. A new `WORKFLOW_DIR` consumer-layout constant, `.workflow`, the gate-sdk default `init` writes against, sits beside `STATE_FILE`. A unit test holds `STATE_FILE` under it.
- **A path the run would write that the index tracks.** The set is every vendored kit file, every gates-directory file, each create-once seed planned, the projection and the lock, asked of the index in one batched `git ls-files`. `.git/info/exclude` cannot hide a tracked file, and writing one would edit the project's own tree. The refusal names the paths in the packer's bounded form (`DIRTY_SHOWN`, then `(and <n> more)`). Its `help:` says a personal install writes only paths the repository does not track. `--force` does not lift it, since `--force` means overwriting what `init` protects, and this protects the project.

The clean-worktree precondition is waived, as `--no-commit` waives it: no commit is made, so its two grounds, the one commit and the residue attribution, do not arise.

### (2) The run under `--personal` {design-bearing} {user-facing: as delta 1, and its "the front door stating the overlay has no project CI, the inner two tiers only"}

`vendor` in `native/src/installer/init.rs` writes the same files on the same claims, with these differences. **Not yet applied.**

- **No CI workflow is seeded.** The create-once CI seed prints `init: .github/workflows/gates.yml not seeded — a personal install has no project CI.` in the not-seeded line's form.
- **Nothing is staged and nothing is committed**, the no-op arm's commit of rewritten residue included. In their place, after the manifest is written, the exclude block (delta 3) is written to the file `git rev-parse --git-path info/exclude` names, with the file and its directory made when absent. It goes through `marker::install_block`, the marker-block writer `--install-lifecycle` uses for `.gitattributes`, between `# checkwright:personal:begin` and `# checkwright:personal:end`. The two markers are installer-module constants.
- **The report** is one line on every such run, re-runs included: `INIT: vendored <n> kit(s) at the <phrase> (v<version>) as a personal install, excluded in <exclude path> — nothing staged or committed.`
- **The follow-up block keeps its commands.** The `then:` line becomes `then: this install is personal: the hooks and --run check your commits in this clone alone, and the project's CI runs none of it.`, since `prove` reads `HEAD`, which carries no install.
- **The block is the install's record of being personal.** A run passing no `--personal` re-applies personal mode when the exclude file carries the begin marker, as a run passing no `--profile` re-applies the profile. The manifest takes no key for it, since a key would be a second source of what the block states. Leaving a personal install is `uninstall` and then a fresh `init`.
- **`--dry-run --personal`** prints the plan as today, then `would write this block to <exclude path>:` and the block, and writes nothing.

### (3) The exclude block's body {design-bearing}

Each recorded `files` path and `checkwright.lock` yields one line. The lines are deduplicated and sorted, and the body is recomputed whole on every run. **Not yet applied.**

- **A path with no `/`** yields `/<path>`.
- **A path under a first component the index holds no path beneath** yields `/<component>/`, one line for the directory, asked in one batched `git ls-files -- <component>…`. A vendored kit root is one line.
- **Any other path** yields `/<path>`, so a project's own tracked `scripts/` keeps its untracked files visible and only the gate files are hidden.
- Every line is anchored with a leading `/`, and each of `\`, `*`, `?` and `[` in a path is backslash-escaped, so a line matches the one path it names.

The directory in the recommended shape is not excluded whole. The gates directory's files are on the roster, and excluding the directory would also hide the project's own untracked files under a shared `scripts/`. The scratch directory needs no line, being self-ignored (§What init seeds).

### (4) `uninstall` over a personal install {design-bearing} {user-facing: as delta 1, the shape's "`uninstall` dropping the block"}

In `native/src/installer/uninstall.rs`, where the exclude file carries the begin marker, the clean-worktree precondition is waived on delta 1's grounds. The removal then runs as today, and its "nothing to commit" path already reports it. **Not yet applied.** Last of all:

- **With nothing kept**, the block is removed (`marker::remove_block`).
- **With any file kept**, the block is rewritten over the residual roster and the residual `checkwright.lock`, so a path the hash rule protected stays excluded.

`--dry-run` names which of the two it would do.

### (5) `prove`'s refusal names the personal install {mechanical} {user-facing: as delta 2, on its `prove` reading `HEAD`}

In `native/src/installer/prove.rs`, the *`HEAD` carries no `checkwright.lock`* refusal keeps its exit 2. Where the exclude file carries the begin marker, its `help:` line becomes `this clone's install is personal and commits nothing, and prove reads HEAD alone; <door> --run checks the working tree instead.` `<door>` is the placed binary's command token, as the follow-up block spells it. **Not yet applied.**

### (6) The SPEC sections {design-bearing}

Each **Not yet applied.**

- `installer/SPEC.md` §init gains a closing subsection:

  > ### The personal install
  >
  > `init --personal` installs for one clone of a repository whose project does not adopt Checkwright. It writes what `init` writes, on the same claims, and makes the files untracked rather than committed: a marker-bounded block in the clone's `info/exclude` names each, so nothing is staged or committed and the project's tree is untouched. The block is derived from the manifest's roster. A first component the index holds nothing beneath is excluded as one directory, and any other path as itself, so a project directory shared with the gates directory keeps its own untracked files visible. The block is also the install's record of being personal: a re-run that passes no `--personal` re-applies it, and the manifest carries no second copy.
  >
  > It is the verification half (§Profiles). Every gate reads the project's tracked content wherever the kits sit, and the placed hooks are per clone already, so the battery and the hooks govern the developer's own commits as they would a tracked install's. The workflow half commits its record, and an excluded path cannot hold one. So a kit set carrying a kit whose seeds land in the workflow directory, or that ships the queue template, is refused, and the refusal names the profiles that pass. A path the index tracks is refused too, since excluding cannot hide it. The clean-worktree precondition is waived, as `--no-commit` waives it, because no commit is made.
  >
  > **It runs at the inner two tiers only** (gate-sdk/SPEC.md §Enforcement tiers). No CI workflow is seeded, since the project's CI runs none of it, and `prove`, which reads `HEAD`, refuses naming the personal install. `uninstall` removes the files and then the block, or rewrites the block over what it kept.
  >
  > **Honest limits.** `git clean -X` or `-x` deletes the install with every other ignored file, and a re-run restores it. The block belongs to the clone, so every linked worktree of the clone reads its install as personal. A release built before `--personal` reads a personal install as a tracked one: its `init` refuses at staging, its `uninstall` removes the files and leaves the block behind, inert, and its `prove` refuses as it would on any lockless `HEAD`.

- §uninstall: in the **Preconditions** list, the clean-worktree bullet ends ", waived for a personal install, which commits nothing (§The personal install)". The paragraph opening **The placed hooks are removed** gains a closing sentence: "A personal install's exclude block goes last, or is rewritten over the residual roster when a file was kept."
- §prove, **Preconditions**: the lock bullet ends ", whose help names a personal install where the clone carries one (§The personal install)".
- §The verbs' table is unchanged, since the verbs' questions do not move.

### (7) The consumer smoke's personal arm {design-bearing}

`native/src/emit/installer_smoke/` gains a **personal arm** in the roster, in whichever part has the wall-clock room, the part count unchanged. §The consumer smoke gains its paragraph, stated as the steps below. **Not yet applied.** It runs on a fresh consumer at the lattice minimum. The consumer carries a tracked `README.md` and a tracked file in the gates directory under a name `init` never writes, and has an uncommitted edit to `README.md`. Each step is a named failure:

1. **Install.** `init --personal --profile <minimum>` exits 0, and its `INIT:` line names the personal install. `HEAD`, the index (`git ls-files -s`) and `README.md`'s uncommitted edit are unchanged. The porcelain shows that edit alone. The exclude file carries exactly one block, and every recorded path and `checkwright.lock` reads ignored through one `git check-ignore --stdin`. The `then:` line is the personal one.
2. **Commit through the hooks.** The follow-up block's commands run, and `--install-hooks` places the hooks. A commit planting the lattice minimum's real defect is refused, the refusing gate read off the hook's `pre-commit: <gate> failed` line and held to the registry. That defect is the shell pipeline that can lose its match, the `bash`-less arm's plant. The `README.md` edit then commits through the hooks.
3. **Re-run.** A bare `init` exits 0. No commit is made, and the exclude file is byte-identical.
4. **`prove`** exits 2, and its `help:` line names the personal install.
5. **Refusals.** Each runs in its own fresh consumer, exits 2, and leaves the tree object, the porcelain and the exclude file unchanged:
   - `--personal --profile <maximum>` names a kit committing a process record;
   - `--personal` over a tracked `<gates-dir>/gates.list` names that path;
   - `--personal --no-commit` names both flags.
6. **Reversal.** `uninstall` exits 0 and makes no commit. The exclude file equals its pre-install bytes, no `checkwright.lock` remains, and the porcelain is empty.

### (8) The front door {mechanical} {user-facing: as delta 2}

Each **Not yet applied.**

- `docs/install.md`, between §Choosing a profile and §Prove it on your tree, a new section:

  > ### On a project that does not adopt it
  >
  > `checkwright init --personal` installs the checks for you alone. It writes the same files untracked, lists them in this clone's `.git/info/exclude`, and commits nothing, so the project's tree and history stay as they were. Your hooks and `--run` check your own commits in this clone, and the project's CI runs none of it. `prove` reads `HEAD`, so it has nothing to read here. It takes `starter` or `prose`: the workflow commits its own record, so a profile carrying it is refused. `uninstall` removes the files and the exclude lines.

  The section adds 98 words to a page `check-docs-page-length` holds at 1500, and the `workflow` profile bullet adds 23 more. The page stood at 1493 words by that gate's measure (front matter and comments excluded), so the two together overrun it by about 114. The cut is owed in the same build, and the decoupling merge landed most of it: existing prose on the page that restates what an owner doc already states (the Content-tiering / SSOT rule) goes, never this section nor the bullet, and that gate's verdict on the page with both in place is the oracle.
- `docs/choosing.md` §When it fits gains a last sentence: "A developer whose project has not adopted it can run the checks on their own clone alone, untracked ([On a project that does not adopt it](install.md#on-a-project-that-does-not-adopt-it))."
- `docs/maintenance.md`, the bullet "`checkwright uninstall` reverses the install in one commit." ends ", or with none for a personal install".
- `installer/README.md`'s flag table gains, after `--no-commit`: `| --personal | init, update | install untracked, for this clone alone, committing nothing |`.

### (9) The two gaps this design leaves {mechanical}

Each is filed with `--emit file-gap` in the commit that lands this amendment. Neither is built here. **Applied at spec**, being gap-inbox writes:

- `prove` over a personal install: copying the roster into its clone, which reads `HEAD`, is the costed widening the entry names.
- `--install-hooks` replaces an existing `core.hooksPath` without a word. A project that already runs its own hooks, the likelier case on a project that does not adopt Checkwright, stops running them.

## Producers and consumers

- **The flag** (delta 1). Producer: the adopter's command line, and the block on a re-run (delta 2). Consumers:
  - `init`'s parser and `update`'s forwarding;
  - the crate test holding every parser and `usage:` line to `FLAGS` (§The verbs);
  - `check-front-door-verbs`. Invariant A holds `installer/README.md`'s flag table equal to `FLAGS`, so delta 8's row lands with delta 1. Invariant B admits the `--personal` that docs/install.md advertises after the `checkwright init` route only while this iteration's release disposition is a release. **So this unit makes the close release, or withdraw the advertisement.**
- **The process-record refusal** (delta 1). Producer: the kit set the selection resolves to. Its red condition is a kit whose empty-tree seed plan names a workflow-directory path, or that ships the queue template. Today that is lifecycle-kit and evidence-kit for the first, and queue-kit for the second, read off `recipe::seed`'s arms and `ls */templates/TASK-QUEUE.md`. `workflow`, `delegation` and `full` are refused, and `starter` and `prose` pass. Its one consumer is the adopter, through the refusal and its derived help.
- **The tracked-path refusal** (delta 1). Producer: the index, read once over the planned paths. Its red condition is any planned path in the index. Delta 7's second refusal is its witness.
- **The exclude block** (deltas 2 and 3). Producer: `init`, on every personal run, written after the manifest. Consumers:
  - git's ignore rules, at every `git status`, `git add` and hook run in the clone;
  - `init`'s re-run, reading the begin marker as the mode;
  - `uninstall` (delta 4) and `prove` (delta 5), reading the begin marker;
  - delta 7's assertions.

  Each field has a reader. The markers are read by all three verbs, and each line by git.
- **Gates that read the tracked tree.** The probe above shows every clean line unchanged except `check-pipe-membership`'s corpus, which narrows to the project's own shell. That gate reds on *finding* a pipeline, never on finding none, so the narrowing clears by inspection (causal-completeness point 5). `check-task-conservation`'s vacuity under `delegation` is unreachable once delta 1 refuses that kit set.
- **The workflow directory.** The overlay excludes no workflow-directory path, since delta 1 refuses every kit seeding one. So no checked projection of that directory is ever made ignored, and its tracked-or-ignored partition (gate-sdk/SPEC.md §The workflow directory) is never put in question.
- **The release.** No tracked install changes shape, and the lock's wire key is unchanged, so nothing in an existing tree reconciles. Which of the release note's declaration-bearing sections names the new flag is the close's call, under §The upgrade contract.

**Inferred, cannot run before build:** the overlay's battery and hooks behave on macOS and native Windows as on Linux, since `git rev-parse --git-path`, the exclude rules and `core.hooksPath` are git's own on every host — no macOS or Windows host is on this machine, and the witness is delta 7's arm, which build writes and the mid-iteration push's install-smoke legs run

## Existing sections updated

- `native/src/installer/mod.rs` (`FLAGS`, `WORKFLOW_DIR`, the marker constants) and `native/src/installer/init.rs` (deltas 1, 2 and 3).
- `native/src/installer/uninstall.rs` (delta 4) and `native/src/installer/prove.rs` (delta 5).
- `installer/SPEC.md` §init, §uninstall and §prove (delta 6).
- `native/src/emit/installer_smoke/` and `installer/SPEC.md` §The consumer smoke (delta 7).
- `docs/install.md`, `docs/choosing.md` §When it fits, `docs/maintenance.md` and `installer/README.md`'s flag table (delta 8).
- `.workflow/gap-inbox.md`, two bullets (delta 9).
- `docs/llms-full.txt`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (delta 8).
- `docs/installer/SPEC.md` and `docs/installer/README.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 6, 7 and 8).
- `installer/SPEC.md` §Profiles, the citation of §The personal install ending the halves paragraph's "so it runs from an untracked install too", which the decoupling merge deferred to this one since the section it cites lands here (delta 6).

The verb surfaces were found by `grep -n "HEAD carries no\|nothing to commit\|fn vendor\|git_batched(&root, &\[\"add\"\]" native/src/installer/*.rs`. The doc surfaces were found by `git grep -n "in one commit\|--no-commit" docs/install.md docs/maintenance.md installer/README.md`.

## Retired spellings

- None — no delta retires a name. `--personal`, the two markers and `WORKFLOW_DIR` are new.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and the `installer_smoke` suite green.
- [ ] **The remote oracle** — the mid-iteration push's install-smoke legs, the PowerShell and macOS ones included, run delta 7's arm green. The Done move lands before the drain stage, after that push's runs are watched green, never in the merge commit.
