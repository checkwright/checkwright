#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Index-first reading — print one Markdown section by heading
# usage: md-section.sh <file> <heading>

set -uo pipefail

FILE="${1:-}"
QUERY="${2:-}"

if [[ -z "$FILE" || -z "$QUERY" ]]; then
    echo "usage: md-section.sh <file.md> \"<heading>\"" >&2
    exit 2
fi
if [[ ! -f "$FILE" ]]; then
    echo "md-section: file not found: $FILE" >&2
    exit 2
fi

needle="$(printf '%s' "$QUERY" | sed 's/^[[:space:]]*§*[[:space:]]*//; s/[[:space:]]*$//' | tr '[:upper:]' '[:lower:]')"

awk -v needle="$needle" '
    function heading_level(line,    n) {
        if (line !~ /^#+[[:space:]]/) return 0
        n = 0
        while (substr(line, n + 1, 1) == "#") n++
        return n
    }
    function heading_text(line,    t) {
        t = line
        sub(/^#+[[:space:]]+/, "", t)
        sub(/[[:space:]]+$/, "", t)
        return tolower(t)
    }
    {
        if ($0 ~ /^[[:space:]]*(```|~~~)/) in_fence = !in_fence
    }
    !found {
        if (!in_fence) {
            lvl = heading_level($0)
            if (lvl > 0 && heading_text($0) == needle) {
                found = 1
                start_lvl = lvl
                print
            }
        }
        next
    }
    {
        if (!in_fence) {
            lvl = heading_level($0)
            if (lvl > 0 && lvl <= start_lvl) exit
        }
        print
    }
    END { if (!found) exit 3 }
' "$FILE"
rc=$?

if [[ $rc -eq 3 ]]; then
    echo "md-section: no heading matched: $QUERY" >&2
    exit 1
fi
exit "$rc"
