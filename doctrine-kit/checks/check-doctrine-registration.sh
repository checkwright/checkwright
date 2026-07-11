#!/usr/bin/env bash
# graph: couples=CLAUDE.md,doctrine-kit/DOCTRINE.md dir=one valve=none tier=precommit
# spec: doctrine-kit/SPEC.md §check-doctrine-registration — the always-loaded agent file carries a markdown link to the doctrine file, fail-closed when the agent file is missing
#
# usage: check-doctrine-registration.sh [agent-file [doctrine-file]]
#   paths resolve relative to cwd (= repo root in a battery run); defaults come
#   from DOCTRINE_KIT_AGENT_FILE / DOCTRINE_KIT_DOCTRINE_FILE (doctrine-kit/lib/doctrine.sh).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/doctrine.sh
source "$KIT/lib/doctrine.sh"

AGENT_FILE="${1:-$DOCTRINE_KIT_AGENT_FILE}"
DOCTRINE_FILE="${2:-$DOCTRINE_KIT_DOCTRINE_FILE}"
[[ -f "$AGENT_FILE" ]] \
    || { echo "check-doctrine-registration: agent file not found: $AGENT_FILE" >&2; exit 2; }  # exit 2: fail-closed

if grep -qF -- "]($DOCTRINE_FILE" "$AGENT_FILE"; then
    echo "DOCTRINE-REGISTRATION: clean ($AGENT_FILE links the doctrine file $DOCTRINE_FILE)"
    exit 0
fi
st=$?
[[ "$st" -eq 1 ]] || fail_closed "$st" check-doctrine-registration grep

echo "check-doctrine-registration: $AGENT_FILE carries no markdown link to the doctrine file:"
echo "  $DOCTRINE_FILE"
echo "  help: install the doctrine reference block into the always-loaded agent file —"
echo "        bash doctrine-kit/bin/install-doctrine.sh — so a session that loads it"
echo "        follows the link to the delivery doctrine. Override the paths with"
echo "        DOCTRINE_KIT_AGENT_FILE / DOCTRINE_KIT_DOCTRINE_FILE."
exit 1
