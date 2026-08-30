#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Index-first reading — compact heading hierarchy with first sentence per section
# usage: md-index.sh [paths…]   (default: whole tree, build dirs skipped)

set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/context.sh
source "$KIT/lib/context.sh"

REPO_ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null || pwd)"

# spec: context-kit/SPEC.md §Index-first reading — the walk's exclusion source is CONTEXT_KIT_PRUNE_DIRS, matched on the leaf basename: pruning a parent path also passes but loses coverage silently, because governed markdown under it is reached by explicit globs no prune touches
PRUNE=()
for _mi_d in "${CONTEXT_KIT_PRUNE_DIRS[@]}"; do PRUNE+=(-not -path "*/$_mi_d/*"); done
unset _mi_d

TARGETS=("${@}")
if [[ ${#TARGETS[@]} -eq 0 ]]; then
    TARGETS=("$REPO_ROOT")
fi

EXTRACT='
function strip_links(s,   out, span, txt) {
    out = ""
    while (match(s, /\[[^]]+\]\([^)]+\)/)) {
        span = substr(s, RSTART, RLENGTH)
        txt = span; sub(/^\[/, "", txt); sub(/\].*$/, "", txt)
        out = out substr(s, 1, RSTART - 1) txt
        s = substr(s, RSTART + RLENGTH)
    }
    return out s
}
{ lines[NR] = $0 }
END {
    for (i = 1; i <= NR; i++) {
        line = lines[i]
        if (! match(line, /^#{1,6}[ \t]+/)) continue
        level = 0
        while (substr(line, level + 1, 1) == "#") level++
        heading = substr(line, RLENGTH + 1)
        gsub(/^[ \t]+|[ \t]+$/, "", heading)
        first = ""
        infence = 0
        for (j = i + 1; j <= NR; j++) {
            l = lines[j]
            gsub(/^[ \t]+|[ \t]+$/, "", l)
            if (l ~ /^```/ || l ~ /^~~~/) { infence = ! infence; continue }
            if (infence) continue
            if (l ~ /^#{1,6}[ \t]/ || l ~ /^---/) break
            if (l != "") {
                text = strip_links(l)
                gsub(/[*_`]/, "", text)
                if (match(text, /[.!?]/)) first = substr(text, 1, RSTART)
                else first = substr(text, 1, 120)
                gsub(/^[ \t]+|[ \t]+$/, "", first)
                break
            }
        }
        indent = ""; for (k = 1; k < level; k++) indent = indent "  "
        prefix = ""; for (k = 1; k <= level; k++) prefix = prefix "#"
        if (first != "") print indent prefix " " heading ":" i "  — " first
        else print indent prefix " " heading ":" i
    }
}
'

FOUND=0

while IFS= read -r -d '' file; do
    output=$(awk "$EXTRACT" < "$file" 2>/dev/null || true)

    if [[ -n "$output" ]]; then
        rel="${file#"$REPO_ROOT"/}"
        lines=$(wc -l < "$file" | tr -d ' ')
        echo "${rel}  (${lines}L)"
        echo "$output"
        echo ""
        FOUND=1
    fi
done < <(find "${TARGETS[@]}" -name "*.md" "${PRUNE[@]}" -print0 | sort -z)

if [[ "$FOUND" -eq 0 ]]; then
    echo "No Markdown files found in ${TARGETS[*]}"
fi
