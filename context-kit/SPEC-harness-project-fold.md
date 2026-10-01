# SPEC amendment: harness-project-fold

**Merged; this file's deletion rides the entry's Done move**, which waits on the Definition of Done's remote oracle. Delta 1's probe answered the inferred passage below: the Windows harness folds git's drive-lettered toplevel (`D:/a/checkwright/checkwright` became `D--a-checkwright-checkwright` under pwsh and Git Bash alike) and roots it under `USERPROFILE`, even where `HOME` is set. The session-id readers did not move with the memory derivation; their `HOME` read on Windows is filed to the gap inbox.

**The memory-dir derivation folds a repository root the way the harness folds it, through the crate's one encoder.** `check-memory-off`'s default derivation (`memory_dir_default`, `native/src/gates/memory_off.rs`) and its two shell twins fold only `/` and `.` to `-`. The twins are step 5 of `scripts/session-context.sh` and of `context-kit/templates/session-context.sh`, each a `tr '/.' '-'` over `pwd -P`. On Windows the two substrates fold one checkout to two names: the crate reads git's drive-lettered toplevel and the shell reads MSYS's `/c/…` spelling. A third fold already exists. `sessions::slug` (`native/src/sessions.rs`) is the sessions-dir slug lifecycle-kit's session-id derivation uses (lifecycle-kit/SPEC.md §bin/session-id.sh), and it maps every non-alphanumeric character.

**Measured at authoring**, with the installed harness (version 2.1.286), each unauthenticated `claude -p hi` run under a scratch `HOME`. Every run exits 1 with `Not logged in`, after creating its project dir:

- A launch in `.tmp/hprobe/my_proj dir.x` created a project dir whose name ends `--tmp-hprobe-my-proj-dir-x`. The `_` and the space fold to `-` as `/` and `.` do, so the tree's two-character fold is wrong on Linux for any root carrying another non-alphanumeric character.
- With `CLAUDE_CONFIG_DIR` set, the project dir was created under `$CLAUDE_CONFIG_DIR/projects/`, not under `$HOME/.claude/projects/`. The memory derivation reads `$HOME/.claude` only.
- A launch in a subdirectory, and one in a linked worktree, each keyed the created dir on the launch directory.

**Read at authoring, in the installed harness's bundled source:**

- The folding function is `e.replace(/[^a-zA-Z0-9]/g, "-")`. It is a regular expression without the `u` flag, so it folds per UTF-16 code unit and a non-BMP character becomes `--`.
- A result longer than 200 units is cut to its first 200, then `-` and a suffix is appended. The suffix is `Math.abs(h).toString(36)`, where `h` is the 32-bit fold `h = (h << 5) - h + c | 0` over the raw path's UTF-16 units.
- The memory dir is `<config home>/projects/<fold>/memory`, the config home being `CLAUDE_CONFIG_DIR` when set, else `~/.claude`. The fold's input is a canonical working-copy root, which a linked worktree may map elsewhere. That last point is filed to the gap inbox rather than taken here: an unauthenticated run writes no memory dir, so nothing observable settles it.

**Inferred, cannot run before build:** the Windows harness's input spelling for a checkout launched at its root (drive-letter case, separator) and the home it resolves when `HOME` and `USERPROFILE` differ — delta 1's probe is the run that observes it, and build lands that probe.

## What changes

### (1) A one-shot Windows probe observes the harness's spelling

{mechanical} `.github/workflows/gates.yml`'s `crate-tests-windows` job gains one step with `continue-on-error: true`, conditioned on `matrix.target == 'x86_64-pc-windows-msvc'` as the job's Windows-only steps are. It installs `@anthropic-ai/claude-code` at an exact version into `$RUNNER_TEMP` through npm, then runs `claude -p hi` from the checkout root twice, once under `pwsh` and once under Git Bash. Each run sets `HOME` and `USERPROFILE` to two distinct scratch dirs, unsets `CLAUDE_CONFIG_DIR`, and passes no credential. It prints:

- the project-dir names each run created, and which of the two homes holds them;
- `git rev-parse --show-toplevel`, Git Bash's `pwd -P`, and pwsh's `(Get-Location).Path`.

The step lands in its own commit and rides the iteration's mid-iteration push (the entry's push need). Build reads the run with `gh run view <id> --log` and records the printed spellings in its journal before delta 3 is written. Delta 6 deletes the step.

### (2) `sessions::slug` becomes the harness's encoder exactly

{design-bearing} `slug` in `native/src/sessions.rs` folds the path's UTF-16 code units, mapping each unit outside ASCII `[A-Za-z0-9]` to `-`. A result longer than 200 units becomes its first 200, then `-`, then the base-36 absolute value of the 32-bit hash above over the raw path's UTF-16 units. Below the cap the function is byte-identical to today's for every ASCII path. It changes only a non-BMP character, which becomes two dashes, and a path past the cap.

