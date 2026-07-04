#!/usr/bin/env bash
# Synthetic gate for the check-gate-assertions bad-case fixture: the contract
# enumerates (A)+(B) but only marker A survives in code — the drift the gate
# exists to catch. Must be REJECTED with 'missing marker(s): B'.
set -uo pipefail

# assertion A: first thing the gate verifies about foo
echo "checking foo axis A"

echo "checking foo axis B (marker comment lost in an edit)"
