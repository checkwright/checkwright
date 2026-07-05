#!/usr/bin/env bash
# gate-sdk consumer-smoke install — the executable form of README.md §Quick start.
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored gate-sdk copy.
# Establishes the gates dir + registry with the seven meta-gates, the .workflow
# dir, and the generated hook + graph artifacts. Runs first, so it seeds
# scripts/gates.list; later kits append to it.
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
EOF

# Regenerate the coupling artifacts check-graph asserts fresh (README steps).
bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
