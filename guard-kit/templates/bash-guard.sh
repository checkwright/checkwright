#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §Consumer rules — consumer-copy PreToolUse(Bash) hook: project block/steer/allow rules before guard-kit's generic ruleset; wire via templates/settings-hooks.json
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="bash-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open if absent
source "$GUARD_KIT_LIB" 2>/dev/null || exit 0

cmd="$(guard_read_command)" || exit 0

# spec: guard-kit/SPEC.md §Consumer rules — add project block/steer/allow rules here (compose guard.sh primitives), before the generic ruleset [EDIT ME]

guard_generic_rules "$cmd"
guard_log_fallthrough "$cmd"
exit 0
