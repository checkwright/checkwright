# SPEC amendment: baseline-grain

Closes `validate-baseline-suite-coverage`. Re-measured at this stage against
`c2cdef35` rather than carried from the entry: `.workflow/validate-baseline.txt`
carries rows for **22** suites while `EVIDENCE_KIT_SUITES` configures **24**, and
the two with no row are `dispatch_guard_tests` and `native_crate` — the same two
the 2026-08-14 drain found, unchanged.

**The entry reserved a design call and this amendment rules it.** *Is the
baseline per-suite or per-scenario for a suite this size?* It is
**per-scenario**, and the consequence is the useful half: **a suite's scenario
granularity is its parser's**, so a suite whose whole verdict is one scenario is
a suite that has not been given a parser, and under-coverage is a *parser*
question rather than a row-count one. That single move dissolves the entry's own
objection to a mechanical backfill — "adding twenty-four rows because
twenty-four suites exist is the maintained-roster shape derivation-first refuses"
— because what a suite owes is not a row per arm hand-written into a file, it is
a parser that derives its arms.

**What a rowless suite actually loses, stated precisely, because the entry's
readers keep over-reading it.** `lib/evidence.sh:166`'s rule is *an observed
failure absent from the baseline is a new failure*, so a rowless suite going
**red is still caught**. What a `pass` row alone buys is that the scenario going
**absent** reds too. For `dispatch_guard_tests` and `native_crate` under the
`exit-code` parser the scenario name *is* the suite name, so the uncovered
failure mode is exactly one: the suite silently ceasing to run — dropped from
`EVIDENCE_KIT_SUITES`, or renamed under a config edit. One row each closes it.

## What changes

### (1) The granularity contract: a row is a claim about a scenario, and scenarios come from the parser

§Baseline manifest states what a row asserts and where a scenario comes from: a
baseline row is a **held-constant claim about one scenario**, and a suite's
scenario set is whatever its configured parser emits. Two consequences follow and
are stated with it. A suite carrying one scenario has a baseline that asserts
about the suite as a whole and nothing finer, which is **adequate** where the
suite's arms are not independently meaningful and **empty** where they are. And
finer coverage is bought by **configuring a parser**, never by hand-authoring
rows — the rows follow the parser's output, so they are derived from a run rather
than maintained against one. **{design-bearing}**

**The degenerate end of the axis is named because it is attested.**
`installer_smoke` sits on the `exit-code` parser, so its whole verdict is one
scenario and its baseline row is that scenario at `fail`. Enumerate the outcomes
and the coverage is not thin but **empty**: any non-zero exit matches the
baselined `fail` and reads clean, and a zero is an unpromoted recovery, which is
also not a red. A suite in that state asserts nothing at all, and §Baseline
manifest must say so where a consumer writing a baseline will read it.

### (2) `check-evidence-baseline` gains a suite-coverage arm

A fourth assertion joins grammar, liveness and manifest↔disk coverage: **every
configured suite carries at least one baseline row**. Red (exit 1) names each
configured suite with no row. The suite set is read from `EVIDENCE_KIT_SUITES`
through the config bridge's array channel (`walk::knob_array`), which the crate
already provides. **{design-bearing}**

**This is a derived obligation, not a maintained roster, and the distinction is
the whole justification.** Nothing enumerates suites in the gate: the roster is
`EVIDENCE_KIT_SUITES`, which this repo already *derives* for its fixture half
(`gate_fixture_suites` in `scripts/evidence-config.sh`) and hand-lists for the
rest. A suite added there acquires the obligation with no edit anywhere else, and
a suite removed drops it. That is derivation-first satisfied rather than worked
around.

**It is the enforcement half of delta 1 and lands in the same unit as the fix**,
which is the enforcement-first rule: the two rows delta 3 adds close today's
instance, and this arm is what stops the twenty-fifth suite landing rowless.
Without it, the entry recurs at the next suite.

**Fail-closed (exit 2)** on the shape the existing arms already refuse for: an
`EVIDENCE_KIT_SUITES` that resolves to nothing where a baseline file exists is a
config the gate cannot judge, not a clean run — the same reasoning
gate-sdk/SPEC.md §Fail-closed contract applies to an unresolvable knob elsewhere.
A consumer configuring **no** suites at all disarms the arm cleanly, matching
`check-evidence-manifest`'s no-cursor early-out: a gate with nothing to say says
nothing at a declared branch rather than falling through two live assertions.

### (3) The two rowless suites gain their rows

`dispatch_guard_tests` and `native_crate` each gain one `pass` row, at the
scenario name their `exit-code` parser emits, which is the suite name.
**{mechanical}**

