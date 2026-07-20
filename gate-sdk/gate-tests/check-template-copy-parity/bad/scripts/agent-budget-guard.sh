#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Layout and configuration — the one-sided edit, synthesized: this copy was hand-edited and the template was not.
set -uo pipefail

GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
VERDICT_BIN="${DELEGATION_KIT_VERDICT_BIN:-delegation-kit/bin/usage-verdict.sh}"
THRESHOLD="${DELEGATION_KIT_PAUSE_PCT:-90}"
[[ -f "$GUARD_KIT_LIB" ]] || exit 0
source "$GUARD_KIT_LIB"

verdict="$("$VERDICT_BIN")"
rc=$?
case "$rc" in
    1) guard_block "$verdict" ;;
    *) guard_log_fallthrough "$verdict" ;;
esac
exit 0
