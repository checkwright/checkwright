#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — canon-kit consumer-smoke violation: drops a consumer SPEC.md with two Definition-of-Done headings, reddening check-spec-dod-singleton
set -euo pipefail

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
