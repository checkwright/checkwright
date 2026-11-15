# SPEC amendment: verdict-contract

Pairs `machine-verdict-contract`. An external orchestrator driving the battery reads its verdict from the SARIF log `GATE_SDK_SARIF_FILE` names, and today that log can tell it only which members failed. A green member leaves no trace, so a passing gate and one that checked nothing look the same. Nothing records which binary or which kits produced the verdict, or what tree they ran on. This amendment makes the log the versioned verdict contract. It carries a provenance block (the tool, each kit, the commit), a result for every member (pass, fail or not applicable), a contract version under semver, and golden fixtures. The exit statuses stay as they are, 0, 1 and 2, because a hook or CI reader blocks on any non-zero.

**What the tree does today**, read from `native/src/sarif.rs`, `native/src/runner.rs`, `native/build.rs`, `native/src/installer/lock.rs`, `native/src/emit/run_consumer_smoke.rs`, `installer/action.yml`, `.github/workflows/gates.yml`, `.github/workflows/publish.yml` and `scripts/ci-build-artifact.sh`:

- **Only red members contribute to the log.** `sarif::render` iterates `members.iter().filter(|m| m.failed)`. A green run's log holds an empty `results` array and an empty `rules` array, and every result it writes is `"level": "error"` with no `kind`.
- **The log carries no provenance.** The driver has a `name` and `rules` and nothing else. Neither the log nor the run has a `properties` bag.
- **The binary carries no release version.** `native/Cargo.toml`'s `version = "0.1.0"` is a placeholder no release moves, and the binary has no `--version`. `build.rs` stamps only `CHECKWRIGHT_SOURCE_STAMP`, a content hash of the crate's inputs, and `CHECKWRIGHT_TARGET`. Its rerun triggers are its input paths and the git index. A release's version exists only as its `v<X.Y.Z>` tag, as the `--version` the publish workflow's pack job hands the packer (`version="${TAG#v}"`), and as `checkwright.lock`'s `version` in a tree `init` vendored.
- **A kit has no version of its own.** `checkwright.lock` records one `version`, the release the payload was cut at, and the vendored set as `kits`. In a source tree, a hand-vendored tree or a residual manifest, there is no `version`.
- **The publish build legs run `bash scripts/ci-build-artifact.sh "$TARGET" "$RUNNER_TEMP/artifact"`**, which builds through `gate-sdk/bin/build-native.sh --target` and passes no version.
- **Gates that checked nothing say so in prose alone.** The survey record's not-applicable block lists 29 early-out clean lines over `native/src/gates`, such as `MD-UNWRAPPED: clean (CANON_KIT_UNWRAP_GLOBS is unset; nothing scanned)`. Each exits 0 with an ordinary clean line.
- **Two readers upload the log to GitHub code scanning**: the installer's CI action (`installer/action.yml`) and this tree's own `gates` job (`.github/workflows/gates.yml`, the `upload the battery's SARIF log` step). One reader asserts on its content: `--run-consumer-smoke`'s `sarif_names` looks for a result carrying the red gate's rule id.

## What changes

### (1) The provenance block {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): the SARIF log becomes the versioned verdict contract, carrying a tool version, one per kit and the commit; `semanticVersion` only for an exact release stamp, by intent-oracle answer at align 2026-11-15, lead-relayed (not a ruling)}

§run-gates gains a paragraph, **The provenance block**, after the `GATE_SDK_SARIF_FILE` paragraph, and it is the one definition of the block. evidence-kit's gate-run predicate cites it rather than restating it. The block is a JSON object whose keys are always sorted. Each field is **absent** where the run cannot fill it, never null or blank, following the manifest's convention (installer/SPEC.md §The manifest). **Not yet applied.**

