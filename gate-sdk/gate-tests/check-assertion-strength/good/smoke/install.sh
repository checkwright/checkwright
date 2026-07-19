#!/usr/bin/env bash
# The compliant shape: the guard captures the status and compares it to the code
# its message claims, so the message asserts exactly what the guard established.
# The second guard names OK, which the callee's header binds to code 0 — a
# truthiness guard discriminates 0 exactly, so it is honest as written.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

bash "$SMOKE_KIT_ROOT/bin/verdict.sh" "$snap" >/dev/null 2>&1 && vrc=0 || vrc=$?
if [[ "$vrc" -ne 1 ]]; then
    echo "smoke: verdict on a live 95% reading: want exit 1 (PAUSE), got $vrc" >&2
    exit 1
fi

bash "$SMOKE_KIT_ROOT/bin/verdict.sh" "$other" >/dev/null || {
    echo "smoke: poller snapshot did not verdict OK" >&2; exit 1; }
