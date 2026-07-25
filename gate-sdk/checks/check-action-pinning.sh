#!/usr/bin/env bash
# graph: couples=.github/workflows/*.yml,.github/workflows/*.yaml,.github/ISSUE_TEMPLATE/*.yml,docs/_config.yml,kit:templates/*.yml,kit:templates/*.yaml dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-action-pinning — every `uses:` ref in a scanned YAML file is immutable: a full 40-hex commit SHA, or a repo-local ./ path git pins at checkout
#
# usage: check-action-pinning.sh [scan-root]
#   scan-root: the walked tree (default '.').
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

SCANROOT="${1:-.}"
[[ -d "$SCANROOT" ]] || { echo "check-action-pinning: scan root not found: $SCANROOT" >&2; exit 2; }

listing="$(gate_find "$SCANROOT" -type f \( -name '*.yml' -o -name '*.yaml' \))"; st=$?
fail_closed "$st" ACTION-PINNING gate_find
mapfile -t files < <(printf '%s' "$listing")

if [[ ${#files[@]} -eq 0 ]]; then
    echo "ACTION-PINNING: clean (no YAML under $SCANROOT — 0 uses: refs to pin)"
    exit 0
fi

scan="$(awk '
match($0, /^[[:space:]]*(#[[:space:]]*)?(-[[:space:]]+)?uses:[[:space:]]*/) {
    v = substr($0, RSTART + RLENGTH)
    sub(/[[:space:]].*$/, "", v)
    gsub(/^[\047"]+|[\047"]+$/, "", v)
    if (v != "") printf "%s:%d\t%s\n", FILENAME, FNR, v
}
' "${files[@]}")"; st=$?
fail_closed "$st" ACTION-PINNING awk

refs=0; stray=()
while IFS=$'\t' read -r loc ref || [[ -n "$loc" ]]; do
    [[ -n "$ref" ]] || continue
    refs=$((refs + 1))
    [[ "$ref" == ./* ]] && continue
    [[ "$ref" =~ @[0-9a-f]{40}$ ]] && continue
    stray+=("$loc: $ref")
done <<< "$scan"

if [[ ${#stray[@]} -gt 0 ]]; then
    echo "check-action-pinning: mutable action ref — a tag or branch is repointable by"
    echo "whoever owns it, so the code a run executes is not the code that was reviewed:"
    printf '  %s\n' "${stray[@]}"
    echo "  help: replace the ref with the full 40-hex commit SHA the tag resolves to,"
    echo "        keeping the tag as a trailing comment (uses: owner/repo@<sha> # v1.2.3)."
    echo "        A repo-local ./ action needs no pin — the checkout already pins it."
    exit 1
fi

echo "ACTION-PINNING: clean ($refs uses: ref(s) across ${#files[@]} YAML file(s) under $SCANROOT, all immutable)"
exit 0
