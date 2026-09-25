# SPEC amendment: arm-test-floor

gate-sdk/SPEC.md §The non-gate arm says what a non-gate arm does not owe: a descriptor, a registration and a fixture pair. It says nothing about what testing it does owe. So an arm can change behaviour with nothing red, and four arm-table rows resolve to a file with no test module.

**The ruling: an arm owes a unit test in its own file, and a crate test beside the arm table holds that the test module exists.** §check-crate-arms' test arm already runs every crate test at each crate-touching commit, so the floor needs no new gate, no knob and no descriptor. The crate stays outside every kit root (§Consumer payload), so the rule binds contributors, and adopters get nothing to configure.

**Why the floor is the test module's presence, not a test that calls the arm's entry.** The census below found that most arm files test their helpers and never call their own `emit` or `run`. A floor requiring an entry call would red most of the table and turn a four-file fix into a rewrite of the table's tests. A presence floor catches the four files that have no test at all, and the normative sentence in delta 1 says what the test is for.

**Why a test elsewhere does not stand in.** `--emit-usage-trend` has no test module of its own, yet `native/src/usage_tests.rs` drives its entry over delegation-kit's fixtures, including its argv refusals. Letting a test elsewhere count would make the census find `#[cfg(test)]`-gated call sites across the crate. That is a parser for a question the file layout already answers: the arm's own grammar test lives beside the arm, and the kit's fixture runner keeps the fixture cases.

**Measured at authoring (2026-09-26):**

- The census reads every `Arm::Emit(` and `Arm::Run(` path in `native/src/emit/mod.rs`'s arm table, resolves each to its file, and greps that file for `cfg(test)`. It finds four files without one: `emit/md_unwrap.rs`, `emit/value_rollup.rs`, `emit/install_hooks.rs` and `emit/usage_trend.rs`. Every other row's file has a test module, including the shared ones (`knobs/mod.rs`, `registry.rs`, `walk.rs`, `runner.rs`, `doctrine.rs`, `hook/mod.rs`, `hook/verdict.rs`, `hook/poll.rs`, `hook/statusline.rs`).
- `usage_tests.rs` asserts `usage_trend::emit`'s `--help` refusal, its `-notapath` shape refusal and its `--` escape. The first two fail in `positionals` before any knob is read. The escape reads a copy of the fixture history.
- `md_unwrap::emit` refuses an empty file list or a `--`-led operand with `USAGE` before `declaration_leads()` reads its knob. `install_hooks::dispatch` refuses a dash-led argument through `file_survey::positionals` before either knob is resolved. `value_rollup`'s `token_cell`, `count_of` and `axis` are pure over a `Join`.
- Tests that read crate source at test time already use `env!("CARGO_MANIFEST_DIR")` (`native/src/proc.rs`, `native/src/knobenv.rs`).

## What changes

### (1) The section states the floor {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §The non-gate arm, the second of the three properties becomes:

> - **It owes no `.gate` descriptor, no `gates.list` registration and no `good/`+`bad/` fixture pair, and it owes a unit test.** Those three are the gate contract, attached to a verdict a battery reads, and an arm that returns a document has no pass and no fail to fixture. The test is a `#[cfg(test)]` module in the file implementing the arm, which §check-crate-arms' test arm runs. It exercises the arm's own rule on input it builds: its argument grammar, or the document or status it returns. A crate test elsewhere or a kit smoke that also drives the arm adds to that module and never stands in for it. A unit test beside the arm table resolves each row's function to its file and reds any file with no test module. It checks presence, never coverage. So a shared file such as `walk.rs` passes on tests of its other functions, and a hardcoded top-level flag, which has no row, is outside the check.

### (2) The arm table's census test {design-bearing}

