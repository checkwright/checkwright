#!/usr/bin/env bash
# Synthetic gate for the check-gate-assertions good-case fixture. Its markers use
# the `//` leader rather than `#`, and the second is indented the way a module
# body's marker is: the leader is the substrate's and the marker is a code marker
# either way, so both spellings must be read.
set -uo pipefail

: <<'MARKERS'
// assertion A: the first thing the gate verifies about qux
    // assertion B: the second thing the gate verifies about qux
MARKERS

echo "checking qux"
