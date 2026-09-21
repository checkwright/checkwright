# SPEC amendment: fixture-discharge

§Fixture-pair discipline says a rule narrowed in a shared module discharges the
fixture contract only once every `<tests-dir>/<gate>/` that module backs has been
re-run, kit-owned and consumer-owned alike. Nothing runs them at commit time: the
battery runs gates, not fixture suites. So the obligation is a grep the author has to
remember, and missing it has cost a stage-late repair commit twice. The entry asked
whether the fix is a new arm, an argument shape on `--run-gate-tests`, or a widening
of what commit time runs. **This amendment rules the third, scoped to the commits
that can move a shared module's verdict. `check-crate-arms` gains a fixture arm that
runs every registered fixture suite when the crate source changed. No new arm or name
is minted.**

**The measurement (2026-09-21).**

- **The suite roster is already derived.** `registry::fixture_suites()` is the one
  fixture-suite derivation, and `--emit-fixture-suites` prints it. It gives eleven
  rows: the ten kit roots' `gate-tests/` and the consumer's `scripts/gate-tests`. So
  the consumer-owned pair behind a shared module, `check-docs-nav-reachable`'s, the
  shape that filed the entry, is in the roster by derivation, and no fan-out
  resolution is needed.
- **Running all of them is affordable where it matters.** Eleven suites (129 case
  pairs plus 104 bespoke tests) take 16.1 s of wall-clock, against a 41 s full
  battery. The only commits that can move a shared module's verdict touch the crate,
  and those already pay `check-crate-arms`' clippy and `cargo test` under the same
  source-stamp cache.
- **Why not a manual arm.** A `--gate-fixtures <gate>` arm would turn the grep into
  one command, but it is still a command the author has to remember. The failure
  it would close is remembering, not typing. An argument shape on `--run-gate-tests`
  would overload a positional that today means a tests-dir.

## What changes

### (1) `check-crate-arms` runs every fixture suite as its third arm {design-bearing}

**Not yet applied.** After the lint and test arms, and under the same predicate,
`check-crate-arms` runs `--run-gate-tests` over each row of
`registry::fixture_suites()`. The predicate is: crate present, `cargo` present, and
the source stamp missed. The arm runs whether or not the first two failed, so one
report carries all three. A suite's failure is the gate's failure. The report
names the suite and the case, and the help line gives the one-suite re-run command.

- **The binary it exercises** is `GATE_SDK_NATIVE_BIN`, the one every suite already
  runs. The arm is meaningful only with `check-gate-binary-fresh` green, which the
  same commit's battery requires. The section says so rather than making the arm
  rebuild.
- **The stamp cache** is unchanged. A fixture-only edit is not re-run by this arm:
  its author edited that case knowingly, and the fixture discipline's per-case run
  covers it.
- **Tiering.** `precommit`, as today. The added wall-clock falls only on a commit
  that misses the stamp.

### (2) §check-crate-arms and §Fixture-pair discipline state the arm {mechanical}

**Not yet applied.**

- In §check-crate-arms, the paragraph **Two arms, and the absent third is a contract
  rather than an omission: there is no formatter arm** becomes "Three arms, and the
  absent fourth…". It keeps the formatter ruling verbatim. A new paragraph states
  delta 1's arm, its binary-freshness dependency, and its measured cost.
- In §Fixture-pair discipline, the sentence "Skipping it is invisible at commit
  time, because the pre-commit battery runs gates and not fixture suites" is
  replaced. The new text says the obligation is discharged at commit time for a crate
  edit by `check-crate-arms`' fixture arm. What it leaves to the author is an edit
  outside the crate that moves a gate's verdict, such as a descriptor's arguments,
  and that is re-run per case.

## Producers and consumers

- **The fixture arm (delta 1).** Producer: `check-crate-arms`, registered at
  `precommit`, and triggered by the crate-source couples it already carries.
  Consumers: the battery and hook, through the gate's existing verdict. It reads
  `registry::fixture_suites()`, which already has one reader, `--emit-fixture-suites`.
  No knob, arm or gate is minted, and no roster changes. The gate is `install: never`,
  so no vendored tree registers it by `init`. build adds a Behavior-changes bullet to
  `.workflow/release-declarations.md` for a consumer that registered it by hand.
- **Point 5.** Nothing narrows. The gate gains a red condition, and a fixture suite
  failing is already a red in its own runner.
- **Point 6.** The corpus is the eleven derived suites, and each one's satisfying
  value is its present clean run, measured above.

## Existing sections updated

Rosters from `--emit-fixture-suites`, reading §check-crate-arms and §Fixture-pair
discipline, and `grep -n stamp native/src/gates/crate_arms.rs`.

- gate-sdk/SPEC.md §check-crate-arms and §Fixture-pair discipline (delta 2).
- `native/src/gates/crate_arms.rs` and `.workflow/release-declarations.md`
  (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — no name is removed.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the arm.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The formatter ruling survives the
      renumbering untouched.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `gate-fixture-fanout-arm` moves to Done in the merge commit,
      at a stage before the drain stage.
- [ ] **Fails closed.** A crate unit test drives the arm over a synthetic suite with
      a failing case and asserts the gate's red.
- [ ] **Measured at landing.** A stamp-missing commit's added wall-clock is recorded
      in the landing commit's message, against this amendment's 16.1 s.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
