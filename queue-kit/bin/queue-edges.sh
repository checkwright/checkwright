#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §bin/queue-edges.sh — inbound citation aggregator (a tool, not a gate; no # graph: manifest)
#
# usage: queue-edges.sh [--inbound <slug>] [queue-file]
#   default: live slugs with inbound edges in queue order, then retired targets
#   --inbound <slug>: the inbound set for one live or retired slug
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

target=""; file=""
while (($#)); do
    case "$1" in
        --inbound) target="${2:-}"; shift 2 || true ;;
        -h|--help) sed -n '4,6p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "queue-edges: unknown option: $1" >&2; exit 2 ;;
        *)  file="$1"; shift ;;
    esac
done
FILE="${file:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "queue-edges: file not found: $FILE" >&2; exit 2; }

live="$(queue_live_slugs "$FILE")"

# spec: queue-kit/SPEC.md §bin/queue-edges.sh — the retired set: every slug that ever held an entry lead line in this file's history, less the live set. Absent git, a file outside a work tree, or a file git has no history for all yield the empty set and today's output exactly, which is why the degradation is silent-safe rather than misleading
retired=""
if command -v git >/dev/null 2>&1; then
    _qe_dir="$(dirname -- "$FILE")"
    _qe_base="$(basename -- "$FILE")"
    if git -C "$_qe_dir" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        retired="$(git -C "$_qe_dir" log -p --format= -- "$_qe_base" 2>/dev/null \
            | awk -v leadre="$QUEUE_LEAD_RE" -v slugre="$QUEUE_SLUG_BOLD_RE" -v live="$live" '
            BEGIN { n = split(live, a, "\n"); for (i = 1; i <= n; i++) if (a[i] != "") L[a[i]] = 1 }
            # spec: queue-kit/SPEC.md §bin/queue-edges.sh — one substr strips the diff column from added, removed and context lines alike, so a lead line counts wherever the walk meets it; the diff headers survive the strip as text no lead-line grammar matches
            { s = substr($0, 2) }
            s ~ leadre {
                match(s, slugre); g = substr(s, RSTART + 2, RLENGTH - 4)
                if (!(g in L) && !(g in seen)) { seen[g] = 1; print g }
            }
        ')"
    fi
fi

# spec: queue-kit/SPEC.md §bin/queue-edges.sh — a slug that is neither live nor retired is a caller error, not an empty set: silence has to mean "no inbound edges" and nothing else, and widening the addressable domain left that meaning untouched
if [[ -n "$target" ]] \
    && ! grep -qxF -- "$target" <<<"$live" \
    && ! grep -qxF -- "$target" <<<"$retired"; then
    echo "queue-edges: not a live or retired slug: $target" >&2
    exit 1
fi

awk -v taskre="$QUEUE_TASK_RE" -v sectre="$QUEUE_SECTION_RE" \
    -v leadre="$QUEUE_LEAD_RE" -v slugre="$QUEUE_SLUG_BOLD_RE" \
    -v live="$live" -v retired="$retired" -v want="$target" '
    BEGIN {
        n = split(live, a, "\n");    for (i = 1; i <= n; i++) if (a[i] != "") L[a[i]] = 1
        m = split(retired, b, "\n"); for (i = 1; i <= m; i++) if (b[i] != "") R[b[i]] = 1
    }
    # spec: queue-kit/SPEC.md §The tag algebra — resolution against the live slug set and the retired one, self-citation dropped; a token in neither is silently not an edge, never a complaint
    function emit(tgt, line,   isret) {
        if (cur == "" || tgt == cur) return
        if (tgt in L)      isret = 0
        else if (tgt in R) isret = 1
        else return
        if (want != "" && tgt != want) return
        sub(/^[[:space:]]+/, "", line); sub(/[[:space:]]+$/, "", line)
        if (!(tgt in seen)) { seen[tgt] = 1; if (isret) rord[++nret] = tgt; else ord[++nt] = tgt }
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
    function block(tgt, suffix,   j) {
        printf "%s (%d inbound%s)\n", tgt, cnt[tgt], suffix
        for (j = 1; j <= cnt[tgt]; j++) print rec[tgt, j]
    }
    $0 ~ taskre { inq = 1; cur = ""; next }
    $0 ~ sectre { inq = 0; cur = ""; next }
    !inq { next }
    # spec: queue-kit/SPEC.md §bin/queue-edges.sh — a citation is attributed to the nearest preceding slug bullet, so a sub-task cites in its own name; the lead line yields its [blocked-by:] tag alone, never its prose
    $0 ~ leadre {
        match($0, slugre)
        cur = substr($0, RSTART + 2, RLENGTH - 4)
        scanblocked($0)
        next
    }
    cur != "" { scanbody($0) }
    END {
        for (i = 1; i <= nt; i++) block(ord[i], "")
        # spec: queue-kit/SPEC.md §bin/queue-edges.sh — retired targets sort alphabetically because a retired slug has no queue position to order by; the string coercion keeps an all-digit slug out of awk numeric comparison
        for (i = 2; i <= nret; i++) {
            k = rord[i]; j = i - 1
            while (j >= 1 && (rord[j] "") > (k "")) { rord[j + 1] = rord[j]; j-- }
            rord[j + 1] = k
        }
        for (i = 1; i <= nret; i++) block(rord[i], ", retired")
    }
' "$FILE"
