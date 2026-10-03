#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The delegation model — consumer-copy PreToolUse(Agent) budget guard: block on PAUSE, advise otherwise; register under matcher `Agent`
set -uo pipefail

GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
VERDICT_BIN="${DELEGATION_KIT_VERDICT_BIN:-delegation-kit/bin/usage-verdict.sh}"
[[ -f "$GUARD_KIT_LIB" ]] || exit 0
source "$GUARD_KIT_LIB"

verdict="$("$VERDICT_BIN")"
rc=$?
case "$rc" in
    1) guard_block "$verdict" ;;
    *) guard_advise "budget verdict (agent-budget-guard): $verdict" ;;
esac
exit 0
