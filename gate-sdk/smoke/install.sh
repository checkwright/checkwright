#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — gate-sdk consumer-smoke install (README.md §Quick start)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored gate-sdk copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT"   # gate-sdk installs itself; its tools live here

mkdir -p scripts .workflow

cat > scripts/gates.list <<'EOF'
# Consumer-smoke gate registry (gate-sdk meta-gates; kits append below).
check-shellcheck
check-gate-output
check-gate-fail-closed
check-gate-fixture-coverage
check-gate-exemption-tasks
check-gate-assertions
check-graph
check-commit-msg
check-tree-terms
EOF

# spec: gate-sdk/SPEC.md §Consumer smoke — ship the tracked default pattern list; the local companion is absent, exercising the fresh-clone path
cp "$SDK/templates/msg-patterns.list" scripts/msg-patterns.list

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