**`native_crate` stays on `exit-code` and does not take `libtest`, and the
refusal is recorded because the parser already exists.** `lib/evidence.sh:99`
ships a `libtest` adapter that would turn `cargo test`'s output into one scenario
per test — roughly ninety of them — and the entry's own framing ("the crate's own
92-test suite… the arm the next port cohort leans on hardest") points straight at
it. It is refused here. A red in that suite is **already** caught by the
absent-row rule, so `libtest` would buy per-test **absence** detection over the
fastest-moving surface in the tree, where a renamed or deleted test is routine
and reds the baseline for a non-defect. That is the maintained-roster cost with
ninety rows instead of twenty-four. The reserved call is available again the day
the crate's test names stop churning, and the trigger is a measurement, not a
taste: the rename rate falling to where a baseline row survives an iteration.

### (4) `installer_smoke` gains an arm parser, which is what makes its baseline assert anything

`EVIDENCE_KIT_PARSER_installer_smoke` becomes a consumer command emitting **one
scenario per printed arm** of `installer/consumer-smoke/run-smoke.sh` — the arms
the smoke already names on stdout as it enters them (`build`, `pack`, `install`,
`profile invariant`, the binary-less leg, and the download, toolchain-free,
jq-less, upgrade, seam, narrowing and artifact arms — twelve in total, re-counted
against the script rather than against this list). Consumer config, not a kit
change, exactly as the entry's third instance predicted. **{design-bearing}**

**The smoke's fail-fast shape is what makes this work, and it is the
non-obvious part.** `run-smoke.sh:16`'s `fail()` exits 1 at the first failure, so
arms after it are never printed. Under the arm parser those arms are **absent**,
and `lib/evidence.sh`'s directional rule reds a baselined `pass` scenario that is
red **or absent**. So an early abort does not hide the arms behind it — it reds
every one of them. The property that today collapses ten arms into one
uninformative verdict becomes the mechanism that gives each of them a real one.

**The baselined failure keeps its slot and its arms behind it are `ignore`, not
`pass`.** Today's single row baselines the whole suite at `fail` for the
binary-less leg. Under the arm parser that row narrows to the binary-less arm,
and the arms the abort prevents from running are recorded at `ignore` — a
non-verdict, which §Baseline manifest already defines and which is the honest
status for an arm nobody observed. When the binary-less failure is repaired, those
rows are promoted to `pass` in the same commit that earns the recovery, which is
the baseline's ordinary promotion motion and not a new one.

**The row set is written from a run, never from this file.** Delta 1's contract
is that rows follow the parser's output; authoring them here would be the
hand-maintained roster the same delta refuses. The build observes one clean run
and records what it emitted.

## Producers and consumers

**The suite-coverage assertion (delta 2)** — the one new interface.

- *Producer:* `native/src/gates/evidence_baseline.rs`, on every
  `check-evidence-baseline` run, after the grammar arm and beside the existing
  coverage arm. **Enabling config actually set:** `EVIDENCE_KIT_SUITES` is set by
  `scripts/evidence-config.sh:5,8,12` in this tree today and reaches the gate
  through the config bridge's array channel, which `walk::knob_array` already
  implements and other members already use — so the arm is live on this repo's
  next battery run rather than reachable only under a fixture.
- *Consumer:* the committing session and CI, through the gate's output contract —
  `precommit` tier, the generated pre-commit hook, `run-gates.sh`, and
  `run-gate-tests.sh` through the fixture pair. No new consumer is introduced.
- *Named reader for every field:* the arm emits one record per uncovered suite,
  carrying the suite name and nothing else, read by the same session in the same
  run. No field is added to the baseline file's `<suite> <scenario> <status>
  [<slug>]` grammar, so no reader of that grammar must learn anything.

**The two new baseline rows (delta 3)** are new *data* in an existing format.
Their producer is the build session writing them; their consumers are
`bin/diff-baseline.sh` and `run-validate.sh`'s verdict arm, both of which already
read every row, at the scenario-diff transition (`lib/evidence.sh:137-166`).

**The `installer_smoke` arm scenarios (delta 4)** are new scenario **names**.

- *Producer:* the configured parser command, run by `run-validate.sh` over the
  captured suite log, once per validate run.
- *Consumer:* `ek_diff_baseline`, at the same transition as every other
  scenario, with no special case — which is the test that the delta is
  configuration and not a mechanism change.
- *Named reader:* each row's status is read by that diff and by nothing else. The
  `ignore` rows delta 4 introduces are read at exactly one transition — the
  `fail`/`ignore`-observed-green branch — and are written at no other.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 4 **narrows** nothing but **replaces** one scenario with many, which
removes the name `installer_smoke` from the observed set. Each affected reader is
enumerated by its **red condition**:

- `ek_diff_baseline` over the existing `installer_smoke installer_smoke fail`
  row — reds when a baselined `pass` scenario is red or absent. **The baselined
  status is `fail`, so it reds on nothing**, and its scenario going absent is
  invisible: a `fail`/`ignore` row is a non-verdict in the absent direction. That
  is precisely why the row must be *rewritten* in the same commit rather than
  left to be caught — nothing would catch it. This is the attested trap of point
  5 in its exact shape.
- `check-evidence-baseline`'s **liveness** arm — reds when a `fail`/`ignore`
  row's slug resolves to no live queue task, and reds on a slug present only
  under `## Done`. **Not monotone**: it holds a resolution condition, so delta 4's
  rewritten rows must carry a resolving slug or the gate goes red on the commit
  that lands them. The existing row's slug moves with it.
