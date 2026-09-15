#!/usr/bin/env bash
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-gates.sh --run-consumer-smoke}"
mkdir -p scripts
cat >> scripts/gates.list <<'EOF'
# alpha-kit
check-alpha
EOF
