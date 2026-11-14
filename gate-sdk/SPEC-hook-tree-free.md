# SPEC amendment: hook-tree-free

Pairs `liveness-hook-no-repo-run`, and carries the rule `nonutf8-repo-path-reads` lands under, one batch since both fix the arm's toplevel read. Started with no front end, `--hook` resolves the tree its working directory stands in and enters it before any knob is read (§The harness-integration arm). Where git answers none, the arm exits `0` before it validates its member. A harness may fire a child-turn-end event from any working directory, so a consumer whose sessions run outside a repository gets no turn-end guard at all, and today has to keep a scratch repository for the arm to stand in. This amendment lets a member whose inputs can all resolve absolute run with no repository, and declines it loudly where one cannot. It also gives the crate one rule for a repository path it reads from git or argv and cannot spell: a named refusal.

**What the tree does today**, read from `native/src/hook/mod.rs`, `native/src/walk.rs`, `native/src/emit/hook_launcher.rs` and `native/src/main.rs`, and probed at spec on Linux with the built binary, its working directory a scratch directory outside any repository:

- **No member runs.** `--hook subagent-stop-liveness` on a `SubagentStop` payload wrote `checkwright-gates: not inside a git repository` on stderr and a `systemMessage` envelope carrying the no-tree notice on stdout, exit 0. With every knob it reads set absolute in the environment and a running `shell` task in the payload, the output was the same, so the turn end went unguarded. An unknown member got the same notice, since `run` declines before the member is looked up.
- **The liveness reader is tree-free already.** `check-producer-liveness <absolute dir>` read an empty directory clean at exit 0, a dead record clean at exit 0, and a live record at exit 1, "the producer for run key 'k' is still running".
- **The knob layers open relative locators.** `knobs::load` opens `<GATE_SDK_GATES_DIR>/<stem>-config.knobs`, its `.local` overlay and the two legacy `.sh` names, the directory defaulting to `scripts`, each relative to the working directory. Outside a repository that is whatever directory the harness fired from, so a stray `scripts/` there would configure the member.
- **Three path reads are lossy.** `walk::toplevel_args` decodes git's answer with `String::from_utf8_lossy`. `walk::cwd` decodes through `display()`. `hook_launcher::hooks_dir` and `hook_launcher::opted_in` read the root through `to_string_lossy` and git's answers through `String::from_utf8_lossy`. `main` reads its arguments through `std::env::args`, which panics on one that is not UTF-8. The queue entry's three reproductions stand: a wrong cause at exit 2, a gate-hooks tree under a U+FFFD spelling with `core.hooksPath` pointed at it, and a panic on every later commit.

## What changes

### (1) The tree-free column and the dispatch {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): a per-member tree-free column in the member table, set for `subagent-stop-liveness` alone; widening the roster is a filed follow-on}

**The column.** `HOOKS` gains a fifth column, `tree_free: bool`, after the owning kit. It is true for `subagent-stop-liveness` alone. A member is tree-free when every input its rule reads can resolve absolute: its payload, its knobs, and every program it spawns. The column is a declaration the member's author makes, and the member enforces it at run time through deltas 2 and 3. Every reader that destructures a row (`members`, `owner`, `run`, the table's tests, and the census test in `native/src/emit/mod.rs`) takes the new column.

**The dispatch.** `hook::run` looks its member up before it asks for the tree. It then takes one of these paths:

- **The absent answer, a tree-free member, the shipped binding.** `walk::toplevel_opt` answers `Ok(None)`, the row is tree-free, and no `--binding` operand is given. The member runs in place, entering no directory, under a process-wide tree-free mark that deltas 2 and 3 read.
- **Everything else declines as today.** A refused or skipped repository, a dead `git`, a member that is not tree-free, an unknown member, or a `--binding` call where git answers none each write the no-tree notice and exit `0`. A refusal is the crosser's error arm, and the caller rule (§The crate's crosser) passes it through. Running a tree-free member there would read a refused repository as none.
- **Inside a tree, nothing changes.** The arm enters the toplevel and the member reads its knob files from there, tree-free or not.

**Honest limits.**

- A dead `git` declines even a tree-free member. The crosser cannot tell it from a refused repository, and the member's own inputs do not need git.
- A binding's payload rewrite is uncovered by the column, so a bound call keeps the decline until a binding is shown to run tree-free.