| Field | Holds | Source |
| --- | --- | --- |
| `tool.name` | `checkwright-gates` | the crate's constant |
| `tool.version` | the binary's release stamp | the build (below) |
| `tool.sourceStamp` | the content hash `check-gate-binary-fresh` compares | `CHECKWRIGHT_SOURCE_STAMP` |
| `tool.target` | the triple the binary was built for | `CHECKWRIGHT_TARGET` |
| `kits[]` | one `{name, root, version}` per resolved kit root, in `GATE_SDK_KIT_DIRS` order: `name` the root's last path component, `root` its repo-relative path, `version` the lock's `version` where `checkwright.lock` names the kit in `kits` | the kit-root resolution and the lock, read through `installer::lock`'s accessors |
| `commit` | `HEAD`, 40 or 64 lowercase hex characters (a SHA-1 or a SHA-256 object format) | `git rev-parse HEAD`; absent on an unborn branch |
| `worktree` | `clean` when tracked content matches `HEAD`, else `modified` | `git status --porcelain --untracked-files=no`; absent with `commit` |

**Where the release stamp comes from.** `build.rs` stamps `CHECKWRIGHT_RELEASE`. Where the build environment sets `CHECKWRIGHT_BUILD_VERSION`, the stamp is that value with any leading `v` removed. Otherwise it is the answer of `git describe --tags --match 'v[0-9]*'` in the crate's checkout, also stripped of its `v`. Otherwise it is empty, and `tool.version` is absent. A release build is therefore exact: the publish build legs set `CHECKWRIGHT_BUILD_VERSION: ${{ github.ref_name }}` on the step that runs `scripts/ci-build-artifact.sh`, the same tag the pack job hands `--pack-installer --version`. A local build reads like `0.36.0-6-g<hash>`. `build.rs` also stamps whether the release stamp is **exact**: it is when it came from `CHECKWRIGHT_BUILD_VERSION`, or when describe answered a tag with no `-<n>-g<hash>` distance suffix. `git describe` failing, on a tagless or shallow checkout, empties the stamp and never fails the build.

**Which builds carry a version.** The publish workflow's build legs are the one release producer, and they set `CHECKWRIGHT_BUILD_VERSION`. The `gates` job's native-artifact build runs the same `scripts/ci-build-artifact.sh` body and sets no version. Its checkout is `actions/checkout`'s default shallow clone of a branch, which fetches no tag, so describe fails and that artifact's `tool.version` is absent. That is the correct answer: the artifact is a CI witness and never released, and no reader of it keys on the tool's version (delta 6's consumer-smoke check reads the contract's major alone).

**The rerun triggers.** `build.rs` adds `cargo:rerun-if-env-changed=CHECKWRIGHT_BUILD_VERSION` and `cargo:rerun-if-changed` on the **common** git directory's `refs/tags` and `packed-refs` (`git rev-parse --git-common-dir`, since a linked worktree keeps its refs there, not under its own git directory), so creating a tag restamps the next build. Each of the two is emitted only where it exists, with `refs` standing in for an absent `refs/tags`, because cargo treats a missing `rerun-if-changed` path as changed and would rerun the build script on every build. The index trigger keeps reading the per-worktree git directory. §check-gate-binary-fresh's trigger paragraph names the added triggers, the stamp takes no part in the freshness comparison, and `build.rs`'s opening comment re-scopes its count of git invocations to the source stamp's. **Not yet applied.**

**Where the block lands in the log.** The run's `properties.provenance` carries the block whole, and that is the field a contract reader reads. For generic SARIF readers, it is also mirrored into the native fields: `tool.driver.version` from `tool.version`, `tool.driver.semanticVersion` only where the release stamp is exact and a valid Semantic Versioning 2.0 string. A describe answer with a distance suffix is a valid SemVer pre-release string, yet it names no release, so it leaves `semanticVersion` absent, and `tool.extensions[]` with one `toolComponent` `{name, version}` per `kits[]` entry. Every rule carries `properties.kit`, the `name` of the kit root its member resolved from. That property is absent for a member resolved from the consumer's gates dir. **Not yet applied.**

**Refused: `run.versionControlProvenance`.** SARIF requires each entry to carry a `repositoryUri`, and the run can name one only by choosing a remote. A remote's URL can carry credentials, and a log uploaded to code scanning would publish them. `commit` and `worktree` in the block carry what a verdict reader needs, and GitHub's upload takes its commit from the workflow rather than from the log. **Not yet applied.**

