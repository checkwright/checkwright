# SPEC amendment: scratch-ignore

A battery run dirties an adopter's worktree. The runner writes `gate-timings.txt` under `GATE_SDK_TMP_DIR` at the end of every `--run` (`native/src/runner.rs`, `write_timings`), and nothing tells git to ignore that directory in a consumer. This repository ignores it only through its own root `.gitignore` line, `.tmp/`. So the next `init` or `update` refuses on a dirty worktree, since its clean-tree precondition reads `git status --porcelain` and an untracked file counts (`native/src/installer/init.rs`). A habitual `git add -A` commits the file. The Spec Kit extension's `install` and `check` commands run the battery in the adopter's own tree, so a catalog adopter meets this on their first check.

**Measured at authoring (2026-09-27).** In a consumer, a plain `--run` leaves one file in the scratch directory, and it is `gate-timings.txt`. `check-kit-roots-dialect` writes scratch there and removes it, and `check-crate-arms`, which caches stamps there, is `install: never`. The other writers are lifecycle and delegation machinery a consumer may also run: `--enter-stage`'s journals and dispatch marker, `--scratch-run`'s snapshots, and context-kit's `session-context.sh` template writing `session-role`. `init` writes no ignore rule anywhere, and neither `.gitignore` nor `.git/info/exclude` appears in `native/src/installer/`.

**The ruling: the scratch directory ignores itself.** When the directory has no `.gitignore`, it is given one holding the single line `*`. That file ignores itself and everything beside it, so git never shows the directory and the consumer's own files are never edited. The rule covers every writer, including those not yet written, because it is attached to the directory rather than to the timing file. It is also the convention adopters' other tools already follow for their caches. Two alternatives are refused:

- **A line in the consumer's root `.gitignore`.** It edits a file the consumer owns and `init` does not, so `init` must claim one line inside it, `uninstall` must reverse that, and the line goes stale when the consumer re-points `GATE_SDK_TMP_DIR`.
- **Timings written elsewhere, under the git directory.** It moves one writer and leaves every other writer under the same directory dirtying the tree. It would also be the crate's first write into `.git/`, where today it only reads `MERGE_HEAD` through `git rev-parse --git-path`.

## What changes

### (1) One helper makes the scratch directory, and it writes the self-ignore {design-bearing}

**Not yet applied.** The crate gains one helper that creates `GATE_SDK_TMP_DIR` (as `create_dir_all` does) and then, when `<dir>/.gitignore` is absent, writes it holding `*` and a newline. It never rewrites an existing `.gitignore`, so a consumer who tracks one of their own keeps it byte-identical. Every crate writer that creates the directory, or a file directly inside it, calls the helper in place of its own `create_dir_all`. Those writers are found among the call sites `grep -rln 'GATE_SDK_TMP_DIR' native/src` returns — a safe superset at the file level, since a hit's file may also read the knob, declare its default, or use it for an unrelated path, so the grep locates the candidate files and the writing call site within each is read, not assumed; `write_timings` is among the real writers. A writer that creates a subdirectory under the scratch root calls the helper for the root first. The helper's failure to write the ignore file is not a writer's failure. The writer proceeds, since the only cost is the untracked file this unit exists to remove.

In gate-sdk/SPEC.md §Layout and configuration, the `GATE_SDK_TMP_DIR` bullet gains, after its first sentence:

> The directory ignores itself: whatever creates it writes `<dir>/.gitignore` holding `*` when none is there, so no consumer's worktree shows scratch and no consumer file is edited to hide it. An existing `.gitignore` there is the consumer's and is never rewritten.

### (2) `init` makes the scratch directory {mechanical}

**Not yet applied.** `init`, behind the invoke, calls the delta 1 helper once per run, after the placement op writes the config seam and before the commit. So the self-ignore is present before any writer runs in a new consumer, the shell-side writers among them. The file is ignored, so it enters no commit and is not recorded in the manifest's `files`. `uninstall` leaves the scratch directory as it finds it, since everything in it is ignored scratch and nothing there was committed.

In installer/SPEC.md §What init seeds, a paragraph is added after the one beginning **Everything `init` seeds takes one of two disciplines**:

> **`init` also makes the scratch directory, which is not a seed.** gate-sdk's helper creates `GATE_SDK_TMP_DIR` with its self-ignoring `.gitignore` (gate-sdk/SPEC.md §Layout and configuration). Neither discipline above applies, because git never sees the file: it enters no commit and no manifest record, and `uninstall` leaves the directory alone. It is made at install so that a writer running before the first battery, a harness hook among them, finds the directory already ignored.

### (3) The boundary wipe and the session-start sweep spare the self-ignore {mechanical}

**Not yet applied.** In `native/src/emit/enter_stage.rs`, the iteration-boundary wipe spares a scratch-root child named `.gitignore` as a kit invariant beside `.gitkeep`. The scratch directory's own creation there goes through the delta 1 helper.

The boundary wipe is not the only roster naming scratch-root children by name. `context-kit/templates/session-context.sh`'s step 6 scratch sweep runs on every session start, on its own day-horizon schedule rather than the iteration boundary, and its `find "$TMP_DIR" -mindepth 1 ! -name .gitkeep -mmin +1440 -depth -print -delete` spares only `.gitkeep`. Left unchanged, it deletes the self-ignore once it is a day old — nothing else stops it — and the directory re-dirties `git status` until the next writer re-creates the file. Its find expression gains a second `! -name .gitignore` clause beside the existing one.

