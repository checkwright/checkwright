# SPEC amendment: tarball-attestation

**The publish workflow mints a GitHub artifact attestation on the Release tarball. Both one-line install scripts check it where `gh` is present and signed in, refuse a tarball that fails it, and print a line where it went unchecked.** Today the tarball carries only its digest, which travels from the same origin as the tarball and proves transfer, not build. The tarball is the primary install path, and installer/SPEC.md §The dependency boundary and docs/install.md both state that it *cannot* carry an attestation. That is a missing workflow step, not a platform limit: `.github/workflows/publish.yml` mints an attestation only in the `npm` job, through `npm publish --provenance` (`grep -n "attest\|id-token\|provenance" .github/workflows/publish.yml`).

This is the **produce half**. The first published Release that carries the attestation, and an install from it that checks it, is `tarball-attestation-observed`, observed by `publish`.

**Measured at authoring**, gh 2.98.0:

- Signed out (`GH_CONFIG_DIR` set to an empty directory, `GH_TOKEN` empty), `gh attestation verify <file> --repo cli/cli` exits 4 and prints `To get started with GitHub CLI, please run: gh auth login`. `gh auth status` exits 1. `gh attestation verify --help` exits 0.
- Signed in, `gh attestation verify` over a file that has no attestation exits 1 with `HTTP 404: Not Found` from the attestations endpoint. That is the same status a failed check returns. So status alone cannot tell *this release was never attested* from *this tarball fails its attestation*, and the scripts need a version floor below which a release is known to be unattested.
- The pin trails the tag (§The hosted install pin). The pinned `0.31.0` and every older release carry no attestation. The scripts this unit ships are served from Pages on its first master push, still pinned to `0.31.0`.

## What changes

### (1) The publish workflow attests the tarball and checks it before the Release exists {design-bearing}

`.github/workflows/publish.yml`.

- **The `pack` job attests what it packs.** Its permissions gain `id-token: write` and `attestations: write`, beside `contents: read`. A step after *assemble the package from the tagged tree* runs `actions/attest-build-provenance`, pinned to a full commit SHA with its version as a trailing comment, the form `check-action-pinning` holds. Its `subject-path` is the one `checkwright-*.tgz` under `${{ runner.temp }}/dist`. The attestation names the job that produced the bytes, and the `npm` and `release` jobs move those same bytes on unchanged (gate-sdk/SPEC.md §Consumer payload, *One producer, two publications*).
- **The `release` job checks it before it creates the Release.** Its permissions gain `attestations: read`. Before the Release-creating invocation, its step runs `gh attestation verify "dist/$name" --repo "$GH_REPO" --signer-workflow "$GH_REPO/.github/workflows/publish.yml"`, reusing the step's existing `GH_TOKEN`. A failure exits 1 before any Release exists. A red publish is then fixed and the tag re-pushed (RELEASING.md step 5), so no Release ships a tarball that fails its own attestation.

The per-target archives and the companion archive are not attested. The gate binary inside the tarball is covered by the tarball's subject, and by its own digest checked at install (§The gate binary).

**Inferred, cannot run before build:** that the permission sets above let the action mint, and the `release` job's token read, the attestation — only a tag-triggered run produces either, and `tarball-attestation-observed` reads that run.

### (2) The one-line install checks the attestation {design-bearing} {user-facing: the deliverable recorded on tarball-build-attestation, returned from the icebox by consult as a trust gap on a public claim — the shell installers verify it where the verifier is present and say so where it is not}

`docs/install.sh` and `docs/install.ps1`, twins, after the digest check and before the extract.

