#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — spec-kit consumer-smoke install (README.md §Install)
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
