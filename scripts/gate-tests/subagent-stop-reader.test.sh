#!/usr/bin/env bash
# Behavioral test of this repo's wiring of the SubagentStop hook: that the reader
# scripts/subagent-stop-liveness.sh defaults to actually answers, and that its verdict reaches
# the exit code. delegation-kit's own test drives the hook with a stub reader, so it holds every
# verdict arm and cannot see whether the CONFIGURED reader resolves — which is how a port that
# deleted the reader's target left the hook logging verdict=unavailable on every firing with a
# green battery over it. Enforcement lands on exactly that seam, so both lanes move together.
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

# fire <name> <run-dir> <want-rc> — one firing against a scratch run dir, returning the logged
# line. The knob is deliberately NOT set: the default in the consumer copy is the subject.
fire() {
    local name="$1" rundir="$2" want_rc="$3" log rc
    shift 3
    log="$tmp/$name.log"
    printf '%s' "$PAYLOAD" | env "$@" DELEGATION_KIT_STOP_LOG="$log" GATE_SDK_TMP_DIR="$rundir" \
        bash "$HOOK" >/dev/null 2>"$tmp/$name.err"
    rc=$?
    [[ "$rc" -eq "$want_rc" ]] || { echo "  FAIL: $name exited $rc, want $want_rc"; fails=$((fails + 1)); }
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

# A — no records: the reader runs and reports a clean scratch dir, and the turn end is allowed.
#     verdict=unavailable here is the whole defect this test exists for, so it is asserted
#     against by name — and under enforcement it would also silently allow every turn end.
empty="$tmp/empty"; mkdir -p "$empty"
line="$(fire clean "$empty" 0)"
want clean "$line" "verdict=green" "live=no" "records=0" "decision=allow"

# B — one record naming a PID that is always alive: the reader reaches the gate's own liveness
#     predicate rather than merely resolving, so a reader that resolved and answered nothing
#     would still fail here. Through the configured reader that verdict now refuses, so the
#     exit code and the stderr reason are asserted beside the logged line.
live="$tmp/live"; mkdir -p "$live"
printf 'pid=1 run=k\n' >"$live/k.run"
line="$(fire live "$live" 2)"
want live "$line" "verdict=red" "live=yes" "records=1" "decision=refuse"
want live-message "$(cat "$tmp/live.err")" "turn-end refused" "delete the record once the producer has exited"

# C — the reader is configured, readable and resolvable, but the binary its gate dispatches to is
#     absent: the shape a worktree-isolated dispatch takes, whose fresh checkout carries no build
#     output. Over an EMPTY run dir the reading is `unresolved` and it refuses. It must NOT be
#     `unavailable` — that would misreport a wired reader as a tree that never configured one,
#     and it is the arm the stub lane cannot reach because a stub is not the configured reader.
absent="$tmp/absent"; mkdir -p "$absent"
line="$(fire no-binary "$absent" 2 GATE_SDK_NATIVE_BIN=/nonexistent)"
want no-binary "$line" "verdict=unresolved" "live=no" "records=0" "decision=refuse"
want no-binary-message "$(cat "$tmp/no-binary.err")" "turn-end refused" "produced no reading at all"

if [[ "$fails" -gt 0 ]]; then
    echo "subagent-stop-reader.test: $fails assertion(s) failed"
    exit 1
fi
echo "subagent-stop-reader.test: ok (the configured reader resolves and answers, and its verdict reaches the exit: clean scratch dir green and allowed, live producer red and refused with a reason, absent dispatch binary unresolved and refused rather than reported unavailable)"
exit 0
