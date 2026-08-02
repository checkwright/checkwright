#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §bin/queue-edges.sh — inbound citation aggregator (a tool, not a gate; no # graph: manifest)
#
# usage: queue-edges.sh [--inbound <slug>] [queue-file]
#   default: every live slug with at least one inbound edge, and its citing entries
#   --inbound <slug>: the inbound set for one slug
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

target=""; file=""
while (($#)); do
    case "$1" in
        --inbound) target="${2:-}"; shift 2 || true ;;
        -h|--help) sed -n '3,6p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "queue-edges: unknown option: $1" >&2; exit 2 ;;
        *)  file="$1"; shift ;;
    esac
done
FILE="${file:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "queue-edges: file not found: $FILE" >&2; exit 2; }

live="$(queue_live_slugs "$FILE")"
# spec: queue-kit/SPEC.md §bin/queue-edges.sh — a dead --inbound slug is a caller error, not an empty set: silence has to mean "no inbound edges" and nothing else
if [[ -n "$target" ]] && ! grep -qxF -- "$target" <<<"$live"; then
    echo "queue-edges: not a live slug: $target" >&2
    exit 1
fi

awk -v taskre="$QUEUE_TASK_RE" -v sectre="$QUEUE_SECTION_RE" \
    -v live="$live" -v want="$target" '
    BEGIN { n = split(live, a, "\n"); for (i = 1; i <= n; i++) if (a[i] != "") L[a[i]] = 1 }
    # spec: queue-kit/SPEC.md §The tag algebra — resolution against the live slug set, self-citation dropped; an unresolved token is silently not an edge, never a complaint
    function emit(tgt, line) {
        if (!(tgt in L) || tgt == cur || cur == "") return
        if (want != "" && tgt != want) return
        sub(/^[[:space:]]+/, "", line); sub(/[[:space:]]+$/, "", line)
        if (!(tgt in seen)) { seen[tgt] = 1; ord[++nt] = tgt }
        cnt[tgt]++
        rec[tgt, cnt[tgt]] = sprintf("  %-46s %s", cur, line)
    }
    function scanblocked(line,   s, b) {
        s = line
        while (match(s, /\[blocked-by:[[:space:]]*[a-z0-9][a-z0-9-]*/)) {
            b = substr(s, RSTART, RLENGTH); sub(/\[blocked-by:[[:space:]]*/, "", b)
            emit(b, line)
            s = substr(s, RSTART + RLENGTH)
        }
    }
    function scanbody(line,   s, t) {
        s = line
        while (match(s, /`[a-z0-9][a-z0-9-]*`/)) {
            t = substr(s, RSTART + 1, RLENGTH - 2)
            emit(t, line)
            s = substr(s, RSTART + RLENGTH)
        }
    }
    $0 ~ taskre { inq = 1; cur = ""; next }
    $0 ~ sectre { inq = 0; cur = ""; next }
    !inq { next }
    # spec: queue-kit/SPEC.md §bin/queue-edges.sh — a citation is attributed to the nearest preceding slug bullet, so a sub-task cites in its own name; the lead line yields its [blocked-by:] tag alone, never its prose
    /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/ {
        match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
        cur = substr($0, RSTART + 2, RLENGTH - 4)
        scanblocked($0)
        next
    }
    cur != "" { scanbody($0) }
    END {
        for (i = 1; i <= nt; i++) {
            t = ord[i]
            printf "%s (%d inbound)\n", t, cnt[t]
            for (j = 1; j <= cnt[t]; j++) print rec[t, j]
        }
    }
' "$FILE"
