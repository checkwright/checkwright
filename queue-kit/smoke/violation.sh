#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — queue-kit consumer-smoke violation:
# appends a column-0 prose line, reddening check-queue-hygiene. First stdout line
# names the expected FAIL gate; git checkout restores the file.
set -euo pipefail

echo "check-queue-hygiene"

printf '%s\n' 'a stray prose line at column zero' >> TASK-QUEUE.md
