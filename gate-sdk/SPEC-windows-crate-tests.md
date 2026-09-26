# SPEC amendment: windows-crate-tests

The `crate-tests-windows` job reports 37 failing crate tests on both Windows triples (run 36254992168: `1128 passed; 37 failed; 1 ignored` on x86_64 and on aarch64, the same set). It reports rather than judges until one run is green on both (gate-sdk/SPEC.md §check-crate-arms), so every failure is a warning, and a new Windows regression is one warning among 37. This amendment makes the portability-versus-defect call for each failure and fixes it.

**The ruling is by §The path-dialect contract, not by which side of an assertion looks wrong.** The declared dialect is forward-slash, drive-lettered on Windows (`C:/repo`). So a test asserting a `/`-joined path asserts the contract, and a product value that `Path::join` spelled with `\` is the defect wherever a reader prints, matches or prefix-tests it. A path the product only operates on, for example a spawn target, is untouched by the contract, and its test compares through the host fold. By that rule, 32 failures are test portability, 2 are a gap in the Windows job's environment, and 3 are product defects at two sites.

**The filing's four-class picture was wrong, and five classes replace it.** Eleven `hook::stop_liveness` failures have nothing to do with separators, and the path class holds 12 tests, not 23. The table below groups every failing test. Per-test panics are in the run's log.

**Measured at authoring (2026-09-26):**

- **The failing set.** `gh run view 36254992168 --log-failed` lists 37 `---- <test> stdout ----` blocks per triple, the same 37 names on both.
- **The four bare `bash` spawns.** `grep -rn 'Command::new("bash")' native/src` finds `ere.rs:924`, `knobs/gate_sdk.rs:200`, `main.rs:307` and `walk.rs:1285`, all in tests. The fifth hit, `emit/wait_probe.rs:249`, is `#[cfg(unix)]` product code. On both runners the bare name reached the System32 WSL launcher. The panics quote its "no installed distributions" and "must be updated" text, printed as UTF-16.
- **The rewrite sandbox.** `native/src/emit/rewrite.rs` `Sandbox::new` roots at `walk::canonicalize(&dir)`, and `Sandbox::file` composes `format!("{}/{}", self.root, name)`. That is composing from the unconverted `\\?\` answer §The crate's crosser forbids. All eight cases panic at the seed write with OS error 123.
- **ShellCheck on the Windows job.** `native/src/gates/action_run_shell.rs:189` refuses at exit 2 when `shellcheck` is not on PATH. The `gates` job installs a pinned, digest-verified release (`.github/workflows/gates.yml`, the step citing §check-shellcheck), and the `crate-tests-windows` job installs none. Both `gates::tests` registry-coverage tests assert `rc != 2` over `check-action-run-shell`'s good case.
- **The printed spellings.** `native/src/emit/enter_stage.rs` `wipe` pushes `root.join(&base).display()` into `removed`, which the entry report prints (`boundary-wiped from <tmpdir>: …`). `native/src/installer/recipe.rs:108` joins `payload.join(kit).join("templates/TASK-QUEUE.md")`, returned to `init` and printed by `--install queue-source` as a `queue-source\t<path>` record, which `installer/consumer-smoke/run-smoke.sh` reads.
- **The operated-on spellings.** `proc::resolve_on_path` and `resolve_outside_system_dir` return `dir.join(c)`. Their callers spawn the result: `resolve_floor_tool` feeds `env_probe.rs` and `installer/doctor.rs` a program to run, and neither prints the path. `proc.rs` already carries `folded`, which its later Windows tests apply to both sides.
- **The foreign-dialect test inputs.** `knobs::tests`' scratch helper and `drift_report`'s test build their directories through `PathBuf::join(...).display()`, and feed that into product code joining with `/`, the same output `walk::child` spells. The mixed result is the test's input, not the product's composition.
- **The shebang stubs.** `native/src/hook/stop_liveness.rs` `Scratch::stub` writes `#!/usr/bin/env bash` scripts, and `reader_argv` spawns an override directly. On Windows the spawn fails with OS error 193, and each case asserting a verdict other than `unstarted` fails. `proc::is_executable` is file-ness off unix by its own contract.

