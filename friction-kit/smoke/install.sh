#!/usr/bin/env bash
# friction-kit consumer-smoke install — the executable form of README.md §Install.
# The kit registers NO gates, so nothing joins scripts/gates.list; the install
# copies the guard framework + config into the gates dir, wires the PreToolUse
# hooks into .claude/settings.json, gitignores the scratch logs, then drives one
# crafted payload through the installed guard and asserts a block (self-verifying).
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored friction-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

# 1. Guard framework + config into the gates dir (gate-sdk seeded scripts/).
cp "$SMOKE_KIT_ROOT/templates/bash-guard.sh"     scripts/bash-guard.sh
cp "$SMOKE_KIT_ROOT/templates/wakeup-guard.sh"   scripts/wakeup-guard.sh
cp "$SMOKE_KIT_ROOT/templates/friction-config.sh" scripts/friction-config.sh

# 2. Hook wiring into .claude/settings.json (drop the "//" note key).
mkdir -p .claude
jq 'del(.["//"])' "$SMOKE_KIT_ROOT/templates/settings-hooks.json" > .claude/settings.json

# 3. The two per-iteration scratch logs are gitignored.
{
    echo '.workflow/prompt-friction.log'
    echo '.workflow/wakeup-attempts.log'
} >> .gitignore

# 4. Self-verify: a crafted compound-cd payload must be blocked (exit 2).
set +e
printf '%s' '{"tool_input":{"command":"cd deploy && ls"}}' | bash scripts/bash-guard.sh >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -ne 2 ]]; then
    echo "friction-kit/smoke/install.sh: installed guard did not block a compound-cd payload (exit $rc, want 2)" >&2
    exit 1
fi
