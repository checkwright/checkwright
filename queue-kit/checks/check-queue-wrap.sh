#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-queue-wrap — no queue line exceeds the wrap budget (Unicode code points), so a runaway never reflows to column 0
#
# usage: check-queue-wrap.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE); the floor is
#   QUEUE_KIT_WRAP_BUDGET (default 100; the authoring target is ~80).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "check-queue-wrap: file not found: $FILE" >&2; exit 2; }

# spec: queue-kit/SPEC.md §check-queue-wrap — width is Unicode code points: LC_ALL=C
# makes awk bytewise, cplen() subtracts UTF-8 continuation bytes (0x80–0xBF) to recover it.
out="$(LC_ALL=C awk -v budget="$QUEUE_KIT_WRAP_BUDGET" '
    function cplen(s,   t, cont) {
        t = s
        cont = gsub(/[\200-\277]/, "", t)
        return length(s) - cont
    }
    /^[[:space:]]*```/ { fence = !fence; next }   # fenced-code delimiter: exempt, toggle
    fence            { next }                     # inside a fenced block: exempt
    /^[[:space:]]*\|/ { next }                    # table row: exempt
    {
        w = cplen($0)
        if (w <= budget) next
        maxtok = 0
        n = split($0, T, /[[:space:]]+/)
        for (i = 1; i <= n; i++) { tl = cplen(T[i]); if (tl > maxtok) maxtok = tl }
        if (maxtok > budget) next                 # one unbreakable token (URL/path): no wrap helps
        printf "%d\t%d\t%s\n", FNR, w, $0
    }
' "$FILE")"; st=$?
fail_closed "$st" QUEUE-WRAP awk

if [[ -n "$out" ]]; then
    echo "check-queue-wrap: line(s) over the $QUEUE_KIT_WRAP_BUDGET-column budget (a runaway"
    echo "that reflows to column 0 corrupts the '- ' lead the tools key on):"
    while IFS=$'\t' read -r ln w text; do
        [[ -n "$ln" ]] || continue
        echo "  $FILE:$ln: $w cols — $text"
    done <<< "$out"
    echo "  help: hard-wrap the line at ~80 columns. Exempt already: table rows,"
    echo "        fenced code, and a line over budget solely from one unbreakable token."
    exit 1
fi

echo "QUEUE-WRAP: clean (no line exceeds $QUEUE_KIT_WRAP_BUDGET columns in $FILE)"
exit 0
