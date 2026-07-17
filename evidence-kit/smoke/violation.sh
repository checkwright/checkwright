#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — evidence-kit consumer-smoke violation: a fail scenario with no blocking slug reddens check-evidence-baseline
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-evidence-baseline"

printf 'unit orphan-scenario fail\n' >> .workflow/validate-baseline.txt
