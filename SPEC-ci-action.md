# SPEC amendment: ci-action

The CI story is a copy-out template. `gate-sdk/templates/gates-workflow.yml` is copied by hand, `init` does not seed it, and a red never reaches the pull request as an annotation. The template also runs the binary committed in the tree, which is the installing host's target (installer/SPEC.md §The gate binary). So a tree installed on macOS commits an artifact a Linux runner cannot execute, and the copied workflow fails on its first run.

This amendment ships three things. A composite Action in this repository, `installer/action.yml`, runs the battery in one step. `init` seeds a workflow calling it. The runner writes its verdict as SARIF, which the Action uploads so each red becomes a pull-request annotation.

**The ruling: the Action runs the published binary for the runner, at the version the lock names.** It runs `docs/install.sh` from its own checkout with `CHECKWRIGHT_VERSION` set to the tree's `checkwright.lock` version and `--run` as the argument. The hosted script fetches that release from the Release host and verifies its digest. The bootstrap then selects the runner's artifact and executes `--run` in the tree. No host-to-target map, download recipe or digest check is copied into the Action, and the binary follows `update` with no edit to the workflow. Because the committed binary is never executed, substituting it cannot turn the check green.

**The ruling: SARIF is a reporting knob on the runner, not a second arm.** `GATE_SDK_SARIF_FILE` names a file the `--run` arm writes from its own in-memory outcomes after the join. gate-sdk/SPEC.md §run-gates already rules that how the battery reports is environment and what it runs is argv, and `GATE_SDK_VERBOSE` and `GATE_SDK_JOBS` are the precedents. The Action fetches the release once per binary execution, so an `--emit sarif` arm reading a saved transcript would cost a second download and a second parser of output the runner already holds.

**The seam.** Kit mechanism: the `GATE_SDK_SARIF_FILE` knob and the kit smoke's reading of it, which name no project, and gate-sdk's pointers to the seed. The installer, repo-root-governed and not a kit, carries the Action and the seed. The seed's repository slug is derived at `init` from the package's own `repository`, so no kit literal names this project. No new consumer config.

**Refused: the Action runs the fixture suites too.** Each suite is a separate binary execution, so each would be a separate download. The release the lock names was tagged only after a `gates` run on its commit passed every suite (RELEASING.md step 4). A consumer's own gates and runners stay later steps of its own workflow, in the shape the copy-out template shows.

**Refused: the Action sits at the repository root.** A root `action.yml` would be a new root surface and a new component. An action in a subdirectory is still one `uses:` line. `installer/` is already the published activation surface in front of the binary, and its `files` roster keeps the file out of both packages.

**Refused: the seed is rewritten on every run.** An adopter extends a workflow with their own steps, so the seed is create-once. That also leaves a workflow they already keep at the path untouched. Only the Action's steps age, and the binary follows the lock.

**Measured at authoring:**

- **The committed binary is the installing host's.** installer/SPEC.md §The gate binary places the artifact the bootstrap selected for the host, and its `own` record joins the staged set, so it is committed.
- **One binary execution runs the whole battery.** `native/src/runner.rs` dispatches every `.gate` member through `current_exe()`. `docs/install.sh` ends `sh "$cw_dir/package/bin/checkwright.sh" "$@"`, and the bootstrap forwards a dashed leading token verbatim (installer/SPEC.md §The install boundary, step 5).
- **The environment outranks a knob file** (gate-sdk/SPEC.md §The knob file). `native/src/knobs/gate_sdk.rs` lists the environment-only names as `GATE_SDK_GATES_DIR`, `GATE_SDK_ROOT`, `GATE_SDK_JOBS` and `GATE_SDK_VERBOSE`.
- **The lock's shape.** `native/src/installer/lock.rs` writes `serde_json::to_string_pretty` over a BTreeMap, so the top-level `version` member is one line at two-space indent, `  "version": "X.Y.Z",`. No nested object carries a `version` key.
- **The seed's two identities are in the package.** `installer/package.json` carries `repository.url` `git+https://github.com/checkwright/checkwright.git`, and `init` already reads `checkwright.commit`, the pack-stamped commit (`native/src/installer/init.rs`, `read_package_field`).
- **Every workflow gate is `on-surface`.** `grep -n "^# install:" gate-sdk/checks/check-action-*.gate` returns `on-surface` for pinning, permissions, run-shell, gh-repo, run-path and job-ref. §check-action-run-path grounds that on workflows "which `init` never writes", and this unit falsifies that ground.
- **gate-sdk/SPEC.md §Consumer smoke is the kit smoke** (`--run-consumer-smoke`), which fires one violation per kit and asserts a `FAIL:` line naming the gate. `init` is exercised by installer/SPEC.md §The consumer smoke instead.

