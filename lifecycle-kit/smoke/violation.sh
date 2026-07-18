#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — lifecycle-kit consumer-smoke violation: renames the header's iteration out from under the stamps, reddening check-stage-evidence's name-axis (staleness) assertion
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-stage-evidence"

sed -i 's/^## Iteration: .*$/## Iteration: renamed-out-of-band/' TASK-QUEUE.md
