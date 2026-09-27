# SPEC amendment: linux-musl

The Linux gate binary is linked against glibc and binds `GLIBC_2.39`. Debian 12, RHEL 9 and Ubuntu 22.04 hosts meet a loader failure, and a musl host (Alpine) is refused by name at the installer's libc check. **The route is ruled: a statically linked musl binary is served to every Linux host** (operator direction, 2026-09-27, lead-relayed, not a /consult ruling, on the evidence below). The operator weighed the alternatives: an older build image, a glibc-versioned cross toolchain, musl for musl hosts only, and a hybrid. The operator accepted the one new cost explicitly: a Linux contributor's local consumer smoke needs the musl standard library. The static binary needs no C library from the host, so the glibc floor retires rather than lowers, and a musl host is served rather than refused.

**Measured at authoring (2026-09-27; survey record, "Does a static x86_64-unknown-linux-musl gate binary build …"):**

- **Builds.** `cargo build --release --target x86_64-unknown-linux-musl` (rust 1.98.1) builds clean with one warning: `native/src/hook/wakeup.rs:40` names `libc::time_t`, which the libc crate deprecates on musl. `cargo clippy --all-targets -- -D warnings` on that target reds on that site alone. With the cast target inferred (`epoch as _`), clippy is clean on both `x86_64-unknown-linux-musl` and `x86_64-unknown-linux-gnu`.
- **Static.** The musl artifact is a static PIE: `readelf -d` lists no `NEEDED` entry and `readelf -l` no program interpreter. The glibc artifact needs `libgcc_s.so.1`, `libc.so.6` and `ld-linux-x86-64.so.2`, and requests `/lib64/ld-linux-x86-64.so.2`.
- **Green.** With the musl binary at the canonical `GATE_SDK_NATIVE_BIN` path, the full battery is 139/139 green. The musl crate-test harness gives 1196 passed, 0 failed, 1 ignored, equal to the glibc build.
- **Serves every Linux host.** The glibc build fails to load on `debian:12-slim` (glibc 2.36) and `ubuntu:22.04` (glibc 2.35) with `GLIBC_2.39' not found`, and on `alpine:3.22`. The musl build runs on all three. In alpine with `git` and `bash` installed, `check-comment-tier`, `check-queue-hygiene`, `check-amendment-queue` and `check-md-refs` exit 0 over the tree.
- **Cost.**

  | measure | musl | glibc |
  |---|---|---|
  | battery wall clock | 15.4 s | 14.5–14.9 s (about 4% faster) |
  | crate tests | 4.99 s | 4.58 s |
  | start-up | 0.39 ms | 0.64 ms (no loader for musl) |
  | shell-guard hook | 8.55 ms | 8.34 ms |
  | binary size | about 0.5% larger | — |
- **The smoke's host triple.** `installer/consumer-smoke/run-smoke.sh` reads its host triple from `rustc -vV`, packs a one-line roster of it and builds with no `--target`. On a Linux host that triple is `*-linux-gnu`, and the bootstrap will map the host to `*-linux-musl`, so the two disagree.
- **Where the gnu triples are spelled.** `git grep -n 'linux-gnu\|libc_flavour\|Get-LibcFlavour'` outside `docs/*/SPEC.md`, the queue and release posts shows the spellings to move: docs/install.md's two platform rows, both detectors, both libc checks, `native/targets.list`, `native/runners.list`, `scripts/ci-build-artifact.sh`'s floor arm, and the arm64 Linux leg's index key in `.github/workflows/gates.yml`. The rest are illustrative test data and fixtures that name no live platform: `native/src/gates/install_platforms.rs` and `native/src/install.rs` tests, `scripts/gate-tests/check-install-platforms/` and `gate-sdk/gate-tests/check-gate-substrate-parity/`.

**Not measured:** `aarch64-unknown-linux-musl` (no arm64 host here), and the kernel floor. Rust's platform table states kernel 3.2+ for `x86_64-unknown-linux-gnu` and 4.1+ for `aarch64-unknown-linux-gnu`, and states only "musl 1.2.5" for the two musl targets.

