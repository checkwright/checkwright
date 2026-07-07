#!/usr/bin/env bash
# queue-kit consumer-smoke install — the executable form of README.md §Install
# steps 1-2. Step 2 is "copy templates/TASK-QUEUE.md and fill it in"; the smoke
# exercises the shipped template verbatim (gate-sdk/SPEC.md §Consumer smoke — a
# starter template ships battery-clean and its smoke installs it unedited, so a
# template regression against any kit's gates reddens the harness).
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored queue-kit copy.
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

# The queue file is the shipped starter template, verbatim (the conformance
# contract: no fill-in, so any drift against a foreign kit's gate reddens here).
cp "$SMOKE_KIT_ROOT/templates/TASK-QUEUE.md" TASK-QUEUE.md

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
