# SPEC amendment: linux-glibc

Two units share one table, and this amendment settles both.

- **The glibc artifacts.** The next release would publish no glibc Linux gate binary. platform-prerequisite-floors replaced the two `*-unknown-linux-gnu` lines of `native/targets.list` with the static `*-unknown-linux-musl` pair. The operator wants glibc artifacts published beside musl before the next tag (operator direction, 2026-09-27, lead-relayed), and v0.27.0 is held on it.
- **The page-to-release gap.** docs/install.md says `joined` means "a binary is published for that system", while `check-install-platforms` binds `joined` to a live roster line. So between a join and the next tag the page promises what the pinned release does not serve. That is the live state today: the page declares the musl pair `joined`, and v0.26.0 publishes only the gnu pair.

**Measured at authoring (2026-09-27).**

- **Selection holds no libc question today.** `target_of_host` (`installer/bin/checkwright.sh`) and `Get-HostTarget` (`installer/bin/checkwright.ps1`) map a Linux host straight to a musl triple.
- **Restoring gnu rows needs the detectors to emit them.** `check-install-platforms` arm E holds each detector's emitted set **equal** to the page's declared set, so gnu rows the detectors never emit red.
- **The build body already measures both kinds.** `scripts/ci-build-artifact.sh`'s floor arm measures a `*-unknown-linux-gnu` artifact's glibc floor with `readelf` and refuses one that differs from its Minimum cell. It holds a `*-unknown-linux-musl` one to a static link. The last gnu build, on `ubuntu-latest` (24.04), bound `GLIBC_2.39`.
- **The pinned release's triples are on record.** `check-release-assets` holds each tag's asset set to the target roster at that tag. `git show v0.26.0:native/targets.list` lists the gnu pair, the two macOS and the two Windows triples, which is what `gh release view v0.26.0` lists. v0.26.0's docs/install.md carries the platform block in its older bullet grammar, with no Minimum cell.
- **The pin and the disposition are already read.** `check-front-door-verbs` resolves the pinned release (`pin=` in `docs/install.sh`, tag `v<pin>`, a `git show` at the tag) and reads this iteration's `.workflow/release-disposition.txt` line. That file carries no line for this iteration yet.
- **A mismatched recorded target is rewritten.** The consumer smoke's host triple comes from `installer/consumer-smoke/host-target.sh`, the detector extracted. The lock records the placed triple as `artifact.target`, and placement rewrites the binary when the recorded target differs from the resolved one.
- **Payload size.** v0.26.0's six archives total 17.0 MB and its npm tarball 17.8 MB. Each Linux archive is about 3 MB.

**The ruling on the host-to-artifact choice: prefer glibc where it runs, fall back to static musl everywhere else, and let the loader answer.** On Linux the detector names a **preferred** triple, the gnu one, and that triple names a **fallback**, the musl one of the same architecture. The bootstrap verifies the preferred artifact and then runs it once with `--help`. If it runs, it is the artifact. If it does not start or exits non-zero, which is what a glibc below its floor or a musl host's missing loader produces, selection moves to the fallback. No bootstrap reads the C library or compares a version. So no floor literal enters either bootstrap as a second copy of the Minimum cell, and a glibc host meets the glibc build exactly when that build runs on it. Two alternatives are refused:

- **Reviving the libc probe** (`libc_flavour`, `Get-LibcFlavour`) with a version comparison. It needs each floor spelled in both bootstraps, a copy of the page's Minimum cell no gate holds, and it re-asks a question the loader answers exactly.
- **musl for every host and gnu on explicit request.** It needs an adopter-facing selector nobody would set, and it publishes a glibc build no install takes.

**The ruling on the gap: the pinned release is what `joined` promises, held like the front door's verbs.** A `joined` row names a triple the pinned release publishes, at the Minimum the pinned release's page states. A row the pinned release does not serve is admitted while this iteration will release, and reds while its disposition withholds a release. That is `check-front-door-verbs`' pending admission, applied to a platform row. It is an arm of `check-install-platforms`, the gate that owns the block, so the block keeps one reader.

## What changes

### (1) The detectors name a preferred triple and its fallback {design-bearing}

