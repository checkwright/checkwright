#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The delegation model — PreToolUse(Agent) budget guard: block on PAUSE, advise otherwise
#
# CONSUMER COPY. Copy into your gates dir (default scripts/) and register under
# PreToolUse matcher `Agent` (delegation-kit/SPEC.md §Layout and configuration).
# Composes guard-kit's lib/guard.sh primitives and delegation-kit's
# bin/usage-verdict.sh; override the vendored paths with GUARD_KIT_LIB /
# DELEGATION_KIT_VERDICT_BIN if the kits are vendored elsewhere. Registration is
# the opt-in valve: a consumer wanting pure advice simply does not wire it.
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block/guard_advise)
GUARD_NAME="agent-budget-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
VERDICT_BIN="${DELEGATION_KIT_VERDICT_BIN:-delegation-kit/bin/usage-verdict.sh}"
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open if absent
source "$GUARD_KIT_LIB" 2>/dev/null || exit 0

verdict="$(bash "$VERDICT_BIN" 2>&1)"; rc=$?

case "$rc" in
    1) guard_block "$verdict"$'\n'"corrective: wait for the 5h window to reset, or re-run with DELEGATION_KIT_PAUSE_PCT deliberately raised if this dispatch is worth the budget." ;;
    *) guard_advise "budget verdict (agent-budget-guard): $verdict" ;;
esac
