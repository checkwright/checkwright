# SPEC amendment: observer-pin

The registry coverage tests fail on both Windows triples, and at one case: `check-gate-binary-fresh`'s `good/` fixture. Its knob file sets `GATE_SDK_NATIVE_BIN = ./stub-bin`, an extensionless bash-shebang script, and a Windows host cannot start it. The member exits 2 and the observer's `assert_ne!(run.rc, 2, …)` panics the whole test at the first failing case, so no member after it is observed on Windows at all.

**Measured at authoring (2026-09-27).**

- Run 36338295147 fails `every_registry_member_declares_the_programs_it_spawns` and `…_roots_it_walks` on both Windows triples, each panicking on `check-gate-binary-fresh errored on …\check-gate-binary-fresh\good`. The captured log does not carry the spawn error itself, since the panic message is the assert's own text.
- The stub is the only fixture in the tree that a member spawns through a knob value. The other extensionless shebang files under a `gate-tests/` directory (`check-portability-floor`'s verbs, `check-hook-exec-bit`'s hook, `check-spec-embedded-source`'s fenced examples) are read as text or as index modes, never executed.
- The environment outranks a knob file for a scalar (`native/src/knobs/mod.rs`, `layered_from`), and `proc::run_merged_in` hands the observer child the parent's environment. With `GATE_SDK_NATIVE_BIN` exported as the absolute host binary, `cargo test --release every_registry_member_declares` passes all three tests on Linux. So every member's observation stays inside its declaration when the observer hands the host binary over the case's pin.
- The unix legs of `crate-tests-unix` install ShellCheck and the docs gates' gems before the tests. `crate-tests-windows` installs ShellCheck only. `check-docs-render-fidelity` and `check-docs-liquid-parse` exit 2 when their ruby probe fails (`docs_render_fidelity.rs`, `docs_liquid_parse.rs`), and the observer asserts no exit 2.

**The ruling: the observer pins the binary, and the spawn funnel keeps its pass-through.** The alternative was resolving `PATHEXT` on an extensionless path in the spawn funnel, with a `.cmd` twin beside each stub. It is refused on three grounds. The rule it reverses (gate-sdk/SPEC.md §Fail-closed contract: an `argv[0]` carrying a separator is a value its caller already resolved) still holds for every shipped spawn. The failure is a test harness's, and that reversal would change the spawn semantics adopters' batteries run under. And it would need a second stub per fixture, in a dialect the fixture runner never exercises on Linux. The observer's subject is a member's **reach**, the programs it spawns and the roots it walks, never its verdict, so which binary answers `--source-stamp` does not move what it asserts. The fixture suite still holds the verdict on the stub, on Linux, where `run-gate-tests` keeps a case's own pin.

## What changes

### (1) The observer hands every case the host binary {mechanical}

**Applied** in the landing commit. In `native/src/gates/mod.rs`, `observe_in_case` adds `GATE_SDK_NATIVE_BIN` to the child-scoped environment slice it already passes `proc::run_merged_in`, beside the marker, the member's name and `GATE_SDK_ROOT`. The value is the host binary's absolute path, resolved in the parent against the same repository root `GATE_SDK_ROOT` is absolutized against. The slice is child-scoped, so no `knobenv` write is made and the environment-serialization rule is untouched. The value outranks the case's own knob file, including `check-gate-binary-fresh`'s pin, on every host.

In gate-sdk/SPEC.md §lib/gate.sh (the paragraph beginning **The registry coverage tests are where the rule bites**), the sentence "The member's name, a marker and an absolute `GATE_SDK_ROOT` ride that call's child-scoped environment slice, so the child reads the case's own knob files from its working directory and finds the kits from the locator, as §run-gate-tests hands them to a case." becomes:

> The member's name, a marker, an absolute `GATE_SDK_ROOT` and the absolute host binary as `GATE_SDK_NATIVE_BIN` ride that call's child-scoped environment slice. The child reads the case's own knob files from its working directory and finds the kits from the locator, as §run-gate-tests hands them to a case, with one difference. The binary overrides a case's own pin, where that runner keeps it. A pin exists to steer a member's verdict, and the observer asserts reach, which no pin moves. A stub pinned for the verdict need not start on every host, and the Windows legs cannot start a shebang script.

### (2) The Windows job installs the docs gates' gems {mechanical}

**Applied** in the landing commit, the step spelling its install `ruby -S gem` so the gem runs from the bare `ruby`'s own bin directory with no `.cmd` lookup under Git Bash. In `.github/workflows/gates.yml`, `crate-tests-windows` gains the `crate-tests-unix` step "install the docs gates' gems", placed after its ShellCheck step and before the host build, and carrying the same `# spec:` directive. The step runs `gem install --no-document kramdown-parser-gfm liquid:4.0.4` in the ruby its bare `ruby` resolves, then prints the ruby and liquid versions. The unix step's writability test and `sudo` arm have no Windows meaning, so this step drops them.

In gate-sdk/SPEC.md §check-crate-arms, "The unix legs also install the ShellCheck and gems the `gates` job installs, whose absence exits the same members 2." becomes "Every leg also installs the ShellCheck and gems the `gates` job installs, whose absence exits the same members 2."

**Inferred, cannot run before build:** both Windows runner images put a `ruby` on `PATH` that `gem install` reaches — the step does not exist until this delta lands, and no Windows host is reachable from here; the mid-iteration run prints the versions or fails naming the image.

If an image carries no ruby, the same batch adds a SHA-pinned `ruby/setup-ruby` step to that job, as `actions/checkout` is pinned, and the observation moves to the close push.

### (3) The members past it are observed on Windows {design-bearing}

**Not yet applied.** The mid-iteration push is the observation (lifecycle-kit/SPEC.md §The state machine's push placement). Its `crate-tests-windows` run is read on both triples, as `crate-tests-unix`'s annotation step reads it. The run reaches every registry member for the first time on Windows, so members 116 onward may fail for causes this amendment has not seen. Each failing test is diagnosed in the batch that reads the run:

- **A missing prerequisite**, a tool the job does not install, joins the job's install steps under delta 2's rule, and is observed at the close push.
- **A code defect** in a member or in the crate's Windows arms is escalated to the lead before any fix is written. It is a scope question, because [crate-tests-windows-flip](../TASK-QUEUE.md#crate-tests-windows-flip) is blocked on this entry and moves only when a run is green on both triples.

When the mid-iteration run is green on both triples, the flip entry's `[observed-by: gates]` predicate holds, and the flip rides the close push, as that entry states. When it is not, the flip cannot be observed before the close push. It is then carried as that entry's residue and does not land this iteration.

## Producers and consumers

- **The pinned value.** Producer: `observe_in_case`, per case, in the parent test process. Consumer: the observer child's knob resolution (`walk::knob_scalar`, through `layered_from`), read by every member that resolves `GATE_SDK_NATIVE_BIN`. The members named by `grep -rn 'knob_scalar("GATE_SDK_NATIVE_BIN")' native/src` read it in the observer. Outside the observer nothing reads the slice, because it is child-scoped. The value is the binary CI's "build the host gate binary the registry-coverage tests spawn" step already produces on every leg, so no new build step is owed.
- **The gems step.** Producer: the new step. Consumers: `check-docs-render-fidelity`'s and `check-docs-liquid-parse`'s ruby probes in the observer child.
- **Point 5.** No corpus narrows. The observation set per case is unchanged in kind, and the Linux probe above shows no member's observed set leaving its declaration.
- **Point 6.** No corpus-wide obligation.
- **Sibling dependency.** [crate-tests-windows-flip](../TASK-QUEUE.md#crate-tests-windows-flip) edits the same §check-crate-arms paragraph, deleting its report-until-green sentences, and a different sentence from the one delta 2 edits. If the flip lands in the same batch as delta 2, one commit carries both edits. If it lands in a later batch, it edits the paragraph delta 2 left.

## Existing sections updated

Roster from `grep -n "registry coverage tests are where\|unix legs also install" gate-sdk/SPEC.md` and `grep -n "crate-tests-windows:" -A40 .github/workflows/gates.yml`, run 2026-09-27.

- `native/src/gates/mod.rs` and gate-sdk/SPEC.md §lib/gate.sh (delta 1).
- `.github/workflows/gates.yml`'s `crate-tests-windows` and gate-sdk/SPEC.md §check-crate-arms (delta 2).
- The on-site mirror `docs/gate-sdk/SPEC.md`, regenerated by the command `check-docs-mirror-fresh` prints (deltas 1 and 2).
- The queue: the entry moves under delta 3's rule.

## Retired spellings

- None — no delta retires a name; delta 1 adds a variable to an existing slice and delta 2 adds a step.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the pinned value and the gems step.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `windows-fresh-fixture-stub` moves to Done once the mid-iteration run shows `crate-tests-windows` green on both triples, at a stage before the drain stage, in a commit after that run is read (build's remote-oracle rule). If the run is not green, the entry stays active until a fix is observed, or is demoted under canon-kit's rule with the observation as its increment.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
