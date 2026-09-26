# SPEC amendment: foreign-shells

A contributor host without `pwsh` or `dash` first runs the suites that need them on CI. `--run-front-end-parity` compares the front end with its PowerShell twin under `pwsh`. The installer smoke drives the POSIX bootstrap through `sh`, which is dash on the Linux CI legs and often bash on a contributor's host. So a change to a PowerShell twin or a POSIX `sh` surface meets its first real run at close's watched push, and a red there costs a second push. A build that needed a local `pwsh` improvised a container shim, which the scope boundary wiped.

**The ruling: a contributor arm, `--with-foreign-shells <command> [<arg>...]`, runs a command with `pwsh`, `dash` and `sh` on `PATH`.** The shells are copied out of pinned container images and run natively on the host. With no usable Docker, the arm skips cleanly. It is kit mechanism in gate-sdk, with its two images as knobs, and no adopter needs it.

**Why copied out rather than run inside a container.** Both suites spawn the host's gate binary. A binary built on the host links a newer C library than an image's, so it cannot start inside the image. The image's own binaries do run on the host's newer library, so a copied `pwsh` or `dash` reaches the host's binary, `git` and `bash` unchanged.

**Why an arm, not a documented recipe or a `scripts/` tool.** A recipe is prose nothing executes, so it rots silently. A new tracked shell file needs a no-port cause from the live exception classes (§The port-candidate criteria), and a contributor convenience has none. An arm is executable, unit-tested, and listed by the binary's own roster.

**Refused: a validate suite wrapping it.** The skip exits 0, so the evidence record would read a run that did nothing as a pass.

**Measured at authoring (2026-09-26), on this Linux host, glibc 2.43, Docker 29.7.2:**

- Running `docker create` then `docker cp <c>:/usr/bin/dash` on `debian:stable-slim` gives a `dash` that runs natively.
- `docker cp <c>:/opt/microsoft/powershell/7` on `mcr.microsoft.com/powershell:latest` gives a 175M tree whose `pwsh`, 7.4.2 from that cached image, runs natively with no environment set.
- Both tools were symlinked into a directory put first on `PATH`. `--run-front-end-parity` then ran clean: 28 cases, the twin under `pwsh`, in 9 s. `bash installer/consumer-smoke/run-smoke.sh`, with `sh` linked to that `dash`, ran clean in 110 s.
- `installer/consumer-smoke/run-smoke.sh` invokes the bootstrap as `sh <path>`, so the `PATH` lookup selects its shell.
- The crate's network spawners are `--usage-poll` and `--pack-installer`, listed as `NETWORK_ARMS` in a test in `native/src/emit/mod.rs` that holds them disjoint from `FENCE_SAFE_ARMS`. The same file's `every_arm_table_row_resolves_to_a_file_with_a_test_module` holds each arm-table row's file to a test module. `pwsh` and `powershell` are `contributor` roster members spawned only by `--run-front-end-parity` (§The program roster).

## What changes

### (1) The arm's contract {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md gains `### with-foreign-shells` directly after §upgrade-smoke:

