# SPEC amendment: user-install

Pairs `hook-only-user-install`, and carries `installer-crate-test-gaps` as its delta 5, since both land on the installer's verbs and crate tests. An adopter running its own work engine wants one session-hook arm and nothing else: the gate binary placed for the user rather than for a repository, the harness settings entry for that arm printed, and the arm's scratch and log outside any repository. Today the only route is to vendor a kit into a tree the adopter owns, or to place the binary by hand and write the knob file and the settings entry from the SPEC. This amendment adds a per-user install of a tree-free arm (gate-sdk/SPEC.md §The harness-integration arm), the `--hook` token that points the arm at its knob file, and the crate-test cases the commit path was missing.

**What the tree does today**, read from `installer/SPEC.md` §The verbs, §init, §The personal install, §Placement, §update, §uninstall and §The manifest, from `native/src/installer/mod.rs`, `native/src/hook/mod.rs` and `native/src/hook/binding.rs`, and probed with the release binary from a scratch directory outside any repository:

- **No verb takes `--user`.** `init --user subagent-stop-liveness --dry-run` and `update --user` print `checkwright init: unknown argument: --user` and exit 2. `uninstall --user` prints `checkwright uninstall: unknown argument: --user` and exits 2.
- **Every install is a repository's.** `init` requires a git work tree, a clean tree and a committer identity, vendors kits, and records `checkwright.lock` with `files` hashed by `git hash-object`. `--personal` still installs into one clone. §Placement writes the binary into the gates directory and its knob seam beside it.
- **`--hook` takes no knob-file token.** `--hook subagent-stop-liveness --knobs <absolute file>` from outside any repository reports `not inside a git repository` on stderr and emits the no-tree decline envelope, exit 0. The trailing token goes unread (gate-sdk/SPEC-hook-observe.md delta 1 makes it a grammar refusal).
- **The event a member registers under is not in the crate's tables.** `HOOKS` rows carry the member, its function, its knob slice and its owning kit. `SubagentStop` appears only in `binding::UNREAD`, the spellings no reader reads. The registration's shape, `command` plus `args`, is delegation-kit/SPEC.md's wiring prose and the `WIRING_ARGS` constant.

## What changes

### (1) The `--knobs` hook token {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): `--knobs <absolute file>` at `--hook` makes the arm tree-free}

**The grammar.** gate-sdk/SPEC-hook-observe.md delta 1's grammar gains one optional token, last: `--hook <member> [--observe] [--binding <name>] [--knobs <absolute file>]`, each at most once and in that order. The one parser that reads the trailing tokens reads it, so the arm and the registration readers agree. **Not yet applied.**

**What it does.** `--knobs` puts the arm under the tree-free mark (gate-sdk/SPEC.md §The harness-integration arm) whether or not the working directory is in a repository: no toplevel is entered and no default knob locator is read. The named file is the **only file layer**. Resolution is the environment, then that file, then the kit default. `<KIT>_KNOB_FILE` and the overlay locators are not read under it. Each line is routed to the kit whose prefix its name carries, and that kit's validator refuses a name it does not declare, as in any knob file (gate-sdk/SPEC.md §The knob file). Two compositions follow from the neighbouring amendments and add no behavior. Under `--observe`, the observed run's child argv carries `--knobs <file>` (gate-sdk/SPEC-hook-observe.md delta 1), so the child reads the layer its parent did. A `--binding` call under `--knobs` takes the bound-call decline of gate-sdk/SPEC.md §The harness-integration arm, since no binding is yet shown to run tree-free. The member's tree-free path rules still apply: `GATE_SDK_TMP_DIR` and `DELEGATION_KIT_STOP_LOG` must resolve absolute, or the arm declines. **Not yet applied.**

**Grounds.** A per-user install has no repository to locate a knob file from, and the environment is the harness's to set, not the adopter's. One absolute path in the registration's own argv makes the registration self-describing, and keeps a project's tracked knobs from leaking into a user-scope arm that happens to run inside that project.

