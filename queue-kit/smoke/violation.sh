#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — queue-kit consumer-smoke violation: column-0 prose reddens check-queue-hygiene
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

echo "check-queue-hygiene"

printf '%s\n' 'a stray prose line at column zero' >> TASK-QUEUE.md
