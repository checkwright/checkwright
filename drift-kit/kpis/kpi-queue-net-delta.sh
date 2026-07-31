#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-queue-net-delta: the design-pending pool at the iteration-start commit against the worktree, two rows because one number would be gameable
set -uo pipefail

QUEUE="${DRIFT_KIT_QUEUE_FILE:-TASK-QUEUE.md}"
DEFERRED="${DRIFT_KIT_DEFERRED_SECTION:-Deferred}"
ICEBOX="${DRIFT_KIT_ICEBOX_SECTION:-}"
BASE="${DRIFT_KIT_ITERATION_START:-}"

na() {
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lead\tqueue net delta\t%s\n' "$2"
    printf 'lead\tqueue carry weight\t%s\n' "$2"
    exit 0
}

[[ -f "$QUEUE" ]] || na "${1:-}" "n/a (no queue file)"
[[ -n "$BASE" ]]  || na "${1:-}" "n/a (no iteration baseline)"

# spec: drift-kit/SPEC.md §Bundled KPIs — one walk per revision emitting "<section>\t<slug>" for a bold lead-in bullet plus a trailing line count, so the entry axis and the weight axis are read off the same parse
pool() {
    awk -v def="$DEFERRED" -v ice="$ICEBOX" '
        /^## / {
            sec = ""
            if ($0 ~ "^## "def"[[:space:]]*$")               sec = "deferred"
            else if (ice != "" && $0 ~ "^## "ice"[[:space:]]*$") sec = "icebox"
            next
        }
        sec == "" { next }
        { n++ }
        /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/ {
            match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
            printf "%s\t%s\n", sec, substr($0, RSTART + 2, RLENGTH - 4)
        }
        END { printf "#lines\t%d\n", n + 0 }
    '
}

base_pool="$(git show "$BASE:$QUEUE" 2>/dev/null | pool)" || true
[[ -n "$base_pool" ]] || na "${1:-}" "n/a (queue absent at $BASE)"
now_pool="$(pool < "$QUEUE")"

base_lines="${base_pool##*$'\n'#lines	}"
now_lines="${now_pool##*$'\n'#lines	}"

declare -A was=() now=()
while IFS=$'\t' read -r sec slug; do
    [[ "$sec" == "#lines" ]] && continue
    was["$slug"]="$sec"
done <<< "$base_pool"
while IFS=$'\t' read -r sec slug; do
    [[ "$sec" == "#lines" ]] && continue
    now["$slug"]="$sec"
done <<< "$now_pool"

# spec: drift-kit/SPEC.md §Bundled KPIs — an icebox move counts as neither filed nor drained: it is compression, not intake and not closure, so a session that mass-evicted to flatter the delta row moves the weight row instead and the gaming is visible
filed=0; drained=0
for s in "${!now[@]}"; do
    [[ "${now[$s]}" == "deferred" && -z "${was[$s]:-}" ]] && filed=$((filed + 1))
done
for s in "${!was[@]}"; do
    [[ -z "${now[$s]:-}" ]] && drained=$((drained + 1))
done

delta=$((filed - drained))
weight=$((now_lines - base_lines))

if [[ "${1:-}" == "--trend" ]]; then
    printf 'qnet %+d\n' "$delta"
    exit 0
fi
printf 'lead\tqueue net delta\t%+d (%d filed, %d drained since %s)\n' "$delta" "$filed" "$drained" "$BASE"
printf 'lead\tqueue carry weight\t%+d lines (%d now, %d at %s)\n' "$weight" "$now_lines" "$base_lines" "$BASE"
exit 0
