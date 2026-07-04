#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §bin/queue-index.sh — compact queue surface for task selection (a tool, not a gate; no # graph: manifest)
#
# usage: queue-index.sh [--collapse-deferred] [--extent <slug>] [queue-file]
#   (default)             iteration header + active entries (• ready / ✗ blocked)
#                         + the deferred entries, one title per line.
#   --collapse-deferred   replaces the deferred listing with a per-### tally.
#   --extent <slug>       prints "<start> <end>" — the inclusive line range of
#                         that entry's whole subtree (incl. the trailing blank).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

mode=index; collapse=0; slug=""; file=""
while (($#)); do
    case "$1" in
        --collapse-deferred) collapse=1; shift ;;
        --extent) mode=extent; slug="${2:-}"; shift 2 || true ;;
        -h|--help) sed -n '3,9p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "queue-index: unknown option: $1" >&2; exit 2 ;;
        *)  file="$1"; shift ;;
    esac
done
FILE="${file:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "queue-index: file not found: $FILE" >&2; exit 2; }

if [[ "$mode" == extent ]]; then
    [[ -n "$slug" ]] || { echo "queue-index: --extent needs a <slug>" >&2; exit 2; }
    range="$(awk -v slug="$slug" '
        !found && $0 ~ /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/ {
            match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
            if (substr($0, RSTART + 2, RLENGTH - 4) == slug) {
                found = 1; start = NR; ind = match($0, /[^[:space:]]/) - 1; next
            }
        }
        found {
            if ($0 ~ /^#/ || $0 ~ /^---[[:space:]]*$/) { print start, NR - 1; done = 1; exit }
            if ($0 ~ /^[[:space:]]*-[[:space:]]/ && (match($0, /[^[:space:]]/) - 1) <= ind) {
                print start, NR - 1; done = 1; exit
            }
        }
        END { if (found && !done) print start, NR }
    ' "$FILE")"
    [[ -n "$range" ]] || { echo "queue-index: slug not found: $slug" >&2; exit 1; }
    echo "$range"
    exit 0
fi

hdr="$(grep -m1 '^## Iteration:' "$FILE" || true)"
[[ -n "$hdr" ]] && { echo "$hdr"; echo ""; }

awk -v activere="$QUEUE_ACTIVE_RE" -v deferredre="$QUEUE_DEFERRED_RE" \
    -v sectre="$QUEUE_SECTION_RE" -v collapse="$collapse" '
    function title(line,   t) {
        t = line
        sub(/^[[:space:]]*-[[:space:]]+/, "", t)
        sub(/^\*\*[a-z0-9][a-z0-9-]*\*\*[[:space:]]*(—[[:space:]]*)?/, "", t)
        gsub(/\[[^]]*\]/, "", t)
        sub(/[[:space:]]+$/, "", t)
        if (length(t) > 64) t = substr(t, 1, 63) "…"
        return t
    }
    function blockers(line,   s, r, b) {
        s = line; r = ""
        while (match(s, /\[blocked-by:[[:space:]]*[a-z0-9][a-z0-9-]*/)) {
            b = substr(s, RSTART, RLENGTH); sub(/\[blocked-by:[[:space:]]*/, "", b)
            r = r (r == "" ? "" : ", ") b
            s = substr(s, RSTART + RLENGTH)
        }
        return r
    }
    $0 ~ activere   { sec = "active";   next }
    $0 ~ deferredre { sec = "deferred"; next }
    $0 ~ sectre     { sec = "other";    next }

    sec == "active" && /^-[[:space:]]/ && match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/) {
        sl = substr($0, RSTART + 2, RLENGTH - 4)
        bl = blockers($0)
        na++; amark[na] = (bl == "" ? "•" : "✗"); aslug[na] = sl
        atitle[na] = title($0) (bl == "" ? "" : "   [blocked-by: " bl "]")
        next
    }
    sec == "deferred" && /^###[[:space:]]/ {
        cur = $0; sub(/^###[[:space:]]+/, "", cur); sub(/[[:space:]]+$/, "", cur); next
    }
    sec == "deferred" && /^-[[:space:]]/ && match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/) {
        sl = substr($0, RSTART + 2, RLENGTH - 4)
        key = (cur == "" ? "(top)" : cur)
        if (!(key in seen)) { seen[key] = 1; dord[++nd] = key }
        cnt[key]++
        dn++; dsub[dn] = key; dtitle[dn] = sl " — " title($0)
        next
    }

    END {
        print "Active (pick the first •):"
        if (na == 0) print "  (none — active queue empty)"
        for (i = 1; i <= na; i++) printf "  %s %s — %s\n", amark[i], aslug[i], atitle[i]
        print ""
        if (collapse == 1) {
            print "Deferred (tally):"
            if (nd == 0) print "  (none)"
            for (i = 1; i <= nd; i++) printf "  %s: %d\n", dord[i], cnt[dord[i]]
        } else {
            print "Deferred:"
            if (dn == 0) print "  (none)"
            for (i = 1; i <= dn; i++) printf "  %s\n", dtitle[i]
        }
    }
' "$FILE"
