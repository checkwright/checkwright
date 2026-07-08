#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The delegation model — PreToolUse(Agent) budget guard (consumer copy): block on PAUSE, advise otherwise
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block/guard_advise)
GUARD_NAME="agent-budget-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
VERDICT_BIN="${DELEGATION_KIT_VERDICT_BIN:-delegation-kit/bin/usage-verdict.sh}"
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open if absent
source "$GUARD_KIT_LIB" 2>/dev/null || exit 0

verdict="$(bash "$VERDICT_BIN" 2>&1)"; rc=$?

case "$rc" in
    1) guard_block "$verdict"$'\n'"corrective: the verdict names the axis that fired — a 5h PAUSE clears when that window resets (hours); a 7-day PAUSE costs days, so pause delegation and let the supervisor carry the week. To override deliberately, re-run with the matching knob raised (DELEGATION_KIT_PAUSE_PCT for 5h, DELEGATION_KIT_PAUSE_PCT_7D for the weekly axis)." ;;
    *) guard_advise "budget verdict (agent-budget-guard): $verdict" ;;
esac
