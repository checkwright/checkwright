# SPEC amendment: release-assets

Two public claims say what a GitHub Release carries, and nothing holds either. gate-sdk/SPEC.md §Consumer payload says each target's binary and sidecar ride the Release as one archive per target. docs/install.md §Requirements, in the `cargo` bullet, says the binary reaches an adopter as a prebuilt Release asset for a declared target. The files a Release carries are whatever `.github/workflows/publish.yml`'s `pack` job writes into its output directory, which the `release` job attaches whole. A rename there, or a target the archive loop skips, changes what every later Release carries while both claims keep reading true.

**The ruling: §Consumer payload declares the asset set in one machine-read line, and a consumer-registered `check-release-assets` holds the `pack` job's output to that line before any Release exists.** The prose and the line sit together in the section that rules the fact, the shape `payload-discloses:` already takes there. The gate has two halves, a battery half and a publish half. The battery half asserts at every commit that the declaration is well-formed and that the publish workflow still calls the publish half. The publish half runs inside `pack` and compares the output directory's file set with the declaration, expanded over the tag's version and the target roster. A mismatch fails `pack`, which the `release` job needs, so a false asset claim never reaches a published Release.

**Its own class, not a second axis of `check-payload-claim`.** That gate matches sentences against a vocabulary of disclosure classes (canon-kit/SPEC.md §check-payload-claim). An asset set is an enumeration whose truth is a build product, and no sentence pattern can hold it. So the claim-registry trigger recorded there does not fire.

**Consumer-registered, like the release gates beside it.** Only the repository that publishes the payload has a publish workflow and a Release. An adopter receives both and publishes neither. So the gate lives in this repo's `scripts/`, compiled into the shared crate like `check-release-channel-parity`, and no kit ships it. The seam, ruled:

- **Kit mechanism:** none. The gate reads two existing gate-sdk knobs and mints no knob.
- **Consumer config:** the declaration line, the gate's registration and its fixture pair, and the call in `publish.yml`.
- **Private rule content:** none. The asset names are the published product's own and already appear in §Consumer payload.

**Refused: a monitor over published Releases**, on the release channel's invariant-C precedent (installer/SPEC.md §The release channel). The newest published Release attaches its binary flat, one target with its sidecar, because the archive scheme came later. A monitor would need a roster of exempt Releases, a maintained copy that holds nothing. The publish half holds every Release the current workflow creates, and that set is the one the prose claims.

**Measured at authoring (2026-09-26):**

- `gh release view v0.25.0 --json assets` lists `checkwright-0.25.0.tgz`, `checkwright-0.25.0.tgz.sha256`, `checkwright-gates-x86_64-unknown-linux-gnu` and its `.sha256`. It lists no `.tar.gz`.
- In `publish.yml`, `pack` writes into `$out` the tarball `--pack-installer` moves there, which `npm pack` names from `installer/package.json`'s `"name": "checkwright"`, its `.sha256` sidecar, and one `$binary-$version-$target.tar.gz` per `gate_native_targets` line. `pack_installer.rs` writes nothing else into `--out`. The `release` job attaches every file of that directory. `pack` has built the gate binary by then.
- `native/targets.list` holds six targets. `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` defaults to `.github/workflows/publish.yml`, and `GATE_SDK_NATIVE_TARGETS_FILE` to `native/targets.list`.
- The front end forwards arguments to one gate with `--only <name> -- <arg>...`.

## What changes

### (1) §Consumer payload declares the set {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §Consumer payload, the paragraph opening "The Release publishes each target's binary and sidecar alongside the tarball as one archive" becomes the following paragraph, followed by the declaration line:

> **A Release carries the tarball, its sidecar, and one archive per roster target**, `checkwright-gates-<version>-<target>.tar.gz`, holding that target's binary and sidecar under their payload names, because Release assets are flat. Once extracted, the sidecar verifies with `sha256sum -c`, so no second spelling of the digest's subject is minted. The digest an installer checks therefore has a source outside the payload it travels in, and a digest shipped only beside its own artifact certifies nothing. The line below declares the set, and §check-release-assets holds each tag's set to it before its Release exists. A Release published before the line carries what its own tag's workflow attached.
>
> `<!-- release-assets: checkwright-{version}.tgz checkwright-{version}.tgz.sha256 checkwright-gates-{version}-{target}.tar.gz -->`

The declaration is written as a bare full line, not in a code span. The code span here only quotes it.

### (2) The gate's contract {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md gains `### check-release-assets` directly after §check-release-change-declared:

