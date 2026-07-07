#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — consumer-smoke violation: stages a gate
# edit co-staged with a product file (assertion A, gate-edit isolation) to redden
# check-gate-tamper; the gate reads the staged index, so the shape must be `git
# add`-ed and the runner's git-reset restore unstages it. First stdout line names
# the expected FAIL gate.
set -euo pipefail

echo "check-gate-tamper"

# spec: delegation-kit/SPEC.md §Testing — default config makes scripts/check-*.sh
# a gate file; staging a new one alongside a non-meta product path is the attested
# assertion-A shape. The gate file is shellcheck-clean so only check-gate-tamper
# reds, not the self-lint gate.
cat > scripts/check-smoke-gate.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
echo "SMOKE-GATE: clean"
exit 0
EOF
mkdir -p product
printf 'product code\n' > product/app.txt
git add scripts/check-smoke-gate.sh product/app.txt
