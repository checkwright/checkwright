#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-always-loaded: standing per-session surface via context-kit
set -uo pipefail

kit_roots() {
    if [[ -n "${DRIFT_KIT_KIT_ROOTS:-}" ]]; then printf '%s\n' "$DRIFT_KIT_KIT_ROOTS"; return 0; fi
    local self gatelib
    self="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    gatelib="$self/../gate-sdk/lib/gate.sh"
    if [[ -f "$gatelib" ]]; then
        # shellcheck source=/dev/null  # sibling gate-sdk lib, resolved at runtime
        source "$gatelib"; gate_kit_roots
    else
        printf '%s\n' "$self/.."
    fi
}

meter=""
while IFS= read -r k; do
    [[ -n "$k" && -f "$k/bin/always-loaded.sh" ]] && { meter="$k/bin/always-loaded.sh"; break; }
done < <(kit_roots)

na() { [[ "${1:-}" == "--trend" ]] && exit 0; printf 'lead\talways-loaded\tn/a (%s)\n' "$2"; exit 0; }

[[ -n "$meter" ]] || na "${1:-}" "context-kit absent"

line="$(bash "$meter" 2>/dev/null)" || na "${1:-}" "meter failed"
value="${line#always-loaded: }"
[[ -n "$value" ]] || na "${1:-}" "empty meter output"

if [[ "${1:-}" == "--trend" ]]; then
    total="$(grep -oE '^[0-9]+l' <<<"$value" | head -1)"
    delta="$(grep -oE '[+-][0-9]+ since' <<<"$value" | grep -oE '[+-][0-9]+' | head -1)"
    [[ -n "$total" ]] || exit 0
    printf 'loaded %s%s\n' "$total" "${delta:+ $delta}"
    exit 0
fi
printf 'lead\talways-loaded\t%s\n' "$value"
exit 0
