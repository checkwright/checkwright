#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §bin/queue-counts.sh — per-task-section entry counts (a tool, not a gate; no # graph: manifest)
#
# usage: queue-counts.sh [queue-file]
#   emits one "<section-name><TAB><count>" line per task section, in configured
#   order; no flags, no modes, one output grammar
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

file=""
while (($#)); do
    case "$1" in
        -h|--help) grep -m5 '^# ' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "queue-counts: unknown option: $1" >&2; exit 2 ;;
        *)  file="$1"; shift ;;
    esac
done
FILE="${file:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "queue-counts: file not found: $FILE" >&2; exit 2; }

# spec: queue-kit/SPEC.md §bin/queue-counts.sh — the counted unit is the top-level entry bullet, the same one the queue-index arm lists, so two readers cannot disagree about the size of one queue
awk -v sects="$(printf '%s\n' "${QUEUE_TASK_SECTIONS[@]}")" -v sectre="$QUEUE_SECTION_RE" '
    BEGIN { n = split(sects, S, "\n") }
    $0 ~ sectre {
        h = $0; sub(/^##[[:space:]]+/, "", h); sub(/[[:space:]]+$/, "", h)
        cur = ""
        for (i = 1; i <= n; i++) if (S[i] == h) cur = h
        next
    }
    cur != "" && /^-[[:space:]]/ && $0 ~ /\*\*[a-z0-9][a-z0-9-]*\*\*/ { c[cur]++ }
    END { for (i = 1; i <= n; i++) printf "%s\t%d\n", S[i], c[S[i]] + 0 }
' "$FILE"