- `check-evidence-baseline`'s **manifest↔disk coverage** arm — reds when a
  baseline scenario has no matching file or a file no baseline line, for suites
  carrying a configured scenario glob. **Not monotone** (set equality). Cleared by
  inspection: `installer_smoke` carries no entry in
  `EVIDENCE_KIT_SCENARIO_GLOBS`, and neither do the two suites delta 3 touches,
  so the arm's population is unchanged.
- `check-evidence-manifest` assertion (A) — reds at a `close` cursor on a
  configured suite with no `verdict=clean` line. **Holds a minimum, so not
  monotone.** Cleared by inspection: no delta adds or removes a configured suite;
  deltas 3 and 4 change what a suite *reports*, never whether it runs.
- `run-validate.sh`'s own verdict — returns non-zero iff a new-failure fired.
  **Not monotone under delta 4**: an arm that was silently absorbed can now fire.
  That is the delta's purpose, and it is also its risk — the build's first run
  after landing the parser is the oracle for which arms are genuinely `pass`, and
  the rows are written from it rather than predicted.

## Existing sections updated

- `evidence-kit/SPEC.md` §Baseline manifest — the granularity contract, the
  scenario-comes-from-the-parser rule, and the named degenerate case where a
  one-scenario baseline asserts nothing (delta 1).
- `evidence-kit/SPEC.md` §check-evidence-baseline — the invariant sentence names
  a fourth assertion; the fail-closed list gains the unresolvable-suite-set case
  and the declared no-suites early-out; the knob roster gains
  `EVIDENCE_KIT_SUITES` as an input this member now reads (delta 2).
- `evidence-kit/SPEC.md` §lib/evidence.sh — the `libtest` adapter's entry records
  that it is available and deliberately unused for `native_crate`, with the
  churn ground and the re-open trigger, so the next reader does not read the
  silence as an oversight (delta 3).
- `evidence-kit/SPEC.md` §Layout and configuration — `EVIDENCE_KIT_PARSER_<suite>`
  is exercised by a second consumer suite; the per-suite override's contract is
  re-read to confirm it already covers a command whose scenarios are arm names
  rather than test names (delta 4).
- `.workflow/validate-baseline.txt` — two rows added and the `installer_smoke`
  row rewritten into its arm set; written from an observed run (deltas 3 and 4).
- `scripts/evidence-config.sh` — `EVIDENCE_KIT_PARSER_installer_smoke` and the
  parser command it names (delta 4).
- `installer/consumer-smoke/run-smoke.sh` — no behavioural change; its arm
  headers become a **parsed** contract rather than narration, so the file states
  that its printed arm lines are read by the validate parser and may not be
  reworded silently (delta 4).
- `.github/workflows/gates.yml`, the install-smoke leg — its own text states the
  limit that `installer_smoke`'s single baselined scenario asserts nothing; that
  sentence is retired with the limit (delta 4).
- `native/src/gates/evidence_baseline.rs` and
  `evidence-kit/checks/check-evidence-baseline.gate` — the module gains the arm
  and the descriptor declares the knob it now reads (delta 2).
- `evidence-kit/gate-tests/check-evidence-baseline/{good,bad}` and
  `gate-tests/check-evidence-baseline.test.sh` — the pair gains an uncovered-suite
  case and the behavioural test takes the two fail-closed branches, matching how
  the existing arms split their coverage (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The retired `installer_smoke` scenario name is
      the one identifier to grep for.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Every row is written from a run** — the `installer_smoke` arm rows and
      the two `pass` rows are recorded from an observed `bash
      evidence-kit/bin/run-validate.sh`, never authored from this file.
- [ ] **The coverage arm is red before it is green** — the new arm is observed
      failing on the two uncovered suites *before* delta 3's rows land, or it has
      not been shown to assert anything.