**The order is forced by `check-install-platforms`.** It holds each detector's emitted triple set equal to the declared set, held rows included. It also holds that a `joined` row has a roster line and a `held` row has none. So the detectors, the rows and the roster move in one commit (delta 2). The rows go `held`, and a second commit flips them to `joined` (delta 8) on the run the first one's push buys. That is two pushes, the budget this iteration already holds.

## What changes

### (1) The crate builds warning-free for musl {mechanical}

**Not yet applied.** In `native/src/hook/wakeup.rs` `local_stamp`, `let t = epoch as libc::time_t;` becomes `let t = epoch as _;`, so the cast's target is inferred from `localtime_r`'s parameter and no deprecated alias is named. `bash gate-sdk/bin/build-native.sh` is owed in the same commit.

### (2) The Linux rows, detectors and roster move to musl, held {design-bearing}

**Not yet applied.** One commit:
- **The rows.** In docs/install.md's platform block the two Linux rows become:
  - `| Linux on x86-64, and WSL | Linux 3.2 | `x86_64-unknown-linux-musl` | held: one `gates` run carrying a `native-artifacts` green for this triple and a green `install-smoke-sh-linux` that consumed that upload |`
  - `| Linux on arm64 | Linux 4.1 | `aarch64-unknown-linux-musl` | held: one `gates` run carrying a `native-artifacts` green for this triple and a green `install-smoke-sh-linux-arm64` that consumed that upload |`
- **The roster.** `native/targets.list` loses its two `*-unknown-linux-gnu` lines and gains no line.
- **The runner map.** `native/runners.list` maps `x86_64-unknown-linux-musl` to `ubuntu-latest` and `aarch64-unknown-linux-musl` to `ubuntu-24.04-arm`, in place of the gnu lines.
- **The detectors.** `target_of_host` in `installer/bin/checkwright.sh` and `Get-HostTarget` in `installer/bin/checkwright.ps1` map their Linux arms to the two musl triples.
- **The libc check retires from both halves**, because no Linux artifact needs a host C library any more: `libc_flavour`, the `*-linux-gnu` branch of `select_artifact` calling it, `Get-LibcFlavour`, and the `-like '*-linux-gnu'` block of `Select-Artifact`.
- **The build.** `bash gate-sdk/bin/build-native.sh` is owed. Both lists are in the crate's source stamp.

**No release tag is cut between this commit and delta 8's.** In that window the roster carries no Linux line, so a payload packed from it serves no Linux host.

### (3) The shared build body holds a musl artifact static {design-bearing}

**Not yet applied.** `scripts/ci-build-artifact.sh`'s floor `case` gains `*-unknown-linux-musl) floor_lead=Linux floor_tool=static ;;`. Under `static` the body does three things:
1. It requires the row's `Linux <version>` token, as it requires the glibc and macOS tokens.
2. It refuses the artifact unless `readelf -d` lists no `NEEDED` entry and `readelf -l` lists no `INTERP` segment. A dynamically linked musl artifact would need a musl loader on every host, which would break the route.
3. It prints `floor <target>: static, no loader or shared library needed; Linux <version> declared, not measured`.

The `*-unknown-linux-gnu` arm stays, as mechanism for any consumer declaring a glibc triple.

### (4) The Linux install-smoke legs consume the musl uploads and witness foreign hosts {design-bearing}

