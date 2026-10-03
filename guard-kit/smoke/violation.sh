#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — guard-kit consumer-smoke violation: a front-end door in the kit README reddens check-door-binding
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

echo "check-door-binding"

printf '\n%s\n' 'Regenerate the hooks: `bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`.' >> "$SMOKE_KIT_ROOT/README.md"
