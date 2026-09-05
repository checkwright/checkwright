#!/usr/bin/env bash
# Behavioral test of this repo's wiring of the SubagentStop hook: that the reader the
# --hook subagent-stop-liveness arm resolves actually answers, and that its verdict reaches
# the exit code. delegation-kit's own test drives the hook with a stub reader, so it holds every
# verdict arm and cannot see whether the RESOLVED reader runs — which is how a port that
# deleted the reader's target left the hook logging verdict=unavailable on every firing with a
# green battery over it. Enforcement lands on exactly that seam, so both lanes move together.
# Since the liveness-reader cut the reader is the binary's own compiled check-producer-liveness,
# reached with the knob unset, so what these arms prove is the whole consumer path — front end,
# binary, compiled gate — rather than a path this repo names.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the preamble above pins every kit's config file at
# an empty one so a KIT test runs on kit defaults; this is a CONSUMER-wiring test, so it un-pins
# delegation-kit's and runs on this repo's real one — which since the liveness-reader cut names no
# reader at all, making these arms the assertion that whatever this repo configures still answers
export DELEGATION_KIT_CONFIG_FILE="$ROOT/scripts/delegation-config.sh"
# spec: gate-sdk/SPEC.md §The non-gate arm — the hook is a binary arm now, dispatched through the
# front-end; the subject of arms A-C is unchanged and only its substrate moved
HOOK_CMD=(bash "$ROOT/gate-sdk/bin/run-gates.sh" --hook subagent-stop-liveness)

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

PAYLOAD='{"session_id":"s-1","hook_event_name":"SubagentStop"}'

# fire <name> <run-dir> <want-rc> — one firing against a scratch run dir, returning the logged
# line. The knob is deliberately NOT set where the default is the subject: the unset knob IS the
# compiled reader, so these arms are what proves the binary reaches its own gate.
fire() {
    local name="$1" rundir="$2" want_rc="$3" log rc
    shift 3
    log="$tmp/$name.log"
    printf '%s' "$PAYLOAD" | env "$@" DELEGATION_KIT_STOP_LOG="$log" GATE_SDK_TMP_DIR="$rundir" \
        "${HOOK_CMD[@]}" >/dev/null 2>"$tmp/$name.err"
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
want clean "$line" "verdict=green" "live=no" "records=0" "runs=-" "decision=allow"

# B — one record naming a PID that is always alive: the reader reaches the gate's own liveness
#     predicate rather than merely resolving, so a reader that resolved and answered nothing
#     would still fail here. That verdict refuses, so the exit code and the stderr reason are
#     asserted beside the logged line, and the reason names the argv the hook actually spawned.
live="$tmp/live"; mkdir -p "$live"
printf 'pid=1 run=k\n' >"$live/k.run"
line="$(fire live "$live" 2)"
want live "$line" "verdict=red" "live=yes" "records=1" "runs=k" "decision=refuse"
want live-message "$(cat "$tmp/live.err")" "turn-end refused" "delete the record once the producer has exited" "runs=k" \
    "check-producer-liveness $live"

# C1 — the hook's OWN binary is absent. Before the port this arm broke only the reader's dispatch,
#      because the hook was a script; the hook and the reader now share one binary and one knob, so
#      GATE_SDK_NATIVE_BIN can no longer break one without the other. What it exercises instead is
#      the rule that replaced the old expectation: gate-sdk/SPEC.md §The non-gate arm fails a
#      harness-integration arm OPEN when it cannot run at all, so an absent binary allows the turn
#      end and writes no record, rather than refusing every turn end in a tree that never built.
absent="$tmp/absent"; mkdir -p "$absent"
line="$(fire no-binary "$absent" 0 GATE_SDK_NATIVE_BIN=/nonexistent)"
[[ -z "$line" ]] || { echo "  FAIL: an absent binary still wrote a record: $line"; fails=$((fails + 1)); }
want no-binary-message "$(cat "$tmp/no-binary.err")" "absent or not" "build-native.sh"

# C2 — the reader is configured and resolvable but produces no reading: over an EMPTY run dir that
#      is `unresolved` and it refuses. It must NOT be `unavailable` — that would misreport a wired
#      reader as a tree that never configured one, and it is the arm the stub lane cannot reach.
#      Driven by an OVERRIDE that exits 2 rather than by the shared binary knob, which C1 now owns.
#      The override carries its own shebang and its own executable bit, which is the whole of what
#      the no-interpreter-word contract asks of a consumer's shell reader — a stub written without
#      them would not spawn at all, and this arm is where that requirement is executable.
mute="$tmp/mute-reader.sh"; printf '#!/usr/bin/env bash\nexit 2\n' > "$mute"; chmod +x "$mute"
line="$(fire unresolved "$absent" 2 DELEGATION_KIT_LIVENESS_CMD="$mute")"
want unresolved "$line" "verdict=unresolved" "live=no" "records=0" "decision=refuse"
want unresolved-message "$(cat "$tmp/unresolved.err")" "turn-end refused" "produced no reading at all"

if [[ "$fails" -gt 0 ]]; then
    echo "subagent-stop-reader.test: $fails assertion(s) failed"
    exit 1
fi
echo "subagent-stop-reader.test: ok (the resolved reader answers and its verdict reaches the exit: clean scratch dir green and allowed, live producer red and refused with a reason naming the argv the hook spawned, an absent dispatch binary declining open with no record, an override that produces no reading unresolved and refused rather than reported unavailable)"
exit 0