**Not yet applied.** In `.github/workflows/gates.yml`:
- **`install-smoke-sh-linux`** gains `needs: [native-artifacts, native-artifacts-roster]`. It gains the arm64 Linux leg's download, normalize and hand-off steps (`INSTALLER_SMOKE_ARTIFACTS_DIR`), so it installs the producer's x86_64 upload rather than a binary it built. That makes it the second half of the join predicate for `x86_64-unknown-linux-musl`. It stays binding through its baseline diff. It is the page-install, one-liner and CI-action witness for every Linux adopter, and a red on the served Linux artifact is a red master deserves. The comment at the diff step saying the binary comes from the smoke's own build arm now names the normalize step.
- **A new step in that leg** runs the normalized artifact's `--list` inside a musl container and inside a glibc-2.36 container: `alpine:3.22` and `debian:12-slim`, each pinned by the digest build resolves when it lands. The step asserts exit 0 and a non-empty listing in each, so every run witnesses the route's claim that one artifact serves musl hosts and glibc hosts below the old floor.
- **`install-smoke-sh-linux-arm64`**'s `runs-on` and `continue-on-error` keys become `['aarch64-unknown-linux-musl']`. Its probe step's libc report is deleted.
- **In both Linux legs**, the normalize step and the smoke's roster steering take the host triple from delta 5's helper instead of `rustc -vV`.

### (5) The consumer smoke takes its host triple from the bootstrap's detector {design-bearing}

**Not yet applied.** A new `installer/consumer-smoke/host-target.sh`, POSIX sh. It extracts `host_arch`, `host_shape` and `target_of_host` from `installer/bin/checkwright.sh`, each bounded by its closing brace at column 0 (the shape `check-install-platforms` already pins and fails closed on). It evaluates them and prints `target_of_host`'s answer. It exits 2 on an extraction that finds no function or an unbounded body, and on an empty answer. So the smoke and the bootstrap cannot disagree about which artifact a host takes: the helper holds no mapping of its own. It carries a `no-port` header citing installer/SPEC.md §The consumer smoke's port disposition for `run-smoke.sh`, whose ground (a harness no payload carries) is the same.

In `run-smoke.sh`:
- `HOST_TARGET` is the helper's answer on every host.
- The `rustc -vV` read retires, and so does the no-rustc fallback to the hand-off's sole target directory, because the helper needs neither rustc nor a hand-off.
- The local build path passes `--target "$HOST_TARGET"` to `build-native.sh` when it differs from `rustc -vV`'s host, and takes the binary from that target's output directory.
- A failed build's `blocked` message names `rustup target add "$HOST_TARGET"` as the remedy when a target was passed.

On macOS and Windows hosts the helper answers the triple `rustc` would have. `check-install-platforms` arm E holds the detector to the declared set, and those triples are each host's own, so those legs keep their `rustc -vV` steering steps unchanged.

### (6) The contributor floor names the musl standard library {mechanical}

**Not yet applied.** CONTRIBUTING.md's build-the-gate-binary bullet gains: "On Linux the consumer smoke builds the musl target the installer serves, so add its standard library once: `rustup target add x86_64-unknown-linux-musl` (or `aarch64-…` on arm64)." In installer/SPEC.md §The consumer smoke, the preflight paragraph ("`cargo` and `rustc` join the preflight …") is rewritten to state three things. The host triple is the bootstrap detector's answer, through the helper. On Linux the local build is `--target` that musl triple, whose standard library the contributor installs. The hand-off path needs neither compiler nor fallback. The contributor-side toolchain rows are not touched: the standard library is a component of the `cargo` member's toolchain, not a program `PATH` resolves.

### (7) The SPECs and headers describe the static Linux artifact {mechanical}

**Not yet applied.**
- **installer/SPEC.md §The gate binary.** The paragraph "**Neither input answers the libc question, …**" becomes:

  > **Neither input answers the libc question, and none is asked.** The Linux artifact is statically linked against musl, so it needs no C library from the host and runs on glibc and musl hosts alike. `uname -s`/`uname -m` and .NET's `OSPlatform`/`OSArchitecture` answer everything selection needs. An earlier glibc-linked artifact made the libc a second selection question with its own refusal row. The static artifact retired both.

  In the selection table, the libc column and the `*-linux-gnu` refusal row are deleted, leaving three rows for three outcomes. "**Four inputs, still three outcomes.**" and its paragraph are deleted. The libc clause of "**Every refusal here names what the host was detected AS.**" ("and the libc verdict where that gate fired") is deleted.
