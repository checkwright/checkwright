# SPEC amendment: toolchain-pin

§check-crate-arms lints the crate at commit time with whatever toolchain the contributor's host carries, and each CI leg lints and builds with whatever its runner image carries. Three pushes went red on that gap, two of them on the version axis: an image's newer clippy flagged code the local one passed, and an image's toolchain moved under a pinned runner label and changed what its linker emitted. The hotfix for the second named a toolchain for the glibc builds alone, with no stated trigger to move it, and left every other build and the lint step on the image's.

**The shape, in one sentence:** one toolchain file at the repo root names the toolchain every cargo call takes, in CI and on any contributor host that runs rustup, and a roster class says when it moves.

**The seam.** Nothing here is kit mechanism. The crate and its CI are this project's own (`native/` is no kit), the file is this repo's, and §check-crate-arms already carries this repo's CI spellings beside the gate's contract, so the rule lands there.

## What changes

### (1) A root toolchain file is the one owner of the Rust toolchain version {design-bearing} {user-facing: operator direction 2026-10-08 on the queue entry — the unit is in this iteration's set; the entry's deliverable is one rule for the build and the lint, and leaves the pin-or-local-lint choice to this stage}

**Not yet applied.** `rust-toolchain.toml` at the repo root names one channel, the minimal profile and the `clippy` component. Its channel at landing is the version `scripts/ci-build-artifact.sh` names today, so no artifact's builder moves in the commit that lands the file.

- **At the root, not under `native/`.** rustup resolves the file from the working directory upward, and every cargo call in this tree runs from the root with `--manifest-path`.
- **CI takes it everywhere.** Every job that runs cargo, the `gates` job, each `native-artifacts` leg's build and lint, both crate-test jobs and `publish.yml`'s builds, resolves the file, so the build and the lint of one leg can no longer disagree and no leg lints ahead of another. `scripts/ci-build-artifact.sh` loses its glibc case and its `RUSTUP_TOOLCHAIN` export, which would override the file for those targets alone. The lint step loses its `rustup component add clippy` fallback, which the file's component list replaces.
- **A contributor host running rustup takes it too**, which is the contributor-side catch for the version axis: the clippy that lints a commit is the clippy every leg runs. The first cargo call in a fresh clone downloads the named toolchain.
- **A host without rustup is untouched.** The file is inert there, and §check-crate-arms lints at what the host carries.
- **The floor is not the pin.** `rust-version` in the crate manifest stays the floor an adopter-facing build must hold, and `incompatible_msrv` keeps reading it. The pin is the version of the tools that check that floor.

**Ruled out: a local lint at the newest runner version.** It needs the newest version on the contributor's host and leaves the legs free to disagree with each other, which is the second incident.

**Ruled out: the version in the workflow or in the build body.** Two workflows and one script run cargo, and a contributor's host reads none of them.

### (2) The pin moves by a roster class, in a commit of its own {mechanical}

**Not yet applied.** `.workflow/audit-roster.txt` gains a class, `toolchain-pin`, due at release prep: the sweep compares the file's channel with the newest stable and either moves it or records why it holds.

- **A move is one commit that edits the file alone.** Its push is its witness, since the lint and the build of every leg run on runners only, and a red there is then attributable to the toolchain and never to a crate edit sharing the commit.
- **A defect a newer toolchain fixes moves it outside the cadence**, by the same one-file commit.

### (3) §check-crate-arms states the rule and what it leaves uncaught {mechanical}

**Not yet applied.**

- `gate-sdk/SPEC.md` §check-crate-arms, the paragraph on each half's CI spelling: every leg lints and builds on the toolchain the root file names, and the gate lints on the same one wherever the host runs rustup.
- The same section gains the honest limit: a host without rustup lints at its own version, and code under `cfg(not(unix))` is linted on a Windows leg alone, so a lint that fires only there still arrives with the push.
- `CONTRIBUTING.md`, the build-before-commit bullet: the root file names the toolchain, and rustup fetches it on first use.

**Open, escalated to the lead 2026-10-08: the `cfg(not(unix))` axis.** The entry's deliverable also names a contributor-side catch for Windows-only code, an opt-in cross-target clippy in `check-crate-arms` or a pre-push tool. Either needs the Windows target's standard library on the contributor's host. Probed at authoring on this repo's one build host: no rustup on `PATH`, and no `x86_64-pc-windows-msvc` library under the toolchain's sysroot. So an opt-in arm would have no configuration that enables it here, which fails the causal-completeness check's first point, and this amendment authors none until the question closes.

## Producers and consumers

- **The toolchain file.** *Producer:* the landing commit, then delta 2's sweep. *Consumers:* rustup, on every CI runner and on a contributor host that has it. *Enabling config:* the file's presence; nothing sets `RUSTUP_TOOLCHAIN` once the build body's export is deleted, by `git grep -n RUSTUP_TOOLCHAIN -- ':!docs' ':!TASK-QUEUE.md'`, which returns that script alone.
- **Every field has a reader.** The channel is read at every cargo call; the profile and the component list are read when rustup installs the toolchain, the component by the lint step.
- **Cargo-running steps,** by `grep -n 'cargo \|build-native.sh\|ci-build-artifact' .github/workflows/*.yml`: in `gates.yml` the `gates` job's build and its Windows `cargo check`, the `native-artifacts` build and lint, and the two crate-test jobs' build and test; in `publish.yml` the artifact build and one host build. Each runs from the repo root.
- **Roster-holding readers of a new root file:** `scripts/root-allowlist.list`, which `GATE_SDK_ROOT_ALLOWLIST` names.
- **Roster-holding reader of a new roster class:** `check-audit-roster`, which grades the block's keys; a never-swept class carries `last: never`.
- **§check-crate-arms' cache** keys on the `rustc` and `cargo` versions, so a moved pin misses the cache on a rustup host with no change to the gate.
- **No corpus is narrowed and no enumerable corpus is obliged member by member**, so causal-completeness points 5 and 6 bind nothing here.

**Inferred, cannot run before build:** that rustup on each runner image selects the file's toolchain for a cargo call made from the repo root, and installs it with its component on that first call rather than failing — this host has no rustup, and the push this unit carries is the witness. Where an image's rustup does not install on first use, each job installs the file's toolchain before its first cargo call, spelled once in the shared build body.

## Existing sections updated

- `rust-toolchain.toml` — new (delta 1); `scripts/root-allowlist.list` — its row (delta 1).
- `scripts/ci-build-artifact.sh` — the glibc case, its export and its comment's toolchain clause removed (delta 1).
- `.github/workflows/gates.yml` — the lint step's clippy fallback removed (delta 1).
- `.workflow/audit-roster.txt` — the `toolchain-pin` class (delta 2).
- `gate-sdk/SPEC.md` §check-crate-arms — the rule and the honest limit (delta 3).
- `CONTRIBUTING.md` (delta 3).
- `docs/gate-sdk/SPEC.md` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).
- `TASK-QUEUE.md`, the entry — its "left open there" sentence, which deltas 1 and 2 answer (deltas 1 and 2).

## Retired spellings

- None — no delta of this amendment retires a name; delta 1 deletes one environment export whose only site is the script it names.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **The push is read.** The unit's one mid-iteration push follows the landing commit, and every run it triggers is watched to green with each leg's toolchain line read off the log.
- [ ] **The marker discharged** — the cannot-run claim under Producers and consumers is corrected to what that run showed.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Queue entry moved after the run is read, before the stage that drains the queue** — never in the merge commit. Demoted with `--queue demote windows-cfg-msrv-lint-local` where the `cfg(not(unix))` axis stays on it, and done where the escalated question removes that axis from it.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
