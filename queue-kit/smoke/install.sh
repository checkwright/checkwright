#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — queue-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored queue-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# queue-kit
check-queue-hygiene
check-queue-wrap
check-tag-lead-line
check-task-names
check-task-conservation
check-queue-prose-precondition
check-roadmap-fresh
check-queue-entry-budget
check-deferred-board-tags
check-queue-sections
check-queue-slug-liveness
EOF

cp "$SMOKE_KIT_ROOT/templates/TASK-QUEUE.md" TASK-QUEUE.md

bash "$SDK/bin/run-gates.sh" --emit git-hooks --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html
