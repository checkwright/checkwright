#!/usr/bin/env bash
# spec: doctrine-kit/SPEC.md §stage-rules — derive the craft-rule pointers routed to a stage from DOCTRINE.md's *Stages:* trailers; an unknown stage yields empty output (graceful by design)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/doctrine.sh
source "$KIT/lib/doctrine.sh"

STAGE="${1:-}"
DOCTRINE_FILE="${2:-$DOCTRINE_KIT_DOCTRINE_FILE}"
[[ -n "$STAGE" ]] || { echo "usage: stage-rules.sh <stage> [doctrine-file]" >&2; exit 2; }  # exit 2: usage error
[[ -f "$DOCTRINE_FILE" ]] \
    || { echo "stage-rules: doctrine file not found: $DOCTRINE_FILE" >&2; exit 2; }  # exit 2: fail-closed

# spec: doctrine-kit/SPEC.md §stage-rules — the craft-section heading is kit mechanism (the kit ships DOCTRINE.md), never config
CRAFT_SECTION="## Engineering-craft rules"

awk -v section="$CRAFT_SECTION" -v stage="$STAGE" -v path="$DOCTRINE_FILE" '
function hlevel(line,   n) {
    if (line !~ /^#+[[:space:]]/) return 0
    n = 0
    while (substr(line, n + 1, 1) == "#") n++
    return n
}
!insec {
    if (hlevel($0) > 0 && substr($0, 1, length(section)) == section) {
        insec = 1; start_lvl = hlevel($0)
    }
    next
}
insec && hlevel($0) > 0 && hlevel($0) <= start_lvl { insec = 0; next }
insec {
    if ($0 ~ /^[0-9]+\.[[:space:]]+\*\*/) {
        num = $0; sub(/\..*/, "", num)
        name = $0
        sub(/^[0-9]+\.[[:space:]]+\*\*/, "", name)
        sub(/\*\*.*/, "", name)
        sub(/\.$/, "", name)
        cur_num = num; cur_name = name
    } else if ($0 ~ /^[[:space:]]*\*Stages:\*/) {
        val = $0
        sub(/^[[:space:]]*\*Stages:\*[[:space:]]*/, "", val)
        n = split(val, toks, /,[[:space:]]*/)
        for (i = 1; i <= n; i++) {
            gsub(/[[:space:]]/, "", toks[i])
            if (toks[i] == stage) { print "  • " cur_num ". " cur_name " — " path; break }
        }
    }
    next
}
' "$DOCTRINE_FILE"
