#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — the floor predicate's case table: one line per (roster element, probed banner) pair, printed for golden comparison so the closed verdict set and its fail-closed arm are asserted, not assumed
# usage: toolfloor-cases.sh   (prints `<element> | <banner> -> <verdict>` for every case)
set -uo pipefail

LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)"
# shellcheck source=../lib/toolfloor.sh
source "$LIB/toolfloor.sh"

cases=(
    'awk|GNU Awk 5.3.1, API 4.0'
    'awk:|GNU Awk 5.3.1, API 4.0'
    'awk::|mawk 1.3.4 20240905'
    'bash:4.0|'
    'bash:4.0|GNU bash, version 5.2.37(1)-release (x86_64-pc-linux-gnu)'
    'bash:4.0|GNU bash, version 4.0.0(1)-release'
    'bash:4.0|GNU bash, version 3.2.57(1)-release (x86_64-apple-darwin20)'
    'awk::GNU|GNU Awk 5.3.1, API 4.0'
    'awk::GNU|mawk 1.3.4 20240905'
    'sort::coreutils|sort (GNU coreutils) 9.5'
    'sort::coreutils|present (/usr/bin/sort)'
    'bash:4.0|GNU bash, no version here'
    'bash:4.0:GNU|GNU bash, version 5.2.37(1)-release'
    'bash:4.0:GNU|bosh, version 3.1'
    'cargo:1.71::contributor|cargo 1.86.0 (adbf5df3f 2026-01-01)'
    'cargo:1.71::contributor|cargo 1.40.0'
)

for c in "${cases[@]}"; do
    printf '%s | %s -> %s\n' "${c%%|*}" "${c#*|}" "$(tool_floor_check "${c%%|*}" "${c#*|}")"
done

# spec: context-kit/SPEC.md §Testing — the audience axis is pinned in its own table because no verdict reads it: these cases assert the fourth field's present, empty and omitted forms and the consumer-side predicate over each, the emptiness rule being the part a reader is likeliest to get wrong
audience_cases=(
    'cargo:1.71::contributor'
    'cargo:::contributor'
    'awk::GNU:contributor'
    'cargo:1.71::'
    'cargo:1.71'
    'cargo'
    'awk::GNU'
)

for c in "${audience_cases[@]}"; do
    tool_floor_parse "$c"
    aud="$TOOL_FLOOR_AUDIENCE"
    if tool_floor_consumer_side "$c"; then side="consumer-side"; else side="contributor-only"; fi
    printf '%s -> audience=%s %s\n' "$c" "${aud:-<empty>}" "$side"
done
