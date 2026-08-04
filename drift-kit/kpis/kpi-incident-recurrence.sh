#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-incident-recurrence: re-filings recorded by the queue's `recurrence:` declarations, and the highest-count slug (queue-kit/SPEC.md §The tag algebra owns the grammar; the second implementation is accepted residual — drift-kit cannot source queue-kit's lib without a cross-kit cycle)
set -uo pipefail

QUEUE="${DRIFT_KIT_QUEUE_FILE:-TASK-QUEUE.md}"

if [[ ! -f "$QUEUE" ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lag\tincident recurrence\tn/a (no queue file)\n'
    exit 0
fi

# spec: drift-kit/SPEC.md §Bundled KPIs — the declaration is self-slug-bearing and lives on a line of its own, so one anchored scan reads it with no entry-boundary parsing
read -r total top_slug top_count <<<"$(
    awk '
        $1 == "recurrence:" && NF >= 3 {
            n = 0
            for (i = 3; i <= NF; i++)
                if ($i ~ /^[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]$/) n++
            if (n == 0) next
            total += n
            if (n > top) { top = n; slug = $2 }
        }
        END { printf "%d %s %d\n", total + 0, (slug == "" ? "-" : slug), top + 0 }
    ' "$QUEUE" 2>/dev/null
)"

[[ "$total" =~ ^[0-9]+$ ]] || total=0

if [[ "$total" -eq 0 ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lag\tincident recurrence\tn/a (no recurrence declaration in the queue)\n'
    exit 0
fi

if [[ "${1:-}" == "--trend" ]]; then
    printf 'recur %d\n' "$total"
    exit 0
fi
printf 'lag\tincident recurrence\t%d re-filing(s) recorded; highest %s at %d (captured filings only — a lower bound)\n' \
    "$total" "$top_slug" "$top_count"
exit 0
