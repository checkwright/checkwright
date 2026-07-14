#!/usr/bin/env bash
# spec: guard-kit/SPEC.md — PreToolUse(Bash) hook (consumer copy): block, steer, or auto-allow
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="bash-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open if absent
source "$GUARD_KIT_LIB" 2>/dev/null || exit 0

cmd="$(guard_read_command)" || exit 0

# spec: guard-kit/SPEC.md §Consumer rules — project block/steer/allow rules go here, before the generic ruleset
# spec: CLAUDE.md §This repo is governed by its own kits — a hook bypass is a one-off with cause, so it must stay visible: the allowlisted 'git commit -m *' glob would otherwise auto-allow a trailing bypass flag; quoted spans are stripped first so a commit message merely naming the flag passes
cmd_unquoted="$(printf '%s' "$cmd" | sed "s/'[^']*'//g;"' s/"[^"]*"//g')"
case " $cmd_unquoted " in
    *" git commit "*"--no-verify"*|*" git commit -n "*)
        guard_block "a hook bypass (--no-verify/-n) is a one-off with cause, never auto-allowed — fix the red gate instead, or run the bypass yourself with !<command> so the cause is on record."
        ;;
esac
# spec: CLAUDE.md §Housekeeping — .tmp/ is gitignored scratch holding crash-recovery resume journals; git clean -x/-X wipes it, so steer to the !<command> escape rather than let the destructive form auto-run
if [[ " $cmd_unquoted " == *" git clean "* && " $cmd_unquoted " =~ [[:space:]]-[A-Za-z]*[xX] ]]; then
    guard_block "git clean -x/-X wipes gitignored scratch — including crash-recovery resume journals under .tmp/. If you mean to discard them, run it yourself with !<command> so the intent is on record."
fi
guard_generic_rules "$cmd"
guard_log_fallthrough "$cmd"
exit 0
