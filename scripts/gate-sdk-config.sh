# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §Layout and configuration — this repo's persistent gate-sdk config, auto-sourced by lib/gate.sh; the config seam's first user

# spec: gate-sdk/SPEC.md §check-graph — publish the coupling-graph artifact on the docs site (a same-tree served file), not in the workflow dir
# shellcheck disable=SC2034  # read by gate-sdk/checks/check-graph.sh after sourcing
GATE_SDK_GRAPH_ARTIFACT="docs/check-graph.html"
