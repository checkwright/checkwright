#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — lifecycle-kit consumer-smoke violation: flips the stage header without the matching stamp, reddening check-stage-evidence
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-stage-evidence"

sed -i 's/\[stage: scope\]/[stage: build]/' TASK-QUEUE.md
