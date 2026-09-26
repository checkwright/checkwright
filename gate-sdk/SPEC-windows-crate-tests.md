# SPEC amendment: windows-crate-tests

No workflow runs `cargo test` on native Windows. The Linux `gates` job runs the crate's tests through §check-crate-arms, and every `native-artifacts` leg compiles them for its own target under `cargo clippy --all-targets`, but nothing executes them on a Windows host. So a unit test pinning a Windows filesystem or process behaviour has never run where that behaviour lives. §The workflow directory records the first instance as an unmeasured honest limit: `--emit capture-drain` renames a log still open for append.

**The ruling: a `crate-tests-windows` job runs the whole suite on each Windows triple, and it reports rather than judges until one of its runs is green.** The whole suite rather than the modules pinning a platform claim, because that narrower set has no derivation: naming its modules would be a hand-kept roster, and the one module this entry names is covered either way. A job of its own rather than a step in `native-artifacts`, because every install-smoke leg waits on that job, and a test run there would lengthen the workflow's critical path.

**Why it reports first, against the derived posture every other Windows leg reads.** Both Windows triples are `joined` (docs/install.md §Requirements), so the derived posture would make the job binding from its first run, and that first run is close's watched push. The suite has 1142 tests. Its only Windows history is a lint pass, and its sources carry unix-shaped literals, so the failure count on that first run is unknown and plausibly large. A binding red there spends the hotfix pushes the close binding allows, on fixes no local host can run. Reporting keeps this unit at the one push scope recorded for it. The hard-coded `continue-on-error: true` is the maintained copy the Windows install-smoke leg's header warns against, so the flip to the derived posture is filed as a deferred entry when the job lands and is not left to a comment.

**Refused: a SPEC boundary note keeping the honest limit.** The limit sits on an arm every adopter's close drain runs, and the witness costs no push of its own.

**Measured at authoring (2026-09-26):**

- `git grep -n "cargo test" .github/workflows` returns nothing. `git grep -c "#\[test\]" -- native/src` sums to 1142, and 5 of them sit directly under `#[cfg(unix)]`. 46 lines across `native/src` carry `"/tmp`, `"/dev/null`, `"/bin/`, `PermissionsExt` or `symlink`.
- Run 36209976485: `native-artifacts` on both Windows triples built in about 1.5 minutes and linted in under 30 seconds. The critical path ran through `native-artifacts (aarch64-pc-windows-msvc)` into `install-smoke-sh-windows-arm64`, about 16 minutes. The roster job took 4 seconds.
- `native-artifacts-roster`'s `pwsh_legs` output is the platform declaration narrowed to the `*-pc-windows-msvc` triples, and it fails its job when that set is empty. Its one reader is `install-smoke-pwsh-windows`.
- The capture-drain case is `emit::capture_drain::tests::a_log_held_open_by_a_writer_still_rotates`.
- The close binding's push-budget line already obliges close to read each job's annotations on a green run and to file one that warrants work with `--emit file-gap`.

## What changes

### (1) The `crate-tests-windows` job {design-bearing}

**Not yet applied.** `.github/workflows/gates.yml` gains a job after `native-artifacts`:

- `needs: native-artifacts-roster`, with its matrix `include:` read from `pwsh_legs`, `runs-on: ${{ matrix.runner }}`, `fail-fast: false`, `timeout-minutes: 30`, `permissions: contents: read`, and `continue-on-error: true`.
- Its steps are: the `core.autocrlf false` step the other Windows jobs run before checkout; the pinned checkout; then `cargo test --release --manifest-path native/Cargo.toml --target "$TARGET"`, its output teed to a log under `$RUNNER_TEMP`, under `set -o pipefail`; then an `if: failure()` step. Every step names `shell: bash`.
- That last step writes one `::warning` annotation per triple. It names the target, the count of `test <name> ... FAILED` lines in the log, and the first ten names, and says the job log carries the rest. It writes one annotation rather than one per test, so a large failure set stays one line a reader can file.
- The header comment says what the job holds, that it reports until a run is green, and that close reads its annotation. It says this in two or three lines, citing §check-crate-arms.

It needs no `native-artifacts` output, so it runs beside that job and off the critical path while its duration stays under the longest install-smoke leg.

### (2) `pwsh_legs` names its second reader {mechanical}

