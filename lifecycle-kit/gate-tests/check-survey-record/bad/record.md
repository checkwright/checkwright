# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-01-02 scope — which registered gates meet every port criterion?
- corpus: scripts/gates.list */checks/
- oracle: bash gate-sdk/bin/run-gates.sh check-gate-substrate-parity
- rev: 0123456
- finding: four gates meet all six criteria; the remaining eleven fail the pure-function criterion.

## 2026-01-03 align — which specs still cite the retired section?
- corpus: */SPEC.md
- oracle:
- rev: 89abcdef0123456789abcdef0123456789abcdef
- finding: two specs still cite it.

## 2026-01-04 build — which fixtures cover the new grammar?
- corpus: lifecycle-kit/gate-tests/
- rev: 3333333333333333333333333333333333333333
- finding: one pair covers it.

## 2026-01-05 build — which commit did the ported cohort land on?
<!-- survey-token-exempt: -->
- corpus: native/src/gates/ as of deadbeef1
- oracle: bash gate-sdk/bin/run-gate-tests.sh
- rev: 2222222222222222222222222222222222222222
- finding: one commit carried the whole cohort.