> **`--with-foreign-shells <command> [<arg>...]` runs a command with `pwsh`, `dash` and `sh` on `PATH`, copied out of pinned container images and run natively.** A contributor on a Linux host without them uses it to run, before a push, a suite that needs them: `--run-front-end-parity` under `pwsh`, and the installer smoke under a `sh` that is dash, as it is on the Linux CI legs. A session changing a PowerShell twin or a POSIX `sh` surface runs that surface's suite under it before committing.
>
> - **Provisioning.** The arm provisions each shell whose image knob is non-empty. It runs `docker create` on the image, which pulls it if absent, `docker cp` the shell out, and removes the container. `pwsh` takes its install directory, `/opt/microsoft/powershell/7`. `dash` takes `/usr/bin/dash`. The copies land under `<GATE_SDK_TMP_DIR>/foreign-shells/<key>/`, where `<key>` is a digest of the image reference. A `bin/` there holds `pwsh`, `dash` and `sh`, the last two linking the one dash. A marker written last lets a later run reuse the directory, so a pull happens only on a first run or a changed reference. Each copy is probed once (`pwsh -NoProfile -Command exit`, `dash -c :`), because an image built against a newer C library than the host's cannot start.
> - **The run.** The command runs through `bash -c 'exec "$@"'` with `bin/` first on `PATH`. The working directory and the rest of the environment stay as invoked, and the arm exits with the command's status.
> - **The skip.** No `docker` on `PATH`, a daemon that fails `docker info`, or a host that is not Linux: the arm runs nothing, writes `with-foreign-shells: skipped — <reason>; <command> not run` on stderr, and exits 0. The copies are Linux binaries, so only a Linux host runs them natively. The provisioning code compiles on Linux alone, and every other build compiles only the skip.
> - **Exit 2.** No command given, an image that cannot be created or copied from, or a copy that fails its probe. Each names the step and the image.
>
> Knobs, config-via-env in the `<KIT>_<KNOB>` shape:
>
> - `GATE_SDK_FOREIGN_PWSH_IMAGE` — the image `pwsh` is copied from. The default is a digest-pinned `mcr.microsoft.com/powershell` reference, and empty provisions no `pwsh`.
> - `GATE_SDK_FOREIGN_DASH_IMAGE` — the image `dash` is copied from. The default is a digest-pinned `debian` stable-slim reference, and empty provisions neither `dash` nor `sh`.
> - Scratch is the existing `GATE_SDK_TMP_DIR`.
>
> A default pins a digest, so a moved tag never changes what a run copies, and a bump is one deliberate edit. The in-image paths are the arm's constants, bound to the images the defaults name. A knob pointing at an image with another layout, or with no variant for the host's architecture, meets the copy step's exit 2.
>
> **Family and spawns.** It is an `Arm::Run`, because the command's status passes through and exit 2 is the arm's own. It spawns `docker`, each copy once for its probe, and `bash` to run the command, so what else it runs is unbounded by construction. When an image is absent, `docker` reaches the network, so the arm is outside `FENCE_SAFE_ARMS`. Its unit tests hold the argument grammar, the skip on a `PATH` with no `docker`, and the `bin/` layout built over a constructed copy directory.
>
> **Honest limits.** Windows PowerShell 5.1 and every Windows host behaviour stay on the Windows legs. A shebang naming `/bin/sh`, such as the generated hooks git runs directly, still runs the host's own `/bin/sh`, since `PATH` does not reach it. The CI legs remain the oracle, and a local green shortens the path to a green push without replacing it.

### (2) §run-gates' parity paragraph names the local route {mechanical}

**Not yet applied.** In §run-gates, the paragraph opening "**The twin is held by `--run-front-end-parity`**" ends, from "It runs on the binding Windows install-smoke leg", with:

> It runs on the binding Windows install-smoke leg, the one site that measures 5.1, and on the `gates` job. It never runs in the battery, whose host may carry no PowerShell. A contributor on a Linux host with Docker runs its `pwsh` half before the push under §with-foreign-shells. Without that, a divergence reds at push, not at commit, the trade the macOS remedy block and the installer's PowerShell half already accept.

### (3) The program roster gains `docker` and `dash` {mechanical}

**Not yet applied.** `native/src/programs.rs` gains `docker` and `dash` as `contributor` members. In §The program roster, the audience bullet for `pwsh` and `powershell` becomes:

> - `pwsh` and `powershell` are `contributor` members: `--run-front-end-parity` spawns them on CI legs, and §with-foreign-shells probes the `pwsh` it copies, so the floor-or-probe relation does not bind them. `docker` and `dash` are `contributor` members on that arm's ground alone.

### (4) The fence-safe network sentence and the spawn list name the arm {mechanical}

**Not yet applied.** In §The non-gate arm, "The crate's only network spawners are `--usage-poll` (`curl`) and `--pack-installer` (`npm`)" names `--with-foreign-shells` (`docker`) as a third. The `NETWORK_ARMS` list in `native/src/emit/mod.rs`'s tests, which holds the network spawners disjoint from `FENCE_SAFE_ARMS`, gains it. The spawn list's "**Unbounded by construction.**" bullet gains:

> `--with-foreign-shells` spawns `docker`, the shells it copies, and `bash`, which runs whatever command it was handed (§with-foreign-shells).

