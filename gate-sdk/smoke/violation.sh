#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — gate-sdk consumer-smoke violation (line 1 = expected gate)
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

echo "check-shellcheck"

cat > scripts/check-smoke-dirty.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
unused_var="this variable is never read"
echo "DIRTY: clean"
EOF
