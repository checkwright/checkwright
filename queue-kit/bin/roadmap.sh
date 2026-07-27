#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §bin/roadmap.sh — the public roadmap projected off the queue's [roadmap:] tags (a tool, not a gate; no # graph: manifest)
#
# usage: roadmap.sh [--emit|--write] [queue-file]
#   --emit (default): print the generated block, the surface check-roadmap-fresh byte-compares.
#   --write: splice that block between QUEUE_KIT_ROADMAP_FILE's markers, touching nothing outside them.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/inject.sh
source "$SDK/lib/inject.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

mode="emit"; file=""
while (($#)); do
    case "$1" in
        --emit)  mode="emit"; shift ;;
        --write) mode="write"; shift ;;
        -h|--help) sed -n '3,6p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "roadmap: unknown option: $1" >&2; exit 2 ;;
        *)  file="$1"; shift ;;
    esac
done

FILE="${file:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "roadmap: queue file not found: $FILE" >&2; exit 2; }
[[ ${#QUEUE_KIT_HORIZONS[@]} -gt 0 ]] \
    || { echo "roadmap: QUEUE_KIT_HORIZONS is empty — no roadmap vocabulary is configured" >&2; exit 2; }

entries="$(queue_roadmap_entries "$FILE")"; st=$?
[[ "$st" -eq 0 ]] || { echo "roadmap: the queue parse failed (exit $st)" >&2; exit 2; }

# spec: queue-kit/SPEC.md §bin/roadmap.sh — every configured horizon gets its heading whether or not the queue fills it: an empty horizon is information, and a section that vanishes when it empties reads as a page that forgot it
emit_body() {
    local first=1 h n ntags fieldv slug summary track
    for h in "${QUEUE_KIT_HORIZONS[@]}"; do
        [[ "$first" -eq 1 ]] || printf '\n'
        first=0
        printf '### %s\n\n' "$h"
        n=0
        while IFS=$'\t' read -r ntags fieldv slug summary; do
            [[ -n "$slug" ]] || continue
            [[ "$ntags" == 1 ]] || continue
            [[ "$fieldv" == "$h/"* ]] || continue
            track="${fieldv#*/}"
            [[ -n "$track" && "$track" != */* ]] || continue
            printf -- '- **`%s`** *(%s)* — %s\n' "$slug" "$track" "$summary"
            n=$((n + 1))
        done <<< "$entries"
        [[ "$n" -gt 0 ]] || printf '%s\n' "_Nothing is queued under this horizon._"
    done
}

if [[ "$mode" == emit ]]; then
    emit_body
    exit 0
fi

[[ -n "$QUEUE_KIT_ROADMAP_FILE" ]] \
    || { echo "roadmap: --write needs QUEUE_KIT_ROADMAP_FILE; it is empty (no projection page configured)" >&2; exit 2; }
[[ -f "$QUEUE_KIT_ROADMAP_FILE" ]] \
    || { echo "roadmap: projection page not found: $QUEUE_KIT_ROADMAP_FILE" >&2; exit 2; }

BEGIN="<!-- ${QUEUE_KIT_ROADMAP_MARKER}:begin -->"
END="<!-- ${QUEUE_KIT_ROADMAP_MARKER}:end -->"
action="$(emit_body | inject_marker_block "$QUEUE_KIT_ROADMAP_FILE" "$BEGIN" "$END")" \
    || { echo "roadmap: failed to write the roadmap block into $QUEUE_KIT_ROADMAP_FILE" >&2; exit 2; }
echo "roadmap: $action the $QUEUE_KIT_ROADMAP_MARKER block in $QUEUE_KIT_ROADMAP_FILE"
