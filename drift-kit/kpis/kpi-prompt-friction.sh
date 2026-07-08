#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-prompt-friction: distinct/total prompting calls via guard-kit
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

scanner=""
while IFS= read -r k; do
    [[ -n "$k" && -f "$k/bin/scan-prompts.sh" ]] && { scanner="$k/bin/scan-prompts.sh"; break; }
done < <(kit_roots)

na() { [[ "${1:-}" == "--trend" ]] && exit 0; printf 'lead\tprompt friction\tn/a (%s)\n' "$2"; exit 0; }

[[ -n "$scanner" ]] || na "${1:-}" "guard-kit absent"

count="$(bash "$scanner" --count 2>/dev/null)" || na "${1:-}" "scanner failed"
[[ "$count" =~ ^[0-9]+/[0-9]+$ ]] || na "${1:-}" "unreadable count"

distinct="${count%%/*}"; total="${count##*/}"
if [[ "$total" -eq 0 ]]; then na "${1:-}" "no friction logged"; fi

if [[ "${1:-}" == "--trend" ]]; then
    printf 'prompt %s\n' "$count"
    exit 0
fi
printf 'lead\tprompt friction\t%s distinct patterns / prompting calls\n' "$count"
exit 0
