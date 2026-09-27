# SPEC amendment: crate-tests-unix

The crate's unit tests run on the battery's own triple, through `check-crate-arms` in the `gates` job, and on each Windows triple, through `crate-tests-windows`. The macOS triples and every non-Windows Linux triple except the battery's own only lint the crate, at `native-artifacts`' clippy step. A unit test pinning a macOS or arm64 behaviour never runs where that behaviour lives. The theme of this iteration is wide platform support (operator direction, 2026-09-27), and a published triple whose tests never run is support in name only. This amendment adds one job that runs `cargo test` on every published non-Windows triple the battery does not already test.

**Measured at authoring (2026-09-27):**

- `git grep -n 'cargo test' .github/workflows` returns only `crate-tests-windows`'s step.
- `native-artifacts-roster` (`.github/workflows/gates.yml`) derives `legs`, `index` and `pwsh_legs` from docs/install.md's platform block. `pwsh_legs` is `legs` narrowed by `select(.target | endswith("-pc-windows-msvc"))`. The roster job runs on `ubuntu-24.04`, the image the `gates` job pins.
- The declared non-Windows triples today are `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `aarch64-apple-darwin` and `x86_64-apple-darwin`. `x86_64-unknown-linux-gnu` is the battery's own triple, whose tests `check-crate-arms` already runs, so three legs remain. That set changes with the route `glibc-floor-lowering` takes (see Producers and consumers). The job derives its legs rather than listing them, so either route reaches it with no edit here.
- The crate-test harness needs the host gate binary, the pinned ShellCheck and the docs gates' ruby gems. The registry-coverage tests assert that no member exits 2 over its fixture cases. A battery run this session in `ubuntu` containers carrying no `shellcheck` and no ruby showed `check-shellcheck`, `check-action-run-shell`, `check-docs-render-fidelity` and `check-docs-liquid-parse` each exiting 2. That the same members exit 2 over their fixture cases on a host without those tools is inferred from the same code path.
- ShellCheck v0.11.0's asset digests, computed from a download on 2026-09-27: `linux.x86_64` `8c3be12b05d5c177a04c29e3c78ce89ac86f1595681cab149b65b97c4e227198` (equal to the `gates` job's pin, which cross-checks the method), `linux.aarch64` `12b331c1d2db6b9eb13cfca64306b1b157a86eb69db83023e261eaa7e7c14588`, `darwin.aarch64` `56affdd8de5527894dca6dc3d7e0a99a873b0f004d7aabc30ae407d3f48b0a79`, `darwin.x86_64` `3c89db4edcab7cf1c27bff178882e0f6f27f7afdf54e859fa041fca10febe4c6`.
- A static `x86_64-unknown-linux-musl` test harness runs on a glibc host: 1196 passed, 0 failed, 1 ignored (the same counts as the glibc harness), measured this session for `linux-musl-artifacts`.

## What changes

### (1) The roster job derives the unix legs {mechanical}

**Applied at build; the merge waits on the mid-iteration push's `gates` run being read.** `native-artifacts-roster` gains a fourth output, `unix_legs`. It is `legs` less the Windows triples and less the battery's own triple. The battery's triple is read in the same step from `rustc -vV`'s `host:` line: the roster job and the `gates` job run on one pinned image, so that line names the triple `check-crate-arms` tests. It is never spelled as a literal. An empty `unix_legs` is written as `[]`, and the new job then has no legs. That is not a refusal, because a roster of Windows and battery-host triples alone has nothing left to test.

### (2) A `crate-tests-unix` job runs `cargo test` per leg {design-bearing}

**Applied at build; the merge waits on the mid-iteration push's `gates` run being read.** A new job in `.github/workflows/gates.yml`, beside `crate-tests-windows`, with `needs: native-artifacts-roster`, a matrix over `unix_legs`, `runs-on: ${{ matrix.runner }}`, `continue-on-error: ${{ matrix.held }}`, `timeout-minutes: 30` and `permissions: contents: read`. Every step names `shell: bash`, because `runs-on` is an expression and `check-action-run-shell` cannot derive the dialect otherwise. Steps, in order:

1. Checkout, SHA-pinned as the sibling jobs pin it.
2. On macOS, `bash scripts/ci-macos-floor.sh`: the harness sources `gate-sdk/lib/gate.sh` through the gates it runs, and a macOS image's `/bin/bash` is 3.2.
3. Install the pinned ShellCheck at the workflow's `SHELLCHECK_VERSION`, choosing the asset for the runner's OS and architecture, verifying the digest pinned beside the step for that asset (the four digests above, keyed by asset suffix), and appending the binary's directory to `GITHUB_PATH`.
4. Install the docs gates' gems into the ruby the gates' bare `ruby` resolves, with the `gates` job's gem set and `liquid` pin.
5. `rustup target add "$TARGET"`, then `bash gate-sdk/bin/build-native.sh`. The registry-coverage tests spawn the host binary, which `cargo test --target` never builds.
6. `cargo test --release --manifest-path native/Cargo.toml --target "$TARGET"`, teed to a log.
7. On failure, one `::warning` annotation per leg naming the failing tests, in `crate-tests-windows`'s annotate step's shape.

A joined leg's red fails the workflow. That is the posture every derived leg reads, and the reason is the one `native-artifacts` states: a held triple is published nowhere, and a joined one is published on the strength of this run.

**If a leg is red at the landing push**, the act depends on the cause:
- **A test-portability or CI-environment failure that one follow-up commit fixes:** build fixes it inside this unit, and that commit's push is the oracle.
- **A failure one commit cannot fix:** the leg takes `crate-tests-windows`' reporting posture (`continue-on-error: true` and the annotation) through a `reports` field that `unix_legs` carries for that triple alone. The failing set is filed to the gap inbox as that triple's flip entry, in `crate-tests-windows-flip`'s shape. A held-back leg is named in gate-sdk/SPEC.md §check-crate-arms beside the Windows job's reporting sentence.

**Inferred, cannot run before build:** each image's toolchain builds the crate for its own triple, the four ShellCheck assets run on their runners, a Homebrew or system ruby on each image accepts the gem install, and the crate's tests pass on macOS and arm64 Linux — only the landing push's run answers these, and no macOS or arm64 host exists before it.

### (3) The SPEC names the new CI spelling {mechanical}

**Applied at build; the merge waits on the mid-iteration push's `gates` run being read.** In gate-sdk/SPEC.md §check-crate-arms, the paragraph opening "**Each half has one CI spelling, on a target the battery's host does not build, so neither is the deleted duplicate.**" has the sentence "The `crate-tests-windows` job runs `cargo test --release` on each Windows triple, where a test pinning a Windows filesystem or process behaviour runs beside that behaviour." rewritten as:

> `cargo test --release` runs on every other published triple, where a test pinning a platform's filesystem or process behaviour runs beside it: `crate-tests-windows` on each Windows triple, and `crate-tests-unix` on each remaining triple except the battery's own, which the roster job reads off the `gates` image's `rustc`. Each builds the host binary first, and the unix legs install the ShellCheck and gems the `gates` job installs.

The sentence after it, "It builds the host binary first, …", is then folded into that rewrite rather than kept. The sentences on the Windows job's reporting posture that follow it stay as they are.

In gate-sdk/SPEC.md §check-shellcheck, "The `gates` job runs the verdict, and `crate-tests-windows` installs it because the crate's registry-coverage tests observe `check-action-run-shell`'s cases." becomes:

> The `gates` job runs the verdict, and `crate-tests-windows` and `crate-tests-unix` install it because the crate's registry-coverage tests observe `check-action-run-shell`'s cases.

## Producers and consumers

- **`unix_legs`.** Producer: the roster step, on every run of `gates.yml`. Consumer: `crate-tests-unix`'s matrix. Its fields are `target` (read by the rustup, test and annotation steps), `runner` (`runs-on`) and `held` (`continue-on-error`). They are the same three `legs` carries. The conditional `reports` field in delta 2 exists only if a leg is held back, and its reader is that leg's `continue-on-error`.
- **The sibling dependency on `glibc-floor-lowering` and `linux-musl-artifacts`.** If their route publishes `*-linux-musl` triples, those join `unix_legs` by derivation, and the x86_64 one is not the battery's triple, so it gets a leg. Its `cargo test --target x86_64-unknown-linux-musl` runs a static harness on the glibc runner, measured green locally. If they land in the same build batch as this unit, their triples are in the declaration before the landing push and that push measures them. If they land after, the push that lands them measures them. Either way this amendment needs no edit.
- **Roster readers.** `check-action-pinning` (the checkout `uses:` is SHA-pinned as its siblings are), `check-action-permissions` (`contents: read`), `check-action-run-shell` (every `run:` names `shell: bash`), and `check-comment-tier` once `comment-tier-surface-excludes-ci-workflows` lands. That sibling lands first (scope's ordering), so every comment this job carries is written as a directive under that gate.
- **Point 5.** No corpus narrows.
- **Point 6.** The legs are enumerable at authoring: the three non-Windows, non-battery triples above under today's declaration. Each leg's satisfying value is a green `cargo test` on its runner, inferred until the push.
- **Oracle.** The landing push's `crate-tests-unix` run, read per leg from its log and annotations. The entry moves when that run is read, by build's remote-oracle rule.

## Existing sections updated

Roster from `git grep -n 'cargo test\|pwsh_legs\|crate-tests-windows' .github/workflows gate-sdk/SPEC.md` and `grep -n "owns that pin\|Each half has one CI spelling" gate-sdk/SPEC.md`, run 2026-09-27.

- `.github/workflows/gates.yml`: `native-artifacts-roster` (delta 1) and the new `crate-tests-unix` job (delta 2).
- gate-sdk/SPEC.md §check-crate-arms and §check-shellcheck (delta 3).
- The on-site mirror of `gate-sdk/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (delta 3).

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for `unix_legs` and the new job.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** The §check-crate-arms edit rewrites the sentence it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `crate-tests-other-triples` moves to Done after the landing push's `crate-tests-unix` run is read, with every leg green or held back under delta 2's second act, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** A held-back leg's flip entry, and any cross-component gap, goes to the gap inbox.
