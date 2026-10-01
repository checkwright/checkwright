#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — canon-kit consumer-smoke violation: drops a consumer SPEC.md with two Definition-of-Done headings, reddening check-spec-dod-singleton
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

echo "check-spec-dod-singleton"

cat > SPEC.md <<'EOF'
# smoke consumer — SPEC

## Definition of Done

- [ ] the one completion contract

## Behaviour

Some behaviour.

## Definition of Done

- [ ] a second, drifting checklist
EOF
