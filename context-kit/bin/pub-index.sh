#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Index-first reading — compact public API surface for Rust source files
# usage: pub-index.sh [paths…]

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"

TARGETS=("${@}")
if [[ ${#TARGETS[@]} -eq 0 ]]; then
    TARGETS=("$REPO_ROOT")
fi

EXTRACT='
{
    line = $0
    if (! match(line, /^[0-9]+:/)) next
    lineno = substr(line, 1, RLENGTH - 1)
    rest = substr(line, RLENGTH + 1)
    sub(/^[ \t]*/, "", rest)
    if (rest !~ /^pub([ \t]|\()/) next
    sub(/^pub(\([^)]*\))?[ \t]+/, "", rest)
    sub(/^async[ \t]+/, "", rest)
    if (! match(rest, /^(fn|struct|enum|trait|type|const|static|mod)[ \t]+/)) next
    kw = substr(rest, 1, RLENGTH); sub(/[ \t]+$/, "", kw)
    rest = substr(rest, RLENGTH + 1)
    if (! match(rest, /^[A-Za-z_][A-Za-z0-9_]*/)) next
    name = substr(rest, 1, RLENGTH)
    print kw " " name " " lineno
}
'

FORMAT='{ printf "  %-8s %s :%s\n", $1, $2, $3 }'

FOUND=0

while IFS= read -r -d '' file; do
    items=$(grep -n -E \
        '^\s*(pub|pub\([^)]*\))\s+(async\s+)?(fn|struct|enum|trait|type|const|static|mod)\s+[A-Za-z_]' \
        "$file" 2>/dev/null | awk "$EXTRACT" | LC_ALL=C sort -k1,1 -k2,2 | awk "$FORMAT" || true)

    if [[ -n "$items" ]]; then
        rel="${file#"$REPO_ROOT"/}"
        count=$(echo "$items" | wc -l | tr -d ' ')
        echo "${rel}  ($count)"
        echo "$items"
        echo ""
        FOUND=1
    fi
done < <(find "${TARGETS[@]}" -name "*.rs" \
    -not -path "*/target/*" \
    -not -path "*/.git/*" \
    -print0 | sort -z)

if [[ "$FOUND" -eq 0 ]]; then
    echo "No Rust files with public items found in ${TARGETS[*]}"
fi