- **installer/SPEC.md §Implementation.** The **No `pipefail`** bullet's "the libc probe reads `grep`'s status," is deleted.
- **installer/SPEC.md §Requirements.** In the **Platform posture** bullet, "The Linux floor is the glibc symbol set the artifact binds, … rather than shipping silently." becomes:

  > The Linux artifact is static musl, so its floor is the kernel, declared from Rust's platform support and not measured, and the shared build body refuses a Linux artifact that links anything dynamically.
- **installer/SPEC.md §The consumer smoke.** In "**The smoke steers its own roster.**", "**A cross-compiling build step is refused**, on grounds already in the tree …" becomes:

  > **A build for a target this host cannot run is refused**, on the ground `native/targets.list` gives: it would publish an artifact no run has executed. A Linux host's musl build is not that: the host runs it, and the smoke executes it.
- **`native/targets.list` header.** The paragraph on the one violation of "Both refuse; neither proceeds" (musl hosts) is replaced by one sentence: every Linux line is a static musl artifact, so a Linux host's C library does not enter selection. The **Cross-compiling** bullet's ground is re-stated as a build for a target its runner cannot execute, and it names the musl build on its own architecture's Linux runner as within the rule.
- **`native/runners.list` header.** The paragraph "Every `-latest` line RIDES its image migrations …" keeps its ride rule. It adds that for the Linux lines a migration cannot raise the artifact's floor, which the build body's static check holds.
- **docs/site-architecture.md**, the install-platforms parity row. The Minimum grammar gains "`Linux <X.Y>` for a `*-unknown-linux-musl` triple, the kernel floor", and the reader sentence for `scripts/ci-build-artifact.sh` gains "or, for a musl triple, refuses an artifact that is not statically linked".

### (8) The join {mechanical}

**Not yet applied.** This is a separate commit, landed only after the mid-iteration push's `gates` run is read and shows four greens: both musl `native-artifacts` legs, `install-smoke-sh-linux`, and `install-smoke-sh-linux-arm64` having consumed its triple's upload and reached the artifact-present branch. The commit:
- flips both Linux rows to `joined`;
- adds `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` to `native/targets.list`;
- adds to `native/targets.list`'s header one attestation bullet in the shape of the existing ones, naming that run's id;
- runs `bash gate-sdk/bin/build-native.sh`.

It rides the close push, whose run is then binding on both triples. If the mid-iteration run is red on a Linux leg, the rows stay `held` and the fix rides the close push. The join then moves to the next iteration as a filed entry, and the close-time release does not ship until it lands, since the roster would carry no Linux line.

### (9) The release declares the change {mechanical}

**Not yet applied.** `.workflow/release-declarations.md` gets two Behavior changes bullets:
- The Linux gate binary is now statically linked against musl, published as `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` in place of the two `*-linux-gnu` triples. It runs on any Linux kernel 3.2 or later (4.1 on arm64), whatever the host's C library, where it needed glibc 2.39. A musl host such as Alpine, refused before, is served. `init` and `update` on an existing install replace the recorded gnu binary with the musl one, because the recorded target differs. Nothing to do.
- The per-target Release archives are renamed with the new triples. A script that downloads a Linux archive by name updates the triple.

## Producers and consumers

- **The musl triples.** Producer: the platform block (delta 2). Readers:
  - the roster steps of `gates.yml` and `publish.yml`, which derive the producer and release matrices;
  - `check-install-platforms`;
  - the shared build body, which reads the Minimum token and runs the static check (delta 3);
  - both detectors;
  - the consumer smoke through the helper (delta 5);
  - `crate-tests-unix`'s derived legs, where the sibling unit `crate-tests-other-triples` lands.
