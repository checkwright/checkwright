# SPEC amendment: per-gate-validate-baseline

Per-gate granularity for the validate baseline's `gates` suite: the exit-code
parser holds the whole battery as one scenario, so any future suite-level
held-red baseline row masks a fresh intra-suite regression. With per-gate
scenario rows, an existing gate turning red diffs as a new failure even while
a sibling gate is legitimately held red.

## What changes

1. **`EVIDENCE_KIT_PARSER_<suite>`** (default unset) — a per-suite parser
   override with the same value grammar as `EVIDENCE_KIT_PARSER` (a built-in
   adapter name or a consumer command run on the log); an unset suite falls
   through to the global knob. The dispatch lives in `ek_parse`
   (`lib/evidence.sh`), so both spine callers — `bin/run-validate.sh` and
   `bin/diff-baseline.sh` — inherit it. The name mirrors the
   `EVIDENCE_KIT_RUN_<suite>` convention.

2. **`ek_diff` fail-closed convergence** (debt, in-envelope precondition) — an
   observed non-pass scenario with no baseline row is a new failure.
   §Baseline manifest already asserts this ("A scenario absent from the
   baseline fails closed: the diff treats its failure as a new failure");
   re-verified this session, today's `ek_diff` walks baseline rows only and
   returns 0 for an observed failure absent from the baseline. Under
   suite-granularity the gap is latent (every configured suite carries a
   row); under per-gate granularity it goes live — a newly registered red
   gate would slip through a clean verdict. Code converges on the existing
   SPEC sentence; the only spec delta is citing the branch in
   §bin/diff-baseline.sh.

3. **Consumer side (this repo)** —
   - `scripts/parse-gates-log.sh` (new consumer script): maps the verbose
     run-gates log to `<gate> <pass|fail>` lines off the `  PASS: <gate>` /
     `  FAIL: <gate>` tails (the unresolved-gate FAIL tail included). A log
     with no per-gate tails yields no output, which `run-validate`'s
     produced-no-result guard turns into a run failure — the correct
     fail-closed reading of an early-crashed battery.
   - `EVIDENCE_KIT_RUN_gates` becomes
     `env GATE_SDK_VERBOSE=1 bash gate-sdk/bin/run-gates.sh` — run-gates
     prints per-gate tails only on failure or under `GATE_SDK_VERBOSE`, and
     the parser needs the PASS tails.
   - `EVIDENCE_KIT_PARSER_gates=bash scripts/parse-gates-log.sh` in
     `scripts/evidence-config.sh`.
   - `.workflow/validate-baseline.txt`: the single `gates gates pass` row
     becomes one `gates <gate> pass` row per registered gate — derived once
     from `scripts/gates.list` at seed time, committed by hand (the baseline
     stays human-edited; tooling never writes it).

**Premise correction on record** — the deferred entry's "consumer-side only;
no kit change expected" re-verified false: `ek_parse`'s consumer-command
branch receives the log alone (no suite name, no exit status), so one global
consumer parser cannot reproduce exit-code semantics for the other suites.
The per-suite knob is the minimal kit change. Rejected alternative: passing
suite+status arguments to every consumer parser command — fields this
consumer's parser would never read (causal completeness: a field with no
reader is removed).

**Rejected coverage route** — arming `EVIDENCE_KIT_SCENARIO_GLOBS` for the
gates suite: the glob assertion keys file basenames while gate scenarios are
`gates.list` member names (no `.sh`, consumer-first shadowing would
double-count a shadowed gate). The `ek_diff` fail-closed branch carries the
enforcement instead; a new *passing* gate with no baseline row remains the
SPEC's stated classification cost, seeded at the next human baseline edit.

## Producers and consumers

- **`EVIDENCE_KIT_PARSER_<suite>`** — produced by consumer config (this repo:
  `scripts/evidence-config.sh`, the gates suite — the deployed enabling
  config); read by the `ek_parse` dispatch, reaching both `run-validate` and
  `diff-baseline`.
- **Per-gate scenario lines** — produced by `parse-gates-log.sh` from the
  captured verbose log; consumed by `ek_diff` (the per-scenario diff) and the
  evidence line's `pass=`/`fail=` counts (existing readers; the manifest's
  eight-field shape is unchanged).
- **Per-gate baseline rows** — produced by human commit; consumed by
  `ek_diff` and `check-evidence-baseline` (grammar, slug liveness — existing
  readers).
- **The verbose log** — produced by the `env GATE_SDK_VERBOSE=1` run command
  set in the deployed `EVIDENCE_KIT_RUN_gates`; consumed by the parser; its
  sha256 pins the run in the evidence line as today.

## Existing sections updated

- **§Layout and configuration** — knob row for the per-suite parser override.
- **§lib/evidence.sh** — the parser-adapter paragraph gains the per-suite
  dispatch; the `ek_diff` description gains the observed-absent-from-baseline
  new-failure branch.
- **§bin/diff-baseline.sh** — states the observed-fail-not-in-baseline
  new-failure rule alongside the existing per-scenario split.

## Tests

evidence-kit `gate-tests` / `smoke` additions: per-suite dispatch (a suite
with an override uses it while a sibling suite keeps the global parser);
`ek_diff` observed-fail-absent-from-baseline returns a new-failure; the
consumer parser fixture — a verbose log maps to per-gate lines, an
early-crash log (no tails) maps to no output.

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
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
