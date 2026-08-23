#!/usr/bin/env bash
# Behavioral test of this repo's wiring of the SubagentStop probe: that the reader
# scripts/subagent-stop-liveness.sh defaults to actually answers. delegation-kit's own test
# drives the probe with a stub reader, so it holds every verdict arm and cannot see whether the
# CONFIGURED reader resolves — which is how a port that deleted the reader's target left the
# probe logging verdict=unavailable on every firing with a green battery over it.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HOOK="$ROOT/scripts/subagent-stop-liveness.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

PAYLOAD='{"session_id":"s-1","hook_event_name":"SubagentStop"}'

# fire <name> <run-dir> — one firing against a scratch run dir, returning the logged line.
# The knob is deliberately NOT set: the default in the consumer copy is the subject.
fire() {
    local name="$1" rundir="$2" log rc
    log="$tmp/$name.log"
    printf '%s' "$PAYLOAD" | DELEGATION_KIT_STOP_LOG="$log" GATE_SDK_TMP_DIR="$rundir" \
        bash "$HOOK" >/dev/null 2>&1
    rc=$?
    [[ "$rc" -eq 0 ]] || { echo "  FAIL: $name exited $rc, not 0"; fails=$((fails + 1)); }
    cat "$log" 2>/dev/null
}

want() {  # $1=name $2=line $3.. = substrings the line must carry
    local name="$1" line="$2"; shift 2
    local s
    for s in "$@"; do
        grep -qF -- "$s" <<<"$line" || { echo "  FAIL: $name line lacks '$s': $line"; fails=$((fails + 1)); }
    done
}

cd "$ROOT" || { echo "  FAIL: cannot reach the repo root"; exit 1; }

# A — no records: the reader runs and reports a clean scratch dir. verdict=unavailable here is
#     the whole defect this test exists for, so it is asserted against by name.
empty="$tmp/empty"; mkdir -p "$empty"
line="$(fire clean "$empty")"
want clean "$line" "verdict=green" "live=no" "records=0"

# B — one record naming a PID that is always alive: the reader reaches the gate's own liveness
#     predicate rather than merely resolving, so a reader that resolved and answered nothing
#     would still fail here.
live="$tmp/live"; mkdir -p "$live"
printf 'pid=1 run=k\n' >"$live/k.run"
line="$(fire live "$live")"
want live "$line" "verdict=red" "live=yes" "records=1"

if [[ "$fails" -gt 0 ]]; then
    echo "subagent-stop-reader.test: $fails assertion(s) failed"
    exit 1
fi
echo "subagent-stop-reader.test: ok (the configured reader resolves and answers: clean scratch dir green, live producer red)"
exit 0