**Refusals.**

- `--knobs` on a member whose tree-free column is false is a grammar refusal: exit 2, naming the token and the tree-free members. A tree-bound member reads repository state, and a file layer cannot supply it.
- A relative, missing or unreadable file, a malformed line, or a name no kit declares **declines** through `hook::decline` at exit 0, naming `--knobs`, the file and the line. It is not exit 2, because an exit 2 on a turn-end event refuses every turn end of every session the user runs, which is the harm the arm exists to prevent, not a misconfiguration to report by causing it.

**The registration readers.** `Registration::Member` carries the knob path where one is named. `check-settings-paths` reds a relative `--knobs` value statically, `— --knobs names a relative path: <value>`, with a `help:` line naming the absolute form. It also reds `--knobs` on a tree-bound member as it reds any `Malformed` registration. Its fixture pair gains both in `bad/` and an absolute `--knobs` registration in `good/`. **Not yet applied.**

### (2) The per-user directories {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): the binary and the user manifest in the per-user data dir, XDG_DATA_HOME/checkwright, default ~/.local/share/checkwright, %LOCALAPPDATA%\checkwright on Windows; scratch and log in the per-user state dir}

**The data dir** holds what the install owns:

- POSIX: `$XDG_DATA_HOME/checkwright`, else `$HOME/.local/share/checkwright`. A relative `XDG_DATA_HOME` is ignored, as the XDG base-directory specification requires. macOS takes the same rule. It is a POSIX host to every other part of the installer, and one rule keeps one smoke arm.
- Windows: `%LOCALAPPDATA%\checkwright`.

**The state dir** holds what the arm writes:

- POSIX: `$XDG_STATE_HOME/checkwright`, else `$HOME/.local/state/checkwright`, with the same relative-value rule.
- Windows: `%LOCALAPPDATA%\checkwright\state`. Windows has no separate state root, and a subdirectory keeps the arm's writes out of the directory the binary is placed in.

An unset `HOME` on POSIX or `LOCALAPPDATA` on Windows is refused at exit 2, naming the variable. A resolved path that is not UTF-8, or that carries a tab, is refused at exit 2, naming it: the first cannot be written into the knob file or the printed JSON, and the second is a value the knob grammar refuses. **Not yet applied.**

**The data dir's layout:**

```
<data>/bin/checkwright-gates[.exe]     the placed binary
<data>/knobs/<member>.knobs            the arm's knob file, the --knobs operand
<data>/user.lock                       the user manifest
```

**The knob file** is written with two scalars, `GATE_SDK_TMP_DIR = <state>/tmp` and `DELEGATION_KIT_STOP_LOG = <state>/<member>.log`. The observe log derives under the scratch dir (gate-sdk/SPEC-hook-observe.md delta 2), so a trial needs no third line. It is a file the adopter edits, so it is claimed like §Placement's seam: written on a first install, rewritten when its hash matches the one `user.lock` records, and left alone and reported when it differs, unless `--force` is given. **The scratch dir is where the arm reads launch records** (delegation-kit/SPEC.md §The turn-end liveness hook), so an adopter whose engine writes launch records must point it at `<state>/tmp`, or edit `GATE_SDK_TMP_DIR` to the directory the engine writes. The printed block says so (delta 3). **Not yet applied.**

**The user manifest**, `user.lock`, is JSON with schema `checkwright-user-lock v1` and the keys `version`, `commit`, `artifact` (`target`, `digest`), `hooks` (the installed members), `state` (the absolute state dir) and `files`, keyed by path relative to the data dir. Its hashes are SHA-256, computed in-process by the crate's `sha256` module, not `git hash-object`: a user install runs where git may be absent, and git is the floor only for a repository install. It shares no reader with `checkwright.lock`, and no verb reads one where it expects the other. **Not yet applied.**

