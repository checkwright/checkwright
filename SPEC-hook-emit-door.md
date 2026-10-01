# SPEC amendment: hook-emit-door

**Every compiled arm prints its remedy on the binary door, as every compiled gate already does, and `check-door-binding` assertion C holds the hook and emit trees to it.** `gate-output-contributor-door` moved the gate modules' remedies to `crate::gates::door_command` and scoped assertion C's Rust reading to `native/src/gates`. The harness-integration members under `native/src/hook` and the emitters under `native/src/emit` were left out of that corpus. They still tell an adopter to run `bash gate-sdk/bin/run-gates.sh …` or print `usage: run-gates.sh …`, and an adopter's host may have no bash.

**Measured at authoring.**

- **The oracle run.** `check-door-binding` was run with a scratch knob file: this repo's `scripts/guard-config.knobs` plus `GUARD_KIT_DOOR_ROOTS[] = native/src/hook` and `native/src/emit`, passed through `GUARD_KIT_KNOB_FILE`, since an indexed knob takes no environment override. Assertion C reds **38 lines over 24 files**.
- **The classification.** Every line was read and classified: **23** are a usage line or a remedy printed to whoever ran the arm, and **15** are contributor-facing. Of the 15, 8 are `bash -c` scripts that a publisher-only smoke executes in a scratch consumer, and 7 are printed: regeneration recipes on generated pages, and one reproduce line of a source-clone arm.
- **Mentions the oracle does not red.** `grep -rln run-gates native/src --include='*.rs'` names 43 files outside `native/src/gates`. The 19 the oracle does not red name the front end only in comments, in rosters, in test modules, in argv arrays a publisher harness executes, or in files outside both trees.
- **The whole-crate root.** Widening to `native/src` instead also reds the front end's own `--help` text (`native/src/runner.rs`, 21 lines pinned across the bash and PowerShell front ends), the runner's `--only` steer, `native/src/installer/demo.rs` and the `EVIDENCE_KIT_RUN_<suite>` default in `native/src/knobs/evidence_kit.rs`. Those are filed as their own gap, not taken here.
- **The usage-line convention.** The crate's usage lines name the arm and no program, for example `usage: --emit git-hooks …` and `usage: --run-demo`, and the dispatcher's own `Grammar::Flags` form is `usage: --emit <name> …`. Of the 39 single-line `const USAGE` declarations `grep -rhno 'const USAGE: &str = "usage: [^"]\{0,40\}' native/src` prints, 7 lead with `run-gates.sh`. The oracle run adds the multi-line and inline usage lines that do the same, listed under delta 1.

## What changes

### (1) A usage line names the arm and no program

