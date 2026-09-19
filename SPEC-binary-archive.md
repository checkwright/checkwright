# SPEC amendment: binary-archive

Each target's gate binary reaches the GitHub Release as two flat assets,
`checkwright-gates-<target>` and `checkwright-gates-<target>.sha256`, with no
release version in either name. The sidecar's content names the bare
`checkwright-gates`, which is the payload name. That is right inside the payload
and wrong beside the renamed Release asset. This amendment ships each target as
one versioned archive, `checkwright-gates-<version>-<target>.tar.gz`, holding the
binary and its sidecar under their unchanged names. The same edit fixes a pack
step that cannot find the Windows artifact.

This is the form the operator put forward (2026-09-14, "to consider"). The
iteration admits it by operator direction. Shipping it in that form is inside the
entry's envelope. What the probes below add is a trust ground for it: today the
out-of-band cross-check the design record promises fails as published.

**Probed at authoring, 2026-09-19, at `afebd4bb`:**

- **The published pair does not verify.** `gh release download v0.25.0 -p
  'checkwright-gates-x86_64-unknown-linux-gnu*'` gives a sidecar reading
  `<hex>  checkwright-gates`. `sha256sum -c` on it exits 1 with
  `checkwright-gates: FAILED open or read`, because the downloaded binary is
  named `checkwright-gates-x86_64-unknown-linux-gnu`. The same pair, archived in
  delta 1's layout, extracted and checked from inside its directory, reports
  `checkwright-gates: OK`. installer/SPEC.md §The gate
  binary's honest bound ("a human can cross-check the value out of band") and
  gate-sdk/SPEC.md §Consumer payload ("the digest an installer verifies against
  has a source outside the payload") both rest on that cross-check.
- **No installer reads a Release asset.** Both bootstraps select and verify out of
  the package's own `payload/artifact/<target>/`
  (`installer/bin/checkwright.sh` `select_artifact`, `checkwright.ps1`
  `Select-Artifact`). `grep -n -i "release\|github\|curl\|download"` over both
  finds only the re-download remedy text. So the deferred entry's reach claim
  (the bootstraps' asset lookup) was wrong. The archive touches the publish
  path and prose only, and `floor-bash-install-bootstrap` does not couple to it.
- **The pack step cannot find the Windows artifact.** `publish.yml`'s assemble
  step takes `binary="$(gate_native_bin)"; binary="${binary##*/}"` on its
  `ubuntu-latest` runner, which gives `checkwright-gates`. It then looks for
  `$art/<target>/$binary` for every roster target. The Windows build leg writes
  `checkwright-gates.exe` (`scripts/ci-build-artifact.sh:36`, the suffix from
  `gate_exe_suffix "$target"`). `x86_64-pc-windows-msvc` has been a live
  `native/targets.list` line since `acbf32e8` (2026-09-08), and the last tag,
  `v0.25.0`, is older. So the next tag's pack job exits 1 on
  `no downloaded artifact for target 'x86_64-pc-windows-msvc'`. The same `$binary`
  drives the Release rename loop this amendment rewrites.
- **Every reader of the per-target Release names**, found with
  `git grep -n -F -e 'binary-$target' -e per-target -e 'gates-<target>'` and a
  read of each hit: `.github/workflows/publish.yml` (the pack job's rename loop
  and comment, the release job's comment); gate-sdk/SPEC.md §Consumer payload
  (the Release paragraph); `RELEASING.md` §The procedure step 5; installer/SPEC.md
  §The gate binary (the honest bound). `SECURITY.md`'s "published per-target
  digest" stays true unchanged.
- **One digest producer per job holds only without an outer sidecar.**
  gate-sdk/SPEC.md §check-gate-substrate-parity assertion F lets the `pack` job
  compute one digest, and it already computes the tarball's. A `sha256sum` of
  each archive there would be a second producer and would go red.

## What changes

### (1) The pack job writes one versioned archive per target {design-bearing}

In `publish.yml`'s assemble step, the Release loop replaces the two flat `cp`s.
For each roster target it:

- resolves the target's own file name as
  `name="${binary%.exe}$(gate_exe_suffix "$target")"`, the derivation
  `scripts/ci-build-artifact.sh` already uses. The owner is `gate_exe_suffix` in
  `gate-sdk/lib/gate.sh`, which the step already sources;
- copies `$art/$target/$name` and `$art/$target/$name.sha256` into a staging
  directory `checkwright-gates-<version>-<target>/` under `$RUNNER_TEMP`. The
  directory name uses `${binary%.exe}` and the step's own `version`, the tag
  without its `v`, which also names `checkwright-<version>.tgz`;
- writes `$out/checkwright-gates-<version>-<target>.tar.gz` with
  `tar -czf … -C <staging> <dir>`.

The normalize loop above it (the three-way `-f` test) takes the same per-target
`name`, so the Windows artifact is found. That closes the pack defect in the step
it lives in. The archive holds one top-level directory named like the archive, so
extracting two releases side by side never collides and the version stays
visible after extraction. The binary and sidecar keep their payload names, so the
sidecar verifies in place:
`tar -xzf checkwright-gates-X.Y.Z-<target>.tar.gz && cd checkwright-gates-X.Y.Z-<target> && sha256sum -c checkwright-gates.sha256`.
That is the cross-check the design record promises, and today it fails.

**Four alternatives are refused:**

- **An outer `.sha256` per archive.** It is a second digest producer in `pack`
  (assertion F). It would also add nothing: the inner sidecar is the published
  digest, and a corrupted archive fails `gzip` or that check.
- **Putting the version in the flat asset names.** It keeps the sidecar's bare
  name disagreeing with its neighbour's, which is the defect in the first probe.
- **Using `.zip` for the Windows target.** One format keeps one loop, and Windows
  10 and later ship `tar`.
- **Renaming the binary or its sidecar.** The entry refused it already, because
  every vendored reference and the upgrade path key on those names.

**Inferred, cannot run before build:** a stock Windows 10 or later host extracts the `.tar.gz` with its bundled `tar.exe` — no Windows host is reachable from the authoring session, and the publish path runs only on a tag; the release that carries this delta settles it for whoever downloads the Windows archive.

### (2) The release job attaches the archives {mechanical}

The release job's asset loop is unchanged: it takes every file in `dist/` except
the tarball and its sidecar, and it still refuses when none arrived. That set is
now the archives. Its comment changes from "They arrive already renamed for their
target" to "They arrive archived per target and version, the binary and its
sidecar inside under their payload names". The pack job's comment above the loop
is rewritten to match: Release assets are flat, so each target's pair travels as
one archive named for version and target, and the sidecar keeps the name it
verifies against.

### (3) The prose that names the Release shape follows {mechanical}

**Not yet applied.** gate-sdk/SPEC.md §Consumer payload. "The Release publishes
the per-target binaries and their sidecars alongside the tarball, renamed per
target because Release assets are flat. The **content** of each sidecar is left
alone — its bare filename is the name the file carries in the payload, where the
machine verification happens, and rewriting it would mint a second spelling of
one published fact." becomes "The Release publishes each target's binary and
sidecar alongside the tarball as one archive,
`checkwright-gates-<version>-<target>.tar.gz`, because Release assets are flat.
Inside it both files keep their payload names, so the sidecar verifies with
`sha256sum -c` once extracted and no second spelling of the digest's subject is
minted." The sentence after it, on what the publication buys, is unchanged.

**Not yet applied.** installer/SPEC.md §The gate binary, the honest bound: "the
identical bytes are published on the Release, so a human can cross-check the value
out of band" becomes "the identical bytes are published on the Release, in a
versioned archive per target whose sidecar verifies with `sha256sum -c` once
extracted, so a human can cross-check the value out of band".

**Not yet applied.** `RELEASING.md` §The procedure, step 5: "`release` attaches
the tarball, the per-target binaries and every `.sha256` to the GitHub Release"
becomes "`release` attaches the tarball, its `.sha256` and one versioned archive
per target (binary and sidecar) to the GitHub Release".

**Not yet applied.** `.workflow/release-declarations.md` `## Behavior changes`
gains: "**GitHub Release assets** — each target's gate binary now ships as
`checkwright-gates-<version>-<target>.tar.gz`, holding `checkwright-gates` (or
`.exe`) and its `.sha256`, in place of the flat `checkwright-gates-<target>` pair.
Extract the archive and run `sha256sum -c checkwright-gates.sha256` inside it to
cross-check the digest the installer verified."

## Producers and consumers

- **The archive.** The producer is the `pack` job (delta 1), on a tag push only.
  The consumers are the `release` job's asset loop (delta 2) and a human
  cross-checking a digest out of band. No installer, smoke or gate reads it
  (second probe). Its one field that matters is the name's `<version>`, and the
  reader of that is the human the entry's cost names. The inner sidecar's reader
  is `sha256sum -c`.
- **The per-target `name`.** It is read by the normalize loop and the archive
  loop in the same step. `gate_exe_suffix` stays its only owner.
- **Point 5 (narrowing).** The Release asset set loses the flat pairs. The
  release job's only red condition on that set is "no file besides the tarball
  and its sidecar", and every archive is such a file, so the condition still fires
  on exactly an empty per-target set.
- **The oracle.** No pushed `gates` run reaches `publish.yml`, so build runs the
  assemble step's loops locally against a staged `<art>/<target>/` set for every
  roster target, the Windows target's `.exe` included. Each archive must come out
  and verify with `sha256sum -c` inside it. The first tag push is the final proof:
  watch `pack` and `release` to green, then download one archive and verify it.

## Existing sections updated

The roster's probe is the reader survey in the fourth probe bullet above.

- `.github/workflows/publish.yml`: the pack job's normalize loop, archive loop and comment (delta 1); the release job's comment (delta 2).
- `gate-sdk/SPEC.md`: §Consumer payload, the Release paragraph (delta 3).
- `installer/SPEC.md`: §The gate binary, the honest bound (delta 3).
- `RELEASING.md`: §The procedure, step 5 (delta 3).
- `.workflow/release-declarations.md`: the Behavior-changes bullet (delta 3).
- `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md`: the generated mirrors, regenerated (delta 3).

## Retired spellings

- None — no delta renames a tracked token. The flat asset names were never spelled literally in the tree; the workflow composed them as `$binary-$target`, which delta 1 rewrites in place.

## Definition of Done

- [ ] **Causal completeness**: every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only**: replacement text for a
      template, agent definition or shim carries no grounds, and a delta places
      them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost**: each addition re-phrases the
      canonical-spec text it refines rather than appending to it, and the merged
      spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted**: this file is removed on merge, and none remain at the
      repo root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated**: every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed**: cross-component gaps found during the work are filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Proved locally, then on the tag**: the staged-artifact run in §Producers
      and consumers is green, the Windows `.exe` included. The tag run is the
      release procedure's own watch (`RELEASING.md` §The procedure, step 5).
