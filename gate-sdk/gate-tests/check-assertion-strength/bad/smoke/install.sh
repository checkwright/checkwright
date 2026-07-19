#!/usr/bin/env bash
# The attested shape: a bare `if` discriminates only zero from non-zero, so it
# accepts PAUSE (1) and STALE (2) alike, under a message asserting specifically
# that the call did not PAUSE. A STALE regression passes under the wrong name.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

if bash "$SMOKE_KIT_ROOT/bin/verdict.sh" "$snap" >/dev/null 2>&1; then
    echo "smoke: verdict did not PAUSE on a live 95% reading" >&2
    exit 1
fi
