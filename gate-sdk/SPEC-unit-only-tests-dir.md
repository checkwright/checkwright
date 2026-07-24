# SPEC amendment: unit-only-tests-dir

<!-- Delta for the `scratch-execution-prompt-friction` unit of the
     permission-posture-reconciliation iteration — the gate-sdk half of a unit
     that spans guard-kit, gate-sdk, and this repo's registration surfaces.
     Merges into gate-sdk/SPEC.md on completion; delete this file then. -->

## What changes

`bin/run-gate-tests.sh` treats a tests dir holding **only** bespoke unit tests as
a supported shape. Today it enumerates `<tests-dir>/*/` and, finding no fixture
subdirectory, exits 2 (`no gate fixture dirs under <dir>`) — the harness-error
class, meaning *malformed fixture tree*. The `<tests-dir>/*.test.sh` lane runs
strictly after that guard, so a dir carrying unit tests and no fixture pair
reports a malformed tree and its tests never execute.

The guard becomes a test of **emptiness, not of shape**: exit 2 only when the
tests dir has *neither* a fixture subdirectory *nor* a `*.test.sh`. A dir with
either — or both — proceeds; the pairs lane runs over however many pairs exist
(possibly zero) and the unit lane over however many unit tests exist (possibly
zero). No other behavior moves: the good/bad contract, the `expect.txt`
requirement, the harness-vs-logic exit-class split, and the run-after-the-pairs
ordering are all unchanged.

**Why this is a gate-sdk shape and not a guard-kit workaround.** A kit that
registers no gates has no gate to build a fixture pair *around*, yet it can still
ship mechanism (`lib/`, `bin/`) that needs testing. gate-sdk already recognizes
exactly this kind of kit — `gate_kit_roots` keys on `checks/` **or** `smoke/`
precisely so a gateless kit is discovered (guard-kit/SPEC.md §Testing). The
runner's guard is the one remaining place that assumes every tests dir is
fixture-first. Closing it here fixes the shape for every such kit rather than
routing one kit around it.

**Ruled out — leaving the runner alone and homing the bespoke test in the
kit's own runner.** For guard-kit that would mean teaching
`bin/run-guard-tests.sh` to run `guard-tests/*.test.sh`. Rejected: the
hermeticity contract is enforced by `check-test-hermetic`, whose assertion A
enumerates `<kit-root>/gate-tests/*.test.sh` only — so a test rehomed outside
`gate-tests/` silently escapes the bootstrap-or-marker obligation. That trades
away the enforcement that is the whole reason gate-sdk owns the bespoke-unit-test
lane, to avoid a change to the lane's owner.

**Not a licence for a pair-less gate.** A registered gate still owes its
`good/`+`bad/` fixture pair (§check-fixture-pair). This amendment widens what a
*tests dir* may contain, never what a *gate* may omit.

## Producers and consumers

- **Changed state: the runner's "nothing to test" verdict.**
  - *Producer:* `bin/run-gate-tests.sh`, the pre-flight guard currently at
    `:69-75`, which enumerates `"$TESTS_DIR"/*/` under `nullglob`. It gains the
    `*.test.sh` enumeration (today performed at `:106-108`, after the pairs loop)
    as a second term, so the guard reads both counts before deciding. Enabling
    config: none — the runner takes its tests dir from `$1` /
    `GATE_SDK_TESTS_DIR`, unchanged.
  - *Consumer:* the caller's exit code — the fixture-runner battery
    (README.md §This repo, governed) and any consumer invoking the runner
    directly. Exit 2 keeps its meaning (the dir names no tests at all, so the
    invocation was a mistake); a dir with tests of either kind now reaches the
    lanes and reports through the existing `GATE-TESTS:` summary line.
- **Changed field: the summary line's pair and unit counts.**
  - *Named reader:* the operator or agent reading the runner's stdout at the end
    of a battery line. A unit-test-only dir prints `0 pairs` alongside its unit
    count — the count that makes the shape legible rather than silent. No gate
    parses this line.

## Existing sections updated

- **gate-sdk/SPEC.md §run-gate-tests** — the section states that
  `<tests-dir>/*.test.sh` unit tests "run after the pairs"; it never says whether
  a dir may carry unit tests without pairs, which is how the pair-less shape came
  to be assumed workable. It gains one sentence: a tests dir may hold fixture
  pairs, bespoke unit tests, or both, and the runner exits 2 only when it holds
  neither — so a gateless kit can ship bespoke tests with no fixture pair to give.

## Definition of Done

- [ ] **Causal completeness** — the changed verdict has a reachable producer (the
      pre-flight guard, reading both enumerations) and a named consumer (the
      caller's exit code); the summary-line counts have a named reader.
- [ ] **Merged with no information lost** — the supported-shape sentence
      integrated into §run-gate-tests; the merged SPEC reads coherently for a
      reader who never saw this file.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — no name retired by this change.
- [ ] **Gaps filed** — any cross-component gap found during build resolved that
      session, not deferred.
