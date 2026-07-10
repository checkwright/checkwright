#!/usr/bin/env bash
# spec: guard-kit/SPEC.md — PreToolUse(Bash) hook (consumer copy): block, steer, or auto-allow
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="bash-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open if absent
source "$GUARD_KIT_LIB" 2>/dev/null || exit 0

cmd="$(guard_read_command)" || exit 0

# spec: guard-kit/SPEC.md §Consumer rules — project block/steer/allow rules go here, before the generic ruleset (this repo adds none)
guard_generic_rules "$cmd"
guard_log_fallthrough "$cmd"
exit 0
