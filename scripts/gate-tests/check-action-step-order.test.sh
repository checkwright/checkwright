#!/usr/bin/env bash
# Behavioral test of check-action-step-order: the two empty-knob cleans, which a
# one-verdict case cannot hold. Each runs over the reject case's tree, so a skip
# that still read a job would red, and each clean line must name its own knob.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # scripts/, the gates dir declaring the member
CASE="$DIR/gate-tests/check-action-step-order/bad"

fails=0
for knob in GATE_LOCAL_STEP_ORDER_ACTION GATE_LOCAL_STEP_ORDER_PROGRAMS; do
    out="$( cd "$CASE" && gate_env "$knob=" && gate_run check-action-step-order "$DIR" tree 2>&1 )"; rc=$?
    if [[ "$rc" -ne 0 ]]; then
        echo "  FAIL [$knob]: want exit 0 under an empty knob, got $rc -- $out"; fails=$((fails + 1))
    elif ! grep -qF "ACTION-STEP-ORDER: clean ($knob is empty" <<<"$out"; then
        echo "  FAIL [$knob]: exit 0 but the clean line does not name the empty knob: $out"; fails=$((fails + 1))
    fi
done

out="$( cd "$CASE" && gate_run check-action-step-order "$DIR" tree 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] || { echo "  FAIL [control]: the same tree under both knobs set exited $rc, not 1 -- $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-action-step-order.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-action-step-order.test.sh: clean (an empty action and an empty program set each skip clean naming their own knob, and the same tree reds with both set; 3 cases)"
exit 0
