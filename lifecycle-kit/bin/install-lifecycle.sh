#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — idempotent insert/replace of the resident registration block between fixed markers in the always-loaded agent file
set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"
# shellcheck source=../../gate-sdk/lib/inject.sh
source "$SDK/lib/inject.sh"

AGENT_FILE="${1:-$LIFECYCLE_KIT_AGENT_FILE}"
[[ -f "$AGENT_FILE" ]] \
    || { echo "install-lifecycle: agent file not found: $AGENT_FILE — nothing to install into" >&2; exit 2; }

BEGIN="<!-- lifecycle-kit:begin -->"
END="<!-- lifecycle-kit:end -->"

action="$(lifecycle_registration_block | inject_marker_block "$AGENT_FILE" "$BEGIN" "$END")" || exit $?
echo "install-lifecycle: $action the lifecycle registration block in $AGENT_FILE"
