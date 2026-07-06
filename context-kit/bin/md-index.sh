#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Index-first reading — compact heading hierarchy with first sentence per section
#
# Extracted from the governance meta-layer of a private production platform;
# the repo-root derivation is de-hardcoded (git top-level, cwd fallback) so the
# tool runs from any consumer layout.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"

TARGETS=("${@}")
if [[ ${#TARGETS[@]} -eq 0 ]]; then
    TARGETS=("$REPO_ROOT")
fi

EXTRACT='
import sys, re

lines = sys.stdin.read().splitlines()
total = len(lines)
i = 0
results = []

while i < total:
    line = lines[i]
    m = re.match(r"^(#{1,6})\s+(.*)", line)
    if m:
        level = len(m.group(1))
        heading = m.group(2).strip()
        lineno = i + 1

        first = ""
        j = i + 1
        in_fence = False
        while j < total:
            l = lines[j].strip()
            if l.startswith("```") or l.startswith("~~~"):
                in_fence = not in_fence
                j += 1
                continue
            if in_fence:
                j += 1
                continue
            if re.match(r"^#{1,6}\s", l) or l.startswith("---"):
                break
            if l:
                text = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", l)
                text = re.sub(r"[*_`]", "", text)
                m2 = re.search(r"[.!?]", text)
                first = text[:m2.end()].strip() if m2 else text[:120].strip()
                break
            j += 1

        results.append((lineno, level, heading, first))
    i += 1

for lineno, level, heading, first in results:
    indent = "  " * (level - 1)
    prefix = "#" * level
    loc = f":{lineno}"
    if first:
        print(f"{indent}{prefix} {heading}{loc}  — {first}")
    else:
        print(f"{indent}{prefix} {heading}{loc}")
'

FOUND=0

while IFS= read -r -d '' file; do
    output=$(python3 -c "$EXTRACT" < "$file" 2>/dev/null || true)

    if [[ -n "$output" ]]; then
        rel="${file#"$REPO_ROOT"/}"
        lines=$(wc -l < "$file" | tr -d ' ')
        echo "${rel}  (${lines}L)"
        echo "$output"
        echo ""
        FOUND=1
    fi
done < <(find "${TARGETS[@]}" -name "*.md" \
    -not -path "*/target/*" \
    -not -path "*/.git/*" \
    -not -path "*/node_modules/*" \
    -print0 | sort -z)

if [[ "$FOUND" -eq 0 ]]; then
    echo "No Markdown files found in ${TARGETS[*]}"
fi