**Applied** in the declaring commit. In `installer/bin/checkwright.sh`, `target_of_host` maps `Linux/x86_64` to `x86_64-unknown-linux-gnu` and `Linux/aarch64|Linux/arm64` to `aarch64-unknown-linux-gnu`. A new function `fallback_of_target` takes a triple and prints its fallback, `x86_64-unknown-linux-musl` for the first and `aarch64-unknown-linux-musl` for the second. It prints nothing for every other triple. Each fallback triple is the sole single-quoted operand of a `printf`, and the case patterns name the preferred triples unquoted. In `installer/bin/checkwright.ps1`, `Get-HostTarget` takes the same two gnu triples, and `Get-FallbackTarget` is the twin, each fallback the sole single-quoted operand of a `return` with `return ''` its no-mapping arm.

`check-install-platforms` arm E (`native/src/gates/install_platforms.rs`) extracts each detector's emitted set as the union of both functions' operands, on the same fail-closed extraction: a function absent, unbounded or yielding nothing is exit 2. The fixture pair's `checkwright.sh` and `checkwright.ps1` gain the fallback function, and `bad/` gains a fallback triple its `install.md` does not declare. `installer/consumer-smoke/host-target.sh`'s bare call is unchanged. It extracts `target_of_host`, so the smoke steers at the preferred triple. It gains `--fallback <triple>`, printing what `fallback_of_target` names, so deltas 4 and 5 name the static and the foreign triples without spelling them.

In installer/SPEC.md §Platform resolution, the two shape bullets name both functions per half. The paragraph beginning "**Neither input answers the libc question, and none is asked.**" becomes:

> **Neither input answers the libc question, and the loader does.** A Linux host maps to the glibc triple, and that triple names the static musl triple of the same architecture as its fallback. The glibc artifact runs only on a glibc at or above the floor it was linked against, while the static one runs on any Linux kernel at its floor. So the bootstrap asks the verified glibc artifact to run, and takes the fallback when it cannot (§Selection). `uname -s`/`uname -m` and .NET's `OSPlatform`/`OSArchitecture` still answer everything the map needs, and neither bootstrap reads a C library's name or version. A version comparison would spell each glibc floor a second time in each bootstrap, where the install page's Minimum cell and the build body's measurement are the owners, and it would re-ask a question the loader answers exactly.

### (2) Selection walks the preferred triple, then its fallback {design-bearing}

**Applied** in the declaring commit. In both bootstraps, selection and verification run per candidate, the preferred triple first:

1. A candidate the payload's roster does not carry passes to its fallback. With no fallback it is the unrostered refusal, unchanged.
2. A rostered candidate whose pair is incomplete is the broken-payload refusal, unchanged. It never passes to the fallback, since a publisher defect must not be masked by a smaller install.
3. A rostered, complete candidate is verified. A digest mismatch refuses, unchanged, and never passes to the fallback.
4. A verified candidate **that has a fallback** is run once as `<artifact> --help` with its output discarded. If it exits 0, it is selected. If it fails to start or exits non-zero, it passes to its fallback. With no rostered fallback, the bootstrap refuses: "the `<triple>` gate binary does not run on this host, and this payload carries no fallback for it", with the remedy "no adopter action; report the host, the triple and the release". A candidate with no fallback, every non-Linux triple, is selected without a probe, as today.

In the PowerShell half, a start failure is the exception `&` throws, caught, and a non-zero `$LASTEXITCODE`.

In installer/SPEC.md §The install boundary, bootstrap steps 2 to 4 become:

> 2. resolve the host to its preferred Rust target triple, and that triple to its fallback where it has one;
> 3. read the payload's target roster and resolve each candidate's artifact and sidecar, preferred first, refusing a declared target whose pair is incomplete;
> 4. verify the artifact's SHA-256 against that sidecar, and where the candidate has a fallback, run it once with `--help`, passing to the fallback when it does not run.

The sentence ending "The bootstrap, which the standing "assume no POSIX shell" obligation binds, spawns nothing." (§What runs behind the invoke) becomes "The bootstrap, which the standing "assume no POSIX shell" obligation binds, spawns nothing but a verified artifact: the one it executes, and on Linux the preferred one it asks to run."

In installer/SPEC.md §Selection, "**Selection has three outcomes, only one of which proceeds, and collapsing any two is a defect.**" becomes "**Selection has four outcomes, only one of which proceeds, and collapsing any two is a defect.**", and the table becomes:

> | the host's candidates | the payload holds | outcome |
> | --- | --- | --- |
> | none in the roster | — | **refuse**: this platform is not in the support roster; there is no adopter action |
> | a rostered one | nothing, or half its pair | **refuse**: the payload is broken |
> | a rostered one, verified | its pair, and it runs or has no fallback | execute |
> | a rostered one with a fallback, verified | its pair, and it does not run, with no rostered fallback | **refuse**: this binary does not run on this host; there is no adopter action |

