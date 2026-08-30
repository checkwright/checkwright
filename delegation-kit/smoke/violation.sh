#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — consumer-smoke violation: co-staged gate edit + product file reddens check-gate-tamper (assertion A)
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30, which reaches this file by its GROUND rather than by its scope: that ruling stated-contract cut covers the recipes answering to §Consumer smoke and this one answers to delegation-kit/SPEC.md §Testing, but both legs hold of it identically. Leg 2: an executable recipe by stated contract — the harness runs it against the scratch tree and reads the expected gate name off its first stdout line. Leg 3: it vendors with the kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers are this repo own validate suites. Structural, not a sizing judgment.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

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
