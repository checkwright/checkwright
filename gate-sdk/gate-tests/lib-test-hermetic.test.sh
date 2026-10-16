#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — inherited kit variables are scrubbed before library pins
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

out="$(bash -c '
    export GATE_SDK_VERBOSE=foreign CANON_KIT_SPEC_NAME=foreign
    export GATE_SDK_GATES_DIR=foreign EVIDENCE_KIT_LOCK_FILE=foreign
    export GATE_SDK_NATIVE_BIN=/pinned/gates
    source "$1"
    [[ ! -v GATE_SDK_VERBOSE && ! -v CANON_KIT_SPEC_NAME && ! -v GATE_SDK_GATES_DIR && ! -v EVIDENCE_KIT_LOCK_FILE ]] || exit 1
    [[ "$GATE_SDK_NATIVE_BIN" == /pinned/gates ]] || exit 1
    [[ -f "$GATE_SDK_KNOB_FILE" && ! -s "$GATE_SDK_KNOB_FILE" ]] || exit 1
    [[ -f "$CANON_KIT_KNOB_FILE" && ! -s "$CANON_KIT_KNOB_FILE" ]] || exit 1
    [[ "$GATE_SDK_ROOT" == "$2" ]] || exit 1
    export GATE_SDK_VERBOSE=deliberate
    [[ "$GATE_SDK_VERBOSE" == deliberate ]] || exit 1
    echo survived
' bash "$GATE_SDK_TEST_LIB_DIR/test-hermetic.sh" "$GATE_SDK_ROOT")" || {
    echo 'FAIL: inherited kit variables survived or library pins were lost'
    exit 1
}
[[ "$out" == survived ]] || exit 1
echo 'LIB-TEST-HERMETIC: clean (inherited kit variables cleared, binary kept, library and later pins applied)'