A paragraph is added after the table:

> **A candidate that does not run passes to its fallback, and no refusal does.** A preferred triple the roster lacks, which is a held one, and a verified preferred artifact that does not run both pass to the fallback. A broken pair and a digest mismatch refuse, because they are defects in what was published, and serving the fallback would hide them behind a working install. The probe runs only after verification, so nothing unverified is executed, and only where a fallback exists, so no other host runs its artifact twice.

The **Every row of the selection table runs under an oracle** paragraph names delta 5's two new cases as the oracle for the fourth row and for the fallback path of the third.

In installer/SPEC.md §Parity between the two bootstraps, the step 2 row's observable becomes "one half's empty string against the other's triple, or the two naming different fallbacks". The step 3 row's becomes "unrostered vs. verify vs. broken payload vs. does not run, and which candidate each half took, by message and remedy, never the exit status alone".

### (3) The gnu triples are declared, built on pinned images, then joined {design-bearing}

**The declaring commit is applied; the join commit is not yet applied.** This lands in two commits, on the join predicate `native/targets.list`'s header states.

**The declaring commit** lands with deltas 1 and 2, before the mid-iteration push:

- `native/runners.list` gains `x86_64-unknown-linux-gnu ubuntu-24.04` and `aarch64-unknown-linux-gnu ubuntu-24.04-arm`. The images are pinned rather than `ubuntu-latest`, so the glibc floor does not follow a floating image.
- docs/install.md's platform block gains two rows, `held: the gates run that builds this triple and installs it on its Linux leg`, with Minimum `glibc <X.Y>`, the floor the build body measures on the pinned image (2.39 at the last gnu build). If the measurement differs, the leg refuses, and the cell takes the measured value. The System cells tell the four Linux rows apart by C library: the gnu rows "Linux on x86-64 (glibc), and WSL" and "Linux on arm64 (glibc)", and the musl rows "Linux on x86-64 (any C library)" and "Linux on arm64 (any C library)". The prose after the block gains one sentence: "On Linux, `init` takes the glibc binary where it runs and the static one everywhere else."
- `.github/workflows/gates.yml`'s two Linux install-smoke legs re-key to the gnu triples (delta 4). This rides the declaring commit rather than waiting for the join: delta 1 lands in the same commit, so `target_of_host`/`Get-HostTarget` already answer the gnu triple once it lands, and the mid-iteration run this commit's own text reads as its oracle (below) needs the re-keyed legs to have consumed the gnu upload before that run can be read at all. Landing delta 4 later would leave `install-smoke-sh-linux` hard-coded to `ubuntu-latest` with no `continue-on-error` for a triple the roster still holds, and `install-smoke-sh-linux-arm64` reading the musl row's `joined` state for what the detector now answers as a gnu triple.
- `bash gate-sdk/bin/build-native.sh`, since `native/runners.list` is in the source stamp.

**The join commit** lands after the mid-iteration run shows, for each gnu triple, `native-artifacts` green and its Linux install-smoke leg green having consumed that upload (delta 4):

- `native/targets.list` gains the two gnu lines, and its header's evidence list gains a bullet naming that run. The header's sentence "Every Linux line is a static musl artifact, so a Linux host's C library does not enter selection." becomes "Each Linux architecture carries two lines: a glibc artifact the bootstrap prefers where it runs, and a static musl artifact that runs on any Linux kernel at its floor (installer/SPEC.md §Selection)."
- Both gnu rows flip to `joined`.
- `bash gate-sdk/bin/build-native.sh`.

The join commit rides the close push, and the release tag follows it. If the mid-iteration run is red on a gnu leg, the rows stay `held`, the fix rides the close push, and v0.27.0 is not tagged until they join, since the operator held it on this unit.

In installer/SPEC.md §Requirements' **Platform posture** bullet, "The Linux artifact is static musl, so its floor is the kernel, declared from Rust's platform support and not measured. The shared build body refuses a Linux artifact that links anything dynamically." becomes:

> Each Linux architecture has two artifacts. The glibc one's floor is the glibc symbol set it binds, measured by the shared build body on a pinned image. The static musl one's floor is the kernel, declared from Rust's platform support, and the build body refuses it if it links anything dynamically. A host meets the glibc artifact where it runs and the musl one everywhere else (§Selection).

### (4) The Linux legs consume the preferred triple and witness the fallback's premise {design-bearing}

**Applied in delta 3's declaring commit**, since the mid-iteration run delta 3 reads as its join oracle needs these legs already consuming the gnu upload. In `.github/workflows/gates.yml`:

