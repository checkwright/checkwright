# SPEC amendment: run-scratch-root

Pairs `run-scratch-system-temp`. The gate binary's runners root their scratch at the system temp dir. That puts each battery member's private `TMPDIR` and capture there, with every `mktemp` and `std::env::temp_dir()` a member makes under it, and binds no `TMPDIR` at all for the fixture runner's children. On a host whose temp dir is a tmpfs that scratch lands in memory, against the repo-local intent `GATE_SDK_TMP_DIR` carries. This amendment roots the runners' scratch under `GATE_SDK_TMP_DIR` and binds each child's temporaries there. The directory a child makes stays outside any repository as git sees it, because the run scratch heads the child's git ceiling. The crate's own unit tests keep the system temp dir, since some of them need a directory outside any repository by every reading, not git's alone.

**What the tree does today**, read at spec:

- `native/src/runner.rs`, the `--run` arm, makes `std::env::temp_dir()/checkwright-run.<pid>`, gives each member a private `TMPDIR` under it through `proc::dispatch`, and removes it on the way out.
- `native/src/emit/git_hook.rs`, the hook arm, makes `std::env::temp_dir()/checkwright-git-hook.<pid>` the same way.
- `native/src/emit/run_gate_tests.rs`, the `--run-gate-tests` arm, absolutizes `GATE_SDK_TMP_DIR` for each case invocation (§run-gate-tests) and binds no `TMPDIR`. A bespoke `*.test.sh` therefore makes its `mktemp -d` sandboxes and `lib/test-hermetic.sh`'s empty knob files at `${TMPDIR:-/tmp}`, and heads `GIT_CEILING_DIRECTORIES` with that dir.
- The crate's unit tests use `std::env::temp_dir()` throughout. `walk.rs`'s toplevel tests clear `GIT_CEILING_DIRECTORIES` to test discovery itself, so their trees must lie outside any repository on disk.
- Measured at scope: `/tmp` on the build host is a 7.6G tmpfs holding 784K at rest and no residue of the binary's, so the scratch is transient and the cost is paid only where a run meets a small or full tmpfs.

**Who reads the tmp dir as a set**, probed so a run scratch inside it disturbs no reader. `hook/stop_liveness.rs` takes only the dir's non-directory `*.run` entries. evidence-kit/SPEC.md §check-producer-liveness reads a directory's own `*.run` files, and guard-kit rule `git_mutation_under_producer` reads its records the same way. A record a member writes under its private `TMPDIR` is therefore nested and read by none of them. `check-crate-arms`' cache keeps its keyed filename in the tmp dir itself, which this amendment does not move.

## What changes

### (1) The run scratch roots under `GATE_SDK_TMP_DIR` {design-bearing} {user-facing: run-scratch-system-temp's body, "against the repo-local scratch intent behind `GATE_SDK_TMP_DIR` (operator direction 2026-10-20, lead session, not a ruling)"; this design, the crate unit tests' exception included, operator direction of 2026-10-20, lead-relayed}

Replacement for `gate-sdk/SPEC.md` §run-gates, the concurrency contract's bullet **Per-gate scratch isolation, split by what the directory is for**. **Not yet applied.**

> - **Per-gate scratch isolation, split by what the directory is for**. The run's own scratch, the **run scratch**, is `checkwright-run.<pid>` under `GATE_SDK_TMP_DIR`, absolutized at the invoker's root, and the tmp dir is made with its self-ignoring `.gitignore` where it is absent (§Layout and configuration). Each child gets a private `TMPDIR` under the run scratch, where a member's *anonymous* temporaries land, and the run scratch at the head of its `GIT_CEILING_DIRECTORIES`, joined to any inherited value by the host's list separator. So a directory a member makes there is outside any repository as git and the crate's discovery read it, as one under the system temp dir was. The scratch is **absolute**, because a child's working directory is its own and a relative `TMPDIR` silently resolves somewhere else inside one. It is **repo-local** because a member's temporaries are the run's scratch, and a system temp dir that is a tmpfs would hold them in memory. `GATE_SDK_TMP_DIR` itself stays **shared**, because it is the home of a declared, content-keyed cache whose whole value is surviving the run: `check-crate-arms` writes its `crate-arms-<hash>.green` there and reads it on the next battery. The rule a later member is measured against: **an anonymous temporary is private; a declared cache is shared and its filename must carry its key**. The run scratch is removed on the way out, so a battery leaves the tmp dir's file set as it found it, the tmp dir itself excepted where the run made it. **Honest limits.** On Windows the crate's temp lookup reads `TMP`, not `TMPDIR`, so a compiled member's own temporaries there stay at the system temp dir. A reset that empties the tmp dir mid-run removes a live run scratch, and the members that lose it fail closed.

