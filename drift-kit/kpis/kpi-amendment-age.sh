#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-amendment-age: age of the oldest amendment on disk
set -uo pipefail

mapfile -t amends < <(git ls-files 2>/dev/null | grep -E '(^|/)SPEC-[^/]*\.md$' | grep -vE '/(gate-tests|templates)/')

if [[ ${#amends[@]} -eq 0 ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lead\tamendment age\tn/a (no amendment on disk)\n'
    exit 0
fi

now="$(date +%s)"
oldest_ts="$now"; oldest_file=""
for f in "${amends[@]}"; do
    ts="$(git log --diff-filter=A --follow --format=%at -- "$f" 2>/dev/null | tail -1)"
    [[ "$ts" =~ ^[0-9]+$ ]] || continue
    if [[ "$ts" -lt "$oldest_ts" ]]; then oldest_ts="$ts"; oldest_file="$f"; fi
done

if [[ -z "$oldest_file" ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lead\tamendment age\tn/a (no add-date resolvable)\n'
    exit 0
fi

days=$(( (now - oldest_ts) / 86400 ))
if [[ "${1:-}" == "--trend" ]]; then
    printf 'amend %dd\n' "$days"
    exit 0
fi
printf 'lead\tamendment age\toldest %dd (%s)\n' "$days" "${oldest_file##*/}"
exit 0
