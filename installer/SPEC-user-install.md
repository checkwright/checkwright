# SPEC amendment: user-install

Pairs `hook-only-user-install`, and carries `installer-crate-test-gaps` as its delta 5, since both land on the installer's verbs and crate tests. An adopter running its own work engine wants one session-hook arm and nothing else: the gate binary placed for the user rather than for a repository, the harness settings entry for that arm printed, and the arm's scratch and log outside any repository. This amendment adds a per-user install of a tree-free arm (gate-sdk/SPEC.md §The harness-integration arm), the `--knobs` token that points the arm at its knob file, and the crate-test cases the commit path was missing.

Deltas 1 to 4 and 6 are applied and merged into their owner sections; each heading below names where. Delta 5 is the one still to apply, and the file goes when it lands.

**What the tree does today**, for delta 5, read from `native/tests/installer_commit.rs`: the `git()` setup helper spawns `git` with the parent's environment unscrubbed, where `verb()` scrubs it.

## What changes

### (1) The `--knobs` hook token {design-bearing}

Applied — merged into gate-sdk/SPEC.md §The harness-integration arm and §The knob file, and context-kit/SPEC.md §check-settings-paths.

### (2) The per-user directories {design-bearing}

Applied — merged into installer/SPEC.md §The user install and §The user manifest.

### (3) `init --user <arm>` {design-bearing}

Applied — merged into installer/SPEC.md §The verbs and §The user install, and `installer/README.md`'s flag table.

### (4) `update --user` and `uninstall --user` {design-bearing}

Applied — merged into installer/SPEC.md §update, §uninstall and §The user install's honest limits.

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

Applied — the cases in the crate and gate-sdk tests, the user-install arm in installer/SPEC.md §The consumer smoke, the release declaration, and the regenerated mirrors.

## Producers and consumers

- **The crate-test cases** (delta 5). Producer: the two test files. Consumer: the crate test run, locally and on every CI leg.

## Existing sections updated

- `native/tests/installer_commit.rs` and the tests in `native/src/installer/mod.rs` (delta 5).

## Retired spellings

- None — delta 5 adds test cases and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — delta 5's **Not yet applied.** is applied, each case shown able to fail before it lands.
- [ ] **Amendment deleted** — `ls installer/SPEC-*.md` lists no `SPEC-user-install.md`.
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery (`bash gate-sdk/bin/run-gates.sh --run`) and the crate tests, all green. The Windows legs, the surrogate case included, ride the closing push.
