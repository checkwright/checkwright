# SPEC amendment: hook-launcher

A placed git hook is the gate binary under a hook's name, and gate-sdk/SPEC.md §git-hook states that it only launches: it starts the binary `GATE_SDK_NATIVE_BIN` names and never judges. One part of it does judge. The launcher resolves that knob through the general knob read, which holds every line of gate-sdk's knob file to the launcher's own compiled table. A launcher older than the installed binary therefore refuses a commit over a `GATE_SDK_` name the installed binary knows and the launcher does not, and the commit that mints such a knob is refused at its own landing.

Measured, in a scratch repository whose knob file points `GATE_SDK_NATIVE_BIN` at the built binary, the placed launcher started under the name `pre-commit`:

- a name the launcher's table lacks in `gate-sdk-config.knobs` exits 2 with `is not a gate-sdk knob`, printed by the launcher;
- a name no table holds in another kit's knob file exits 0, so the defect is confined to gate-sdk's own file;
- with neither, the launcher starts the binary the knob names and returns its status.

Which writers leave a launcher behind is narrower than §git-hook says. Measured on a Linux host: the install placement op, run twice over one destination with two different artifacts, keeps the destination's inode, and a hard link to it carries the second artifact's digest. So an installer `update` writes through the link and the hook follows the binary. A writer that replaces the file leaves the link on the previous build: `cargo` does on every build of this crate, which is where the defect was met.

## What changes

### (1) The launcher reads one knob and holds no other line to its table

