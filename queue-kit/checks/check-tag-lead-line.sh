#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-tag-lead-line — every governed tag (blocked-by/spec/needs-spec in the task sections, attend + configured lesson tags in Lessons) sits on its bullet's lead line, the only line the tag readers scan
#
# usage: check-tag-lead-line.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "check-tag-lead-line: file not found: $FILE" >&2; exit 2; }

lessontags="${QUEUE_KIT_LESSON_TAGS[*]+"${QUEUE_KIT_LESSON_TAGS[*]}"}"
out="$(awk -v taskre="$QUEUE_TASK_RE" -v lessonsre="$QUEUE_LESSONS_RE" \
    -v sectre="$QUEUE_SECTION_RE" -v lessontags="$lessontags" '
    BEGIN { nlt = split(lessontags, lt, " ") }
    function classes(line, arr,   i) {
        delete arr
        if (line ~ /\[blocked-by:/)  arr["blocked-by"] = 1
        if (line ~ /\[spec:/)        arr["spec"]       = 1
        if (line ~ /\[needs-spec\]/) arr["needs-spec"] = 1
        if (line ~ /\[attend\]/)     arr["attend"]     = 1
        for (i = 1; i <= nlt; i++)
            if (index(line, "[" lt[i] "]")) arr[lt[i]] = 1
    }
    $0 ~ taskre    { inscan = 1; leadfnr = 0; delete leadcls; next }
    $0 ~ lessonsre { inscan = 1; leadfnr = 0; delete leadcls; next }
    $0 ~ sectre    { inscan = 0; leadfnr = 0; delete leadcls; next }
    !inscan { next }
    /^[[:space:]]*```/ { fence = !fence; next }
    fence            { next }
    /^[[:space:]]*\|/ { next }                        # table row: exempt
    /^[[:space:]]*-[[:space:]]/ {                     # bullet lead line
        classes($0, leadcls); leadfnr = FNR; next
    }
    {
        if (leadfnr == 0) next                        # continuation with no owning bullet
        classes($0, cont)
        for (k in cont)
            if (!(k in leadcls))
                printf "%d\t%s\t%d\n", FNR, k, leadfnr
    }
' "$FILE")"; st=$?
fail_closed "$st" TAG-LEAD-LINE awk

if [[ -n "$out" ]]; then
    echo "check-tag-lead-line: tag(s) pushed off the bullet lead line (a tag reader"
    echo "scans only the lead; a tag on a continuation line silently stops counting):"
    while IFS=$'\t' read -r ln cls lead; do
        [[ -n "$ln" ]] || continue
        echo "  $FILE:$ln: [$cls] on a continuation line; lead line $lead carries no [$cls]"
    done <<< "$out"
    echo "  help: move the tag back onto the bullet's lead line (the '- ...' line). If a"
    echo "        reflow pushed it there, re-wrap so the lead line carries the tag."
    exit 1
fi

echo "TAG-LEAD-LINE: clean (every governed tag in the task + Lessons sections is on its bullet lead line in $FILE)"
exit 0
