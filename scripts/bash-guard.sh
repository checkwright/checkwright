#!/usr/bin/env bash
# spec: guard-kit/SPEC.md — PreToolUse(Bash) hook (consumer copy): block, steer, or auto-allow
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="bash-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
[[ -f "$GUARD_KIT_LIB" ]] || exit 0
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open above if absent, but the lib's own exit 2 (set-but-missing config) must stay loud
source "$GUARD_KIT_LIB"

cmd="$(guard_read_command)" || exit 0

# spec: guard-kit/SPEC.md §Consumer rules — project block/steer/allow rules go here, before the generic ruleset
# spec: CLAUDE.md §This repo is governed by its own kits — a hook bypass is a one-off with cause, so it must stay visible: the allowlisted 'git commit -m *' glob would otherwise auto-allow a trailing bypass flag; quoted spans are stripped first so a commit message merely naming the flag passes
cmd_unquoted="$(printf '%s' "$cmd" | sed "s/'[^']*'//g;"' s/"[^"]*"//g')"
case " $cmd_unquoted " in
    *" git commit "*"--no-verify"*|*" git commit -n "*)
        guard_block "a hook bypass (--no-verify/-n) is a one-off with cause, never auto-allowed — fix the red gate instead, or run the bypass yourself with !<command> so the cause is on record."
        ;;
esac
# spec: CLAUDE.md §Housekeeping — .tmp/ is this repo's disposable scratch; the harness's
# per-session /tmp scratchpad leaks session work outside it, so the path prefix is blocked.
# Prefix-only match: kit mechanism legitimately uses TMPDIR (the hermetic bootstrap).
if [[ "$cmd_unquoted" == *"/tmp/claude-"* ]]; then
    guard_block "the harness per-session scratchpad (/tmp/claude-...) is not this repo's scratch home — use repo-local .tmp/ instead (CLAUDE.md §Housekeeping): it survives crashes in-tree and is wiped at the scope boundary. If you genuinely need the harness path, run it yourself with !<command>."
fi
# spec: CLAUDE.md §Housekeeping — .metric/ is gitignored persistent measurement trends and .tmp/ crash-recovery scratch; git clean -x/-X wipes both, so steer to the !<command> escape rather than let the destructive form auto-run
if [[ " $cmd_unquoted " == *" git clean "* && " $cmd_unquoted " =~ [[:space:]]-[A-Za-z]*[xX] ]]; then
    guard_block "git clean -x/-X wipes gitignored state — the irreplaceable measurement trends under .metric/ and crash-recovery resume journals under .tmp/. If you mean to discard them, run it yourself with !<command> so the intent is on record."
fi
guard_generic_rules "$cmd"
guard_log_fallthrough "$cmd"
exit 0
