# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §Layout and configuration — this repo's persistent gate-sdk config, auto-sourced by lib/gate.sh; the config seam's first user

# spec: gate-sdk/SPEC.md §check-graph — publish the coupling-graph artifact on the docs site (a same-tree served file), not in the workflow dir
# shellcheck disable=SC2034  # read by gate-sdk/checks/check-graph.sh after sourcing
GATE_SDK_GRAPH_ARTIFACT="docs/check-graph.html"

# spec: gate-sdk/SPEC.md §check-graph — this repo's graph-theme chrome links the docs host and the source repo; sanction those prefixes for the external-ref assertion
# shellcheck disable=SC2034  # read by gate-sdk/checks/check-graph.sh after sourcing
GATE_SDK_GRAPH_EXTERNAL_REFS="https://checkwright.dev https://github.com/checkwright"

# spec: gate-sdk/SPEC.md §check-shellcheck — this repo ships bash under no kit root: the installer's dispatcher, its verbs, the modules those verbs share, and the runnable adoption walkthrough; name them so the lint that governs every other script in the tree governs these too. Each directory is named on its own because the gate globs *.sh per named directory and does not descend
# shellcheck disable=SC2034  # resolved by gate-sdk/lib/gate.sh onto GATE_LINT_EXTRA_DIRS and read by check-shellcheck across the config bridge
GATE_SDK_LINT_EXTRA_DIRS="installer/bin installer/lib installer/lib/common installer/consumer-smoke demo"
