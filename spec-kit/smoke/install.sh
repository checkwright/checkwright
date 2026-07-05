#!/usr/bin/env bash
# spec-kit consumer-smoke install — the executable form of README.md §Install
# step 1, registering each gate whose surface exists. check-surface-duplication
# is omitted (it needs a glossary and exits 2 without one). Under zero config
# the vendored kit SPECs are pruned (SPEC_KIT_SCAN_KIT_ROOTS=0), so the spec
# gates are clean with no consumer spec present; the queue satisfies
# check-amendment-queue empty.
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored spec-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# spec-kit (check-surface-duplication omitted — needs a glossary)
check-amendment-queue
check-spec-dod-singleton
check-spec-derivable-section
check-spec-embedded-source
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