`native/src/runner.rs`: one function makes and names the run scratch from the resolved knob, absolutized as §run-gate-tests absolutizes it for a case. The `--run` arm and `native/src/emit/git_hook.rs` call it in place of `std::env::temp_dir()`, and the hook arm's `KNOBS` declaration gains `GATE_SDK_TMP_DIR`, which it does not declare today. `proc::dispatch` adds the ceiling beside the `TMPDIR` it already sets. A failure to make the run scratch keeps today's message and exit 2. **Not yet applied.**

### (2) The fixture runner serves its children from a run scratch {design-bearing} {user-facing: as delta 1}

`gate-sdk/SPEC.md` §run-gate-tests gains a paragraph after the one opening **The pin's scope is the pair loop**. **Not yet applied.**

> **The runner's temporaries are a run scratch's (§run-gates).** Each case invocation and each bespoke `*.test.sh` runs with a private `TMPDIR` under it and the run scratch heading its `GIT_CEILING_DIRECTORIES`, so a suite's `mktemp -d` sandbox lands repo-local and still sits outside any repository. That leaves the scratch pin above as it stands. The pin is the knob a member resolves its declared cache from, and `TMPDIR` is where an anonymous temporary lands.

Replacement for §lib/test-hermetic.sh's sentence **The library heads `GIT_CEILING_DIRECTORIES` with the system temp dir, `${TMPDIR:-/tmp}` resolved physically**, through its end. **Not yet applied.**

> **The library heads `GIT_CEILING_DIRECTORIES` with the temp dir it is handed, `${TMPDIR:-/tmp}` resolved physically**: the system temp dir in a standalone run, and a private dir under the run scratch under `--run-gate-tests` (§run-gate-tests). So a stray `.git` there, which any process can create, never marks a suite's `mktemp` sandbox as inside a repository. A repository a test builds in its sandbox sits below the ceiling and is still found.

`native/src/emit/run_gate_tests.rs`: the arm makes a run scratch through delta 1's function, hands each case invocation and each bespoke test a private `TMPDIR` under it with the ceiling, and removes it on the way out. `lib/test-hermetic.sh`'s code is unchanged. **Not yet applied.**

### (3) The crate's unit tests keep the system temp dir {mechanical}

§run-gates' run-scratch bullet (delta 1) gains the sentence "The crate's unit tests keep the system temp dir: cargo runs them rather than a runner, and the toplevel tests clear the ceiling to test discovery itself, so their trees must lie outside any repository on disk." **Not yet applied.**

### (4) The holders {mechanical}

- **A crate test in `runner.rs`** asserts that the run scratch lies under the absolutized tmp dir, a relative knob value included, and that `proc::dispatch`'s child environment carries a `TMPDIR` under it and a `GIT_CEILING_DIRECTORIES` headed by it.
- **A row in `gate-sdk/gate-tests/run-gate-tests.test.sh`** drives the runner over a scratch tests dir whose one bespoke test records its `TMPDIR` and `GIT_CEILING_DIRECTORIES`. It asserts the first lies under the invoker's `GATE_SDK_TMP_DIR`, the second is headed by the run scratch, and the run scratch is gone once the runner exits. **Not yet applied.**

### (5) The release declaration and the mirror {mechanical}

