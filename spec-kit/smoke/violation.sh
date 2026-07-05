#!/usr/bin/env bash
# spec-kit consumer-smoke violation — drops a consumer SPEC.md carrying two
# Definition-of-Done headings (two sources on the completion contract). First
# stdout line names the expected FAIL gate.
set -euo pipefail

echo "check-spec-dod-singleton"

# A root-level consumer spec is scanned even with the kit-root prune on; two DoD
# headings trip check-spec-dod-singleton. git clean -fd removes the file.
cat > SPEC.md <<'EOF'
# smoke consumer — SPEC

## Definition of Done

- [ ] the one completion contract

## Behaviour

Some behaviour.

## Definition of Done

- [ ] a second, drifting checklist
EOF
