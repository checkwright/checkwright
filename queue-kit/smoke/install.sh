#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — queue-kit consumer-smoke install
# (README.md §Install): copies the shipped TASK-QUEUE.md template verbatim, so a
# template regression against any kit's gate reddens the harness. cwd =
# scratch-consumer root; SMOKE_KIT_ROOT = the vendored queue-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# queue-kit
check-queue-hygiene
check-queue-wrap
check-tag-lead-line
check-task-names
check-task-conservation
check-queue-prose-precondition
EOF

cp "$SMOKE_KIT_ROOT/templates/TASK-QUEUE.md" TASK-QUEUE.md

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
