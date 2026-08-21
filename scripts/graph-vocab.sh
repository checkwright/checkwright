# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §check-graph — this repo's consumer graph-vocab: group the projection's surfaces into per-kit layers (this is a kit monorepo, so the kit is the natural cluster) with one shared bucket for cross-cutting surfaces

# shellcheck disable=SC2034  # read across the dispatch seam by the compiled member; gate-sdk/lib/gate.sh sources this file and the config bridge resolves the value
GRAPH_LAYER_RULES=(
    "gate-sdk/:k_gate_sdk"
    "lifecycle-kit/:k_lifecycle"
    "queue-kit/:k_queue"
    "canon-kit/:k_canon"
    "guard-kit/:k_guard"
    "delegation-kit/:k_delegation"
    "context-kit/:k_context"
)
# shellcheck disable=SC2034  # read across the dispatch seam by the compiled member
GRAPH_LAYER_DEFAULT=k_shared

# shellcheck disable=SC2034  # read across the dispatch seam by the compiled member
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