§The harness-integration arm's member-table paragraph names the column and its readers. *Started with no front end, the arm resolves the tree it runs in* gains the tree-free exception. The exec-form fail-open bullet says the decline covers every member except a tree-free one on the absent answer. **Not yet applied.**

**The follow-on.** Which other members could run tree-free is not weighed here. The gap is filed through the inbox at spec: widen the column to each member whose inputs can all resolve absolute, with its cases.

### (2) Knob resolution under the mark {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): with no tree only absolute knob values are accepted, and a relative value, the defaults included, declines loudly through `hook::decline` at exit 0}

**The layers open no relative locator.** Under the mark, `knobs::load` opens a knob file only where its locator is absolute by `walk::path_root`:

- **A default locator is skipped, unread.** That covers the tracked `<GATE_SDK_GATES_DIR>/<stem>-config.knobs`, its `.local` overlay, and the two legacy `.sh` refusals, where `GATE_SDK_GATES_DIR` is unset or relative. Their values take the table defaults or the environment.
- **A locator the consumer set is honored when absolute.** That covers `<KIT>_KNOB_FILE`, `<KIT>_LOCAL_KNOB_FILE`, and an absolute `GATE_SDK_GATES_DIR`. A relative one is an error naming the variable and its value, saying a run with no repository takes absolute locators only. The member reaches the error through `walk::knob_scalar` and declines on it, as it does on a malformed knob today.

The precedence is unchanged: the environment, then the overlay, the tracked file and the default (§The knob file).

**Path-valued knobs must be absolute.** Under the mark, `hook::tree_free_path(name, value)` holds a path-valued knob's resolved value to `walk::path_root`. A relative value declines through `hook::decline`, naming the knob and its value and saying that a run with no repository takes absolute paths only. That includes the defaults: `GATE_SDK_TMP_DIR`'s `.tmp` and `DELEGATION_KIT_STOP_LOG`'s `${GATE_SDK_WORKFLOW_DIR:-.workflow}/subagent-stop-liveness.log`. `subagent-stop-liveness` calls it on both knobs before it spawns the reader. **Grounds:** a relative value resolved against the firing directory names a different scratch set and log on each firing, so the reading would be green over an empty directory and would allow the turn end. That is a silent fail-open, the outcome the decline exists to make loud.

**The reader command.** Under the mark, a `DELEGATION_KIT_LIVENESS_CMD` head carrying a separator and no root declines through `hook::decline`, where inside a tree it would resolve against the toplevel. A bare head resolves on `PATH` as today. The empty default spawns the binary's own image, which the spec probe above shows reads an absolute directory with no tree.

**`walk::capture_path` is skipped under the mark.** Its relative-value rebase onto the main checkout has no checkout to find, and every value it would see is absolute by then. That also saves the `git` spawn. **Not yet applied.**

### (3) Both notice streams documented {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): the operator notice is the envelope on stdout, and the line on stderr is the cause}

§The harness-integration arm's decline paragraphs name both streams for both declines, the no-tree notice and `hook::decline`:

- **stdout carries the envelope.** On a turn-end event that is the operator-notice field, which the harness shows the operator.
- **stderr carries the cause line**, the crosser's diagnostic or the member's reason. At exit 0 the harness writes it to its debug log alone. A consumer testing the arm through a pipe reads the decline on stdout, and a test reading stderr alone sees the cause and no decline.

The no-tree notice's literal gains the absent answer's own remedy. It names running the session inside a repository, or registering a tree-free member, since inside a tree a `git status` has nothing to report. **Not yet applied.**

### (4) The turn-end hook's no-repository run {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): `subagent-stop-liveness` runs with no repository when its scratch dir and log are absolute}

delegation-kit/SPEC.md is re-phrased in place. Each **Not yet applied.**

- **§The turn-end liveness hook's contract paragraph** states that the hook runs where its working directory is in no repository, given an absolute `GATE_SDK_TMP_DIR` and `DELEGATION_KIT_STOP_LOG`, and declines with the operator notice where either is relative, the defaults included. It cites gate-sdk/SPEC.md §The harness-integration arm for the column and the knob rule.
- **The `DELEGATION_KIT_STOP_LOG` and `DELEGATION_KIT_LIVENESS_CMD` Layout bullets** each state the absolute requirement with no tree.
- **The registration paragraph** states that a consumer registering the hook outside any repository supplies the two paths absolute, through the environment or an absolute `<KIT>_KNOB_FILE`.
- **`delegation-kit/README.md` step 6** carries the same in one sentence.

