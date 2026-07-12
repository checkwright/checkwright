#!/usr/bin/env bash
# graph: couples=CLAUDE.md,lifecycle-kit/lib/stages.sh dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-lifecycle-registration — the always-loaded agent file carries a lifecycle-kit marker block whose content byte-matches the block regenerated from the live stage machine, fail-closed when the target or a marker is missing
#
# usage: check-lifecycle-registration.sh [agent-file]
#   path resolves relative to cwd (= repo root in a battery run); the default
#   comes from LIFECYCLE_KIT_AGENT_FILE (lifecycle-kit/lib/stages.sh).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

AGENT_FILE="${1:-$LIFECYCLE_KIT_AGENT_FILE}"
[[ -f "$AGENT_FILE" ]] \
    || { echo "check-lifecycle-registration: agent file not found: $AGENT_FILE" >&2; exit 2; }  # exit 2: fail-closed

BEGIN="<!-- lifecycle-kit:begin -->"
END="<!-- lifecycle-kit:end -->"

if ! grep -qF -- "$BEGIN" "$AGENT_FILE"; then
    echo "check-lifecycle-registration: no lifecycle-kit registration block in $AGENT_FILE"
    echo "  help: install the resident registration block into the always-loaded agent file —"
    echo "        bash lifecycle-kit/bin/install-lifecycle.sh — so a session that loads it"
    echo "        is pointed at the stage machine. Override the path with LIFECYCLE_KIT_AGENT_FILE."
    exit 1
fi
grep -qF -- "$END" "$AGENT_FILE" \
    || { echo "check-lifecycle-registration: begin marker present but end marker missing in $AGENT_FILE — the block bounds are unreadable" >&2; exit 2; }  # exit 2: fail-closed

present="$(awk -v b="$BEGIN" -v e="$END" '
    $0 == b { inb = 1; next }
    $0 == e { inb = 0; next }
    inb { print }
' "$AGENT_FILE")"; st=$?
fail_closed "$st" check-lifecycle-registration awk

expected="$(lifecycle_registration_block)"; st=$?
fail_closed "$st" check-lifecycle-registration lifecycle_registration_block

if [[ "$present" != "$expected" ]]; then
    echo "check-lifecycle-registration: the registration block in $AGENT_FILE is stale — it does not match the block derived from the live stage machine:"
    diff <(printf '%s\n' "$expected") <(printf '%s\n' "$present") | sed 's/^/  /'
    echo "  help: a reshaped stage machine (LIFECYCLE_KIT_STAGES / LIFECYCLE_KIT_QUEUE_FILE) or a"
    echo "        hand-edited block staled the registration — regenerate it in place:"
    echo "        bash lifecycle-kit/bin/install-lifecycle.sh"
    exit 1
fi

echo "LIFECYCLE-REGISTRATION: clean ($AGENT_FILE carries the lifecycle-kit registration block in byte-lockstep with the derived stage machine; ${#LIFECYCLE_KIT_STAGES[@]} stage(s))"
exit 0
