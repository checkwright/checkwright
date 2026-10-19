# SPEC amendment: shellcheck-gate-tests

Pairs `shellcheck-gate-tests-derived`. `check-shellcheck` derives its corpus from the gates dir and five directories of each kit root, and reaches the bespoke `gate-tests/*.test.sh` suites only where a consumer names each directory in `GATE_SDK_LINT_EXTRA_DIRS`. This repo names eleven by hand, so a twelfth kit's suites would join no lint until someone remembered to add them. This amendment derives the suites: each kit root's `gate-tests/` and the consumer's tests dir join the corpus of `check-shellcheck`, and of `check-psscriptanalyzer`, which reads the same corpus. The directory derivation moves into one function both gates call, and this repo's knob shrinks back to the shell under no kit root.

**What the tree does today**, read at spec:

- `target_dirs` in `native/src/gates/shellcheck.rs` builds the gates dir, then each `walk::kit_roots_abs()` root crossed with `KIT_SUBDIRS` (`lib`, `bin`, `checks`, `templates`, `smoke`), then the knob's words, and globs each dir's direct `*.sh`. `targets` in `native/src/gates/psscriptanalyzer.rs` holds its own copy of `KIT_SUBDIRS` and the same three legs, then adds the registered exec members' targets. The `spec:` comment on `target_dirs` says "four directories" where the list holds five.
- This repo's `GATE_SDK_LINT_EXTRA_DIRS` (`scripts/gate-sdk-config.knobs`) names `installer/bin installer/consumer-smoke demo docs` and eleven `gate-tests` dirs: the ten kits carrying one and `scripts/gate-tests`, the tests dir `GATE_SDK_TESTS_DIR` defaults to.
- Every tracked `gate-tests/` dir is one of those eleven (`git ls-files | grep /gate-tests/`). Their direct `*.sh` count 129, and every one is a `*.test.sh`. `drift-kit` is a kit root with no `gate-tests/`. No direct `*.ps1` sits in any of them.
- The fixture pair passes `.` as its one argument (`gate-sdk/gate-tests/check-shellcheck/{good,bad}/args`). A positional replaces the derived set, so neither case reaches the derivation, and the crate's tests cover only the positional override.
- The payload withholds `SPEC.md` and `smoke` from each kit root (`GATE_SDK_PAYLOAD_WITHHOLD`'s default), so a vendored kit carries its `gate-tests/`, as it carries the `lib/`, `bin/`, `checks/` and `templates/` the gate already lints.
- No reader holds a count or a floor over the corpus: a grep of `shellcheck` across `*/SPEC.md`, `scripts/` and `native/src` finds none, and the clean line's script count is printed and read by no test.

**Widening rather than refusing.** The entry left the call to this amendment. The widening adds no new class of subject to an adopter's run. A vendored kit's suites are publisher shell of the same standing as the `lib/` and `templates/` the gate already lints there. The consumer's tests dir is shell the consumer authors, the subject §check-shellcheck names. `init` registers the gate in no profile, so only an adopter who registered it meets the change.

**No act here depends on a sibling's landing.** No other unit of the set touches the two lint gates, their descriptors or the knob file.

## What changes

### (1) The derived corpus takes each kit root's `gate-tests/` and the tests dir {design-bearing} {user-facing: shellcheck-gate-tests-derived's Deliverable, "the derived set gains each kit root's and the gates dir's `gate-tests/` direct `*.sh`, with fixture coverage and the knob list shrunk back to the installer, demo and docs directories"}

`gate-sdk/SPEC.md` §check-shellcheck, the invariant's first sentence reads as follows. The rest of the paragraph stays. **Not yet applied.**

> Invariant: every `*.sh` directly under the consumer gates dir, the consumer tests dir `GATE_SDK_TESTS_DIR`, each vendored kit's `lib/`, `bin/`, `checks/`, `templates/`, `smoke/` and `gate-tests/`, and each directory named in `GATE_SDK_LINT_EXTRA_DIRS` passes ShellCheck at `-S warning` (the self-lint contract).

The same section's **The gate ships to adopters for their own shell.** paragraph: "An adopter's subject is the shell they author under their gates directory, since the vendored kit shell is the publisher's and the publisher's battery lints it." reads "An adopter's subject is the shell they author under their gates and tests directories, since the vendored kit shell, a kit's suites included, is the publisher's and the publisher's battery lints it." **Not yet applied.**

The same section, a new paragraph after the one opening **A narrowing of its corpus is safe**. **Not yet applied.**

> **The suites are derived, never named.** `--run-gate-tests` runs each direct `*.test.sh` of a tests dir as shell, so a kit's suites are shell the kit ships. A derivation that named them per directory would leave a new kit's suites unlinted until a knob was edited. The corpus therefore takes each kit root's `gate-tests/` and the resolved `GATE_SDK_TESTS_DIR`, globbed for `*.sh` like every other directory, so a helper beside the suites is linted too. A widening can only add findings, by the monotone red condition above. A directory both derived and named in the knob may be linted twice and print its findings twice, with the verdict unchanged.

The same section's **The knob widens when the hook fires it too.** paragraph: "couples `knob:GATE_SDK_GATES_DIR/*.sh,kit:*.sh,knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh`" reads "couples `knob:GATE_SDK_GATES_DIR/*.sh,knob:GATE_SDK_TESTS_DIR/*.sh,kit:*.sh,knob:GATE_SDK_LINT_EXTRA_DIRS/*.sh`", and after its last sentence it adds "The tests dir's token fires a relocated tests dir, which the gates dir's does not reach." **Not yet applied.**

`gate-sdk/SPEC.md` §check-psscriptanalyzer, the invariant: "the direct members of the consumer gates dir, of each vendored kit's `lib/`, `bin/`, `checks/`, `templates/` and `smoke/`, and of each directory `GATE_SDK_LINT_EXTRA_DIRS` names" reads "the direct members of the consumer gates and tests dirs, of each vendored kit's `lib/`, `bin/`, `checks/`, `templates/`, `smoke/` and `gate-tests/`, and of each directory `GATE_SDK_LINT_EXTRA_DIRS` names, one derivation both gates call". **Not yet applied.**

`gate-sdk/SPEC.md`, the sentence opening "Every script in the family (the consumer's gates and the kit's own `lib/`, `bin/`, `checks/`, `templates/`, `smoke/`)" reads "Every script in the family (the consumer's gates and tests and the kit's own `lib/`, `bin/`, `checks/`, `templates/`, `smoke/`, `gate-tests/`)". The rest of the paragraph stays. **Not yet applied.**

`native/src/gates/shellcheck.rs`. **Not yet applied.**

- `KIT_SUBDIRS` gains `gate-tests` last, and its `spec:` comment reads "each kit root contributes these six directories and no other, in this order: the target list's order is the order shellcheck reports in".
- `target_dirs`' derived branch becomes two public functions. `lint_dirs_from(gates_dir, tests_dir, kit_roots, extra)` is pure, answering the gates dir, then the tests dir, then each kit root's six directories, then the extra words. `lint_dirs()` resolves `GATE_SDK_GATES_DIR`, `GATE_SDK_TESTS_DIR`, `walk::kit_roots_abs()` and the knob's words and calls it. The `spec:` comment on the pair reads "the derived corpus both lint gates read: the consumer gates and tests dirs, each kit root's six directories, then the extra-dirs knob appended". `target_dirs` keeps the positional override and otherwise calls `lint_dirs()`.
- A unit test, `the_derived_dirs_take_each_kit_root_s_suites_and_the_tests_dir`, asserts `lint_dirs_from`'s answer for two kit roots and one extra word, in order. The function is pure because the crate has no env-serializing test helper (`git grep -n -E "ENV_LOCK|env_lock|fn with_env|serial_env" -- native/src` finds none), so a test setting knobs in the environment would race the parallel suite.

`native/src/gates/psscriptanalyzer.rs`: its `KIT_SUBDIRS` is deleted, and `targets`' derived branch takes its dirs from `super::shellcheck::lint_dirs()`, still reading the gates dir for `exec_targets`. The `spec:` comment on `targets` reads "the derived corpus: §check-shellcheck's directories, then each registered exec member's PowerShell target". **Not yet applied.**

`native/src/gates/mod.rs`, the registry rows of `check-shellcheck` and `check-psscriptanalyzer`: each reads list gains `GATE_SDK_TESTS_DIR`. **Not yet applied.**

`gate-sdk/checks/check-shellcheck.gate`, the `# graph:` couples gain `knob:GATE_SDK_TESTS_DIR/*.sh` after the gates dir's token, and `gate-sdk/checks/check-psscriptanalyzer.gate`'s gain `knob:GATE_SDK_TESTS_DIR/*.ps1`. `check-gate-fixture-coverage.gate` already couples `knob:GATE_SDK_TESTS_DIR/*`. **Not yet applied.**

### (2) The fixture pair reaches the derivation {design-bearing}

`gate-sdk/gate-tests/check-shellcheck/`. Each case drops its `args` file, so the gate runs on its derived set, and pins its kit roots in a case-local knob file. A case without the pin would derive the real gate-sdk root the runner exports. **Not yet applied.**

- **Both cases:** `scripts/gate-sdk-config.knobs` holds `GATE_SDK_KIT_DIRS = alpha-kit` under a `spec:` comment naming §check-shellcheck. The gates dir is the default `scripts`, and the tests dir its default `scripts/gate-tests`. The fixture scripts are named `*.sh`, not `*.test.sh`, so no gate reading suites by suffix takes them for one.
- **`good/`:** `alpha-kit/gate-tests/kit-ok.sh` and `scripts/gate-tests/tests-ok.sh`, each the current `ok.sh` body. `expect.txt` reads `SHELLCHECK: clean (2 scripts)`, so the case reds unless both legs are read. `ok.sh` and `args` are deleted.
- **`bad/`:** `alpha-kit/gate-tests/kit-warn.sh` and `scripts/gate-tests/tests-warn.sh`, each the current `warn.sh` body. `expect.txt` holds three lines, `SC2164`, `kit-warn.sh` and `tests-warn.sh`, a conjunction. `warn.sh` and `args` are deleted. With either leg missing, a line is absent. With both missing, the target set is empty and the gate exits 2, where the case wants 1.

The crate's existing positional test keeps the override covered.

Inferred, to be confirmed at build: the fixture runner hands a case with no `args` file an empty argv, which `run_case` in `native/src/emit/run_gate_tests.rs` shows, and a relative `GATE_SDK_KIT_DIRS` word resolves against the case dir, as `check-smoke-entry-guard`'s case-local `GATE_SDK_KIT_DIRS = alpha-kit beta-kit` does.

### (3) This repo's knob names only the shell under no kit root {mechanical}

`scripts/gate-sdk-config.knobs`. **Not yet applied.**

- `GATE_SDK_LINT_EXTRA_DIRS = installer/bin installer/consumer-smoke docs`.
- Its `spec:` comment drops the clause about the bespoke `gate-tests/*.test.sh` suites and the walkthrough. It reads "this repo ships shell under no kit root: the installer's two bootstraps, its consumer smoke, and the hosted one-line install script `docs/install.sh`, the only direct `*.sh` member of `docs`; name them so the lint that governs every other script in the tree governs these too".
- `demo` leaves the list too, though the entry's Deliverable names it. No tracked file sits under `demo/` since the walkthrough was ported to the `--run-demo` arm (`git ls-files demo` prints nothing), so the word lints nothing.

The battery's `check-shellcheck` and `check-psscriptanalyzer` then read the same files as before the change: the 129 suites through the derivation, and the three named directories through the knob.

### (4) The release record declares the widening {mechanical} {user-facing: shellcheck-gate-tests-derived's Deliverable, as delta 1}

`.workflow/release-declarations.md`, `## New and tightened gates`, a new bullet beside the `smoke/` widening's. **Not yet applied.**

> - `check-shellcheck`, `check-psscriptanalyzer` — lint each kit root's `gate-tests/` and your tests dir `GATE_SDK_TESTS_DIR` as well, so a suite ShellCheck finds a warning in reds. Nothing to do unless you registered either gate: then fix a finding in your own suites, and drop any `gate-tests` directory you named in `GATE_SDK_LINT_EXTRA_DIRS`, which may now be linted twice.

## Producers and consumers

- **The widened corpus (delta 1).**
  - Producer: `shellcheck::lint_dirs`, over the pure `lint_dirs_from`, called by `check-shellcheck`'s `target_dirs` and `check-psscriptanalyzer`'s `targets` whenever either runs without positionals: in the battery, at the pre-commit hook, and wherever a consumer registers either gate. No knob enables it.
  - Consumer: ShellCheck and PSScriptAnalyzer, spawned with the target list. Their verdicts are each gate's exit code and printed findings, read by the battery and the hook.
  - New reads: `GATE_SDK_TESTS_DIR`, a derived row of gate-sdk's table, read through `walk::knob_scalar`. The two registry rows' reads lists gain it, so the roster-holding reader of declared reads does not red on it.
  - Graph trigger: the `knob:GATE_SDK_TESTS_DIR` couples fire each gate on a staged script in a relocated tests dir. `kit:*.sh` and `knob:GATE_SDK_GATES_DIR/*.sh` already fire on a staged suite under a kit root or the default tests dir.
- **Point 5:** delta 3 narrows the knob, a corpus, and delta 1 widens the derivation by the same directories.
  - The red condition of both gates is a finding in a target, and it is monotone, so the narrowing removes no finding the widening does not restore.
  - Each gate's reds-on-empty arm cannot fire on this tree: the gates dir holds nine direct `*.sh`, and `installer/bin`, `installer/consumer-smoke` and `docs` one each (`git ls-files` over the four).
  - `check-gate-fixture-coverage` reds on a gate without a pair, and the pair stays.
  - No reader asserts a count.
- **Point 6:** delta 1 obliges each direct `*.sh` of every kit root's `gate-tests/` and the tests dir to pass ShellCheck. The members are the 129 suites of the eleven directories above, and each one's satisfying value is its present text: the battery lints all 129 through the knob today and passes (`--run`, at spec, 155 gates passed). No `*.ps1` is a member.

## Existing sections updated

The roster came from `git grep -n "LINT_EXTRA_DIRS\|KIT_SUBDIRS\|check-shellcheck\|check-psscriptanalyzer" -- ':!docs/'` and `grep -n "smoke/" gate-sdk/SPEC.md` over the family sentences.

- `gate-sdk/SPEC.md` §check-shellcheck, §check-psscriptanalyzer, and the family sentence (delta 1).
- `native/src/gates/shellcheck.rs`, `native/src/gates/psscriptanalyzer.rs`, `native/src/gates/mod.rs` (delta 1).
- `gate-sdk/checks/check-shellcheck.gate`, `gate-sdk/checks/check-psscriptanalyzer.gate` (delta 1).
- `gate-sdk/gate-tests/check-shellcheck/` (delta 2).
- `scripts/gate-sdk-config.knobs` (delta 3).
- `.workflow/release-declarations.md` (delta 4).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).
- `docs/check-graph.html`, the coupling-graph artifact the descriptors' couples feed, regenerated by the command its freshness gate prints (delta 1).

## Retired spellings

- None — no name is retired. `target_dirs`, `KIT_SUBDIRS` in `shellcheck.rs` and the knob keep their names. `psscriptanalyzer.rs`'s private copy of `KIT_SUBDIRS` is deleted, and the name survives in `shellcheck.rs`.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The corpus held** — `check-shellcheck` run on the tree before and after the knob shrinks prints the same script count, and the 129 suites are among its targets.
- [ ] **Suites green** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-gate-tests gate-sdk/gate-tests gate-sdk/checks`, and the battery.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, at a stage before the drain stage.