### (5) A repository path the crate cannot spell is refused by name {design-bearing}

**The rule.** A repository path the crate reads from git, from the working directory or from argv, and which has no UTF-8 spelling, is refused at exit 2, or declined on a fail-open arm. The refusal names the read, the path with each byte that is not UTF-8 escaped as `\xNN` (each unpaired UTF-16 surrogate as `\u{XXXX}` on Windows), and the remedy. It is never decoded lossily, and the bytes are not carried.

**Grounds:**

- The crate's path model is `String`, so carrying bytes would mean an `OsString` path through every reader downstream of these three.
- The crate already refuses a name that is not UTF-8 at its listing (§Fail-closed contract), and a `core.hooksPath` that is not UTF-8 at `--install-hooks`. A lossy read beside those refusals names a path that does not exist, or worse, another path.

**The reads it lands at:**

- **`walk::cwd`** decodes `current_dir()` through `to_str`, and refuses a directory with no UTF-8 spelling.
- **`walk::toplevel_args`** decodes git's answer strictly. An answer that is not UTF-8 is the error arm, with its own cause rather than *not inside a git repository*. Every caller's existing error handling then reports it: `--install-hooks` exits 2 naming it, and `--hook` declines with the no-tree notice, the cause on stderr. `selected_repository`'s read decodes strictly too, and an answer that is not UTF-8 is not the selected repository.
- **`hook_launcher::hooks_dir` and `hook_launcher::opted_in`** take the root as `&str`, and decode git's `--git-common-dir` and `core.hooksPath` answers strictly. They return the refusal as an error. `--install-hooks` and the installer's `uninstall` exit 2 naming the common directory, placing, pointing and removing nothing. `doctor` and `prove` stay soft, reporting the cause as their finding. A UTF-8 linked worktree whose common directory is not UTF-8 no longer gets a U+FFFD tree.
- **`main`** reads `std::env::args_os`. An argument with no UTF-8 spelling, the start name included, is refused at exit 2 before any arm is chosen, naming its position and escaped bytes. Under a served hook's or a forwarding hook's start name, the refusal also names the remedies for a clone hooked before this rule: `git commit --no-verify` as a one-off, and unsetting `core.hooksPath` where `git config --show-origin --get core.hooksPath` says it was set.

**Honest limit:** a forwarding hook refuses an operand that is not UTF-8, which a pure forwarder could carry. One rule over all three reads is the point. A prior hook whose operands git spells outside UTF-8 is the only cost, and the refusal names it.

§The crate's crosser gains the rule after the `toplevel_opt` caller-rule paragraph, and its `walk::cwd()` bullet says the producer refuses. §git-hook's exit-2 list gains an argument that is not UTF-8. §install-hooks' honest-limit sentence, *a git directory whose path is not UTF-8 is not refused here*, is re-phrased: the common directory is refused, and the default hooks directory's read keeps `walk::physical_against`'s answer only beneath a UTF-8 common directory. installer/SPEC.md §uninstall's refusals gain the common directory, and §doctor's and §prove's hook readings gain it as a finding. **Not yet applied.**

### (6) Cases, the release declaration and the mirrors {mechanical}

Each **Not yet applied.**

- **The dispatch's cases** (`hook/mod.rs` tests, and the arm's integration tests where the probe above runs). These cover:
  - A tree-free member with no repository and absolute knobs, refusing on a running own `shell` task at exit 2.
  - The same member with default knobs, declining at exit 0, the envelope on stdout naming `GATE_SDK_TMP_DIR`.
  - A relative `DELEGATION_KIT_STOP_LOG`, declining.
  - A relative `DELEGATION_KIT_KNOB_FILE`, declining, and an absolute one, honored.
  - A stray `scripts/delegation-config.knobs` in the firing directory, unread.
  - A relative separator-bearing `DELEGATION_KIT_LIVENESS_CMD` head, declining.
  - A member that is not tree-free, an unknown member, and a `--binding` call, each keeping the no-tree notice.
  - A refused repository, declining a tree-free member.
  - The table test that holds `tree_free` true for `subagent-stop-liveness` alone, until the follow-on widens it.
