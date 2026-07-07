#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §wakeup-guard — PreToolUse(ScheduleWakeup|CronCreate)
# hook: deny self-scheduled wakeups, log the attempt.
#
# CONSUMER COPY. Optional second guard, same framework, opposite posture:
# fail-CLOSED. Its hook *matcher* already proves the tool identity, so a
# logging or parse failure still denies. Deliberate scheduling stays possible
# by disabling the hook for a session — the block is the default, not a
# capability removal.
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="wakeup-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
# shellcheck source=/dev/null  # sourced only for the log-path knob; deny stands even if absent
source "$GUARD_KIT_LIB" 2>/dev/null || true
: "${GUARD_KIT_WAKEUP_LOG:=.workflow/wakeup-attempts.log}"

input="$(cat 2>/dev/null || true)"
{
    printf '%s ' "$(date -Is)"
    printf '%s\n' "$input" | jq -c '{session_id, tool_name, tool_input}' 2>/dev/null \
        || printf '%s\n' "$input"
} >>"$GUARD_KIT_WAKEUP_LOG" 2>/dev/null || true

printf '%s\n' "wakeup-guard: ScheduleWakeup/CronCreate is blocked in this repo — a self-scheduled wakeup re-fires its stored prompt in a later session as if the user typed it, long after its premises are stale, and the scheduling call is invisible at the moment it matters. Surface the intent to the user and let them re-invoke instead. Attempt logged (triaged at close alongside the friction log)." >&2
exit 2
