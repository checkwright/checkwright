#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-deferred-age: age of the oldest defer date in the deferred section, the Surfaced mark where one exists and the Filed provenance date otherwise (queue-kit/SPEC.md §The queue format owns the definition; the second implementation is accepted residual — drift-kit cannot source queue-kit's lib without a cross-kit cycle)
set -uo pipefail

QUEUE="${DRIFT_KIT_QUEUE_FILE:-TASK-QUEUE.md}"
SECTION="${DRIFT_KIT_DEFERRED_SECTION:-Deferred}"

[[ -f "$QUEUE" ]] || { [[ "${1:-}" == "--trend" ]] || printf 'lead\tdeferred age\tn/a (no queue file)\n'; exit 0; }

# spec: drift-kit/SPEC.md §Bundled KPIs — the unknown-heading reset drops an icebox placed after the deferred section out of this input by construction, which is wanted: an evicted entry's age is no longer the thing this KPI trends
mapfile -t dates < <(
    awk -v sec="$SECTION" '
        $0 ~ "^## "sec"[[:space:]]*$" { inx=1; next }
        /^## / { inx=0 }
        inx { print }
    ' "$QUEUE" 2>/dev/null \
        | grep -oE '(Surfaced|Filed) [0-9]{4}-[0-9]{2}-[0-9]{2}' | awk '{print $2}' | sort -u
)

if [[ ${#dates[@]} -eq 0 ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lead\tdeferred age\tn/a (no defer date)\n'
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
    printf 'lead\tdeferred age\tn/a (no parseable defer date)\n'
    exit 0
fi

days=$(( (now - oldest_ts) / 86400 ))
if [[ "${1:-}" == "--trend" ]]; then
    printf 'defer %dd\n' "$days"
    exit 0
fi
printf 'lead\tdeferred age\toldest %dd (deferred %s)\n' "$days" "$oldest_date"
exit 0
