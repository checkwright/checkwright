# SPEC amendment: case-env

Pairs `case-pin-outranked-by-env`. A scalar knob exported in the environment outranks every knob file, on purpose, so that a harness can pin a scalar by exporting it (§The knob file). The same order lets whatever the *invoker* exported outrank what a test means to run under. Two instances landed at one build:

- **A fixture pair.** A workflow step's export armed `check-psscriptanalyzer`'s pair against the real analyser on CI alone.
- **A bespoke test.** An exported `GATE_SDK_VERBOSE` redded `run-arm-contract.test.sh`'s quiet case under every verbose battery, and `--run-config-variant` always runs one.

The deliverable is the catcher at the source: each harness hands a test the knobs it means to run under. The alternative the entry offers, a check over workflow `env:` blocks and bespoke tests' unpinned reads, is refused. It would gate one channel of the export, the workflow, and leave a seat's own exports and an arm that exports for its children, such as `--run-config-variant`, to fail first on a run.

## What changes

### (1) A fixture case reads its knob layers from its own directory, and its own pins outrank the invoker's {design-bearing} {user-facing: case-pin-outranked-by-env's Deliverable, its first branch, the fixture harness scrubbing what a case pins}

§run-gate-tests already states the pair's hermeticity: the case dir is the child's working directory, so a gate resolving its knob files under the cwd finds only the case's own. Two kinds of inherited variable defeat that today, and the runner removes both from the resolving shell and from the case invocation before it applies its own pins:

- **The file-layer locators.** These are `GATE_SDK_GATES_DIR`, and each static kit's `<KIT>_KNOB_FILE`, `<KIT>_LOCAL_KNOB_FILE` and `<KIT>_CONFIG_FILE`. An inherited one moves a layer out of the case dir: an absolute `GATE_SDK_GATES_DIR` points every case at the invoker's own configuration, which `--run-config-variant` exports into its scratch battery.
- **Every scalar the case's own knob files set.** With the locators gone, the case reads `<case>/scripts/<stem>-config.knobs` and its `.local.knobs` overlay, for each static kit's stem (§The knob file). The runner reads every scalar line in those files, a declared family's members included, and removes the names they set. So the case's value is the one the member resolves.

**The resulting order for one case**, highest first:

1. The runner's own pins: the binary, the scratch dir and the gate-sdk root.
2. The case's knob files.
3. The invoker's environment, for a scalar the case does not set.
4. The kit default.

**A scalar the case leaves unset still inherits, deliberately.** It is how a seat hands a case a per-machine value no tracked case file can carry. A PowerShell host is the instance: `pwsh` on one seat and `powershell` on another, read by a consumer's PowerShell gate pair. **Honest limit:** a case that leaves a knob unset and reds under some exported value is still non-hermetic. Its remedy is to pin the knob in its file, which then outranks the export.

The runner keeps its existing rules for its pins. A case pinning `GATE_SDK_NATIVE_BIN` keeps its pin, and the scratch pin outranks a case's own `GATE_SDK_TMP_DIR`, both as §run-gate-tests states them today.

Replacement for §run-gate-tests, the paragraph beginning **Fixture-pair hermeticity is the case dir**, its first sentence. **Not yet applied.**

> Fixture-pair hermeticity is the case dir as the child's working directory, and the case's own knob layers outranking the invoker's. Before its pins, the runner removes from the resolving shell and from the case invocation every file-layer locator (`GATE_SDK_GATES_DIR`, and each static kit's `<KIT>_KNOB_FILE`, `<KIT>_LOCAL_KNOB_FILE` and `<KIT>_CONFIG_FILE`) and every scalar a scalar line of the case's own knob files sets. A gate resolving its knob files under the cwd then finds only the case's own, and its value for each scalar the case sets is the case's. A scalar the case leaves unset still reaches it from the invoker, which is how a seat hands a case a per-machine value such as a PowerShell host. A fixture may ship its own cwd-relative config deliberately.

