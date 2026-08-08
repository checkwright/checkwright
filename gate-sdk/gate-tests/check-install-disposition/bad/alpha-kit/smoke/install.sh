#!/usr/bin/env bash
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
mkdir -p scripts
cat >> scripts/gates.list <<'EOF'
# alpha-kit
EOF
