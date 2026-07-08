#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-task-split: feature↔debt split of the queue's Done slugs
set -uo pipefail

QUEUE="${DRIFT_KIT_QUEUE_FILE:-TASK-QUEUE.md}"
SECTION="${DRIFT_KIT_DONE_SECTION:-Done}"

[[ -f "$QUEUE" ]] || { [[ "${1:-}" == "--trend" ]] || printf 'lead\ttask split (feat/debt)\tn/a (no queue file)\n'; exit 0; }

mapfile -t slugs < <(
    awk -v sec="$SECTION" '
        $0 ~ "^## "sec"[[:space:]]*$" { inx=1; next }
        /^## / { inx=0 }
        inx && /^-[[:space:]]+[a-z0-9][a-z0-9-]*[[:space:]]*$/ { print $2 }
    ' "$QUEUE" 2>/dev/null | sort -u
)

total=${#slugs[@]}
if [[ "$total" -eq 0 ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lead\ttask split (feat/debt)\tn/a (nothing Done this iteration)\n'
    exit 0
fi

feat=0; debt=0
for s in "${slugs[@]}"; do
    subj="$(git log -1 --format=%s --grep="$s" 2>/dev/null)" || true
    case "$subj" in
        feat*)              feat=$((feat + 1)) ;;
        fix* | refactor*)   debt=$((debt + 1)) ;;
    esac
done

if [[ "${1:-}" == "--trend" ]]; then
    printf 'split %df/%dd\n' "$feat" "$debt"
    exit 0
fi

unclassified=$((total - feat - debt))
value="${feat}f / ${debt}d of ${total} done"
[[ "$unclassified" -gt 0 ]] && value="$value ($unclassified unclassified)"
printf 'lead\ttask split (feat/debt)\t%s\n' "$value"
exit 0