**Not yet applied.** In `native-artifacts-roster`'s `outputs:` block, the comment over `pwsh_legs` names both matrices it feeds, `install-smoke-pwsh-windows` and `crate-tests-windows`, in place of the first alone.

### (3) §check-crate-arms states the test half's CI spelling {mechanical}

**Not yet applied.** The paragraph opening "**The lint half does have one CI spelling, and it holds a different target.**" becomes:

> **Each half has one CI spelling, on a target the battery's host does not build, so neither is the deleted duplicate.** Every `native-artifacts` leg runs `cargo clippy --release --all-targets` at `-D warnings` on its own `--target`, after the artifact build, so a warning reds the leg and never changes what a release publishes. Only a Windows leg compiles the crate's `cfg(not(unix))` code, so a dead item there is invisible to this gate on every contributor host. The `crate-tests-windows` job runs `cargo test --release` on each Windows triple, the one place a test pinning a Windows filesystem or process behaviour runs where that behaviour lives. It reports rather than judges until one of its runs is green on both triples, and then takes the platform-derived posture the other Windows legs read. Until then a failure reaches the run's annotations as one warning per triple naming the failing tests, and close reads those at its watched push.

Its act depends on the debt unit `gate-sdk-native-brevity`, which rewrites this section:

- **Brevity has landed before this delta's batch:** apply the replacement to the rewritten paragraph that carries the lint half's CI spelling, keeping its wording where it states the same fact.
- **Brevity lands in the same batch or later:** apply the replacement as written. Brevity then keeps it, since it keeps every contract sentence.

### (4) §The workflow directory's capture-drain limit points at the job {mechanical}

**Not yet applied.** The paragraph's last two sentences, from "**Honest limit, unmeasured:**" to its end, become:

> **Honest limit, until the Windows crate-tests job judges:** on native Windows the rename under an open handle is expected to succeed, because Rust's standard library opens files sharing delete access. `crate-tests-windows` runs this module's tests there (§check-crate-arms), so each run's log answers whether it does; while that job reports rather than judges, nothing holds it.

## Producers and consumers

- **The job.** Producer: every push and pull request to master, since the job rides `gates.yml`'s triggers and needs only the roster job. It needs no enabling configuration: its matrix is the platform declaration's Windows triples, and both are declared today. Consumer: the run's annotations, which close reads at its watched push under the close binding's push-budget line and files with `--emit file-gap` when one warrants work. The flip entry the build files is the consumer of a green run.
- **The warning's fields.** The target, the failure count and the first ten names are all read by close, which files the finding. The pointer to the job log is read by whoever fixes it. No other field is written.
- **Point 5.** No corpus narrows. `pwsh_legs` gains a reader and keeps its red condition, an empty Windows set, which now fails before either reader starts.
- **Point 6.** The obliged corpus is the Windows triples in `pwsh_legs`: `x86_64-pc-windows-msvc` on `windows-latest` and `aarch64-pc-windows-msvc` on `windows-11-arm`. Each one's satisfying value is a preinstalled cargo that builds that target, attested for both by run 36209976485's `native-artifacts` legs.

**Inferred, cannot run before build:** the suite's pass count on each triple, and the job's wall-clock against the 30-minute timeout and the critical path — only a push runs a Windows leg, and the first push is close's.

## Existing sections updated

The roster comes from `git grep -n "No leg runs the crate's unit tests\|test half stays this gate" -- ':!docs/'` and `git grep -n pwsh_legs`, both run 2026-09-26 over the tracked tree.

- `.github/workflows/gates.yml`, the new job (delta 1) and the `pwsh_legs` output comment (delta 2).
- `gate-sdk/SPEC.md` §check-crate-arms, the lint-half paragraph (delta 3).
- `gate-sdk/SPEC.md` §The workflow directory, the capture-drain paragraph (delta 4).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — the deltas add a job and rewrite two passages, and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the job and its annotation.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** Deltas 3 and 4 re-phrase the passages they replace.
- [ ] **Flip filed.** The commit landing delta 1 files, with `--emit file-gap`, the move of `crate-tests-windows` from `continue-on-error: true` to `${{ matrix.held }}` once one of its runs is green on both triples, costed at a Windows test failure passing unseen.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `crate-tests-unrun-on-windows` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
