#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — evidence-kit consumer-smoke violation: a fail scenario with no blocking slug reddens check-evidence-baseline
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

echo "check-evidence-baseline"

printf 'unit orphan-scenario fail\n' >> .workflow/validate-baseline.txt