- **The rule's cases**, compiled for Linux alone, a host whose filesystem admits a name that is not UTF-8:
  - `walk::cwd` and `walk::toplevel_opt` in a work tree under such a name, each refusing with the escaped name.
  - `--install-hooks` and the installer's `uninstall` in a UTF-8 linked worktree whose common directory is not UTF-8, each exiting 2 with nothing placed or removed and `core.hooksPath` unchanged.
  - The binary started with an argument that is not UTF-8, and under a served hook's start name with such an operand, each exiting 2 with no panic.
- **The release declaration** (`.workflow/release-declarations.md`):
  - Under `## Behavior changes`: `--hook subagent-stop-liveness` runs outside any repository when its scratch dir and log are absolute, and otherwise declines naming the knob. `--install-hooks` and the installer's `uninstall` refuse a common directory that is not UTF-8, where `--install-hooks` placed a replaced spelling. The binary refuses an argument that is not UTF-8 at exit 2, where it panicked.
- **The mirrors**: `docs/gate-sdk/SPEC.md` and `docs/delegation-kit/SPEC.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **The `tree_free` column** (delta 1). Producer: the `HOOKS` row's author. Consumers: `hook::run`'s dispatch, and the table's own tests. `check-settings-paths`, the enforcement map and `check-plugin-parity` read the table through `members()` and `owner()` and ignore the column.
- **The tree-free mark** (deltas 1 and 2). Producer: `hook::run`. Consumers: `knobs::load`, `hook::tree_free_path`, `walk::capture_path`, and `stop_liveness::reader_argv`.
- **The decline text** (deltas 2 and 3). Producer: `hook::decline` and the no-tree notice. Consumers: the operator, through the envelope on stdout, and the harness debug log, through stderr.
- **The named refusal** (delta 5). Producers: `walk::cwd`, `walk::toplevel_args`, `hook_launcher::hooks_dir`, `hook_launcher::opted_in`, and `main`. Consumers: every caller of the first two, through their existing error arms; `--install-hooks`, and the installer's `init`, `uninstall`, `doctor` and `prove`, through `hooks_dir` and `opted_in`; and the placed launcher.

The table's readers were found by `grep -rn "HOOKS\|hook::members\|hook::owner" native/src`, the lossy reads by `grep -rn "from_utf8_lossy\|to_string_lossy\|env::args" native/src/walk.rs native/src/emit/hook_launcher.rs native/src/main.rs`, and the launcher readers' callers by `grep -rn "hooks_dir(\|opted_in(" native/src`. The knob layers' file opens are the `load` function in `native/src/knobs/mod.rs`. The no-tree literal's copies were found by `grep -rn "every hook guard in this tree is off"`: the crate's one, and the front-end stubs' two, which the stubs keep for a refused or skipped repository and which this amendment leaves unchanged.

## Existing sections updated

- `gate-sdk/SPEC.md` §The harness-integration arm (deltas 1 to 3), §The crate's crosser (delta 5), §git-hook (delta 5), §install-hooks' honest limits (delta 5), §The knob file's precedence paragraph (delta 2).
- `delegation-kit/SPEC.md` §The turn-end liveness hook, the two Layout bullets and the registration paragraph (delta 4); `delegation-kit/README.md` step 6 (delta 4).
- `installer/SPEC.md` §uninstall, §doctor and §prove (delta 5).
- `native/src/hook/mod.rs`, `native/src/hook/stop_liveness.rs`, `native/src/knobs/mod.rs`, `native/src/walk.rs`, `native/src/emit/hook_launcher.rs`, `native/src/emit/install_hooks.rs`, `native/src/installer/` (`init.rs`, `uninstall.rs`, `doctor.rs`, `prove.rs`, `mod.rs`), `native/src/main.rs`, and the census test in `native/src/emit/mod.rs`, with their tests (deltas 1, 2, 5 and 6).
- `.workflow/release-declarations.md` (delta 6).
- `docs/gate-sdk/SPEC.md`, `docs/delegation-kit/SPEC.md`, the generated mirrors (all deltas).

## Retired spellings

- None — every name this change touches stays, and the no-tree literal is re-phrased rather than renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and the fixture suites of gate-sdk and delegation-kit green. No remote run observes this entry, so its Done move lands before the drain stage, in the build batch that lands it.
