#!/usr/bin/env bash
# spec: friction-kit/SPEC.md — PreToolUse(Bash) hook (consumer copy): block, steer, or auto-allow
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="bash-guard"
FRICTION_KIT_LIB="${FRICTION_KIT_LIB:-friction-kit/lib/guard.sh}"
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open if absent
source "$FRICTION_KIT_LIB" 2>/dev/null || exit 0

cmd="$(guard_read_command)" || exit 0

# spec: friction-kit/SPEC.md §Consumer rules — project block/steer/allow rules go here, before the generic ruleset (this repo adds none)
guard_generic_rules "$cmd"      # rules 1-8 (see friction-kit/SPEC.md)
guard_log_fallthrough "$cmd"    # rule 9: log anything neither blocked nor auto-allowed
exit 0
