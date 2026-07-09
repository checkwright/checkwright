#!/usr/bin/env bash
# graph: couples=docs/index.md,CLAUDE.md,kit:gate-tests/* dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-kit-registration — docs/index.md carries a registry row for every kit root (this consumer re-scopes the kit-registration invariant onto the docs index; wrapper, not mechanism)
#
# usage: check-docs-kit-parity.sh [registry-doc]   (default docs/index.md; the
#   optional arg points the fixture pair at a synthetic index)
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
WRAPPED="$SDK/checks/check-kit-registration.sh"
[[ -x "$WRAPPED" ]] || { echo "check-docs-kit-parity: wrapped gate not found: $WRAPPED" >&2; exit 2; }

REG="${1:-docs/index.md}"

out="$("$WRAPPED" "$REG" 2>&1)"; rc=$?
if [[ "$rc" -eq 2 ]]; then
    printf '%s\n' "$out" >&2
    exit 2
fi
if [[ "$rc" -ne 0 ]]; then
    echo "check-docs-kit-parity: a kit root is missing its row in the docs index ($REG):"
    printf '%s\n' "$out"
    echo "  help: add the kit's '[<kit>](<kit>/index.md)' row to $REG (docs/<kit>/ is"
    echo "        the kit's docs page dir), so a landed kit cannot fall out of the docs site."
    exit 1
fi

echo "DOCS-KIT-PARITY: clean ($REG registers every kit root)"
exit 0
