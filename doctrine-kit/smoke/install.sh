#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — doctrine-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored doctrine-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

# spec: gate-sdk/SPEC.md §Consumer smoke — seed a minimal always-loaded agent file (guarded, so it composes with any kit that already dropped one), then install the reference block into it via the shipped installer
if [[ ! -f CLAUDE.md ]]; then
    cat > CLAUDE.md <<'EOF'
# CLAUDE.md — smoke consumer

Resident bindings the consumer keeps.
EOF
fi
bash "$SMOKE_KIT_ROOT/bin/install-doctrine.sh" >/dev/null

cat >> scripts/gates.list <<'EOF'
# doctrine-kit
check-doctrine-registration
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
