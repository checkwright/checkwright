#!/usr/bin/env bash
# graph: couples=docs/index.md,docs/*/index.md,kit:gate-tests/* dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-kit-registration — docs/index.md carries a registry row for every kit root (this consumer re-scopes the kit-registration invariant onto the docs index; wrapper, not mechanism) and every docs/<kit>/index.md carries the nav front-matter block so a landed kit page cannot fall out of the site nav
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

has_nav_block() {  # $1=docs/<kit>/index.md — true iff its front matter carries nav_order (sidebar slot) and nav_id (mirror-child nesting)
    awk '
        NR==1 { if ($0 != "---") exit 1; fm=1; next }
        fm && $0 == "---" { fm=0; next }
        fm && /^nav_order:[[:space:]]*[0-9]/ { o=1 }
        fm && /^nav_id:[[:space:]]*[A-Za-z]/ { i=1 }
        END { exit ((o && i) ? 0 : 1) }
    ' "$1"
}

navbad=()
shopt -s nullglob
for idx in "$(dirname "$REG")"/*/index.md; do
    has_nav_block "$idx" || navbad+=("$idx")
done
shopt -u nullglob
if [[ ${#navbad[@]} -gt 0 ]]; then
    echo "check-docs-kit-parity: a docs kit page lacks the nav front-matter block — it would render but fall out of the sidebar nav:"
    printf '  %s\n' "${navbad[@]}"
    echo "  help: give each docs/<kit>/index.md a front-matter block with 'nav_order: <n>'"
    echo "        (its sidebar slot) and 'nav_id: <kit>' (so the kit's mirror pages nest under it)."
    exit 1
fi

echo "DOCS-KIT-PARITY: clean ($REG registers every kit root; every docs kit page carries the nav front-matter block)"
exit 0
