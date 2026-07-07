#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-deferred-age: age of the oldest Surfaced mark in the deferred section
#
# Lead. Premise-rot pressure on design-pending work: the oldest `Surfaced
# <date>` mark (queue-kit's ungated convention) in the queue's deferred
# section, in days. Degrades to n/a when no mark is present.
set -uo pipefail

QUEUE="${DRIFT_KIT_QUEUE_FILE:-TASK-QUEUE.md}"
SECTION="${DRIFT_KIT_DEFERRED_SECTION:-Deferred}"

[[ -f "$QUEUE" ]] || { [[ "${1:-}" == "--trend" ]] || printf 'lead\tdeferred age\tn/a (no queue file)\n'; exit 0; }

mapfile -t dates < <(
    awk -v sec="$SECTION" '
        $0 ~ "^## "sec"[[:space:]]*$" { inx=1; next }
        /^## / { inx=0 }
        inx { print }
    ' "$QUEUE" 2>/dev/null | grep -oE 'Surfaced [0-9]{4}-[0-9]{2}-[0-9]{2}' | awk '{print $2}' | sort -u
)

if [[ ${#dates[@]} -eq 0 ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lead\tdeferred age\tn/a (no Surfaced mark)\n'
    exit 0
fi

now="$(date +%s)"
oldest_ts="$now"; oldest_date=""
for d in "${dates[@]}"; do
    ts="$(date -d "$d" +%s 2>/dev/null)" || continue
    [[ "$ts" =~ ^[0-9]+$ ]] || continue
    if [[ "$ts" -lt "$oldest_ts" ]]; then oldest_ts="$ts"; oldest_date="$d"; fi
done

if [[ -z "$oldest_date" ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lead\tdeferred age\tn/a (no parseable Surfaced date)\n'
    exit 0
fi

days=$(( (now - oldest_ts) / 86400 ))
if [[ "${1:-}" == "--trend" ]]; then
    printf 'defer %dd\n' "$days"
    exit 0
fi
printf 'lead\tdeferred age\toldest %dd (Surfaced %s)\n' "$days" "$oldest_date"
exit 0