- **The floor.** Each script carries one floor line below its pin line: `attest_from='X.Y.Z'` in the sh script, `$attestFrom = 'X.Y.Z'` in the PowerShell one. Its value is the **patch successor of the newest tag at the commit landing this delta**, which is `0.31.1` at authoring. Every later release is cut from master, which carries delta 1, so every release at or above that floor is attested, and every one below it is not. The value never moves after landing. The version being installed, the pin or `CHECKWRIGHT_VERSION`, is compared with the floor over its dotted digit runs, and a version that does not parse is compared as attested, so the check fails closed.
- **The verifier is present** when all three hold:
  - `gh` is on `PATH`;
  - `gh attestation verify --help` exits 0, so this gh has the subcommand;
  - `gh auth status --hostname github.com` exits 0, so gh can reach the attestations API.
- **The four outcomes.** Each prints one line to stderr under the script's `checkwright install:` prefix. Only the fourth stops the install.
  1. **Below the floor:** `v<version> predates build attestation; checked against its digest only`, and the install proceeds.
  2. **No verifier:** `build attestation not checked (gh is not installed or not signed in); to check it: gh attestation verify checkwright-<version>.tgz --repo checkwright/checkwright`, and the install proceeds.
  3. **Checked:** `build attestation verified`, and the install proceeds.
  4. **Refused:** `gh attestation verify <tgz> --repo checkwright/checkwright --signer-workflow checkwright/checkwright/.github/workflows/publish.yml` fails. gh's own output is relayed, then `verify failed: <tgz> carries no build attestation from checkwright/checkwright's publish workflow`. The sh script exits 2 and the PowerShell one throws, the way each refuses a digest mismatch today.
- **What the PowerShell twin must do.** It reads gh's status from `$LASTEXITCODE`, never from a thrown error (§The dependency boundary, *Truncation and status*). It keeps every added byte ASCII, which `check-portability-floor`'s ASCII arm holds.
- **No new override.** `CHECKWRIGHT_RELEASE_BASE` moves the download and not the verification, which always names this repository. A release served from elsewhere is still checked against this repository's workflow.

### (3) check-install-pin holds the twins' floors equal {design-bearing}

`native/src/gates/install_pin.rs`, with its fixtures under `scripts/gate-tests/check-install-pin/`.

Invariant A widens. Each script carries exactly one pin line and exactly one floor line of its grammar. The two pins are equal, and so are the two floors. A floor not of the form `<major>.<minor>.<patch>` exits 2, as a pin does. A missing or duplicated floor line exits 2 too. The floor is not compared with the pin or the tags: it is fixed at landing, and at landing it sits above the pin.

Fixtures:

- `good/`'s two scripts carry equal floors.
- `bad/` gains a case whose floors disagree, with its `expect.txt` line.
- A missing floor line is pinned by a unit test, because a case can hold only one verdict.

### (4) The CI legs witness each outcome {design-bearing}

`.github/workflows/gates.yml`, the one-liner steps of `install-smoke-sh-linux` and `install-smoke-pwsh-windows`.

- **The existing runs** serve the packed tarball at its own version. That version is below the floor, so they now assert outcome 1's line as well as the status and the lock.
- **A second served release.** Each leg serves the same tarball and a recomputed digest under a version above the floor, `99.0.0`. The scripts read the version only to name files, and the bootstrap reads its own stamp. Against it, the leg runs three arms in fresh consumers:
  - **A stub `gh` that passes** every call, first on `PATH`: assert outcome 3's line, exit 0 and a written lock.
  - **A stub whose `attestation verify` fails**: assert a non-zero exit, or a throw caught by the PowerShell step, with outcome 4's line and no `checkwright.lock`.
  - **With `PATH` as the runner sets it, and no `GH_TOKEN`**, which no step in this workflow sets: assert outcome 2's line and a written lock.
- **The stubs.** On Linux the stub is a POSIX `sh` script. On Windows it is a `gh.cmd` the PowerShell twin's `Get-Command` resolves.

These legs run only on a push. This is the unit's mid-iteration push need, recorded on its queue entry. The real `gh attestation verify` against a published Release is `tarball-attestation-observed`'s.

**Inferred, cannot run before build:** that the hosted Windows and Linux runners resolve the stub ahead of a preinstalled `gh` once the stub's directory leads `PATH` — the legs run only on a push.

### (5) The install page states the attestation {mechanical}

docs/install.md. **Not yet applied.**

- **§Install's opening paragraph.** Its second sentence becomes *The line downloads the newest Release tarball, checks it against its published digest and, where `gh` is installed and signed in, against its build attestation, unpacks it outside your repository and runs `init`, with no runtime to install first.*
- **Each system's step-by-step form.** After the download fence comes one sentence and one fence, outside the marked install block, so no leg runs it. The sentence: *Where `gh` is installed and signed in, check the build attestation too.* The fence holds `gh attestation verify "$cw/checkwright-$v.tgz" --repo checkwright/checkwright`, or its PowerShell spelling with `"$cw\checkwright-$v.tgz"`.
- **The prerequisites block** gains one row. Its cells are:
  - Tool: `gh`.
  - Minimum: *any carrying `gh attestation`*.
  - Needed for: *optional: to check the build attestation*.
  - Why: *the one-line install runs `gh attestation verify` when `gh` is signed in, and says so when it is not*.
- **§With Node.** Its first sentence becomes *`npx checkwright init` runs the same `init` from the npm package, which carries npm's provenance attestation as the tarball carries a GitHub build attestation ([installer/SPEC.md](installer/SPEC.md#the-dependency-boundary)).*

### (6) The owning sections state the rule {mechanical}

**Not yet applied.**

installer/SPEC.md:

- **§The dependency boundary, the paragraph *The tarball is the primary path*.** Its last two sentences become *Both channels carry a build attestation: the `pack` job mints a GitHub artifact attestation on the tarball, and `npm publish --provenance` mints npm's on the package, so a reader can tell which workflow built either.*
- **§The dependency boundary, *The one-line install*.** It gains a bullet, *The attestation check*. The bullet states:
  - delta 2's floor, the three-part presence test and the four outcomes;
  - that the check names this repository whatever `CHECKWRIGHT_RELEASE_BASE` says;
  - that a release below the floor, or a host with no signed-in `gh`, is checked against its digest alone and is told so.

  Its honest limit: the check proves that this repository's publish workflow built the bytes. It does not prove that the workflow or the tagged tree was sound.
- **§The dependency boundary, *The checksum's honest limit*.** Its last sentence becomes *The property that carries that is the build attestation, which the one-line install checks where `gh` is signed in.*
- **§Requirements, *The one-line witness*.** It gains the second served release and its three stubbed arms (delta 4), and outcome 1's assertion on the existing runs.
- **§The CI action, *Honest limits*.** It gains: *The Action sets no `GH_TOKEN` for the script, so its install checks the digest alone and prints that the attestation went unchecked.*
- **§The hosted install pin.** Invariant A reads *the twins agree: each script carries exactly one pin line and one floor line of its grammar, and the two pins are equal, as are the two floors.* The fail-closed paragraph gains the floor line's grammar and absence.

gate-sdk/SPEC.md:

- **§Consumer payload, *What the digest proves, at its honest bound*.** Its third sentence becomes *What that floor does not provide is a reproducible build: the build attestation on the tarball (installer/SPEC.md §The dependency boundary) names which workflow built it, and a reproducible build stays the outstanding ground.*

RELEASING.md:

- **Step 5.** The sentence on the jobs gains: the `pack` job attests the tarball it assembled, and `release` checks that attestation before it creates the Release.

### (7) The release declaration and the mirrors {mechanical}

- `.workflow/release-declarations.md` gains a Behavior changes bullet. Its lead is **docs/install.sh, docs/install.ps1**, followed by: the one-line install now checks the Release tarball's build attestation where `gh` is installed and signed in, refuses one that fails it, and says so where it went unchecked; nothing to do.
- `docs/installer/SPEC.md` and `docs/gate-sdk/SPEC.md` are regenerated with the command their freshness gate prints, in the commit landing delta 6.

## Producers and consumers

- **The attestation.**
  - Producer: the `pack` job's attest step, on every `v*` tag push. Its enabling configuration is the workflow's own permission block, which every tag run uses.
  - Consumers: the `release` job's check (delta 1); the scripts' check (delta 2), by the GitHub attestations API through `gh`; and a reader running the page's fence (delta 5).
- **The floor line.**
  - Producer: the commit landing delta 2.
  - Consumers: each script's comparison, and `check-install-pin` invariant A.
  - The pin line's readers, by `grep -rln -e "pin='" -e 'pin = ' native/src scripts docs/install.sh docs/install.ps1 installer`, are `install_pin.rs`, `plugin_parity.rs` and `pinned_release.rs`. The last two read the pin through `install_pin::pin_of(path, text, "pin")`, which matches a line opening with the given name and then `=`. A floor line opens with `attest_from` or `$attestFrom`, so it is never a second pin line, and those two readers are unchanged. Delta 3 reads the floor through the same `pin_of`, with the floor's name.
- **The four outcome lines.**
  - Consumers: the adopter, and the CI legs (delta 4).
  - The installed Action runs `docs/install.sh` (installer/action.yml). It reaches outcome 1 or 2, never 4, because no step of it sets a token.
- **Fields:** none. Point 4 is vacuous.
- **Red conditions (point 5):** no corpus narrows.
  - Invariant A gains a floor arm. It reds when the twins' floors disagree, and exits 2 on a missing or malformed floor line.
  - The one-liner legs gain assertions. Each reds when its arm's line or status is absent.
  - The `release` job reds a tag whose tarball fails its attestation.
- **Every member's satisfying value (point 6):** the two fetch scripts, each satisfied by its floor line and the check. The page's fences carry no check of their own; the page states it.

## Existing sections updated

Roster by `git grep -n -i -e "attestation" -e "--provenance" -- ':!TASK-QUEUE.md' ':!docs/posts' ':!docs/*/SPEC.md'` filtered to the release and install surfaces, by `grep -n "install.sh\|install.ps1" .github/workflows/gates.yml`, and by `grep -rln "pin='" native/src`, over the tracked tree.

- `.github/workflows/publish.yml` — the `pack` job's attest step and permissions; the `release` job's check and permission (delta 1).
- `docs/install.sh` — the floor line and the check (delta 2).
- `docs/install.ps1` — the floor line and the check (delta 2).
- `native/src/gates/install_pin.rs` — invariant A's floor arm and its unit test (delta 3).
- `scripts/gate-tests/check-install-pin/` — the floor fixtures (delta 3).
- `.github/workflows/gates.yml` — the two one-liner steps' arms (delta 4).
- `docs/install.md` — §Install, the two step-by-step forms, the prerequisites block and §With Node (delta 5).
- `installer/SPEC.md` — §The dependency boundary, §Requirements, §The CI action and §The hosted install pin (delta 6).
- `gate-sdk/SPEC.md` — §Consumer payload, one sentence (delta 6).
- `RELEASING.md` — step 5, one sentence (delta 6).
- `.workflow/release-declarations.md` — the Behavior changes bullet (delta 7).
- `docs/installer/SPEC.md` — the regenerated mirror (delta 7).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 7).

## Retired spellings

- None — the retired claim is a sentence, not a name, and delta 5 and 6 rewrite each site that states it.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — no template or agent definition is touched.
- [ ] **Merged with no information lost** — §The dependency boundary states the attestation check whole, and no surface still says the tarball cannot carry one (`git grep -n -i "attestation the tarball cannot\|this channel does not"`).
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`), discharged at the iteration while sibling installer amendments are in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery, this repo's withheld-gate fixture suite for `check-install-pin`, `shellcheck` over `docs/install.sh`, and the mid-iteration push's `install-smoke-sh-linux` and `install-smoke-pwsh-windows` legs watched green.
- [ ] **The entry is done** — `tarball-build-attestation` moves to Done once that push's legs are green, before the drain stage. Its merge commit bridges the push with a `[spec:]` path ref to installer/SPEC.md (canon-kit/SPEC.md §Merging an amendment step 4).
