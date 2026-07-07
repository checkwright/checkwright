#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-gate-backlog: proposed-but-absent gates over the live gate count
#
# Lead. check-*/scan-* names named anywhere in the queue with no file in any
# gate-resolution dir (gates dir, each kit's checks/ + bin/). A name with a
# file on disk is built and drops out of the numerator.
set -uo pipefail

QUEUE="${DRIFT_KIT_QUEUE_FILE:-TASK-QUEUE.md}"
GATES_DIR="${GATE_SDK_GATES_DIR:-scripts}"

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

[[ -f "$QUEUE" ]] || { [[ "${1:-}" == "--trend" ]] || printf 'lead\tgate backlog\tn/a (no queue file)\n'; exit 0; }

resolve_dirs=("$GATES_DIR")
while IFS= read -r k; do
    [[ -n "$k" ]] || continue
    [[ -d "$k/checks" ]] && resolve_dirs+=("$k/checks")
    [[ -d "$k/bin" ]] && resolve_dirs+=("$k/bin")
done < <(kit_roots)

built() {
    local name="$1" d
    for d in "${resolve_dirs[@]}"; do [[ -f "$d/$name.sh" ]] && return 0; done
    return 1
}

mapfile -t proposed < <(grep -oE '\b(check|scan)-[a-z0-9]+(-[a-z0-9]+)*' "$QUEUE" 2>/dev/null | sort -u)

unbuilt=0
for name in "${proposed[@]}"; do
    built "$name" || unbuilt=$((unbuilt + 1))
done

live=0
LIST="$GATES_DIR/gates.list"
[[ -f "$LIST" ]] && live="$(grep -cEv '^[[:space:]]*(#|$)' "$LIST" 2>/dev/null || echo 0)"

if [[ "${1:-}" == "--trend" ]]; then
    [[ "$unbuilt" -gt 0 ]] && printf 'gate-backlog %d/%d\n' "$unbuilt" "$live"
    exit 0
fi
printf 'lead\tgate backlog\t%d unbuilt / %d live gates\n' "$unbuilt" "$live"
exit 0