- **`install-smoke-sh-linux`** reads its `runs-on` and `continue-on-error` from the roster index keyed `x86_64-unknown-linux-gnu`, as the other platform legs read theirs, in place of `ubuntu-latest`. While the gnu row is held, this leg reports and does not bind, and the join commit makes it binding with no workflow edit.
- **`install-smoke-sh-linux-arm64`** re-keys its index lookups from `aarch64-unknown-linux-musl` to `aarch64-unknown-linux-gnu`, the triple `host-target.sh` answers there.
- **Both legs** take the host triple from `host-target.sh` as today, so each consumes its gnu upload through the smoke's one-line roster. That is the consumer half of the join.
- **The foreign-host step** of `install-smoke-sh-linux` keeps its musl assertions and adds the fallback's premise. The musl artifact's `--list` exits 0 with a non-empty listing in the pinned `alpine` and `debian:12-slim` images, as today. The x86_64 gnu artifact's `--help` exits non-zero in both, since that failure is what passes selection to the fallback. The step's name and its `# spec:` directive say it witnesses both.

`crate-tests-unix` gains an `aarch64-unknown-linux-gnu` leg by derivation. `x86_64-unknown-linux-gnu` is the `gates` image's own `rustc` host, so the roster job leaves it out.

In installer/SPEC.md §The consumer smoke, the paragraph "**The baseline Linux leg also runs the served artifact on foreign hosts.**" becomes:

> **The baseline Linux leg also runs both Linux artifacts on foreign hosts.** Inside an `alpine:3.22` and a `debian:12-slim` container, each pinned by digest, it asserts that the x86_64 musl artifact's `--list` exits 0 with a non-empty listing, and that the x86_64 glibc artifact's `--help` exits non-zero. The first witnesses that the static artifact serves a musl host and a glibc host below the glibc floor. The second witnesses the premise §Selection's fallback rests on, that the glibc artifact does not run on either.

In the paragraph "**Each leg installs the producer's upload, never a build of its own.**", "A Linux leg takes the host triple from the bootstrap's detector through `host-target.sh`, since its `rustc` names a gnu triple the installer does not serve, and every other leg from `rustc -vV`, which names the detector's triple there." becomes "Every leg takes the host triple from the bootstrap's detector through `host-target.sh`, which on Linux names the preferred glibc triple."

### (5) The artifact arm witnesses the fallback and the fourth refusal {design-bearing}

**Applied** in the declaring commit. `installer/consumer-smoke/run-smoke.sh`'s artifact arm gains three cases on a Linux host whose hand-off carries a foreign-architecture gnu artifact. Every CI Linux leg's does, since each downloads every producer's upload. Each case builds its payload by placing artifacts **with their producer sidecars**, never by computing a digest (gate-sdk/SPEC.md §Consumer payload's one-producer rule). The foreign-architecture gnu artifact stands in for a preferred artifact that does not run, because its bytes verify and cannot start on this host.

- **Falls back.** Roster: the host's gnu triple and its musl fallback. The gnu directory holds the foreign artifact and its sidecar, and the musl directory holds the host's musl upload. `init` completes, and the lock's `artifact.target` is the musl triple.
- **Held preferred.** Roster: the musl fallback alone. `init` completes on the musl triple.
- **No fallback.** Roster: the host's gnu triple alone, holding the foreign artifact. `init` refuses with the does-not-run message. The arm asserts that message and remedy differ from the unrostered and broken-payload refusals', as it already asserts theirs differ.

A host with no foreign artifact in its hand-off, including every local run, skips the three cases and says so, as the `shasum`-less host skips its case. In installer/SPEC.md §The consumer smoke, the artifact arm's paragraph ending "…That comparison keeps §The gate binary's three outcomes from collapsing into each other." takes "four outcomes" and a sentence naming the three cases and their skip.

The paragraphs on the local build (the one beginning "`cargo` and `rustc` join the preflight" and the one after it) lose the musl build. On Linux the detector's triple is now the glibc one `rustc` builds natively, so the local smoke needs no `--target`, no musl standard library and no container route. "On Linux that triple is the musl one the installer serves, so the local build passes `--target` that triple, whose standard library the contributor installs once with `rustup target add`, and a failed build names that remedy." becomes "On a Linux host that triple is the glibc one `rustc` builds natively. Where the detector's triple is not `rustc`'s host, the build passes `--target` that triple and a failed build names the `rustup target add` remedy." The next paragraph, "A host with no rustup takes the hand-off path…", is deleted. The sentence "on Linux it costs the musl standard library the preflight paragraph below names" in "**The payload every profile installs carries a real gate binary**" is deleted. In `run-smoke.sh`, the comment "a Linux host builds the musl triple the installer serves, which its own toolchain's host triple is not" becomes "a host whose detector triple is not its toolchain's host cross-builds that triple". The branch stays as mechanism.