- **The sibling dependency on `crate-tests-other-triples`.** Both musl triples join its `unix_legs` by derivation: neither is the battery's host. The x86_64 leg's static harness was measured green locally. If that unit lands in the same batch as delta 2, the mid-iteration push measures the musl crate tests beside the artifacts. If it lands after, its own push does. Neither case changes this amendment.
- **The sibling dependency on `comment-tier-surface-excludes-ci-workflows`.** It lands first (scope's ordering). Then delta 4's workflow comments are written as directives under the widened gate. If it lands in the same commit batch, they are written as directives anyway, which is what that sweep would make them.
- **The helper.** Producer: `host-target.sh`, reading the bootstrap's own functions. Consumers: `run-smoke.sh`'s `HOST_TARGET`, and the two Linux legs' normalize and steering steps.
- **Roster-holding readers of the new names.**
  - `native/runners.list` names each triple (delta 2), and `native/targets.list` does too after delta 8.
  - `check-install-platforms` fixture pairs stay illustrative and need no row.
  - `host-target.sh` meets `check-shellcheck` and `check-comment-tier` as any shell file does.
  - `check-gate-substrate-parity` is inferred to read the new file's `no-port` header as it reads `run-smoke.sh`'s. Build confirms that against the gate's verdict.
- **Point 5: two corpora narrow.**
  - **The retired libc refusal row.** Its readers are the bootstraps' refusal messages, which go with it, and the SPEC table, which delta 7 rewrites. `git grep -in 'musl\|libc' installer/consumer-smoke scripts/gate-tests gate-sdk/gate-tests` finds no smoke arm or fixture that asserts the refusal, so no reader reds on its absence.
  - **The roster's Linux lines, empty between deltas 2 and 8.** Its readers are pack, which refuses only a declared target it lacks and so is monotone; `check-install-platforms`, held green by the rows being `held`; and the publish matrix, which then has no Linux leg. A tag in that window would ship no Linux artifact, which is why delta 2 forbids one.
- **Point 6.** The two Linux triples are enumerable. Their satisfying values are delta 8's four greens, measured on one run.
- **Oracle.** Locally: the battery and the consumer smoke on a Linux host with the musl standard library. Remotely: the mid-iteration push's `gates` run for the four greens, and the close push's run, binding on both triples.

## Existing sections updated

Roster from the `git grep` in the measured list above, `grep -n "libc\|cross-compil\|Cross-compil\|rustc -vV" installer/SPEC.md native/targets.list native/runners.list installer/consumer-smoke/run-smoke.sh docs/site-architecture.md` and `grep -n "floor_lead\|linux-gnu" scripts/ci-build-artifact.sh .github/workflows/gates.yml`, run 2026-09-27.

- `native/src/hook/wakeup.rs` (delta 1).
- `docs/install.md`, the platform block (deltas 2 and 8).
- `native/targets.list` (deltas 2, 7 and 8).
- `native/runners.list` (deltas 2 and 7).
- `installer/bin/checkwright.sh`, the detector and the libc check (delta 2).
- `installer/bin/checkwright.ps1`, the detector and the libc check (delta 2).
- `scripts/ci-build-artifact.sh` (delta 3).
- `.github/workflows/gates.yml`: `install-smoke-sh-linux` and `install-smoke-sh-linux-arm64` (delta 4).
- `installer/consumer-smoke/host-target.sh`, new (delta 5).
- `installer/consumer-smoke/run-smoke.sh` (delta 5).
- `CONTRIBUTING.md`, the build-the-gate-binary bullet (delta 6).
- installer/SPEC.md §The consumer smoke (deltas 5, 6 and 7).
- installer/SPEC.md §The gate binary, §Implementation and §Requirements (delta 7).
- `docs/site-architecture.md`, the install-platforms parity row (delta 7).
- The on-site mirror of `installer/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (deltas 6 and 7).
- `.workflow/release-declarations.md` (delta 9).

## Retired spellings

- `libc_flavour` — the POSIX half's libc probe, deleted with its only caller (delta 2).
- `Get-LibcFlavour` — the PowerShell half's libc probe, deleted with its only caller (delta 2).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the musl triples, the static check and the helper.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved.** `linux-musl-artifacts` and `glibc-floor-lowering` move to Done in delta 8's commit, after the mid-iteration run is read (build's remote-oracle rule), at a stage before the drain stage. If delta 8 cannot land this iteration, both are demoted per canon-kit's demotion rule, and the join is carried as that entry's increment.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs both declared names against the tree.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
