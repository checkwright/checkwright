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
)

for c in "${cases[@]}"; do
    printf '%s | %s -> %s\n' "${c%%|*}" "${c#*|}" "$(tool_floor_check "${c%%|*}" "${c#*|}")"
done
