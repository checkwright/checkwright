#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — gate-sdk consumer-smoke violation (line 1 = expected gate)
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-shellcheck"

cat > scripts/check-smoke-dirty.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
unused_var="this variable is never read"
echo "DIRTY: clean"
EOF