Replacement for §run-gate-tests, the paragraph beginning **A bespoke `*.test.sh` reaches its gate the same way, through `gate_run`**, its last two sentences. **Not yet applied.**

> The `--run-gate-tests` arm, by contrast, **does** read the case's knob files: it runs each case in the case dir, strips every inherited file-layer locator and every scalar the case's files set, and then applies its own pins. The asymmetry is stated here because it is invisible at the call site that suffers from it.

### (2) A bespoke test inherits no variable under a kit's prefix {design-bearing} {user-facing: case-pin-outranked-by-env's Deliverable, its first branch, `lib/test-hermetic.sh` scrubbing what a case pins}

A bespoke test pins every kit's file layers to one empty file, so its stated contract is that "the gate runs on kit defaults" (§lib/test-hermetic.sh). Read as pins, that is every knob pinned to its default, and the inherited environment is the one layer the library leaves open. So, before it exports anything, the library unsets every exported variable whose name begins with a rostered kit's prefix. That prefix is the uppercased name its kit loop already derives, followed by `_`, gate-sdk's `GATE_SDK_` included. The library then sets its own pins as it does today.

- **One name is kept: `GATE_SDK_NATIVE_BIN`.** The already-set guard keeps an invoker's value by contract, for a test run against a binary it pins itself.
- **The library's two existing unsets, `GATE_SDK_TMP_DIR` and `GATE_SDK_WORKFLOW_DIR`, become cases of the rule.** Their paragraph is rewritten to name the rule, and its honest limit narrows: a knob derived from either root and exported directly is now unset too.
- **A test still pins a value after its `source` line**, by `gate_env` or a later assignment, which the scrub cannot reach, since it runs before.

The library stays knob-free: it reads no knob to decide what to unset, only the kit roster it derives from its own location.

Replacement for §lib/test-hermetic.sh, the paragraph beginning **The library also unsets**. **Not yet applied.**

> **The library first unsets every inherited variable under a rostered kit's prefix**, `GATE_SDK_` and each `<KIT>_`, except `GATE_SDK_NATIVE_BIN`, whose invoker-set value the binary pin below keeps. A bespoke test then runs on kit defaults whatever its invoker exported, whether a workflow's `env:` block, a seat's shell, or an arm such as `--run-config-variant` that exports a setting for its children. It still pins a value after its `source` line, which by construction comes later. Among what this clears are `GATE_SDK_TMP_DIR` and `GATE_SDK_WORKFLOW_DIR`, so a sourcing test resolves both against its own sandbox cwd and not the invoker's live state, a real `run-validate.lock` or a foreign `validate-evidence.txt`, and so is every knob derived from either root and exported directly.

### (3) The oracles {design-bearing}

- **`gate-sdk/gate-tests/run-gate-tests.test.sh` gains a row.** It builds a scratch pair whose case knob file pins a scalar its gate prints, and runs the runner under three contrary exports, asserting the case's value each time:
  - the scalar itself;
  - `GATE_SDK_GATES_DIR` pointing elsewhere;
  - the kit's `<KIT>_KNOB_FILE` pointing at an empty file.

  A fourth export, of a scalar the case does not set, must reach the member.
- **A new bespoke test, `gate-sdk/gate-tests/lib-test-hermetic.test.sh`, holds delta 2.** It sources the library as its first act, as every bespoke test must. It then spawns a child bash that exports `GATE_SDK_VERBOSE`, a `CANON_KIT_` scalar, `GATE_SDK_GATES_DIR` and `GATE_SDK_NATIVE_BIN`, sources the library, and prints what survives. It asserts that only `GATE_SDK_NATIVE_BIN` and the library's own pins remain.
- **The instance re-run.** The full fixture battery passes under an exported `GATE_SDK_VERBOSE=1`. `run-arm-contract.test.sh`'s own `GATE_SDK_VERBOSE=''` pin is then redundant, and stays as that case's statement of what it runs under.

