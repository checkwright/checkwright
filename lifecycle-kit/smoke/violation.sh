#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — lifecycle-kit consumer-smoke violation: flips the stage header without the matching stamp, reddening check-stage-evidence
set -euo pipefail

echo "check-stage-evidence"

sed -i 's/\[stage: scope\]/[stage: build]/' TASK-QUEUE.md