**Honest limits.** A kit's version is the lock's. A source tree, a hand-vendored tree and a residual manifest carry none, so in them `kits[].version` is absent and the commit is the kits' only identity. The lock records a single release for the whole vendored set, so a kit edited after `init` still reports the version `init` wrote. The divergence the block exists to expose is between `tool.version` and `kits[].version`: a binary installed from one release reading kits vendored from another. A local build's stamp is the describe answer as of its last build. **Not yet applied.**

### (2) A result for every member, and which results a log carries {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): a result for every member — pass, fail or not applicable — and no fourth exit code}

§run-gates' two SARIF paragraphs are re-phrased into one contract. **Not yet applied.**

- **One rule per selected member**, in registry order, whatever its verdict. Each rule carries the declared invariant, `helpUri` and `properties.kit` as delta 1 states, so a reader can see what a green gate holds.
- **One or more results per member.** A red or unresolved member keeps today's placement: one result per `<path>:<line>:` finding line, else one result at its registry line. Each such result carries `"kind": "fail"` and `"level": "error"`. A green member gives one result at its registry line, with its clean line as the message, `"level": "none"`, and `"kind": "pass"` or `"kind": "notApplicable"` per delta 3. SARIF requires `level` to be `none` whenever `kind` is anything other than `fail`.
- **A declared omission is a result.** In a bare run, each `# omitted: <name> <reason>` line gives one `notApplicable` result located at that comment's own 1-based line in the registry, which `registry::member_lines` never numbers because it drops comments. Its message is the reason token, the line's fourth whitespace field, the same token `report_omissions` groups the summary's omission lines by (§run-gates' *one line per reason token*), and its rule carries no invariant. That puts the omission accounting the summary line prints into the log as well. A `--only` or `--for` run reports only the members it selected.
- **The run says what it covers.** `properties.scope` is `registry` for a bare run, `only` for `--only` and `for` for `--for`. `properties.results` records the selection below. A reader can therefore tell a complete verdict from a partial one without parsing argv.

**`GATE_SDK_SARIF_RESULTS` selects which results the log carries.** It is a fourth reporting knob, environment-only like `GATE_SDK_SARIF_FILE` and read only where that knob is set. `all`, the default, writes every result above. `failures` writes the `fail` results and the rules they cite, which is today's red-only log plus the provenance and version markers. Any other value refuses at exit 2 before the first member runs, naming the knob and the two values. **Not yet applied.**

