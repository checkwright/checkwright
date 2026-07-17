#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — queue-kit consumer-smoke violation: column-0 prose reddens check-queue-hygiene
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-queue-hygiene"

printf '%s\n' 'a stray prose line at column zero' >> TASK-QUEUE.md
