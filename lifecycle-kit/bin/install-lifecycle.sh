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

# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the merge-attribute step: inject the derived iteration-scoped supersede lines into .gitattributes (marker-bounded, idempotent). Unlike the agent file this installer legitimately mints .gitattributes when absent (it is not an always-loaded file the consumer authored).
ATTRS=".gitattributes"
ABEGIN="# lifecycle-kit:merge:begin"
AEND="# lifecycle-kit:merge:end"
[[ -f "$ATTRS" ]] || : > "$ATTRS"
aaction="$(lifecycle_merge_attrs_block | inject_marker_block "$ATTRS" "$ABEGIN" "$AEND")" || exit $?
echo "install-lifecycle: $aaction the iteration-scoped merge attributes in $ATTRS"

# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the driver-config step: register the keep-ours merge driver, per-clone (the install-hooks.sh opt-in class). The .gitattributes attribute is inert without it (recorded honest limit); a non-repo cwd degrades to a skip, never a hard failure.
if git rev-parse --git-dir >/dev/null 2>&1; then
    git config merge.iteration-scoped.driver true
    echo "install-lifecycle: registered the keep-ours merge.iteration-scoped driver (per-clone git config)"
else
    echo "install-lifecycle: not a git repository — skipped the merge.iteration-scoped driver (the .gitattributes attribute stays inert until 'git config merge.iteration-scoped.driver true' is run in a clone)" >&2
fi
