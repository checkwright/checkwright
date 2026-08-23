#!/usr/bin/env bash
# Synthetic gate for the check-gate-assertions bad-case fixture: an enumerated
# contract exists but the code carries no marker at all -- the retrofit
# obligation, distinct from a marker set that merely disagrees.
set -uo pipefail

echo "checking zero, with nothing marked"