Two claims remain for the build's full suite run to settle, with this tree and the CI legs as the oracle.

**Inferred, cannot run before build:** no bespoke test relies on an inherited kit-prefixed variable other than `GATE_SDK_NATIVE_BIN` — the scrub that would expose one lands at build; the read so far, `git grep -nE '\$\{?(GATE_SDK|[A-Z]+_KIT)_[A-Z_]+' -- '*gate-tests/*.test.sh'` over the 136 tracked bespoke tests, hit only corpus text a test writes or a test's own pin.

**Inferred, cannot run before build:** no bespoke test that drives `--run-gate-tests` relies on its inner cases reading the empty knob file its inherited `<KIT>_KNOB_FILE` named — the locator strip lands at build, and the suite run after it is the oracle.

## Producers and consumers

- **The case scrub (delta 1).**
  - Producer: `native/src/emit/run_gate_tests.rs`, once per case.
  - Enabling config: none. Every `--run-gate-tests` run reaches it, `check-crate-arms`' fixture arm and validate's fixture suites included.
  - Consumers: the resolving `bash` spawn and the case invocation, through their environments.
  - Names: the static kits' prefixes and stems come from the crate's static kit table (`knobs::STATIC_KITS`). The scalar names come from the crate's one knob-line grammar, so neither is a roster the runner keeps.
  - The spawn faces already carry a strip list (`proc::run_merged_scrubbed` takes names to unset). Probe: `grep -n "pub fn run_merged" native/src/proc.rs`.
- **The bespoke scrub (delta 2).**
  - Producer: `gate-sdk/lib/test-hermetic.sh`, at its source.
  - Enabling config: none. Every bespoke test sources it, which `check-test-hermetic` assertion A enforces.
  - Consumers: every gate and arm the test drives afterwards.
- **No field and no corpus narrows**, so points 4 and 5 add nothing.
- **Point 6: one corpus-wide obligation.** It is the second inferred premise in delta 3, that every bespoke test passes with nothing inherited. Its members are the tracked `*gate-tests/*.test.sh` set, and each one's satisfying value is a pass under the battery's fixture arm, which runs them all.

## Existing sections updated

The roster below came from `grep -n "hermetic\|outranks\|exported" gate-sdk/SPEC.md` over §The knob file, §lib/test-hermetic.sh and §run-gate-tests.

- `gate-sdk/SPEC.md` §run-gate-tests, the hermeticity paragraph and the asymmetry sentence (delta 1). The paragraph stating that the scratch pin outranks a case's own `GATE_SDK_TMP_DIR` stands.
- `gate-sdk/SPEC.md` §The knob file, the paragraph **The environment beats the file, on purpose**. It gains one clause: a test harness strips what a test pins before it pins, §run-gate-tests and §lib/test-hermetic.sh (deltas 1 and 2).
- `gate-sdk/SPEC.md` §lib/test-hermetic.sh, the unset paragraph (delta 2).
- `gate-sdk/SPEC.md` §check-test-hermetic, assertion A's exposure paragraph. It names the bootstrap as the fix for inherited files, and gains "and inherited variables" (delta 2).
- `docs/gate-sdk/SPEC.md`, the generated mirror, regenerated by the command its freshness gate prints (deltas 1 and 2).
- `native/src/emit/run_gate_tests.rs` (delta 1).
- `gate-sdk/lib/test-hermetic.sh`, its header directive and the scrub (delta 2).
- `gate-sdk/gate-tests/run-gate-tests.test.sh` (delta 3).
- `gate-sdk/gate-tests/lib-test-hermetic.test.sh`, new (delta 3).

## Retired spellings

- None — no delta of this amendment retires a name; the scrub adds behavior to two existing surfaces.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it.
- [ ] **The instances re-run** — every fixture suite and bespoke test passes under an exported `GATE_SDK_VERBOSE=1`, and under `--run-config-variant` over a variant directory that changes no knob.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