### (3) `init --user <arm>` {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): verbs `init --user <arm>`, `update --user`, `uninstall --user`; the settings entry printed, never written; the harness-version floor probed at build, inferred until then}

**The operand.** `<arm>` is a member whose `HOOKS` tree-free column is true, a set derived from the table and never listed. A missing operand, an unknown member or a tree-bound one is refused at exit 2, naming the tree-free members.

**No repository precondition.** `init --user` runs from any directory, inside a repository or not, and never reads or writes one: no work tree, clean-tree, identity or commit precondition. It runs no `doctor` precondition either, because it vendors no battery and the arm spawns only its own image. It refuses `--profile`, `--recipe`, `--no-recipe`, `--personal`, `--no-commit` and the selection flags at exit 2, naming the flag. It takes `--force` and `--dry-run` with their `init` meanings. **Not yet applied.**

**What it does**, in order:

1. Resolves the two dirs (delta 2) and creates the data dir, the state dir and `<state>/tmp`.
2. Places the payload artifact the bootstrap verified at `<data>/bin`, in-process, with §Placement's skip-rewrite test against `user.lock`'s `artifact` key and the on-disk digest. The write goes to a sibling temporary file renamed over the target. On Windows a rename over an image in use fails, and the verb refuses at exit 1, naming the file and telling the user to end the sessions running it and re-run.
3. Writes the member's knob file by delta 2's claim.
4. Writes `user.lock`, the member joining `hooks`.
5. Prints the report.

A bare re-run leaves the data dir byte-identical. **Not yet applied.**

**The report** is one `INIT:` line naming the data dir, then a `settings:` block. The block's grammar is a banner line then indented lines, so the consumer smoke parses it (delta 6). It carries:

- A header line naming the harness the binding targets, its user-scope settings file, and the harness version from which a registration's `args` array is honoured. The version is probed at build against the harness's own documentation and release notes. Until that probe it is printed as `inferred`.
- The JSON fragment for the member's event: the event key, a `hooks` array holding one `command` entry whose `command` is the absolute binary and whose `args` are `--hook`, the member, `--knobs` and the absolute knob file. Paths are JSON-escaped, which is the whole of the Windows spelling, since `args` passes no shell. The fragment is checked against a live harness at build, and printed as `inferred` until then.
- A line naming `--observe` as the trial token, inserted after the member, and the observe log's path.
- A line naming the scratch dir the arm reads launch records from (delta 2).

The settings entry is printed and never written. A settings write is a permission-surface write, the user's own act (delegation-kit/SPEC.md §The turn-end liveness hook). **Not yet applied.**

**The member's event.** Each `HOOKS` row gains an `event` column, a `binding.rs` constant, and `init --user` reads it. `SubagentStop` moves from `binding::UNREAD` to a named constant. A unit test holds every tree-free row to carry an event the binding names. The column is new rather than derived because no crate table records a member's event, and printing a registration needs one. **Not yet applied.**

