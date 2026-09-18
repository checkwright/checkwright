# SPEC amendment: smoke-leg-binary

**The four platform install-smoke legs compile a binary the same run has already
compiled.** `install-smoke-windows`, `install-smoke-macos`,
`install-smoke-macos-intel` and `install-smoke-linux-arm64` each run
`bash gate-sdk/bin/build-native.sh` in a `probe the crate build` step. Each leg then
downloads the `native-artifacts` producer's upload for its own host triple.

**Measured at spec, 2026-09-18, and each finding corrects a premise the paired entry
rests on.**

- **The step is load-bearing, and the suite does not build.** Every one of the four
  legs sets `INSTALLER_SMOKE_ARTIFACTS_DIR`. On that path
  `installer/consumer-smoke/run-smoke.sh` adopts the handed-off artifact and never
  runs `build-native.sh` (its `if [[ -n "$PREBUILT_DIR" ]]` branch). The step's only
  live product is the host binary at `gate_native_bin`'s path. `run-gates.sh`'s
  `exec_arm` dispatches the suite's `--pack-installer` call to that binary and exits
  2 when it is absent. The arm64 leg's step header already records this from run
  34543644528. So the entry's "removing the step buys nothing, the suite rebuilds"
  is wrong: removing the step without a replacement kills the pack.
- **A cross-run build cache would save almost nothing.** In gates run 35214069775
  the step's cargo output shows the crate's dependencies (`serde_json` and its
  closure, `native/Cargo.toml`'s one dependency) finishing in 3 to 11 seconds. The
  crate itself took the rest: 1m28s on Intel macOS, 1m19s on Windows, 36.9s on
  arm64 macOS and 31.6s on arm64 Linux. A cache keyed on the crate's sources misses
  on every push that touches `native/src`. A cache keyed on `Cargo.lock` alone
  restores only the dependencies. The entry's design question, what a restored
  target directory could mask, does not need an answer, because the cache has
  nothing to buy.
- **The consumer's compile repeats a cold compile the producer already did on the
  same runner.** Each `native-artifacts` leg builds the same commit cold, with
  `scripts/ci-build-artifact.sh`, on the runner `native/runners.list` maps its
  triple to. Each consumer leg's `runs-on` resolves to that same label. So a
  host-side cold-build failure is already caught on this run by the producer leg,
  whose posture matches the consumer's (both derived from the declaration, or both
  hard-coded binding for arm64 macOS). The consumer compile adds no coverage.
- **The saving is on the critical path.** In that run every consumer waited on the
  slowest producer (Intel macOS, done at 11:11:39), and `install-smoke-windows` was
  the last job to finish (11:22:41). Its 79 seconds of compile is wall-clock per
  watched push. Across the four legs it is about 236 seconds of runner time.

**The ruling.** Each hand-off leg installs the producer's artifact as the tree's own
gate binary. There is no compile and no cache. The front end then runs the same
bytes the leg later installs into its scratch consumer.

**Why the publish-path refusal does not reach this.** installer/SPEC.md §The
packer records an operator ruling that refuses resolving the
gate binary onto a downloaded artifact in `publish.yml`'s `pack:` job. Its ground
is that the one job that assembles and stamps the **published** tarball would run
bytes it did not check out. Neither half of that ground holds on these legs:

- nothing these legs pack is ever published;
- each leg already runs those exact bytes, because installing them into a scratch
  consumer and running its battery is what the leg asserts.

This amendment leaves that ruling and `publish.yml` unchanged. The leg header cites
the ruling's reach so that a reader comparing the two jobs does not see a
contradiction.

**This reading of the ruling's reach is confirmed, operator direction,
2026-09-18:** the 2026-09-09 refusal (installer/SPEC.md §The packer) is scoped
to `publish.yml`'s `pack:` job and does not reach the install-smoke legs; they
may adopt the producer's downloaded artifact. The scoping above is this
amendment's own argument for that reach; the direction settles it rather than
resting on the argument alone.

**Not reached.** `install-smoke` (the baseline Linux leg) and
`install-smoke-powershell` build on purpose. The first sets no hand-off, so its
suite compiles the payload's artifact. The second packs a locally built artifact
because it installs no producer upload. Between them they keep build-then-pack
rehearsed in one job on every run, which is the claim installer/SPEC.md makes for
ROUTE 1.

## What changes

### (1) The hand-off legs install the producer's artifact as the tree's gate binary

On each of the four hand-off legs, the `probe the crate build` step is deleted
{mechanical}. The step that normalizes the producer's upload gains a copy after its
completeness check, so that `gate_native_bin`'s path holds the artifact for the host
triple. **Not yet applied.**

```bash
dest="$(gate_native_bin)"
mkdir -p "$(dirname "$dest")"
cp "$art/$host/$binary" "$dest"
chmod +x "$dest"
[[ -x "$dest" ]] || { echo "could not place the producer's $host artifact at $dest"; exit 1; }
```

- `$art`, `$host` and `$binary` are the names each normalize step already binds.
  `$binary` is `gate_native_bin`'s basename, which is the producer's artifact name
  for the host triple because both derive the suffix from `gate_exe_suffix`.
- The `chmod` restores the mode the artifact transport drops.
- The copy lands under the crate's gitignored `target/` (`.gitignore`'s `target/`),
  so the smoke's clean-worktree preflight still holds.
