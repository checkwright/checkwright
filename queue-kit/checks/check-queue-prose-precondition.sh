#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-queue-prose-precondition — no active entry states a forward precondition in prose without a blocked-by tag (selection trusts tags, not prose)
#
# usage: check-queue-prose-precondition.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE); the forward
#   trigger set is QUEUE_KIT_PRECONDITION_REGEX (matched against lowercased prose).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "check-queue-prose-precondition: file not found: $FILE" >&2; exit 2; }

out="$(awk -v activere="$QUEUE_ACTIVE_RE" -v sectre="$QUEUE_SECTION_RE" -v trig="$QUEUE_KIT_PRECONDITION_REGEX" '
    function flush() {
        if (startln == 0) return
        b = tolower(body)
        gsub(/\[[^]]*\]/, " ", b)                                                   # drop bracket tags/links from the prose
        gsub(/(once|when|after)[^.,;]*(landed|shipped|merged|resolved|completed|was [a-z]+ed)/, " ", b)  # strip past-tense narration
        if (b ~ trig && !hasblock)
            printf "%d\t%s\n", startln, lead
        startln = 0; body = ""; lead = ""; hasblock = 0
    }
    $0 ~ activere { inq = 1; flush(); next }
    $0 ~ sectre   { inq = 0; flush(); next }
    !inq { next }
    /^-[[:space:]]/ {                              # a new top-level active entry
        flush()
        startln = FNR; lead = $0; body = $0
        if ($0 ~ /\[blocked-by:/ || $0 ~ /\[precondition-ok:/) hasblock = 1
        next
    }
    {
        if (startln == 0) next
        body = body " " $0
        if ($0 ~ /\[blocked-by:/ || $0 ~ /\[precondition-ok:/) hasblock = 1
    }
    END { flush() }
' "$FILE")"; st=$?
fail_closed "$st" QUEUE-PROSE-PRECONDITION awk

if [[ -n "$out" ]]; then
    echo "check-queue-prose-precondition: active entry states a forward precondition in prose"
    echo "but carries no [blocked-by:] tag — selection trusts tags, so it is latently blocked"
    echo "yet mechanically pickable as 'first unblocked':"
    while IFS=$'\t' read -r ln lead; do
        [[ -n "$ln" ]] || continue
        echo "  $FILE:$ln: $lead"
    done <<< "$out"
    echo "  help: tag the real blocker '[blocked-by: <slug>]', or move the entry to the"
    echo "        Deferred section, or rephrase past-tense if the precondition is already"
    echo "        met, or opt out with '[precondition-ok: <reason>]' anywhere in the entry."
    exit 1
fi

echo "QUEUE-PROSE-PRECONDITION: clean (no untagged forward precondition in the active sections of $FILE)"
exit 0
