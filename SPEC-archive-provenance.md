# SPEC amendment: archive-provenance

Pairs `release-archive-provenance`. A Release's per-target archive carries its binary's sidecar inside it and nothing outside it, and only the tarball is an attestation subject. So a consumer taking the binary alone cannot check an archive before trusting a file inside that archive. This amendment gives every Release asset its own `.sha256` beside it on the Release and makes every asset other than a sidecar the subject of a build attestation. The `release` job verifies each before the Release exists, and `check-release-assets` holds the sidecars and the attestation subjects at the commit and the sidecars' content at the tag.

**What the tree does today**, each bullet with the probe that settled it:

- The one attest step names the tarball alone: `subject-path: ${{ runner.temp }}/dist/checkwright-*.tgz` (`grep -n 'subject-path' .github/workflows/publish.yml`, one hit).
- The release-assets line declares no archive sidecar: its templates are the tarball, its sidecar, the per-target archive, the companion archive and its sidecar (`grep -n '<!-- release-assets:' gate-sdk/SPEC.md`).
- The `pack` job carries one digest emission, the tarball's `sha256sum` in the assemble step. `scripts/ci-pack-extension.sh` hashes through `"${hasher[@]}"`, an expansion assertion F does not count (`grep -n 'sha256sum\|shasum\|hasher' .github/workflows/publish.yml scripts/ci-pack-extension.sh`). Assertion F reds a job whose count exceeds one (`grep -n 'j.computed > 1' native/src/gates/gate_substrate_parity.rs`), so a second `sha256sum` for the archives would red it.
- The `release` job verifies the tarball's attestation alone, with `--repo`, `--signer-workflow` and `--source-ref` (`grep -n 'gh attestation verify' -A2 .github/workflows/publish.yml`).
- The install page's attestation recipe prints `.statement.subject[0].name` in both twins (`git grep -n 'subject\[0\]' -- docs/manual-install.md`, two hits). A multi-subject attestation over the tarball would leave that first subject unspecified.
- `actions/attest-build-provenance` from v4 wraps `actions/attest`, whose `subject-path` takes a glob or a comma- or newline-separated list and mints **one** attestation naming every subject (the two actions' READMEs). *Inferred:* whether a glob matching no file fails the step; nothing in this design rests on it, since the `release` job's verify of every asset fails such a tag.
- The crate hashes a file in process (`grep -n 'pub fn file_hex' native/src/sha256.rs`) and matches a whole-string glob (`grep -n 'pub fn glob_match' native/src/walk.rs`).

## What changes

### (1) Every Release asset carries its sidecar and an attestation {design-bearing} {user-facing: a Release gains `checkwright-gates-<version>-<target>.tar.gz.sha256` beside each per-target archive and a build attestation over each archive (adopter direction 2026-11-27, lead-relayed: a per-archive sidecar plus the attestation). The companion archive is attested with them, reading the lead's "each archive in the attestation subject-path" (lead dispatch 2026-11-27, under the operator's delegation)}

**Not yet applied** — gate-sdk/SPEC.md §Consumer payload. The paragraph **One producer, two publications, and no recomputation anywhere.** becomes:

> **One producer per digest, and no recomputation anywhere.** A digest is computed once, in the job that produced the bytes it vouches for, and every later hop moves the file rather than re-deriving it. The build leg hashes the binary, which the payload carries and the Release's archive carries again. The `pack` job hashes what it cuts, the tarball and each per-target archive, at one site. A second `sha256sum` on a later job would let a published and an installed digest diverge while both look computed. §check-gate-substrate-parity assertion F holds the rule mechanically, **within its reach: the text of each workflow the knob names and the scripts its steps call, one hop deep.** A build body factored into a called script stays inside that reach. A body a second hop down, or a hasher reached through an expansion, rests on construction alone.

In the paragraph after it, "so a build attestation can later land beside these files with no migration and no digest changing" becomes "so the build attestation over the Release's assets, below, names the digest each sidecar carries, with no migration and no digest changing". Its other two sentences stand.

The paragraph **A Release carries the tarball, its sidecar, and one archive per roster target** becomes:

> **A Release carries the tarball and one archive per roster target**, `checkwright-gates-<version>-<target>.tar.gz`, holding that target's binary and sidecar under their payload names, because Release assets are flat. Each archive is drawn from the packed tarball's `payload/artifact/<target>/` rather than from the downloaded artifact, so it carries the bytes and the `0755` mode the packer placed, and the mode is still restored in the packer alone. A Release also carries the companion archive, which no payload reads.
>
> **Every asset carries its own sidecar beside it on the Release**, `<asset>.sha256` in the same format, so no second spelling of a digest's subject is minted. The sidecar inside an archive checks the binary once extracted, so the digest an installer checks has a source outside the payload it travels in. The one beside the archive checks the archive before it is opened, since a digest shipped only inside what it vouches for certifies nothing. **Every asset other than a sidecar is the subject of a build attestation the `pack` job mints**, and the `release` job verifies each against the publish workflow and the tag's ref before the Release exists. The tarball's attestation names the tarball alone, because the install page's attestation recipe prints an attestation's first subject as the tarball's. The other assets share a second. No install path reads an archive, so an archive's attestation is checked by the consumer who downloads it. The line below declares the set. §check-release-assets holds every asset's sidecar and attestation subject at the commit, and the set and each sidecar's content at the tag, before its Release exists. A Release published before the line carries what its own tag's workflow attached.

The declaration line becomes:

> `<!-- release-assets: checkwright-{version}.tgz checkwright-{version}.tgz.sha256 checkwright-gates-{version}-{target}.tar.gz checkwright-gates-{version}-{target}.tar.gz.sha256 checkwright-companion-{version}.zip checkwright-companion-{version}.zip.sha256 -->`

In **What the digest proves, at its honest bound.**, "the build attestation on the tarball (installer/SPEC.md §The dependency boundary) names which workflow built it" becomes "the build attestation on each asset (above, and installer/SPEC.md §The dependency boundary) names which workflow built it". In **The obligation opacity buys.**, "That is the shape the release uses for the tarball and the install page documents a reader verifying." becomes "That is the shape every Release asset's sidecar takes, and the one the install page documents a reader verifying."

**Not yet applied** — gate-sdk/SPEC.md §check-gate-substrate-parity, assertion F's third bullet. "no job may compute more than one digest, and a job that downloads a run artifact and uploads none may compute none. A step *computes* a digest when" becomes "no job may carry more than one digest emission, however many files that one emission hashes, and a job that downloads a run artifact and uploads none may carry none. A step *emits* a digest when". The crate counts emissions already, so no code moves.

### (2) The publish workflow cuts, attests and verifies each asset {design-bearing} {user-facing: the same direction as delta 1}

**Not yet applied** — `.github/workflows/publish.yml`:

- **The `pack` job's assemble step** drops the tarball's own `sha256sum` line. The per-target archive loop records each archive name it writes. After that loop, one loop over the tarball's name and the recorded names writes each file's `<name>.sha256` from inside `$out`, so each sidecar carries a bare name. That is the job's one emission, and it lands before the `check-release-assets --dist` call. Its `# spec:` line keeps the §Consumer payload citation the old line carried.
- **A second `actions/attest-build-provenance` step**, at the same pinned revision and directly after the tarball's, carries the `subject-path` block `${{ runner.temp }}/dist/checkwright-gates-*.tar.gz` and `${{ runner.temp }}/dist/checkwright-companion-*.zip`, one entry per line, under a `# spec:` line citing gate-sdk/SPEC.md §Consumer payload. The tarball's step is unchanged. The job's `id-token` and `attestations` permissions already cover both.
- **The `release` job** runs the verify it runs on the tarball, `--repo`, `--signer-workflow` and `--source-ref` unchanged, over every asset in its attach list not ending `.sha256`, once that list is complete and before `gh release list`. So a tag whose assets miss an attestation has no Release. The `-le 2` guard stays.

### (3) `check-release-assets` holds the sidecars and the attestation subjects {design-bearing}

**Not yet applied** — gate-sdk/SPEC.md §check-release-assets. The battery-half bullet becomes the first bullet below, and the second lands after **The wiring read.**:

> - **Battery half.** Exactly one declaration line in the declaring doc, every template well-formed, no two templates alike. **Every asset carries its sidecar**: each template not ending `.sha256` has the same template with `.sha256` appended declared beside it, and each `.sha256` template's stem is itself declared. **Every asset other than a sidecar is attested**: each template not ending `.sha256` is matched by an entry of some attest step's `subject-path` in a workflow `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` names. The publish workflow carries a line invoking this gate with `--dist`. Each is a finding naming the doc or workflow and the remedy. A doc with no declaration is a finding, not a refusal, because nothing owning the claim is the defect this gate exists for.
> - **The subject read.** An attest step is one whose `uses:` names `actions/attest-build-provenance` or `actions/attest`. Its `subject-path` is read under its `with:`, inline or as a `|` or `>` block indented past the key, the entries split on newlines and commas as the action splits them. A `#`-led line is no entry. An entry's last `/`-separated component is a glob, `*` crossing any byte, and it matches a template when it matches the template's own text, a placeholder standing as any run. An entry's directory part is not read.

The publish-half bullet gains, after its mode sentences:

> Each `.sha256` entry whose stem is also an entry, declared or not, holds exactly one line: sixty-four lowercase hex digits, two spaces or a space and `*`, and the stem's bare name. The digits are the stem's SHA-256, computed in process. A sidecar of another shape, naming another file or carrying another digest is a finding, since `sha256sum -c` beside the downloaded asset is the check its consumer runs.

The exit bullet gains: "An unreadable sidecar, or an asset it vouches for that cannot be read, is exit 3." The argument-mode paragraph's fixture roster becomes:

> `good/` holds a declaration with a `{target}` template and one without, each with its sidecar template, and a workflow carrying the call and two attest steps, one with an inline `subject-path` and one with a block, together naming every non-sidecar template. Its directory's archives carry their binary at `0755`, and every sidecar there holds its asset's digest. `bad/` holds a second declaration line, a template missing `{version}`, a template with no sidecar template, and a workflow with no call whose attest step names the tarball alone. Its directory is missing one target's archive and its sidecar and holds an undeclared file, an archive carrying its binary at `0644`, a sidecar carrying another digest and an undeclared sidecar naming a path. A declaring doc with no declaration, each exit-2 and exit-3 path, a comma-separated `subject-path`, the `actions/attest` spelling, an orphan sidecar template and a `#`-led entry are pinned by the module's unit tests, since a case can hold one verdict.

The honest-limits paragraph gains: "The subject read checks the step's text, not that the step runs or mints, and an entry naming the right file under the wrong directory passes here. Either fails the `release` job's verify at the tag, before the Release exists. That verify is held by no gate."

`native/src/gates/release_assets.rs` carries the two battery rules, the subject read over each named workflow, and the sidecar check through `crate::sha256::file_hex`. The clean line adds the count of sidecars verified and of attest steps read. The descriptor's one-line invariant names the sidecars and the attestation beside the asset set.

### (4) The fixtures and the unit tests {mechanical}

- `scripts/gate-tests/check-release-assets/good/` — `payload.md` gains `app-gates-{version}-{target}.tar.gz.sha256`, `publish.yml` gains the two attest steps, `dist/` gains each archive's sidecar, and `dist/app-0.1.0.tgz.sha256` holds the empty file's digest. `expect.txt` takes the new clean line.
- `scripts/gate-tests/check-release-assets/bad/` — `payload.md` gains `app-gates-{version}-{target}.tar.gz.sha256` and leaves the tarball's sidecar undeclared, `publish.yml` gains an attest step naming `app-*.tgz` alone, `dist/` gains the present archive's sidecar carrying another digest and an `app-0.1.0.tgz.sha256` naming `dist/app-0.1.0.tgz`, and `expect.txt` gains one line per new finding.
- `native/src/gates/release_assets.rs` unit tests for the cases delta 3 assigns them.

### (5) The prose surfaces {mechanical} {user-facing: the same direction as delta 1}

**Not yet applied** — installer/SPEC.md §The dependency boundary. The sentence "Both channels carry a build attestation: the `pack` job mints a GitHub artifact attestation on the tarball, and `npm publish --provenance` mints npm's on the package, so a reader can tell which workflow built either." becomes:

> Every channel carries a build attestation, so a reader can tell which workflow built what it took: `npm publish --provenance` mints npm's on the package, and the `pack` job mints GitHub's on every Release asset other than a sidecar, the tarball's alone and the archives' together (gate-sdk/SPEC.md §Consumer payload). A consumer taking the binary alone checks the archive it downloads before extracting it, with `sha256sum -c` against the sidecar beside it and `gh attestation verify <archive> --repo checkwright/checkwright --source-ref refs/tags/v<version>`.

**Not yet applied** — RELEASING.md §The procedure, step 5. "and the job then **attests** the tarball it assembled with a GitHub artifact attestation" becomes "and the job then writes one sidecar per archive it cut and **attests** what it assembled, the tarball in one GitHub artifact attestation and the archives in a second". "`release` checks that attestation before it creates the Release, then attaches the tarball, its `.sha256` and one versioned archive per target (binary and sidecar)" becomes "`release` checks every asset's attestation before it creates the Release, then attaches the tarball, one versioned archive per target (binary and sidecar) and the companion archive, each with its `.sha256`,".

**Not yet applied** — installer/README.md. "holds the gate binary and its sidecar alone, so on its own it runs" becomes "holds the gate binary and its sidecar alone, and from the first release after v0.41.0 the Release carries the archive's own `.sha256` and build attestation beside it. On its own it runs".

`.workflow/release-declarations.md`: the behavior section gains a bullet naming the archive sidecars, the archives' attestation and the `release` job's verify of every asset.

## Producers and consumers

- **The archive sidecars.** Producer: the `pack` job's one hashing loop, on every tag push. Consumers: `check-release-assets --dist` in the same job, which holds each one's presence and content; the `release` job, which attaches them; a consumer running `sha256sum -c` beside the downloaded archive.
- **The archives' attestation.** Producer: the second attest step, on every tag push, through the job's existing OIDC permissions. Consumers: the `release` job's verify loop; a consumer running `gh attestation verify`; the observe half `archive-provenance-observe` at the first tag after this lands. Each subject's `{name, digest}` has those readers.
- **The release-assets line's new template.** Producer: delta 1. Consumers: `check-release-assets`, both halves. `check-install-pin` invariant C reads the line at the pinned tag (`grep -n 'declaration' native/src/gates/install_pin.rs`), so it reads the new template only once the pin moves past the release carrying it, and no fetch surface names an archive (`git grep -n 'checkwright-gates-' -- docs/install.md docs/requirements.md docs/manual-install.md docs/maintenance.md docs/install.sh docs/install.ps1`, no hit).
- **The battery rules and the subject read.** Producer: the gate, at precommit and in the battery. Their red conditions: a template with no sidecar template, an orphan sidecar template, and a non-sidecar template no subject entry matches, each a finding.
- **Assertion F's count.** No producer moves: the assemble step's emissions go from one to one.
- **Point 6, every member.** The non-sidecar templates after delta 1 are the tarball, the per-target archive and the companion archive. Their sidecar templates are all declared. Their subject entries are `checkwright-*.tgz`, `checkwright-gates-*.tar.gz` and `checkwright-companion-*.zip` (`grep -n '<!-- release-assets:' gate-sdk/SPEC.md`).

## Existing sections updated

Roster probe: `git grep -n 'release-assets\|subject-path\|attestation verify\|checkwright-gates-\|attests\? the tarball\|attestation on the tarball' -- ':!docs/llms-full.txt' ':!docs/*/SPEC.md' ':!docs/posts' ':!TASK-QUEUE.md' ':!.workflow/survey-record.md'`, over the tracked tree, read hit by hit. The install-page and hosted-script hits name the tarball, whose attestation and sidecar do not move, and the guard-kit hits name the sidecar inside an archive, which does not move either.

- `gate-sdk/SPEC.md` — §Consumer payload's four passages and the declaration line (delta 1), §check-gate-substrate-parity assertion F's third bullet (delta 1), §check-release-assets (delta 3).
- `.github/workflows/publish.yml` — the assemble step, the second attest step and the `release` job's verify loop (delta 2).
- `native/src/gates/release_assets.rs` — the battery rules, the subject read and the sidecar check (delta 3), their unit tests (delta 4).
- `scripts/check-release-assets.gate` — the one-line invariant (delta 3).
- `scripts/gate-tests/check-release-assets/` — both cases (delta 4).
- `installer/SPEC.md` — §The dependency boundary (delta 5).
- `RELEASING.md` — §The procedure, step 5 (delta 5).
- `installer/README.md` — the per-target archive sentence (delta 5).
- `.workflow/release-declarations.md` — the behavior bullet (delta 5).
- `docs/gate-sdk/SPEC.md`, `docs/installer/README.md` — generated mirrors (all deltas).
- `docs/llms-full.txt` and `docs/requirements.md`'s install-size block — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; every asset, step and gate keeps its name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the sidecars, the attestation, the battery rules and the sidecar check.
- [ ] **Instruction surfaces: instruction only** — the workflow's `# spec:` lines name the step's rule, the grounds staying in §Consumer payload.
- [ ] **Merged with no information lost** — each passage is re-phrased in place, not appended to.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Done move** — `release-archive-provenance` moves to Done at the build that merges this, a stage before the drain stage; its observe half `archive-provenance-observe` stays deferred for the tag's `publish` run.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
