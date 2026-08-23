#!/usr/bin/env bash
# Synthetic gate for the check-gate-assertions bad-case fixture: the contract
# enumerates (A)+(B) and the code grew a third marker nobody wrote into the span.
# Must be REJECTED with an extra-marker finding.
set -uo pipefail

# assertion A: first thing the gate verifies about extra
echo "checking extra axis A"

# assertion B: second thing the gate verifies about extra
echo "checking extra axis B"

# assertion C: a third assertion the code grew with no contract label
echo "checking extra axis C"
