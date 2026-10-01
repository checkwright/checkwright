#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — consumer-smoke violation: co-staged gate edit + product file reddens check-gate-tamper (assertion A)
# no-port: delegation-kit/SPEC.md §Testing — legs 2 and 3 of gate-sdk/SPEC.md §Consumer smoke, The port disposition, reaching this file by ground.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

echo "check-gate-tamper"

cat > scripts/check-smoke-gate.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
echo "SMOKE-GATE: clean"
exit 0
EOF
mkdir -p product
printf 'product code\n' > product/app.txt
git add scripts/check-smoke-gate.sh product/app.txt
