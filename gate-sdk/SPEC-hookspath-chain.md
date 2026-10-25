# SPEC amendment: hookspath-chain

Pairs `hookspath-chain`. `--install-hooks` refuses a clone already running hooks (gate-sdk/SPEC.md §install-hooks), so a personal install on a project whose tracked hook manager runs its hooks gets no gate hooks. Calling `--git-hook` from that manager means editing the project's tracked files. The entry's deliverable is the envelope: chain rather than refuse. Record the prior hooks directory, run its same-named hook ahead of the battery from the placed hooks, place a forwarding hook for each other live hook, re-place them on `--refresh`, and restore the prior value at uninstall.

**What the tree does today**, probed at spec on git 2.55:

- **The refusal is the bare arm's one branch on a live prior.** `native/src/emit/install_hooks.rs`, `dispatch`, matches `prior()`'s `Prior::Live` and returns `live_refusal`'s two lines at exit 2. `Prior::Quiet { set: Some(_) }` prints the replaced-value note.
- **The launcher answers only to the served names.** `native/src/main.rs` calls `hook_launcher::served` on the start name and otherwise parses arms, so the binary started under `pre-push` would read git's operands as arms.
- **`git hook run` gives the hook no standard input unless `--to-stdin=<path>` is passed.** Probed in a scratch repository: a `pre-push` printing its stdin saw nothing under `git -c core.hooksPath=<dir> hook run pre-push -- origin url`, and saw the line under `--to-stdin=<file>`. The exit status passed through (3 in, 3 out). A missing hook exits 1, and 0 under `--ignore-missing`. git 2.55's usage line also lists `--allow-unknown-hook-name` and `-j`. The command's introducing release, 2.36, is above the git floor of 2.15 (the survey record's scope block).
- **The launcher's spawn inherits standard input.** `proc::run_to` sets no stdin, so a child started through it reads what git gave the hook.
- **A spawned program needs a declared ground.** `Program::consumer` takes a ground that `programs.rs`'s `every_consumer_commands_ground_names_a_knob_or_the_gate_declaration` holds to a knob or `GATE_DECLARATION` (gate-sdk/SPEC.md §The program roster). A prior hook is neither.
- **`uninstall` prints a fixed line.** `native/src/installer/uninstall.rs` sets `hooks_line` to `git config --unset core.hooksPath` whenever the clone is opted in.

**Two choices this amendment makes.**

- **The restore is printed, not written.** installer/SPEC.md §uninstall rules that the opt-in "is reported, not rewritten", git config lying outside the ownership roster. The governing spec settles that, so "restore the prior value at uninstall" is met by printing the restoring command. The command is read from the record, where today's line is fixed.
- **The Windows path is the lead's grant to this stage.** On unix the placed hook starts the prior hook directly, which is what git does, so no git release is needed. On native Windows a script hook needs git's own shebang handling, so the chain goes through `git hook run --to-stdin`. A git that cannot run it keeps today's refusal, decided by a probe at install time rather than a parsed version.

## What changes

### (1) The bare arm chains a live prior {design-bearing} {user-facing: the queue entry's deliverable, "chain rather than refuse"}

In `native/src/emit/install_hooks.rs`, `dispatch`, a `Prior::Live` proceeds rather than refusing, except as delta 3 states. **Not yet applied.**

- **The chain record.** A bare run whose prior is not the hooks directory itself writes `chain`, a file in the hooks directory, named by a new `hook_launcher::CHAIN` constant. It holds two lines:
  - `dir=<prior-dir>`, the directory `prior()` resolved. That resolution is physical once `hookspath-prior-physical` has landed, which the set's order puts first. If the two land in one batch, the record still reads `prior()`'s result and takes no resolution of its own.
  - `restore=<value>`, `core.hooksPath`'s local-scope value before the run as `git config --local --get core.hooksPath` prints it, empty where that scope set none.
- **Where it is written.** The record is written under a sibling name and renamed, as a hook is, after the hooks are placed and before `core.hooksPath` is written. A failed write exits 2 there, with git not pointed at the hooks. A value containing a newline is refused at exit 2 before anything is placed, since the record cannot carry it.
- **A re-run.** A run whose prior is the hooks directory writes no record and keeps the one there.
- **Forwarding hooks.** For each live hook in the recorded directory whose name, with the host suffix removed, is not a served hook, the run places a **forwarding hook**: the binary under that hook's file name (`hook_launcher::file_name`), placed by `place_as`'s sibling-and-rename as the served hooks are. The served hooks are placed as today.
- **The chain line.** Where the prior directory held a live hook, one line on stdout ahead of the `Installed:` line names it and them:

  ```text
  install-hooks: this clone ran hooks from <prior-dir> (<names>); each now runs from <hooks-dir>, ahead of the gates where they share a hook
  ```

  `<names>` is the live hooks in name order, joined by `, `. The replaced-value note for a prior holding no live hook stays as it is, and a default directory holding none still prints nothing.

Unit tests in the module's `tests`, each over a scratch repository:

- A live `pre-commit` in the default directory is chained: `prior()` still answers `Prior::Live`. The run writes a record whose `dir` is that directory and whose `restore` is empty, and places no forwarding hook.
- A local `core.hooksPath` naming a directory with a live `pre-push` writes `restore=<that value>` and places a forwarding `pre-push`.
- A global-scope value only (set through `GIT_CONFIG_GLOBAL` naming a scratch file) writes an empty `restore`.
- A default directory of `*.sample` files writes a record and places no forwarding hook.
- A re-run keeps the record byte-identical.

### (2) The placed hook runs the prior hook, and the forwarding hook {design-bearing} {user-facing: the queue entry's deliverable, "run its same-named hook ahead of the battery … a forwarding launcher for each other live hook"}

In `native/src/emit/hook_launcher.rs` and `native/src/main.rs`. **Not yet applied.**

- **Reading the record.** The launcher's directory is the start name's parent. Where `<that dir>/chain` exists, the launcher reads it.
  - A record that cannot be read, or lacks a `dir=` line, refuses the hook at exit 2 with `<hook>: cannot read the chain record <path> (<cause>) — the hook could not run; treating as failure (not clean)`, since reading past it would drop the prior hooks silently.
  - A `dir` naming the hooks directory itself (`same_dir`) is read as no record.
- **A served hook runs the prior hook first.** The served-hook launcher resolves `GATE_SDK_NATIVE_BIN` as today, refusals included, before anything runs. Where the record's directory holds a live hook of its own name, by `install_hooks::live_hooks`' rule, the launcher starts it with git's arguments, standard streams and working directory.
  - A non-zero status prints `<hook>: the chained hook <path> exited <N> — the gates did not run` on stderr and returns `<N>`, the funnel's `128 + <signal>` for a hook a signal ended.
  - Zero goes on to start the binary as today.
  - With no record, or no live hook of that name, the launcher is as today.
- **The forwarding hook.** `main.rs` treats a start name whose stem is a git hook name (`GIT_HOOKS`) but not a served one, with `chain` beside it, as a forwarding hook (`hook_launcher::forward`). It runs the recorded directory's live hook of that name in the same way and returns its status. It resolves no knob and starts no gate binary. With no live hook of that name it exits 0, as git does for a hook it cannot find. A start name of that kind with no record beside it parses arms as today.
- **How the prior hook starts.**
  - **On unix** it is started directly, as git starts a hook. It is a `Program::consumer` under a new ground, `programs::CHAINED_HOOK`, whose value names the chain record.
  - **On native Windows** it is started as `git -c core.hooksPath=<dir> hook run --ignore-missing --to-stdin=<file> <hook> -- <arguments>`. `<file>` is a scratch file under the system temp dir holding the launcher's standard input, read to its end, and is removed after. git's own lookup and shebang handling start the hook.

Unit tests:

- `hook_launcher`'s record read, for a well-formed record, one with no `dir=` line, an unreadable one, and one whose `dir` is its own directory.
- The start-name dispatch: a non-served hook name with a record beside it is a forwarding hook, and without one it is not.

The end-to-end case is delta 5's.

### (3) The native Windows capability refusal {design-bearing} {user-facing: lead dispatch 2026-10-25, "the chain's Windows path is a decision for this stage"}

On native Windows only, a bare run whose prior directory holds a live hook first runs, before anything is placed, `git -c core.hooksPath=<empty scratch dir> hook run --ignore-missing --to-stdin=<empty scratch file> pre-commit`. Exit 0 proceeds to delta 1. Any other status refuses at exit 2, placing nothing, with two lines on stderr. `live_refusal` is rewritten to carry them. **Not yet applied.**

```text
install-hooks: this clone runs hooks from <prior-dir> (<names>), which this git cannot chain ('git hook run --to-stdin' is not available) — nothing placed
  help: upgrade git, or call the gates from your hook manager — '<bin> --git-hook pre-commit' from its pre-commit and '<bin> --git-hook commit-msg <message-file>' from its commit-msg — or move those hooks aside or unset core.hooksPath, then re-run --install-hooks.
```

A unit test, `#[cfg(windows)]`, holds the probe exiting 0 on the leg's git, and a host-independent one holds the two lines' shape.

### (4) Re-run and `--refresh` re-place the forwarding hooks {mechanical}

In `native/src/emit/install_hooks.rs`, a bare re-run and `refresh` both read the record where one exists. They place the served hooks and a forwarding hook for each live hook in the recorded directory, so a hook the prior directory gained since the last run is reached. A forwarding hook whose prior hook has gone is left in place, and it exits 0. `--refresh` still writes no config, runs no identity rung and prints no receipt, and the record is not rewritten. **Not yet applied.**

The unit test extends delta 1's re-run case. A `post-merge` planted in the recorded directory after the first run gains a forwarding hook at the re-run, and at a `refresh_target`-gated refresh too.

### (5) The end-to-end case {design-bearing}

`gate-sdk/gate-tests/native-git-hooks.test.sh` gains one case, over a scratch repository whose default hooks directory holds two hooks before `--install-hooks`. **Not yet applied.**

- **The hooks.** A live `pre-commit` script appends `pre-commit` to a file under the git directory, and a live `post-commit` script appends `post-commit`.
- **A clean commit.** One commit through the placed hooks leaves the file reading `pre-commit` then `post-commit`. The hook's `pre-commit: <N> gate(s) passed.` line is printed, and the opt-in's stdout carries the chain line naming both hooks.
- **A failing prior hook.** With the `pre-commit` script changed to exit 1, a commit is refused. stderr carries `the chained hook` and `the gates did not run`, and stdout carries no `gate(s) passed.` line.

On a host with an executable suffix the case skips, as the suite's stub cases do (gate-sdk/SPEC.md §git-hook), since its hooks are sh scripts. The Windows witness is delta 7's.

### (6) The restore line {mechanical} {user-facing: the queue entry's deliverable, "restore the prior value at uninstall", met by printing under installer/SPEC.md §uninstall's reported-not-rewritten rule}

**Not yet applied.**

- **The receipt.** `install_hooks.rs`'s receipt line `Disable with:  git config --unset core.hooksPath` names `git config core.hooksPath <restore>` where the record carries a non-empty `restore`, and is unchanged otherwise.
- **`uninstall`.** `native/src/installer/uninstall.rs` reads the record before it removes the hooks directory. Where the clone is opted in and the record carries a non-empty `restore`, `hooks_line` is `git config core.hooksPath <restore>`, and otherwise it is unchanged. `--dry-run` prints the same line.
- **Unit tests.** One case in `uninstall.rs`, over a scratch opted-in clone whose record carries a value, reads the printed line.

### (7) The SPEC sections and the personal arm {design-bearing}

Each **Not yet applied.**

- gate-sdk/SPEC.md §install-hooks, the paragraph "**A clone already running hooks is refused at exit 2, placing nothing**, since git runs hooks from one directory and pointing it here stops every other." and its six bullets become:

  > **A clone already running hooks is chained, never silenced**, since git runs hooks from one directory and pointing it here stops every other.
  >
  > - **The prior hooks directory** is `core.hooksPath`'s effective value at any scope, read with `git config --path --get core.hooksPath` and resolved against the work-tree top when relative, as git resolves it. With the key unset it is git's default, the path `git rev-parse --git-path hooks` names.
  > - **A live hook** is a file there that git would run: named for a git hook, with the host's executable suffix or without, a symlink followed, and carrying the executable bit where the host has one.
  > - **The chain record.** A bare run whose prior is not the hooks directory writes `chain` there, placed as a hook is: `dir=` the prior directory, and `restore=` `core.hooksPath`'s local-scope value before the run, empty where that scope set none. A re-run keeps it, and the default directory's contents are not read.
  > - **Each live prior hook keeps running.** One the gates serve runs from the placed hook, ahead of them (§git-hook). Each other gets a **forwarding hook**, the binary under its name, which runs it and nothing else. A bare run and `--refresh` place one for each live hook in the recorded directory, so a hook added there is reached at the next run. One whose prior hook has gone exits 0.
  > - **The run names what it chained**, on a line ahead of `Installed:`. A prior `core.hooksPath` holding no live hook prints a note naming the value instead. A default directory holding none prints nothing.
  > - **On native Windows the chain starts a prior hook through `git hook run --to-stdin`**, since a script hook there needs git's shebang handling. A bare run over a live hook first probes that this git runs it, and one that does not refuses at exit 2, placing nothing. The refusal names the directory, the hooks and the cause, and its help names the routes: upgrade git, call `<bin> --git-hook` from that hook manager (§git-hook), or move those hooks aside and re-run.
  > - **The restore is printed, never written** (installer/SPEC.md §uninstall): the receipt and `uninstall` name `git config core.hooksPath <restore>` where the record carries a value, and `git config --unset core.hooksPath` otherwise.
  > - **Honest limits:** the hook names are githooks(5)'s at the git release this build read, so a hook a later release adds reads as no hook. A hook manager that writes `core.hooksPath` again on its own install step points git away from the gates, and a re-run chains it again. A clone opted in before the record existed chains nothing until `core.hooksPath` is cleared and the arm re-run.

- The same section's paragraph opening "**Both served hooks are always placed**": its sentence "A re-run replaces both files and is otherwise idempotent." becomes "A forwarding hook is placed the same way, and a re-run replaces every placed hook and is otherwise idempotent."
- The paragraph opening "**`--refresh` re-places the files**" gains, after its first sentence, "The files are the served hooks and a forwarding hook for each live hook in the recorded directory."
- The case paragraph opening "Its cases are the placement module's unit tests": its prior-directory sentences, from "The prior hooks directory has five" to the paragraph's end, are rewritten as the unit cases of deltas 1, 3, 4 and 6 and delta 5's suite case, stated as their cases.
- The paragraph opening "**It is an `Arm::Run`**", and §The non-gate arm's `--install-hooks` bullet: the exit-2 member "a live hook in the prior hooks directory" becomes "a live prior hook this git cannot chain on native Windows, a chain record that cannot be written".
- §git-hook, after the paragraph opening "**The binary, started under a file name whose stem is a git hook it serves**", a new paragraph:

  > **Where the hooks directory holds the chain record (§install-hooks), a placed hook runs the prior hook first.** The launcher resolves the binary as above, then starts the recorded directory's live hook of its own name with git's arguments, standard streams and working directory. A non-zero status ends the hook with that status and `<hook>: the chained hook <path> exited <N> — the gates did not run` on stderr. Zero goes on to the binary. **Started under any other git hook's name with the record beside it, the binary is a forwarding hook:** it runs that prior hook the same way and returns its status, reading no knob and starting no gate binary. On native Windows the prior hook starts through `git -c core.hooksPath=<dir> hook run --ignore-missing --to-stdin=<file>`, `<file>` holding the standard input git gave the hook. A record that cannot be read refuses the hook at exit 2, since reading past it would drop the prior hooks silently. **Honest limit:** a chained script hook still needs the interpreter its first line names.

- The same section's sentence "The launcher reads no knob beyond `GATE_SDK_NATIVE_BIN`, and the work-tree pin, the selection and the output strings below are the arm's." becomes "The launcher reads no knob beyond `GATE_SDK_NATIVE_BIN`, and no file beyond the knob files and the chain record. The work-tree pin, the selection and the output strings below are the arm's."
- §The program roster, the first bullet "**A consumer-named command** (a knob's argv, a gate's `.sh` declaration) enters through `Program::consumer`, which carries the knob or declaration that named it." becomes "**A consumer-named command** (a knob's argv, a gate's `.sh` declaration, a chained hook) enters through `Program::consumer`, which carries the knob, the declaration or the chain record that named it." The fourth unit-test relation becomes "every consumer command's ground names a knob, the gate-declaration ground or the chained-hook ground". `programs.rs`'s ground scan admits `CHAINED_HOOK` beside `GATE_DECLARATION`, and its test name follows.
- installer/SPEC.md §uninstall, the paragraph opening "**The placed hooks are removed**": the sentence "When `core.hooksPath` names that directory, `uninstall` prints the `git config --unset core.hooksPath` line for you to run." becomes "When `core.hooksPath` names that directory, `uninstall` prints the line restoring the prior value for you to run, read from the directory's chain record before its removal (gate-sdk/SPEC.md §install-hooks): `git config core.hooksPath <value>`, or `git config --unset core.hooksPath` where none was set."
- installer/SPEC.md §The consumer smoke, the personal arm's step 2: its first two sentences, "A live `pre-commit` planted in the clone's default hooks directory first makes the block's `--install-hooks` exit 2 naming it, with `core.hooksPath` unset. With the plant removed, the follow-up block's commands run, `--install-hooks` among them.", become "A live `pre-commit` planted in the clone's default hooks directory appends a line to a file under the git directory. The follow-up block's commands run, `--install-hooks` among them, whose chain line names the plant." Its last sentence gains ", and the plant's file holds two lines, one for the refused commit and one for this".
- `native/src/emit/installer_smoke/personal.rs`: `live_hook_refused` becomes the step above. The plant stays in place through the block and both commits, and the two-line read follows the `README.md` commit. Its `say` line names the chain and the two runs.

This step is the one witness that runs the chain through a placed binary on every install-smoke host. On the Windows legs it is the `git hook run --to-stdin` path.

**Inferred, cannot run before build:** that each Windows install-smoke leg's git passes delta 3's probe, so the step chains there rather than refusing — no Windows host is on this machine. The legs run at the set's mid-iteration push.

### (8) The front door and the release declaration {mechanical} {user-facing: the queue entry's deliverable, as delta 1}

Each **Not yet applied.**

- `docs/install.md` §On a project that does not adopt it: "Where a hook manager already runs this clone's hooks, `--install-hooks` refuses rather than silence them, and `--run` is your check." becomes "Where a hook manager already runs this clone's hooks, `--install-hooks` keeps them running, each ahead of the gates where they share a hook."
- installer/SPEC.md §The personal install, **Honest limits**, the last sentence "On a clone already running hooks, `--install-hooks` refuses (gate-sdk/SPEC.md §install-hooks), so the install checks at the battery alone until those hooks call the gates, which a hook manager whose configuration the project tracks cannot do untracked." becomes "On native Windows with a git that cannot chain a clone's existing hooks, `--install-hooks` refuses (gate-sdk/SPEC.md §install-hooks), so the install checks at the battery alone there."
- `.workflow/release-declarations.md`, the `**gate-sdk \`--install-hooks\`**` bullet, is rewritten whole, since the refusal it declares has not shipped:

  > - **gate-sdk `--install-hooks`** — a clone whose prior hooks directory, `core.hooksPath`'s effective value or else git's default `hooks`, holds a live hook now keeps running those hooks, chained from the placed ones, where `core.hooksPath` was repointed and they silently stopped. A hook the gates serve runs ahead of them, and each other through a forwarding hook. The receipt and `uninstall` print the command restoring the prior `core.hooksPath`. On native Windows a git without `git hook run --to-stdin` is refused at exit 2 placing nothing. Nothing to do unless the arm refuses; then upgrade git, or call `<bin> --git-hook pre-commit` and `<bin> --git-hook commit-msg <message-file>` from your hook manager.

The page sentence is three words shorter, so `check-docs-page-length` moves only down.

## Producers and consumers

- **The chain record** (delta 1). Producer: the bare arm, on every first run whose prior is not the hooks directory, the deployed path for any adopter running the follow-up block. Fields and their readers:
  - `dir`: the served launcher and the forwarding hook at every hook start (delta 2), and the bare re-run and `--refresh` (delta 4);
  - `restore`: the receipt's disable line and `uninstall`'s printed line (delta 6).

  Roster-holding readers of the hooks directory, found by `git grep -n "hook_launcher::DIR\|hook_launcher::missing\|hooks_dir(" native/src`:
  - `doctor` and `prove` read `missing()`, served names only, so a record is no missing hook;
  - `uninstall` removes the directory whole, after reading the record;
  - the receipt lists every file there, the record included, as it lists a stray sibling;
  - `installer_smoke/moves.rs` reads the placed `pre-commit` path alone.
- **The chained run and the forwarding hook** (delta 2). Producer: git starting a placed hook. Consumers: the adopter's prior hook, through its arguments, streams and status; the adopter, through the failure line; delta 5's case and delta 7's step.
- **`programs::CHAINED_HOOK`** (delta 2). Consumers:
  - `programs.rs`'s ground scan and its test, which hold a roster of admissible grounds and red on a name absent from it, so the scan and §The program roster are update targets;
  - a spawn failure's label, which names the ground.
- **`hook_launcher::CHAIN` and `hook_launcher::forward`** (deltas 1 and 2). Readers: the arm, the launcher, `main.rs`'s start-name dispatch and `uninstall.rs`.
- **The Windows probe and refusal** (delta 3). Producer: the bare arm on native Windows over a live prior. Consumer: the adopter. No leg's git is expected to refuse, so no smoke step reads the refusal, and its unit test reads the lines' shape.
- **The retired refusal on unix.** Its readers move with it. The unit test `a_live_hook_in_the_default_directory_refuses_and_samples_proceed` and `a_hooks_path_with_a_live_hook_refuses_and_an_empty_one_is_replaced` are rewritten as delta 1's cases. The personal arm's `live_hook_refused` is rewritten as delta 7's step. installer/SPEC.md's step 2 and §The personal install's honest limit are rewritten in deltas 7 and 8. Found by `git grep -n "live_refusal\|live_hook_refused\|refuses (gate-sdk/SPEC.md §install-hooks)\|exit 2 naming it, with"` over the tracked tree.
- **This repository.** Its clone's `core.hooksPath` names its own `gate-hooks` and it carries no record, so `build-native.sh`'s refresh places the served hooks alone, as today.

## Existing sections updated

- `native/src/emit/install_hooks.rs` (`dispatch`, `refresh`, the receipt, `live_refusal`, and the tests) (deltas 1, 3, 4 and 6).
- `native/src/emit/hook_launcher.rs` (`CHAIN`, `forward`, the record read and the chained start) and `native/src/main.rs` (the start-name dispatch) (delta 2).
- `native/src/programs.rs` (`CHAINED_HOOK` and the ground scan) and gate-sdk/SPEC.md §The program roster (deltas 2 and 7).
- `native/src/installer/uninstall.rs` (`hooks_line`) and installer/SPEC.md §uninstall (deltas 6 and 7).
- `gate-sdk/gate-tests/native-git-hooks.test.sh` (delta 5).
- gate-sdk/SPEC.md §install-hooks, §git-hook and §The non-gate arm's `--install-hooks` bullet (delta 7).
- `native/src/emit/installer_smoke/personal.rs` and installer/SPEC.md §The consumer smoke, the personal arm's step 2 (delta 7).
- `docs/install.md` §On a project that does not adopt it, installer/SPEC.md §The personal install, and `.workflow/release-declarations.md` (delta 8).
- `docs/llms-full.txt`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (delta 8).
- `docs/gate-sdk/SPEC.md` and `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 7 and 8).

The arm's surfaces were found by `grep -n "Prior::\|live_refusal\|Disable with" native/src/emit/install_hooks.rs` and `grep -n "hook_launcher::" native/src/main.rs native/src/installer/uninstall.rs`. The doc surfaces were found by `git grep -n -i "already running hooks\|refuses rather than silence\|runs hooks from\|live hook\|hook manager" -- '*.md'` over the tracked tree, the mirrors and dated posts excluded, read for the passages stating what the arm does to a clone running hooks.

## Retired spellings

- None — no delta retires a name. `live_refusal` survives with delta 3's narrower lines. The unit tests renamed in delta 1 are crate-internal, and no surface outside their module cites them.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests, the gate-sdk fixture suite (`native-git-hooks.test.sh` among it), the installer suite and the `installer_smoke` suite green.
- [ ] **The Done move** waits for delta 7's step watched green on the Windows legs at the set's mid-iteration push, and lands before the drain stage. A red there is fixed in the batch, since the push precedes the drain.