**Inferred, cannot run before build:** GitHub resolves `uses: <owner>/<repo>/installer@<sha>` to `installer/action.yml` at that commit — no run of this repository executes the remote form, so an adopter's run is its first.

**Inferred, cannot run before build:** a composite step honours `continue-on-error` and `if: always()`, and a `uses:` step's `env:` reaches the composite's `run:` steps — only a remote run executes a composite action; the mid-iteration push's witness runs both through `uses: ./installer`.

**Inferred, cannot run before build:** `upload-sarif` needs `security-events: write`, and on a private repository possibly `actions: read` as well — only a remote upload settles the scope set; the `gates` job's upload step runs it on this public repository.

## What changes

### (1) The runner writes SARIF on request {design-bearing}

**Not yet applied.** In gate-sdk/SPEC.md §run-gates, after the `GATE_SDK_JOBS` paragraph:

> **`GATE_SDK_SARIF_FILE` is the third reporting knob.** Where the invoking environment sets it, the `--run` arm writes a SARIF 2.1.0 log of its verdict to that path once after the join, in registry order, and the exit status is unchanged. It is environment-only, like the worker count. One run holds one tool, `checkwright-gates`, with one rule per red member. The rule's id is the member name, its short description is the invariant the red path prints beneath the verdict, and its help URI is the published location where `GATE_SDK_SPEC_BASE_URL` is set. Each line of a red member's captured output that opens `<path>:<line>:` (or `<path>:<line>:<col>:`), where `<path>` is repo-relative and names a regular file, is one `error` result at that region, its message the line. A red or unresolved member with no such line is one result at the registry line that names it, its message the captured block. A green run writes the log with an empty result set, so a code-scanning reader closes what the previous run opened. A path the arm cannot write is exit 2, naming it, after the verdict lines print. **Honest limit:** a result's location is only as exact as the member's finding line. A finding naming its file in another shape lands at the registry line.

In §Layout and configuration's *Execution* list, after `GATE_SDK_VERBOSE`:

> - `GATE_SDK_SARIF_FILE` (default unset = none written): the SARIF log `--run` writes (§run-gates). Environment-only, because it is how the battery reports.

In `native/src/knobs/gate_sdk.rs`, the name joins `env_only`. In the crate, a `sarif` module renders the log from the runner's outcomes with `serde_json`, which is already a dependency. `native/src/runner.rs` calls it after `write_timings`, and the member's registry line is carried from the list read. Unit tests cover the location reader (each shape, a non-file path, a path with no line), the registry-line fallback, the empty log, and the unwritable path. The knob roster projection is regenerated by the command its freshness gate prints.

### (2) The kit smoke reads the SARIF log {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §Consumer smoke, the violation sentence ("Per kit shipping `smoke/violation.sh` it fires one crafted violation…") ends, instead of "…and asserts a non-zero exit **and** a `FAIL:` line naming the expected gate":

> …and asserts a non-zero exit, a `FAIL:` line naming the expected gate, and a result with that gate's rule id in the SARIF log the run wrote under `GATE_SDK_SARIF_FILE`.

The `--run-consumer-smoke` arm sets the knob to a file in its scratch on each violation run and reads the log with `serde_json`.

### (3) The CI action {design-bearing}

**Not yet applied.** `installer/action.yml`, a composite action:

- **Inputs.** `path`, the installed tree relative to the workspace, default `.`. `version`, a release to run in place of the lock's, for a tree vendored without the installer. `upload-sarif`, default `true`.
- **Output.** `sarif`, the log's path under `$RUNNER_TEMP`.
- **The battery step.** It has `shell: bash` and `working-directory: ${{ inputs.path }}`, and takes the version from the input or from the lock's top-level `version` line. With neither, it exits 2 naming the `version` input. It then runs `GATE_SDK_SARIF_FILE="$sarif" CHECKWRIGHT_VERSION="$v" sh "$GITHUB_ACTION_PATH/../docs/install.sh" --run`.
- **The upload step.** `github/codeql-action/upload-sarif`, pinned to a full commit SHA with its tag as a trailing comment, under `if: always() && inputs.upload-sarif == 'true'` and `continue-on-error: true`.

