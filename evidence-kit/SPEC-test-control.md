# SPEC amendment: test-control

Pairs `recorded-test-mutation-control`. The fixture pair proves that a gate can fail (gate-sdk/SPEC.md §Fixture-pair discipline), but nothing shipped proves the same of a test a validate suite records. A recorded run of a test that cannot fail therefore still supports a done claim. This amendment adds a **declared test control**: a consumer names a test and the environment that re-introduces the defect the test exists to catch. The validate spine re-runs the suite under that environment and records whether the test went red. A gate then requires that a newly baselined test carries a control, and that the control was witnessed red before close. It is off unless a consumer declares a control. The record is a line kind in the evidence manifest's second major version, which `gate-run-attestation-predicate` introduces (evidence-kit/SPEC-gate-run-predicate.md, delta 2).

**What the tree does today**, read from `native/src/emit/run_validate.rs`, `native/src/evidence.rs`, `native/src/stages.rs`, `evidence-kit/SPEC.md`, `scripts/gates.list` and `doctrine-kit/DOCTRINE.md`:

- **A suite runs once per invocation, in one environment.** `run_validate.rs` spawns the pre-hook, then the suite's resolved `EVIDENCE_KIT_RUN_<suite>` argv, and nothing else. No knob varies the environment between two runs of one suite.
- **A baseline `pass` row is optional.** §Baseline manifest's fail-closed rule keys on `fail`, so an unbaselined test that passes is no red, and pass rows are recorded from a run by human commit.
- **evidence-kit reads the stage cursor and not the iteration-start commit.** `evidence.rs` reads the state file's last data line for the cursor. The iteration-start commit, the first data line's `<head>` (lifecycle-kit/SPEC.md §The state machine), has a reader only in `stages.rs`, which evidence-kit does not call, for the independence §The evidence adapters states.
- **A shipped gate may be left out of a registry with a declared reason** (`# unregistered:` lines in `scripts/gates.list`).
- **The doctrine names the check as unbuilt.** doctrine-kit/DOCTRINE.md rule 26's corollary says a mutation proves an assertion bites only against a target the assertion ranges over. Its *Enforced by* lists the fixture pair as shipped, and nothing for a recorded test.

## What changes

### (1) The declared control {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling), promoting the consult advice of 2026-11-14: off unless a consumer selects it, a declared control that re-introduces the defect and names the test it must fail}

evidence-kit/SPEC.md gains a section, **Test controls**, after §bin/run-validate.sh, and §Layout and configuration gains two knobs. **Not yet applied.**

