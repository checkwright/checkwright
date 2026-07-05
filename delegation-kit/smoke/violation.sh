#!/usr/bin/env bash
# delegation-kit consumer-smoke violation — stages a gate edit co-staged with a
# product file (assertion A: gate-edit isolation). check-gate-tamper reads the
# staged index, so the tamper shape must be `git add`-ed; the runner's git-reset
# restore unstages it. First stdout line names the expected FAIL gate.
set -euo pipefail

echo "check-gate-tamper"

# Default config: scripts/check-*.sh is a gate file. Stage a new one together
# with a non-meta product path — the attested tamper shape assertion A blocks.
# (The gate file is ShellCheck-clean so only check-gate-tamper reds, not the
# self-lint gate.)
cat > scripts/check-smoke-gate.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
echo "SMOKE-GATE: clean"
exit 0
EOF
mkdir -p product
printf 'product code\n' > product/app.txt
git add scripts/check-smoke-gate.sh product/app.txt
