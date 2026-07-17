#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — doctrine-kit consumer-smoke violation: strips the reference block from the agent file, reddening check-doctrine-registration
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-doctrine-registration"

cat > CLAUDE.md <<'EOF'
# CLAUDE.md — smoke consumer

Resident bindings, but the doctrine reference block was removed — no markdown
link to the doctrine file.
EOF
