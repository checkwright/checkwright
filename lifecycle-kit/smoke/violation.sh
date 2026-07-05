#!/usr/bin/env bash
# lifecycle-kit consumer-smoke violation — advances the stage header without
# stamping the new stage, the canonical "skipped a stage's invocation" defect.
# First stdout line names the gate the harness expects to see FAIL.
set -euo pipefail

echo "check-stage-evidence"

# Flip the header to build with no matching build stamp in the evidence file;
# git checkout restores it. (check-stage-entry stays clean: build's predecessor
# scope is stamped and there is no cross-component amendment on the tree.)
sed -i 's/\[stage: scope\]/[stage: build]/' TASK-QUEUE.md