CONTRIBUTING.md's pull-request bullet "On Linux the consumer smoke builds the musl target the installer serves, so add its standard library once: `rustup target add x86_64-unknown-linux-musl` (or `aarch64-unknown-linux-musl` on arm64)." becomes "On a Linux host the consumer smoke builds natively against the glibc triple `rustc` already targets; a host whose detector triple differs from its toolchain's own, as a foreign-architecture CI leg's does, adds that target once with `rustup target add`."

### (6) `check-install-platforms` holds `joined` to the pinned release {design-bearing}

**Applied**, landing `install-platform-release-gap`. `check-install-platforms` gains **arm G**. For each `joined` row, with `<pin>` the pinned version and `v<pin>` its tag:

- **Triple.** The row's triple must be a line of the target roster at `v<pin>`, read by `git show v<pin>:<roster>`, where `<roster>` is the gate's own roster operand (default `native/targets.list`), never `GATE_SDK_NATIVE_TARGETS_FILE`, whose value a smoke narrows. The roster at a tag is that tag's published asset set, which `check-release-assets` holds.
- **Minimum.** When the page at `v<pin>` carries the platform block in the table grammar, the row's Minimum cell must equal that tagged page's Minimum for the same triple. When the tagged block does not parse as the table, this half is dormant, and the clean line says so. v0.26.0's bullet grammar is that case, and every tag from the next release on carries the table.

A finding is **admitted as pending**, listed on the clean line, while `.workflow/release-disposition.txt` carries no line for the iteration the queue header names, or a `vX.Y.Z` field. It **reds** while that field is `none` or `deferred:vX.Y.Z`. Where `v<pin>` does not resolve, as in a shallow checkout, the arm is dormant and the clean line says so. Its help line names the remedies: release so the pin carries the row, or return the row to `held`.

The pin resolution, the tag read and the disposition reader are the ones `check-front-door-verbs` runs, moved into one crate module both gates call, so the two cannot disagree about the pinned release or the pending window. The descriptor `scripts/check-install-platforms.gate` gains `docs/install.sh`, `.workflow/release-disposition.txt` and `TASK-QUEUE.md` in `couples=`, and its `# spec:` line gains a clause for arm G. The gate's positional form gains the pinned roster, the pinned page, the disposition file and the queue file, as `check-front-door-verbs`' does, so the fixture pair holds still as the tags move. `good/` carries a joined row the pinned roster lacks under a pending disposition, and `bad/` carries it under `deferred:`, with a Minimum that differs from the pinned page's.

docs/install.md's sentence "`joined` means a binary is published for that system; `held` means it is supported but not published yet, and names the run that would publish it." becomes "`joined` means the current release publishes a binary for that system, at the floor shown; `held` means it is supported but not published yet, and names the run that would publish it."

In docs/site-architecture.md's **install-platforms parity contract** row, "`joined` means the triple is a live line in `native/targets.list`." becomes "`joined` means the triple is a live line in `native/targets.list` and a line of the roster at the pinned release's tag, at that tag's Minimum." A sentence naming arm G and its pending admission, pointing at installer/SPEC.md §The front door's verbs for the admission's grounds, is added after the one naming the lockstep's two directions. The binding sentence "**The binding is four-way rather than three-way, and the fourth and fifth surfaces are the two host detectors:**" names both functions per detector.

In installer/SPEC.md §The front door's verbs, the **pending admission** paragraph gains a closing sentence: "`check-install-platforms` arm G admits a `joined` platform row the pinned release does not serve on the same terms, through the same reader (docs/site-architecture.md §Generated projections and their freshness gates)."

## Producers and consumers

