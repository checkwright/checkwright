# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §check-graph — this repo's consumer graph-vocab: group the projection's surfaces into per-kit layers (this is a kit monorepo, so the kit is the natural cluster) with one shared bucket for cross-cutting surfaces

graph_surface_layer() {
    case "$1" in
        gate-sdk/*)       echo k_gate_sdk ;;
        lifecycle-kit/*)  echo k_lifecycle ;;
        queue-kit/*)      echo k_queue ;;
        canon-kit/*)      echo k_canon ;;
        guard-kit/*)      echo k_guard ;;
        delegation-kit/*) echo k_delegation ;;
        context-kit/*)    echo k_context ;;
        *)                echo k_shared ;;
    esac
}

# shellcheck disable=SC2034  # consumed by gate-sdk/checks/check-graph.sh after sourcing
GRAPH_LAYERS=(
    "k_gate_sdk:gate-sdk"
    "k_lifecycle:lifecycle-kit"
    "k_queue:queue-kit"
    "k_canon:canon-kit"
    "k_guard:guard-kit"
    "k_delegation:delegation-kit"
    "k_context:context-kit"
    "k_shared:shared / consumer surfaces"
)
