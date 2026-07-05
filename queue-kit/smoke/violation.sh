#!/usr/bin/env bash
# queue-kit consumer-smoke violation — writes a column-0 prose line into the
# queue (the hygiene gate's no-prose axis: every column-0 line must be a
# heading, bullet, or rule). First stdout line names the expected FAIL gate.
set -euo pipefail

echo "check-queue-hygiene"

# Append stray column-0 prose under a section; git checkout restores the file.
printf '%s\n' 'a stray prose line at column zero' >> TASK-QUEUE.md
