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
import sys, re
pat = re.compile(
    r"^(\d+):[ \t]*pub(?:\([^)]*\))?[ \t]+(?:async[ \t]+)?"
    r"(fn|struct|enum|trait|type|const|static|mod)[ \t]+([A-Za-z_]\w*)"
)
items = []
for line in sys.stdin:
    m = pat.match(line)
    if m:
        lineno, kind, name = m.group(1), m.group(2), m.group(3)
        items.append((kind, name, int(lineno)))
for kind, name, lineno in sorted(items, key=lambda x: (x[0], x[1])):
    print(f"  {kind:<8} {name} :{lineno}")
'

FOUND=0

while IFS= read -r -d '' file; do
    items=$(grep -n -E \
        '^\s*(pub|pub\([^)]*\))\s+(async\s+)?(fn|struct|enum|trait|type|const|static|mod)\s+[A-Za-z_]' \
        "$file" 2>/dev/null | python3 -c "$EXTRACT" || true)

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