**The flag.** `FLAGS` gains `("--user", &["init", "update", "uninstall"])`, and `installer/README.md`'s flag table gains its rows, which check-front-door-verbs invariant A holds equal. Each verb's usage line names it. A front-door page advertising it is admitted as pending until the release carries it (§The front door's verbs). **Not yet applied.**

### (4) `update --user` and `uninstall --user` {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): verbs `init --user <arm>`, `update --user`, `uninstall --user`}

**`update --user`** takes no operand. It requires `user.lock`, and with none it refuses at exit 1, naming `init --user`. Otherwise it applies §update's downgrade refusal against `user.lock`'s `version`, then runs `init --user` for each member in `hooks`, so it places the new binary once and reprints each member's settings block. **Not yet applied.**

**`uninstall --user`** takes no operand and requires `user.lock` in the same way. It:

- removes the binary whatever its hash, because the install owns that path and an edited binary is no adopter's work;
- removes each knob file whose hash matches `user.lock`, and keeps and names one that differs unless `--force` is given;
- deletes `user.lock`, or rewrites it to name only the kept files;
- leaves the state dir, its logs and its launch records, which are the adopter's evidence, not the install's;
- removes empty directories it created, and names every path that remains;
- prints a reminder to remove the settings entry, naming the settings file and the event.

**Not yet applied.**

**Honest limits.**

- A harness still carrying the entry after `uninstall --user` fails to spawn the hook. That this fails open, without blocking the turn end, is inferred from the harness's documented behavior for a hook that cannot start, and is not probed.
- A user install gets no update notice (§The update notice reads a repository's knob seam), and `doctor` and `diff` read no user install.
- Two concurrent writers to one data dir are not serialised. The verbs are run by a person, once.
- `personal-one-kit-hooks` stays its own entry. A user install serves an arm outside any repository, and that entry serves a kit's hooks in one clone.

### (5) The installer's crate-test gaps {mechanical}

The deliverable is `installer-crate-test-gaps`' own. Each **Not yet applied.**

- `native/tests/installer_commit.rs`' `git()` helper scrubs `GIT_*` from the inherited environment as `verb()` does, so a parent's `GIT_DIR` or identity variables cannot steer a fixture's setup. `the_identity_precondition_refuses_what_gits_strict_read_refuses` in `native/src/installer/mod.rs` scrubs `GIT_AUTHOR_*` and `GIT_COMMITTER_*`.
- One case per promise the suite does not exercise, each shown able to fail by breaking its promise once before it lands:
  - the author identity alone, and the committer identity alone;
  - each waiver, `--no-commit`, `--personal` and `--dry-run`, running with no identity;
  - an identity-less run with nothing to change;
  - the identity refusal reached through `update`;
  - a `git var` read that cannot spawn;
  - the restore line on every commit failure path, not only the failed commit and the unset value;
  - gate-sdk/SPEC.md §install-hooks' unpaired-surrogate escape on Windows. This case rides the Windows legs of the closing push, since no local leg runs Windows.

### (6) Cases, the release declaration and the mirrors {mechanical}

Each **Not yet applied.**

- **The arm's cases** (`hook/mod.rs` tests and a gate-sdk gate test):
  - `--knobs` with an absolute file, inside and outside a repository, reading only that file. A project knob file present in the working tree is not read.
  - The decline cases: relative, missing and unreadable files, a malformed line, an undeclared name.
  - The grammar refusal on a tree-bound member, and `--knobs` before `--observe`.
  - The parser's `Member` carrying the path; the `check-settings-paths` fixture pair (delta 1).
- **The installer's cases** (crate tests):
  - dir resolution, including a relative XDG value, unset `HOME` and a tab in the path;
  - the operand and flag refusals;
  - the byte-identical re-run;
  - the kept edited knob file, and `--force` over it;
  - `update --user` and `uninstall --user` with no `user.lock`;
  - the downgrade refusal;
  - uninstall's residue report.
- **The consumer smoke** gains a user-install arm on every install-smoke leg, the PowerShell leg included. It points `XDG_DATA_HOME` and `XDG_STATE_HOME`, or `LOCALAPPDATA`, at scratch dirs and runs `init --user subagent-stop-liveness`. From the `settings:` block it parses the command and args, and runs them from a directory outside any repository on a payload with no live task, expecting exit 0 and nothing on stdout. Then it runs `update --user` and expects an identical data dir, then `uninstall --user`, expecting the data dir gone and the state dir kept. The Windows legs run on the closing push.
- **The release declaration** (`.workflow/release-declarations.md`), under `## Behavior changes`: the `--user` form of `init`, `update` and `uninstall`; `--hook` accepting `--knobs <absolute file>`; and the settings block `init --user` prints.
- **The regenerated surfaces**: the mirrors `docs/installer/SPEC.md`, `docs/installer/README.md`, `docs/gate-sdk/SPEC.md` and `docs/context-kit/SPEC.md`, each by the command docs/generated-projections.md rosters for it.

## Producers and consumers

- **The `--knobs` token** (delta 1). Producers: the settings block `init --user` prints, and a consumer's hand-written registration. Consumers: `hook::run`, the knob resolver under the tree-free mark, and the registration parser with its readers.
- **The data and state dirs** (delta 2). Producer: one resolver in `native/src/installer/`. Consumers: the three `--user` verbs and the consumer smoke's arm.
- **`user.lock`** (delta 2). Producer: `init --user`. Consumers: `init --user`'s claim and skip-rewrite test, `update --user` and `uninstall --user`.
- **The member's knob file** (delta 2). Producer: `init --user`. Consumers: the arm, through `--knobs`, and the adopter.
- **The `settings:` block** (delta 3). Producer: `init --user` and `update --user`. Consumers: the adopter, and the consumer smoke's parser.
- **The `HOOKS` `event` column** (delta 3). Producer: the table. Consumers: `init --user` and its unit test.
- **The `--user` flag** (delta 3). Producer: `FLAGS`. Consumers: the three verbs' parsers, the usage lines, and check-front-door-verbs through `installer/README.md`'s flag table.

The flag table's readers were found by `grep -rn "FLAGS" native/src/installer native/src/gates`, the registration readers by `grep -rln "Registration::\|hook::registration" native/src`, the knob locators the token bypasses by `grep -rn "fn knob_scalar\|_KNOB_FILE" native/src/walk.rs native/src/registry.rs`, and the event spellings by `grep -rn "SubagentStop" native/src`.

## Existing sections updated

- `installer/SPEC.md` §The verbs, §init (a new subsection, **The user install**, beside §The personal install), §update, §uninstall, §The manifest (the user manifest beside `checkwright.lock`), §The consumer smoke, and `installer/README.md`'s verb prose and flag table (deltas 2 to 4 and 6).
- `gate-sdk/SPEC.md` §The harness-integration arm (the grammar and the `--knobs` resolution) and §The knob file (the `--knobs` layer in the precedence) (delta 1).
- `context-kit/SPEC.md` §check-settings-paths (delta 1).
- `native/src/hook/mod.rs`, `native/src/hook/binding.rs`, the knob resolver under the tree-free mark, `native/src/gates/settings_paths.rs`, `native/src/installer/mod.rs` and a new user-install module beside `init.rs`, `native/src/emit/installer_smoke/`, and `native/tests/installer_commit.rs`, with their tests and the `check-settings-paths` fixture pair (deltas 1 to 6).
- `.workflow/release-declarations.md`, and the regenerated surfaces delta 6 lists.

## Retired spellings

- None — every change adds a token, a flag, a column or a file, and no existing name is renamed.

## Definition of Done

- [ ] **Causal completeness** — every delta's **Not yet applied.** is applied, and each user-facing mark's direction is visible in the merged section it lands in.
- [ ] **Instruction surfaces** — `installer/README.md`'s verb prose and flag table, the usage lines, and the printed `settings:` block say what the merged sections say. The block's two `inferred` items are probed and either confirmed or corrected.
- [ ] **Merged with no information lost** — every delta's content sits in the section **Existing sections updated** names, including each honest limit.
- [ ] **Amendment deleted** — `ls installer/SPEC-*.md` lists no `SPEC-user-install.md`.
- [ ] **Removals propagated** — `SubagentStop` leaves `binding::UNREAD` in the commit that names it, and no reader keeps the old trailing-token behavior.
- [ ] **Gaps filed** — a scope cut found in build (`diff --user`, a user-install update notice) is filed with `--emit file-gap`, costed, never flagged and skipped.
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery (`bash gate-sdk/bin/run-gates.sh --run`), the crate tests, the gate-sdk and context-kit fixture suites, `scripts/gate-tests`, and `bash gate-sdk/bin/run-gates.sh --installer-smoke`, all green. No remote run observes this entry before close. The Windows legs, the PowerShell smoke arm and delta 5's surrogate case included, ride the closing push.