> **What a Release carries is declared once and held at the tag, before the Release exists.** §Consumer payload carries the declaration, a full-line `<!-- release-assets: <template>... -->` comment. Each whitespace-separated template names one asset. `{version}` stands for the tag's version and is required. A template also carrying `{target}` names one asset per target-roster line (`GATE_SDK_NATIVE_TARGETS_FILE`). No other placeholder, and no `/`, is admitted. The gate is consumer-declared in this repo's `scripts/` at `tier=precommit`, compiled like the release gates beside it.
>
> - **Battery half.** Exactly one declaration line in the declaring doc, every template well-formed, no two templates alike. The publish workflow (`GATE_SDK_NATIVE_PUBLISH_WORKFLOW`) carries a line invoking this gate with `--dist`. Each is a finding naming the doc or workflow and the remedy. A doc with no declaration is a finding, not a refusal, because nothing owning the claim is the defect this gate exists for.
> - **Publish half, `--dist <dir> <version>`.** The directory's entries, read non-recursively, equal the declaration expanded over `<version>` and the roster. It reports one finding per declared asset absent and one per entry declared by nothing, a subdirectory included. It runs in `publish.yml`'s `pack` job after the per-target archives are written. A red there fails `pack`, so the `release` job attaches nothing.
> - **Exit 2.** An absent or unreadable declaring doc or workflow, an unreadable `<dir>`, an empty `<version>` or one carrying whitespace or `/`, and a roster that resolves empty.
>
> **Argument mode (fixture capability):** `check-release-assets [<doc> <workflow>] [--dist <dir> <version>]`. With no positional, the doc is `gate-sdk/SPEC.md` and the workflow is the knob's. The fixture pair runs both halves in one case, with the roster pinned by the case's knob file. `good/` holds a declaration with a `{target}` template and one without, a workflow carrying the call, and a matching directory. `bad/` holds a second declaration line, a template missing `{version}`, a workflow with no call, a directory missing one target's archive, and an undeclared file. A declaring doc with no declaration, and each exit-2 path, are pinned by the module's unit tests, since a case can hold one verdict.
>
> **Honest limits.** The publish half runs only at a tag, so a declaration that drifts from `publish.yml`'s naming is red at that tag's `pack` job, not at the commit that moved it. That still stops before any Release exists, and the cost is a failed release run. The wiring check reads the call's text, not whether its step runs. A surface that spells an asset name for its own use, such as the install page's recipes or the hosted install scripts, is not compared with the declaration.

### (3) `pack` runs the publish half {mechanical}

**Not yet applied.** In `.github/workflows/publish.yml`, the `pack` job's "assemble the package from the tagged tree" step ends, after the per-target archive loop, with:

```bash
bash gate-sdk/bin/run-gates.sh --only check-release-assets -- --dist "$out" "$version"
```

### (4) The gate lands born native {design-bearing}

**Not yet applied.** Four pieces land together:

- A module `native/src/gates/release_assets.rs` carrying the rule and its unit tests, with a `REGISTRY` row declaring `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` and `GATE_SDK_NATIVE_TARGETS_FILE` and spawning no program.
- A descriptor `scripts/check-release-assets.gate`, with `# graph: couples=gate-sdk/SPEC.md,.github/workflows/publish.yml,knob:GATE_SDK_NATIVE_TARGETS_FILE dir=one valve=none tier=precommit`. Its `# spec:` line cites §check-release-assets.
- A `scripts/gates.list` line.
- The fixture pair `scripts/gate-tests/check-release-assets/{good,bad}` described in delta 2.

The target roster is read through the crate's one roster reader, never re-parsed.

## Producers and consumers

- **The declaration.** Producer: the maintainer editing §Consumer payload, a live tracked line and not a fixture-only one. Consumers: the battery half at every commit touching the doc, and the publish half at every tag. Each template is read at both, the battery half for its grammar and the publish half for its expansion.
- **The battery half.** Producer: the pre-commit hook and the battery, through the descriptor's `couples=`. Consumer: the committing contributor, through the output contract.
- **The publish half.** Producer: a `v*` tag push running `publish.yml`, whose `pack` job carries the call from delta 3. Consumer: the `release` job, which `needs: [build, pack]`, so a red leaves it unrun. Findings reach the tagging maintainer through the failed run.
- **Roster-holding readers of the minted names.** `scripts/gates.list`, the registry and its coverage tests, `check-gate-fixture-coverage`, and the generated hook and projections, which the build regenerates by their printed commands. The marker's comment grammar has no roster reader, as `payload-discloses:` has none beyond its gate.
- **Point 5.** No corpus narrows.
- **Point 6.** The obliged corpus is the declared templates expanded over the six `native/targets.list` targets, eight assets per tag. The tarball's satisfying value is `npm pack`'s output. Its sidecar's is the `sha256sum` line in `pack`. Each target's archive is the loop's `tar -czf`, whose name, `$binary-$version-$target.tar.gz`, expands `checkwright-gates-{version}-{target}.tar.gz` because `$binary` is the gate binary's basename, `checkwright-gates`.

**Inferred, cannot run before build:** that the fixture tree's empty `.tgz` and `.tar.gz` files pass every gate the case directory is exposed to — the files do not exist until build writes them, and the battery at build is the oracle.

## Existing sections updated

The roster comes from `git grep -n "checkwright-gates-<version>" -- ':!docs/'`, `git grep -n "release-assets\|check-release-assets"`, and `grep -n "tar -czf" .github/workflows/publish.yml`, all run 2026-09-26 over the tracked tree. The first returns the §Consumer payload paragraph and the Behavior-changes bullet in `.workflow/release-declarations.md` that announces the archive scheme to the next release note. That bullet describes the change and agrees with the declaration, so it is not a target. The second returns nothing.

- `gate-sdk/SPEC.md` §Consumer payload, the Release-archive paragraph and the declaration line (delta 1).
- `gate-sdk/SPEC.md`, the new §check-release-assets (delta 2).
- `.github/workflows/publish.yml`, the `pack` job's assemble step (delta 3).
- `native/src/gates/release_assets.rs`, `native/src/gates/mod.rs`, `scripts/check-release-assets.gate`, `scripts/gates.list` and `scripts/gate-tests/check-release-assets/` (delta 4).
<!-- update-target-exempt: generated projections, each regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `scripts/git-hooks/pre-commit`, and the enforcement map.

## Retired spellings

- None — the deltas add a declaration, a gate and a call, and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the declaration and both halves.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** Delta 1 re-phrases the paragraph it replaces. Delta 2 is a new section, since no section held the rule.
- [ ] **Born native.** Module, registry row, descriptor, `gates.list` line and fixture pair land in one commit, and `--run-gate-tests scripts/gate-tests` runs green.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `release-asset-claim-class-owner` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
