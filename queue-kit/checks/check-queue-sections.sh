#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-queue-sections — the queue carries each required ## section heading exactly once, the fail-closed floor under every section-scoped scanner
#
# usage: check-queue-sections.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "check-queue-sections: file not found: $FILE" >&2; exit 2; }

missing=(); dup=()
for sec in "${QUEUE_KIT_REQUIRED_SECTIONS[@]}"; do
    if [[ "$sec" == *: ]]; then re="^## ${sec}"; else re="^## ${sec}[[:space:]]*\$"; fi
    n="$(grep -cE -- "$re" "$FILE")"; gst=$?
    if [[ "$gst" -ge 2 ]]; then
        echo "check-queue-sections: grep failed on '$re' (exit $gst) — cannot verify" >&2
        exit 2
    fi
    if   [[ "$n" -eq 0 ]]; then missing+=("$sec")
    elif [[ "$n" -gt 1 ]]; then dup+=("$sec ($n occurrences)")
    fi
done

if [[ ${#missing[@]} -gt 0 || ${#dup[@]} -gt 0 ]]; then
    echo "check-queue-sections: required '##' section(s) not present exactly once in $FILE"
    echo "(every section-scoped scanner — amendment-queue, task-names, conservation, the"
    echo "session-context index — locates work by these headings and finds nothing when one drops):"
    for s in "${missing[@]}"; do echo "  missing:   ## $s"; done
    for s in "${dup[@]}";     do echo "  duplicate: ## $s"; done
    echo "  help: restore the heading (spelled exactly), or remove the duplicate. The"
    echo "        required set is QUEUE_KIT_REQUIRED_SECTIONS (queue-config.sh)."
    exit 1
fi

echo "QUEUE-SECTIONS: clean (${#QUEUE_KIT_REQUIRED_SECTIONS[@]} required section(s) each present once in $FILE)"
exit 0
