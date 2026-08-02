#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-queue-entry-budget — a deferred entry is a costed filing: bounded above so it is not an inlined amendment, bounded below so it is not a flag-and-skip; an icebox entry is its lead line and nothing else
#
# usage: check-queue-entry-budget.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "check-queue-entry-budget: file not found: $FILE" >&2; exit 2; }

out="$(awk -v defre="$QUEUE_DEFERRED_RE" -v icere="$QUEUE_ICEBOX_RE" \
    -v sectre="$QUEUE_SECTION_RE" -v cap="$QUEUE_KIT_ENTRY_LINE_CAP" '
    # spec: queue-kit/SPEC.md §check-queue-entry-budget — an extent runs from the lead line to the line before the next bullet at the same or shallower indent, the rule bin/queue-index.sh --extent already yields; a sub-task nests inside its parent and is measured as its own entry too
    # spec: queue-kit/SPEC.md §check-queue-entry-budget — assertion A measures the raw extent (trailing blank included), the range an eviction deletes; assertion B counts content lines, because a blank before the next heading is not a continuation line
    function emit(i,   n) {
        n = bound - o_start[i]
        if (o_sec[i] == "deferred") {
            if (n > cap) printf "size\t%d\t%s\t%d\n", o_start[i], o_slug[i], n
            if (o_ind[i] == 0 && !o_costed[i]) printf "cost\t%d\t%s\t0\n", o_start[i], o_slug[i]
        } else if (o_sec[i] == "icebox" && o_nb[i] > 1) {
            printf "shape\t%d\t%s\t%d\n", o_start[i], o_slug[i], o_nb[i]
        }
    }
    function close_to(ind,   i) {
        for (i = nopen; i >= 1; i--) {
            if (o_ind[i] < ind) break
            emit(i); nopen--
        }
    }
    function cost_seen(   i) { for (i = 1; i <= nopen; i++) o_costed[i] = 1 }

    /^#/ || /^---[[:space:]]*$/ { bound = FNR; close_to(0) }
    $0 ~ sectre {
        sec = "other"
        if ($0 ~ defre)                     sec = "deferred"
        else if (icere != "" && $0 ~ icere) sec = "icebox"
        next
    }
    sec != "deferred" && sec != "icebox" { next }
    /^[[:space:]]*-[[:space:]]/ {
        ind = match($0, /[^[:space:]]/) - 1
        bound = FNR; close_to(ind)
        if (!match($0, /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/)) {
            for (i = 1; i <= nopen; i++) o_nb[i]++      # prose-note bullet: a content line
            next
        }
        match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
        for (i = 1; i <= nopen; i++) o_nb[i]++          # a sub-task is content of the parent too
        nopen++
        o_slug[nopen] = substr($0, RSTART + 2, RLENGTH - 4)
        o_start[nopen] = FNR; o_ind[nopen] = ind; o_sec[nopen] = sec
        o_costed[nopen] = 0; o_nb[nopen] = 1
        if ($0 ~ /\*\*Cost while deferred/) cost_seen()
        next
    }
    nopen > 0 && NF > 0 { for (i = 1; i <= nopen; i++) o_nb[i]++ }
    nopen > 0 && /\*\*Cost while deferred/ { cost_seen() }
    END { bound = FNR + 1; close_to(0) }
' "$FILE")"; st=$?
fail_closed "$st" check-queue-entry-budget awk

if [[ -n "$out" ]]; then
    size=(); shape=(); cost=()
    while IFS=$'\t' read -r class ln slug n; do
        [[ -n "$class" ]] || continue
        case "$class" in
            size)  size+=("$FILE:$ln: $slug — $n lines (cap $QUEUE_KIT_ENTRY_LINE_CAP)") ;;
            shape) shape+=("$FILE:$ln: $slug — $n content lines; an icebox entry is exactly one") ;;
            cost)  cost+=("$FILE:$ln: $slug") ;;
        esac
    done <<< "$out"
    echo "check-queue-entry-budget: deferred-pool entry budget violation(s):"
    echo ""
    if (( ${#size[@]} )); then
        echo "over the per-entry line cap (a body that long is an amendment inlined where"
        echo "the amendment gates cannot see it):"
        printf '  %s\n' "${size[@]}"
    fi
    if (( ${#cost[@]} )); then
        echo "no 'Cost while deferred' field (a gap you defer is costed and filed, never"
        echo "flagged-and-skipped):"
        printf '  %s\n' "${cost[@]}"
    fi
    if (( ${#shape[@]} )); then
        echo "icebox entry carrying a body (the tier's whole purpose is minimum residency;"
        echo "membership in it is itself the cost declaration):"
        printf '  %s\n' "${shape[@]}"
    fi
    echo "  help: add the cost field, or evict the entry to the icebox as a one-line lead."
    echo "        Over the cap: compress by ANSWERING grounds, never by dropping them —"
    echo "        an unanswered ground is relocated to a linked entry, and that split is"
    echo "        authorization-gated, not self-served (queue-kit/SPEC.md"
    echo "        section check-queue-entry-budget)."
    exit 1
fi

echo "QUEUE-ENTRY-BUDGET: clean (every $QUEUE_KIT_DEFERRED_SECTION entry within $QUEUE_KIT_ENTRY_LINE_CAP lines and carrying a cost field in $FILE)"
exit 0