The launcher resolves `GATE_SDK_NATIVE_BIN` by a read of that one name, and a line naming any other knob is not its to judge. {design-bearing} {user-facing: operator direction at spec, on the entry — the launcher reads its one knob, a refresh option re-places an opted-in clone's hooks, the build calls it}

The read keeps gate-sdk/SPEC.md §The knob file's precedence for a scalar: the environment, the gitignored `.local` overlay, the tracked knob file, then the default. It parses the knob files only far enough to find that name. A line naming another knob, known to this build or not, is skipped unread. A line that sets `GATE_SDK_NATIVE_BIN` and cannot be parsed as a scalar refuses the commit at exit 2, naming the file and line, since the launcher then has no binary to start. A knob file that cannot be read refuses the same way.

Nothing is left unvalidated. The binary the launcher starts resolves its knobs through the general read, so every refusal the launcher used to print is printed by the installed build instead, against the table that build compiled. The remedy line a reader meets is therefore the current build's.

The cases: a crate unit test that the launcher's read returns the named binary from a knob file also carrying a name no table holds; and a case in `gate-tests/native-git-hooks.test.sh` in which `GATE_SDK_NATIVE_BIN` names a stub that exits 0, the knob file carries such a name, and a commit through the placed hook lands. A stub is the subject because one build cannot be older than itself: the case shows the launcher did not refuse, and the stub stands for a build that knows the name.

### (2) `--install-hooks --refresh` re-places an opted-in clone's hooks and does nothing else

The arm takes one option, `--refresh`, under which it re-places both hook files where the clone is already opted in and makes no other change. {design-bearing} {user-facing: operator direction at spec, on the entry — the launcher reads its one knob, a refresh option re-places an opted-in clone's hooks, the build calls it}

Opted in is §install-hooks' own predicate, `core.hooksPath` naming the hooks directory. Under `--refresh` the arm:

- places both served hooks as a bare run does, a hard link to the binary `GATE_SDK_NATIVE_BIN` names and a copy where the link cannot be made, and prints `install-hooks: refreshed <hooks-dir>` at exit 0;
- writes no git config, runs no `check-identity` rung and prints no `Active hooks:` receipt, since none of those changes when a file is re-placed;
- places nothing and prints nothing at exit 0 in a clone that is not opted in, so its caller never opts a clone in;
- places nothing and prints nothing at exit 0 when its working directory is not the work-tree top, because the binary's path and the hooks directory are both resolved from there, and a build run from a scratch directory inside a checkout would otherwise link that checkout's hooks to the scratch tree's binary;
- refuses at exit 2, with the bare run's message, when the binary it would link is absent or a file cannot be placed.

Its exit grammar is the bare arm's with the `1` unreachable, since no rung runs. Any other option stays the refusal it is today. The cases are the placement module's unit tests and two in `gate-tests/native-git-hooks.test.sh`: after the binary's file is replaced, `--refresh` leaves both hooks byte-identical to the new file, compared by digest so the copy fallback passes too; and in a clone with `core.hooksPath` unset it places nothing and leaves the key unset.

### (3) The crate build refreshes the placed hooks

`gate-sdk/bin/build-native.sh` runs the built artifact's `--install-hooks --refresh` after its artifact verification passes, on a host build only. {mechanical}

A build with a `--target` operand skips the step, because its artifact may not run on this host. The step's output passes through. A refresh that exits non-zero leaves the script's status cargo's and prints one line on stderr naming `--install-hooks` as the remedy, so a hook that could not be refreshed is named rather than silently left behind. The script calls the arm rather than linking the files itself: placement, the executable suffix and the opted-in predicate have one implementation, and it is the binary's.

No fixture builds the crate, `gate-tests/native-git-hooks.test.sh` included, and a build inside a test is not added for this. The case is the script run in this tree, opted in, with each placed hook's digest compared against the built binary's and the comparison recorded in the landing commit's message. What the fixtures hold is the arm the script calls, through delta 2's cases.

### (4) The staleness account names which writers leave a launcher behind

§git-hook and §install-hooks state when a placed launcher is older than the installed binary and what that costs. {mechanical}

Replacement text for §git-hook, the two sentences from "A placed hook therefore never judges" to "the judging build is the installed one". **Not yet applied.**

> A placed hook therefore never judges, a line of the knob files included: it reads `GATE_SDK_NATIVE_BIN` by name and leaves every other line to the build it starts. A hook is a hard link, so a writer that rewrites the binary's file in place carries the hook with it, as the install placement does (installer/SPEC.md §Placement). A writer that replaces the file leaves the hook on the previous build, acting only as a launcher: the crate build does, which is why it refreshes the hooks itself (§build-native), and so does a hook placed as a copy.

Replacement text for §install-hooks' opening sentence. **Not yet applied.**

> The binary's `--install-hooks` arm is the per-clone opt-in, run once to opt in and again only to take a newer launcher.

Added to §install-hooks after "A re-run replaces both files and is otherwise idempotent.", carrying delta 2's contract in the section's own voice. **Not yet applied.**

> **`--refresh` re-places the files of a clone already opted in and changes nothing else**: no config write, no identity rung, no receipt. It prints `install-hooks: refreshed <hooks-dir>`. A clone not opted in, or a working directory that is not the work-tree top, is a silent exit 0, so a build script may call it unconditionally.

**Inferred, not run:** that a checkout replacing a tracked binary leaves a teammate's hard-linked hook on the previous build — in a scratch repository that tracks a binary, link a file to it, check out a commit that changes it, and run `ls -li` on the two

Where that run shows the link left behind, §git-hook's list of replacing writers gains the checkout; where the link follows, the replacement text above stands as written.

## Producers and consumers

- **The launcher's one-name read (delta 1).** Producer: git, starting a placed hook at every commit in an opted-in clone, which this repo's contributors and every adopter who ran `--install-hooks` are. Consumer: the launcher's own start of the installed binary. The value has one reader, the spawn. No new knob, file or field is minted.
- **`--refresh` (delta 2).** Producer: `gate-sdk/bin/build-native.sh` after a host build (delta 3), and a person typing it. Consumer: git, which starts the re-placed files. Its one output line is read by the building session. The front end forwards its arguments to the binary, whose usage line and help paragraph for `--install-hooks` name the option.
- **The build step (delta 3).** Producer: every contributor build through the script, the commit-time obligation this repo's agent file already states. A consumer tree never reaches the script's build path, since it carries no crate, so the step is contributor-side only.
- **Roster-holding readers.** `--refresh` is an option of an existing arm, so it adds no arm-table row, no knob-roster entry and no front-end case arm. The arm's declared knobs are unchanged.
- **Narrowed corpus, point 5.** Delta 1 narrows what the launcher validates. Its one reader with a red condition is the commit itself, which the installed binary's read still refuses on every line the launcher skipped; no reader reds on finding none, asserts a count or holds a floor over the launcher's refusals.

## Existing sections updated

- `gate-sdk/SPEC.md` §git-hook — the launcher paragraph's staleness sentences, and "The launcher reads no knob beyond `GATE_SDK_NATIVE_BIN`" gains what that means for the file's other lines (deltas 1 and 4).
- `gate-sdk/SPEC.md` §install-hooks — the opening sentence, the `--refresh` paragraph, and the option-half sentence of "The member takes no positional", which today says every flag is a refusal (deltas 2 and 4).
- `gate-sdk/SPEC.md` §build-native — the refresh step, its host-build condition and its status rule (delta 3).
- `gate-sdk/bin/build-native.sh` — the step itself (delta 3).
- `native/src/emit/hook_launcher.rs` — the one-name read (delta 1).
- `native/src/emit/install_hooks.rs` — the option and its three no-op branches (delta 2).
- `native/src/runner.rs` — the usage line and help paragraph for `--install-hooks` (delta 2).
- `gate-sdk/gate-tests/native-git-hooks.test.sh` — the cases deltas 1 and 2 name.
- `installer/SPEC.md` §Reviewing the pre-commit hook — "each a hard link to the gate binary, or a copy where the link cannot be made" stands; the paragraph gains nothing unless delta 4's inferred claim comes back false, in which case it names the checkout (delta 4).
- `.workflow/release-declarations.md` — the `--install-hooks` behavior entry gains `--refresh` and the launcher's one-name read (deltas 1 and 2).
- `docs/gate-sdk/SPEC.md` — the generated mirror, stale the moment any of them lands (all deltas).

Produced by `git grep -n -i "one-time per-clone\|hook left behind\|older build\|re-run replaces both\|no re-install"` and `git grep -n "install-hooks"` over the tracked tree, read for passages that state when a hook is placed or how old it is. The kit READMEs' "places both hooks in a clone once" lines describe the opt-in and are left as they are.

## Retired spellings

- None — no delta of this amendment retires a spelling; the arm, its knob and both hook names keep their names.

## Refused alternatives

- **A freshness check naming a stale hook.** After delta 1 a stale launcher changes no verdict, so a check would red, or a `doctor` line would warn, on a state that harms nothing, and it would run at every commit to find what delta 3 removes at the one writer that causes it.
- **Refreshing from the installer's `update`.** The placement op rewrites the binary in place, so a linked hook already follows it; a refresh there would cover only the copy fallback, in the one clone that ran the verb.
- **Placing a symbolic link.** A link to a working-tree path is absent or wrong in a linked worktree whose binary lives elsewhere, and native Windows does not grant one without a privilege.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
