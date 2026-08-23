#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The turn-end liveness probe (template) — SubagentStop probe (consumer copy): log one line per firing, emit no hook JSON, exit 0 unconditionally
set -uo pipefail

LOG="${DELEGATION_KIT_STOP_LOG:-${GATE_SDK_WORKFLOW_DIR:-.workflow}/subagent-stop-liveness.log}"
# spec: delegation-kit/SPEC.md §The turn-end liveness probe (template) — this consumer's own reader, the adapter that reaches check-producer-liveness by name; the template ships no default because only a consumer knows its front end
LIVENESS_CMD="${DELEGATION_KIT_LIVENESS_CMD-scripts/producer-liveness-reader.sh}"
RUN_DIR="${GATE_SDK_TMP_DIR:-.tmp}"

# spec: delegation-kit/SPEC.md §The turn-end liveness probe (template) — every value is one whitespace-free token, so a payload string can never split the space-delimited line
sanitize() {
    local v="${1//[[:space:]]/_}"
    printf '%s' "${v:--}"
}

payload="$(cat 2>/dev/null)"

event='-'
session='-'
keys='-'
if [[ -n "$payload" ]] && command -v jq >/dev/null 2>&1; then
    event="$(sanitize "$(printf '%s' "$payload" | jq -r '.hook_event_name // "-"' 2>/dev/null)")"
    session="$(sanitize "$(printf '%s' "$payload" | jq -r '.session_id // "-"' 2>/dev/null)")"
    keys="$(sanitize "$(printf '%s' "$payload" | jq -r 'keys_unsorted | join(",")' 2>/dev/null)")"
fi

shopt -s nullglob
records=("$RUN_DIR"/*.run)
shopt -u nullglob

verdict=unavailable
live=no
if [[ -n "$LIVENESS_CMD" && -r "$LIVENESS_CMD" ]]; then
    # spec: delegation-kit/SPEC.md §The turn-end liveness probe (template) — the bounded call: a reader that hung would refuse the turn end by accident, which is the blocking variant this one is not
    if command -v timeout >/dev/null 2>&1; then
        timeout 10 bash "$LIVENESS_CMD" "$RUN_DIR" >/dev/null 2>&1
    else
        bash "$LIVENESS_CMD" "$RUN_DIR" >/dev/null 2>&1
    fi
    case "$?" in
        0) verdict=green ;;
        1) verdict=red; live=yes ;;
        2) verdict=corrupt ;;
        *) verdict=unavailable ;;
    esac
fi

ts="$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null)"
printf -v line '%s  event=%s  session=%s  live=%s  verdict=%s  records=%s  keys=%s' \
    "${ts:--}" "$event" "$session" "$live" "$verdict" "${#records[@]}" "$keys"

logdir="${LOG%/*}"
[[ "$logdir" == "$LOG" || -d "$logdir" ]] || mkdir -p "$logdir" 2>/dev/null
{ printf '%s\n' "$line" >> "$LOG"; } 2>/dev/null

exit 0