In lifecycle-kit/SPEC.md §bin/enter-stage.sh, the wipe's sentence "every immediate child named neither `.gitkeep`, nor `LIFECYCLE_KIT_LEAD_JOURNAL_FILE`, nor a `LIFECYCLE_KIT_BOUNDARY_PRESERVE` entry is deleted" becomes "every immediate child named neither `.gitkeep`, nor `.gitignore`, nor `LIFECYCLE_KIT_LEAD_JOURNAL_FILE`, nor a `LIFECYCLE_KIT_BOUNDARY_PRESERVE` entry is deleted". The paragraph beginning "`.gitkeep` is a **kit invariant, not configuration**" gains, after its first sentence:

> `.gitignore` is the second such name. The wipe spares the lead journal and the keep-list, so deleting the self-ignore (gate-sdk/SPEC.md §Layout and configuration) would expose those survivors as untracked until the next writer re-made it.

In context-kit/SPEC.md §The session-context hook, step 6's sentence "reclaim `${GATE_SDK_TMP_DIR:-.tmp}` entries older than a day, depth-first (`-mindepth 1 -depth`) so stray directories are reclaimed too, never touching `.gitkeep`." becomes "reclaim `${GATE_SDK_TMP_DIR:-.tmp}` entries older than a day, depth-first (`-mindepth 1 -depth`) so stray directories are reclaimed too, never touching `.gitkeep` or the scratch directory's self-ignore `.gitignore` (gate-sdk/SPEC.md §Layout and configuration)."

### (4) The consumer smoke asserts a clean worktree after the battery {mechanical}

**Not yet applied.** In `installer/consumer-smoke/run-smoke.sh`, right after each profile's battery run and before the manifest checks, the smoke asserts `git -C "$C" status --porcelain` is empty. Its failure names the untracked or modified paths and the battery as the writer. It is the attribution the later idempotent re-run's porcelain check cannot give, since that check runs after a second `init`.

In installer/SPEC.md §The consumer smoke, the sequence item "The battery must be green" becomes "The battery must be green, and leave the worktree clean: a run's scratch is ignored by the directory itself (gate-sdk/SPEC.md §Layout and configuration), so a path the porcelain shows after it is a writer outside that directory."

## Producers and consumers

- **The self-ignore file.** Producer: the delta 1 helper, reached from `init` on every install and from every crate writer under the scratch root. Consumers: git's ignore machinery, read by `git status --porcelain` in `init`'s and `uninstall`'s clean-tree preconditions and by the delta 4 assertion. No kit gate reads the file.
- **The wipe's spare.** Producer: the delta 3 name test, and the delta 3 sweep edit. Consumer: the self-ignore's survival across the boundary and across a session start, which delta 4 does not reach, since the smoke runs neither a stage entry nor the session-context hook. Its oracle is an `enter_stage` unit test beside the `.gitkeep` one; the sweep's own oracle is read, not tested by this amendment.
- **Roster-holding readers.** The wipe's spare list and the session-start sweep's name filter are the two rosters naming scratch-root children, and delta 3 updates both. `LIFECYCLE_KIT_BOUNDARY_PRESERVE` is unchanged, on the `.gitkeep` precedent: a defaulted array is replaced when a consumer assigns it.
- **Point 5.** No corpus narrows. The ignore removes paths from `git status`, whose readers here red on *finding* a path, so each is monotone under the removal.
- **Point 6.** Delta 1 obliges every crate writer creating the scratch root. They are found among the grep's hits at build, each read down to its call site rather than assumed from the file alone, and each real writer's satisfying value is the helper call.

## Existing sections updated

Roster from `grep -n "GATE_SDK_TMP_DIR" gate-sdk/SPEC.md installer/SPEC.md`, `grep -n "gitkeep" lifecycle-kit/SPEC.md context-kit/SPEC.md` and `grep -n "The battery must be green" installer/SPEC.md`, run 2026-09-27.

- The crate's scratch writers and gate-sdk/SPEC.md §Layout and configuration (delta 1).
- `native/src/installer/init.rs` and installer/SPEC.md §What init seeds (delta 2).
- `native/src/emit/enter_stage.rs`, `context-kit/templates/session-context.sh`, lifecycle-kit/SPEC.md §bin/enter-stage.sh and context-kit/SPEC.md §The session-context hook (delta 3).
- `installer/consumer-smoke/run-smoke.sh` and installer/SPEC.md §The consumer smoke (delta 4).
- The on-site mirrors `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md`, `docs/lifecycle-kit/SPEC.md` and `docs/context-kit/SPEC.md`, regenerated by the command `check-docs-mirror-fresh` prints (all deltas).
- `.workflow/release-declarations.md` gets one Behavior changes bullet. The gate scratch directory, `.tmp` unless `GATE_SDK_TMP_DIR` says otherwise, now carries a `.gitignore` holding `*`, written by `init` and by the battery when absent, so a battery run no longer leaves an untracked file. Nothing to do. An adopter who already added a root ignore line may keep it (deltas 1 and 2).

## Retired spellings

- None — no delta retires a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the self-ignore file, the helper, `init`'s call, the wipe's spare and the session-start sweep's spare.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `consumer-scratch-unignored` moves to Done in its landing commit, at a stage before the drain stage. Its oracles are local: the crate tests and the consumer smoke.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
