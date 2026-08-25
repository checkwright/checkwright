#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — SubagentStop hook (consumer copy): log one line per firing, emit no hook JSON, exit 2 with a stderr reason on a red, corrupt or unresolved reading and 0 on every other path
set -uo pipefail

LOG="${DELEGATION_KIT_STOP_LOG:-${GATE_SDK_WORKFLOW_DIR:-.workflow}/subagent-stop-liveness.log}"
# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — this consumer's own reader, the adapter that reaches check-producer-liveness by name; the template ships no default because only a consumer knows its front end
LIVENESS_CMD="${DELEGATION_KIT_LIVENESS_CMD-scripts/producer-liveness-reader.sh}"
RUN_DIR="${GATE_SDK_TMP_DIR:-.tmp}"

# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — every value is one whitespace-free token, so a payload string can never split the space-delimited line
sanitize() {
    local v="${1//[[:space:]]/_}"
    printf '%s' "${v:--}"
}

payload="$(cat 2>/dev/null)"

event='-'
session='-'
keys='-'
continuing=no
if [[ -n "$payload" ]] && command -v jq >/dev/null 2>&1; then
    event="$(sanitize "$(printf '%s' "$payload" | jq -r '.hook_event_name // "-"' 2>/dev/null)")"
    session="$(sanitize "$(printf '%s' "$payload" | jq -r '.session_id // "-"' 2>/dev/null)")"
    keys="$(sanitize "$(printf '%s' "$payload" | jq -r 'keys_unsorted | join(",")' 2>/dev/null)")"
    # spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — the contracted loop guard, read only to bound the one arm whose condition never resolves
    [[ "$(printf '%s' "$payload" | jq -r '.stop_hook_active // false' 2>/dev/null)" == true ]] && continuing=yes
fi

verdict=unavailable
live=no
reader_ran=no
reader_status=0
if [[ -n "$LIVENESS_CMD" && -r "$LIVENESS_CMD" ]]; then
    # spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — the bounded call keeps a hung READER from being read as a live PRODUCER: a timeout is an error and allows, so a refusal is only ever the reader's own verdict
    if command -v timeout >/dev/null 2>&1; then
        timeout 10 bash "$LIVENESS_CMD" "$RUN_DIR" >/dev/null 2>&1
    else
        bash "$LIVENESS_CMD" "$RUN_DIR" >/dev/null 2>&1
    fi
    reader_status=$?
    reader_ran=yes
fi

# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — the glob is taken AFTER the reader ran, so a record created during the reading is counted rather than missed and an in-flight record errs toward refusing
shopt -s nullglob
records=("$RUN_DIR"/*.run)
shopt -u nullglob

# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — reader exit 2 splits by record count, and the split names the DIAGNOSIS without deciding the refusal: over a non-empty set it is `corrupt`, over an empty one it is `unresolved`, and both refuse
if [[ "$reader_ran" == yes ]]; then
    case "$reader_status" in
        0) verdict=green ;;
        1) verdict=red; live=yes ;;
        2) if [[ ${#records[@]} -gt 0 ]]; then verdict=corrupt; else verdict=unresolved; fi ;;
        *) verdict=error ;;
    esac
fi

decision=allow
[[ "$verdict" == red || "$verdict" == corrupt || "$verdict" == unresolved ]] && decision=refuse
# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — `unresolved` refuses ONCE: its condition is a reader that cannot run, which no turn content changes, so a second refusal is a loop that spends the session's whole budget; `red` and `corrupt` stay unconditional
[[ "$verdict" == unresolved && "$continuing" == yes ]] && decision=allow

ts="$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null)"
printf -v line '%s  event=%s  session=%s  live=%s  verdict=%s  records=%s  decision=%s  keys=%s' \
    "${ts:--}" "$event" "$session" "$live" "$verdict" "${#records[@]}" "$decision" "$keys"

logdir="${LOG%/*}"
[[ "$logdir" == "$LOG" || -d "$logdir" ]] || mkdir -p "$logdir" 2>/dev/null
{ printf '%s\n' "$line" >> "$LOG"; } 2>/dev/null

[[ "$decision" == refuse ]] || exit 0

# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — the message branch is three-way because the three refusing verdicts have three different findings and three different remedies; folding `unresolved` onto `corrupt`'s arm would print "does not parse" over a case holding no record to parse
ways="Two ways forward: wait for the producer on its own artifact, in a loop that ends when the condition goes true; or delete the record once the producer has exited."
if [[ "$verdict" == red ]]; then
    finding="a launch record under $RUN_DIR names a live producer, so this turn may not end on it"
    look="to see the record set for yourself"
elif [[ "$verdict" == corrupt ]]; then
    finding="a launch record under $RUN_DIR does not parse, so no reading says whether a producer is live and this turn may not end on it"
    look="to see which record is malformed"
else
    finding="the liveness reader produced no reading at all, and there is no launch record under $RUN_DIR for it to have been about, so nothing says whether a producer is live and this turn may not end on it"
    ways="The reader is what to fix here, not a record: it failed over an empty record set, so this is a reader that could not run at all rather than a malformed record. Under a worktree-isolated dispatch, binary-dispatched gates do not resolve — the lawful response there is to report the gate as unavailable and return, never to build one."
    look="to see the reader's own reason"
fi
printf 'turn-end refused: %s.\n' "$finding" >&2
printf '%s\n' "$ways" >&2
printf 'Run `bash %s %s` %s.\n' "$LIVENESS_CMD" "$RUN_DIR" "$look" >&2
exit 2