## What changes

### (1) Test-side bash goes through the spawn funnel {mechanical}

**Not yet applied.** The four test sites above spawn `programs::BASH` through `proc::run` (or the funnel helper the call's shape needs) rather than `Command::new("bash")`, so a test resolves bash the way product code does: past the System32 homonym (§Fail-closed contract, the spawn funnel). Class: 4 tests, test portability. `ere::tests::the_one_group_capture_agrees_with_bash_rematch_on_a_generated_cross_product`, `knobs::gate_sdk::tests::the_pre_binary_accessors_answer_what_the_table_resolves`, `tests::source_stamp_agrees_with_the_shell_library`, `walk::tests::the_shell_absoluteness_predicate_answers_what_path_root_answers`.

**Inferred, cannot run before build:** each runner image's PATH carries a Git-for-Windows `bash` the funnel resolves, and `gate-sdk/lib/gate.sh` answers these three differential cases under it as the crate does — only a Windows run of the changed tests settles it, and none exists before build pushes.

### (2) The rewrite sandbox composes in the declared dialect {mechanical}

**Not yet applied.** `Sandbox` in `native/src/emit/rewrite.rs`'s tests converts its canonical root before composing: it holds the root through `walk::normalize_abs` of the prefix-stripped answer, and `file` spells through `walk::child`. Class: 8 tests, test portability, every `emit::rewrite::tests` failure: `a_duplicate_operand_is_refused_before_any_write`, `a_literal_spans_a_line_boundary_and_reports_both_lines`, `a_regex_anchors_per_line_and_an_empty_match_advances`, `a_result_that_is_not_utf8_is_refused`, `a_state_file_predicate_error_is_a_refusal`, `a_whole_line_deletion_reports_no_resulting_line`, `an_expect_mismatch_writes_nothing`, `no_match_exits_one_and_writes_nothing`. The prefix strip exists once today, private, in `native/src/emit/run_guard_tests.rs`. It moves to `walk.rs` beside `canonicalize`, whose UNC clause already names stripping as a caller's second lawful property, and both callers use it.

In gate-sdk/SPEC.md §The crate's crosser, the UNC paragraph's "as `--run-guard-tests` does through a pure `strip_extended_prefix` helper before `normalize_abs` runs." becomes:

> through `walk::strip_extended_prefix`, a pure helper, before `normalize_abs` runs, as `--run-guard-tests` and the rewrite arm's test sandbox do.

The same paragraph's last sentence, "Which spelling Windows actually returns, and whether a strip rule is owed, is not decidable from a Linux host.", becomes:

> The first Windows crate-test run failed eight sandbox seeds that composed `/` onto the unconverted answer, with OS error 123, which is the extended-length spelling's refusal of `/`.

### (3) The Windows job installs the pinned ShellCheck {design-bearing}

**Not yet applied.** In `.github/workflows/gates.yml`, the ShellCheck version moves from the `gates` job's step to the workflow's `env`, and `crate-tests-windows` gains an install step before `cargo test`. The step fetches that version's Windows release asset, refuses unless the asset's digest matches the one pinned beside the step, and appends the binary's directory to `GITHUB_PATH`. The `gates` job reads the same version and keeps its own asset's digest. Class: 2 tests, a CI environment gap: `gates::tests::every_registry_member_declares_the_programs_it_spawns` and `gates::tests::every_registry_member_declares_the_roots_it_walks`. Skipping a member whose program is absent is refused: the two tests hold every member to what it spawns and walks, and a skip would stop observing exactly the member that spawns a program.

**Inferred, cannot run before build:** upstream publishes a Windows ShellCheck asset for the pinned version, and that x86_64 binary runs on the `windows-11-arm` image under emulation — the aarch64 leg is the only oracle, reached by a push.

In gate-sdk/SPEC.md §check-shellcheck, "In this tree the `gates` job of `.github/workflows/gates.yml` owns that pin, and a local verdict is read against the version it names." becomes:

> In this tree `.github/workflows/gates.yml` owns that pin: one version in the workflow's `env`, read by each job that installs the analyser, each verifying its own asset's digest. The `gates` job runs the verdict, and `crate-tests-windows` installs it because the crate's registry-coverage tests observe `check-action-run-shell`'s cases. A local verdict is read against the version the workflow names.

### (4) Two printed spellings move to the `child` speller {design-bearing}

**Not yet applied.** Class: 3 tests, **product defects**. Each site's tests already assert the declared dialect and pass unchanged once the product spells in it.

- `native/src/emit/enter_stage.rs` `wipe`: the removed member is still deleted through its `PathBuf`, and the string pushed into `removed` is spelled `walk::child(root, &base)`. The entry report prints it. Test: `emit::enter_stage::tests::the_wipe_spares_root_names_only_and_a_spared_directory_whole`.
- `native/src/installer/recipe.rs` `queue_source`: the candidate is spelled through `walk::child` for the kit and for each of `templates` and `TASK-QUEUE.md`, so the returned string carries no `\`. `init` receives it, and `--install queue-source` prints it for a shell reader. Tests: `install::tests::the_queue_source_op_emits_one_record_or_an_empty_wire` and `installer::recipe::tests::the_queue_source_is_resolved_over_the_whole_kit_set`.

### (5) Path tests spell their inputs and expectations as the product does {mechanical}

**Not yet applied.** Class: 9 tests, test portability.

- `proc::tests`: `a_pathext_shim_beats_the_extensionless_file_beside_it`, `a_posix_path_resolves_its_first_match_unchanged`, `an_empty_path_entry_still_means_the_working_directory` and `the_bare_name_is_still_reached_where_no_variant_exists` compare through `folded` on both sides, as the file's later Windows tests do. The joined path is a spawn target, which the contract leaves to `Path`.
- `knobs::tests`: the scratch directory helper spells its paths through `walk::child`, so the refusal text the tests match is composed from an in-dialect directory: `every_file_refusal_names_its_file_and_line`, `a_locator_in_a_file_and_a_retired_gate_sdk_name_are_refused`, `a_retired_name_is_refused_naming_its_replacement`, `every_reference_refusal_names_its_file_and_line`.
- `emit::drift_report::tests::a_consumer_plugin_shadows_the_built_in_of_the_same_name` builds its expected plugin path through `walk::child` rather than `PathBuf::join(...).display()`.

In gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure, after the paragraph opening "**So `walk.rs` owns a `child` speller**", add:

> **A test is a reader too.** A test that feeds the product a path, or states the path it expects back, spells it through the same `walk` helper the product uses. A path the product only operates on is compared through the host fold on both sides, which is `proc.rs`'s `folded`. A test spelling with `PathBuf::join(...).display()` feeds the product a value no crosser produced, and its failure on Windows reads as a product defect that is not there.

### (6) The liveness stubs are native on Windows {design-bearing}

**Not yet applied.** In `native/src/hook/stop_liveness.rs`'s tests, the stub helper takes the stub's behaviour, an exit code and an optional record line to write first, rather than a bash body. It renders a `#!/usr/bin/env bash` script with its execute bit on unix, and a `.cmd` file on Windows, so each case spawns a reader the host can start, as the override contract requires of a consumer's reader (delegation-kit/SPEC.md §The turn-end liveness hook). The no-bit half of `an_override_is_spawned_with_no_interpreter_word_and_owes_its_own_bit` is `#[cfg(unix)]`: off unix `proc::is_executable` is file-ness by contract, so no file lacks the bit. The unstarted case keeps its nonexistent-interpreter script, which fails to start on every host. Class: 11 tests, test portability: `a_running_shell_task_refuses_beside_a_green_reading`, `an_override_is_spawned_with_no_interpreter_word_and_owes_its_own_bit`, `an_empty_payload_degrades_its_fields_and_still_decides`, `each_reader_exit_class_takes_its_own_verdict_arm`, `each_refusing_arm_names_its_own_finding_and_remedy`, `only_a_firing_meeting_all_three_helper_conditions_is_a_helper`, `only_the_emitting_sessions_own_shell_task_refuses`, `reader_exit_two_over_an_empty_set_is_unresolved_not_corrupt`, `the_record_set_is_counted_after_the_reader_ran`, `the_two_record_bearing_arms_name_the_set_and_unresolved_does_not`, `unresolved_allows_once_the_harness_is_already_continuing`.

**Inferred, cannot run before build:** a `.cmd` reader spawned by path returns its `exit /b` code to the hook on both runners' toolchains, the rung-1 question gate-sdk/SPEC.md §Fail-closed contract leaves to a Windows run — no Windows host exists before build pushes.

## Producers and consumers

- **The fixes.** Every delta changes a test or a spelled string. No new state, event or interface, except the workflow-level ShellCheck version (delta 3). Its readers are the `gates` job's install step and the new `crate-tests-windows` step, and §check-shellcheck names both.
- **The oracle.** The `crate-tests-windows` job on a push, read from its per-triple annotations and log. The entry is done when one run is green on both triples. That run needs a push, so this unit's completion reads a remote run.
- **Roster readers.** `check-action-pinning` and `check-action-permissions` read the workflow. The new step's `uses:`, if any, is SHA-pinned, and the job's permissions stay `contents: read`. The on-site mirror of gate-sdk/SPEC.md re-renders.
- **Point 5.** No corpus narrows. Delta 6's `#[cfg(unix)]` removes one assertion from the Windows build, one whose premise, a file without the bit, does not exist there.
- **Point 6.** The 37 failing tests are the enumerable corpus. Each delta names its members and their fix, and the classes sum to 37 (4 + 8 + 2 + 3 + 9 + 11).

## Existing sections updated

Roster from `gh run view 36254992168 --log-failed` (the 37 names), `grep -rn 'Command::new("bash")' native/src`, `grep -n "shellcheck" .github/workflows/gates.yml` and `grep -n "owns that pin" gate-sdk/SPEC.md`, run 2026-09-26.

- `native/src/ere.rs`, `native/src/knobs/gate_sdk.rs`, `native/src/main.rs`, `native/src/walk.rs` tests (delta 1).
- `native/src/emit/rewrite.rs` tests, `native/src/walk.rs` and `native/src/emit/run_guard_tests.rs` for the moved prefix strip, and gate-sdk/SPEC.md §The crate's crosser (delta 2).- `.github/workflows/gates.yml` and gate-sdk/SPEC.md §check-shellcheck (delta 3).
- `native/src/emit/enter_stage.rs` and `native/src/installer/recipe.rs` (delta 4).
- `native/src/proc.rs`, `native/src/knobs/mod.rs` and `native/src/emit/drift_report.rs` tests, and gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure (delta 5).
- `native/src/hook/stop_liveness.rs` tests (delta 6).
- The on-site mirror of `gate-sdk/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (deltas 2, 3 and 5).
- `.workflow/release-declarations.md`: one Behavior changes bullet (delta 4). On native Windows, the iteration-boundary report and `--install queue-source` spell paths with `/`.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the workflow version and the two respelled values.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines. The one added paragraph has no passage to rewrite.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `crate-tests-windows-failures` moves to Done after the landing push's `crate-tests-windows` run is read green on both triples (build's remote-oracle rule), at a stage before the drain stage. `crate-tests-windows-flip` stays Deferred and becomes buildable.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