`scripts/core-files.list` registers the file beside the workflow template. installer/SPEC.md §Layout gains:

> - `action.yml` — the CI action (§The CI action). It sits in this directory because the activation surface does, and it is off the `files` roster, so neither package carries it.

installer/SPEC.md gains a section before §Profiles:

> ## The CI action
>
> `installer/action.yml` is a composite GitHub Action. A workflow runs the battery of a tree `init` installed with one step, `uses: <owner>/<repo>/installer@<commit>`, where `<owner>/<repo>` is `package.json`'s `repository` and `<commit>` is a release's tagged commit. §What init seeds writes that step.
>
> **It runs the published binary for the runner, at the version the tree installed, and never the committed one.** The committed binary is the installing host's target (§The gate binary). So the Action runs `docs/install.sh` from its own checkout, with `CHECKWRIGHT_VERSION` set to the lock's `version` and `--run` as the argument, and that script fetches and verifies the release. The bootstrap then selects the runner's artifact and executes the battery in the tree (§The install boundary). Every member dispatches through that binary (gate-sdk/SPEC.md §run-gates), so a substituted committed binary cannot turn the check green. The version follows the lock, so `update` moves the binary with no edit to the workflow. The `version` input names a release for a tree vendored without the installer (§Vendoring without the installer), and a tree with neither is refused at exit 2.
>
> **The reds reach the pull request as SARIF.** The battery step sets `GATE_SDK_SARIF_FILE` (gate-sdk/SPEC.md §run-gates), and a second step uploads the log with GitHub's `upload-sarif` action, even after a red. The upload is presentation, never the verdict. It runs with `continue-on-error`, since a fork's pull request carries a read-only token and a repository without code scanning refuses the upload, so the job's status stays the battery's. The calling job declares `security-events: write`.
>
> **It runs the battery and nothing else.** The release the lock names was tagged on a commit whose `gates` run passed every fixture suite (RELEASING.md), and each further binary execution would be a further download. A consumer's own gates and runners are later steps of its own workflow (gate-sdk/SPEC.md §templates/gates-workflow.yml).
>
> **Honest limits.** The Action trusts the Release host as the one-line install does, and its digest travels with the tarball (§The dependency boundary). The runner must carry `curl`, `tar`, a SHA-256 hasher and `/bin/sh`, as GitHub's hosted runners do. A consumer's shell member that reaches the binary through `GATE_SDK_NATIVE_BIN` runs the committed copy, because `--run` hands each member the invoking environment unchanged. The witness runs on a Linux runner. The Action's other hosts are measured only by the install-smoke legs, which run the same bootstrap.

### (4) `init` seeds the workflow {design-bearing}

**Not yet applied.** installer/SPEC.md §What init seeds gains, after the queue-file paragraph:

> **The CI workflow is seeded when the package names a GitHub repository.** `init` writes `.github/workflows/gates.yml` only when it is absent, the create-once discipline above, so a workflow already at that path is left alone. The workflow holds one job on `ubuntu-latest`. It triggers on a push to the branch `init` ran on (any branch from a detached `HEAD`) and on every pull request, checks the tree out with its full history, and runs the CI action (§The CI action). The action step names `package.json`'s `repository` and the payload's `commit` as a full SHA with the version as a trailing comment, the form gate-sdk/SPEC.md §check-action-pinning holds, and the checkout step is pinned the same way. The job declares `contents: read` and `security-events: write`, and carries no `run:` body. A package whose `repository` is not a GitHub URL, or whose `commit` is unstamped, seeds nothing and says so, since the step would name an action no runner can fetch. **Honest limits:** the ref stays at the release that seeded it, while the binary it runs follows the lock. A tree hosted elsewhere carries a file nothing runs, and deleting it does not stick, since a deleted create-once seed is re-seeded.

In the crate, the recipe writes the file from a template in `native/src/installer/` with the slug, commit, version and branch filled in. The slug is derived from `repository.url` and the checkout pin is a recipe constant. The workflow is recorded in `files` like every create-once seed, so `diff`, `uninstall` and the `--dry-run` plan reach it by their existing rules. The `// spec:` directive on the template points at this paragraph.

### (5) The consumer smoke holds the seed {mechanical}

**Not yet applied.** installer/SPEC.md §The consumer smoke, in the per-profile sequence after the install, gains:

> The seeded workflow is asserted on every profile: `.github/workflows/gates.yml` exists, and its action step names the packed `repository` slug, the packed commit and the packed version. The binary's `check-action-pinning` and `check-action-permissions`, run in the consumer, are green over it. `uninstall` later removes it with every other unedited seed.

`installer/consumer-smoke/run-smoke.sh` carries the arm as its own printed header, and `.workflow/validate-baseline.txt` gains its `installer_smoke` row at `pass`.

### (6) The workflow gates' install ground {design-bearing}

**Not yet applied — open question to the lead** on the dispositions of `check-action-pinning` and `check-action-permissions`. The text below is the recommended option, which keeps them.

In gate-sdk/SPEC.md §check-action-run-path, "`install: on-surface`, because the subject is the adopter's own workflows, which `init` never writes (§The install disposition), the disposition of every `check-action-*` sibling." becomes:

> `install: on-surface`, the disposition of every `check-action-*` sibling (§The install disposition): the subject is the `run:` bodies of the adopter's own workflows, and the workflow `init` seeds carries none (installer/SPEC.md §What init seeds).

In §The install disposition, the `on-surface` bullet's parenthesis "(a glossary, a docs host, a stage attestation, their own workflows)" becomes "(a glossary, a docs host, a stage attestation, their own workflows beside the one `init` seeds, which the installer's smoke holds)".

### (7) The template points at the seed {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §templates/gates-workflow.yml, the first sentence gains, after "copied out to a consumer's `.github/workflows/gates.yml`":

> …by a consumer vendoring without the installer, since `init` seeds a workflow calling the installer's CI action instead (installer/SPEC.md §The CI action)…

The template header's "To adopt: copy to .github/workflows/gates.yml" sentence opens "A tree `init` installed already carries a workflow; to adopt this one instead, copy it…".

In §Enforcement tiers, the CI bullet's "The copy-out is `templates/gates-workflow.yml` (see there);" becomes:

> A tree `init` installed carries a seeded workflow running the battery through the installer's CI action (installer/SPEC.md §The CI action). The fixture suites for the gates it vendors were executed on the tagged commit of the release its lock names, and it adds its own fixture step when it authors gates. The copy-out for any other tree is `templates/gates-workflow.yml` (see there);

### (8) The remote witnesses {design-bearing}

**Not yet applied.** In `.github/workflows/gates.yml`:

- **`install-smoke-sh-linux`.** After the hosted one-liner step, a step serves the same release layout on the local origin again, left running for the job. Then a step `uses: ./installer` runs with `path:` set to the one-liner's consumer, `upload-sarif: 'false'`, and `env: CHECKWRIGHT_RELEASE_BASE` set to that origin. A last step asserts that the action's `sarif` output names a file whose `runs` array holds one run, read with `jq`, which the runner image carries. The one-liner's consumer sits under `$RUNNER_TEMP`, so the action's `path` input takes an absolute path as well as a workspace-relative one.
- **`gates`.** The full battery step sets `GATE_SDK_SARIF_FILE` under `$RUNNER_TEMP`, and a following step uploads the log with the same pinned `upload-sarif`, under `if: always()` and `continue-on-error: true`. The job's `permissions:` gains `security-events: write`, and its header comment names why.

The entry's oracle is the mid-iteration push: the Action step green, the assertion green, and the `gates` job's upload step concluding `success`, each read with `gh run view <id> --log`.

### (9) The install page and the package README {mechanical}

**Not yet applied.** docs/install.md §Managing, the paragraph "The pre-commit hook is a local backstop anyone can skip. Make the gate battery a required status check in CI, so a red battery blocks the merge, and keep that check where the authors it holds cannot edit it." becomes:

> The pre-commit hook is a local backstop anyone can skip. `init` also commits `.github/workflows/gates.yml`, which runs the battery on every push and pull request at the release you installed and marks each red on the pull request. Make its check required, so a red battery blocks the merge, and keep that check where the authors it holds cannot edit it.

installer/README.md §Quick start's `init` paragraph, "vendors the selected profile's kit directories, writes a `gates.list`…", gains "and a CI workflow" after "the config seam those kits need".

### (10) The release declarations {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`, `## Behavior changes`, gains:

