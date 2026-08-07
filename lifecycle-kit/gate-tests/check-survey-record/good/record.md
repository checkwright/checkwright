# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-01-02 scope — which registered gates meet every port criterion?
- corpus: scripts/gates.list */checks/
- oracle: bash gate-sdk/bin/run-gates.sh check-gate-substrate-parity
- rev: 0123456789abcdef0123456789abcdef01234567
- finding: four gates meet all six criteria; the remaining eleven fail the pure-function criterion.

## 2026-01-03 spec — does any kit README still name the retired knob?
- corpus: */README.md
- oracle: none
- rev: 89abcdef0123456789abcdef0123456789abcdef
- finding: a reading of prose, so this block is a note — re-derive before relying on it.