**The code-scanning uploaders select `failures`.** Code scanning presents findings, and a green member's result there is noise at best. Community reports, not GitHub's documentation, say its importer ignores `kind` and opens an alert for a `pass` result *(inferred: no code-scanning upload was run; GitHub's SARIF support page names neither `kind` nor `level: none` on a result)*. The selection does not rest on that report: a `failures` upload is right either way. `installer/action.yml`'s battery step and the `gates` job's `full battery` step each set `GATE_SDK_SARIF_RESULTS: failures`. installer/SPEC.md §The CI action's SARIF paragraph says so, and adds that the action's `sarif` output is therefore the failures log, which is presentation and never the verdict. A green run's `failures` log still carries an empty result set, so code scanning still closes what the previous run opened. **Not yet applied.**

The summary line, the per-member verdict lines and the exit status are unchanged.

### (3) How a gate signals not applicable {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): the log reports not applicable as a result, with no fourth exit code; the honest limit naming a `pass` outside the sweep's candidates, by intent-oracle answer at align 2026-11-15, lead-relayed (not a ruling)}

§Output contract's clean-line sentence, which says the parenthetical states what was checked, is re-phrased so it covers both cases, rather than gaining a sentence that contradicts it. **Not yet applied.**

> A gate signals success with exactly one line `^<NAME>: clean (<parenthetical>)$`, where `<NAME>` is the gate's stable upper-token id and the parenthetical states what was checked. A gate that evaluated no instance of its rule — its configuration declares nothing to check, or the corpus it reads is empty — checked nothing, so its parenthetical opens `not applicable: ` and states why, as in `<NAME>: clean (not applicable: <reason>)`, and it exits 0.

**Why the clean line carries it.** It is still the one canonical success line, so `run_gate_tests::clean_line`, `check-gate-output`'s static read and every reader keyed on `: clean (` are untouched, and a hook or CI reader sees exit 0 as before. A fourth exit code would make every caller that branches on the status learn a new value, and the operator direction refuses one. A descriptor field is refused as well: whether a gate applies depends on its knobs and its corpus at run time, which a static declaration cannot know.

**The runner reads it.** The arm already captures a green member's output, and today it discards it. It now scans that capture for the last line `clean_line` accepts: a parenthetical opening `not applicable: ` makes the result `notApplicable`, and any other clean line makes it `pass`. A green member that printed no clean line is `pass`, since the exit status is the verdict (§Output contract), and its message is its verdict line. `run_gate_tests::clean_line` moves to a module both the runner and the fixture runner read. **Not yet applied.**

**Honest limit.** The log is only as honest as each gate's early-out: a gate that checks nothing and prints a plain clean line reports `pass`, and no reader can tell that apart from a real pass. The sweep below respells the early-outs its oracle reaches, and that oracle is lexical. A bundled gate outside its candidates may still print a plain clean line on a branch that evaluated nothing, so a `pass` from such a gate may hide an unsignalled not-applicable. A consumer's own gate adopts the form by printing it.

### (4) The contract version and its migration rule {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): a contract version under semver}

§run-gates gains a paragraph, **The verdict contract's version**. **Not yet applied.**

- **The marker.** The log's top-level `properties.verdictContract` holds a Semantic Versioning 2.0 string. This change ships `1.0.0`. A log with no marker predates the contract: red results only, with no provenance.
- **What the contract is.** The log's shape and its meanings: the provenance block, `properties.scope` and `properties.results`, one rule per member, rule ids being member names, the result kinds and levels, the placement rule, and registry order. Message text, a rule's description text and the human transcript are not part of it, and they change freely.
- **The rule.** The **major** version rises when a field is removed or renamed, a field's meaning or a kind's meaning changes, a value set a reader switches on narrows, or the placement or ordering rule changes. The **minor** version rises when a field, a property, a `scope` or `results` value, or a result-bearing class is added. The **patch** version rises for a correction that changes neither shape nor meaning.
- **The reader's half.** A reader refuses a major it does not know and ignores fields it does not know. Every reader this tree ships holds to that.
- **Migration.** The binary writes one major, never two side by side. A major change lands with a migration note in this paragraph naming what moved. Its release note's Behavior changes section names the release that carries it, following the provenance seam's rule for data a kit writes into a consumer's tree (§The provenance seam). The note for `1.0.0` is this delta: green and not-applicable members gained results, the run gained the provenance block, and `failures` reproduces the earlier red-only shape.

### (5) The golden fixtures {mechanical}

`docs/contracts/verdict/v1/` holds four logs, each rendered from a fixed member set and a fixed provenance block, so the provenance carries no live stamp: `green.sarif` (two passes and one not applicable), `red.sarif` (one finding-line failure, one registry-line failure and one pass), `omitted.sarif` (a bare run with one declared omission), and `failures.sarif` (`red.sarif`'s members under `failures`). They are published on the site at `/contracts/verdict/v1/`, so an outside caller can test its reader against them without a checkout. The build first runs the battery with the four files in place, because the docs gates have never held a `.sarif` file under `docs/`. A docs gate refusing one takes the `docs/contracts/` tree out through that gate's own path knob in this tree's consumer config, the exclusion its SPEC section names, landed in the same commit. A refusing gate with no such knob is a cross-component gap the build escalates. The goldens are never renamed or moved to fit a gate. **Not yet applied.**

A unit test in `native/src/sarif.rs` renders each one through `sarif::render` and byte-compares the result against the file, which it reads with `include_str!`. The test lives in the module because the crate is a binary with no library target, so an integration test under `native/tests/` cannot reach `sarif::render`. A renderer change that moves a golden is therefore a visible contract diff, which the commit making it judges against delta 4's rule and lands with the golden edited. The goldens are fixtures held by a test, never a regenerated projection, so they take no row in docs/generated-projections.md. `sarif::render` takes the provenance block and the scope as arguments, rather than reading them inside, so that the test can fix them. **Not yet applied.**

### (6) The readers {mechanical}

Each **Not yet applied.**

- **`--run-consumer-smoke`** (§Consumer smoke): the violation run's log must hold a `fail` result carrying the red gate's rule id, and its `verdictContract` major must be `1`. `sarif_names` reads both, and its `help:` line is unchanged.
- **The installer's CI action**: `installer/action.yml`'s battery step sets `GATE_SDK_SARIF_RESULTS=failures` beside `GATE_SDK_SARIF_FILE`, and installer/SPEC.md §The CI action says so (delta 2).
- **This tree's `gates` job**: `.github/workflows/gates.yml`'s `full battery` step sets `GATE_SDK_SARIF_RESULTS: failures`.
- **The lock's reader table**: installer/SPEC.md §The manifest's `version` and `kits` rows add the battery's provenance block to their *Read by* cells (delta 1).
- **The knob rosters**: §Layout and configuration's knob list gains `GATE_SDK_SARIF_RESULTS` beside `GATE_SDK_SARIF_FILE`, and its sentence naming the five environment-only names re-phrases its count and its list of execution settings. §The knob file's environment-only paragraph re-phrases its count of gate-sdk's environment-only names to cover four execution settings. `native/src/knobs/gate_sdk.rs`'s `env_only` list and the `--run` usage text in `native/src/runner.rs` gain the knob.
- **The publish workflow**: each build leg's `scripts/ci-build-artifact.sh` step sets `CHECKWRIGHT_BUILD_VERSION` (delta 1). The `gates` job's native-artifact build is left as it is, for the reason delta 1's *Which builds carry a version* states.

### (7) The not-applicable sweep {mechanical} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): a result for every member, pass, fail or not applicable}

Each gate whose clean line reports that it evaluated nothing respells that line in delta 3's form. The parenthetical keeps the reason it gives today, after `not applicable: `. The candidates are the survey record's not-applicable block (`.workflow/survey-record.md`, rev f63f137e2). Its oracle is the `git grep` that block records. It matches 36 lines over `native/src/gates`, and the block's 29 candidates are those 36 less the seven it excludes as having checked something. The build reads each candidate's branch and respells it only where that branch evaluated no instance of the rule. The block lists the lines already excluded as having checked something. Every case and crate test pinning a respelled line moves with it. **Not yet applied.**

### (8) Cases, the release declaration and the mirrors {mechanical}

Each **Not yet applied.**

- **`sarif.rs` unit tests**: a pass, a not-applicable and a no-clean-line green result; an omission's result in a bare run and none under `--only`; `failures` keeping fail results and their rules only; a refused `GATE_SDK_SARIF_RESULTS` value; the provenance block with each optional field absent; `semanticVersion` present for an exact `0.36.0` and absent for `0.36.0-6-gabc`; a `commit` of 64 hex characters; the four goldens' byte comparison (delta 5).
- **`build.rs`'s stamp**: the build-input precedence over describe, the `v` strip, and the exactness flag (a build-input stamp and a distance-free describe answer exact, a distance form not), through unit tests of the stamp function. `cargo test` never compiles a build script's tests, so the function, pure over its two inputs, lives in `native/src/release_stamp.rs`: `build.rs` includes it by `#[path]`, and the crate declares it under `#[cfg(test)]`, where its tests run.
- **The release declaration** (`.workflow/release-declarations.md`). Under `## Behavior changes`: the SARIF log carries a result for every member, along with the provenance block and `verdictContract` `1.0.0`; set `GATE_SDK_SARIF_RESULTS=failures` to keep the red-only log; and a bundled gate that checked nothing prints `clean (not applicable: …)`. Under `## Knob changes`: `GATE_SDK_SARIF_RESULTS`.
- **The regenerated surfaces**: the mirrors `docs/gate-sdk/SPEC.md` and `docs/installer/SPEC.md`, by the command docs/generated-projections.md rosters for them.

## Producers and consumers

- **The provenance block** (delta 1). Producer: the `--run` arm, through a new `native/src/provenance.rs` that `sarif::render` reads. Consumers: the log's contract readers, generic SARIF readers through the mirrored native fields, and evidence-kit's gate-run predicate, which embeds the block (`gate-run-attestation-predicate`).
- **`CHECKWRIGHT_BUILD_VERSION`** (delta 1). Producer: the publish workflow's build legs. Consumer: `build.rs`.
- **The per-member results, `scope` and `results`** (delta 2). Producer: `sarif::render`. Consumers: an outside caller, `--run-consumer-smoke`, and code scanning through the two uploaders, which select `failures`.
- **The not-applicable clean line** (delta 3). Producers: the swept gates and any consumer gate that adopts it. Consumers: the runner's result classification, plus every existing clean-line reader, which is unchanged.
- **`verdictContract`** (delta 4). Producer: `sarif::render`. Consumers: `--run-consumer-smoke`'s major check and every outside reader.
- **The goldens** (delta 5). Producer: the commit that changes the contract. Consumers: `native/src/sarif.rs`'s golden test and outside readers through the site.

The readers were found by `git grep -n -i -l sarif` over the whole tracked tree, which names this SPEC, installer/SPEC.md, their mirrors, `installer/action.yml`, `.github/workflows/gates.yml`, one release note and the crate modules above. The lock's readers were found by `git grep -n "checkwright-lock v1" -- native/src`. The clean-line readers were found by `git grep -n "clean_line\|: clean (" -- native/src/emit native/src/runner.rs`. The gate side was found by the survey block's oracle.

## Existing sections updated

- `gate-sdk/SPEC.md` §run-gates (deltas 1, 2 and 4), §Output contract (delta 3), §check-gate-binary-fresh (delta 1), §Layout and configuration and §The knob file (delta 6), §Consumer smoke (delta 6).
- `installer/SPEC.md` §The CI action and §The manifest (delta 6).
- `native/src/sarif.rs`, the new `native/src/provenance.rs` (declared by `mod provenance;` in `native/src/main.rs`), `native/src/registry.rs` (its comment on a red member with no finding line), `native/src/runner.rs`, `native/build.rs`, `native/src/emit/run_gate_tests.rs` (`clean_line`'s move), `native/src/emit/run_consumer_smoke.rs` and `native/src/knobs/gate_sdk.rs`, with their tests, and the new `native/src/release_stamp.rs`, which `build.rs` includes and `native/src/main.rs` declares under `#[cfg(test)]` (deltas 1 to 6 and 8).
- The swept gates under `native/src/gates/`, with their cases (delta 7).
- `installer/action.yml`, `.github/workflows/gates.yml` and `.github/workflows/publish.yml` (delta 6).
- `docs/contracts/verdict/v1/` (delta 5), `.workflow/release-declarations.md`, and the regenerated surfaces delta 8 lists.
- Where delta 5's probe finds a docs gate refusing a golden, that gate's path knob in this tree's consumer config under `scripts/` (delta 5).

## Retired spellings

- None — every knob, arm and field keeps its name. The log gains fields and results. The swept clean lines keep their gate names and the `clean (` token, and only their parentheticals are reworded.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — all green: `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests including `sarif.rs`'s golden test and `release_stamp`'s, the fixture suites of gate-sdk, installer and every kit owning a swept gate, and `--run-consumer-smoke`. The entry's Done move lands before the drain stage, in the build batch that lands it. The `gates` job's run at the closing push is the remote witness of the `failures` upload, and the first release built after it lands witnesses an exact `tool.version`. Neither one gates the move: a miss in either is filed as a gap.