Each usage line below drops its `run-gates.sh` lead and keeps the rest of its grammar unchanged {mechanical} {user-facing: hook-emit-remedy-door's deliverable, that an adopter is never told to run the front end}. One binary answers the arm through every door, so the usage line names the arm. A line that pointed at `run-gates.sh --help` names the arm itself instead, in the dispatcher's no-argument form `usage: --<arm>   (it takes no argument)`. The sites are:

- `native/src/hook/model_verdict.rs` — `USAGE`.
- `native/src/hook/verdict.rs` — `USAGE`.
- `native/src/hook/poll.rs` — the no-argument refusal.
- `native/src/emit/diff_baseline.rs` — `USAGE`.
- `native/src/emit/enter_stage.rs` — `usage()`, every line.
- `native/src/emit/foreign_run.rs` — `USAGE`.
- `native/src/emit/install_hooks.rs` — `USAGE`.
- `native/src/emit/port_blockers.rs` — `USAGE`.
- `native/src/emit/price_coverage.rs` — the no-argument refusal.
- `native/src/emit/run_gate_tests.rs` — `USAGE`.
- `native/src/emit/run_validate.rs` — the no-argument refusal.
- `native/src/emit/usage_trend.rs` — `USAGE`.
- `native/src/emit/wait_probe.rs` — `ROSTER`.

Every one stays a `&'static str` or a plain `format!`, so `Grammar::Parsed` rows are untouched. These exact-text pins move with them:

- `gate-sdk/gate-tests/run-gate-tests.test.sh` (`usage: run-gates.sh --run-gate-tests`);
- `gate-sdk/smoke/install.sh` (`usage: run-gates.sh --emit port-blockers`);
- the unit test in `native/src/emit/usage_trend.rs`.

`install_hooks.rs`' own tests assert `contains(USAGE)` and follow the constant.

### (2) A remedy to run names the binary door

Each remedy below is built with `crate::gates::door_command(<arm and operands>)`, the crate's one door speller (gate-sdk/SPEC.md §run-gates), in place of its `bash gate-sdk/bin/run-gates.sh` text {design-bearing} {user-facing: hook-emit-remedy-door's deliverable, each remedy printed through the gate module's door spelling}. The sites are:

- `native/src/hook/workflow_state.rs` — `unstamped()` (`--enter-stage <stage>`) and `blocked()` (`--enter-stage <stage>` and `--enter-stage --rename <name>`).
- `native/src/hook/dispatch.rs` — `bound_tier()` (`--emit knob-values DELEGATION_KIT_TIER_MODEL`).
- `native/src/emit/always_loaded.rs` — `ceiling_rows()` (`--emit always-loaded --ceiling`). It already returns `Result`, and its second caller, `native/src/gates/surface_ratchet.rs`, already maps that error to exit 2.
- `native/src/emit/diff_baseline.rs` — the missing-status remedy (`--diff-baseline <suite> <log> <status>`).
- `native/src/emit/install_hooks.rs` — the no-hooks-dir remedy (`--emit git-hooks --write`).

**A hook member resolves the door with its other knobs, and a failed read takes the posture the member already gives a knob its rule cannot resolve.** This is the fail-open-but-loud posture of guard-kit/SPEC.md §The fail-open postures, which these members reproduce. `workflow-state-guard` answers `hook::decline` with a note naming the rule and `GATE_SDK_NATIVE_BIN`, as it does for `LIFECYCLE_KIT_STATE_FILE`. `agent-dispatch-guard` leaves D6 inert with a note, as `read_array` does for `DELEGATION_KIT_TIER_MODEL`. An emitter maps the failure to its exit 2, as gate modules do through `door_or_report`.

Each module that starts reading the knob declares `GATE_SDK_NATIVE_BIN` on its row: the hook table's knob slice for the two members, and the arm's `KNOBS` for the emitters. A member's declared knobs are held against what it reads (gate-sdk/SPEC.md §lib/gate.sh). `run_gate_tests.rs` already declares the knob.

### (3) A contributor-facing site declares itself

Each site below takes `// door-contributor: <reason>` on its line or the line above, the Rust-member form of assertion C's declaration (guard-kit/SPEC.md §check-door-binding) {mechanical}.

- **Printed to the contributor who regenerates or reproduces:**
  - `native/src/emit/docs_mirror.rs` — the mirror banner;
  - `native/src/emit/enforcement_map.rs` — `HEAD`;
  - `native/src/emit/footprint.rs` — `PREAMBLE`;
  - `native/src/emit/graph.rs` — `REGEN_CMD`;
  - `native/src/emit/value_rollup.rs` — the block banner;
  - `native/src/emit/run_consumer_smoke.rs` — the reproduce line of a source-clone arm.

  Each emitter already writes the page-side `<!-- door-contributor: -->` declaration into its output, which is a ruling of record. This delta adds the source-side twin the Rust reading needs and changes no emitted byte.
- **Executed by a source-clone smoke, never printed:**
  - `native/src/emit/agents_md_smoke.rs` — three `bash -c` scripts;
  - `native/src/emit/upgrade_smoke.rs` — five `bash -c` scripts.

  These drive the vendored front end on purpose, since the upgrade contract and the AGENTS.md smoke exercise it.

**A door inside a multi-line raw string has no line above it outside the literal.** At the time of writing those are `HEAD` in `enforcement_map.rs` and `PREAMBLE` in `footprint.rs`. The door's line splits into its own literal, joined with `concat!`, so the declaration stands above it. The emitted bytes stay identical, which `check-enforcement-fresh` and `check-footprint-fresh` byte-compare.

### (4) Assertion C's corpus gains the hook and emit trees

`scripts/guard-config.knobs`: `GUARD_KIT_DOOR_ROOTS` gains `native/src/hook` and `native/src/emit` beside `native/src/gates`, and the comment above it names the compiled members' printed remedies rather than the compiled gates' {mechanical}. Each entry is a whole tree, so a module added later is swept unedited. This is consumer config, and no kit default moves.

### (5) The owning sections state the wider set

{mechanical} **Not yet applied.**

gate-sdk/SPEC.md §run-gates, in *The front-end serves a harness shim and a pre-build clone rather than an adopter door*, the second sentence becomes:

> Every kit README, template, knob header and stage procedure names the binary `GATE_SDK_NATIVE_BIN` names, and every compiled gate, hook member and emitter prints its remedy through one crate helper spelling that same door.

gate-sdk/SPEC.md §The bin/-tool contract, in the paragraph *An `--emit-` member declares its argument grammar on its arm-table row*, the sentence on the usage block becomes:

> A member parsing its own argv declares its usage block instead, naming the arm and no program, since one binary answers it through every door, and spells it beneath its own shape refusal, once, since only the member can tell that refusal from a runtime one.

guard-kit/SPEC.md §check-door-binding, the paragraph on a configured `.rs` member:

- Its first sentence becomes: "**A configured member whose name ends `.rs` is read as Rust source, for its output strings**, because a compiled member's printed remedy, a gate's, a hook member's or an emitter's, reaches an adopter as surely as a page does."
- After *A Rust member takes site scope alone* in the two-scopes paragraph, add: "A door inside a multi-line string literal is split into its own literal so the declaration can stand on the line above it."

### (6) The site mirrors follow

`docs/gate-sdk/SPEC.md` and `docs/guard-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 5 {mechanical}.

## Producers and consumers

- **The door-spelled remedies.** Producer: `crate::gates::door_command`, called by the delta 2 sites when their refusal or block fires. It resolves `GATE_SDK_NATIVE_BIN`, which every consumer's gate-sdk table resolves with a default, so no deployed tree leaves it unset. Consumer: the adopter or agent reading the printed block reason, stderr line or hook JSON. The `door_command` call is by in-process reads; the printed text is for a human or agent.
- **The program-free usage lines.** Producer: each delta 1 module's misuse path. Consumers: the caller reading the refusal, and the three exact-text pins named under delta 1, each moved in the same commit.
- **The widened corpus.** Producer: `scripts/guard-config.knobs`. Consumer: `check-door-binding` assertion C, by its tracked-member walk of each configured directory. Its red condition is an undeclared door on a configured surface, and the clean line reports the configured-surface count as a report with no floor (§check-door-binding). The change widens the corpus, so no reader reds on finding none. After deltas 1 through 3 land, the oracle run above is green, which is the build's check.
- **The declared knob.** `GATE_SDK_NATIVE_BIN` joins the knob slice of the `workflow-state-guard` and `agent-dispatch-guard` hook rows and of the emitter arms in delta 2. Readers of those slices: the knob-file derivation, `check-reads-couples` and `check-gate-substrate-parity` (gate-sdk/SPEC.md §lib/gate.sh). Each gains a declared read that matches an executed one.
- **No new state, event, field or name.**

## Existing sections updated

Roster produced by the scratch-knob oracle run above, by `grep -rln run-gates native/src --include='*.rs'` outside `native/src/gates` with each mention read to its test-module cut, and by `grep -rn "usage: run-gates.sh" --include='*.sh' --include='*.ps1' --include='*.md' .` for the pins.

- The `native/src/hook/` and `native/src/emit/` modules named under deltas 1, 2 and 3.
- `gate-sdk/gate-tests/run-gate-tests.test.sh` and `gate-sdk/smoke/install.sh` — the usage pins (delta 1).
- `scripts/guard-config.knobs` — `GUARD_KIT_DOOR_ROOTS` and its comment (delta 4).
- `gate-sdk/SPEC.md` — §run-gates and §The bin/-tool contract (delta 5).
- `guard-kit/SPEC.md` — §check-door-binding (delta 5).
- `docs/gate-sdk/SPEC.md` and `docs/guard-kit/SPEC.md` — the regenerated mirrors (delta 6).

## Retired spellings

- None — the usage lines drop a program word and the remedies change their door; no name a surface cites is retired, and the front end's own help keeps `usage: run-gates.sh`.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
