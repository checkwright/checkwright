#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — consumer-smoke violation: co-staged gate edit + product file reddens check-gate-tamper (assertion A)
set -euo pipefail

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
