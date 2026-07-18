#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — canon-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored canon-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# canon-kit (check-surface-duplication omitted — needs a glossary)
check-amendment-queue
check-spec-dod-singleton
check-spec-derivable-section
check-spec-embedded-source
EOF

# spec: gate-sdk/SPEC.md §Consumer smoke — seed check-amendment-queue's surface (guarded; carries lifecycle-kit's inert header so the seed composes with the stage gates)
if [[ ! -f TASK-QUEUE.md ]]; then
    cat > TASK-QUEUE.md <<'EOF'
# TASK-QUEUE.md — smoke consumer work queue

## Iteration: —

---

## New Features

## Technical Debt

## Deferred

## Done
EOF
fi

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
