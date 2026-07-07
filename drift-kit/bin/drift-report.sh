#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §The report skeleton — the collator; owns the frame, every measurement lives in a plugin
#
#   drift-report.sh            full report: lead/lag sections under the honesty labels, one row per KPI
#   drift-report.sh --trend    one compact line (fragments joined with ·) for the session-context hook
#
# Advisory by construction: exit is always 0, the report never joins gates.list.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 0

_ds_gatelib="$KIT/../gate-sdk/lib/gate.sh"
if [[ -f "$_ds_gatelib" ]]; then
    # shellcheck source=../../gate-sdk/lib/gate.sh
    source "$_ds_gatelib"
fi
unset _ds_gatelib

# spec: drift-kit/SPEC.md §Layout and configuration
_ds_cfg="${DRIFT_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/drift-config.sh}"
if [[ -f "$_ds_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ds_cfg"
fi
unset _ds_cfg

: "${DRIFT_KIT_KPIS_FILE:=${GATE_SDK_GATES_DIR:-scripts}/kpis.list}"
: "${DRIFT_KIT_QUEUE_FILE:=${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}}"
: "${DRIFT_KIT_KNOWLEDGE_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log}"
: "${DRIFT_KIT_TIMINGS_FILE:=${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt}"
: "${DRIFT_KIT_TMP_DIR:=${GATE_SDK_TMP_DIR:-.tmp}}"
: "${DRIFT_KIT_DONE_SECTION:=Done}"
: "${DRIFT_KIT_DEFERRED_SECTION:=Deferred}"
declare -p DRIFT_KIT_KPI_DIRS >/dev/null 2>&1 || DRIFT_KIT_KPI_DIRS=("${GATE_SDK_GATES_DIR:-scripts}")

export DRIFT_KIT_KPIS_FILE DRIFT_KIT_QUEUE_FILE DRIFT_KIT_KNOWLEDGE_LOG \
    DRIFT_KIT_TIMINGS_FILE DRIFT_KIT_TMP_DIR DRIFT_KIT_DONE_SECTION DRIFT_KIT_DEFERRED_SECTION

if declare -f gate_kit_roots >/dev/null 2>&1; then
    DRIFT_KIT_KIT_ROOTS="$(gate_kit_roots)"
else
    DRIFT_KIT_KIT_ROOTS="$KIT/.."
fi
export DRIFT_KIT_KIT_ROOTS

mkdir -p "$DRIFT_KIT_TMP_DIR" 2>/dev/null || true

kpi_dirs() {
    local d k
    for d in "${DRIFT_KIT_KPI_DIRS[@]}"; do printf '%s\n' "$d"; done
    while IFS= read -r k; do
        [[ -n "$k" && -d "$k/kpis" ]] && printf '%s/kpis\n' "$k"
    done <<<"$DRIFT_KIT_KIT_ROOTS"
    [[ -d "$KIT/kpis" ]] && printf '%s/kpis\n' "$KIT"
}

resolve_kpi() {
    local name="$1" d
    while IFS= read -r d; do
        [[ -f "$d/$name.sh" ]] && { printf '%s\n' "$d/$name.sh"; return 0; }
    done < <(kpi_dirs)
    return 1
}

registry_members() {
    [[ -f "$DRIFT_KIT_KPIS_FILE" ]] || return 0
    grep -Ev '^[[:space:]]*(#|$)' "$DRIFT_KIT_KPIS_FILE" || true
}

iteration_start() {
    local state="${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt" iter
    [[ -f "$DRIFT_KIT_QUEUE_FILE" ]] || return 0
    iter="$(awk '/^## Iteration:/ { for (i=3;i<=NF;i++){ if ($i=="[stage:") break; printf "%s%s",(i>3?" ":""),$i } exit }' \
        "$DRIFT_KIT_QUEUE_FILE" 2>/dev/null)"
    [[ -n "$iter" ]] || return 0
    git log --format='%h' -S"$iter scope " -- "$state" 2>/dev/null | tail -1
}

if [[ "${1:-}" == "--trend" ]]; then
    frags=()
    while IFS= read -r name; do
        [[ -n "$name" ]] || continue
        path="$(resolve_kpi "$name")" || continue
        frag="$("$path" --trend 2>/dev/null)" || true
        frag="${frag%%$'\n'*}"
        [[ -n "$frag" ]] && frags+=("$frag")
    done < <(registry_members)
    [[ ${#frags[@]} -eq 0 ]] && exit 0
    line="drift:"
    for f in "${frags[@]}"; do line="$line $f ·"; done
    printf '%s\n' "${line% ·}"
    exit 0
fi

lead_rows=()
lag_rows=()
while IFS= read -r name; do
    [[ -n "$name" ]] || continue
    if ! path="$(resolve_kpi "$name")"; then
        lead_rows+=("$name"$'\t'"n/a (unresolved — not in any KPI dir)")
        continue
    fi
    out="$("$path" 2>/dev/null)"; rc=$?
    if [[ "$rc" -ne 0 || -z "$out" ]]; then
        lead_rows+=("$name"$'\t'"n/a (plugin failed)")
        continue
    fi
    while IFS=$'\t' read -r section label value; do
        [[ -z "$section" ]] && continue
        case "$section" in
            lag) lag_rows+=("$label"$'\t'"$value") ;;
            *)   lead_rows+=("$label"$'\t'"$value") ;;
        esac
    done <<<"$out"
done < <(registry_members)

print_section() {
    local -n rows="$1"
    local r label value
    for r in "${rows[@]}"; do
        IFS=$'\t' read -r label value <<<"$r"
        printf '  %-22s  %s\n' "$label" "$value"
    done
}

start="$(iteration_start)"
header="=== Drift KPIs (advisory — trend, not level) ==="
[[ -n "$start" ]] && header="$header  [iteration start $start]"
printf '%s\n\n' "$header"

printf -- '--- Lead (weighted high — act before drift compounds) ---\n'
if [[ ${#lead_rows[@]} -gt 0 ]]; then print_section lead_rows; else echo "  (none registered)"; fi
printf '\n'
printf -- '--- Lag (weighted low — undercounts by construction) ---\n'
if [[ ${#lag_rows[@]} -gt 0 ]]; then print_section lag_rows; else echo "  (none registered)"; fi
printf '\n'
echo "Read trend across sessions; lag KPIs lower-bound only."
exit 0
