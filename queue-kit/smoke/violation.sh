#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — queue-kit consumer-smoke violation: column-0 prose reddens check-queue-hygiene
set -euo pipefail

echo "check-queue-hygiene"

printf '%s\n' 'a stray prose line at column zero' >> TASK-QUEUE.md
