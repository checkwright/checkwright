#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — friction-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored friction-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

cp "$SMOKE_KIT_ROOT/templates/bash-guard.sh"     scripts/bash-guard.sh
cp "$SMOKE_KIT_ROOT/templates/wakeup-guard.sh"   scripts/wakeup-guard.sh
cp "$SMOKE_KIT_ROOT/templates/friction-config.sh" scripts/friction-config.sh

mkdir -p .claude
jq 'del(.["//"])' "$SMOKE_KIT_ROOT/templates/settings-hooks.json" > .claude/settings.json

{
    echo '.workflow/prompt-friction.log'
    echo '.workflow/wakeup-attempts.log'
} >> .gitignore

set +e
printf '%s' '{"tool_input":{"command":"cd deploy && ls"}}' | bash scripts/bash-guard.sh >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -ne 2 ]]; then
    echo "friction-kit/smoke/install.sh: installed guard did not block a compound-cd payload (exit $rc, want 2)" >&2
    exit 1
fi
