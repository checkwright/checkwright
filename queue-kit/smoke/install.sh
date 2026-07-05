#!/usr/bin/env bash
# queue-kit consumer-smoke install — the executable form of README.md §Install
# steps 1-2. Step 2 is "copy templates/TASK-QUEUE.md and fill it in"; the
# filled-in result is a clean minimal queue (the verbatim example template
# teaches queue grammar to a queue-only consumer, but its example [spec:]/
# feature entries are not spec-kit-clean in a combined tree — see the debt task
# queue-starter-template-not-spec-kit-clean).
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored queue-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# queue-kit
check-queue-hygiene
check-queue-wrap
check-blocked-by-lead-line
check-task-names
check-task-conservation
check-queue-prose-precondition
EOF

# The section skeleton (create if an earlier kit did not); identical content to
# lifecycle-kit's install, so either order yields one minimal queue.
if [[ ! -f TASK-QUEUE.md ]]; then
    cat > TASK-QUEUE.md <<'EOF'
# TASK-QUEUE.md — smoke consumer work queue

## Iteration: —  [stage: scope]

---

## New Features

## Technical Debt

## Deferred

## Done
EOF
fi

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
