#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The delegation model — PreToolUse(Agent) budget guard (consumer copy): block on PAUSE, advise otherwise
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block/guard_advise)
GUARD_NAME="agent-budget-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
VERDICT_BIN="${DELEGATION_KIT_VERDICT_BIN:-delegation-kit/bin/usage-verdict.sh}"
[[ -f "$GUARD_KIT_LIB" ]] || exit 0
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open above if absent, but the lib's own exit 2 (set-but-missing config) must stay loud
source "$GUARD_KIT_LIB"

verdict="$(bash "$VERDICT_BIN" 2>&1)"; rc=$?

case "$rc" in
    1) guard_block "$verdict"$'\n'"corrective: the verdict names the axis that fired — a 5h PAUSE clears when that window resets (hours); a 7-day PAUSE costs days, so pause delegation and let the supervisor carry the week. The full delegation protocol is /agent-execution. To override deliberately, raise the matching knob (DELEGATION_KIT_PAUSE_PCT for 5h, DELEGATION_KIT_PAUSE_PCT_7D for the weekly axis) via the .claude/settings.local.json env block, which the hook re-reads per fire (delegation-kit/SPEC.md §The delegation model)." ;;
    2) guard_advise "budget verdict (agent-budget-guard): $verdict — advisory only: STALE is budget-unknown, never blocks and needs no override (only PAUSE does; delegation-kit/SPEC.md §The delegation model)." ;;
    *) guard_advise "budget verdict (agent-budget-guard): $verdict" ;;
esac