`.workflow/release-declarations.md`, under **Behavior changes**, gains one bullet. It opens **gate-sdk `--run`, the git hooks and `--run-gate-tests`** and says each run's scratch, with every member's and every suite's temporaries, now lies under `GATE_SDK_TMP_DIR` rather than the system temp dir, made with its self-ignoring `.gitignore` where absent and removed on the way out. It ends "Nothing to do; point `GATE_SDK_TMP_DIR` elsewhere to move it." **Not yet applied.**

The docs mirror regenerates: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **The run scratch** (deltas 1 and 2). Producer: delta 1's function, reached by every `--run` (the battery, and each one `--run-consumer-smoke` runs in its scratch consumer), every placed git hook, and every `--run-gate-tests`. It reads `GATE_SDK_TMP_DIR`, which `--run` and `--run-gate-tests` already declare and the hook arm's declaration gains (delta 1). Consumers: `proc::dispatch` and the fixture runner's spawns, which set it in the child's environment, and the arm's own exit path, which removes it.
- **The child's `TMPDIR`**. Producer: the arm, per child. Consumers: `mktemp` and every `${TMPDIR:-/tmp}` in a shell member or suite, `lib/test-hermetic.sh`'s empty knob files and ceiling, and `std::env::temp_dir()` in a compiled member on a Unix host.
- **The child's ceiling**. Producer: the arm, per child. Consumers: git's own discovery, `walk.rs`'s ceiling read, and `bin/run-gates.sh`'s crosser mark, which reads the same variable. A child's working directory is the repository root or a case dir, neither under the run scratch, so its own discovery is untouched.
- **Roster-holding readers, each red condition named:**
  - §run-gates' invariant, "a full run leaves the tmp dir with no new entry", reds on a run scratch left behind. Delta 1 removes it on the way out, and delta 4's runner row asserts the removal.
  - `check-test-hermetic` reds a bespoke test that does not source the library first. No test changes its source line.
  - `check-amendment-queue` reds a `[spec:]` ref resolving to no file. The paired entry carries this file's name.
  - `check-docs-mirror-fresh` reds the mirror until it regenerates (delta 5).
  - **Every bespoke suite and fixture pair** is a reader of where its sandbox lands. A suite whose subject discovers a repository by a means other than git's ceiling reds by finding this repository above its sandbox. The fixture suites of every kit and `--run-guard-tests` are each run at build. A suite that so reds keeps its sandbox outside by an explicit system temp path, named in this section's honest limit at merge.

## Existing sections updated

- `gate-sdk/SPEC.md` §run-gates, the concurrency contract's scratch-isolation bullet (deltas 1 and 3).
- `gate-sdk/SPEC.md` §run-gate-tests (delta 2).
- `gate-sdk/SPEC.md` §lib/test-hermetic.sh, the ceiling paragraph (delta 2).
- `native/src/runner.rs`, `native/src/proc.rs`'s `dispatch`, `native/src/emit/git_hook.rs` and `native/src/emit/run_gate_tests.rs` (deltas 1, 2 and 4), found by `grep -rn "temp_dir()" native/src` read for production sites, before each file's first `#[cfg(test)]`. A compiled member's own sites, two gates' work dirs and `proc.rs`'s captures among them, follow the bound `TMPDIR` on a Unix host. The sites a top-level arm reaches in its own process stay: guard-kit's decision-table sandbox, context-kit's index tests, the front-end parity arm, the installer smoke's masked roster and `proc.rs`'s captures there. Each lies outside this unit's component or its runners, and they are filed as one costed gap with the Windows `TMP` limit.
- `gate-sdk/gate-tests/run-gate-tests.test.sh` (delta 4).
- `.workflow/release-declarations.md` (delta 5).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no delta renames or retires a name; `checkwright-run.<pid>` keeps its spelling under a new root.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved to Done after the remote run** — the Windows and macOS legs run the new root first on the granted mid-iteration push. The entry moves when build's remote-oracle rule says, at a stage before the drain stage, never in its merge commit.

**Inferred, cannot run before build:** Git for Windows honors the run scratch in the native spelling the crate exports into `GIT_CEILING_DIRECTORIES`, joined by `;` — no Windows host is local; the crate-tests-windows leg on the granted mid-iteration push runs it.
