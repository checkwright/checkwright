#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §bin/queue-index.sh — compact queue surface for task selection (a tool, not a gate; no # graph: manifest)
#
# usage: queue-index.sh [--collapse-deferred] [--extent <slug>] [--icebox-candidates] [queue-file]
#   default: header + active (• ready / ✗ blocked) + deferred titles + icebox tally;
#   --collapse-deferred: per-### tally; --extent <slug>: "<start> <end>"; --icebox-candidates: eviction worklist
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

mode=index; collapse=0; slug=""; file=""
while (($#)); do
    case "$1" in
        --collapse-deferred) collapse=1; shift ;;
        --extent) mode=extent; slug="${2:-}"; shift 2 || true ;;
        --icebox-candidates) mode=candidates; shift ;;
        -h|--help) sed -n '3,5p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
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

if [[ "$mode" == candidates ]]; then
    cutoff="$(date -d "$QUEUE_KIT_ICEBOX_AGE_DAYS days ago" +%F 2>/dev/null)" \
        || { echo "queue-index: cannot compute the age cutoff (date -d unavailable)" >&2; exit 2; }
    # spec: queue-kit/SPEC.md §bin/queue-index.sh — an advisory worklist, never a verdict: the age filter only bounds how much close must look at, and the cost-class opener is printed for the reading judgment rather than matched on
    awk -v defre="$QUEUE_DEFERRED_RE" -v sectre="$QUEUE_SECTION_RE" -v cutoff="$cutoff" '
        function opener(t) {
            sub(/^.*\*\*Cost while deferred:?\*\*:?[[:space:]]*/, "", t)
            sub(/[[:space:]]*$/, "", t)
            if (length(t) > 48) t = substr(t, 1, 47) "…"
            return (t == "" ? "(unstated)" : t)
        }
        # spec: queue-kit/SPEC.md §bin/queue-index.sh — the low cost class is matched on the opener as prose, which is an unacceptable heuristic in a gate and exactly the right ceiling in an advisory worklist; an uncosted entry is listed too, because an absent input appears rather than vanishing
        function lowclass(t) { return (t ~ /^(low|zero|bounded|cosmetic)/) }
        function flush(   d) {
            if (slug == "") return
            d = (surfaced != "" ? surfaced : filed)
            if ((d == "" || d < cutoff) && (cost == "" || lowclass(cost)))
                printf "%-46s %4dl  %-11s %s\n", slug, FNR - start, \
                       (d == "" ? "(undated)" : d), (cost == "" ? "(uncosted)" : cost)
            slug = ""; surfaced = ""; filed = ""; cost = ""
        }
        $0 ~ sectre { flush(); ind = ($0 ~ defre); next }
        !ind { next }
        /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/ {
            flush()
            match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
            slug = substr($0, RSTART + 2, RLENGTH - 4); start = FNR
        }
        slug == "" { next }
        match($0, /Surfaced [0-9]{4}-[0-9]{2}-[0-9]{2}/) {
            if (surfaced == "") surfaced = substr($0, RSTART + 9, 10)
        }
        match($0, /Filed [0-9]{4}-[0-9]{2}-[0-9]{2}/) {
            if (filed == "") filed = substr($0, RSTART + 6, 10)
        }
        /\*\*Cost while deferred/ { if (cost == "") cost = opener($0) }
        END { FNR++; flush() }
    ' "$FILE"
    exit 0
fi

hdr="$(grep -m1 '^## Iteration:' "$FILE" || true)"
[[ -n "$hdr" ]] && { echo "$hdr"; echo ""; }

awk -v activere="$QUEUE_ACTIVE_RE" -v deferredre="$QUEUE_DEFERRED_RE" \
    -v lessonsre="$QUEUE_LESSONS_RE" -v cap="$QUEUE_KIT_ATTEND_CAP" \
    -v sectre="$QUEUE_SECTION_RE" -v collapse="$collapse" \
    -v iceboxre="$QUEUE_ICEBOX_RE" -v iceboxname="$QUEUE_KIT_ICEBOX_SECTION" '
    function title(line,   t) {
        t = line
        sub(/^[[:space:]]*-[[:space:]]+/, "", t)
        sub(/^\*\*[a-z0-9][a-z0-9-]*\*\*[[:space:]]*(—[[:space:]]*)?/, "", t)
        gsub(/\[[^]]*\]/, "", t)
        sub(/[[:space:]]+$/, "", t)
        if (length(t) > 64) t = substr(t, 1, 63) "…"
        return t
    }
    function drainex(line,   d) {
        if (!match(line, /\[drain-exempt:[[:space:]]*[^]]+\]/)) return ""
        d = substr(line, RSTART, RLENGTH)
        sub(/\[drain-exempt:[[:space:]]*/, "", d); sub(/\][[:space:]]*$/, "", d)
        return d
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
    iceboxre != "" && $0 ~ iceboxre { sec = "icebox"; next }
    $0 ~ lessonsre  { sec = "lessons";  next }
    $0 ~ sectre     { sec = "other";    next }

    # spec: queue-kit/SPEC.md §bin/queue-index.sh — the icebox is a tally and never a listing: this surface is embedded in the always-loaded session brief, so listing the tier would re-import the very tokens the tier exists to remove
    sec == "icebox" && /^-[[:space:]]/ && match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/) { ni++; next }

    sec == "lessons" && /^-[[:space:]]/ && /\[attend\]/ {
        nl++
        if (nl <= cap) { line = $0; sub(/[[:space:]]+$/, "", line); att[nl] = line }
        next
    }

    sec == "active" && /^-[[:space:]]/ && match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/) {
        sl = substr($0, RSTART + 2, RLENGTH - 4)
        bl = blockers($0); de = drainex($0)
        na++; amark[na] = (bl == "" ? "•" : "✗"); aslug[na] = sl
        atitle[na] = title($0) (bl == "" ? "" : "   [blocked-by: " bl "]") \
                              (de == "" ? "" : "   [drain-exempt: " de "]")
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
        if (iceboxre != "") printf "%s: %d entries\n", iceboxname, ni + 0
        if (nl > 0) {
            print ""
            print "Attention (Lessons [attend], this iteration):"
            lim = (nl < cap ? nl : cap)
            for (i = 1; i <= lim; i++) printf "  %s\n", att[i]
            if (nl > cap) printf "  (+%d more [attend])\n", nl - cap
        }
    }
' "$FILE"