Its act depends on the debt unit `gate-sdk-native-brevity`, which rewrites §The non-gate arm:

- **Brevity has landed before this delta's batch:** add the arm to the rewritten sentence naming the network spawners and to the rewritten unbounded-spawn entry, in their new wording.
- **Brevity lands in the same batch or later:** apply the edits as written. Brevity then keeps them, since it keeps every contract sentence.

### (5) The arm lands {design-bearing}

**Not yet applied.** Five pieces land together:

- A module `native/src/emit/foreign_shells.rs` holding the arm and its `#[cfg(test)]` module.
- An `ARMS` row spelling `--with-foreign-shells` as an `Arm::Run`, declaring `GATE_SDK_FOREIGN_PWSH_IMAGE`, `GATE_SDK_FOREIGN_DASH_IMAGE` and `GATE_SDK_TMP_DIR`.
- The two knobs in gate-sdk's static table, each defaulting to a digest-pinned reference. The build resolves those references at landing, `mcr.microsoft.com/powershell:latest` and `debian:stable-slim`, with `docker buildx imagetools inspect`, and verifies that the pinned `pwsh` keeps its install directory at the constant path.
- The roster members from delta 3.
- A `.workflow/release-declarations.md` Behavior-changes bullet naming the new arm and its two knobs, which ship in the binary.

## Producers and consumers

- **The arm.** Producer: a contributor or session invoking it through the front end. Its named caller is the session changing a PowerShell twin or a POSIX `sh` surface, which the arm's own section names (§The non-gate arm, the named-caller property). It needs no enabling configuration: both knobs default non-empty.
- **The copies and `bin/`.** Producer: the provisioning step. Consumers: the wrapped command, through `PATH`, and a later run, through the marker. The marker's one field is its presence, read at the reuse check.
- **The skip line.** Read by the invoking contributor or session, the only reader of stderr here. Its reason and its `not run` clause are what tell a skip from a run.
- **Roster-holding readers of the minted names.** The flag joins `ARMS`, and the arm-table census test holds its module's test block. The knobs join gate-sdk's static table, which `--emit knob-roster`, `check-docs-cmd` assertion B and `check-knob-default-coupling` read. `docker` and `dash` join `programs.rs`, whose four unit-tested relations are met by the `contributor` audience. The arm joins the network-spawner set its unit test holds.
- **Point 5.** No corpus narrows.
- **Point 6.** The obliged corpus is the two shells. `pwsh`'s satisfying value is `/opt/microsoft/powershell/7` in the PowerShell image, and `dash`'s is `/usr/bin/dash` in the Debian image. Both were measured above on the cached images, and the build re-measures on the pinned ones.

## Existing sections updated

The roster comes from `grep -n "reds at push, not at commit" gate-sdk/SPEC.md`, `grep -n "only network spawners" gate-sdk/SPEC.md`, `grep -n "Unbounded by construction" gate-sdk/SPEC.md` and `grep -n "are .contributor. members" gate-sdk/SPEC.md`, all run 2026-09-26.

- `gate-sdk/SPEC.md`, the new §with-foreign-shells (delta 1).
- `gate-sdk/SPEC.md` §run-gates, the parity paragraph (delta 2).
- `gate-sdk/SPEC.md` §The program roster and `native/src/programs.rs` (delta 3).
- `gate-sdk/SPEC.md` §The non-gate arm, the fence-safe paragraph and the spawn list, and the network-spawner unit test (delta 4).
- `native/src/emit/foreign_shells.rs`, `native/src/emit/mod.rs`, gate-sdk's static knob table and `.workflow/release-declarations.md` (delta 5).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — the deltas add an arm, two knobs and two roster members, and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, its knobs and its copies.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** Deltas 2 to 4 re-phrase the passages they touch. Delta 1 is a new section, since no section held the rule.
- [ ] **Run on this host.** `--with-foreign-shells` wrapping `--run-front-end-parity`, and wrapping `bash installer/consumer-smoke/run-smoke.sh`, both exit 0 with the pinned images. With `docker` off `PATH`, it prints the skip line and exits 0.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `foreign-toolchain-docker-legs` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