- The artifact directory stays as it arrived. The copy leaves the digest-verified
  bytes and their sidecar in place for the smoke's own hand-off check
  (gate-sdk/SPEC.md §Consumer payload: moved, never re-derived).

### (2) The leg headers say why nothing builds and why no cache is added

The four legs' comments are re-phrased {mechanical}. **Not yet applied.**

- **The deleted step's headers go with it.** That covers the three "Reporting only"
  probe headers and the arm64 leg's "It is also load-bearing" paragraph.
- **The arm64 macOS normalize header** (the sibling the others cite) gains the
  argument, stated once:
  - the producer's upload becomes this tree's gate binary because `run-gates.sh`
    dispatches the suite's pack to it;
  - the producer leg on this same runner label compiled it cold on this run, so a
    compile here repeats that and catches nothing new;
  - this is not the case installer/SPEC.md §The packer's publish-path refusal rules
    on, because
    nothing packed here is published and the leg installs these bytes anyway.
- **One directive in that same header:** do not add a build cache here. The crate is
  nearly all of the compile and changes on most pushes, so a cache restores only the
  dependencies (run 35214069775). A leg that needs the crate compiled on its host
  has the producer leg for that.
- **The Windows, Intel macOS and arm64 Linux normalize headers** cite the arm64
  macOS sibling rather than restating it, as they already do for the pattern
  download.
- **The Intel macOS leg's timeout comment** ("same cold Homebrew and cold cargo
  build") drops the cargo clause. The arm64 macOS leg's measured `20m12s` stays as
  it is: it is a dated measurement and is still true of that run.

## Producers and consumers

- **The placed binary (delta 1).**
  - Producer: the normalize step on each of the four hand-off legs, from the
    `native-artifacts` upload for the host triple.
  - Consumers: `run-gates.sh`'s `exec_arm`, reached by `run-smoke.sh`'s pack and
    planted-pack call sites and by the rest of its `run-gates.sh` calls on the tree.
    Its absent-binary refusal (exit 2) is the red if the copy did not land. The
    step's own `-x` assertion fails first.
  - Enabling config: `INSTALLER_SMOKE_ARTIFACTS_DIR`, set by all four legs. The
    baseline Linux leg sets no hand-off and is untouched.
- **The header prose (delta 2).** Its readers are the next session editing these
  legs. There is no machine reader.
- **Roster-holding readers.** No knob, gate, arm, tag or marker is minted. Readers
  of `.github/workflows/*.yml` among the gates (`check-action-run-shell`,
  `check-action-pinning`, `check-action-permissions`, `check-action-gh-repo`,
  `check-enforcement-fresh`, from `git grep -ln 'workflows/' -- '*.gate'`) see one
  fewer `run:` step per leg and no new action. Every remaining step on those legs
  keeps `shell: bash`.
- **Point 5 (narrowing).** Delta 1 removes four steps and one compile per leg. Red
  conditions of the readers:
  - `check-action-run-shell` reds on a Windows or expression-runner `run:` with no
    `shell:`. The edit adds lines to an existing step and no step, so it can only
    remove findings.
  - `check-enforcement-fresh` reds on a stale `docs/enforcement.md`. That projection
    names no `build-native.sh` invocation (`grep -n build-native docs/enforcement.md`
    is empty), so it does not move. Build runs the gate to confirm.
  - The legs' own verdicts. The smoke refuses on a dirty worktree or an absent
    front-end binary; the copy prevents the second and cannot cause the first.
- **Point 6 (members).** The obligation reaches the four legs that set
  `INSTALLER_SMOKE_ARTIFACTS_DIR`, listed by
  `grep -n INSTALLER_SMOKE_ARTIFACTS_DIR= .github/workflows/gates.yml`. Each one's
  satisfying value is its producer's `gate-binary-<triple>` upload, which its
  normalize step already asserts complete (binary plus sidecar) before the copy.

## Existing sections updated

- `.github/workflows/gates.yml`: on the four hand-off legs, the deleted step and the
  normalize steps' copy (delta 1), plus the step, normalize and timeout comments
  (delta 2).
- `TASK-QUEUE.md`: at merge, in the build session that lands delta 1 and before the
  drain stage is entered, `smoke-leg-crate-build-uncached` moves to Done. The commit
  message records the measured critical-path saving on the first watched push (all
  deltas).

Roster produced by `git grep -n "probe the crate build\|INSTALLER_SMOKE_ARTIFACTS_DIR="`
and `git grep -ln 'workflows/' -- '*.gate'` over the tracked tree. It is a floor that
build re-derives.

## Retired spellings

- `probe the crate build` — the deleted step's name (delta 1).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each delta.
- [ ] **Run the system**: on the watched push, all four hand-off legs conclude as
      their posture requires, each log shows the copy and no `Compiling
      checkwright-gates` line, and `install-smoke-windows`' duration drops by about
      the compile it no longer runs.
- [ ] **Merged with no information lost**: the measured grounds (dependency share,
      producer-on-same-label, the publish ruling's reach) survive in the leg
      headers.
- [ ] **Queue move placed before the drain stage.**
- [ ] **Amendment deleted**: this file is removed on merge.
- [ ] **Gaps filed**: any gap build finds.
