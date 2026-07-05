#!/usr/bin/env bash
# lifecycle-kit consumer-smoke install — the executable form of README.md §Install
# steps 1-2, plus the first /scope stamp the README notes is required for green
# (the bare header leaves the battery red at check-stage-evidence by design).
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored lifecycle-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

# Register the stage gates (gate-sdk seeded scripts/gates.list).
cat >> scripts/gates.list <<'EOF'
# lifecycle-kit
check-stage-evidence
check-stage-entry
EOF

# The queue file carries the iteration header; create the minimal skeleton if no
# earlier kit did (same content queue-kit's install would write).
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

# The evidence file: skeleton plus the bootstrap /scope stamp (an unnamed
# iteration at its first stage — the stamp check-stage-evidence requires).
mkdir -p .workflow
cat > .workflow/WORKFLOW-STATE.txt <<EOF
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

— scope smoke001 $(date +%F)
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