- **The preferred and fallback triples.** Producers: `target_of_host`/`Get-HostTarget` and `fallback_of_target`/`Get-FallbackTarget`. Consumers: selection in the same half (delta 2), `host-target.sh` (preferred only), and arm E, which extracts both.
- **The probe's verdict.** Producer: the preferred artifact's `--help` exit status. Consumer: delta 2's step 4, which reads it only to pass to the fallback. It is written nowhere.
- **The fourth refusal.** Producer: delta 2's step 4 with no rostered fallback. Consumers: the adopter, and delta 5's no-fallback case.
- **The recorded target.** Unchanged field. A Linux host whose glibc artifact runs records the gnu triple. An install recorded on musl before this release is rewritten on its next `init` or `update` by placement's existing mismatch rule.
- **Arm G's findings and pending list.** Producer: arm G. Consumers: the battery at commit, and the close's disposition commit, which meets the red when it writes `none` or `deferred:` over an unserved row.
- **Roster-holding readers of the new names.** Arm E is the one reader holding the detectors' function names, and it names both new functions (delta 1). `host-target.sh` extracts `target_of_host` alone and needs no edit. `scripts/check-install-platforms.gate`'s `couples=` and `# spec:` line gain arm G's inputs (delta 6). No knob is minted.
- **Point 5.** No corpus narrows. Arm E's detector corpus widens by one function per half, and arm G adds findings.
- **Point 6.** Delta 3 obliges each gnu triple to join. The members are the two gnu triples, and each one's satisfying value is its declaring commit's runner line and held row, and then its join commit's roster line and `joined` state.
- **Sibling dependency.** [crate-tests-windows-flip](TASK-QUEUE.md#crate-tests-windows-flip) and [windows-fresh-fixture-stub](TASK-QUEUE.md#windows-fresh-fixture-stub) share the mid-iteration push. Their legs are Windows-only and read no Linux row, so the two units compose in either batch order.

## Existing sections updated

Roster from `git grep -n -i "musl\|glibc\|libc" -- '*.md' '*.list' '*.yml' '*.sh' '*.ps1' ':!docs/*/SPEC.md' ':!TASK-QUEUE.md' ':!docs/posts'`, `grep -n "Neither input answers\|three outcomes\|spawns nothing\|five steps\|joined. means" installer/SPEC.md docs/install.md docs/site-architecture.md` and `grep -n "The pending admission" installer/SPEC.md`, run 2026-09-27.

- `installer/bin/checkwright.sh`, `installer/bin/checkwright.ps1`, `native/src/gates/install_platforms.rs` arm E and its fixture pair, and installer/SPEC.md §Platform resolution (delta 1).
- Both bootstraps' selection, and installer/SPEC.md §The install boundary, §What runs behind the invoke, §Selection and §Parity between the two bootstraps (delta 2).
- `native/runners.list`, `native/targets.list`, docs/install.md's platform block, and installer/SPEC.md §Requirements (delta 3).
- `.github/workflows/gates.yml`'s two Linux install-smoke legs, and installer/SPEC.md §The consumer smoke (delta 4).
- `installer/consumer-smoke/run-smoke.sh` and installer/SPEC.md §The consumer smoke (delta 5).
- `native/src/gates/install_platforms.rs` arm G, the module shared with `native/src/gates/front_door_verbs.rs`, `scripts/check-install-platforms.gate`, its fixture pair, docs/install.md, docs/site-architecture.md's parity contract row, and installer/SPEC.md §The front door's verbs (delta 6).
- CONTRIBUTING.md's Linux wording, which `git grep` shows naming the musl build, re-read for the native glibc local build (delta 5).
- The on-site mirror `docs/installer/SPEC.md`, regenerated by the command `check-docs-mirror-fresh` prints (all deltas).
- `.workflow/release-declarations.md`'s Behavior changes section. The unreleased musl-switch bullets are rewritten into the net change from v0.26.0, whose Linux archives were gnu: the gate binary is published for Linux twice per architecture, a glibc build and the static musl build, and the installer takes the glibc build where it runs and the musl build everywhere else, asking no question about the host's C library. Nothing to do (deltas 1 to 3).
- `installer/consumer-smoke/host-target.sh` gains `--fallback <triple>` (deltas 4 and 5).

## Retired spellings

- None — no name is retired. The musl triples stay published, and the libc probe this amendment refuses to revive left the tree at 162ff6af.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the fallback function, the probe, the fourth refusal, the two gnu lines, the re-keyed Linux legs, the three `run-smoke.sh` artifact-arm cases and arm G.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved.** `install-platform-release-gap` moves to Done in delta 6's landing commit, at a stage before the drain stage. Its oracles are local. `linux-glibc-artifacts` moves to Done in delta 3's join commit, after the mid-iteration run is read (build's remote-oracle rule), at a stage before the drain stage. If the join cannot land this iteration, `linux-glibc-artifacts` is demoted under canon-kit's rule, with the join carried as its increment, and the release stays held.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