Its readers are the session-id derivation (lifecycle-kit's sessions dir) and, after delta 3, the memory derivation. The roster comes from `git grep -n 'slug(' -- native/src`. The constant 200 and the hash are named once, beside `slug`, as the harness's own values. They are not knobs: a value the harness owns is not the consumer's to set (`CONTEXT_KIT_MEMORY_DIRS` remains the consumer's override when the layout moves).

### (3) The memory derivation reads the shared encoder and config home

{design-bearing} `memory_dir_default` returns `<config home>/projects/<slug(input)>/memory`:

- `<config home>` is `sessions::config_home(CLAUDE_CONFIG_DIR, home)`, so a set `CLAUDE_CONFIG_DIR` is honoured.
- `<input>` is the repository toplevel in the spelling delta 1 observed the harness folding. On Linux and macOS that spelling is `walk::toplevel_opt`'s, which is what the harness folded on this host.
- On Windows, if the probe shows the harness folding the drive-lettered root git reports, the input is `walk::toplevel_opt` unchanged. If it shows another spelling (a different drive-letter case, say), the input is converted to it in this function and nowhere else.
- `home` is `HOME`. If the probe shows the Windows harness reading `USERPROFILE` where the two differ, `home` is `USERPROFILE` on Windows. Either way an unset home stays the fail-closed exit 2 §check-memory-off states, and it binds only when `CLAUDE_CONFIG_DIR` is unset.

The `spec:` comment at `memory_dir_default` stating the `/`-and-`.` fold is rewritten to name the shared encoder, and the `spec:` comments recording the open Windows question at both shell twins are removed with the fold they qualified. `sessions::slug` and `resolve_memory_dirs` become `pub(crate)` for the cross-module reads this delta and delta 4 add. Where the Windows probe shows `USERPROFILE` read, the session-id arm's `Inputs.home` is the caller's and the build records whether the two readers move together.

### (4) The shell twins read the derivation from the binary

{mechanical} A new non-gate arm, `--emit memory-dirs` (`--emit-memory-dirs`, an `Arm::Emit` row in `native/src/emit/mod.rs` declaring `CONTEXT_KIT_MEMORY_DIRS`), prints `resolve_memory_dirs()`'s answer one directory per line. That answer is the knob's globs expanded, else the derived dir. Step 5 of `scripts/session-context.sh` and of `context-kit/templates/session-context.sh` reads its scan set from this arm through `$NATIVE_BIN`, under its own binary guard (`-x "$NATIVE_BIN"`, as steps 1 and 3 carry, since step 2's guard is combined with a dirty-component test and does not bind step 5). It no longer folds anything itself. The step is silent where the binary is absent, as steps 1 to 3 already are. A consumer's knob-file value now reaches the step, which read only the environment before.

### (5) Tests pin the encoder against the observed spellings

{mechanical} `native/src/sessions.rs` gains unit tests for `slug`, each against its observed spelling:

- a Linux root of the measured shape, `/srv/my_proj dir.x` → `-srv-my-proj-dir-x`;
- the Windows spelling delta 1 recorded → the name delta 1 saw the harness create;
- a non-BMP character → two dashes;
- a 260-unit path → its first 200 folded units, `-`, and a suffix. The expected suffix is computed at build by evaluating the harness's own expression under a JS runtime, and is pinned as a literal with that command named in the test's comment.

`native/src/gates/memory_off.rs` gains a test that a set `CLAUDE_CONFIG_DIR` roots the derived dir. `context-kit/gate-tests/check-memory-off.test.sh` gains a case: under a scratch `HOME` with a file in the derived memory dir, the gate reds, and `--emit memory-dirs` prints that same dir. This is the cross-substrate agreement, with both substrates reading one derivation.

### (6) The probe leaves and the fact stays

{mechanical} The commit landing deltas 3 to 5 deletes delta 1's step. In context-kit/SPEC.md §Layout and configuration, the `CONTEXT_KIT_MEMORY_DIRS` bullet's sentence "the current project's dir under the operator's home, `$HOME/.claude/projects/<slug>/memory`, where `<slug>` is the project's absolute path with every `/` and `.` folded to `-` (the harness's own encoding)" becomes:

> the current project's dir, `<config home>/projects/<slug>/memory`, where the config home is `CLAUDE_CONFIG_DIR` or `~/.claude` and `<slug>` is the harness's own encoding of the repository root: every UTF-16 unit outside `[A-Za-z0-9]` folded to `-`, cut past 200 units with a hash suffix (lifecycle-kit/SPEC.md §bin/session-id.sh owns the encoder). On Windows the root is folded in `<the spelling delta 1 observed>`. The witness for each fact is an unauthenticated harness run, which creates its project dir before refusing.

**Not yet applied.** The Windows clause is written from delta 1's observation.

### (7) The owning sections state the encoder, the arm and step 5

{mechanical} **Not yet applied.**

- drift-kit/SPEC.md §`DRIFT_KIT_SESSIONS_DIR`: the `<cwd-slug>` sentence ("the working directory with every non-alphanumeric replaced by `-`") becomes the encoder's statement, citing lifecycle-kit/SPEC.md §bin/session-id.sh.
- lifecycle-kit/SPEC.md §bin/session-id.sh: the clause "and the cwd slug maps every non-alphanumeric character of the cwd to `-`" becomes "and the cwd slug is the harness's encoder: every UTF-16 unit outside `[A-Za-z0-9]` maps to `-`, and a slug past 200 units is cut there and suffixed with `-` and the base-36 absolute value of the path's 32-bit string hash".
- context-kit/SPEC.md §check-memory-off: the fail-closed sentence's "or a `HOME` it cannot read when the default derivation is the one in play" becomes "or no home it can read when the default derivation is the one in play and `CLAUDE_CONFIG_DIR` is unset". A paragraph is added after *It has one arm, and the knobs are it*: "**`--emit memory-dirs` prints the scan set**, one directory per line, from the same resolution the gate scans, so the session-context hook's step 5 folds nothing of its own."
- context-kit/SPEC.md §The session-context hook (template), step 5: "one warning line when the harness memory dir (`CONTEXT_KIT_MEMORY_DIRS`) holds content" becomes "one warning line when a dir `--emit memory-dirs` prints holds content, under its own binary guard".
- context-kit/SPEC.md §Layout and configuration, the paragraph after the knob list: `CONTEXT_KIT_MEMORY_DIRS` leaves the list of names the template reads from its own environment, since step 5 reads it through the arm.

### (8) The site mirrors follow

{mechanical} `docs/context-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md` and `docs/drift-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, in the commit landing deltas 6 and 7.

## Producers and consumers

- **The probe step (delta 1).** Producer: the `crate-tests-windows` job on a push. Consumer: build, reading the run log. It writes nothing to the tree and lives from its landing commit to delta 6's.
- **The encoder (delta 2).** Producer: `sessions::slug`. Consumers:
  - `sessions::sessions_dir`, behind `--emit-session-id` and `--enter-stage`'s stamp id, and drift-kit's meter through the same module;
  - `memory_dir_default`, behind `check-memory-off` and the new arm.

  The output is byte-identical for every ASCII path at or below 200 units, the population every recorded stamp was derived under, so no stamp id moves.
- **The new arm (delta 4).** Producer: `--emit memory-dirs`, run by the hook's step 5 at every session start where the binary is present. Consumer: step 5's loop. Its declared knob is `CONTEXT_KIT_MEMORY_DIRS`, and the knob-file derivation and `check-reads-couples` read that roster off the row.
- **Readers whose verdict moves, and their red conditions.**
  - `check-memory-off` reds on a regular file in a scanned dir. The derived dir moves from a name the harness never creates to the one it does, wherever the root carries a character beyond `/` and `.` or `CLAUDE_CONFIG_DIR` is set. That widens what it can find, which is the defect's fix.
  - The gate is clean on an absent dir, so on CI and on this host, whose root folds the same under both rules, its verdict is unchanged.
  - The session-id arm exits 2 on an empty sessions dir. Its slug is unchanged for every root it has met, so its verdict is too.
- **No new knob.** The new arm's name is declared on its row and §check-memory-off states it (delta 7).

## Existing sections updated

Roster produced by `git grep -n "tr '/.' '-'\|memory_dir_default\|fn slug\|projects/<slug>\|cwd slug\|cwd-slug\|CONTEXT_KIT_MEMORY_DIRS" -- . ':!TASK-QUEUE.md'`, with each hit read.

- `.github/workflows/gates.yml` — `crate-tests-windows`, the probe step (deltas 1 and 6).
- `native/src/sessions.rs` — `slug` and its tests (deltas 2 and 5).
- `native/src/gates/memory_off.rs` — `memory_dir_default` and its test (deltas 3 and 5).
- `native/src/emit/mod.rs` — the `--emit-memory-dirs` row (delta 4).
- `scripts/session-context.sh`, `context-kit/templates/session-context.sh` — step 5 (delta 4).
- `context-kit/gate-tests/check-memory-off.test.sh` — the agreement case (delta 5).
- `context-kit/SPEC.md` — §Layout and configuration (deltas 6 and 7); §check-memory-off and §The session-context hook (template), step 5 (delta 7).
- `lifecycle-kit/SPEC.md` — §bin/session-id.sh (delta 7).
- `drift-kit/SPEC.md`, `docs/drift-kit/SPEC.md` — the `DRIFT_KIT_SESSIONS_DIR` slug sentence and its mirror (delta 7; the mirror regenerated in delta 8).
- `docs/context-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md` — the regenerated mirrors (delta 8).

## Retired spellings

<!-- retired-spelling-exempt: the two-character fold survives as history in queue prose and past survey text, which are evidence about the past rather than live derivations -->
- `tr '/.' '-'` — the shell twins' fold, replaced by the arm's derivation (delta 4).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Remote oracle** — the entry moves on the closing push's `crate-tests-windows` run green over delta 5's Windows case, by build's remote-oracle rule, never in the merge commit.