- **`EVIDENCE_KIT_CONTROLS`** holds the ordered control names. The default is empty, and empty runs no control and arms none of the gate's assertions that read a declaration, so a consumer selects the feature by declaring a control.
- **`EVIDENCE_KIT_CONTROL_<name>`** is a third declared family, a resolution set beside `EVIDENCE_KIT_RUN_<suite>` (§Layout and configuration's family rule). Its value is `<suite> <scenario> <VAR>=<value> [<VAR>=<value> …]`. `<suite>` is a member of `EVIDENCE_KIT_SUITES`, `<scenario>` is a scenario that suite's effective parser emits, and each assignment names an environment variable as a shell identifier with a value containing no whitespace, the word-split rule §The evidence adapters states for every spawned value.
- **What a control means.** The assignments switch on the defect the scenario exists to catch, and the scenario must fail under them. The kit gives the assignments no meaning. The consumer's code, or its test harness, reads the variable, so the control is stated in environment variables and test names and in no consumer's vocabulary.
- **A malformed declaration refuses.** A name that is not a valid family suffix is refused by the table validator. A value naming an unconfigured suite, carrying no assignment, or spelling an assignment that is not `<identifier>=<value>` refuses `--run-validate` at the guards' exit 2 before the claim, and reds `check-test-control` (A). One parser in `native/src/evidence.rs` serves both.

**Honest limits.** The kit witnesses that the scenario fails under the assignments, never that the assignments re-introduce *the* defect the scenario names. A control whose flag breaks the build reds the scenario too. A consumer reading a control's variable in shipped code carries that switch into its product, and keeping it in test-only code is the consumer's choice.

### (2) The control run and its record {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): its run recorded as a witnessed red beside the suite's line, in the one manifest major version the evidence-kit pair shares}

§bin/run-validate.sh's spine and §Evidence manifest are re-phrased. **Not yet applied.**

- **The spine runs a suite's controls straight after the suite**, in `EVIDENCE_KIT_CONTROLS` order, each a spawn of the suite's resolved argv with the control's assignments added to the inherited environment, each overriding a same-named variable. The pre-hook does not re-run: a control runs on the tree its suite just ran on. Its log is captured under the tmp dir and parsed by the suite's effective parser, and the produced-no-result guard holds for it as for the suite.
- **The witness is the scenario's observed status.** `fail` is `red`, `pass` is `green`, and an `ignore` or an unobserved scenario is `absent`.
- **The control line** is `<run-key> <suite> control=<name> scenario=<scenario> witnessed=<red|green|absent> <date> commit=<hex|-> cmd=<command-digest>`. `commit=` and `cmd=` are the values on the suite line the control follows (§The gate-run predicate's command digest), so the line says which run of which command it witnessed. A control line sits directly after its suite's line, in control order. Its third field, `control=`, is the line kind §Evidence manifest reserves, so the trajectory extractor, which takes only `sha256=` lines, never counts it.
- **Supersession follows the suite.** A run, or an import, that supersedes a suite's line drops that suite's control lines with it, since they witnessed the superseded run. An imported suite therefore carries no control line until a local run witnesses one.
- **A control that witnesses no red fails the run.** The exit contract's 1 re-phrases to *a suite recorded `new-failures`, or a control witnessed no red*, and the verdict output names each such control with its witness. The suite's own `verdict=` is unchanged, since that is the baseline diff's answer. The lines are written either way, so the manifest shows the failed control.
- **The declared knob roster** gains `EVIDENCE_KIT_CONTROLS` and `EVIDENCE_KIT_CONTROL_*`, and its count is re-phrased.

**Cost.** A control re-runs its suite's whole command. A consumer controlling a test in a slow suite declares a narrower suite for it, which is roster configuration and needs no new mechanism.

**The predicate carries no control.** Predicate `1.0.0` (§The gate-run predicate) has no control field, so a Statement attests the suite runs alone. A control field is a minor addition when a verifier asks for one.

**Sibling dependency, both acts.** The control line is a v2 line kind, and the set record lands this unit in the predicate's batch, after it. If the predicate lands earlier in the same batch, this delta adds the `control=` kind to the grammar the predicate's delta 2 reserves, reusing its `commit=` and `cmd=` values. If the predicate has not landed when this delta's batch is cut, this delta waits for it: a v1 manifest has no line kind to carry a control, and a second major version for the control alone is the outcome the set record refused.

### (3) `check-test-control` {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling), promoting the consult advice of 2026-11-14: a gate that a new test carries a control}

evidence-kit/SPEC.md gains a section, **check-test-control**, after §check-evidence-manifest: `checks/check-test-control.gate`, `precommit`, binary-dispatched, with an `# install:` disposition that is not `zero-config`, so `init` never registers it (installer/SPEC.md §What init seeds). Registering it is how a consumer selects enforcement, beside declaring controls. Invariant: every newly baselined test is controlled, and every control was witnessed red before close. **Not yet applied.**

- **(A) declarations.** Every name in `EVIDENCE_KIT_CONTROLS` resolves to a well-formed `EVIDENCE_KIT_CONTROL_<name>` value (delta 1), and no two controls name the same suite and scenario.
- **(B) new tests.** A **new test** is a baseline `pass` row present in the working baseline and absent, by suite and scenario, from the baseline file at the **since commit**. Every new test must be named by a declared control. The since commit is the iteration-start commit (lifecycle-kit/SPEC.md §The state machine), read by a new self-contained reader in `native/src/evidence.rs`. Where lifecycle yields none it is `EVIDENCE_KIT_CONTROL_SINCE`, a revision, default empty. Where both are empty, (B) is disarmed at a declared early-out and the clean line says so. A baseline file absent at the since commit makes every row new.
- **(C) close-entry witness.** A `close` cursor requires, for every declared control, a current-key control line with `witnessed=red`, whose `scenario=` is the declaration's and whose `commit=` and `cmd=` equal the line of the suite it follows. Its date window is §check-evidence-manifest (A)'s. An empty cursor disarms (C), as it disarms that gate's (A).

A red (A) suppresses (B) and (C). Argument mode takes the knob file, the baseline, the since baseline, the manifest, the queue and the state file as positionals, each overriding its knob or read. The `good/`/`bad/` pair reaches (B) through the since-baseline positional, because a fixture has no history. `gate-tests/check-test-control.test.sh` covers the git read, the `EVIDENCE_KIT_CONTROL_SINCE` fall-through, the disarmed early-out and (C).

**Why a baselined row is what counts as new.** A baseline row is the one record of a test the tree carries per suite and scenario, and it is a human commit, so *added since the iteration started* is a git fact rather than a guess about a log. **Honest limit:** a test that is never baselined is invisible to (B). A consumer selecting the gate baselines the tests it means to hold, which §Baseline manifest's parser rule already asks of finer coverage.

### (4) The readers {mechanical}

Each **Not yet applied.**

- **`check-evidence-manifest`** (B): the v2 grammar gains the control line's eight fields, and a control line whose key and suite have no suite line above it is red.
- **The knob table**: `native/src/knobs/evidence_kit.rs` gains `EVIDENCE_KIT_CONTROLS` (list, default empty), the `EVIDENCE_KIT_CONTROL_<name>` family (declared, no derived member) and `EVIDENCE_KIT_CONTROL_SINCE` (scalar, default empty). The family rule's *two declared families* sentence is re-phrased to name three. `evidence-kit/templates/evidence-config.knobs` gains the three knobs, commented out.
- **`native/src/evidence.rs`** gains the declaration parser and the iteration-start reader, each with a directive citing its section.
- **The release declaration** (`.workflow/release-declarations.md`): under `## Behavior changes`, declared test controls, the control line, and a `--run-validate` exit 1 when a control witnesses no red; under `## Knob changes`, the three knobs; and the new gate, named as opt-in.
- **`scripts/gates.list`**: an `# unregistered: check-test-control` line whose reason is that this tree declares no control, the registration being a consumer's selection.
- **doctrine-kit/DOCTRINE.md rule 26**: *Enforced by*'s shipped list gains evidence-kit's test controls (evidence-kit/SPEC.md §Test controls), which witness a recorded test failing against the defect it names. The buildable-but-unbuilt branch-coverage assertion stays, since it asks a different question.
- **`evidence-kit/README.md`**: its gate roster gains `check-test-control` and its tool sentence the control run.
- **§Producers and consumers**: bullets for the declaration, the control line and the since commit.

### (5) Cases and the mirrors {mechanical}

Each **Not yet applied.**

- **`gate-tests/check-test-control.test.sh`** and the gate's `good/`/`bad/` pair, as delta 3 states.
- **`gate-tests/check-evidence-manifest.test.sh`**: a control line with no suite line above it is red, and a malformed `witnessed=` is red.
- **The spine's cases**: a `gate-tests/` case declaring a control on a fixture suite whose scenario fails under the variable records `witnessed=red` and exits 0; a control whose scenario still passes records `witnessed=green` and exits 1; a malformed declaration exits 2 before the claim; and a re-run supersedes the suite's control lines.
- **Crate unit tests**: the declaration parser's refusals, the witness mapping, and `trajectory.rs`'s extractor skipping a control line.
- **The regenerated surfaces**: the mirrors `docs/evidence-kit/SPEC.md`, `docs/evidence-kit/README.md` and `docs/doctrine-kit/DOCTRINE.md`, and the graph artifact for the new descriptor, each by the command docs/generated-projections.md rosters for it.

## Producers and consumers

- **The declaration** (delta 1). Producer: consumer config. Consumers: the spine's control run, and `check-test-control` (A), (B) and (C).
- **The control line** (delta 2). Producer: the spine. Consumers: `check-evidence-manifest` (B) for its grammar and placement, and `check-test-control` (C), which reads every field: `control=` to match a declaration, `scenario=` against it, `witnessed=` for the red, the date for the window, and `commit=` and `cmd=` against the suite line. The trajectory extractor skips it by its third field.
- **The since commit** (delta 3). Producers: the state file's first data line, else `EVIDENCE_KIT_CONTROL_SINCE`. Consumer: `check-test-control` (B).
- **The control run's exit** (delta 2). Producer: the spine. Consumers: its caller, as the existing exit 1 already is.

The spine's spawn sites were found by `git grep -n "spawn(" -- native/src/emit/run_validate.rs`, the iteration-start reader by `git grep -n "iteration_start" -- native/src`, and the registry's unregistered form by `git grep -n "# unregistered:" -- scripts/gates.list`. The manifest's line readers are the ones the predicate amendment probed (evidence-kit/SPEC-gate-run-predicate.md §Producers and consumers).

## Existing sections updated

- `evidence-kit/SPEC.md` §Layout and configuration (deltas 1, 4), §bin/run-validate.sh (delta 2), the new §Test controls (deltas 1, 2), §Evidence manifest (delta 2), the new §check-test-control (delta 3), §check-evidence-manifest (delta 4), §Producers and consumers (delta 4).
- `native/src/evidence.rs`, `native/src/emit/run_validate.rs`, `native/src/gates/evidence_manifest.rs`, the new gate module, `native/src/knobs/evidence_kit.rs` and `native/src/emit/trajectory.rs`'s tests, with their cases (deltas 1, 2, 3, 4, 5).
- `evidence-kit/checks/check-test-control.gate` and its fixture pair, `evidence-kit/gate-tests/`, `evidence-kit/README.md`, `evidence-kit/templates/evidence-config.knobs`, `scripts/gates.list`, `doctrine-kit/DOCTRINE.md` rule 26, `.workflow/release-declarations.md`, and the regenerated surfaces delta 5 lists (deltas 3, 4, 5).

## Retired spellings

- None — no knob, arm or field is renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — all green: `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests, the fixture suites of evidence-kit, gate-sdk, drift-kit and doctrine-kit, and `--run-consumer-smoke`. The entry's Done move lands before the drain stage, in the build batch that lands it.