**Not yet applied.** `native/src/emit/mod.rs`'s test block gains one test. It reads `src/emit/mod.rs` under `env!("CARGO_MANIFEST_DIR")` and takes the arm table's extent, from the `ARMS` declaration to its closing `];`. It collects the function path after every `Arm::Emit(` and `Arm::Run(` token there. It resolves each path to a file: a `crate::a::b::f` path to `src/a/b.rs`, or `src/a/b/mod.rs` where that is the file, and a bare `m::f` to `src/emit/m.rs`. It asserts that each file carries a `#[cfg(test)]` line. The failure names every offending row and its file in one message. It also asserts that the collected row count equals `ARMS.len()`, so a row the text scan misses reds rather than escaping the census.

### (3) The four files brought to the floor {mechanical}

**Not yet applied.** Each file gains a `#[cfg(test)]` module holding its own rule:

- `native/src/emit/md_unwrap.rs`: `emit` refuses with `USAGE` for no operand, for `--write` alone, for `--help`, and for a file list carrying a `--`-led operand. Every case fails before the knob read.
- `native/src/emit/install_hooks.rs`: `dispatch` refuses `--help` with the arm's `USAGE` in the message, before any knob is resolved.
- `native/src/emit/value_rollup.rs`: on a constructed `Join`, `token_cell` renders `0, 0` as the em dash and otherwise `~<bytes/4>t`; `axis` lists the footprint kits in order, then each enforcement-only kit once and sorted; `count_of` counts a class-and-kit key.
- `native/src/emit/usage_trend.rs`: the `--help` and `-notapath` refusal assertions move here from `native/src/usage_tests.rs`, and neither needs the fixture. The `--` escape stays in `usage_tests.rs`, since it reads the fixture history. That file's comment over the moved block is rewritten to name only the escape.

### (4) delegation-kit's assertion list follows the move {mechanical}

**Not yet applied.** In delegation-kit/SPEC.md §Testing, the usage-trend assertion set's last bullet, "the three argv-shape behaviours the front-end arm owns, which exist in one substrate only.", becomes:

> - the `--` escape reaching a dash-led history path. The shape refusal and its `--help` instance are the arm's own test module's (gate-sdk/SPEC.md §The non-gate arm).

## Producers and consumers

- **The floor.** Producer: the census test, run by `cargo test` under §check-crate-arms at every crate-touching commit and in full on CI and a fresh clone. Consumer: the committing contributor, through that gate's red report, which prints the failing test's message.
- **The obligation.** Every arm-table row, now and future. A new row's author meets it in the commit that adds the row, since the census reds that commit.
- **Fields.** The census's message names row and file. Nothing else is emitted.
- **Point 5.** No corpus narrows. The move in delta 3 narrows `usage_tests.rs`'s trend assertions, and its reader is `cargo test`, which reds only on a failed assertion. So moving an assertion cannot turn it red, and the moved copy keeps both refusals held.
- **Point 6.** The obliged corpus is the arm table's rows, enumerated by the census in the measurement above. Each row's satisfying value is a `#[cfg(test)]` line in its file. The four files lacking one take delta 3's cases, and every other row already carries one.

## Existing sections updated

The roster comes from the census above (the arm table's function paths, each file grepped for `cfg(test)`), from `git grep -n "no fixture pair\|owes no \`.gate\`"` over the tracked tree excluding `docs/`, and from `grep -n "usage_tests\|argv-shape" delegation-kit/SPEC.md`, all run 2026-09-26.

- `gate-sdk/SPEC.md` §The non-gate arm (delta 1).
- `native/src/emit/mod.rs`, the test block (delta 2).
- `native/src/emit/md_unwrap.rs`, `native/src/emit/install_hooks.rs`, `native/src/emit/value_rollup.rs` and `native/src/emit/usage_trend.rs` (delta 3).
- `native/src/usage_tests.rs`, the trend test's argv block and its comment (delta 3).
- `delegation-kit/SPEC.md` §Testing, the usage-trend assertion set (delta 4).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md` and `docs/delegation-kit/SPEC.md`.

## Retired spellings

- None — the deltas add an obligation and move two assertions, and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the floor and its census.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** Delta 1 re-phrases the property bullet, and delta 4 re-phrases one bullet of the assertion list.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `non-gate-arm-testing-floor-unstated` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