> - **`init`** — seeds `.github/workflows/gates.yml` where it is absent, a workflow running the battery through this project's CI action at the release your `checkwright.lock` names and uploading each red as SARIF. `update` seeds it into an existing install too. If you already run the battery from a workflow at another path, delete one of the two.
> - **`GATE_SDK_SARIF_FILE`** — a new environment-only gate-sdk knob: set, the battery also writes its verdict as a SARIF 2.1.0 log to that path. Unset, nothing changes.

## Producers and consumers

- **`GATE_SDK_SARIF_FILE` and the log.**
  - Producer: the invoker's environment. The Action sets it on every run, and this repo's `gates` job sets it on its battery step.
  - Consumers: `upload-sarif`, and GitHub code scanning behind it, which turns each result into an alert and a pull-request annotation. Also the kit smoke's violation assertion (delta 2) and the witness assertion (delta 8).
  - Fields: `ruleId` is read by code scanning and the kit smoke. A location is read by code scanning, which places the annotation. `shortDescription` and `helpUri` are read by code scanning's rule pane. `level` is read by code scanning's severity. The empty result set on green is read by code scanning, which closes resolved alerts. Every field has a reader.
  - Red condition: an unwritable path is exit 2. The knob never changes the battery's own exit.
- **The Action's version resolution.**
  - Producer: the lock's `version`, written by `init` and `update`, or the `version` input.
  - Consumer: `docs/install.sh`'s `CHECKWRIGHT_VERSION`, which names the release fetched.
  - Red condition: neither present is exit 2. A version with no release is `install.sh`'s download refusal, exit 2.
- **The seeded workflow.**
  - Producer: `init`, on every profile, when the package names a GitHub repository and a stamped commit, which every published package does. The pack arm stamps the commit, and `repository` is tracked in `installer/package.json`.
  - Consumers: GitHub Actions on the adopter's push, and the install smoke's seed arm (delta 5). The manifest's `files` roster reaches it through `diff`, `uninstall` and `--dry-run` by the existing create-once rules. The workflow gates read it wherever an adopter registers them.
  - Obligation over an enumerable corpus: every profile seeds it, and the smoke asserts it per profile, so each profile's satisfying value is the one seeded file.
- **The Action's `sarif` output.** Its producer is the battery step. Its consumers are the upload step and a caller's later step, the witness assertion being one.
- **Downstream.** The seed pins a commit carrying `installer/action.yml`, so it is fetchable only from the release that ships this unit. A tree installed from an earlier release has no seed and gets one at its next `update`.

## Existing sections updated

Roster probes: `git grep -n "never writes (§The install disposition)"` and `git grep -n "gates-workflow"` over the tracked tree, and `grep -n "^# install:" gate-sdk/checks/check-action-*.gate`.

- `gate-sdk/SPEC.md` — §run-gates and §Layout and configuration (delta 1); §Consumer smoke (delta 2); §check-action-run-path and §The install disposition (delta 6); §templates/gates-workflow.yml and §Enforcement tiers (delta 7).
- `native/src/knobs/gate_sdk.rs`, `native/src/runner.rs`, a new `native/src/sarif.rs` (delta 1).
- The `--run-consumer-smoke` arm's source (delta 2).
- `installer/action.yml`, `scripts/core-files.list`, installer/SPEC.md §Layout and the new §The CI action (delta 3).
- installer/SPEC.md §What init seeds; `native/src/installer/` init recipe and its workflow template (delta 4).
- installer/SPEC.md §The consumer smoke; `installer/consumer-smoke/run-smoke.sh`; `.workflow/validate-baseline.txt` (delta 5).
- `gate-sdk/templates/gates-workflow.yml`'s header (delta 7).
- `.github/workflows/gates.yml` — `install-smoke-sh-linux` and `gates` (delta 8).
- `docs/install.md` §Managing; `installer/README.md` §Quick start (delta 9).
- `.workflow/release-declarations.md` (delta 10).
- `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md` and the knob roster projection — generated, regenerated by the command each freshness gate prints on red (all deltas).

## Retired spellings

- `never writes (§The install disposition)` — §check-action-run-path's ground for the workflow gates' disposition, falsified by the seed (delta 6).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Local oracles green** — the full battery, the gate-sdk kit smoke with its SARIF assertion, and `bash installer/consumer-smoke/run-smoke.sh` with its seed arm.
- [ ] **The remote oracle read** — the mid-iteration push's `install-smoke-sh-linux` Action step and its assertion, and the `gates` job's upload step, each concluding `success`. The entry moves to Done when build's remote-oracle rule says, before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
